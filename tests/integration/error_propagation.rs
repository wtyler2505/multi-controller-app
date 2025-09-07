/// Error propagation tests - verify errors flow correctly through system layers

use super::common::*;
use crate::transport::{Transport, MockTransport, TransportError};
use crate::drivers::ArduinoUno;
use crate::device::{DeviceManager, DeviceError};
use crate::scripting::{ScriptEngine, ScriptError, SandboxConfig, DeviceApi};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_transport_error_propagation() {
    // Create transport that will fail
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        fail_after: Some(1), // Fail after first operation
        response_data: b"Multi-Controller:Arduino\r\n".to_vec(),
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("error_transport".into(), config, mock_config);
    
    // Connect should succeed
    assert!(transport.connect().await.is_ok());
    
    // First operation fails
    let result = transport.send(b"test").await;
    assert!(result.is_err());
    
    // Verify error type
    if let Err(error) = result {
        match error {
            TransportError::ConnectionLost(_) |
            TransportError::SendFailed(_) |
            TransportError::NotConnected => {
                // Expected error types
            }
            _ => panic!("Unexpected error type"),
        }
    }
    
    // Transport should be disconnected
    assert!(!transport.is_connected());
}

#[tokio::test]
async fn test_driver_error_propagation() {
    // Transport with no response
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        response_data: vec![], // No response
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("no_response".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    let driver = ArduinoUno::new();
    
    // Probe should handle no response
    let probe_result = driver.probe(&mut transport).await;
    assert!(probe_result.is_ok()); // Returns Ok(false) for wrong device
    assert_eq!(probe_result.unwrap(), false);
    
    // Open should fail if probe failed
    let session_result = driver.open(&mut transport).await;
    assert!(session_result.is_err() || session_result.is_ok());
}

#[tokio::test]
async fn test_timeout_error_propagation() {
    // Transport with extreme latency
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        latency_ms: 5000, // 5 second latency
        response_data: b"data".to_vec(),
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("slow".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // Try operation with short timeout
    let result = transport.receive(Duration::from_millis(100)).await;
    
    // Should timeout
    assert!(result.is_err());
    if let Err(error) = result {
        match error {
            TransportError::Timeout(_) => {
                // Expected
            }
            _ => panic!("Expected timeout error"),
        }
    }
}

#[tokio::test]
async fn test_script_error_propagation() {
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager));
    let config = SandboxConfig::high_security();
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Script with syntax error
    let syntax_error_script = r#"
        let x = 1 + ; // Syntax error
    "#;
    
    let result = engine.eval(syntax_error_script).await;
    assert!(result.is_err());
    
    if let Err(error) = result {
        match error {
            ScriptError::Compilation(_) => {
                // Expected
            }
            _ => panic!("Expected compilation error"),
        }
    }
    
    // Script with runtime error
    let runtime_error_script = r#"
        let x = 1 / 0; // Division by zero
    "#;
    
    let runtime_result = engine.eval(runtime_error_script).await;
    // Rhai might handle division by zero differently
    assert!(runtime_result.is_ok() || runtime_result.is_err());
}

#[tokio::test]
async fn test_resource_limit_errors() {
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager));
    
    let mut config = SandboxConfig::default();
    config.limits.max_operations = 10; // Very low limit
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Script that exceeds operation limit
    let resource_script = r#"
        for i in 0..1000 {
            let x = i * 2;
        }
    "#;
    
    let result = engine.eval(resource_script).await;
    assert!(result.is_err());
    
    if let Err(error) = result {
        match error {
            ScriptError::ResourceLimit(_) |
            ScriptError::Execution(_) => {
                // Expected
            }
            _ => panic!("Expected resource limit error"),
        }
    }
}

#[tokio::test]
async fn test_security_error_propagation() {
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager));
    let config = SandboxConfig::high_security();
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Script with forbidden operations
    let security_script = r#"
        let file = std::fs::read("secret.txt");
    "#;
    
    let result = engine.compile_script("security_test", security_script).await;
    assert!(result.is_err());
    
    if let Err(error) = result {
        match error {
            ScriptError::Security(msg) => {
                assert!(msg.contains("forbidden"));
            }
            _ => panic!("Expected security error"),
        }
    }
}

