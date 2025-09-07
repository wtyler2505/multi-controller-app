//! Comprehensive Driver Integration Tests
//! 
//! Tests covering multiple driver scenarios, hot-swapping, error recovery,
//! failover, transport reconnection, device manager integration, 
//! plugin loading, safety controller integration, and performance under load.

use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::{timeout, sleep};
use tokio::sync::{RwLock, mpsc, Semaphore};
use serde_json::json;

// Import all required modules
use multi_controller_app::device::{
    DeviceManager, DeviceDriver, DeviceSession, DeviceResult, DeviceError,
    Transport, TransportConfig, TransportType, DriverCapabilities, DriverInfo, DriverPriority,
    SafetyController, EmergencyStop
};
use multi_controller_app::drivers::ArduinoUnoDriver;
use multi_controller_app::transport::{MockTransport, MockConfig};

mod test_drivers;
use test_drivers::*;

mod test_utils;
use test_utils::*;

/// Comprehensive test suite for driver integration scenarios
#[cfg(test)]
mod driver_integration_tests {
    use super::*;

    /// Test multiple drivers working together simultaneously
    #[tokio::test]
    async fn test_multiple_drivers_concurrent_operation() {
        let mut test_env = TestEnvironment::new().await;
        
        // Create multiple mock drivers with different capabilities
        let arduino_driver = test_env.create_mock_arduino_driver("Arduino_1").await;
        let raspberry_pi_driver = test_env.create_mock_raspberry_pi_driver("RaspberryPi_1").await;
        let generic_driver = test_env.create_mock_generic_driver("Generic_1").await;
        
        // Register all drivers
        test_env.register_driver(arduino_driver.clone()).await;
        test_env.register_driver(raspberry_pi_driver.clone()).await;
        test_env.register_driver(generic_driver.clone()).await;
        
        // Create transports for each device
        let arduino_transport = test_env.create_mock_transport("arduino_port", TransportType::Serial).await;
        let pi_transport = test_env.create_mock_transport("pi_ssh", TransportType::SSH).await;
        let generic_transport = test_env.create_mock_transport("generic_tcp", TransportType::TCP).await;
        
        // Open concurrent sessions
        let arduino_session_result = test_env.manager.open_device(arduino_transport, Some("arduino_session".into())).await;
        let pi_session_result = test_env.manager.open_device(pi_transport, Some("pi_session".into())).await;
        let generic_session_result = test_env.manager.open_device(generic_transport, Some("generic_session".into())).await;
        
        assert!(arduino_session_result.is_ok(), "Arduino session should open successfully");
        assert!(pi_session_result.is_ok(), "Raspberry Pi session should open successfully");
        assert!(generic_session_result.is_ok(), "Generic session should open successfully");
        
        // Verify all sessions are active
        let active_sessions = test_env.manager.list_sessions().await;
        assert_eq!(active_sessions.len(), 3, "Should have 3 active sessions");
        assert!(active_sessions.contains(&"arduino_session".to_string()));
        assert!(active_sessions.contains(&"pi_session".to_string()));
        assert!(active_sessions.contains(&"generic_session".to_string()));
        
        // Perform concurrent operations on all devices
        let operations = vec![
            test_concurrent_device_operation(&test_env.manager, "arduino_session", "digitalWrite", json!([13, true])),
            test_concurrent_device_operation(&test_env.manager, "pi_session", "gpioWrite", json!([18, true])),
            test_concurrent_device_operation(&test_env.manager, "generic_session", "status", json!([])),
        ];
        
        let results = futures::future::join_all(operations).await;
        for (i, result) in results.iter().enumerate() {
            assert!(result.is_ok(), "Concurrent operation {} should succeed", i);
        }
        
        // Clean up all sessions
        test_env.cleanup_all_sessions().await;
    }

