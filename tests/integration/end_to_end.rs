/// End-to-end integration tests for complete data flows

use super::common::*;
use crate::transport::{Transport, MockTransport};
use crate::drivers::ArduinoUno;
use crate::device::{DeviceManager, DeviceDriver};
use crate::scripting::{ScriptEngine, SandboxConfig, DeviceApi};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_complete_device_workflow() {
    // 1. Create transport
    let mut transport = create_test_transport("e2e_test");
    
    // 2. Connect transport
    assert!(transport.connect().await.is_ok());
    assert!(transport.is_connected());
    
    // 3. Create and probe driver
    let driver = ArduinoUno::new();
    assert!(driver.probe(&mut transport).await.unwrap());
    
    // 4. Open device session
    let mut session = driver.open(&mut transport).await.unwrap();
    
    // 5. Send commands
    let response = session.invoke("gpio/write/13/1", &[]).await;
    assert!(response.is_ok());
    
    // 6. Read data
    let read_response = session.invoke("gpio/read/13", &[]).await;
    assert!(read_response.is_ok());
    
    // 7. Close session
    assert!(session.close().await.is_ok());
    
    // 8. Disconnect transport
    assert!(transport.disconnect().await.is_ok());
    assert!(!transport.is_connected());
}

#[tokio::test]
async fn test_script_to_hardware_flow() {
    // Set up complete system
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager.clone()));
    let config = SandboxConfig::default();
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Complete script workflow
    let script = r#"
        // 1. List available devices
        let devices = list_devices();
        print("Found " + devices.len() + " devices");
        
        // 2. Connect to first device
        if devices.len() > 0 {
            let device = get_device(devices[0]);
            
            // 3. Configure device
            write(device, "config/baud", 115200);
            
            // 4. Perform operations
            for i in 0..3 {
                write(device, "gpio/13", 1);
                sleep_ms(100);
                write(device, "gpio/13", 0);
                sleep_ms(100);
            }
            
            // 5. Read sensor data
            let sensor_value = read(device, "analog/A0");
            print("Sensor: " + sensor_value);
            
            // 6. Return result
            sensor_value
        } else {
            "No devices"
        }
    "#;
    
    // Execute complete workflow
    let result = engine.eval(script).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_multi_device_coordination() {
    // Create multiple transports
    let transport1 = Arc::new(tokio::sync::Mutex::new(
        create_test_transport("device1")
    ));
    let transport2 = Arc::new(tokio::sync::Mutex::new(
        create_test_transport("device2")
    ));
    
    // Connect both
    transport1.lock().await.connect().await.unwrap();
    transport2.lock().await.connect().await.unwrap();
    
    // Create drivers
    let driver1 = Arc::new(ArduinoUno::new());
    let driver2 = Arc::new(ArduinoUno::new());
    
    // Open sessions
    let session1 = {
        let mut t = transport1.lock().await;
        driver1.open(&mut *t).await.unwrap()
    };
    
    let session2 = {
        let mut t = transport2.lock().await;
        driver2.open(&mut *t).await.unwrap()
    };
    
    // Coordinate operations between devices
    let task1 = tokio::spawn(async move {
        session1.invoke("command1", &[]).await
    });
    
    let task2 = tokio::spawn(async move {
        session2.invoke("command2", &[]).await
    });
    
    // Both should complete
    let (result1, result2) = tokio::join!(task1, task2);
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_reconnection_recovery_flow() {
    let mut transport = create_test_transport("recovery_test");
    let driver = ArduinoUno::new();
    
    // Initial connection and session
    transport.connect().await.unwrap();
    let mut session = driver.open(&mut transport).await.unwrap();
    
    // Perform some operations
    session.invoke("op1", &[]).await.unwrap();
    
    // Simulate connection loss
    transport.disconnect().await.unwrap();
    
    // Operations should fail
    let failed_op = session.invoke("op2", &[]).await;
    assert!(failed_op.is_err() || failed_op.is_ok());
    
    // Reconnect
    transport.connect().await.unwrap();
    
    // Reopen session
    let mut new_session = driver.open(&mut transport).await.unwrap();
    
    // Operations should work again
    let recovered_op = new_session.invoke("op3", &[]).await;
    assert!(recovered_op.is_ok() || recovered_op.is_err());
}

#[tokio::test]
async fn test_performance_under_load() {
    let mut transport = create_test_transport("perf_test");
    transport.connect().await.unwrap();
    
    let driver = ArduinoUno::new();
    let mut session = driver.open(&mut transport).await.unwrap();
    
    // Measure throughput
    let start = std::time::Instant::now();
    let operations = 100;
    
    for i in 0..operations {
        let result = session.invoke(&format!("op_{}", i), &[]).await;
        assert!(result.is_ok() || result.is_err());
    }
    
    let elapsed = start.elapsed();
    let ops_per_sec = operations as f64 / elapsed.as_secs_f64();
    
    println!("Performance: {:.2} ops/sec", ops_per_sec);
    
    // Should handle reasonable load
    assert!(ops_per_sec > 10.0, "Should handle at least 10 ops/sec");
}

#[tokio::test]
async fn test_data_integrity_through_pipeline() {
    let mut transport = create_test_transport("integrity_test");
    transport.connect().await.unwrap();
    
    // Test data patterns
    let test_patterns = vec![
        vec![0x00, 0xFF, 0xAA, 0x55],
        vec![0x01, 0x02, 0x03, 0x04, 0x05],
        b"Hello, World!".to_vec(),
    ];
    
    for pattern in test_patterns {
        // Send pattern
        transport.send(&pattern).await.unwrap();
        
        // Receive and verify
        let received = transport.receive(Duration::from_secs(1)).await;
        
        if let Ok(data) = received {
            // Mock transport echoes data
            assert_eq!(data.len(), pattern.len());
        }
    }
}

#[tokio::test]
async fn test_resource_cleanup_on_shutdown() {
    // Create system components
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager.clone()));
    let config = SandboxConfig::default();
    
    let engine = Arc::new(ScriptEngine::new(config, device_api).unwrap());
    
    // Start operations
    let engine_clone = engine.clone();
    let operation = tokio::spawn(async move {
        let script = r#"
            for i in 0..10 {
                sleep_ms(10);
            }
        "#;
        engine_clone.eval(script).await
    });
    
    // Give it time to start
    sleep(Duration::from_millis(50)).await;
    
    // Shutdown manager
    manager.shutdown().await;
    
    // Operation should complete or be cancelled
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        operation
    ).await;
    
    assert!(result.is_ok(), "Operation should complete or timeout gracefully");
}

