use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use crate::device::{DeviceResult, DeviceError, Transport, TransportType, DeviceSession};

/// Device driver interface (equivalent to IDeviceDriver)
/// All device plugins must implement this trait
#[async_trait]
pub trait DeviceDriver: Send + Sync {
    /// Get the driver name
    fn name(&self) -> &str;
    
    /// Get driver version
    fn version(&self) -> &str;
    
    /// Get supported transport types
    fn supported_transports(&self) -> Vec<TransportType>;
    
    /// Probe if this driver can handle the connected device
    /// Returns true if the driver recognizes and can control the device
    async fn probe_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<bool>;
    
    /// Open a device session for communication
    async fn open_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<Box<dyn DeviceSession>>;
    
    /// Get driver capabilities
    fn capabilities(&self) -> DriverCapabilities;
    
    /// Get driver metadata (for UI/configuration)
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({
            "name": self.name(),
            "version": self.version(),
            "transports": self.supported_transports(),
            "capabilities": self.capabilities(),
        })
    }
}

/// Driver capabilities flags
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverCapabilities {
    /// Supports hot-plug detection
    pub hot_plug: bool,
    
    /// Supports telemetry streaming
    pub telemetry: bool,
    
    /// Supports PWM output
    pub pwm: bool,
    
    /// Supports GPIO control
    pub gpio: bool,
    
    /// Supports analog input
    pub analog_input: bool,
    
    /// Supports serial communication
    pub serial_passthrough: bool,
    
    /// Supports firmware updates
    pub firmware_update: bool,
    
    /// Requires authentication
    pub requires_auth: bool,
    
    /// Maximum data rate (bytes/second)
    pub max_data_rate: Option<u32>,
    
    /// Minimum latency requirement (ms)
    pub min_latency_ms: Option<u32>,
}

impl Default for DriverCapabilities {
    fn default() -> Self {
        DriverCapabilities {
            hot_plug: false,
            telemetry: false,
            pwm: false,
            gpio: false,
            analog_input: false,
            serial_passthrough: false,
            firmware_update: false,
            requires_auth: false,
            max_data_rate: None,
            min_latency_ms: Some(50), // Default 50ms latency requirement
        }
    }
}

/// Driver priority for probe order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DriverPriority {
    Low = 0,
    Normal = 50,
    High = 100,
    Critical = 200,
}

/// Driver registration info
#[derive(Clone)]
pub struct DriverInfo {
    pub name: String,
    pub version: String,
    pub priority: DriverPriority,
    pub driver: Arc<dyn DeviceDriver>,
}

impl std::fmt::Debug for DriverInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DriverInfo")
            .field("name", &self.name)
            .field("version", &self.version)
            .field("priority", &self.priority)
            .field("driver", &format!("<DeviceDriver: {}>", self.name))
            .finish()
    }
}

impl DriverInfo {
    pub fn new(driver: Arc<dyn DeviceDriver>) -> Self {
        DriverInfo {
            name: driver.name().to_string(),
            version: driver.version().to_string(),
            priority: DriverPriority::Normal,
            driver,
        }
    }
    
    pub fn with_priority(mut self, priority: DriverPriority) -> Self {
        self.priority = priority;
        self
    }
}