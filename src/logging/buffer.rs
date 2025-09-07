// Rolling log buffer with fixed-size memory management

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

/// Log severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARN",
            LogLevel::Error => "ERROR",
            LogLevel::Fatal => "FATAL",
        }
    }
}

/// Individual log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp in milliseconds since Unix epoch
    pub timestamp: u64,
    
    /// Log level
    pub level: LogLevel,
    
    /// Source of the log (e.g., component name)
    pub source: String,
    
    /// Log message
    pub message: String,
    
    /// Optional binary data (e.g., raw device I/O)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<u8>>,
    
    /// Thread ID that created the log
    pub thread_id: String,
}

impl LogEntry {
    /// Create a new log entry
    pub fn new(level: LogLevel, source: String, message: String, data: Option<Vec<u8>>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        let thread_id = format!("{:?}", std::thread::current().id());
        
        Self {
            timestamp,
            level,
            source,
            message,
            data,
            thread_id,
        }
    }
    
    /// Format as a single-line string
    pub fn format_line(&self) -> String {
        let timestamp = SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(self.timestamp);
        let datetime = chrono::DateTime::<chrono::Utc>::from(timestamp);
        let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S%.3f");
        
        if let Some(ref data) = self.data {
            format!(
                "[{}] {} [{}]: {} ({}B data)",
                formatted_time,
                self.level.as_str(),
                self.source,
                self.message,
                data.len()
            )
        } else {
            format!(
                "[{}] {} [{}]: {}",
                formatted_time,
                self.level.as_str(),
                self.source,
                self.message
            )
        }
    }
    
    /// Estimated memory usage in bytes
    pub fn memory_usage(&self) -> usize {
        std::mem::size_of::<Self>() 
            + self.source.len()
            + self.message.len()
            + self.thread_id.len()
            + self.data.as_ref().map_or(0, |d| d.len())
    }
}

/// Rolling log buffer with fixed capacity
pub struct LogBuffer {
    /// Maximum number of entries
    capacity: usize,
    
    /// Log entries in chronological order
    entries: VecDeque<LogEntry>,
    
    /// Total number of logs ever added (including rolled out)
    total_logged: u64,
    
    /// Number of entries that have been rolled out
    rolled_out: u64,
    
    /// Current memory usage estimate
    memory_usage: usize,
    
    /// Maximum memory limit in bytes
    max_memory: Option<usize>,
}

impl LogBuffer {
    /// Create a new log buffer with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: VecDeque::with_capacity(capacity),
            total_logged: 0,
            rolled_out: 0,
            memory_usage: 0,
            max_memory: None,
        }
    }
    
    /// Create a new log buffer with memory limit
    pub fn with_memory_limit(capacity: usize, max_memory: usize) -> Self {
        let mut buffer = Self::new(capacity);
        buffer.max_memory = Some(max_memory);
        buffer
    }
    
    /// Add a log entry
    pub fn log(&mut self, level: LogLevel, source: &str, message: String, data: Option<Vec<u8>>) {
        let entry = LogEntry::new(level, source.to_string(), message, data);
        self.push(entry);
    }
    
    /// Push a log entry to the buffer
    pub fn push(&mut self, entry: LogEntry) {
        let entry_size = entry.memory_usage();
        
        // Check memory limit
        if let Some(max_mem) = self.max_memory {
            while self.memory_usage + entry_size > max_mem && !self.entries.is_empty() {
                self.remove_oldest();
            }
        }
        
        // Check capacity limit
        while self.entries.len() >= self.capacity && !self.entries.is_empty() {
            self.remove_oldest();
        }
        
        self.memory_usage += entry_size;
        self.entries.push_back(entry);
        self.total_logged += 1;
    }
    
    /// Remove the oldest entry
    fn remove_oldest(&mut self) {
        if let Some(old_entry) = self.entries.pop_front() {
            self.memory_usage = self.memory_usage.saturating_sub(old_entry.memory_usage());
            self.rolled_out += 1;
        }
    }
    
    /// Get all entries
    pub fn entries(&self) -> &VecDeque<LogEntry> {
        &self.entries
    }
    
    /// Get entries filtered by level
    pub fn entries_by_level(&self, min_level: LogLevel) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.level >= min_level)
            .collect()
    }
    
    /// Get entries in time range
    pub fn entries_in_range(&self, start_ms: u64, end_ms: u64) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp >= start_ms && e.timestamp <= end_ms)
            .collect()
    }
    
    /// Search entries by text
    pub fn search(&self, query: &str) -> Vec<&LogEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| {
                e.message.to_lowercase().contains(&query_lower) ||
                e.source.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.memory_usage = 0;
        // Don't reset counters - they track lifetime stats
    }
    
    /// Get current number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    
    /// Get buffer capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    
    /// Get total number of logs ever added
    pub fn total_logged(&self) -> u64 {
        self.total_logged
    }
    
    /// Get number of logs rolled out
    pub fn rolled_out(&self) -> u64 {
        self.rolled_out
    }
    
    /// Get current memory usage
    pub fn memory_usage(&self) -> usize {
        self.memory_usage
    }
    
    /// Get buffer statistics
    pub fn stats(&self) -> BufferStats {
        BufferStats {
            current_entries: self.entries.len(),
            capacity: self.capacity,
            total_logged: self.total_logged,
            rolled_out: self.rolled_out,
            memory_usage: self.memory_usage,
            max_memory: self.max_memory,
        }
    }
}

