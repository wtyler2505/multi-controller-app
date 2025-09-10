//! Telemetry channel implementation
//! 
//! A channel represents a single stream of telemetry data with its own
//! ring buffer, configuration, and statistics.

use crate::telemetry::{RingBuffer, TelemetrySample, SampleType, SampleStatistics};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};

/// Configuration for a telemetry channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    /// Channel name
    pub name: String,
    /// Buffer size (minimum 2000 samples)
    pub buffer_size: usize,
    /// Expected sample rate (Hz) for rate limiting
    pub sample_rate: f32,
    /// Sample type for this channel
    pub sample_type: SampleType,
}

impl Default for ChannelConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            buffer_size: 2000,
            sample_rate: 30.0,
            sample_type: SampleType::Float32,
        }
    }
}

/// A telemetry channel with its own ring buffer
pub struct TelemetryChannel {
    config: ChannelConfig,
    buffer: Arc<RingBuffer<TelemetrySample>>,
    stats: Arc<RwLock<ChannelStats>>,
    rate_limiter: Arc<RwLock<RateLimiter>>,
}

impl TelemetryChannel {
    /// Create a new telemetry channel
    pub fn new(config: ChannelConfig) -> Self {
        let buffer_size = config.buffer_size.max(2000); // Enforce minimum
        
        Self {
            buffer: Arc::new(RingBuffer::new(buffer_size)),
            stats: Arc::new(RwLock::new(ChannelStats::new(config.name.clone()))),
            rate_limiter: Arc::new(RwLock::new(RateLimiter::new(config.sample_rate))),
            config,
        }
    }
    
    /// Add a sample to the channel
    pub fn add_sample(&self, sample: TelemetrySample) {
        // Check rate limiting
        if !self.rate_limiter.write().should_accept() {
            self.stats.write().samples_dropped += 1;
            return;
        }
        
        // Type checking (optional, for safety)
        if sample.sample_type() != self.config.sample_type {
            self.stats.write().type_mismatches += 1;
            // Could still accept or convert, depending on policy
        }
        
        // Add to buffer
        self.buffer.push(sample);
        
        // Update stats
        let mut stats = self.stats.write();
        stats.total_samples += 1;
        stats.last_sample_time = SystemTime::now();
    }
    
    /// Add multiple samples at once
    pub fn add_samples(&self, samples: Vec<TelemetrySample>) {
        for sample in samples {
            self.add_sample(sample);
        }
    }
    
    /// Get current buffer snapshot
    pub fn snapshot(&self) -> Vec<TelemetrySample> {
        self.buffer.snapshot()
    }
    
    /// Get last N samples
    pub fn last_n(&self, n: usize) -> Vec<TelemetrySample> {
        self.buffer.last_n(n)
    }
    
    /// Get samples for charting (returns f32 values only)
    pub fn chart_data(&self, max_points: usize) -> Vec<(u64, f32)> {
        let samples = self.buffer.snapshot();
        
        // Decimate if necessary
        let samples = if samples.len() > max_points {
            decimate_samples(&samples, max_points)
        } else {
            samples
        };
        
        samples
            .iter()
            .filter_map(|s| s.as_f32().map(|v| (s.timestamp_ms, v)))
            .collect()
    }
    
    /// Get channel configuration
    pub fn config(&self) -> &ChannelConfig {
        &self.config
    }
    
    /// Get channel statistics
    pub fn get_stats(&self) -> ChannelStats {
        let mut stats = self.stats.write().clone();
        
        // Update buffer stats
        let buffer_stats = self.buffer.stats();
        stats.buffer_capacity = buffer_stats.capacity;
        stats.buffer_used = buffer_stats.current_size;
        stats.buffer_fill_ratio = buffer_stats.fill_ratio();
        stats.memory_bytes = buffer_stats.memory_bytes;
        
        // Calculate sample statistics
        let samples = self.buffer.snapshot();
        stats.sample_stats = Some(SampleStatistics::from_samples(&samples));
        
        stats
    }
    
    /// Clear all data in the channel
    pub fn clear(&self) {
        self.buffer.clear();
        self.stats.write().reset();
    }
    
    /// Export channel data
    pub fn export_data(&self) -> ChannelExportData {
        ChannelExportData {
            config: self.config.clone(),
            samples: self.buffer.snapshot(),
            stats: self.get_stats(),
            exported_at: SystemTime::now(),
        }
    }
    
    /// Get memory usage
    pub fn memory_usage(&self) -> usize {
        self.buffer.memory_usage()
    }
    
    /// Prune oldest samples by percentage
    pub fn prune_oldest(&self, percentage: u8) {
        self.buffer.prune_oldest(percentage);
    }
    
    /// Set sample rate for rate limiting
    pub fn set_sample_rate(&self, rate_hz: f32) {
        self.rate_limiter.write().set_rate(rate_hz);
    }
}

/// Channel statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelStats {
    pub name: String,
    pub total_samples: u64,
    pub samples_dropped: u64,
    pub type_mismatches: u64,
    pub buffer_capacity: usize,
    pub buffer_used: usize,
    pub buffer_fill_ratio: f32,
    pub memory_bytes: usize,
    pub created_at: SystemTime,
    pub last_sample_time: SystemTime,
    pub sample_stats: Option<SampleStatistics>,
}

