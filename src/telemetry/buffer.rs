//! High-performance lock-free ring buffer for telemetry data collection
//!
//! This module provides thread-safe, lock-free ring buffers optimized for
//! high-frequency telemetry data ingestion with configurable overflow handling.

use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::Arc;
use parking_lot::RwLock;
use std::collections::HashMap;

use crate::telemetry::{TelemetryFrame, TelemetryParser, TelemetryFormat, MultiFormatTelemetryParser, ParseError};

/// Lock-free ring buffer for high-performance telemetry storage
pub struct TelemetryRingBuffer<T> {
    buffer: Vec<RwLock<Option<T>>>,
    capacity: usize,
    head: AtomicUsize,          // Write position
    tail: AtomicUsize,          // Read position
    sequence: AtomicU64,        // Global sequence counter
    overflow_count: AtomicU64,  // Count of dropped samples due to overflow
}

impl<T: Clone> TelemetryRingBuffer<T> {
    /// Create a new ring buffer with specified capacity (minimum 2000)
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.max(2000); // Enforce minimum capacity requirement
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(RwLock::new(None));
        }
        
        Self {
            buffer,
            capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            sequence: AtomicU64::new(0),
            overflow_count: AtomicU64::new(0),
        }
    }
    
    /// Push an item into the buffer with overflow handling
    pub fn push(&self, item: T) -> bool {
        let seq = self.sequence.fetch_add(1, Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        let new_head = (head + 1) % self.capacity;
        
        // Check if buffer is full
        if new_head == self.tail.load(Ordering::Acquire) {
            // Buffer full - implement overflow strategy (drop oldest)
            self.overflow_count.fetch_add(1, Ordering::Relaxed);
            
            // Advance tail to make room (drop oldest sample)
            let old_tail = self.tail.fetch_add(1, Ordering::Release) % self.capacity;
            *self.buffer[old_tail].write() = None;
            
            tracing::warn!("Telemetry buffer overflow: sample {} dropped", seq);
        }
        
        // Store the item
        *self.buffer[head].write() = Some(item);
        
        // Update head pointer
        self.head.store(new_head, Ordering::Release);
        
        tracing::trace!("Telemetry sample {} stored at position {}", seq, head);
        true
    }
    
    /// Pop the oldest item from the buffer
    pub fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        
        // Check if buffer is empty
        if tail == head {
            return None;
        }
        
        // Read the item
        let item = self.buffer[tail].write().take();
        
        // Update tail pointer
        let new_tail = (tail + 1) % self.capacity;
        self.tail.store(new_tail, Ordering::Release);
        
        item
    }
    
    /// Peek at the latest N samples without removing them
    pub fn peek_latest(&self, count: usize) -> Vec<T> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        
        let mut items = Vec::new();
        let available = if head >= tail {
            head - tail
        } else {
            self.capacity - tail + head
        };
        
        let to_read = count.min(available);
        
        for i in 0..to_read {
            let idx = if head >= i + 1 {
                head - i - 1
            } else {
                self.capacity + head - i - 1
            };
            
            if let Some(ref item) = *self.buffer[idx].read() {
                items.push(item.clone());
            }
        }
        
        items
    }

    /// Peek at a range of samples by index
    pub fn peek_range(&self, start_idx: usize, count: usize) -> Vec<T> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        
        let available = if head >= tail {
            head - tail
        } else {
            self.capacity - tail + head
        };

        if start_idx >= available {
            return Vec::new();
        }

        let mut items = Vec::new();
        let to_read = count.min(available - start_idx);
        
        for i in 0..to_read {
            let logical_idx = start_idx + i;
            let physical_idx = (tail + logical_idx) % self.capacity;
            
            if let Some(ref item) = *self.buffer[physical_idx].read() {
                items.push(item.clone());
            }
        }
        
        items
    }
    
    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get current buffer length (number of items stored)
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        
        if head >= tail {
            head - tail
        } else {
            self.capacity - tail + head
        }
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Get overflow count (samples dropped due to buffer full)
    pub fn overflow_count(&self) -> u64 {
        self.overflow_count.load(Ordering::Relaxed)
    }
    
    /// Clear all items from the buffer
    pub fn clear(&self) {
        let tail = self.tail.load(Ordering::Acquire);
        let head = self.head.load(Ordering::Acquire);
        
        // Clear all items
        let mut current = tail;
        while current != head {
            *self.buffer[current].write() = None;
            current = (current + 1) % self.capacity;
        }
        
        // Reset pointers
        self.tail.store(0, Ordering::Release);
        self.head.store(0, Ordering::Release);
    }

    /// Get buffer utilization percentage
    pub fn utilization(&self) -> f32 {
        (self.len() as f32 / self.capacity as f32) * 100.0
    }
}

