// Device abstraction layer module
pub mod driver;
pub mod session;
pub mod manager;
pub mod plugin;
pub mod safety;

pub use driver::{DeviceDriver, DriverCapabilities, DriverInfo, DriverPriority};
pub use session::{DeviceSession, DeviceEndpoint, StreamData};
pub use manager::DeviceManager;
pub use plugin::{PluginLoader, PluginManifest};
pub use safety::{SafetyController, EmergencyStop, HotPlugMonitor, HotPlugEvent};

// Re-export transport types for convenience
pub use crate::transport::{Transport, TransportType};

use std::fmt;
use std::error::Error;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};

/// Result type for device operations
pub type DeviceResult<T> = Result<T, DeviceError>;

/// Device-specific errors
#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    
    #[error("Plugin load error: {0}")]
    PluginLoadError(String),
    
    #[error("Timeout after {0}ms")]
    Timeout(u64),
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Safety violation: {0}")]
    SafetyViolation(String),
    
    #[error("Invalid manifest: {0}")]
    InvalidManifest(String),
    
    #[error("Transport error: {0}")]
    TransportError(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}