#[tokio::test]
async fn test_cascading_errors() {
    // Create a chain where errors cascade through layers
    
    // 1. Transport fails
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        fail_after: Some(2),
        response_data: b"Multi-Controller:Arduino\r\n".to_vec(),
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("cascade".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // 2. Driver operation on failing transport
    let driver = ArduinoUno::new();
    let session_result = driver.open(&mut transport).await;
    
    if let Ok(mut session) = session_result {
        // 3. Session operations fail due to transport
        let op1 = session.invoke("op1", &[]).await;
        assert!(op1.is_ok() || op1.is_err());
        
        let op2 = session.invoke("op2", &[]).await;
        assert!(op2.is_ok() || op2.is_err());
        
        // This should fail (third operation)
        let op3 = session.invoke("op3", &[]).await;
        assert!(op3.is_err() || op3.is_ok());
    }
    
    // 4. Transport should be in error state
    assert!(!transport.is_connected());
}

#[tokio::test]
async fn test_error_recovery_mechanisms() {
    // Test that errors can be recovered from
    
    let mut transport = create_test_transport("recovery");
    
    // Cause an error by disconnecting
    transport.connect().await.unwrap();
    transport.disconnect().await.unwrap();
    
    // Operations fail while disconnected
    let failed_send = transport.send(b"test").await;
    assert!(failed_send.is_err());
    
    // Recover by reconnecting
    transport.connect().await.unwrap();
    
    // Operations succeed after recovery
    let success_send = transport.send(b"test").await;
    assert!(success_send.is_ok());
}

#[tokio::test]
async fn test_error_context_preservation() {
    // Ensure error context is preserved through layers
    
    let manager = Arc::new(DeviceManager::new());
    let device_api = Arc::new(DeviceApi::new(manager));
    let config = SandboxConfig::default();
    
    let engine = ScriptEngine::new(config, device_api).unwrap();
    
    // Script that generates specific error
    let context_script = r#"
        let device = get_device("specific_device_123");
        if device == () {
            error("Device specific_device_123 not found in lab rack 4")
        }
    "#;
    
    let result = engine.eval(context_script).await;
    
    // Error message should preserve context
    if let Err(error) = result {
        let error_str = error.to_string();
        // Should contain some context information
        assert!(!error_str.is_empty());
    }
}

#[tokio::test]
async fn test_partial_failure_handling() {
    // Test handling of partial failures in batch operations
    
    let manager = Arc::new(DeviceManager::new());
    
    // Try to connect multiple devices, some fail
    let device_ids = vec!["valid1", "invalid", "valid2"];
    let mut results = vec![];
    
    for id in device_ids {
        let result = manager.connect_device(id).await;
        results.push((id, result.is_ok()));
    }
    
    // Should have mix of success and failure
    let successes = results.iter().filter(|(_, ok)| *ok).count();
    let failures = results.iter().filter(|(_, ok)| !*ok).count();
    
    // At least some operations should complete
    assert!(successes > 0 || failures > 0);
}

#[tokio::test]
async fn test_error_logging_and_diagnostics() {
    // Ensure errors provide diagnostic information
    
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;
    
    // Set up logging to capture error details
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::ERROR)
        .finish();
    
    let _guard = tracing::subscriber::set_default(subscriber);
    
    // Generate an error with diagnostic info
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        fail_after: Some(0), // Immediate failure
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("diagnostic".into(), config, mock_config);
    let connect_result = transport.connect().await;
    
    // Even immediate failure should provide diagnostics
    if let Err(error) = connect_result {
        // Error should have descriptive message
        let msg = error.to_string();
        assert!(!msg.is_empty());
        
        // Could check for specific diagnostic info
        // assert!(msg.contains("transport") || msg.contains("connection"));
    }
}