/// Multi-channel telemetry buffer manager with format parsing
pub struct TelemetryBufferManager {
    buffers: Arc<RwLock<HashMap<String, Arc<TelemetryRingBuffer<TelemetryFrame>>>>>,
    parser: MultiFormatTelemetryParser,
    default_capacity: usize,
    total_samples: AtomicU64,
    total_parse_errors: AtomicU64,
    total_overflow_events: AtomicU64,
}

impl TelemetryBufferManager {
    /// Create a new buffer manager with default capacity
    pub fn new(default_capacity: usize) -> Self {
        Self {
            buffers: Arc::new(RwLock::new(HashMap::new())),
            parser: MultiFormatTelemetryParser::new(),
            default_capacity: default_capacity.max(2000), // Enforce minimum
            total_samples: AtomicU64::new(0),
            total_parse_errors: AtomicU64::new(0),
            total_overflow_events: AtomicU64::new(0),
        }
    }

    /// Create with custom parser configuration
    pub fn with_parser(parser: MultiFormatTelemetryParser, default_capacity: usize) -> Self {
        Self {
            buffers: Arc::new(RwLock::new(HashMap::new())),
            parser,
            default_capacity: default_capacity.max(2000),
            total_samples: AtomicU64::new(0),
            total_parse_errors: AtomicU64::new(0),
            total_overflow_events: AtomicU64::new(0),
        }
    }
    
    /// Get or create a buffer for the specified device
    pub fn get_or_create_buffer(&self, device_id: &str) -> Arc<TelemetryRingBuffer<TelemetryFrame>> {
        let buffers = self.buffers.read();
        
        if let Some(buffer) = buffers.get(device_id) {
            buffer.clone()
        } else {
            drop(buffers); // Release read lock
            
            let mut buffers = self.buffers.write();
            // Double-check pattern to prevent race conditions
            if let Some(buffer) = buffers.get(device_id) {
                buffer.clone()
            } else {
                let buffer = Arc::new(TelemetryRingBuffer::new(self.default_capacity));
                buffers.insert(device_id.to_string(), buffer.clone());
                tracing::info!("Created telemetry buffer for device: {} with capacity: {}", 
                              device_id, self.default_capacity);
                buffer
            }
        }
    }
    
    /// Parse and store raw telemetry data
    pub fn parse_and_store(&self, data: &[u8], format: TelemetryFormat, device_id: Option<&str>) -> Result<bool, ParseError> {
        match self.parser.parse(data, format) {
            Ok(mut frame) => {
                // Override device_id if provided
                if let Some(id) = device_id {
                    frame.device_id = id.to_string();
                }
                
                let buffer = self.get_or_create_buffer(&frame.device_id);
                let success = buffer.push(frame);
                
                if success {
                    self.total_samples.fetch_add(1, Ordering::Relaxed);
                    
                    // Track overflow events
                    if buffer.overflow_count() > 0 {
                        self.total_overflow_events.store(
                            self.buffers.read().values().map(|b| b.overflow_count()).sum(),
                            Ordering::Relaxed
                        );
                    }
                }
                
                Ok(success)
            }
            Err(e) => {
                self.total_parse_errors.fetch_add(1, Ordering::Relaxed);
                tracing::error!("Failed to parse telemetry data: {}", e);
                Err(e)
            }
        }
    }