impl ChannelStats {
    pub(crate) fn new(name: String) -> Self {
        let now = SystemTime::now();
        Self {
            name,
            total_samples: 0,
            samples_dropped: 0,
            type_mismatches: 0,
            buffer_capacity: 0,
            buffer_used: 0,
            buffer_fill_ratio: 0.0,
            memory_bytes: 0,
            created_at: now,
            last_sample_time: now,
            sample_stats: None,
        }
    }
    
    fn reset(&mut self) {
        self.total_samples = 0;
        self.samples_dropped = 0;
        self.type_mismatches = 0;
        self.buffer_used = 0;
        self.buffer_fill_ratio = 0.0;
        self.sample_stats = None;
    }
    
    /// Get effective sample rate
    pub fn effective_sample_rate(&self) -> Option<f32> {
        if self.total_samples <= 1 {
            return None;
        }
        
        if let Ok(elapsed) = self.created_at.elapsed() {
            let seconds = elapsed.as_secs_f32();
            if seconds > 0.0 {
                return Some(self.total_samples as f32 / seconds);
            }
        }
        
        None
    }
    
    /// Get drop rate percentage
    pub fn drop_rate(&self) -> f32 {
        let total = self.total_samples + self.samples_dropped;
        if total == 0 {
            return 0.0;
        }
        (self.samples_dropped as f32 / total as f32) * 100.0
    }
}

/// Data structure for exporting channel data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelExportData {
    pub config: ChannelConfig,
    pub samples: Vec<TelemetrySample>,
    pub stats: ChannelStats,
    pub exported_at: SystemTime,
}

/// Rate limiter for controlling sample rate
struct RateLimiter {
    rate_hz: f32,
    min_interval_ns: u64,
    last_accept_ns: u64,
}

impl RateLimiter {
    fn new(rate_hz: f32) -> Self {
        let min_interval_ns = if rate_hz > 0.0 {
            (1_000_000_000.0 / rate_hz) as u64
        } else {
            0
        };
        
        Self {
            rate_hz,
            min_interval_ns,
            last_accept_ns: 0,
        }
    }
    
    fn should_accept(&mut self) -> bool {
        if self.min_interval_ns == 0 {
            return true; // No rate limiting
        }
        
        let now_ns = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        if now_ns - self.last_accept_ns >= self.min_interval_ns {
            self.last_accept_ns = now_ns;
            true
        } else {
            false
        }
    }
    
    fn set_rate(&mut self, rate_hz: f32) {
        self.rate_hz = rate_hz;
        self.min_interval_ns = if rate_hz > 0.0 {
            (1_000_000_000.0 / rate_hz) as u64
        } else {
            0
        };
    }
}

/// Decimate samples for visualization
fn decimate_samples(samples: &[TelemetrySample], target_count: usize) -> Vec<TelemetrySample> {
    if samples.is_empty() || target_count == 0 {
        return Vec::new();
    }
    
    if samples.len() <= target_count {
        return samples.to_vec();
    }
    
    // Use LTTB (Largest Triangle Three Buckets) algorithm for better visualization
    // For now, simple decimation by taking every Nth sample
    let step = samples.len() / target_count;
    samples
        .iter()
        .step_by(step.max(1))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::SampleValue;
    
    #[test]
    fn test_channel_creation() {
        let config = ChannelConfig {
            name: "test".to_string(),
            buffer_size: 100,
            sample_rate: 10.0,
            sample_type: SampleType::Float32,
        };
        
        let channel = TelemetryChannel::new(config);
        assert_eq!(channel.config().name, "test");
        assert_eq!(channel.get_stats().buffer_capacity, 2000); // Minimum enforced
    }
    
    #[test]
    fn test_channel_add_samples() {
        let mut config = ChannelConfig::default();
        config.sample_rate = 0.0; // Disable rate limiting for test
        let channel = TelemetryChannel::new(config);
        
        for i in 0..10 {
            channel.add_sample(TelemetrySample::new_f32(i as f32));
        }
        
        let stats = channel.get_stats();
        assert_eq!(stats.total_samples, 10);
        assert_eq!(stats.buffer_used, 10);
        
        let snapshot = channel.snapshot();
        assert_eq!(snapshot.len(), 10);
    }
    
    #[test]
    fn test_rate_limiting() {
        let mut config = ChannelConfig::default();
        config.sample_rate = 1000.0; // Very high rate for testing
        
        let channel = TelemetryChannel::new(config);
        
        // Add samples rapidly
        for i in 0..100 {
            channel.add_sample(TelemetrySample::new_f32(i as f32));
            std::thread::sleep(Duration::from_micros(100)); // Much faster than rate limit
        }
        
        let stats = channel.get_stats();
        // Some samples should be dropped due to rate limiting
        assert!(stats.samples_dropped > 0 || stats.total_samples < 100);
    }
    
    #[test]
    fn test_chart_data() {
        let channel = TelemetryChannel::new(ChannelConfig::default());
        
        for i in 0..50 {
            channel.add_sample(TelemetrySample::new_f32(i as f32));
        }
        
        let chart_data = channel.chart_data(10);
        assert!(chart_data.len() <= 10);
        
        for (timestamp, value) in chart_data {
            assert!(timestamp > 0);
            assert!(value >= 0.0 && value < 50.0);
        }
    }
}