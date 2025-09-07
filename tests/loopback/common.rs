/// Common utilities for loopback testing

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::transport::{Transport, TransportConfig, TransportStats};

/// Test data patterns for validation
pub struct TestPatterns {
    pub simple: Vec<u8>,
    pub binary: Vec<u8>,
    pub text: Vec<u8>,
    pub large: Vec<u8>,
    pub edge_cases: Vec<Vec<u8>>,
}

impl TestPatterns {
    pub fn new() -> Self {
        let mut large = Vec::with_capacity(8192);
        for i in 0..8192 {
            large.push((i % 256) as u8);
        }

        Self {
            simple: vec![0x01, 0x02, 0x03, 0x04],
            binary: vec![0x00, 0xFF, 0xAA, 0x55, 0xDE, 0xAD, 0xBE, 0xEF],
            text: b"Hello, Transport Layer!\r\n".to_vec(),
            large,
            edge_cases: vec![
                vec![],                              // Empty
                vec![0x00],                          // Single null
                vec![0xFF; 1],                       // Single 0xFF
                vec![0x0D, 0x0A],                    // CR LF
                vec![0x1B, 0x5B, 0x32, 0x4A],       // ANSI escape
                vec![0x00; 256],                     // All nulls
                vec![0xFF; 256],                     // All 0xFF
                (0..=255).collect(),                 // All bytes
            ],
        }
    }
}

/// Loopback test configuration
pub struct LoopbackConfig {
    pub timeout: Duration,
    pub retry_count: u32,
    pub verify_stats: bool,
    pub test_reconnection: bool,
    pub inject_errors: bool,
}

impl Default for LoopbackConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            retry_count: 3,
            verify_stats: true,
            test_reconnection: true,
            inject_errors: false,
        }
    }
}

/// Verify data integrity through loopback
pub async fn verify_loopback<T: Transport>(
    transport: &mut T,
    data: &[u8],
    config: &LoopbackConfig,
) -> Result<bool, Box<dyn std::error::Error>> {
    // Send data
    transport.send(data).await?;
    
    // Receive and verify
    let received = transport.receive(config.timeout).await?;
    
    // Compare data
    if received.len() != data.len() {
        return Ok(false);
    }
    
    for (i, (sent, recv)) in data.iter().zip(received.iter()).enumerate() {
        if sent != recv {
            eprintln!("Data mismatch at position {}: sent 0x{:02X}, received 0x{:02X}", 
                     i, sent, recv);
            return Ok(false);
        }
    }
    
    Ok(true)
}

/// Test framing and protocol compliance
pub async fn test_framing<T: Transport>(
    transport: &mut T,
    frame_delimiter: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    // Test message with delimiter
    let mut message = b"TEST_MESSAGE".to_vec();
    message.extend_from_slice(frame_delimiter);
    
    transport.send(&message).await?;
    let received = transport.receive(Duration::from_secs(2)).await?;
    
    // Verify frame structure preserved
    assert!(received.ends_with(frame_delimiter), 
            "Frame delimiter not preserved");
    
    Ok(())
}

/// Test error recovery mechanisms
pub async fn test_error_recovery<T: Transport>(
    transport: &mut T,
) -> Result<(), Box<dyn std::error::Error>> {
    // Disconnect transport
    transport.disconnect().await?;
    assert!(!transport.is_connected());
    
    // Attempt operation (should fail)
    let send_result = transport.send(b"test").await;
    assert!(send_result.is_err(), "Send should fail when disconnected");
    
    // Reconnect
    transport.connect().await?;
    assert!(transport.is_connected());
    
    // Verify operations work again
    transport.send(b"recovered").await?;
    let _ = transport.receive(Duration::from_secs(1)).await?;
    
    Ok(())
}

