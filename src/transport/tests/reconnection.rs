/// Comprehensive reconnection logic tests
use std::sync::{Arc, atomic::{AtomicU32, Ordering}};
use std::time::Duration;
use tokio::time::sleep;
use crate::transport::{Transport, TransportConfig, TransportError};
use crate::transport::mock::{MockTransport, MockConfig};
use crate::transport::backoff::ExponentialBackoff;

#[tokio::test]
async fn test_basic_reconnection() {
    let config = TransportConfig {
        max_reconnect_attempts: 3,
        reconnect_delay_ms: 100,
        ..Default::default()
    };
    
    let mock_config = MockConfig {
        connect_failures: 2, // Fail twice, succeed on third
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    
    // First attempt fails
    assert!(transport.connect().await.is_err());
    assert!(!transport.is_connected());
    
    // Second attempt fails
    assert!(transport.connect().await.is_err());
    assert!(!transport.is_connected());
    
    // Third attempt succeeds
    assert!(transport.connect().await.is_ok());
    assert!(transport.is_connected());
}

#[tokio::test]
async fn test_exponential_backoff() {
    let mut backoff = ExponentialBackoff::new()
        .with_initial_delay(100)
        .with_max_delay(2000)
        .with_factor(2.0)
        .with_jitter(false);
    
    // Test exponential growth
    assert_eq!(backoff.next_delay(), Some(Duration::from_millis(100)));
    assert_eq!(backoff.next_delay(), Some(Duration::from_millis(200)));
    assert_eq!(backoff.next_delay(), Some(Duration::from_millis(400)));
    assert_eq!(backoff.next_delay(), Some(Duration::from_millis(800)));
    assert_eq!(backoff.next_delay(), Some(Duration::from_millis(1600)));
    assert_eq!(backoff.next_delay(), Some(Duration::from_secs(2))); // Max cap
    assert_eq!(backoff.next_delay(), Some(Duration::from_secs(2))); // Stays at max
    
    // Test reset
    backoff.reset();
    assert_eq!(backoff.next_delay(), Some(Duration::from_millis(100)));
}

#[tokio::test]
async fn test_reconnection_with_backoff() {
    let start = std::time::Instant::now();
    
    let config = TransportConfig {
        max_reconnect_attempts: 3,
        reconnect_delay_ms: 50,
        ..Default::default()
    };
    
    let mock_config = MockConfig {
        connect_failures: 2,
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config.clone(), mock_config);
    
    // Simulate reconnection loop with backoff
    let mut backoff = ExponentialBackoff::new()
        .with_initial_delay(config.reconnect_delay_ms as u64)
        .with_max_delay(1000)
        .with_factor(2.0)
        .with_jitter(false);
    
    let mut attempts = 0;
    while attempts < 3 {
        match transport.connect().await {
            Ok(_) => break,
            Err(_) => {
                attempts += 1;
                if attempts < 3 {
                    if let Some(delay) = backoff.next_delay() {
                        sleep(delay).await;
                    }
                }
            }
        }
    }
    
    assert!(transport.is_connected());
    
    // Should have taken at least 50ms + 100ms for the delays
    let elapsed = start.elapsed();
    assert!(elapsed >= Duration::from_millis(150));
}

#[tokio::test]
async fn test_max_reconnect_attempts() {
    let config = TransportConfig {
        max_reconnect_attempts: 2,
        reconnect_delay_ms: 10,
        ..Default::default()
    };
    
    let mock_config = MockConfig {
        connect_failures: 5, // More failures than max attempts
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config.clone(), mock_config);
    
    let mut attempts = 0;
    let max = config.max_reconnect_attempts;
    
    while attempts < max {
        if transport.connect().await.is_ok() {
            break;
        }
        attempts += 1;
    }
    
    // Should still be disconnected after max attempts
    assert!(!transport.is_connected());
    assert_eq!(attempts, 2);
}

#[tokio::test]
async fn test_reconnection_after_disconnect() {
    let config = TransportConfig::default();
    
    let mock_config = MockConfig {
        disconnect_after_ops: Some(2),
        ..Default::default()
    };
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    
    // Connect successfully
    transport.connect().await.unwrap();
    assert!(transport.is_connected());
    
    // Perform operations until disconnect
    transport.send(b"data1").await.unwrap();
    transport.send(b"data2").await.unwrap();
    
    // Next operation should fail (simulated disconnect)
    assert!(transport.send(b"data3").await.is_err());
    assert!(!transport.is_connected());
    
    // Reset the mock to allow reconnection
    transport.reset().await.unwrap();
    transport.set_mock_config(MockConfig::default()).await;
    
    // Should be able to reconnect
    transport.connect().await.unwrap();
    assert!(transport.is_connected());
}

#[tokio::test]
async fn test_concurrent_reconnection_attempts() {
    let config = TransportConfig::default();
    let mock_config = MockConfig {
        connect_failures: 1,
        latency_ms: 100,
        ..Default::default()
    };
    
    let transport = Arc::new(tokio::sync::Mutex::new(
        MockTransport::new("test".into(), config, mock_config)
    ));
    
    let success_count = Arc::new(AtomicU32::new(0));
    let error_count = Arc::new(AtomicU32::new(0));
    
    // Spawn multiple concurrent connection attempts
    let mut handles = vec![];
    for i in 0..5 {
        let transport = transport.clone();
        let success_count = success_count.clone();
        let error_count = error_count.clone();
        
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(i * 10)).await;
            
            let mut transport = transport.lock().await;
            match transport.connect().await {
                Ok(_) => {
                    success_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(_) => {
                    error_count.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all attempts
    for handle in handles {
        handle.await.unwrap();
    }
    
    // At least one should succeed
    assert!(success_count.load(Ordering::Relaxed) > 0);
}

#[tokio::test]
async fn test_reconnection_clears_buffers() {
    let config = TransportConfig::default();
    let mock_config = MockConfig::default();
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    
    // Connect and send data
    transport.connect().await.unwrap();
    transport.send(b"old_data").await.unwrap();
    
    // Verify data is in buffer
    let sent = transport.get_sent_data().await;
    assert_eq!(sent, b"old_data");
    
    // Disconnect and reconnect
    transport.disconnect().await.unwrap();
    transport.connect().await.unwrap();
    
    // Send new data
    transport.send(b"new_data").await.unwrap();
    
    // Should only have new data, not old
    let sent = transport.get_sent_data().await;
    assert_eq!(sent, b"new_data");
}

#[tokio::test]
async fn test_reconnection_preserves_stats() {
    let config = TransportConfig::default();
    let mock_config = MockConfig::default();
    
    let mut transport = MockTransport::new("test".into(), config, mock_config);
    
    // First connection
    transport.connect().await.unwrap();
    transport.send(b"data1").await.unwrap();
    let stats1 = transport.stats();
    assert_eq!(stats1.bytes_sent, 5);
    assert_eq!(stats1.reconnect_count, 1);
    
    // Disconnect and reconnect
    transport.disconnect().await.unwrap();
    transport.connect().await.unwrap();
    
    // Stats should show reconnection
    let stats2 = transport.stats();
    assert_eq!(stats2.reconnect_count, 2);
    // Bytes sent should be preserved
    assert_eq!(stats2.bytes_sent, 5);
}

#[tokio::test]
async fn test_reconnection_with_jitter() {
    // Jitter helps prevent thundering herd when many clients reconnect
    let mut backoff = ExponentialBackoff::new()
        .with_initial_delay(100)
        .with_max_delay(1000)
        .with_factor(2.0)
        .with_jitter(true);
    
    // Collect delays - jitter is built into ExponentialBackoff
    let mut delays = vec![];
    for _ in 0..5 {
        if let Some(delay) = backoff.next_delay() {
            delays.push(delay);
        }
    }
    
    // Verify delays are not all identical (jitter working)
    let unique_delays: std::collections::HashSet<_> = delays.iter().collect();
    assert!(unique_delays.len() > 1);
}