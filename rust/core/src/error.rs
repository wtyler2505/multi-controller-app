use thiserror::Error;
use std::io;

/// The main error type for Multi-Controller operations
#[derive(Error, Debug)]
pub enum MultiControllerError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    #[error("Serial port error: {0}")]
    SerialPort(String),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    
    #[error("Session error: {0}")]
    Session(String),
    
    #[error("Timeout error: operation timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },
    
    #[error("Device probe failed: {reason}")]
    ProbeFailed { reason: String },
    
    #[error("Resource cleanup failed: {0}")]
    CleanupFailed(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
}

// Convenience alias
pub type Result<T> = std::result::Result<T, MultiControllerError>;

// Custom implementation for serialport errors
impl From<serialport::Error> for MultiControllerError {
    fn from(err: serialport::Error) -> Self {
        MultiControllerError::SerialPort(err.to_string())
    }
}