/// Test reconnection logic with retries
pub async fn test_reconnection_logic<T: Transport>(
    transport: &mut T,
    max_retries: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let initial_stats = transport.stats();
    
    // Force disconnect
    transport.disconnect().await?;
    
    // Attempt reconnection with retries
    let mut connected = false;
    for attempt in 1..=max_retries {
        match transport.connect().await {
            Ok(_) => {
                connected = true;
                break;
            }
            Err(e) => {
                eprintln!("Reconnection attempt {} failed: {}", attempt, e);
                tokio::time::sleep(Duration::from_millis(100 * attempt)).await;
            }
        }
    }
    
    assert!(connected, "Failed to reconnect after {} attempts", max_retries);
    
    // Verify stats updated
    let final_stats = transport.stats();
    assert!(final_stats.reconnect_count > initial_stats.reconnect_count);
    
    Ok(())
}

/// Stress test with rapid send/receive
pub async fn stress_test_loopback<T: Transport>(
    transport: &mut T,
    iterations: usize,
    data_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let test_data: Vec<u8> = (0..data_size).map(|i| (i % 256) as u8).collect();
    let mut success_count = 0;
    let mut failure_count = 0;
    
    for i in 0..iterations {
        // Modify data slightly each iteration
        let mut data = test_data.clone();
        data[0] = (i % 256) as u8;
        
        match transport.send(&data).await {
            Ok(_) => {
                match transport.receive(Duration::from_millis(100)).await {
                    Ok(received) => {
                        if received == data {
                            success_count += 1;
                        } else {
                            failure_count += 1;
                            eprintln!("Data mismatch at iteration {}", i);
                        }
                    }
                    Err(e) => {
                        failure_count += 1;
                        eprintln!("Receive failed at iteration {}: {}", i, e);
                    }
                }
            }
            Err(e) => {
                failure_count += 1;
                eprintln!("Send failed at iteration {}: {}", i, e);
            }
        }
    }
    
    let success_rate = (success_count as f64) / (iterations as f64) * 100.0;
    println!("Stress test complete: {:.2}% success rate ({}/{} successful)",
             success_rate, success_count, iterations);
    
    assert!(success_rate > 95.0, "Success rate too low: {:.2}%", success_rate);
    
    Ok(())
}

/// Verify transport statistics are correctly updated
pub fn verify_stats(
    initial: &TransportStats,
    final_stats: &TransportStats,
    expected_sends: u64,
    expected_receives: u64,
) -> bool {
    let transactions_total = final_stats.transactions_success + final_stats.transactions_failed;
    let initial_transactions_total = initial.transactions_success + initial.transactions_failed;
    let sends_diff = transactions_total - initial_transactions_total;
    let bytes_sent_diff = final_stats.bytes_sent - initial.bytes_sent;
    let bytes_recv_diff = final_stats.bytes_received - initial.bytes_received;
    
    // Verify transaction counts
    if sends_diff < expected_sends {
        eprintln!("Transaction count mismatch: expected at least {}, got {}", 
                 expected_sends, sends_diff);
        return false;
    }
    
    // Verify bytes were transmitted
    if bytes_sent_diff == 0 || bytes_recv_diff == 0 {
        eprintln!("No bytes transmitted: sent {}, received {}", 
                 bytes_sent_diff, bytes_recv_diff);
        return false;
    }
    
    true
}

/// Create a loopback pair for testing
pub async fn create_loopback_pair<T: Transport + Clone>(
    base_transport: T,
) -> (T, T) {
    let transport1 = base_transport.clone();
    let transport2 = base_transport;
    
    (transport1, transport2)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_patterns_creation() {
        let patterns = TestPatterns::new();
        assert_eq!(patterns.simple.len(), 4);
        assert_eq!(patterns.large.len(), 8192);
        assert!(!patterns.edge_cases.is_empty());
    }
    
    #[test]
    fn test_config_defaults() {
        let config = LoopbackConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(5));
        assert_eq!(config.retry_count, 3);
        assert!(config.verify_stats);
    }
}