/// UDP transport loopback tests
/// Tests UDP datagram communication with local loopback

use super::common::*;
use crate::transport::{Transport, TransportConfig, TransportType, TransportSettings, UdpSettings};
use crate::transport::udp::UdpTransport;
use std::time::Duration;
use std::sync::Arc;
use tokio;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;

/// Configuration for UDP loopback testing
struct UdpLoopbackConfig {
    local_addr: String,
    remote_addr: String,
    start_echo_server: bool,
}

impl Default for UdpLoopbackConfig {
    fn default() -> Self {
        Self {
            local_addr: "127.0.0.1:9876".to_string(),
            remote_addr: "127.0.0.1:9877".to_string(),
            start_echo_server: true,
        }
    }
}

/// Simple UDP echo server for testing
async fn start_udp_echo_server(addr: &str) -> tokio::task::JoinHandle<()> {
    let addr = addr.to_string();
    tokio::spawn(async move {
        let socket = UdpSocket::bind(&addr).await
            .expect("Failed to bind UDP echo server");
        
        let mut buf = vec![0u8; 65536];
        
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, src)) => {
                    // Echo back to sender
                    let _ = socket.send_to(&buf[..len], src).await;
                }
                Err(e) => {
                    eprintln!("UDP echo server error: {}", e);
                    break;
                }
            }
        }
    })
}

/// Create a UDP transport for testing
async fn create_udp_transport(config: &UdpLoopbackConfig) -> UdpTransport {
    let transport_config = TransportConfig {
        transport_type: TransportType::Udp,
        address: config.remote_addr.clone(),
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
        settings: TransportSettings::Udp(UdpSettings {
            host: config.remote_addr.split(':').next().unwrap_or("127.0.0.1").to_string(),
            port: config.remote_addr.split(':').nth(1).unwrap_or("8766").parse().unwrap_or(8766),
            bind_port: config.local_addr.split(':').nth(1).unwrap_or("0").parse().unwrap_or(0),
            ..Default::default()
        }),
    };
    
    UdpTransport::new(transport_config).expect("Failed to create UDP transport")
}

#[tokio::test]
async fn test_udp_basic_loopback() {
    let config = UdpLoopbackConfig::default();
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    
    // Connect (bind local socket)
    transport.connect().await.expect("Failed to connect");
    assert!(transport.is_connected());
    
    // Test simple datagram
    let test_data = b"Hello UDP!";
    transport.send(test_data).await.expect("Failed to send");
    
    let received = transport.receive(Duration::from_secs(2)).await
        .expect("Failed to receive");
    
    assert_eq!(received, test_data, "Data mismatch in loopback");
    
    // Disconnect
    transport.disconnect().await.expect("Failed to disconnect");
}

#[tokio::test]
async fn test_udp_datagram_boundaries() {
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9878".to_string();
    config.remote_addr = "127.0.0.1:9879".to_string();
    
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Failed to connect");
    
    // UDP preserves message boundaries
    let messages = vec![
        b"First".to_vec(),
        b"Second".to_vec(),
        b"Third".to_vec(),
    ];
    
    for msg in &messages {
        transport.send(msg).await.expect("Send failed");
    }
    
    // Each should be received as separate datagram
    for expected in &messages {
        let received = transport.receive(Duration::from_secs(1)).await
            .expect("Receive failed");
        assert_eq!(received, *expected, 
                   "UDP datagram boundary not preserved");
    }
}

#[tokio::test]
async fn test_udp_max_datagram_size() {
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9880".to_string();
    config.remote_addr = "127.0.0.1:9881".to_string();
    
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Failed to connect");
    
    // Test maximum practical UDP datagram size (65507 bytes)
    // IP header (20) + UDP header (8) = 28 bytes overhead
    let max_size = 65507;
    let large_datagram: Vec<u8> = (0..max_size).map(|i| (i % 256) as u8).collect();
    
    transport.send(&large_datagram).await.expect("Send max datagram failed");
    
    let received = transport.receive(Duration::from_secs(5)).await
        .expect("Receive max datagram failed");
    
    assert_eq!(received.len(), large_datagram.len(), 
               "Max datagram size not preserved");
    assert_eq!(received, large_datagram, 
               "Max datagram corrupted");
}

