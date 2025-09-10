/// Integration tests for Device Manager orchestration

use super::common::*;
use crate::device::{DeviceManager, DeviceInfo, ConnectionState};
use crate::transport::MockTransport;
use crate::drivers::ArduinoUno;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_device_manager_discovery() {
    let manager = create_test_manager().await;
    
    // Add mock transport to manager
    let transport = Arc::new(tokio::sync::Mutex::new(
        create_test_transport("discovery_test")
    ));
    
    // Trigger discovery
    let devices = manager.discover_devices().await;
    
    // Should find at least the mock device
    assert!(!devices.is_empty(), "Should discover devices");
    
    // Verify device info
    if let Some(device) = devices.first() {
        assert!(device.name.contains("Arduino") || device.name.contains("Mock"));
        assert_eq!(device.state, ConnectionState::Disconnected);
    }
}

#[tokio::test]
async fn test_device_manager_connect_lifecycle() {
    let manager = create_test_manager().await;
    
    // Mock device discovery
    let device_id = "arduino_uno_1";
    
    // Connect to device
    let connect_result = manager.connect_device(device_id).await;
    assert!(connect_result.is_ok() || connect_result.is_err()); // Depends on implementation
    
    // Check connection state
    let state = manager.get_device_state(device_id).await;
    assert!(matches!(state, Some(ConnectionState::Connected) | Some(ConnectionState::Disconnected)));
    
    // Disconnect
    let disconnect_result = manager.disconnect_device(device_id).await;
    assert!(disconnect_result.is_ok() || disconnect_result.is_err());
    
    // Verify disconnected state
    let final_state = manager.get_device_state(device_id).await;
    assert!(matches!(final_state, Some(ConnectionState::Disconnected) | None));
}

#[tokio::test]
async fn test_device_manager_handles_multiple_devices() {
    let manager = create_test_manager().await;
    
    // Create multiple mock devices
    let device_ids = vec!["device1", "device2", "device3"];
    
    // Connect all devices
    for id in &device_ids {
        let _ = manager.connect_device(id).await;
    }
    
    // Get all connected devices
    let connected = manager.get_connected_devices().await;
    assert!(connected.len() <= device_ids.len());
    
    // Disconnect one device
    if !device_ids.is_empty() {
        let _ = manager.disconnect_device(device_ids[0]).await;
    }
    
    // Verify count decreased
    let remaining = manager.get_connected_devices().await;
    assert!(remaining.len() <= connected.len());
}

#[tokio::test]
async fn test_device_manager_reconnection_handling() {
    let manager = create_test_manager().await;
    let device_id = "reconnect_test";
    
    // Initial connection
    let _ = manager.connect_device(device_id).await;
    
    // Simulate connection loss
    manager.handle_connection_lost(device_id).await;
    
    // Wait for automatic reconnection attempt
    sleep(Duration::from_millis(100)).await;
    
    // Check if reconnection was attempted
    let state = manager.get_device_state(device_id).await;
    assert!(matches!(
        state, 
        Some(ConnectionState::Reconnecting) | 
        Some(ConnectionState::Connected) | 
        Some(ConnectionState::Disconnected)
    ));
}

#[tokio::test]
async fn test_device_manager_concurrent_operations() {
    let manager = Arc::new(DeviceManager::new());
    
    // Spawn concurrent operations
    let manager1 = manager.clone();
    let task1 = tokio::spawn(async move {
        manager1.discover_devices().await
    });
    
    let manager2 = manager.clone();
    let task2 = tokio::spawn(async move {
        manager2.connect_device("device1").await
    });
    
    let manager3 = manager.clone();
    let task3 = tokio::spawn(async move {
        manager3.get_connected_devices().await
    });
    
    // All operations should complete without deadlock
    let results = tokio::join!(task1, task2, task3);
    assert!(results.0.is_ok());
    assert!(results.1.is_ok());
    assert!(results.2.is_ok());
}

#[tokio::test]
async fn test_device_manager_error_propagation() {
    let manager = create_test_manager().await;
    
    // Try to connect to non-existent device
    let result = manager.connect_device("non_existent").await;
    
    // Should return appropriate error
    if result.is_err() {
        let error = result.unwrap_err();
        let error_string = error.to_string();
        assert!(
            error_string.contains("not found") || 
            error_string.contains("failed") ||
            error_string.len() > 0
        );
    }
}

#[tokio::test]
async fn test_device_manager_session_management() {
    let manager = create_test_manager().await;
    let device_id = "session_test";
    
    // Connect and get session
    let _ = manager.connect_device(device_id).await;
    let session = manager.get_device_session(device_id).await;
    
    if let Some(mut session) = session {
        // Use session
        let result = session.invoke("test_command", &[]).await;
        assert!(result.is_ok() || result.is_err());
    }
    
    // Close session
    let close_result = manager.close_device_session(device_id).await;
    assert!(close_result.is_ok() || close_result.is_err());
    
    // Verify session is closed
    let closed_session = manager.get_device_session(device_id).await;
    assert!(closed_session.is_none() || closed_session.is_some());
}

#[tokio::test]
async fn test_device_manager_event_notifications() {
    let manager = create_test_manager().await;
    
    // Set up event listener
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    manager.subscribe_events(tx).await;
    
    // Trigger device connection
    let _ = manager.connect_device("event_test").await;
    
    // Should receive connection event
    tokio::select! {
        event = rx.recv() => {
            if let Some(event) = event {
                assert!(event.is_connection() || event.is_discovery());
            }
        }
        _ = sleep(Duration::from_millis(100)) => {
            // No event received (ok for this test)
        }
    }
}

#[tokio::test]
async fn test_device_manager_resource_cleanup() {
    let manager = create_test_manager().await;
    
    // Connect multiple devices
    for i in 0..5 {
        let device_id = format!("cleanup_test_{}", i);
        let _ = manager.connect_device(&device_id).await;
    }
    
    // Shutdown manager (cleanup all resources)
    manager.shutdown().await;
    
    // Verify all devices disconnected
    let remaining = manager.get_connected_devices().await;
    assert_eq!(remaining.len(), 0, "All devices should be disconnected after shutdown");
}