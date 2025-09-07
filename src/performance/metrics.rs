// System and process metrics collection

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// System-wide metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// Timestamp in milliseconds since Unix epoch
    pub timestamp: u64,
    
    /// Total system memory in bytes
    pub total_memory: u64,
    
    /// Available system memory in bytes
    pub available_memory: u64,
    
    /// CPU usage percentage (0-100)
    pub cpu_usage: f32,
    
    /// Number of CPU cores
    pub cpu_cores: usize,
    
    /// System uptime in seconds
    pub uptime_seconds: u64,
}

impl SystemMetrics {
    /// Create metrics with current timestamp
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        Self {
            timestamp,
            total_memory: 0,
            available_memory: 0,
            cpu_usage: 0.0,
            cpu_cores: num_cpus::get(),
            uptime_seconds: 0,
        }
    }
    
    /// Calculate memory usage percentage
    pub fn memory_usage_percent(&self) -> f32 {
        if self.total_memory == 0 {
            return 0.0;
        }
        let used = self.total_memory - self.available_memory;
        (used as f32 / self.total_memory as f32) * 100.0
    }
}

/// Process-specific metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMetrics {
    /// Timestamp in milliseconds since Unix epoch
    pub timestamp: u64,
    
    /// Process ID
    pub pid: u32,
    
    /// Process memory usage in bytes (RSS)
    pub memory_bytes: u64,
    
    /// Virtual memory size in bytes
    pub virtual_memory_bytes: u64,
    
    /// CPU usage percentage (0-100)
    pub cpu_percent: f32,
    
    /// Number of threads
    pub thread_count: u32,
    
    /// Process uptime in seconds
    pub uptime_seconds: u64,
}

impl ProcessMetrics {
    /// Create metrics with current timestamp
    pub fn new(pid: u32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        Self {
            timestamp,
            pid,
            memory_bytes: 0,
            virtual_memory_bytes: 0,
            cpu_percent: 0.0,
            thread_count: 0,
            uptime_seconds: 0,
        }
    }
    
    /// Get memory usage in MB
    pub fn memory_mb(&self) -> f64 {
        self.memory_bytes as f64 / (1024.0 * 1024.0)
    }
    
    /// Get virtual memory in MB
    pub fn virtual_memory_mb(&self) -> f64 {
        self.virtual_memory_bytes as f64 / (1024.0 * 1024.0)
    }
}

/// Resource usage over time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    /// Memory usage samples (MB)
    pub memory_samples: Vec<f64>,
    
    /// CPU usage samples (%)
    pub cpu_samples: Vec<f32>,
    
    /// Timestamps for samples
    pub timestamps: Vec<u64>,
    
    /// Maximum observed memory (MB)
    pub peak_memory_mb: f64,
    
    /// Maximum observed CPU (%)
    pub peak_cpu_percent: f32,
    
    /// Average memory (MB)
    pub avg_memory_mb: f64,
    
    /// Average CPU (%)
    pub avg_cpu_percent: f32,
}

impl ResourceUsage {
    /// Create empty usage tracking
    pub fn new() -> Self {
        Self {
            memory_samples: Vec::new(),
            cpu_samples: Vec::new(),
            timestamps: Vec::new(),
            peak_memory_mb: 0.0,
            peak_cpu_percent: 0.0,
            avg_memory_mb: 0.0,
            avg_cpu_percent: 0.0,
        }
    }
    
    /// Add a sample
    pub fn add_sample(&mut self, memory_mb: f64, cpu_percent: f32) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        self.memory_samples.push(memory_mb);
        self.cpu_samples.push(cpu_percent);
        self.timestamps.push(timestamp);
        
        // Update peaks
        if memory_mb > self.peak_memory_mb {
            self.peak_memory_mb = memory_mb;
        }
        if cpu_percent > self.peak_cpu_percent {
            self.peak_cpu_percent = cpu_percent;
        }
        
        // Update averages
        self.update_averages();
    }
    
    /// Update running averages
    fn update_averages(&mut self) {
        if !self.memory_samples.is_empty() {
            let memory_sum: f64 = self.memory_samples.iter().sum();
            self.avg_memory_mb = memory_sum / self.memory_samples.len() as f64;
        }
        
        if !self.cpu_samples.is_empty() {
            let cpu_sum: f32 = self.cpu_samples.iter().sum();
            self.avg_cpu_percent = cpu_sum / self.cpu_samples.len() as f32;
        }
    }
    
    /// Get samples in time range
    pub fn samples_in_range(&self, start_ms: u64, end_ms: u64) -> (Vec<f64>, Vec<f32>) {
        let mut memory = Vec::new();
        let mut cpu = Vec::new();
        
        for (i, &timestamp) in self.timestamps.iter().enumerate() {
            if timestamp >= start_ms && timestamp <= end_ms {
                memory.push(self.memory_samples[i]);
                cpu.push(self.cpu_samples[i]);
            }
        }
        
        (memory, cpu)
    }
    
    /// Trim old samples to maintain fixed size
    pub fn trim_to_size(&mut self, max_samples: usize) {
        while self.memory_samples.len() > max_samples {
            self.memory_samples.remove(0);
            self.cpu_samples.remove(0);
            self.timestamps.remove(0);
        }
    }
}

impl Default for ResourceUsage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_metrics() {
        let metrics = SystemMetrics::new();
        assert!(metrics.timestamp > 0);
        assert!(metrics.cpu_cores > 0);
        assert_eq!(metrics.memory_usage_percent(), 0.0);
        
        let mut metrics = SystemMetrics::new();
        metrics.total_memory = 1000;
        metrics.available_memory = 400;
        assert!((metrics.memory_usage_percent() - 60.0).abs() < 0.01);
    }
    
    #[test]
    fn test_process_metrics() {
        let metrics = ProcessMetrics::new(1234);
        assert_eq!(metrics.pid, 1234);
        assert!(metrics.timestamp > 0);
        assert_eq!(metrics.memory_mb(), 0.0);
        
        let mut metrics = ProcessMetrics::new(1234);
        metrics.memory_bytes = 100 * 1024 * 1024; // 100 MB
        assert!((metrics.memory_mb() - 100.0).abs() < 0.01);
    }
    
    #[test]
    fn test_resource_usage() {
        let mut usage = ResourceUsage::new();
        assert!(usage.memory_samples.is_empty());
        
        usage.add_sample(100.0, 5.0);
        usage.add_sample(120.0, 7.0);
        usage.add_sample(110.0, 6.0);
        
        assert_eq!(usage.memory_samples.len(), 3);
        assert_eq!(usage.peak_memory_mb, 120.0);
        assert_eq!(usage.peak_cpu_percent, 7.0);
        assert!((usage.avg_memory_mb - 110.0).abs() < 0.01);
        assert!((usage.avg_cpu_percent - 6.0).abs() < 0.01);
    }
    
    #[test]
    fn test_resource_usage_trim() {
        let mut usage = ResourceUsage::new();
        
        for i in 0..10 {
            usage.add_sample(i as f64, i as f32);
        }
        
        assert_eq!(usage.memory_samples.len(), 10);
        
        usage.trim_to_size(5);
        assert_eq!(usage.memory_samples.len(), 5);
        assert_eq!(usage.memory_samples[0], 5.0); // First 5 removed
    }
}