    /// Test driver hot-swapping scenarios
    #[tokio::test]
    async fn test_driver_hot_swapping() {
        let mut test_env = TestEnvironment::new().await;
        
        // Start with initial driver
        let initial_driver = test_env.create_mock_arduino_driver("Arduino_V1").await;
        test_env.register_driver(initial_driver.clone()).await;
        
        let transport = test_env.create_mock_transport("test_port", TransportType::Serial).await;
        let session_id = test_env.manager.open_device(transport.clone(), Some("hot_swap_session".into())).await.unwrap();
        
        // Verify initial session works
        let initial_result = test_concurrent_device_operation(&test_env.manager, &session_id, "digitalRead", json!([2])).await;
        assert!(initial_result.is_ok(), "Initial driver operation should work");
        
        // Simulate driver update - close existing session
        test_env.manager.close_device(&session_id).await.unwrap();
        
        // Register updated driver with enhanced capabilities
        let updated_driver = test_env.create_mock_arduino_driver("Arduino_V2").await;
        test_env.register_driver(updated_driver.clone()).await;
        
        // Open new session with updated driver
        let new_session_id = test_env.manager.open_device(transport, Some("hot_swap_session_v2".into())).await.unwrap();
        
        // Verify new session works with enhanced capabilities
        let new_result = test_concurrent_device_operation(&test_env.manager, &new_session_id, "readTemperature", json!([0, "DHT22"])).await;
        assert!(new_result.is_ok(), "Updated driver should support enhanced features");
        
        test_env.cleanup_session(&new_session_id).await;
    }

    /// Test error recovery and failover mechanisms
    #[tokio::test]
    async fn test_error_recovery_and_failover() {
        let mut test_env = TestEnvironment::new().await;
        
        // Create primary and backup drivers
        let primary_driver = test_env.create_failing_driver("Primary", 3).await;  // Fails after 3 operations
        let backup_driver = test_env.create_mock_arduino_driver("Backup").await;
        
        // Register both drivers with different priorities
        test_env.register_driver_with_priority(primary_driver.clone(), DriverPriority::High).await;
        test_env.register_driver_with_priority(backup_driver.clone(), DriverPriority::Normal).await;
        
        let transport = test_env.create_mock_transport("failover_port", TransportType::Serial).await;
        
        // Primary driver should be selected initially
        let session_id = test_env.manager.open_device(transport.clone(), Some("failover_session".into())).await.unwrap();
        
        // Perform operations until primary fails
        let mut success_count = 0;
        let mut failure_detected = false;
        
        for i in 0..10 {
            let result = test_concurrent_device_operation(&test_env.manager, &session_id, "digitalRead", json!([2])).await;
            
            if result.is_ok() {
                success_count += 1;
            } else {
                failure_detected = true;
                break;
            }
            
            sleep(Duration::from_millis(10)).await;
        }
        
        assert!(failure_detected, "Primary driver should eventually fail");
        assert!(success_count >= 3, "Should have some successful operations before failure");
        
        // Close failed session and attempt recovery with backup
        test_env.manager.close_device(&session_id).await.unwrap();
        
        // Backup driver should take over
        let backup_session_id = test_env.manager.open_device(transport, Some("backup_session".into())).await.unwrap();
        let backup_result = test_concurrent_device_operation(&test_env.manager, &backup_session_id, "digitalRead", json!([2])).await;
        
        assert!(backup_result.is_ok(), "Backup driver should handle operations successfully");
        
        test_env.cleanup_session(&backup_session_id).await;
    }

    /// Test transport reconnection with driver coordination
    #[tokio::test]
    async fn test_transport_reconnection_with_drivers() {
        let mut test_env = TestEnvironment::new().await;
        
        let driver = test_env.create_mock_arduino_driver("Arduino_Reconnect").await;
        test_env.register_driver(driver.clone()).await;
        
        // Create transport with reconnection configuration
        let mock_config = MockConfig {
            disconnect_after_ops: Some(5), // Disconnect after 5 operations
            connect_failures: 1, // Fail first reconnect attempt
            ..Default::default()
        };
        
        let transport = test_env.create_configured_mock_transport("reconnect_port", TransportType::Serial, mock_config).await;
        let session_id = test_env.manager.open_device(transport.clone(), Some("reconnect_session".into())).await.unwrap();
        
        // Perform operations until disconnection
        let mut operations_completed = 0;
        let mut reconnection_detected = false;
        
        for i in 0..20 {
            let result = test_concurrent_device_operation(&test_env.manager, &session_id, "digitalRead", json!([2])).await;
            
            if result.is_ok() {
                operations_completed += 1;
            } else {
                // Simulate driver handling reconnection
                sleep(Duration::from_millis(100)).await; // Wait for reconnection logic
                
                // Try to reopen session
                match test_env.manager.open_device(transport.clone(), Some("reconnect_session_2".into())).await {
                    Ok(new_session_id) => {
                        reconnection_detected = true;
                        test_env.cleanup_session(&new_session_id).await;
                        break;
                    }
                    Err(_) => continue,
                }
            }
            
            sleep(Duration::from_millis(10)).await;
        }
        
        assert!(operations_completed >= 5, "Should complete some operations before disconnection");
        assert!(reconnection_detected || operations_completed > 10, "Should handle reconnection or continue operating");
        
        test_env.cleanup_session(&session_id).await;
    }

