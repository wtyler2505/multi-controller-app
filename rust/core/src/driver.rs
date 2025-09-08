use crate::{Result, Transport, async_trait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information about a detected device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_type: String,
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub capabilities: Vec<String>,
    pub transport_requirements: TransportRequirements,
}

/// Transport requirements for a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportRequirements {
    pub supported_transports: Vec<String>,
    pub default_config: HashMap<String, String>,
}

/// The DeviceDriver trait defines the interface for device-specific implementations
#[async_trait]
pub trait DeviceDriver: Send + Sync {
    /// Get the driver name (e.g., "Arduino", "ESP32")
    fn name(&self) -> &str;
    
    /// Get supported transport types
    fn supported_transports(&self) -> &[String];
    
    /// Probe a transport to see if this driver can handle the connected device
    async fn probe(&self, transport: &mut dyn Transport) -> Result<Option<DeviceInfo>>;
    
    /// Open a device session using the provided transport
    async fn open(&self, transport: Box<dyn Transport>) -> Result<Box<dyn crate::DeviceSession>>;
    
    /// Get driver version information
    fn version(&self) -> &str;
    
    /// Get driver capabilities/features
    fn capabilities(&self) -> Vec<String>;
}