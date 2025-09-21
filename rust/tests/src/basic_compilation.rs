//! Basic compilation test to validate trait implementations

#[tokio::test]
async fn test_traits_compile() {
    // This test just checks that our traits compile correctly
    use multi_controller_core::{DeviceDriver, Transport, DeviceSession};
    use multi_controller_serial::{SerialTransport, SerialConfig};
    use multi_controller_arduino::ArduinoDriver;
    
    // Create instances to ensure compilation works
    let config = SerialConfig::new("/dev/ttyUSB0".to_string());
    let _transport = SerialTransport::new(config);
    let _driver = ArduinoDriver::new();
    
    println!("All traits and implementations compile successfully!");
}

#[tokio::test]
async fn test_serial_transport_basic() {
    use multi_controller_core::{Transport, TransportType};
    use multi_controller_serial::{SerialTransport, SerialConfig};
    
    let config = SerialConfig::new("/dev/ttyUSB0".to_string());
    let transport = SerialTransport::new(config);
    
    // Test basic properties
    assert_eq!(transport.transport_type(), TransportType::Serial);
    assert_eq!(transport.name(), "/dev/ttyUSB0");
    assert!(!transport.is_connected());
    
    let info = transport.info();
    assert_eq!(info.endpoint, "/dev/ttyUSB0");
    assert!(!info.connected);
    assert_eq!(info.bytes_sent, 0);
    assert_eq!(info.bytes_received, 0);
}