    /// Test comprehensive device manager integration
    #[tokio::test]
    async fn test_device_manager_integration() {
        let mut test_env = TestEnvironment::new().await;
        
        // Test plugin loading
        let drivers = vec![
            test_env.create_mock_arduino_driver("Arduino_Manager").await,
            test_env.create_mock_raspberry_pi_driver("Pi_Manager").await,
            test_env.create_mock_generic_driver("Generic_Manager").await,
        ];
        
        for driver in drivers {
            test_env.register_driver(driver).await;
        }
        
        // Test device probing
        let serial_transport = test_env.create_mock_transport("probe_serial", TransportType::Serial).await;
        let ssh_transport = test_env.create_mock_transport("probe_ssh", TransportType::SSH).await;
        let tcp_transport = test_env.create_mock_transport("probe_tcp", TransportType::TCP).await;
        
        // Probe should find appropriate drivers
        let serial_driver = test_env.manager.probe_device(serial_transport.clone()).await;
        let ssh_driver = test_env.manager.probe_device(ssh_transport.clone()).await;
        let tcp_driver = test_env.manager.probe_device(tcp_transport.clone()).await;
        
        assert!(serial_driver.is_ok(), "Should find driver for serial transport");
        assert!(ssh_driver.is_ok(), "Should find driver for SSH transport");
        assert!(tcp_driver.is_ok(), "Should find driver for TCP transport");
        
        // Test session management
        let session_ids = vec![
            test_env.manager.open_device(serial_transport, Some("serial_session".into())).await.unwrap(),
            test_env.manager.open_device(ssh_transport, Some("ssh_session".into())).await.unwrap(),
            test_env.manager.open_device(tcp_transport, Some("tcp_session".into())).await.unwrap(),
        ];
        
        // Verify all sessions are tracked
        let active_sessions = test_env.manager.list_sessions().await;
        assert_eq!(active_sessions.len(), 3, "All sessions should be tracked");
        
        for session_id in &session_ids {
            assert!(active_sessions.contains(session_id), "Session {} should be in active list", session_id);
        }
        
        // Test batch cleanup
        for session_id in session_ids {
            test_env.cleanup_session(&session_id).await;
        }
        
        let final_sessions = test_env.manager.list_sessions().await;
        assert_eq!(final_sessions.len(), 0, "All sessions should be closed");
    }

    /// Test driver plugin loading and unloading
    #[tokio::test]
    async fn test_driver_plugin_loading() {
        let mut test_env = TestEnvironment::new().await;
        
        // Test initial state - no drivers loaded
        let initial_sessions = test_env.manager.list_sessions().await;
        assert_eq!(initial_sessions.len(), 0, "Should start with no sessions");
        
        // Simulate plugin loading
        let plugin_drivers = vec![
            ("ArduinoPlugin", test_env.create_mock_arduino_driver("ArduinoPlugin").await),
            ("PiPlugin", test_env.create_mock_raspberry_pi_driver("PiPlugin").await),
            ("GenericPlugin", test_env.create_mock_generic_driver("GenericPlugin").await),
        ];
        
        // Load plugins dynamically
        for (name, driver) in &plugin_drivers {
            test_env.register_driver(driver.clone()).await;
            
            // Test that newly loaded driver works immediately
            let transport = test_env.create_mock_transport(&format!("{}_port", name), TransportType::Serial).await;
            let session_id = test_env.manager.open_device(transport, Some(format!("{}_session", name))).await.unwrap();
            
            let operation_result = test_concurrent_device_operation(&test_env.manager, &session_id, "digitalRead", json!([2])).await;
            assert!(operation_result.is_ok(), "Newly loaded plugin {} should work immediately", name);
            
            test_env.cleanup_session(&session_id).await;
        }
        
        // Test plugin priority and selection
        let transport = test_env.create_mock_transport("priority_test", TransportType::Serial).await;
        let driver_result = test_env.manager.probe_device(transport.clone()).await;
        assert!(driver_result.is_ok(), "Should find an appropriate driver from loaded plugins");
        
        // Verify the selected driver
        let selected_driver = driver_result.unwrap();
        assert!(!selected_driver.name().is_empty(), "Selected driver should have a name");
    }

