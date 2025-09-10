/// Serial transport loopback tests
/// Tests serial port communication with hardware or virtual loopback

use super::common::*;
use crate::transport::{Transport, TransportConfig, TransportType, TransportSettings, SerialSettings};
use crate::transport::serial::SerialTransport;
use std::time::Duration;
use tokio;

/// Configuration for serial loopback testing
struct SerialLoopbackConfig {
    port: String,
    baud_rate: u32,
    use_hardware_loopback: bool,
}

impl Default for SerialLoopbackConfig {
    fn default() -> Self {
        Self {
            port: if cfg!(target_os = "windows") {
                "COM99".to_string()  // Virtual port for testing
            } else {
                "/dev/ttyUSB99".to_string()
            },
            baud_rate: 115200,
            use_hardware_loopback: false,
        }
    }
}

/// Create a serial transport for testing
async fn create_serial_transport(config: &SerialLoopbackConfig) -> SerialTransport {
    let transport_config = TransportConfig {
        transport_type: TransportType::Serial,
        address: config.port.clone(),
        connect_timeout_ms: 5000,
        read_timeout_ms: 1000,
        write_timeout_ms: 1000,
        auto_reconnect: true,
        max_reconnect_attempts: 3,
        reconnect_delay_ms: 100,
        read_buffer_size: 4096,
        write_buffer_size: 4096,
        require_handshake: false,
        min_latency: Some(Duration::from_millis(50)),
        settings: TransportSettings::Serial(SerialSettings {
            baud_rate: config.baud_rate,
            ..Default::default()
        }),
    };
    
    SerialTransport::new(transport_config).expect("Failed to create serial transport")
}

#[tokio::test]
#[ignore] // Requires hardware or virtual serial port
async fn test_serial_basic_loopback() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    
    // Connect
    transport.connect().await.expect("Failed to connect");
    assert!(transport.is_connected());
    
    // Test simple data
    let test_data = b"Hello Serial!\r\n";
    transport.send(test_data).await.expect("Failed to send");
    
    let received = transport.receive(Duration::from_secs(2)).await
        .expect("Failed to receive");
    
    assert_eq!(received, test_data, "Data mismatch in loopback");
    
    // Disconnect
    transport.disconnect().await.expect("Failed to disconnect");
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_framing() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Test different frame endings
    let frame_endings = vec![
        b"\r\n",     // CRLF
        b"\n",       // LF only
        b"\r",       // CR only
        b"\x00",     // Null terminator
        b"\x03",     // ETX
    ];
    
    for ending in frame_endings {
        let mut message = b"FRAME_TEST".to_vec();
        message.extend_from_slice(ending);
        
        transport.send(&message).await.expect("Send failed");
        let received = transport.receive(Duration::from_secs(1)).await
            .expect("Receive failed");
        
        assert!(received.ends_with(ending), 
                "Frame ending not preserved for {:?}", ending);
    }
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_binary_data() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    let patterns = TestPatterns::new();
    
    transport.connect().await.expect("Failed to connect");
    
    // Test binary patterns
    for pattern in &[patterns.binary, patterns.edge_cases[7].clone()] {
        transport.send(pattern).await.expect("Send failed");
        let received = transport.receive(Duration::from_secs(2)).await
            .expect("Receive failed");
        
        assert_eq!(received.len(), pattern.len(), 
                   "Length mismatch for binary data");
        
        for (i, (sent, recv)) in pattern.iter().zip(received.iter()).enumerate() {
            assert_eq!(sent, recv, 
                      "Byte mismatch at position {}: sent 0x{:02X}, received 0x{:02X}",
                      i, sent, recv);
        }
    }
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_flow_control() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Send large data to test flow control
    let large_data: Vec<u8> = (0..16384).map(|i| (i % 256) as u8).collect();
    
    // Send in chunks
    for chunk in large_data.chunks(1024) {
        transport.send(chunk).await.expect("Send chunk failed");
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    // Receive and verify
    let mut total_received = Vec::new();
    while total_received.len() < large_data.len() {
        match transport.receive(Duration::from_millis(100)).await {
            Ok(data) => total_received.extend_from_slice(&data),
            Err(_) => break,
        }
    }
    
    assert_eq!(total_received.len(), large_data.len(), 
               "Not all data received through flow control");
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_reconnection() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    let loopback_config = LoopbackConfig::default();
    
    transport.connect().await.expect("Initial connection failed");
    
    // Test reconnection
    test_reconnection_logic(&mut transport, loopback_config.retry_count).await
        .expect("Reconnection test failed");
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_error_recovery() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    
    transport.connect().await.expect("Initial connection failed");
    
    test_error_recovery(&mut transport).await
        .expect("Error recovery test failed");
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_stress() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Stress test with 100 iterations of 256 byte packets
    stress_test_loopback(&mut transport, 100, 256).await
        .expect("Stress test failed");
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_stats_tracking() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    let initial_stats = transport.stats();
    
    // Perform operations
    for i in 0..10 {
        let data = format!("Message {}\r\n", i);
        transport.send(data.as_bytes()).await.expect("Send failed");
        let _ = transport.receive(Duration::from_millis(100)).await;
    }
    
    let final_stats = transport.stats();
    
    assert!(verify_stats(&initial_stats, &final_stats, 10, 10),
            "Stats not properly tracked");
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_latency_enforcement() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Test that 50ms latency is enforced
    let start = std::time::Instant::now();
    
    for _ in 0..5 {
        transport.send(b"LATENCY_TEST\r\n").await.expect("Send failed");
    }
    
    let elapsed = start.elapsed();
    
    // Should take at least 250ms (5 * 50ms)
    assert!(elapsed >= Duration::from_millis(240), 
            "Latency enforcement not working: elapsed {:?}", elapsed);
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_partial_reads() {
    let config = SerialLoopbackConfig::default();
    let mut transport = create_serial_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Send message in parts
    transport.send(b"PART1").await.expect("Send part 1 failed");
    tokio::time::sleep(Duration::from_millis(50)).await;
    transport.send(b"PART2").await.expect("Send part 2 failed");
    tokio::time::sleep(Duration::from_millis(50)).await;
    transport.send(b"\r\n").await.expect("Send terminator failed");
    
    // Should receive complete message
    let received = transport.receive(Duration::from_secs(2)).await
        .expect("Receive failed");
    
    let expected = b"PART1PART2\r\n";
    assert_eq!(received, expected, "Partial reads not assembled correctly");
}

/// Helper to test with different baud rates
async fn test_baud_rate(baud: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = SerialLoopbackConfig::default();
    config.baud_rate = baud;
    
    let mut transport = create_serial_transport(&config).await;
    transport.connect().await?;
    
    let test_data = b"BAUD_TEST\r\n";
    transport.send(test_data).await?;
    let received = transport.receive(Duration::from_secs(2)).await?;
    
    assert_eq!(received, test_data);
    transport.disconnect().await?;
    
    Ok(())
}

#[tokio::test]
#[ignore] // Requires hardware
async fn test_serial_multiple_baud_rates() {
    let baud_rates = vec![9600, 19200, 38400, 57600, 115200];
    
    for baud in baud_rates {
        match test_baud_rate(baud).await {
            Ok(_) => println!("Baud rate {} OK", baud),
            Err(e) => eprintln!("Baud rate {} failed: {}", baud, e),
        }
    }
}