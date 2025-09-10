// Logger configuration and management

use serde::{Serialize, Deserialize};

/// Logger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggerConfig {
    /// Buffer size for device I/O logs
    pub device_io_buffer_size: usize,
    
    /// Buffer size for event logs
    pub event_buffer_size: usize,
    
    /// Buffer size for system logs
    pub system_buffer_size: usize,
    
    /// Maximum memory usage per buffer (bytes)
    pub max_buffer_memory: Option<usize>,
    
    /// Minimum log level to capture
    pub min_level: super::LogLevel,
    
    /// Enable automatic export on buffer full
    pub auto_export_on_full: bool,
    
    /// Export directory path
    pub export_dir: std::path::PathBuf,
    
    /// Log file rotation settings
    pub rotation: RotationConfig,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            device_io_buffer_size: 5000,  // 5k entries for device I/O
            event_buffer_size: 2000,       // 2k entries for events
            system_buffer_size: 1000,      // 1k entries for system
            max_buffer_memory: Some(10 * 1024 * 1024), // 10MB per buffer
            min_level: super::LogLevel::Debug,
            auto_export_on_full: false,
            export_dir: std::path::PathBuf::from("logs"),
            rotation: RotationConfig::default(),
        }
    }
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationConfig {
    /// Enable automatic rotation
    pub enabled: bool,
    
    /// Maximum log file size before rotation (bytes)
    pub max_file_size: usize,
    
    /// Maximum number of rotated files to keep
    pub max_files: usize,
    
    /// Rotation interval (hours)
    pub rotation_interval_hours: u32,
}

impl Default for RotationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_file_size: 50 * 1024 * 1024, // 50MB
            max_files: 10,
            rotation_interval_hours: 24,
        }
    }
}

/// Simple logger interface
pub struct Logger {
    config: LoggerConfig,
}

impl Logger {
    /// Create a new logger with default config
    pub fn new() -> Self {
        Self::with_config(LoggerConfig::default())
    }
    
    /// Create a new logger with custom config
    pub fn with_config(config: LoggerConfig) -> Self {
        Self { config }
    }
    
    /// Get the configuration
    pub fn config(&self) -> &LoggerConfig {
        &self.config
    }
    
    /// Update configuration
    pub fn set_config(&mut self, config: LoggerConfig) {
        self.config = config;
    }
    
    /// Create export directory if it doesn't exist
    pub fn ensure_export_dir(&self) -> std::io::Result<()> {
        std::fs::create_dir_all(&self.config.export_dir)
    }
    
    /// Generate export filename with timestamp
    pub fn generate_export_filename(&self, prefix: &str, extension: &str) -> std::path::PathBuf {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}_{}.{}", prefix, timestamp, extension);
        self.config.export_dir.join(filename)
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = LoggerConfig::default();
        assert_eq!(config.device_io_buffer_size, 5000);
        assert_eq!(config.event_buffer_size, 2000);
        assert_eq!(config.system_buffer_size, 1000);
        assert_eq!(config.min_level, super::super::LogLevel::Debug);
    }
    
    #[test]
    fn test_logger_creation() {
        let logger = Logger::new();
        assert_eq!(logger.config().device_io_buffer_size, 5000);
        
        let custom_config = LoggerConfig {
            device_io_buffer_size: 10000,
            ..Default::default()
        };
        let custom_logger = Logger::with_config(custom_config);
        assert_eq!(custom_logger.config().device_io_buffer_size, 10000);
    }
    
    #[test]
    fn test_export_filename_generation() {
        let logger = Logger::new();
        let filename = logger.generate_export_filename("test", "log");
        let filename_str = filename.to_string_lossy();
        
        assert!(filename_str.contains("test_"));
        assert!(filename_str.ends_with(".log"));
    }
}