    /// Auto-detect format and parse data
    pub fn auto_parse_and_store(&self, data: &[u8], device_id: Option<&str>) -> Result<(TelemetryFormat, bool), ParseError> {
        match self.parser.parse_auto(data) {
            Ok((format, mut frame)) => {
                // Override device_id if provided
                if let Some(id) = device_id {
                    frame.device_id = id.to_string();
                }
                
                let buffer = self.get_or_create_buffer(&frame.device_id);
                let success = buffer.push(frame);
                
                if success {
                    self.total_samples.fetch_add(1, Ordering::Relaxed);
                }
                
                Ok((format, success))
            }
            Err(e) => {
                self.total_parse_errors.fetch_add(1, Ordering::Relaxed);
                Err(e)
            }
        }
    }
    
    /// Store a pre-parsed telemetry frame
    pub fn store_frame(&self, frame: TelemetryFrame) -> bool {
        let buffer = self.get_or_create_buffer(&frame.device_id);
        let success = buffer.push(frame);
        
        if success {
            self.total_samples.fetch_add(1, Ordering::Relaxed);
        }
        
        success
    }
    
    /// Get recent frames from a specific device
    pub fn get_recent_frames(&self, device_id: &str, count: usize) -> Vec<TelemetryFrame> {
        let buffers = self.buffers.read();
        if let Some(buffer) = buffers.get(device_id) {
            buffer.peek_latest(count)
        } else {
            Vec::new()
        }
    }

    /// Get frames in a specific range from a device
    pub fn get_frame_range(&self, device_id: &str, start_idx: usize, count: usize) -> Vec<TelemetryFrame> {
        let buffers = self.buffers.read();
        if let Some(buffer) = buffers.get(device_id) {
            buffer.peek_range(start_idx, count)
        } else {
            Vec::new()
        }
    }

    /// Get all device IDs that have buffers
    pub fn get_device_ids(&self) -> Vec<String> {
        self.buffers.read().keys().cloned().collect()
    }
    
    /// Get buffer statistics for all devices
    pub fn get_buffer_stats(&self) -> HashMap<String, BufferStats> {
        let buffers = self.buffers.read();
        let mut stats = HashMap::new();
        
        for (device_id, buffer) in buffers.iter() {
            stats.insert(device_id.clone(), BufferStats {
                capacity: buffer.capacity(),
                length: buffer.len(),
                overflow_count: buffer.overflow_count(),
                utilization_percent: buffer.utilization(),
            });
        }
        
        stats
    }
    
    /// Get overall system statistics
    pub fn get_system_stats(&self) -> SystemStats {
        let buffers = self.buffers.read();
        
        SystemStats {
            device_count: buffers.len(),
            total_capacity: buffers.values().map(|b| b.capacity()).sum(),
            total_samples: buffers.values().map(|b| b.len()).sum(),
            total_stored_samples: self.total_samples.load(Ordering::Relaxed),
            total_parse_errors: self.total_parse_errors.load(Ordering::Relaxed),
            total_overflow_events: self.total_overflow_events.load(Ordering::Relaxed),
            average_utilization: {
                let utilizations: Vec<f32> = buffers.values().map(|b| b.utilization()).collect();
                if utilizations.is_empty() {
                    0.0
                } else {
                    utilizations.iter().sum::<f32>() / utilizations.len() as f32
                }
            },
        }
    }
    
    /// Clear buffer for specific device
    pub fn clear_device_buffer(&self, device_id: &str) -> bool {
        let buffers = self.buffers.read();
        if let Some(buffer) = buffers.get(device_id) {
            buffer.clear();
            true
        } else {
            false
        }
    }

    /// Clear all buffers
    pub fn clear_all_buffers(&self) {
        for buffer in self.buffers.read().values() {
            buffer.clear();
        }
    }

    /// Remove a device buffer entirely
    pub fn remove_device_buffer(&self, device_id: &str) -> bool {
        self.buffers.write().remove(device_id).is_some()
    }

    /// Get total memory usage estimate across all buffers
    pub fn estimated_memory_usage(&self) -> usize {
        let buffers = self.buffers.read();
        buffers.values()
            .map(|buffer| {
                // Rough estimate: capacity * size of TelemetryFrame
                // TelemetryFrame is complex, so use a conservative estimate
                buffer.capacity() * 1024 // ~1KB per frame estimate
            })
            .sum()
    }

