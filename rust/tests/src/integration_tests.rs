//! Integration tests demonstrating device connection and communication

use multi_controller_core::{DeviceDriver, Transport, DeviceSession, Value};
use multi_controller_serial::{SerialTransport, SerialConfig};
use multi_controller_arduino::ArduinoDriver;
use tokio_test;

#[tokio::test]
async fn test_arduino_driver_probe() {
    let driver = ArduinoDriver::new();
    
    // Test driver properties
    assert_eq!(driver.name(), "Arduino");
    assert_eq!(driver.version(), "1.0.0");
    assert!(driver.capabilities().contains(&"digital_io".to_string()));
    assert!(driver.supported_transports().contains(&"serial".to_string()));
    
    // Create a mock transport
    let config = SerialConfig::new("/dev/ttyUSB0".to_string());
    let mut transport = SerialTransport::new(config);
    
    // Connect the transport
    assert!(!transport.is_connected());
    transport.connect().await.unwrap();
    assert!(transport.is_connected());
    
    // Probe for device (will return None since it's a simulated transport)
    let probe_result = driver.probe(&mut transport).await;
    assert!(probe_result.is_ok());
    // In a real implementation with actual Arduino, this would return Some(DeviceInfo)
    // but our simulated transport doesn't respond to commands
}

#[tokio::test]
async fn test_device_session_lifecycle() {
    let driver = ArduinoDriver::new();
    
    // Create and connect transport
    let config = SerialConfig::new("/dev/ttyUSB0".to_string());
    let mut transport = SerialTransport::new(config);
    transport.connect().await.unwrap();
    
    // Open device session
    let session_result = driver.open(Box::new(transport)).await;
    assert!(session_result.is_ok());
    
    let mut session = session_result.unwrap();
    
    // Test session properties
    let session_info = session.session_info();
    assert_eq!(session_info.device_info.device_type, "Arduino");
    
    // Test session commands
    let digital_write_result = session.invoke("digital_write", vec![
        Value::Number(13.into()),
        Value::Number(1.into())
    ]).await;
    
    // In our simplified implementation, this should work
    assert!(digital_write_result.is_ok());
    
    // Test cleanup
    let cleanup_result = session.cleanup_resources().await;
    assert!(cleanup_result.is_ok());
}

#[tokio::test]
async fn test_serial_transport_data_flow() {
    let config = SerialConfig::new("/dev/ttyUSB0".to_string())
        .with_baud_rate(9600);
    let mut transport = SerialTransport::new(config);
    
    // Test connection
    transport.connect().await.unwrap();
    assert!(transport.is_connected());
    
    // Test sending data
    let test_data = b"Hello Arduino!";
    let bytes_sent = transport.send(test_data).await.unwrap();
    assert_eq!(bytes_sent, test_data.len());
    
    // Test transport info
    let info = transport.info();
    assert_eq!(info.bytes_sent, test_data.len() as u64);
    assert!(info.last_activity.is_some());
    
    // Test receiving (will return 0 in simulation)
    let mut buffer = [0u8; 32];
    let bytes_received = transport.receive(&mut buffer).await.unwrap();
    assert_eq!(bytes_received, 0); // Simulated transport returns no data
    
    // Test flush
    transport.flush().await.unwrap();
    
    // Test disconnect
    transport.disconnect().await.unwrap();
    assert!(!transport.is_connected());
}

#[tokio::test] 
async fn test_session_management_with_unique_ids() {
    let driver = ArduinoDriver::new();
    
    // Create multiple sessions
    let config1 = SerialConfig::new("/dev/ttyUSB0".to_string());
    let mut transport1 = SerialTransport::new(config1);
    transport1.connect().await.unwrap();
    
    let config2 = SerialConfig::new("/dev/ttyUSB1".to_string());
    let mut transport2 = SerialTransport::new(config2);
    transport2.connect().await.unwrap();
    
    let session1 = driver.open(Box::new(transport1)).await.unwrap();
    let session2 = driver.open(Box::new(transport2)).await.unwrap();
    
    // Verify unique session IDs
    let id1 = session1.session_id();
    let id2 = session2.session_id();
    assert_ne!(id1, id2);
    
    // Test connection states
    use multi_controller_core::ConnectionState;
    assert_eq!(session1.connection_state(), ConnectionState::Connected);
    assert_eq!(session2.connection_state(), ConnectionState::Connected);
}

#[tokio::test]
async fn test_error_handling() {
    let config = SerialConfig::new("/dev/ttyUSB0".to_string());
    let mut transport = SerialTransport::new(config);
    
    // Test operations on disconnected transport
    let send_result = transport.send(b"test").await;
    assert!(send_result.is_err());
    
    let mut buffer = [0u8; 10];
    let receive_result = transport.receive(&mut buffer).await;
    assert!(receive_result.is_err());
    
    let flush_result = transport.flush().await;
    assert!(flush_result.is_err());
}