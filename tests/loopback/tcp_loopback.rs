/// TCP transport loopback tests
/// Tests TCP socket communication with local loopback

use super::common::*;
use crate::transport::{Transport, TransportConfig, TransportType, TransportSettings, TcpSettings};
use crate::transport::tcp::TcpTransport;
use std::time::Duration;
use tokio;
use tokio::net::TcpListener;

/// Configuration for TCP loopback testing
struct TcpLoopbackConfig {
    host: String,
    port: u16,
    start_echo_server: bool,
}

impl Default for TcpLoopbackConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8765,
            start_echo_server: true,
        }
    }
}

/// Simple echo server for testing
async fn start_echo_server(port: u16) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await
            .expect("Failed to bind echo server");
        
        while let Ok((mut socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let (reader, writer) = socket.split();
                let _ = tokio::io::copy(&mut tokio::io::BufReader::new(reader), 
                                       &mut tokio::io::BufWriter::new(writer)).await;
            });
        }
    })
}

/// Create a TCP transport for testing
async fn create_tcp_transport(config: &TcpLoopbackConfig) -> TcpTransport {
    let transport_config = TransportConfig {
        transport_type: TransportType::Tcp,
        address: format!("{}:{}", config.host, config.port),
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
        settings: TransportSettings::Tcp(TcpSettings {
            host: config.host.clone(),
            port: config.port,
            ..Default::default()
        }),
    };
    
    TcpTransport::new(transport_config).expect("Failed to create TCP transport")
}

#[tokio::test]
async fn test_tcp_basic_loopback() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_tcp_transport(&config).await;
    
    // Connect
    transport.connect().await.expect("Failed to connect");
    assert!(transport.is_connected());
    
    // Test simple data
    let test_data = b"Hello TCP!";
    transport.send(test_data).await.expect("Failed to send");
    
    let received = transport.receive(Duration::from_secs(2)).await
        .expect("Failed to receive");
    
    assert_eq!(received, test_data, "Data mismatch in loopback");
    
    // Disconnect
    transport.disconnect().await.expect("Failed to disconnect");
}

#[tokio::test]
async fn test_tcp_large_data() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 1).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 1;
    let mut transport = create_tcp_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Test large data (64KB)
    let large_data: Vec<u8> = (0..65536).map(|i| (i % 256) as u8).collect();
    
    transport.send(&large_data).await.expect("Send failed");
    
    let mut total_received = Vec::new();
    let start = std::time::Instant::now();
    
    while total_received.len() < large_data.len() && start.elapsed() < Duration::from_secs(5) {
        match transport.receive(Duration::from_millis(100)).await {
            Ok(data) => total_received.extend_from_slice(&data),
            Err(_) => continue,
        }
    }
    
    assert_eq!(total_received.len(), large_data.len(), 
               "Not all large data received");
    assert_eq!(total_received, large_data, 
               "Large data corrupted in transmission");
}

#[tokio::test]
async fn test_tcp_multiple_messages() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 2).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 2;
    let mut transport = create_tcp_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Send multiple messages
    let messages = vec![
        b"First message".to_vec(),
        b"Second message".to_vec(),
        b"Third message with special chars: \x00\xFF\xAA".to_vec(),
    ];
    
    for msg in &messages {
        transport.send(msg).await.expect("Send failed");
        let received = transport.receive(Duration::from_secs(1)).await
            .expect("Receive failed");
        assert_eq!(&received, msg, "Message corrupted");
    }
}

#[tokio::test]
async fn test_tcp_reconnection() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 3).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 3;
    let mut transport = create_tcp_transport(&config).await;
    
    // Initial connection
    transport.connect().await.expect("Initial connection failed");
    
    // Test data before disconnect
    transport.send(b"Before disconnect").await.expect("Send failed");
    let _ = transport.receive(Duration::from_millis(500)).await;
    
    // Disconnect and reconnect
    transport.disconnect().await.expect("Disconnect failed");
    assert!(!transport.is_connected());
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    transport.connect().await.expect("Reconnection failed");
    assert!(transport.is_connected());
    
    // Test data after reconnect
    transport.send(b"After reconnect").await.expect("Send after reconnect failed");
    let received = transport.receive(Duration::from_secs(1)).await
        .expect("Receive after reconnect failed");
    
    assert_eq!(received, b"After reconnect");
}