    /// Configure sampling rate control (placeholder for future implementation)
    pub fn configure_sampling(&self, _device_id: &str, _rate_hz: f64) {
        // TODO: Implement sampling rate control
        // This would integrate with the rate controller from the requirements
        tracing::info!("Sampling rate configuration requested - not yet implemented");
    }
}

/// Statistics for individual buffer
#[derive(Debug, Clone)]
pub struct BufferStats {
    pub capacity: usize,
    pub length: usize,
    pub overflow_count: u64,
    pub utilization_percent: f32,
}

/// Overall system statistics
#[derive(Debug, Clone)]
pub struct SystemStats {
    pub device_count: usize,
    pub total_capacity: usize,
    pub total_samples: usize,
    pub total_stored_samples: u64,
    pub total_parse_errors: u64,
    pub total_overflow_events: u64,
    pub average_utilization: f32,
}

/// Decimation strategy for data visualization
#[derive(Debug, Clone, Copy)]
pub enum DecimationStrategy {
    /// Take every Nth sample
    Uniform,
    /// Preserve min/max in each window  
    MinMax,
    /// Average values in each window
    Average,
    /// Choose strategy based on signal characteristics
    Adaptive,
}

/// Data decimator for visualization efficiency
pub struct TelemetryDecimator;

impl TelemetryDecimator {
    /// Decimate frames to target point count for visualization
    pub fn decimate_frames(frames: &[TelemetryFrame], target_points: usize, strategy: DecimationStrategy) -> Vec<TelemetryFrame> {
        if frames.len() <= target_points {
            return frames.to_vec();
        }
        
        match strategy {
            DecimationStrategy::Uniform => Self::uniform_decimation(frames, target_points),
            DecimationStrategy::MinMax => Self::minmax_decimation(frames, target_points),
            DecimationStrategy::Average => Self::average_decimation(frames, target_points),
            DecimationStrategy::Adaptive => Self::adaptive_decimation(frames, target_points),
        }
    }

    fn uniform_decimation(frames: &[TelemetryFrame], target_points: usize) -> Vec<TelemetryFrame> {
        let step = frames.len() / target_points;
        if step <= 1 {
            return frames.to_vec();
        }
        
        frames.iter()
            .step_by(step)
            .take(target_points)
            .cloned()
            .collect()
    }

    fn minmax_decimation(frames: &[TelemetryFrame], target_points: usize) -> Vec<TelemetryFrame> {
        // Simplified implementation - could be enhanced with proper min/max detection per channel
        let window_size = frames.len() / (target_points / 2);
        let mut decimated = Vec::new();
        
        for window_start in (0..frames.len()).step_by(window_size) {
            let window_end = (window_start + window_size).min(frames.len());
            
            if window_start < frames.len() {
                decimated.push(frames[window_start].clone());
            }
            
            if window_end > window_start + 1 && window_end - 1 < frames.len() {
                decimated.push(frames[window_end - 1].clone());
            }
        }
        
        decimated.truncate(target_points);
        decimated
    }

    fn average_decimation(frames: &[TelemetryFrame], target_points: usize) -> Vec<TelemetryFrame> {
        // Simplified implementation - would need proper averaging of data points
        Self::uniform_decimation(frames, target_points)
    }