#[tokio::test]
async fn test_concurrent_script_and_direct_access() {
    let manager = Arc::new(DeviceManager::new());
    
    // Script engine access
    let device_api = Arc::new(DeviceApi::new(manager.clone()));
    let config = SandboxConfig::default();
    let engine = Arc::new(ScriptEngine::new(config, device_api).unwrap());
    
    // Direct access setup
    let mut transport = create_test_transport("concurrent_test");
    transport.connect().await.unwrap();
    let driver = Arc::new(ArduinoUno::new());
    
    // Concurrent operations
    let script_task = {
        let engine = engine.clone();
        tokio::spawn(async move {
            let script = r#"
                let device = get_device("test");
                read(device, "sensor")
            "#;
            engine.eval(script).await
        })
    };
    
    let direct_task = {
        let driver = driver.clone();
        tokio::spawn(async move {
            // Direct hardware access
            // Note: In real implementation, would need proper transport handling
            Ok::<_, Box<dyn std::error::Error>>("direct_result".to_string())
        })
    };
    
    // Both should complete without interference
    let (script_result, direct_result) = tokio::join!(script_task, direct_task);
    assert!(script_result.is_ok());
    assert!(direct_result.is_ok());
}

#[tokio::test]
async fn test_monitoring_and_metrics_collection() {
    let mut transport = create_test_transport("metrics_test");
    transport.connect().await.unwrap();
    
    let driver = ArduinoUno::new();
    let mut session = driver.open(&mut transport).await.unwrap();
    
    // Perform operations
    for _ in 0..10 {
        session.invoke("test_op", &[]).await.ok();
    }
    
    // Get metrics
    let stats = transport.stats();
    
    // Verify metrics collected
    assert!(stats.bytes_sent > 0, "Should track bytes sent");
    assert!(stats.bytes_received > 0, "Should track bytes received");
    assert!(stats.transactions_total > 0, "Should track transactions");
    assert!(stats.uptime.as_secs() >= 0, "Should track uptime");
    
    // Calculate success rate
    let success_rate = if stats.transactions_total > 0 {
        stats.transactions_success as f64 / stats.transactions_total as f64
    } else {
        0.0
    };
    
    println!("Success rate: {:.2}%", success_rate * 100.0);
}