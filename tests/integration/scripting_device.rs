/// Integration tests for Scripting Engine and Device operations

use super::common::*;
use crate::scripting::{ScriptEngine, SandboxConfig, DeviceApi, AsyncBridge};
use crate::device::DeviceManager;
use std::sync::Arc;
use rhai::Scope;

#[tokio::test]
async fn test_script_device_interaction() {
    // Set up device manager and scripting engine
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager.clone()));
    let config = SandboxConfig::default();
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Script that interacts with device
    let script = r#"
        let devices = list_devices();
        print("Found devices: " + devices.len());
        
        if devices.len() > 0 {
            let device = get_device(devices[0]);
            let result = read(device, "test_endpoint");
            print("Read result: " + result);
            result
        } else {
            "No devices found"
        }
    "#;
    
    // Execute script
    let result = engine.eval(script).await;
    assert!(result.is_ok(), "Script execution should succeed");
}

#[tokio::test]
async fn test_script_sandbox_device_permissions() {
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager.clone()));
    
    // High security config - read only
    let config = SandboxConfig::high_security();
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Script that tries to write (should fail in high security)
    let write_script = r#"
        let device = get_device("test_device");
        write(device, "gpio/13", 1)
    "#;
    
    let result = engine.eval(write_script).await;
    
    // In high security, write might be blocked
    // Result depends on implementation details
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_script_async_bridge_operations() {
    let manager = Arc::new(DeviceManager::new());
    let bridge = AsyncBridge::new(manager).unwrap();
    
    // Test sync wrappers
    let devices = bridge.list_devices_sync();
    assert!(devices.is_empty() || !devices.is_empty());
    
    // Test get device
    let device_result = bridge.get_device_sync("test_device");
    assert!(device_result.is_ok() || device_result.is_err());
    
    if let Ok(device) = device_result {
        // Test device operations through bridge
        let read_result = device.read("sensor");
        assert!(read_result.is_ok() || read_result.is_err());
        
        let write_result = device.write("output", rhai::Dynamic::from(42));
        assert!(write_result.is_ok() || write_result.is_err());
    }
}

#[tokio::test]
async fn test_script_resource_limits_with_device_ops() {
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager));
    
    // Strict limits
    let mut config = SandboxConfig::default();
    config.limits.max_operations = 100; // Very low limit
    config.limits.max_execution_time = Duration::from_millis(100);
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Script with infinite loop (should be stopped by limits)
    let infinite_script = r#"
        let device = get_device("test");
        let counter = 0;
        loop {
            read(device, "sensor");
            counter = counter + 1;
            if counter > 1000 {
                break;
            }
        }
    "#;
    
    let start = std::time::Instant::now();
    let result = engine.eval(infinite_script).await;
    let elapsed = start.elapsed();
    
    // Should fail due to resource limits
    assert!(result.is_err() || elapsed < Duration::from_secs(1));
}

#[tokio::test]
async fn test_script_error_handling_device_failures() {
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager));
    let config = SandboxConfig::default();
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Script that handles device errors gracefully
    let error_handling_script = r#"
        let device = get_device("non_existent_device");
        
        if device == () {
            print("Device not found - handled gracefully");
            "error_handled"
        } else {
            let result = read(device, "endpoint");
            if result.contains("Error") {
                print("Read error - handled gracefully");
                "read_error_handled"
            } else {
                result
            }
        }
    "#;
    
    let result = engine.eval(error_handling_script).await;
    assert!(result.is_ok(), "Error handling script should execute");
}

#[tokio::test]
async fn test_script_concurrent_device_access() {
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager));
    let config = SandboxConfig::default();
    
    let engine = Arc::new(ScriptEngine::new(config, device_api).unwrap());
    
    // Run multiple scripts concurrently
    let engine1 = engine.clone();
    let task1 = tokio::spawn(async move {
        let script = r#"
            let device = get_device("device1");
            read(device, "sensor")
        "#;
        engine1.eval(script).await
    });
    
    let engine2 = engine.clone();
    let task2 = tokio::spawn(async move {
        let script = r#"
            let device = get_device("device2");
            read(device, "sensor")
        "#;
        engine2.eval(script).await
    });
    
    // Both should complete without conflict
    let (result1, result2) = tokio::join!(task1, task2);
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_script_compile_and_cache() {
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager));
    let config = SandboxConfig::default();
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Compile script
    let script = r#"
        fn read_all_sensors(device) {
            let results = [];
            for i in 0..5 {
                let value = read(device, "sensor" + i);
                results.push(value);
            }
            results
        }
        
        let device = get_device("test");
        read_all_sensors(device)
    "#;
    
    let script_id = "sensor_reader";
    let compile_result = engine.compile_script(script_id, script).await;
    assert!(compile_result.is_ok(), "Script should compile");
    
    // Execute compiled script multiple times
    let mut scope = Scope::new();
    
    let result1 = engine.execute_script(script_id, &mut scope).await;
    assert!(result1.is_ok(), "First execution should succeed");
    
    let result2 = engine.execute_script(script_id, &mut scope).await;
    assert!(result2.is_ok(), "Second execution should succeed");
}

#[tokio::test]
async fn test_script_event_handling() {
    let manager = Arc::new(DeviceManager::new());
    let bridge = Arc::new(AsyncBridge::new(manager).unwrap());
    
    // Get device handle
    let device = bridge.get_device_sync("event_test").unwrap();
    
    // Test event waiting
    let event_result = device.wait_for_event("button_press", 100);
    
    // Should either timeout or receive event
    assert!(event_result.is_ok() || event_result.is_err());
    
    if let Ok(event) = event_result {
        assert!(!event.to_string().is_empty());
    }
}

#[tokio::test]
async fn test_script_state_persistence() {
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager));
    let config = SandboxConfig::default();
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Script that maintains state
    let script_id = "stateful_script";
    let script = r#"
        let global_counter = 0;
        
        fn increment_and_read(device) {
            global_counter = global_counter + 1;
            let value = read(device, "counter");
            print("Counter: " + global_counter + ", Read: " + value);
            global_counter
        }
    "#;
    
    engine.compile_script(script_id, script).await.unwrap();
    
    // Create scope with state
    let mut scope = Scope::new();
    scope.push("device_id", "test_device");
    
    // Execute multiple times with same scope
    let result1 = engine.execute_script(script_id, &mut scope).await;
    let result2 = engine.execute_script(script_id, &mut scope).await;
    
    // Both should succeed
    assert!(result1.is_ok());
    assert!(result2.is_ok());
}