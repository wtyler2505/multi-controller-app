/// Comprehensive error handling and recovery tests
use std::time::Duration;
use crate::transport::{Transport, TransportConfig, TransportError};
use crate::transport::mock::{MockTransport, MockConfig};

#[tokio::test]
async fn test_not_connected_errors() {
    let config = TransportConfig::default();
    let mock_config = MockConfig::default();
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    
    // All operations should fail when not connected
    assert!(matches!(
        transport.send(b"data").await,
        Err(TransportError::NotConnected)
    ));
    
    assert!(matches!(
        transport.receive(Duration::from_millis(100)).await,
        Err(TransportError::NotConnected)
    ));
    
    assert!(matches!(
        transport.transact(b"data", Duration::from_millis(100)).await,
        Err(TransportError::NotConnected)
    ));
}

#[tokio::test]
async fn test_send_failure_handling() {
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        send_failures: 3,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // First 3 sends should fail
    for i in 1..=3 {
        let result = transport.send(b"test").await;
        assert!(result.is_err());
        assert!(matches!(result, Err(TransportError::IoError(_))));
        
        // Verify stats track failures
        let stats = transport.stats();
        assert_eq!(stats.transactions_failed, i);
    }
    
    // Fourth send should succeed
    assert!(transport.send(b"test").await.is_ok());
    let stats = transport.stats();
    assert_eq!(stats.transactions_success, 1);
    assert_eq!(stats.transactions_failed, 3);
}

#[tokio::test]
async fn test_receive_timeout() {
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        receive_failures: 2,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // First 2 receives should timeout
    for _ in 0..2 {
        let result = transport.receive(Duration::from_millis(50)).await;
        assert!(matches!(result, Err(TransportError::Timeout(_))));
    }
    
    // Third receive should succeed (echoes empty buffer)
    let result = transport.receive(Duration::from_millis(50)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_partial_data_handling() {
    let config = TransportConfig::default();
    let mock_config = MockConfig::default();
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // Send partial data
    transport.send(b"partial").await.unwrap();
    
    // Inject additional data
    transport.inject_receive_data(b"complete".to_vec()).await.unwrap();
    
    // Should receive injected data, not sent data
    let received = transport.receive(Duration::from_millis(100)).await.unwrap();
    assert_eq!(received, b"complete");
}

#[tokio::test]
async fn test_connection_lost_during_operation() {
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        disconnect_after_ops: Some(1),
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // First operation succeeds
    transport.send(b"data").await.unwrap();
    
    // Second operation fails due to disconnect
    let result = transport.send(b"data").await;
    assert!(matches!(result, Err(TransportError::ConnectionFailed(_))));
    assert!(!transport.is_connected());
}

#[tokio::test]
async fn test_error_message_propagation() {
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        connect_failures: 1,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    
    // Capture error message
    let error = transport.connect().await.unwrap_err();
    match error {
        TransportError::ConnectionFailed(msg) => {
            assert!(msg.contains("Mock connect failure 1/1"));
        }
        _ => panic!("Unexpected error type"),
    }
}

#[tokio::test]
async fn test_transact_error_handling() {
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        send_failures: 1,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // Transact should fail on send error
    let result = transport.transact(b"data", Duration::from_millis(100)).await;
    assert!(result.is_err());
    
    // Second attempt should succeed
    let result = transport.transact(b"data", Duration::from_millis(100)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_reset_clears_error_state() {
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        send_failures: 1,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config.clone());
    transport.connect().await.unwrap();
    
    // First send fails
    assert!(transport.send(b"data").await.is_err());
    
    // Reset transport
    transport.reset().await.unwrap();
    transport.reset_counters();
    transport.set_mock_config(mock_config).await;
    
    // Disconnect and reconnect to reset internal state
    transport.disconnect().await.unwrap();
    transport.connect().await.unwrap();
    
    // Send should fail again (counter reset)
    assert!(transport.send(b"data").await.is_err());
    // Second send succeeds
    assert!(transport.send(b"data").await.is_ok());
}

#[tokio::test]
async fn test_cleanup_resources_on_error() {
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        disconnect_after_ops: Some(1),
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // Operation that triggers disconnect
    transport.send(b"data").await.unwrap();
    transport.send(b"data").await.unwrap_err();
    
    // Cleanup should work even after error
    assert!(transport.cleanup_resources().await.is_ok());
    assert!(!transport.is_connected());
}

#[tokio::test]
async fn test_error_recovery_sequence() {
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        send_failures: 1,
        receive_failures: 1,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    transport.connect().await.unwrap();
    
    // Test recovery sequence
    // 1. Send fails
    assert!(transport.send(b"data").await.is_err());
    
    // 2. Send succeeds
    assert!(transport.send(b"data").await.is_ok());
    
    // 3. Receive fails
    assert!(transport.receive(Duration::from_millis(100)).await.is_err());
    
    // 4. Receive succeeds
    assert!(transport.receive(Duration::from_millis(100)).await.is_ok());
    
    // Verify connection remained stable
    assert!(transport.is_connected());
}

#[tokio::test] 
async fn test_concurrent_error_handling() {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        send_failures: 5,
        ..Default::default()
    };
    
    let transport = Arc::new(Mutex::new(
        MockTransport::new("test".into(), config, mock_config)
    ));
    
    // Connect first
    transport.lock().await.connect().await.unwrap();
    
    // Spawn multiple tasks that will encounter errors
    let mut handles = vec![];
    for i in 0..10 {
        let transport = transport.clone();
        let handle = tokio::spawn(async move {
            let mut transport = transport.lock().await;
            let result = transport.send(format!("data{}", i).as_bytes()).await;
            result.is_ok()
        });
        handles.push(handle);
    }
    
    // Collect results
    let mut successes = 0;
    let mut failures = 0;
    for handle in handles {
        if handle.await.unwrap() {
            successes += 1;
        } else {
            failures += 1;
        }
    }
    
    // First 5 should fail, rest succeed
    assert_eq!(failures, 5);
    assert_eq!(successes, 5);
}