#[tokio::test]
async fn test_udp_packet_loss_simulation() {
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9882".to_string();
    config.remote_addr = "127.0.0.1:9883".to_string();
    
    // Create lossy echo server
    let remote_addr = config.remote_addr.clone();
    let _server = tokio::spawn(async move {
        let socket = UdpSocket::bind(&remote_addr).await.unwrap();
        let mut buf = vec![0u8; 65536];
        let mut packet_count = 0;
        
        loop {
            if let Ok((len, src)) = socket.recv_from(&mut buf).await {
                packet_count += 1;
                // Simulate 20% packet loss
                if packet_count % 5 != 0 {
                    let _ = socket.send_to(&buf[..len], src).await;
                }
            }
        }
    });
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Failed to connect");
    
    // Send 10 packets
    let mut sent = 0;
    let mut received = 0;
    
    for i in 0..10 {
        let data = format!("Packet {}", i);
        transport.send(data.as_bytes()).await.expect("Send failed");
        sent += 1;
        
        // Try to receive with short timeout
        if let Ok(_) = transport.receive(Duration::from_millis(200)).await {
            received += 1;
        }
    }
    
    println!("UDP packet loss test: sent {}, received {}", sent, received);
    
    // Should receive at least 70% (accounting for simulated loss)
    assert!(received >= 7, "Too many packets lost: {}/10", received);
}

#[tokio::test]
async fn test_udp_out_of_order() {
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9884".to_string();
    config.remote_addr = "127.0.0.1:9885".to_string();
    
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Failed to connect");
    
    // Send numbered packets
    for i in 0..10 {
        let data = format!("SEQ:{:03}", i);
        transport.send(data.as_bytes()).await.expect("Send failed");
    }
    
    // Receive and check sequence
    let mut received_sequences = Vec::new();
    for _ in 0..10 {
        if let Ok(data) = transport.receive(Duration::from_millis(500)).await {
            let seq_str = std::str::from_utf8(&data).unwrap();
            if let Some(seq) = seq_str.strip_prefix("SEQ:") {
                let seq_num: u32 = seq.parse().unwrap();
                received_sequences.push(seq_num);
            }
        }
    }
    
    // UDP doesn't guarantee order, but in loopback should mostly be ordered
    println!("Received sequence: {:?}", received_sequences);
    
    // Check all packets arrived (order may vary)
    assert_eq!(received_sequences.len(), 10, "Not all packets received");
}

#[tokio::test]
async fn test_udp_broadcast() {
    // Note: Actual broadcast requires special permissions and network config
    // This tests the pattern with loopback
    
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9886".to_string();
    config.remote_addr = "127.0.0.1:9887".to_string();
    
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Failed to connect");
    
    // Send broadcast-style message
    let broadcast_msg = b"BROADCAST:Hello all listeners!";
    transport.send(broadcast_msg).await.expect("Broadcast send failed");
    
    let received = transport.receive(Duration::from_secs(1)).await
        .expect("Broadcast receive failed");
    
    assert_eq!(received, broadcast_msg);
}

#[tokio::test]
async fn test_udp_multicast_pattern() {
    // Note: Actual multicast requires special network configuration
    // This tests the pattern with loopback
    
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9888".to_string();
    config.remote_addr = "127.0.0.1:9889".to_string();
    
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Failed to connect");
    
    // Send multicast-style messages
    let group_messages = vec![
        b"MULTICAST:Group1:Message1".to_vec(),
        b"MULTICAST:Group2:Message2".to_vec(),
        b"MULTICAST:Group1:Message3".to_vec(),
    ];
    
    for msg in &group_messages {
        transport.send(msg).await.expect("Multicast send failed");
        let received = transport.receive(Duration::from_millis(500)).await
            .expect("Multicast receive failed");
        assert_eq!(received, *msg);
    }
}

#[tokio::test]
async fn test_udp_fragmentation_reassembly() {
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9890".to_string();
    config.remote_addr = "127.0.0.1:9891".to_string();
    
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Failed to connect");
    
    // Send data that would fragment at IP level (> 1500 MTU)
    let fragment_size = 2048;
    let test_data: Vec<u8> = (0..fragment_size).map(|i| (i % 256) as u8).collect();
    
    transport.send(&test_data).await.expect("Send fragmented failed");
    
    let received = transport.receive(Duration::from_secs(2)).await
        .expect("Receive fragmented failed");
    
    assert_eq!(received.len(), test_data.len(), 
               "Fragmented data size mismatch");
    assert_eq!(received, test_data, 
               "Fragmented data corrupted");
}