#[tokio::test]
async fn test_tcp_concurrent_operations() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 4).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 4;
    let transport = std::sync::Arc::new(tokio::sync::Mutex::new(
        create_tcp_transport(&config).await
    ));
    
    transport.lock().await.connect().await.expect("Failed to connect");
    
    // Spawn concurrent send operations
    let mut handles = vec![];
    
    for i in 0..10 {
        let transport_clone = transport.clone();
        let handle = tokio::spawn(async move {
            let data = format!("Concurrent message {}", i);
            let mut t = transport_clone.lock().await;
            t.send(data.as_bytes()).await.expect("Concurrent send failed");
        });
        handles.push(handle);
    }
    
    // Wait for all sends to complete
    for handle in handles {
        handle.await.expect("Task panicked");
    }
    
    // Receive all messages
    let mut received_count = 0;
    let start = std::time::Instant::now();
    
    while received_count < 10 && start.elapsed() < Duration::from_secs(5) {
        let mut t = transport.lock().await;
        if let Ok(_) = t.receive(Duration::from_millis(100)).await {
            received_count += 1;
        }
    }
    
    assert_eq!(received_count, 10, "Not all concurrent messages received");
}

#[tokio::test]
async fn test_tcp_error_recovery() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 5).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 5;
    let mut transport = create_tcp_transport(&config).await;
    
    transport.connect().await.expect("Initial connection failed");
    
    test_error_recovery(&mut transport).await
        .expect("Error recovery test failed");
}

#[tokio::test]
async fn test_tcp_stress() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 6).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 6;
    let mut transport = create_tcp_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Stress test with 200 iterations of 512 byte packets
    stress_test_loopback(&mut transport, 200, 512).await
        .expect("Stress test failed");
}

#[tokio::test]
async fn test_tcp_nagle_algorithm() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 7).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 7;
    let mut transport = create_tcp_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Send small packets rapidly (test Nagle's algorithm)
    let small_data = b"X";
    let send_count = 100;
    
    let start = std::time::Instant::now();
    for _ in 0..send_count {
        transport.send(small_data).await.expect("Send failed");
    }
    let send_time = start.elapsed();
    
    // Receive all data
    let mut total_received = 0;
    let receive_start = std::time::Instant::now();
    
    while total_received < send_count && 
          receive_start.elapsed() < Duration::from_secs(5) {
        if let Ok(data) = transport.receive(Duration::from_millis(10)).await {
            total_received += data.len();
        }
    }
    
    println!("Nagle test: sent {} bytes in {:?}, received {}", 
             send_count, send_time, total_received);
    
    assert_eq!(total_received, send_count, "Not all small packets received");
}

#[tokio::test]
async fn test_tcp_keep_alive() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 8).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 8;
    let mut transport = create_tcp_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Send initial data
    transport.send(b"Keep alive test").await.expect("Initial send failed");
    let _ = transport.receive(Duration::from_millis(500)).await;
    
    // Wait for potential timeout (simulate idle connection)
    tokio::time::sleep(Duration::from_secs(3)).await;
    
    // Connection should still be alive
    assert!(transport.is_connected(), "Connection died during idle period");
    
    // Should still be able to send
    transport.send(b"Still alive").await.expect("Send after idle failed");
    let received = transport.receive(Duration::from_secs(1)).await
        .expect("Receive after idle failed");
    
    assert_eq!(received, b"Still alive");
}

#[tokio::test]
async fn test_tcp_fragmentation() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 9).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 9;
    let mut transport = create_tcp_transport(&config).await;
    
    transport.connect().await.expect("Failed to connect");
    
    // Send data that will be fragmented (> MTU)
    let large_packet: Vec<u8> = (0..2048).map(|i| (i % 256) as u8).collect();
    
    transport.send(&large_packet).await.expect("Send large packet failed");
    
    // Receive potentially fragmented data
    let mut total_received = Vec::new();
    let start = std::time::Instant::now();
    
    while total_received.len() < large_packet.len() && 
          start.elapsed() < Duration::from_secs(2) {
        if let Ok(data) = transport.receive(Duration::from_millis(100)).await {
            total_received.extend_from_slice(&data);
        }
    }
    
    assert_eq!(total_received, large_packet, 
               "Fragmented packet not correctly reassembled");
}

#[tokio::test]
async fn test_tcp_binary_patterns() {
    let config = TcpLoopbackConfig::default();
    let _server = start_echo_server(config.port + 10).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut config = config;
    config.port += 10;
    let mut transport = create_tcp_transport(&config).await;
    let patterns = TestPatterns::new();
    
    transport.connect().await.expect("Failed to connect");
    
    // Test all edge case patterns
    for (i, pattern) in patterns.edge_cases.iter().enumerate() {
        transport.send(pattern).await
            .expect(&format!("Failed to send pattern {}", i));
        
        if pattern.is_empty() {
            // Can't receive empty data, skip
            continue;
        }
        
        let received = transport.receive(Duration::from_secs(1)).await
            .expect(&format!("Failed to receive pattern {}", i));
        
        assert_eq!(received, *pattern, 
                   "Pattern {} corrupted in transmission", i);
    }
}