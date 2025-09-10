use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};

const HISTORY_SIZE: usize = 1000;
const VIOLATION_HISTORY_SIZE: usize = 100;
const BUCKET_BOUNDARIES: &[u64] = &[10, 25, 50, 75, 100, 150, 200, 500, 1000];

/// Comprehensive latency metrics with percentile tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyMetrics {
    pub count: u64,
    pub sum_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub avg_ms: f64,
    pub p50: f64,
    pub p95: f64,
    pub p99: f64,
    pub violations: u64,
    pub histogram: Vec<u64>,
}

impl Default for LatencyMetrics {
    fn default() -> Self {
        Self {
            count: 0,
            sum_ms: 0.0,
            min_ms: f64::MAX,
            max_ms: 0.0,
            avg_ms: 0.0,
            p50: 0.0,
            p95: 0.0,
            p99: 0.0,
            violations: 0,
            histogram: vec![0; BUCKET_BOUNDARIES.len() + 1],
        }
    }
}

/// Details about a latency budget violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyViolation {
    pub timestamp: u64,  // Using u64 for serialization compatibility
    pub operation: String,
    pub measured_ms: f64,
    pub budget_ms: f64,
    pub context: String,
}

/// Advanced latency monitoring with rich metrics and violation tracking
pub struct LatencyMonitor {
    metrics: Arc<RwLock<LatencyMetrics>>,
    recent_latencies: Arc<RwLock<VecDeque<f64>>>,
    violations: Arc<RwLock<VecDeque<LatencyViolation>>>,
    budget_ms: f64,
    start_time: Instant,
}

impl LatencyMonitor {
    /// Create a new latency monitor with specified budget
    pub fn new(budget_ms: f64) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(LatencyMetrics::default())),
            recent_latencies: Arc::new(RwLock::new(VecDeque::with_capacity(HISTORY_SIZE))),
            violations: Arc::new(RwLock::new(VecDeque::with_capacity(VIOLATION_HISTORY_SIZE))),
            budget_ms,
            start_time: Instant::now(),
        }
    }
    
    /// Start timing an operation with RAII guard
    pub fn start_operation(&self, operation: &str) -> OperationGuard {
        OperationGuard {
            monitor: self,
            start: Instant::now(),
            operation: operation.to_string(),
            completed: false,
        }
    }
    
    /// Record a latency measurement
    pub async fn record_latency(&self, latency_ms: f64, operation: &str) {
        let mut metrics = self.metrics.write().await;
        let mut recent = self.recent_latencies.write().await;
        
        // Update basic metrics
        metrics.count += 1;
        metrics.sum_ms += latency_ms;
        metrics.min_ms = metrics.min_ms.min(latency_ms);
        metrics.max_ms = metrics.max_ms.max(latency_ms);
        metrics.avg_ms = metrics.sum_ms / metrics.count as f64;
        
        // Update histogram
        let bucket_idx = BUCKET_BOUNDARIES
            .iter()
            .position(|&b| latency_ms <= b as f64)
            .unwrap_or(BUCKET_BOUNDARIES.len());
        metrics.histogram[bucket_idx] += 1;
        
        // Track violations
        if latency_ms > self.budget_ms {
            metrics.violations += 1;
            
            let violation = LatencyViolation {
                timestamp: self.start_time.elapsed().as_secs(),
                operation: operation.to_string(),
                measured_ms: latency_ms,
                budget_ms: self.budget_ms,
                context: format!("Operation '{}' exceeded {}ms budget with {}ms latency", 
                    operation, self.budget_ms, latency_ms),
            };
            
            let mut violations = self.violations.write().await;
            violations.push_back(violation);
            if violations.len() > VIOLATION_HISTORY_SIZE {
                violations.pop_front();
            }
            
            tracing::warn!(
                operation = %operation,
                latency_ms = latency_ms,
                budget_ms = self.budget_ms,
                excess_ms = latency_ms - self.budget_ms,
                "Latency budget violation detected"
            );
        }
        
        // Update recent history for percentile calculation
        recent.push_back(latency_ms);
        if recent.len() > HISTORY_SIZE {
            recent.pop_front();
        }
        
        // Recalculate percentiles
        if !recent.is_empty() {
            let mut sorted: Vec<f64> = recent.iter().cloned().collect();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            let len = sorted.len();
            metrics.p50 = sorted[len / 2];
            metrics.p95 = sorted[len * 95 / 100];
            metrics.p99 = sorted[(len * 99 / 100).min(len - 1)];
        }
    }
    
    /// Get current metrics snapshot
    pub async fn get_metrics(&self) -> LatencyMetrics {
        self.metrics.read().await.clone()
    }
    
    /// Get recent violations
    pub async fn get_recent_violations(&self, limit: usize) -> Vec<LatencyViolation> {
        let violations = self.violations.read().await;
        violations
            .iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
    
    /// Reset all metrics
    pub async fn reset(&self) {
        *self.metrics.write().await = LatencyMetrics::default();
        self.recent_latencies.write().await.clear();
        self.violations.write().await.clear();
    }
    
    /// Get histogram buckets for visualization
    pub fn get_histogram_buckets() -> Vec<String> {
        let mut buckets = Vec::new();
        buckets.push("0-10ms".to_string());
        for i in 1..BUCKET_BOUNDARIES.len() {
            buckets.push(format!("{}-{}ms", BUCKET_BOUNDARIES[i-1], BUCKET_BOUNDARIES[i]));
        }
        buckets.push(format!(">{}ms", BUCKET_BOUNDARIES.last().unwrap()));
        buckets
    }
}