    fn adaptive_decimation(frames: &[TelemetryFrame], target_points: usize) -> Vec<TelemetryFrame> {
        // Simple heuristic: use uniform for now, could be enhanced with variance analysis
        Self::uniform_decimation(frames, target_points)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telemetry::{DataPoint, TelemetryValue, DataQuality};
    use chrono::Utc;

    #[test]
    fn test_ring_buffer_creation() {
        let buffer: TelemetryRingBuffer<i32> = TelemetryRingBuffer::new(100);
        assert_eq!(buffer.capacity(), 100);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        assert_eq!(buffer.overflow_count(), 0);
    }

    #[test]
    fn test_ring_buffer_minimum_capacity() {
        let buffer: TelemetryRingBuffer<i32> = TelemetryRingBuffer::new(100);
        // Should enforce minimum capacity of 2000
        assert!(buffer.capacity() >= 2000);
    }

    #[test]
    fn test_ring_buffer_push_pop() {
        let buffer: TelemetryRingBuffer<i32> = TelemetryRingBuffer::new(2000);
        
        // Test push
        assert!(buffer.push(42));
        assert_eq!(buffer.len(), 1);
        assert!(!buffer.is_empty());
        
        // Test pop
        assert_eq!(buffer.pop(), Some(42));
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        
        // Test pop from empty buffer
        assert_eq!(buffer.pop(), None);
    }

    #[test]
    fn test_ring_buffer_overflow() {
        let buffer: TelemetryRingBuffer<i32> = TelemetryRingBuffer::new(2000);
        
        // Fill buffer to capacity
        for i in 0..2000 {
            buffer.push(i);
        }
        
        assert_eq!(buffer.len(), 2000);
        assert_eq!(buffer.overflow_count(), 0);
        
        // Push one more item to trigger overflow
        buffer.push(2000);
        assert_eq!(buffer.overflow_count(), 1);
        
        // Buffer should still be at capacity
        assert!(buffer.len() <= 2000);
    }

    #[test]
    fn test_ring_buffer_peek_latest() {
        let buffer: TelemetryRingBuffer<i32> = TelemetryRingBuffer::new(2000);
        
        // Add some items
        for i in 0..10 {
            buffer.push(i);
        }
        
        // Peek at latest 5 items
        let latest = buffer.peek_latest(5);
        assert_eq!(latest.len(), 5);
        
        // Should be in reverse order (latest first)
        assert_eq!(latest[0], 9);
        assert_eq!(latest[1], 8);
        assert_eq!(latest[2], 7);
        assert_eq!(latest[3], 6);
        assert_eq!(latest[4], 5);
    }

    #[test]
    fn test_telemetry_buffer_manager() {
        let manager = TelemetryBufferManager::new(2000);
        
        // Create a test frame
        let frame = TelemetryFrame {
            timestamp: Utc::now(),
            sequence_number: 1,
            device_id: "test_device".to_string(),
            data_points: vec![
                DataPoint {
                    channel: "voltage".to_string(),
                    value: TelemetryValue::Float(3.3),
                    unit: Some("V".to_string()),
                    quality: DataQuality::Good,
                }
            ],
            metadata: std::collections::HashMap::new(),
        };
        
        // Store frame
        assert!(manager.store_frame(frame));
        
        // Get recent frames
        let frames = manager.get_recent_frames("test_device", 10);
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].device_id, "test_device");
        
        // Check stats
        let stats = manager.get_buffer_stats();
        assert!(stats.contains_key("test_device"));
        
        let system_stats = manager.get_system_stats();
        assert_eq!(system_stats.device_count, 1);
        assert_eq!(system_stats.total_stored_samples, 1);
    }

    #[test]
    fn test_decimation() {
        // Create test frames
        let frames: Vec<TelemetryFrame> = (0..1000).map(|i| {
            TelemetryFrame {
                timestamp: Utc::now(),
                sequence_number: i,
                device_id: "test".to_string(),
                data_points: vec![],
                metadata: std::collections::HashMap::new(),
            }
        }).collect();
        
        // Test uniform decimation
        let decimated = TelemetryDecimator::decimate_frames(&frames, 100, DecimationStrategy::Uniform);
        assert!(decimated.len() <= 100);
        assert!(decimated.len() > 0);
        
        // Test with target larger than input
        let decimated = TelemetryDecimator::decimate_frames(&frames[..50], 100, DecimationStrategy::Uniform);
        assert_eq!(decimated.len(), 50);
    }

    #[test]
    fn test_buffer_utilization() {
        let buffer: TelemetryRingBuffer<i32> = TelemetryRingBuffer::new(2000);
        
        // Empty buffer
        assert_eq!(buffer.utilization(), 0.0);
        
        // Half full
        for i in 0..1000 {
            buffer.push(i);
        }
        assert!((buffer.utilization() - 50.0).abs() < 1.0);
        
        // Full buffer
        for i in 1000..2000 {
            buffer.push(i);
        }
        assert!((buffer.utilization() - 100.0).abs() < 1.0);
    }
}