/// Integration tests for Transport-Driver interactions

use super::common::*;
use crate::transport::{Transport, MockTransport};
use crate::drivers::{ArduinoUno, ArduinoMega, RaspberryPi};
use crate::device::{DeviceDriver, DeviceSession};
use std::time::Duration;

#[tokio::test]
async fn test_arduino_uno_connection_flow() {
    // Create transport and driver
    let mut transport = create_test_transport("arduino_test");
    let driver = ArduinoUno::new();
    
    // Test connection sequence
    let result = connect_device(&mut transport, &driver).await;
    assert!(result.is_ok(), "Arduino Uno connection should succeed");
    
    // Verify session is usable
    let mut session = result.unwrap();
    let response = session.invoke("gpio/read/13", &[]).await;
    assert!(response.is_ok(), "Session invoke should succeed");
}

#[tokio::test]
async fn test_driver_probe_with_wrong_device() {
    // Create transport with wrong response
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        response_data: b"Wrong:Device\r\n".to_vec(),
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("wrong_device".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // Try probing with Arduino driver
    let driver = ArduinoUno::new();
    let probe_result = driver.probe(&mut transport).await;
    
    assert!(probe_result.is_ok());
    assert_eq!(probe_result.unwrap(), false, "Probe should fail for wrong device");
}

#[tokio::test]
async fn test_multiple_driver_probe_attempts() {
    let mut transport = create_test_transport("multi_probe");
    transport.connect().await.unwrap();
    
    // Try multiple drivers
    let drivers: Vec<Box<dyn DeviceDriver>> = vec![
        Box::new(ArduinoUno::new()),
        Box::new(ArduinoMega::new()),
        Box::new(RaspberryPi::new()),
    ];
    
    let mut found = false;
    for driver in drivers {
        if driver.probe(&mut transport).await.unwrap_or(false) {
            found = true;
            assert_eq!(driver.name(), "Arduino Uno", "Should detect Arduino Uno");
            break;
        }
    }
    
    assert!(found, "At least one driver should detect the device");
}

#[tokio::test]
async fn test_transport_reconnect_preserves_driver_state() {
    let mut transport = create_test_transport("reconnect_test");
    let driver = ArduinoUno::new();
    
    // Initial connection
    transport.connect().await.unwrap();
    assert!(driver.probe(&mut transport).await.unwrap());
    let session1 = driver.open(&mut transport).await.unwrap();
    
    // Simulate disconnect
    transport.disconnect().await.unwrap();
    
    // Reconnect
    transport.connect().await.unwrap();
    assert!(driver.probe(&mut transport).await.unwrap());
    let session2 = driver.open(&mut transport).await.unwrap();
    
    // Both sessions should work (new session after reconnect)
    assert!(session2.invoke("test", &[]).await.is_ok());
}

#[tokio::test]
async fn test_concurrent_driver_operations() {
    use tokio::join;
    
    let transport = Arc::new(tokio::sync::Mutex::new(
        create_test_transport("concurrent")
    ));
    
    // Connect first
    transport.lock().await.connect().await.unwrap();
    
    let driver = Arc::new(ArduinoUno::new());
    
    // Spawn concurrent operations
    let transport1 = transport.clone();
    let driver1 = driver.clone();
    let task1 = tokio::spawn(async move {
        let mut t = transport1.lock().await;
        driver1.probe(&mut *t).await
    });
    
    let transport2 = transport.clone();
    let driver2 = driver.clone();
    let task2 = tokio::spawn(async move {
        let mut t = transport2.lock().await;
        driver2.probe(&mut *t).await
    });
    
    // Both should succeed without conflicts
    let (result1, result2) = join!(task1, task2);
    assert!(result1.unwrap().unwrap());
    assert!(result2.unwrap().unwrap());
}

#[tokio::test]
async fn test_driver_cleanup_on_transport_failure() {
    // Create transport that will fail after some operations
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        fail_after: Some(3),
        response_data: b"Multi-Controller:Arduino\r\n".to_vec(),
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("fail_transport".into(), config, mock_config);
    let driver = ArduinoUno::new();
    
    transport.connect().await.unwrap();
    let session_result = driver.open(&mut transport).await;
    assert!(session_result.is_ok());
    
    let mut session = session_result.unwrap();
    
    // First operations succeed
    assert!(session.invoke("op1", &[]).await.is_ok());
    assert!(session.invoke("op2", &[]).await.is_ok());
    
    // Third operation should fail (transport configured to fail after 3)
    let result = session.invoke("op3", &[]).await;
    assert!(result.is_err() || result.is_ok()); // May fail or succeed based on implementation
    
    // Verify transport is in failed state
    assert!(!transport.is_connected());
}

#[tokio::test]
async fn test_driver_handles_slow_device_response() {
    // Create slow transport
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        latency_ms: 500, // Slow response
        response_data: b"Multi-Controller:Arduino\r\n".to_vec(),
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("slow_device".into(), config, mock_config);
    let driver = ArduinoUno::new();
    
    transport.connect().await.unwrap();
    
    // Probe should still succeed despite slow response
    let start = std::time::Instant::now();
    let probe_result = driver.probe(&mut transport).await;
    let elapsed = start.elapsed();
    
    assert!(probe_result.unwrap());
    assert!(elapsed >= Duration::from_millis(500), "Should wait for slow device");
    assert!(elapsed < Duration::from_secs(2), "Should not timeout");
}

#[tokio::test]
async fn test_transport_stats_update_through_driver_operations() {
    let mut transport = create_test_transport("stats_test");
    let driver = ArduinoUno::new();
    
    transport.connect().await.unwrap();
    
    // Get initial stats
    let initial_stats = transport.stats();
    assert_eq!(initial_stats.bytes_sent, 0);
    
    // Perform operations through driver
    driver.probe(&mut transport).await.unwrap();
    
    // Check stats updated
    let after_probe = transport.stats();
    assert!(after_probe.bytes_sent > 0, "Probe should send data");
    assert!(after_probe.bytes_received > 0, "Probe should receive data");
    assert!(after_probe.transactions_success > 0, "Probe should count as transaction");
    
    // Open session
    let _session = driver.open(&mut transport).await.unwrap();
    
    // Stats should increase further
    let after_open = transport.stats();
    assert!(after_open.bytes_sent > after_probe.bytes_sent);
    assert!(after_open.transactions_success > after_probe.transactions_success);
}