    /// Test safety controller integration with drivers
    #[tokio::test]
    async fn test_safety_controller_integration() {
        let mut test_env = TestEnvironment::new().await;
        
        let driver = test_env.create_mock_arduino_driver("Arduino_Safety").await;
        test_env.register_driver(driver.clone()).await;
        
        let transport = test_env.create_mock_transport("safety_port", TransportType::Serial).await;
        let session_id = test_env.manager.open_device(transport, Some("safety_session".into())).await.unwrap();
        
        // Test normal operation first
        let normal_result = test_concurrent_device_operation(&test_env.manager, &session_id, "digitalRead", json!([2])).await;
        assert!(normal_result.is_ok(), "Normal operation should work before emergency stop");
        
        // Trigger emergency stop
        test_env.manager.emergency_stop("Test emergency stop".into()).await;
        
        // Wait for emergency stop to take effect
        sleep(Duration::from_millis(100)).await;
        
        // All sessions should be closed
        let active_sessions = test_env.manager.list_sessions().await;
        assert_eq!(active_sessions.len(), 0, "All sessions should be closed after emergency stop");
        
        // New sessions should fail while emergency stop is active
        let emergency_transport = test_env.create_mock_transport("emergency_port", TransportType::Serial).await;
        let emergency_session_result = test_env.manager.open_device(emergency_transport, Some("emergency_session".into())).await;
        assert!(emergency_session_result.is_err(), "Should not be able to open sessions during emergency stop");
        
        // Reset emergency stop
        test_env.manager.reset_emergency_stop().await;
        
        // Wait for reset to take effect
        sleep(Duration::from_millis(100)).await;
        
        // Normal operation should resume
        let recovery_transport = test_env.create_mock_transport("recovery_port", TransportType::Serial).await;
        let recovery_session_result = test_env.manager.open_device(recovery_transport, Some("recovery_session".into())).await;
        assert!(recovery_session_result.is_ok(), "Should be able to open sessions after emergency stop reset");
        
        if let Ok(recovery_session_id) = recovery_session_result {
            let recovery_result = test_concurrent_device_operation(&test_env.manager, &recovery_session_id, "digitalRead", json!([2])).await;
            assert!(recovery_result.is_ok(), "Operations should work after emergency stop reset");
            test_env.cleanup_session(&recovery_session_id).await;
        }
    }

    /// Test performance under high load scenarios
    #[tokio::test]
    async fn test_performance_under_load() {
        let mut test_env = TestEnvironment::new().await;
        
        // Create multiple drivers for load testing
        let load_drivers = (0..5).map(|i| {
            test_env.create_mock_arduino_driver(&format!("LoadTest_{}", i))
        }).collect::<Vec<_>>();
        
        let mut driver_futures = Vec::new();
        for driver_future in load_drivers {
            driver_futures.push(driver_future);
        }
        
        let drivers = futures::future::join_all(driver_futures).await;
        
        for driver in drivers {
            test_env.register_driver(driver).await;
        }
        
        let start_time = Instant::now();
        let session_count = 20;
        let operations_per_session = 50;
        
        // Create multiple concurrent sessions
        let mut session_handles = Vec::new();
        
        for i in 0..session_count {
            let transport = test_env.create_mock_transport(&format!("load_port_{}", i), TransportType::Serial).await;
            let session_id = test_env.manager.open_device(transport, Some(format!("load_session_{}", i))).await.unwrap();
            session_handles.push(session_id);
        }
        
        let session_creation_time = start_time.elapsed();
        println!("Created {} sessions in {:?}", session_count, session_creation_time);
        
        // Perform concurrent operations across all sessions
        let operation_start = Instant::now();
        let semaphore = Arc::new(Semaphore::new(10)); // Limit concurrent operations
        
        let mut operation_tasks = Vec::new();
        
        for session_id in &session_handles {
            for op_num in 0..operations_per_session {
                let manager = &test_env.manager;
                let session_id = session_id.clone();
                let semaphore = semaphore.clone();
                
                let task = async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    
                    let operation_types = ["digitalRead", "analogRead", "digitalWrite"];
                    let operation = operation_types[op_num % operation_types.len()];
                    
                    let args = match operation {
                        "digitalRead" => json!([2]),
                        "analogRead" => json!([0]),
                        "digitalWrite" => json!([13, op_num % 2 == 0]),
                        _ => json!([]),
                    };
                    
                    test_concurrent_device_operation(manager, &session_id, operation, args).await
                };
                
                operation_tasks.push(task);
            }
        }
        
