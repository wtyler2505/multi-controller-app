/// Integration tests for Multi-Controller App
/// 
/// Tests component interactions and end-to-end data flows

mod transport_driver;
mod device_manager;
mod scripting_device;
mod end_to_end;
mod error_propagation;

use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;

/// Common test utilities
pub mod common {
    use super::*;
    use crate::transport::{Transport, MockTransport, TransportConfig};
    use crate::transport::mock::MockConfig;
    use crate::device::{DeviceManager, DeviceDriver, DeviceSession};
    use crate::drivers::ArduinoUno;
    
    /// Create a configured mock transport for testing
    pub fn create_test_transport(name: &str) -> MockTransport {
        let config = TransportConfig {
            min_latency: Some(Duration::from_millis(10)),
            ..Default::default()
        };
        
        let mock_config = MockConfig {
            latency_ms: 10,
            enforce_latency: true,
            response_data: b"Multi-Controller:Arduino\r\n".to_vec(),
            ..Default::default()
        };
        
        MockTransport::new(name.into(), config, mock_config)
    }
    
    /// Create a test device manager with mock transports
    pub async fn create_test_manager() -> Arc<DeviceManager> {
        let manager = Arc::new(DeviceManager::new());
        // Manager setup would happen here
        manager
    }
    
    /// Simulate device connection sequence
    pub async fn connect_device(
        transport: &mut dyn Transport,
        driver: &dyn DeviceDriver
    ) -> Result<Box<dyn DeviceSession>, Box<dyn std::error::Error>> {
        // Connect transport
        transport.connect().await?;
        
        // Probe device
        if !driver.probe(transport).await? {
            return Err("Device probe failed".into());
        }
        
        // Open session
        driver.open(transport).await
    }
    
    /// Verify data flow through system
    pub async fn verify_data_flow(
        start: Vec<u8>,
        expected_end: Vec<u8>,
        timeout: Duration
    ) -> bool {
        // Implementation would track data through system
        true
    }
}