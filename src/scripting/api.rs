use rhai::{Dynamic, Engine};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use crate::device::{DeviceManager, DeviceSession};
use super::errors::{ScriptError, ScriptResult};

/// Safe handle to a device for script access
#[derive(Clone)]
pub struct ScriptDeviceHandle {
    device_id: String,
    session: Arc<RwLock<Option<Box<dyn DeviceSession>>>>,
    allowed_operations: Vec<String>,
}

impl ScriptDeviceHandle {
    pub fn new(
        device_id: String, 
        session: Box<dyn DeviceSession>,
        allowed_operations: Vec<String>
    ) -> Self {
        Self {
            device_id,
            session: Arc::new(RwLock::new(Some(session))),
            allowed_operations,
        }
    }
    
    /// Check if an operation is allowed
    fn check_permission(&self, operation: &str) -> ScriptResult<()> {
        if self.allowed_operations.contains(&operation.to_string()) {
            Ok(())
        } else {
            Err(ScriptError::Security(
                format!("Operation '{}' not allowed for device {}", operation, self.device_id)
            ))
        }
    }
    
    /// Safe read operation
    pub async fn read(&self, endpoint: &str) -> ScriptResult<Dynamic> {
        self.check_permission("read")?;
        
        let session = self.session.read().await;
        if let Some(session) = session.as_ref() {
            // Note: This is a simplified example - actual implementation
            // needs to handle async operations properly
            Ok(Dynamic::from(format!("Read from {}: placeholder", endpoint)))
        } else {
            Err(ScriptError::DeviceOperation(
                format!("Device {} not connected", self.device_id)
            ))
        }
    }
    
    /// Safe write operation
    pub async fn write(&self, endpoint: &str, value: Dynamic) -> ScriptResult<()> {
        self.check_permission("write")?;
        
        let mut session = self.session.write().await;
        if let Some(_session) = session.as_mut() {
            // Note: Actual implementation would invoke device command
            tracing::info!("Script write to {}: {:?}", endpoint, value);
            Ok(())
        } else {
            Err(ScriptError::DeviceOperation(
                format!("Device {} not connected", self.device_id)
            ))
        }
    }
    
    /// Safe control operation
    pub async fn control(&self, command: &str, params: Vec<Dynamic>) -> ScriptResult<Dynamic> {
        self.check_permission("control")?;
        
        // Validate command against whitelist
        let safe_commands = ["set_gpio", "read_sensor", "set_pwm"];
        if !safe_commands.contains(&command) {
            return Err(ScriptError::Security(
                format!("Command '{}' not in whitelist", command)
            ));
        }
        
        let mut session = self.session.write().await;
        if let Some(_session) = session.as_mut() {
            // Note: Actual implementation would invoke device command
            Ok(Dynamic::from(format!("Executed {}: OK", command)))
        } else {
            Err(ScriptError::DeviceOperation(
                format!("Device {} not connected", self.device_id)
            ))
        }
    }
}

/// Device API exposed to scripts
pub struct DeviceApi {
    devices: Arc<RwLock<HashMap<String, ScriptDeviceHandle>>>,
    manager: Arc<DeviceManager>,
}

impl DeviceApi {
    pub fn new(manager: Arc<DeviceManager>) -> Self {
        Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
            manager,
        }
    }
    
    /// List available devices
    pub async fn list_devices(&self) -> Vec<String> {
        // Get device list from manager
        vec!["device1".to_string(), "device2".to_string()] // Placeholder
    }
    
    /// Get a device handle for script access
    pub async fn get_device(&self, device_id: &str) -> ScriptResult<ScriptDeviceHandle> {
        let devices = self.devices.read().await;
        
        if let Some(handle) = devices.get(device_id) {
            Ok(handle.clone())
        } else {
            Err(ScriptError::DeviceOperation(
                format!("Device {} not found", device_id)
            ))
        }
    }
    
    /// Register device API functions with Rhai engine
    pub fn register_api(engine: &mut Engine, api: Arc<DeviceApi>) {
        // Register the DeviceApi type
        engine.register_type::<ScriptDeviceHandle>()
            .register_fn("device_id", |handle: &mut ScriptDeviceHandle| {
                handle.device_id.clone()
            });
        
        // Register global functions
        let api_clone = api.clone();
        engine.register_fn("list_devices", move || {
            // Note: This is a sync wrapper - actual implementation needs
            // to handle async properly with tokio::runtime::Handle
            vec!["device1".to_string(), "device2".to_string()]
        });
        
        let api_clone = api.clone();
        engine.register_fn("get_device", move |device_id: &str| {
            // Sync wrapper for get_device
            // In practice, this would use Handle::current() to run async code
            ScriptDeviceHandle {
                device_id: device_id.to_string(),
                session: Arc::new(RwLock::new(None)),
                allowed_operations: vec!["read".to_string()],
            }
        });
        
        // Register device operations
        engine.register_fn("device_read", |handle: &mut ScriptDeviceHandle, endpoint: &str| {
            // Sync wrapper for async read
            format!("Read from {}: placeholder", endpoint)
        });
        
        engine.register_fn("device_write", 
            |handle: &mut ScriptDeviceHandle, endpoint: &str, value: Dynamic| {
            // Sync wrapper for async write
            format!("Write to {}: {:?}", endpoint, value)
        });
        
        // Register utility functions
        engine.register_fn("sleep", |millis: i64| {
            std::thread::sleep(std::time::Duration::from_millis(millis as u64));
        });
        
        engine.register_fn("print", |text: &str| {
            tracing::info!("[Script]: {}", text);
        });
    }
}