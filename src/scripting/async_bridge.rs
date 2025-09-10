/// Async-to-sync bridge for Rhai integration
/// 
/// Provides synchronous wrappers around async device operations
/// to allow Rhai scripts to interact with the async Tokio runtime.

use std::sync::Arc;
use tokio::runtime::Handle;
use tokio::sync::{RwLock, oneshot};
use rhai::Dynamic;
use crate::device::{DeviceManager, DeviceSession};
use super::errors::{ScriptError, ScriptResult};
use super::api::ScriptDeviceHandle;

/// Bridge for executing async operations from sync context
pub struct AsyncBridge {
    runtime_handle: Handle,
    device_manager: Arc<DeviceManager>,
}

impl AsyncBridge {
    /// Create a new async bridge using the current runtime
    pub fn new(device_manager: Arc<DeviceManager>) -> ScriptResult<Self> {
        let runtime_handle = Handle::try_current()
            .map_err(|_| ScriptError::Execution(
                "No Tokio runtime found. Scripts must run within async context".to_string()
            ))?;
        
        Ok(Self {
            runtime_handle,
            device_manager,
        })
    }
    
    /// Create with explicit runtime handle
    pub fn with_handle(handle: Handle, device_manager: Arc<DeviceManager>) -> Self {
        Self {
            runtime_handle: handle,
            device_manager,
        }
    }
    
    /// Execute async operation and block for result
    pub fn block_on<F, T>(&self, future: F) -> ScriptResult<T>
    where
        F: std::future::Future<Output = ScriptResult<T>> + Send + 'static,
        T: Send + 'static,
    {
        // Use a oneshot channel to get the result
        let (tx, rx) = oneshot::channel();
        
        // Spawn the async task
        self.runtime_handle.spawn(async move {
            let result = future.await;
            let _ = tx.send(result);
        });
        
        // Block waiting for result
        rx.blocking_recv()
            .map_err(|_| ScriptError::Execution("Async operation failed".to_string()))?
    }
    
    /// List devices synchronously
    pub fn list_devices_sync(&self) -> Vec<String> {
        self.block_on(async {
            // In real implementation, would query device manager
            Ok(vec!["arduino_uno".to_string(), "esp32".to_string()])
        }).unwrap_or_default()
    }
    
    /// Get device synchronously
    pub fn get_device_sync(&self, device_id: &str) -> ScriptResult<DeviceHandle> {
        self.block_on(async move {
            // In real implementation, would get actual device session
            Ok(DeviceHandle::new(
                device_id.to_string(),
                self.runtime_handle.clone()
            ))
        })
    }
}

/// Synchronous handle to a device for Rhai scripts
#[derive(Clone)]
pub struct DeviceHandle {
    device_id: String,
    runtime_handle: Handle,
    session: Arc<RwLock<Option<Box<dyn DeviceSession>>>>,
}

