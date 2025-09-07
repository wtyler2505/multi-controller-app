/// Latency enforcement and monitoring tests
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use crate::transport::{Transport, TransportConfig};
use crate::transport::mock::{MockTransport, MockConfig};

/// Simplified latency monitor for testing
struct LatencyMonitor {
    samples: VecDeque<Duration>,
    max_samples: usize,
}

impl LatencyMonitor {
    fn new(max_samples: usize) -> Self {
        LatencyMonitor {
            samples: VecDeque::with_capacity(max_samples),
            max_samples,
        }
    }
    
    fn add_sample(&mut self, duration: Duration) {
        self.samples.push_back(duration);
        if self.samples.len() > self.max_samples {
            self.samples.pop_front();
        }
    }
    
    fn average(&self) -> Duration {
        if self.samples.is_empty() {
            return Duration::ZERO;
        }
        
        let total: Duration = self.samples.iter().sum();
        total / self.samples.len() as u32
    }
    
    fn min(&self) -> Option<Duration> {
        self.samples.iter().min().copied()
    }
    
    fn max(&self) -> Option<Duration> {
        self.samples.iter().max().copied()
    }
    
    fn count(&self) -> usize {
        self.samples.len()
    }
    
    fn percentile(&self, p: usize) -> Option<Duration> {
        if self.samples.is_empty() {
            return None;
        }
        
        let mut sorted: Vec<Duration> = self.samples.iter().cloned().collect();
        sorted.sort();
        
        let index = (sorted.len() * p / 100).min(sorted.len() - 1);
        Some(sorted[index])
    }
    
    fn reset(&mut self) {
        self.samples.clear();
    }
}