/// Buffer statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferStats {
    pub current_entries: usize,
    pub capacity: usize,
    pub total_logged: u64,
    pub rolled_out: u64,
    pub memory_usage: usize,
    pub max_memory: Option<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(
            LogLevel::Info,
            "Test".to_string(),
            "Test message".to_string(),
            None,
        );
        
        assert_eq!(entry.level, LogLevel::Info);
        assert_eq!(entry.source, "Test");
        assert_eq!(entry.message, "Test message");
        assert!(entry.timestamp > 0);
    }
    
    #[test]
    fn test_buffer_capacity() {
        let mut buffer = LogBuffer::new(3);
        
        buffer.log(LogLevel::Info, "Test", "Message 1".to_string(), None);
        buffer.log(LogLevel::Info, "Test", "Message 2".to_string(), None);
        buffer.log(LogLevel::Info, "Test", "Message 3".to_string(), None);
        assert_eq!(buffer.len(), 3);
        
        // Adding a 4th entry should roll out the oldest
        buffer.log(LogLevel::Info, "Test", "Message 4".to_string(), None);
        assert_eq!(buffer.len(), 3);
        assert_eq!(buffer.rolled_out(), 1);
        assert_eq!(buffer.total_logged(), 4);
        
        // First message should be gone
        let messages: Vec<String> = buffer.entries()
            .iter()
            .map(|e| e.message.clone())
            .collect();
        assert!(!messages.contains(&"Message 1".to_string()));
        assert!(messages.contains(&"Message 4".to_string()));
    }
    
    #[test]
    fn test_buffer_memory_limit() {
        // Create buffer with very small memory limit
        let mut buffer = LogBuffer::with_memory_limit(100, 500);
        
        // Add entries with data
        for i in 0..10 {
            buffer.log(
                LogLevel::Info,
                "Test",
                format!("Message {}", i),
                Some(vec![0u8; 50]),
            );
        }
        
        // Should have rolled out some entries due to memory limit
        assert!(buffer.rolled_out() > 0);
        assert!(buffer.memory_usage() <= 500);
    }
    
    #[test]
    fn test_log_filtering() {
        let mut buffer = LogBuffer::new(10);
        
        buffer.log(LogLevel::Debug, "Test", "Debug message".to_string(), None);
        buffer.log(LogLevel::Info, "Test", "Info message".to_string(), None);
        buffer.log(LogLevel::Warning, "Test", "Warning message".to_string(), None);
        buffer.log(LogLevel::Error, "Test", "Error message".to_string(), None);
        
        let warnings_and_above = buffer.entries_by_level(LogLevel::Warning);
        assert_eq!(warnings_and_above.len(), 2);
        
        let search_results = buffer.search("Warning");
        assert_eq!(search_results.len(), 1);
    }
    
    #[test]
    fn test_log_formatting() {
        let entry = LogEntry::new(
            LogLevel::Error,
            "TestComponent".to_string(),
            "Something went wrong".to_string(),
            Some(vec![1, 2, 3]),
        );
        
        let formatted = entry.format_line();
        assert!(formatted.contains("ERROR"));
        assert!(formatted.contains("TestComponent"));
        assert!(formatted.contains("Something went wrong"));
        assert!(formatted.contains("3B data"));
    }
}