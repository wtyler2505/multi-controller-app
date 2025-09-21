//! Example demonstrating device connection and session management

use multi_controller_core::{DeviceDriver, Transport, DeviceSession, Value, ConnectionState};
use multi_controller_serial::{SerialTransport, SerialConfig};
use multi_controller_arduino::ArduinoDriver;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    println!("🚀 Multi-Controller Device Connection Demo");
    println!("===========================================");
    
    // 1. Create a serial transport configuration
    let config = SerialConfig::new("/dev/ttyUSB0".to_string())
        .with_baud_rate(115200);
    
    let mut transport = SerialTransport::new(config);
    
    println!("📡 Transport created: {}", transport.name());
    println!("   Type: {:?}", transport.transport_type());
    
    // 2. Connect the transport (simulated for demo)
    transport.connect().await?;
    println!("✅ Transport connected!");
    
    // 3. Create an Arduino driver
    let driver = ArduinoDriver::new();
    println!("🔧 Driver created: {}", driver.name());
    println!("   Version: {}", driver.version());
    println!("   Supported transports: {:?}", driver.supported_transports());
    println!("   Capabilities: {:?}", driver.capabilities());
    
    // 4. Probe for devices (simulated response)
    println!("🔍 Probing for Arduino devices...");
    let probe_result = driver.probe(&mut transport).await?;
    match probe_result {
        Some(device_info) => {
            println!("✅ Device found: {}", device_info.name);
            println!("   Type: {}", device_info.device_type);
            println!("   Capabilities: {:?}", device_info.capabilities);
        }
        None => {
            println!("ℹ️  No Arduino device detected (expected in simulation)");
        }
    }
    
    // 5. Open a device session
    println!("🔌 Opening device session...");
    let mut session = driver.open(Box::new(transport)).await?;
    
    let session_info = session.session_info();
    println!("✅ Session created with ID: {}", session_info.session_id);
    println!("   Device: {}", session_info.device_info.name);
    println!("   State: {:?}", session.connection_state());
    
    // 6. Send some commands
    println!("📤 Sending commands to device...");
    
    // Digital write
    let result = session.invoke("digital_write", vec![
        Value::Number(13.into()),  // Pin 13
        Value::Number(1.into())    // HIGH
    ]).await?;
    println!("   digital_write(13, HIGH): {:?}", result);
    
    // Analog read
    let result = session.invoke("analog_read", vec![
        Value::Number(0.into())    // Pin A0
    ]).await?;
    println!("   analog_read(A0): {:?}", result);
    
    // 7. Send raw data
    let test_data = b"Hello Arduino!";
    let bytes_sent = session.send_raw(test_data).await?;
    println!("📤 Sent {} bytes of raw data", bytes_sent);
    
    // 8. Subscribe to a data stream (mock)
    println!("📡 Setting up data stream subscription...");
    let subscription = session.subscribe("sensor_data", Box::new(|data| {
        println!("📊 Received sensor data: {} bytes", data.len());
    })).await?;
    
    println!("   Subscribed with handle: {:?}", subscription);
    
    // 9. Unsubscribe
    session.unsubscribe(subscription).await?;
    println!("   Unsubscribed from data stream");
    
    // 10. Clean up resources
    println!("🧹 Cleaning up resources...");
    session.cleanup_resources().await?;
    
    // 11. Check final state
    println!("📊 Final session state: {:?}", session.connection_state());
    
    println!("✅ Demo completed successfully!");
    println!("\nThis demo showcased:");
    println!("• Cross-platform serial transport configuration");
    println!("• Device driver architecture with pluggable drivers");
    println!("• Session management with unique IDs");
    println!("• Command execution and raw data handling");
    println!("• Resource cleanup following RAII patterns");
    println!("• Async/await patterns for non-blocking operations");
    
    Ok(())
}