// Logging system with rolling buffers and export functionality

pub mod buffer;
pub mod exporter;
pub mod logger;

pub use buffer::{LogBuffer, LogEntry, LogLevel};
pub use exporter::{LogExporter, LogFormat};
pub use logger::{Logger, LoggerConfig};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Global logging system
pub struct LoggingSystem {
    /// Device I/O log buffer
    pub device_io: Arc<RwLock<LogBuffer>>,
    
    /// Event log buffer
    pub events: Arc<RwLock<LogBuffer>>,
    
    /// System log buffer
    pub system: Arc<RwLock<LogBuffer>>,
    
    /// Configuration
    config: LoggerConfig,
}

impl LoggingSystem {
    /// Create a new logging system with default configuration
    pub fn new() -> Self {
        Self::with_config(LoggerConfig::default())
    }
    
    /// Create a new logging system with custom configuration
    pub fn with_config(config: LoggerConfig) -> Self {
        Self {
            device_io: Arc::new(RwLock::new(LogBuffer::new(config.device_io_buffer_size))),
            events: Arc::new(RwLock::new(LogBuffer::new(config.event_buffer_size))),
            system: Arc::new(RwLock::new(LogBuffer::new(config.system_buffer_size))),
            config,
        }
    }
    
    /// Log device I/O
    pub async fn log_device_io(&self, level: LogLevel, message: String, data: Option<Vec<u8>>) {
        let mut buffer = self.device_io.write().await;
        buffer.log(level, "DeviceIO", message, data);
    }
    
    /// Log event
    pub async fn log_event(&self, level: LogLevel, source: &str, message: String) {
        let mut buffer = self.events.write().await;
        buffer.log(level, source, message, None);
    }
    
    /// Log system message
    pub async fn log_system(&self, level: LogLevel, message: String) {
        let mut buffer = self.system.write().await;
        buffer.log(level, "System", message, None);
    }
    
    /// Generic log method that routes to system buffer
    pub async fn log(&self, level: LogLevel, source: &str, message: String, data: Option<Vec<u8>>) {
        let mut buffer = self.system.write().await;
        buffer.log(level, source, message, data);
    }
    
    /// Export all logs to file
    pub async fn export_all(&self, format: LogFormat) -> Result<Vec<u8>, std::io::Error> {
        let device_io = self.device_io.read().await;
        let events = self.events.read().await;
        let system = self.system.read().await;
        
        let exporter = LogExporter::new(format);
        exporter.export_multiple(&[
            ("device_io", &*device_io),
            ("events", &*events),
            ("system", &*system),
        ])
    }
    
    /// Clear all log buffers
    pub async fn clear_all(&self) {
        self.device_io.write().await.clear();
        self.events.write().await.clear();
        self.system.write().await.clear();
    }
    
    /// Get total memory usage across all buffers
    pub async fn memory_usage(&self) -> usize {
        let device_io = self.device_io.read().await.memory_usage();
        let events = self.events.read().await.memory_usage();
        let system = self.system.read().await.memory_usage();
        device_io + events + system
    }
}

impl Default for LoggingSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_logging_system_creation() {
        let logging = LoggingSystem::new();
        assert_eq!(logging.memory_usage().await, 0);
    }
    
    #[tokio::test]
    async fn test_log_and_export() {
        let logging = LoggingSystem::new();
        
        logging.log_device_io(LogLevel::Info, "Test device message".to_string(), None).await;
        logging.log_event(LogLevel::Warning, "TestSource", "Test event".to_string()).await;
        logging.log_system(LogLevel::Error, "Test system error".to_string()).await;
        
        let export = logging.export_all(LogFormat::Json).await.unwrap();
        assert!(!export.is_empty());
        
        let json_str = String::from_utf8(export).unwrap();
        assert!(json_str.contains("Test device message"));
        assert!(json_str.contains("Test event"));
        assert!(json_str.contains("Test system error"));
    }
}