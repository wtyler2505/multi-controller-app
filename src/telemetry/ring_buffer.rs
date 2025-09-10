//! Lock-free ring buffer implementation for telemetry data
//! 
//! Provides a high-performance, thread-safe ring buffer with support for
//! multiple concurrent readers and a single writer.

use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::Arc;
use parking_lot::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

/// A fixed-size ring buffer for telemetry data
/// 
/// This implementation uses atomic operations for the write position
/// and RwLock for the data buffer to support concurrent access.
pub struct RingBuffer<T: Clone + Send + Sync> {
    /// The underlying data buffer
    buffer: Arc<RwLock<Vec<Option<T>>>>,
    /// Current write position (head)
    write_pos: AtomicUsize,
    /// Total number of items written (for wraparound detection)
    total_written: AtomicU64,
    /// Buffer capacity
    capacity: usize,
    /// Creation timestamp
    created_at: SystemTime,
    /// Last write timestamp
    last_write: AtomicU64,
}

impl<T: Clone + Send + Sync> RingBuffer<T> {
    /// Create a new ring buffer with specified capacity
    /// 
    /// # Arguments
    /// * `capacity` - The fixed size of the buffer (minimum 2000 for telemetry)
    /// 
    /// # Panics
    /// Panics if capacity is 0
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Ring buffer capacity must be greater than 0");
        
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(None);
        }
        
        Self {
            buffer: Arc::new(RwLock::new(buffer)),
            write_pos: AtomicUsize::new(0),
            total_written: AtomicU64::new(0),
            capacity,
            created_at: SystemTime::now(),
            last_write: AtomicU64::new(0),
        }
    }
    
    /// Push a new value into the buffer
    /// 
    /// This will overwrite the oldest value if the buffer is full.
    /// Thread-safe for single writer.
    pub fn push(&self, value: T) {
        let pos = self.write_pos.fetch_add(1, Ordering::AcqRel) % self.capacity;
        
        {
            let mut buffer = self.buffer.write();
            buffer[pos] = Some(value);
        }
        
        self.total_written.fetch_add(1, Ordering::Relaxed);
        self.last_write.store(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            Ordering::Relaxed
        );
    }
    
    /// Push multiple values at once (batch operation)
    /// 
    /// More efficient than multiple individual pushes
    pub fn push_batch(&self, values: &[T]) {
        if values.is_empty() {
            return;
        }
        
        let start_pos = self.write_pos.fetch_add(values.len(), Ordering::AcqRel);
        
        {
            let mut buffer = self.buffer.write();
            for (i, value) in values.iter().enumerate() {
                let pos = (start_pos + i) % self.capacity;
                buffer[pos] = Some(value.clone());
            }
        }
        
        self.total_written.fetch_add(values.len() as u64, Ordering::Relaxed);
        self.last_write.store(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            Ordering::Relaxed
        );
    }
    
    /// Get the current number of valid items in the buffer
    pub fn len(&self) -> usize {
        let total = self.total_written.load(Ordering::Relaxed);
        std::cmp::min(total as usize, self.capacity)
    }
    
    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.total_written.load(Ordering::Relaxed) == 0
    }
    
    /// Check if the buffer is full
    pub fn is_full(&self) -> bool {
        self.total_written.load(Ordering::Relaxed) >= self.capacity as u64
    }
    
    /// Get the buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get a snapshot of all valid data in the buffer
    /// 
    /// Returns data in chronological order (oldest first)
    pub fn snapshot(&self) -> Vec<T> {
        let buffer = self.buffer.read();
        let total = self.total_written.load(Ordering::Relaxed);
        
        if total == 0 {
            return Vec::new();
        }
        
        let mut result = Vec::with_capacity(self.len());
        
        if total < self.capacity as u64 {
            // Buffer not yet full, data is from 0 to write_pos
            for i in 0..total as usize {
                if let Some(ref value) = buffer[i] {
                    result.push(value.clone());
                }
            }
        } else {
            // Buffer has wrapped, start from write_pos (oldest data)
            let write_pos = self.write_pos.load(Ordering::Acquire) % self.capacity;
            
            for i in 0..self.capacity {
                let pos = (write_pos + i) % self.capacity;
                if let Some(ref value) = buffer[pos] {
                    result.push(value.clone());
                }
            }
        }
        
        result
    }
    
    /// Get the last N samples (most recent)
    pub fn last_n(&self, n: usize) -> Vec<T> {
        let snapshot = self.snapshot();
        let len = snapshot.len();
        
        if n >= len {
            snapshot
        } else {
            snapshot[len - n..].to_vec()
        }
    }
    
    /// Get samples within a time window (requires T to have timestamp)
    pub fn window(&self, start_idx: usize, end_idx: usize) -> Vec<T> {
        let snapshot = self.snapshot();
        let len = snapshot.len();
        
        if start_idx >= len {
            return Vec::new();
        }
        
        let end = std::cmp::min(end_idx, len);
        snapshot[start_idx..end].to_vec()
    }
    
    /// Clear the buffer
    pub fn clear(&self) {
        let mut buffer = self.buffer.write();
        for item in buffer.iter_mut() {
            *item = None;
        }
        
        self.write_pos.store(0, Ordering::Release);
        self.total_written.store(0, Ordering::Release);
    }
    
    /// Get buffer statistics
    pub fn stats(&self) -> RingBufferStats {
        RingBufferStats {
            capacity: self.capacity,
            current_size: self.len(),
            total_written: self.total_written.load(Ordering::Relaxed),
            is_full: self.is_full(),
            created_at: self.created_at,
            last_write_ms: self.last_write.load(Ordering::Relaxed),
            memory_bytes: self.memory_usage(),
        }
    }
    
    /// Estimate memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of::<T>() * self.capacity
            + std::mem::size_of::<Self>()
            + std::mem::size_of::<Option<T>>() * self.capacity
    }
    
    /// Remove oldest n% of data
    pub fn prune_oldest(&self, percentage: u8) {
        if percentage == 0 || percentage > 100 {
            return;
        }
        
        let to_remove = (self.len() * percentage as usize) / 100;
        if to_remove == 0 {
            return;
        }
        
        let snapshot = self.snapshot();
        if snapshot.len() <= to_remove {
            self.clear();
            return;
        }
        
        // Clear and re-add remaining data
        self.clear();
        for item in &snapshot[to_remove..] {
            self.push(item.clone());
        }
    }
}