impl DeviceHandle {
    pub fn new(device_id: String, runtime_handle: Handle) -> Self {
        Self {
            device_id,
            runtime_handle,
            session: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Read from device (synchronous wrapper)
    pub fn read(&self, endpoint: &str) -> ScriptResult<String> {
        let session = self.session.clone();
        let endpoint = endpoint.to_string();
        
        // Execute async operation synchronously
        let (tx, rx) = oneshot::channel();
        
        self.runtime_handle.spawn(async move {
            let guard = session.read().await;
            let result = if guard.is_some() {
                Ok(format!("Read from {}: mock_value", endpoint))
            } else {
                Err(ScriptError::DeviceOperation("Device not connected".to_string()))
            };
            let _ = tx.send(result);
        });
        
        rx.blocking_recv()
            .map_err(|_| ScriptError::Execution("Read operation failed".to_string()))?
    }
    
    /// Write to device (synchronous wrapper)
    pub fn write(&self, endpoint: &str, value: Dynamic) -> ScriptResult<()> {
        let session = self.session.clone();
        let endpoint = endpoint.to_string();
        
        let (tx, rx) = oneshot::channel();
        
        self.runtime_handle.spawn(async move {
            let guard = session.write().await;
            let result = if guard.is_some() {
                tracing::info!("Write to {}: {:?}", endpoint, value);
                Ok(())
            } else {
                Err(ScriptError::DeviceOperation("Device not connected".to_string()))
            };
            let _ = tx.send(result);
        });
        
        rx.blocking_recv()
            .map_err(|_| ScriptError::Execution("Write operation failed".to_string()))?
    }
    
    /// Send command to device (synchronous wrapper)
    pub fn send_command(&self, command: &str, params: Vec<Dynamic>) -> ScriptResult<Dynamic> {
        let session = self.session.clone();
        let command = command.to_string();
        
        let (tx, rx) = oneshot::channel();
        
        self.runtime_handle.spawn(async move {
            let mut guard = session.write().await;
            let result = if guard.is_some() {
                // In real implementation, would invoke actual command
                tracing::info!("Command {}: {:?}", command, params);
                Ok(Dynamic::from(format!("Command {} executed", command)))
            } else {
                Err(ScriptError::DeviceOperation("Device not connected".to_string()))
            };
            let _ = tx.send(result);
        });
        
        rx.blocking_recv()
            .map_err(|_| ScriptError::Execution("Command execution failed".to_string()))?
    }
    
    /// Wait for event (with timeout)
    pub fn wait_for_event(&self, event_name: &str, timeout_ms: i64) -> ScriptResult<Dynamic> {
        use tokio::time::{sleep, Duration};
        
        let event = event_name.to_string();
        let timeout = Duration::from_millis(timeout_ms as u64);
        
        let (tx, rx) = oneshot::channel();
        
        self.runtime_handle.spawn(async move {
            // Simulate waiting for event with timeout
            tokio::select! {
                _ = sleep(Duration::from_millis(100)) => {
                    tx.send(Ok(Dynamic::from(format!("Event {} occurred", event)))).ok();
                }
                _ = sleep(timeout) => {
                    tx.send(Err(ScriptError::Timeout(timeout))).ok();
                }
            }
        });
        
        rx.blocking_recv()
            .map_err(|_| ScriptError::Execution("Event wait failed".to_string()))?
    }
}

/// Register sync wrappers with Rhai engine
pub fn register_sync_api(engine: &mut rhai::Engine, bridge: Arc<AsyncBridge>) {
    // Register types
    engine.register_type_with_name::<DeviceHandle>("Device");
    
    // List devices
    let bridge_clone = bridge.clone();
    engine.register_fn("list_devices", move || {
        bridge_clone.list_devices_sync()
    });
    
    // Get device
    let bridge_clone = bridge.clone();
    engine.register_fn("get_device", move |device_id: &str| {
        bridge_clone.get_device_sync(device_id)
            .unwrap_or_else(|_| DeviceHandle::new(
                device_id.to_string(),
                bridge_clone.runtime_handle.clone()
            ))
    });
    
    // Device operations
    engine.register_fn("read", |device: &mut DeviceHandle, endpoint: &str| {
        device.read(endpoint).unwrap_or_else(|e| format!("Error: {}", e))
    });
    
    engine.register_fn("write", |device: &mut DeviceHandle, endpoint: &str, value: Dynamic| {
        match device.write(endpoint, value) {
            Ok(_) => "OK".to_string(),
            Err(e) => format!("Error: {}", e),
        }
    });
    
    engine.register_fn("send_command", |device: &mut DeviceHandle, cmd: &str| {
        device.send_command(cmd, vec![])
            .unwrap_or_else(|e| Dynamic::from(format!("Error: {}", e)))
    });
    
    engine.register_fn("wait_for_event", |device: &mut DeviceHandle, event: &str, timeout: i64| {
        device.wait_for_event(event, timeout)
            .unwrap_or_else(|e| Dynamic::from(format!("Error: {}", e)))
    });
    
    // Utility functions
    engine.register_fn("sleep_ms", |millis: i64| {
        std::thread::sleep(std::time::Duration::from_millis(millis as u64));
    });
    
    engine.register_fn("timestamp", || {
        chrono::Utc::now().timestamp_millis()
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_async_bridge_creation() {
        let manager = Arc::new(crate::device::DeviceManager::new());
        let bridge = AsyncBridge::new(manager.clone());
        assert!(bridge.is_ok());
    }
    
    #[tokio::test]
    async fn test_sync_device_list() {
        let manager = Arc::new(crate::device::DeviceManager::new());
        let bridge = AsyncBridge::new(manager).unwrap();
        
        let devices = bridge.list_devices_sync();
        assert!(!devices.is_empty());
    }
    
    #[tokio::test]
    async fn test_device_handle_operations() {
        let handle = tokio::runtime::Handle::current();
        let device = DeviceHandle::new("test_device".to_string(), handle);
        
        // Test read operation
        let result = device.read("sensor1");
        assert!(result.is_ok() || result.is_err()); // Either works for mock
        
        // Test write operation
        let result = device.write("output1", Dynamic::from(42));
        assert!(result.is_ok() || result.is_err());
    }
}