#[tokio::test]
async fn test_udp_rapid_fire() {
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9892".to_string();
    config.remote_addr = "127.0.0.1:9893".to_string();
    
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Failed to connect");
    
    // Send packets as fast as possible
    let packet_count = 1000;
    let start = std::time::Instant::now();
    
    for i in 0..packet_count {
        let data = format!("Rapid {}", i);
        transport.send(data.as_bytes()).await.expect("Rapid send failed");
    }
    
    let send_time = start.elapsed();
    let send_rate = packet_count as f64 / send_time.as_secs_f64();
    
    // Try to receive all
    let mut received_count = 0;
    let receive_start = std::time::Instant::now();
    
    while received_count < packet_count && 
          receive_start.elapsed() < Duration::from_secs(10) {
        if let Ok(_) = transport.receive(Duration::from_millis(10)).await {
            received_count += 1;
        }
    }
    
    println!("UDP rapid fire: sent {} packets in {:?} ({:.2} pkt/s), received {}",
             packet_count, send_time, send_rate, received_count);
    
    // Allow some loss at high rates
    assert!(received_count >= packet_count * 95 / 100, 
            "Too many packets lost in rapid fire");
}

#[tokio::test]
async fn test_udp_concurrent_sockets() {
    // Test multiple UDP transports on different ports
    let base_port = 9900;
    let socket_count = 5;
    
    let mut servers = vec![];
    let mut transports = vec![];
    
    for i in 0..socket_count {
        let local_port = base_port + i * 2;
        let remote_port = base_port + i * 2 + 1;
        
        let config = UdpLoopbackConfig {
            local_addr: format!("127.0.0.1:{}", local_port),
            remote_addr: format!("127.0.0.1:{}", remote_port),
            start_echo_server: true,
        };
        
        servers.push(start_udp_echo_server(&config.remote_addr).await);
        transports.push(Arc::new(Mutex::new(create_udp_transport(&config).await)));
    }
    
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    // Connect all transports
    for transport in &transports {
        transport.lock().await.connect().await.expect("Connect failed");
    }
    
    // Send from each transport concurrently
    let mut handles = vec![];
    
    for (i, transport) in transports.iter().enumerate() {
        let transport_clone = transport.clone();
        let handle = tokio::spawn(async move {
            let data = format!("Socket {} message", i);
            let mut t = transport_clone.lock().await;
            t.send(data.as_bytes()).await.expect("Concurrent send failed");
            t.receive(Duration::from_secs(1)).await.expect("Concurrent receive failed")
        });
        handles.push(handle);
    }
    
    // Wait for all operations
    for handle in handles {
        let received = handle.await.expect("Task panicked");
        assert!(!received.is_empty(), "No data received from concurrent socket");
    }
}

#[tokio::test]
async fn test_udp_error_recovery() {
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9910".to_string();
    config.remote_addr = "127.0.0.1:9911".to_string();
    
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Initial connection failed");
    
    test_error_recovery(&mut transport).await
        .expect("Error recovery test failed");
}

#[tokio::test]
async fn test_udp_stress() {
    let mut config = UdpLoopbackConfig::default();
    config.local_addr = "127.0.0.1:9912".to_string();
    config.remote_addr = "127.0.0.1:9913".to_string();
    
    let _server = start_udp_echo_server(&config.remote_addr).await;
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let mut transport = create_udp_transport(&config).await;
    transport.connect().await.expect("Failed to connect");
    
    // Stress test with 500 iterations of 256 byte packets
    // Allow lower success rate for UDP
    let iterations = 500;
    let data_size = 256;
    
    let test_data: Vec<u8> = (0..data_size).map(|i| (i % 256) as u8).collect();
    let mut success_count = 0;
    
    for i in 0..iterations {
        let mut data = test_data.clone();
        data[0] = (i % 256) as u8;
        
        if transport.send(&data).await.is_ok() {
            if let Ok(received) = transport.receive(Duration::from_millis(50)).await {
                if received == data {
                    success_count += 1;
                }
            }
        }
    }
    
    let success_rate = (success_count as f64) / (iterations as f64) * 100.0;
    println!("UDP stress test: {:.2}% success rate ({}/{})",
             success_rate, success_count, iterations);
    
    // UDP can have some loss, especially under stress
    assert!(success_rate > 90.0, "UDP success rate too low: {:.2}%", success_rate);
}