        let operation_results = futures::future::join_all(operation_tasks).await;
        let operation_duration = operation_start.elapsed();
        
        let total_operations = session_count * operations_per_session;
        let successful_operations = operation_results.iter().filter(|r| r.is_ok()).count();
        let operations_per_second = successful_operations as f64 / operation_duration.as_secs_f64();
        
        println!("Completed {} operations in {:?}", total_operations, operation_duration);
        println!("Success rate: {}/{} ({:.1}%)", successful_operations, total_operations, 
                 successful_operations as f64 / total_operations as f64 * 100.0);
        println!("Operations per second: {:.1}", operations_per_second);
        
        // Performance assertions
        assert!(operations_per_second > 100.0, "Should achieve at least 100 operations per second");
        assert!(successful_operations as f64 / total_operations as f64 > 0.95, "Should have >95% success rate");
        assert!(session_creation_time.as_millis() < 1000, "Session creation should be fast");
        
        // Cleanup all sessions
        for session_id in session_handles {
            test_env.cleanup_session(&session_id).await;
        }
        
        let cleanup_time = Instant::now();
        let final_sessions = test_env.manager.list_sessions().await;
        let cleanup_duration = cleanup_time.elapsed();
        
        assert_eq!(final_sessions.len(), 0, "All sessions should be cleaned up");
        assert!(cleanup_duration.as_millis() < 500, "Cleanup should be fast");
        