/// Statistics for a ring buffer
#[derive(Debug, Clone)]
pub struct RingBufferStats {
    pub capacity: usize,
    pub current_size: usize,
    pub total_written: u64,
    pub is_full: bool,
    pub created_at: SystemTime,
    pub last_write_ms: u64,
    pub memory_bytes: usize,
}

impl RingBufferStats {
    /// Get fill percentage (0.0 to 1.0)
    pub fn fill_ratio(&self) -> f32 {
        if self.capacity == 0 {
            return 0.0;
        }
        (self.current_size as f32) / (self.capacity as f32)
    }
    
    /// Get write rate (samples per second) over lifetime
    pub fn average_write_rate(&self) -> f32 {
        if let Ok(elapsed) = self.created_at.elapsed() {
            let seconds = elapsed.as_secs_f32();
            if seconds > 0.0 {
                return self.total_written as f32 / seconds;
            }
        }
        0.0
    }
}

// Thread-safety marker
unsafe impl<T: Clone + Send + Sync> Send for RingBuffer<T> {}
unsafe impl<T: Clone + Send + Sync> Sync for RingBuffer<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::Arc;
    
    #[test]
    fn test_ring_buffer_basic() {
        let buffer = RingBuffer::<f32>::new(10);
        
        assert_eq!(buffer.capacity(), 10);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());
        
        // Push some values
        for i in 0..5 {
            buffer.push(i as f32);
        }
        
        assert_eq!(buffer.len(), 5);
        assert!(!buffer.is_empty());
        assert!(!buffer.is_full());
        
        // Fill the buffer
        for i in 5..10 {
            buffer.push(i as f32);
        }
        
        assert_eq!(buffer.len(), 10);
        assert!(buffer.is_full());
        
        // Overwrite oldest
        buffer.push(10.0);
        assert_eq!(buffer.len(), 10);
        assert!(buffer.is_full());
        
        let snapshot = buffer.snapshot();
        assert_eq!(snapshot[0], 1.0); // 0.0 was overwritten
        assert_eq!(snapshot[9], 10.0);
    }
    
    #[test]
    fn test_ring_buffer_wraparound() {
        let buffer = RingBuffer::<i32>::new(5);
        
        // Fill buffer beyond capacity
        for i in 0..12 {
            buffer.push(i);
        }
        
        let snapshot = buffer.snapshot();
        assert_eq!(snapshot.len(), 5);
        assert_eq!(snapshot, vec![7, 8, 9, 10, 11]);
    }
    
    #[test]
    fn test_ring_buffer_concurrent() {
        let buffer = Arc::new(RingBuffer::<i32>::new(1000));
        let mut handles = vec![];
        
        // Multiple readers
        for _ in 0..5 {
            let buf = buffer.clone();
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    let _ = buf.snapshot();
                    let _ = buf.stats();
                }
            }));
        }
        
        // Single writer
        let buf = buffer.clone();
        handles.push(thread::spawn(move || {
            for i in 0..1000 {
                buf.push(i);
            }
        }));
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(buffer.len(), 1000);
    }
    
    #[test]
    fn test_batch_operations() {
        let buffer = RingBuffer::<i32>::new(10);
        
        let batch = vec![1, 2, 3, 4, 5];
        buffer.push_batch(&batch);
        
        assert_eq!(buffer.len(), 5);
        assert_eq!(buffer.snapshot(), vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_last_n() {
        let buffer = RingBuffer::<i32>::new(10);
        
        for i in 0..8 {
            buffer.push(i);
        }
        
        assert_eq!(buffer.last_n(3), vec![5, 6, 7]);
        assert_eq!(buffer.last_n(20), (0..8).collect::<Vec<_>>());
    }
    
    #[test]
    fn test_pruning() {
        let buffer = RingBuffer::<i32>::new(10);
        
        for i in 0..10 {
            buffer.push(i);
        }
        
        buffer.prune_oldest(30); // Remove 30% (3 items)
        
        assert_eq!(buffer.len(), 7);
        assert_eq!(buffer.snapshot(), vec![3, 4, 5, 6, 7, 8, 9]);
    }
}