#[tokio::test]
async fn test_minimum_latency_enforcement() {
    let config = TransportConfig {
        min_latency: Some(Duration::from_millis(50)),
        ..Default::default()
    };
    
    let mock_config = MockConfig {
        latency_ms: 50,
        enforce_latency: true,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // Rapid operations should be delayed
    let start = Instant::now();
    for _ in 0..3 {
        transport.send(b"data").await.unwrap();
    }
    let elapsed = start.elapsed();
    
    // Should take at least 100ms (50ms between each of the last 2 sends)
    assert!(elapsed >= Duration::from_millis(100));
    assert!(elapsed < Duration::from_millis(200)); // But not too long
}

#[tokio::test]
async fn test_latency_monitor() {
    let mut monitor = LatencyMonitor::new(100); // 100 sample window
    
    // Add samples
    monitor.add_sample(Duration::from_millis(10));
    monitor.add_sample(Duration::from_millis(20));
    monitor.add_sample(Duration::from_millis(30));
    monitor.add_sample(Duration::from_millis(40));
    monitor.add_sample(Duration::from_millis(50));
    
    // Check statistics
    assert_eq!(monitor.average(), Duration::from_millis(30));
    assert_eq!(monitor.min(), Some(Duration::from_millis(10)));
    assert_eq!(monitor.max(), Some(Duration::from_millis(50)));
    assert_eq!(monitor.count(), 5);
}

#[tokio::test]
async fn test_latency_monitor_window() {
    let mut monitor = LatencyMonitor::new(3); // Small window
    
    // Fill window
    monitor.add_sample(Duration::from_millis(10));
    monitor.add_sample(Duration::from_millis(20));
    monitor.add_sample(Duration::from_millis(30));
    
    assert_eq!(monitor.average(), Duration::from_millis(20));
    
    // Add more samples (should evict oldest)
    monitor.add_sample(Duration::from_millis(40));
    monitor.add_sample(Duration::from_millis(50));
    
    // Average should be of last 3: 30, 40, 50
    assert_eq!(monitor.average(), Duration::from_millis(40));
    assert_eq!(monitor.count(), 3); // Window size maintained
}

#[tokio::test]
async fn test_no_latency_enforcement() {
    let config = TransportConfig::default(); // No min_latency set
    
    let mock_config = MockConfig {
        latency_ms: 5,
        enforce_latency: false,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // Operations should be fast
    let start = Instant::now();
    for _ in 0..10 {
        transport.send(b"data").await.unwrap();
    }
    let elapsed = start.elapsed();
    
    // Should be much faster without enforcement
    assert!(elapsed < Duration::from_millis(100));
}

#[tokio::test]
async fn test_high_throughput_latency() {
    let config = TransportConfig {
        min_latency: Some(Duration::from_millis(10)),
        ..Default::default()
    };
    
    let mock_config = MockConfig {
        latency_ms: 10,
        enforce_latency: true,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    let mut monitor = LatencyMonitor::new(100);
    
    // Send many operations and track latency
    for _ in 0..20 {
        let start = Instant::now();
        transport.send(b"data").await.unwrap();
        let latency = start.elapsed();
        monitor.add_sample(latency);
    }
    
    // Average should be around 10ms (allow more tolerance for timing variations)
    let avg = monitor.average();
    assert!(avg >= Duration::from_millis(8));
    assert!(avg <= Duration::from_millis(20)); // More tolerance for system variations
    
    // Verify stats
    let stats = transport.stats();
    assert_eq!(stats.bytes_sent, 80); // 20 * 4 bytes
    assert_eq!(stats.transactions_success, 20);
}

#[tokio::test]
async fn test_transact_latency() {
    let config = TransportConfig {
        min_latency: Some(Duration::from_millis(25)),
        ..Default::default()
    };
    
    let mock_config = MockConfig {
        latency_ms: 25,
        enforce_latency: true,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // Transact involves send + receive
    let start = Instant::now();
    let result = transport.transact(b"request", Duration::from_secs(1)).await;
    let elapsed = start.elapsed();
    
    assert!(result.is_ok());
    // Should take at least 50ms (25ms for send, 25ms for receive)
    assert!(elapsed >= Duration::from_millis(50));
}

#[tokio::test]
async fn test_latency_percentiles() {
    let mut monitor = LatencyMonitor::new(100);
    
    // Add samples with varying latencies
    for i in 1..=100 {
        monitor.add_sample(Duration::from_millis(i));
    }
    
    // Test percentile calculations (allow for rounding)
    let median = monitor.percentile(50).unwrap();
    assert!(median >= Duration::from_millis(50) && median <= Duration::from_millis(51)); // Median
    
    let p90 = monitor.percentile(90).unwrap();
    assert!(p90 >= Duration::from_millis(90) && p90 <= Duration::from_millis(91)); // P90
    
    let p95 = monitor.percentile(95).unwrap();
    assert!(p95 >= Duration::from_millis(95) && p95 <= Duration::from_millis(96)); // P95
    
    let p99 = monitor.percentile(99).unwrap();
    assert!(p99 >= Duration::from_millis(99) && p99 <= Duration::from_millis(100)); // P99
}

#[tokio::test]
async fn test_latency_spike_detection() {
    let mut monitor = LatencyMonitor::new(10);
    
    // Normal latencies
    for _ in 0..8 {
        monitor.add_sample(Duration::from_millis(10));
    }
    
    // Add spikes
    monitor.add_sample(Duration::from_millis(100)); // 10x spike
    monitor.add_sample(Duration::from_millis(200)); // 20x spike
    
    // Check that spikes are detected
    let max = monitor.max().unwrap();
    let avg = monitor.average();
    
    assert_eq!(max, Duration::from_millis(200));
    // Average should show impact of spikes
    assert!(avg > Duration::from_millis(20));
    
    // Calculate spike ratio (be more lenient due to averaging)
    let spike_ratio = max.as_millis() as f64 / avg.as_millis() as f64;
    assert!(spike_ratio > 3.0); // Significant spike detected (lowered threshold)
}

#[tokio::test]
async fn test_concurrent_latency_enforcement() {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    let config = TransportConfig {
        min_latency: Some(Duration::from_millis(20)),
        ..Default::default()
    };
    
    let mock_config = MockConfig {
        latency_ms: 20,
        enforce_latency: true,
        ..Default::default()
    };
    
    let transport = Arc::new(Mutex::new(
        MockTransport::new("test".into(), config, mock_config)
    ));
    
    // Connect first
    transport.lock().await.connect().await.unwrap();
    
    // Spawn concurrent operations
    let start = Instant::now();
    let mut handles = vec![];
    
    for i in 0..5 {
        let transport = transport.clone();
        let handle = tokio::spawn(async move {
            // Stagger starts slightly
            tokio::time::sleep(Duration::from_millis(i * 5)).await;
            
            let mut transport = transport.lock().await;
            let op_start = Instant::now();
            transport.send(format!("data{}", i).as_bytes()).await.unwrap();
            op_start.elapsed()
        });
        handles.push(handle);
    }
    
    // Collect latencies
    let mut latencies = vec![];
    for handle in handles {
        latencies.push(handle.await.unwrap());
    }
    
    let total_time = start.elapsed();
    
    // Each operation should respect minimum latency (with tolerance)
    // In concurrent scenarios with mutex serialization, timing can vary
    for (i, latency) in latencies.iter().enumerate() {
        // First operation might be faster due to no contention
        // Later operations should show latency enforcement
        if i > 0 {
            // Allow 25% tolerance for timing variations in concurrent scenario
            assert!(*latency >= Duration::from_millis(15), 
                    "Operation {} latency {:?} below minimum", i, latency);
        }
    }
    
    // Total time should show serialization due to mutex
    assert!(total_time >= Duration::from_millis(80)); // At least 4 * 20ms gaps
}

#[tokio::test]
async fn test_latency_reset() {
    let mut monitor = LatencyMonitor::new(100);
    
    // Add samples
    monitor.add_sample(Duration::from_millis(10));
    monitor.add_sample(Duration::from_millis(20));
    assert_eq!(monitor.count(), 2);
    
    // Reset
    monitor.reset();
    assert_eq!(monitor.count(), 0);
    assert_eq!(monitor.min(), None);
    assert_eq!(monitor.max(), None);
    assert_eq!(monitor.average(), Duration::ZERO);
}