/// RAII guard for automatic timing
pub struct OperationGuard<'a> {
    monitor: &'a LatencyMonitor,
    start: Instant,
    operation: String,
    completed: bool,
}

impl<'a> OperationGuard<'a> {
    /// Complete the operation and record its latency
    pub async fn complete(mut self) {
        let elapsed_ms = self.start.elapsed().as_millis() as f64;
        self.monitor.record_latency(elapsed_ms, &self.operation).await;
        self.completed = true;
        
        tracing::trace!(
            operation = %self.operation,
            latency_ms = elapsed_ms,
            "Operation completed"
        );
    }
    
    /// Complete with additional context
    pub async fn complete_with_context(mut self, context: &str) {
        let elapsed_ms = self.start.elapsed().as_millis() as f64;
        let op_with_context = format!("{}: {}", self.operation, context);
        self.monitor.record_latency(elapsed_ms, &op_with_context).await;
        self.completed = true;
    }
}

impl<'a> Drop for OperationGuard<'a> {
    fn drop(&mut self) {
        if !self.completed {
            let elapsed_ms = self.start.elapsed().as_millis() as f64;
            tracing::warn!(
                operation = %self.operation,
                elapsed_ms = elapsed_ms,
                "OperationGuard dropped without completion - operation may have failed or panicked"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_latency_metrics_basic() {
        let monitor = LatencyMonitor::new(50.0);
        
        // Record some latencies
        monitor.record_latency(10.0, "test_op").await;
        monitor.record_latency(30.0, "test_op").await;
        monitor.record_latency(60.0, "test_op").await; // Violation
        monitor.record_latency(20.0, "test_op").await;
        
        let metrics = monitor.get_metrics().await;
        
        assert_eq!(metrics.count, 4);
        assert_eq!(metrics.violations, 1);
        assert_eq!(metrics.min_ms, 10.0);
        assert_eq!(metrics.max_ms, 60.0);
        assert_eq!(metrics.avg_ms, 30.0);
    }
    
    #[tokio::test]
    async fn test_violation_tracking() {
        let monitor = LatencyMonitor::new(25.0);
        
        // Generate violations
        monitor.record_latency(30.0, "slow_op1").await;
        monitor.record_latency(50.0, "slow_op2").await;
        monitor.record_latency(10.0, "fast_op").await;
        
        let violations = monitor.get_recent_violations(10).await;
        
        assert_eq!(violations.len(), 2);
        assert_eq!(violations[0].operation, "slow_op2");
        assert_eq!(violations[1].operation, "slow_op1");
    }
    
    #[tokio::test]
    async fn test_operation_guard() {
        let monitor = LatencyMonitor::new(50.0);
        
        {
            let guard = monitor.start_operation("test_operation");
            tokio::time::sleep(Duration::from_millis(10)).await;
            guard.complete().await;
        }
        
        let metrics = monitor.get_metrics().await;
        assert_eq!(metrics.count, 1);
        assert!(metrics.min_ms >= 10.0);
    }
    
    #[tokio::test]
    async fn test_histogram_distribution() {
        let monitor = LatencyMonitor::new(100.0);
        
        // Add latencies in different buckets
        monitor.record_latency(5.0, "op").await;    // 0-10ms bucket
        monitor.record_latency(15.0, "op").await;   // 10-25ms bucket
        monitor.record_latency(30.0, "op").await;   // 25-50ms bucket
        monitor.record_latency(500.0, "op").await;  // 200-500ms bucket
        monitor.record_latency(1500.0, "op").await; // >1000ms bucket
        
        let metrics = monitor.get_metrics().await;
        
        assert_eq!(metrics.histogram[0], 1); // 0-10ms
        assert_eq!(metrics.histogram[1], 1); // 10-25ms
        assert_eq!(metrics.histogram[2], 1); // 25-50ms
        assert_eq!(metrics.histogram[7], 1); // 200-500ms
        assert_eq!(metrics.histogram[9], 1); // >1000ms
    }
}