        println!("Load test completed successfully");
    }

    /// Test driver compatibility matrix
    #[tokio::test]
    async fn test_driver_transport_compatibility() {
        let mut test_env = TestEnvironment::new().await;
        
        // Create drivers with different transport support
        let arduino_driver = test_env.create_mock_arduino_driver("Arduino_Compat").await; // Serial only
        let pi_driver = test_env.create_mock_raspberry_pi_driver("Pi_Compat").await; // SSH, Serial
        let generic_driver = test_env.create_mock_generic_driver("Generic_Compat").await; // All transports
        
        test_env.register_driver(arduino_driver.clone()).await;
        test_env.register_driver(pi_driver.clone()).await;
        test_env.register_driver(generic_driver.clone()).await;
        
        // Test compatibility matrix
        let transport_types = vec![
            TransportType::Serial,
            TransportType::SSH,
            TransportType::TCP,
            TransportType::UDP,
        ];
        
        let mut compatibility_results = HashMap::new();
        
        for transport_type in transport_types {
            let transport = test_env.create_mock_transport(&format!("compat_{:?}", transport_type), transport_type).await;
            
            match test_env.manager.probe_device(transport.clone()).await {
                Ok(driver) => {
                    compatibility_results.insert(transport_type, Some(driver.name().to_string()));
                    
                    // Test that the session actually works
                    if let Ok(session_id) = test_env.manager.open_device(transport, Some(format!("compat_{:?}_session", transport_type))).await {
                        let operation_result = test_concurrent_device_operation(&test_env.manager, &session_id, "digitalRead", json!([2])).await;
                        assert!(operation_result.is_ok(), "Compatible driver should handle operations on {:?}", transport_type);
                        test_env.cleanup_session(&session_id).await;
                    }
                }
                Err(_) => {
                    compatibility_results.insert(transport_type, None);
                }
            }
        }
        
        // Verify expected compatibility
        assert!(compatibility_results.get(&TransportType::Serial).is_some(), "Should find driver for Serial");
        
        println!("Driver compatibility matrix:");
        for (transport, driver) in compatibility_results {
            match driver {
                Some(name) => println!("  {:?}: {}", transport, name),
                None => println!("  {:?}: No compatible driver", transport),
            }
        }
    }

    /// Test concurrent driver failures and recovery
    #[tokio::test]
    async fn test_concurrent_driver_failures_and_recovery() {
        let mut test_env = TestEnvironment::new().await;
        
        // Create multiple drivers with different failure patterns
        let stable_driver = test_env.create_mock_arduino_driver("Stable").await;
        let intermittent_driver = test_env.create_failing_driver("Intermittent", 10).await; // Fails every 10th operation
        let unstable_driver = test_env.create_failing_driver("Unstable", 3).await; // Fails every 3rd operation
        
        test_env.register_driver(stable_driver).await;
        test_env.register_driver(intermittent_driver).await;
        test_env.register_driver(unstable_driver).await;
        
        // Create multiple sessions
        let session_configs = vec![
            ("stable_session", "stable_port"),
            ("intermittent_session", "intermittent_port"),
            ("unstable_session", "unstable_port"),
        ];
        
        let mut session_ids = Vec::new();
        for (session_name, port_name) in session_configs {
            let transport = test_env.create_mock_transport(port_name, TransportType::Serial).await;
            let session_id = test_env.manager.open_device(transport, Some(session_name.into())).await.unwrap();
            session_ids.push(session_id);
        }
        
        // Run concurrent operations with failure monitoring
        let operation_count = 100;
        let mut failure_counts = HashMap::new();
        let mut recovery_counts = HashMap::new();
        
        for i in 0..operation_count {
            let mut operation_tasks = Vec::new();
            
            for session_id in &session_ids {
                let manager = &test_env.manager;
                let session_id = session_id.clone();
                
                let task = async move {
                    test_concurrent_device_operation(manager, &session_id, "digitalRead", json!([2])).await
                };
                
                operation_tasks.push(task);
            }
            
            let results = futures::future::join_all(operation_tasks).await;
            
            // Track failures and recoveries
            for (session_idx, result) in results.iter().enumerate() {
                let session_id = &session_ids[session_idx];
                
                match result {
                    Ok(_) => {
                        // Check if this is a recovery from previous failure
                        if failure_counts.get(session_id).unwrap_or(&0) > recovery_counts.get(session_id).unwrap_or(&0) {
                            *recovery_counts.entry(session_id.clone()).or_insert(0) += 1;
                        }
                    }
                    Err(_) => {
                        *failure_counts.entry(session_id.clone()).or_insert(0) += 1;
                    }
                }
            }
            
            // Small delay between operation rounds
            if i % 10 == 0 {
                sleep(Duration::from_millis(10)).await;
            }
        }
        
        println!("Concurrent failure analysis:");
        for session_id in &session_ids {
            let failures = failure_counts.get(session_id).unwrap_or(&0);
            let recoveries = recovery_counts.get(session_id).unwrap_or(&0);
            let success_rate = (operation_count - failures) as f64 / operation_count as f64 * 100.0;
            
            println!("  {}: {} failures, {} recoveries, {:.1}% success rate", 
                     session_id, failures, recoveries, success_rate);
        }
        
        // Verify that stable driver has high success rate
        let stable_failures = failure_counts.get(&session_ids[0]).unwrap_or(&0);
        let stable_success_rate = (operation_count - stable_failures) as f64 / operation_count as f64;
        assert!(stable_success_rate > 0.95, "Stable driver should have >95% success rate");
        
        // Verify that unstable drivers have some failures but also recoveries
        for session_id in &session_ids[1..] {
            let failures = failure_counts.get(session_id).unwrap_or(&0);
            assert!(*failures > 0, "Unstable drivers should have some failures");
            
            let recoveries = recovery_counts.get(session_id).unwrap_or(&0);
            if *failures > 10 {
                assert!(*recoveries > 0, "Should have some recoveries after multiple failures");
            }
        }
        
        // Cleanup
        for session_id in session_ids {
            test_env.cleanup_session(&session_id).await;
        }
    }
}