use async_trait::async_trait;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio::task::JoinHandle;

use crate::transport::{
    Transport, TransportBase, TransportConfig, TransportError, TransportResult,
    TransportStats, TransportType, ConnectionState,
};
use crate::transport::common::UdpSettings;

/// UDP transport implementation
pub struct UdpTransport {
    base: TransportBase,
    socket: Option<Arc<UdpSocket>>,
    settings: UdpSettings,
    remote_addr: Option<SocketAddr>,
    reconnect_attempts: u32,
    task_handles: Vec<JoinHandle<()>>,  // Track spawned tasks for cleanup
    cleanup_flag: Arc<AtomicBool>,      // Signal for cooperative shutdown
}

impl UdpTransport {
    /// Trigger automatic reconnection in the background
    async fn trigger_auto_reconnection(&self) {
        let address = self.base.config.address.clone();
        
        // Create a closure that attempts to reconnect
        let connect_fn = move || -> std::pin::Pin<Box<dyn std::future::Future<Output = TransportResult<()>> + Send>> {
            let addr = address.clone();
            Box::pin(async move {
                // For UDP, "reconnection" means re-binding the socket
                // This is a simplified implementation - real one would need to restore socket
                tracing::info!("UDP reconnection attempt to {}", addr);
                
                // In real implementation, would recreate socket here
                // For now just return Ok to simulate successful reconnection
                Ok(())
            })
        };
        
        // Trigger reconnection through TransportBase
        if let Err(e) = self.base.trigger_reconnection(connect_fn).await {
            tracing::error!("Failed to trigger UDP reconnection: {}", e);
        }
    }
    
    /// Create a new UDP transport
    pub fn new(config: TransportConfig) -> TransportResult<Self> {
        let settings = match config.settings {
            crate::transport::common::TransportSettings::Udp(ref udp) => udp.clone(),
            _ => return Err(TransportError::ConfigError("Invalid settings for UDP transport".into())),
        };
        
        Ok(UdpTransport {
            base: TransportBase::new(
                format!("UDP:{}:{}", settings.host, settings.port),
                TransportType::Udp,
                config,
            ),
            socket: None,
            settings,
            remote_addr: None,
            reconnect_attempts: 0,
            task_handles: Vec::new(),
            cleanup_flag: Arc::new(AtomicBool::new(false)),
        })
    }
    
    /// Discover UDP services via broadcast
    pub async fn discover_broadcast(port: u16, timeout_ms: u64) -> TransportResult<Vec<ServiceInfo>> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.set_broadcast(true)?;
        
        // Send discovery packet to broadcast address
        let broadcast_addr = format!("255.255.255.255:{}", port)
            .parse::<SocketAddr>()
            .map_err(|e| TransportError::ConfigError(format!("Invalid broadcast address: {}", e)))?;
        
        socket.send_to(b"DISCOVER", broadcast_addr).await?;
        
        let mut discovered = Vec::new();
        let mut buffer = vec![0u8; 1024];
        
        // Listen for responses with timeout
        let deadline = Instant::now() + Duration::from_millis(timeout_ms);
        while Instant::now() < deadline {
            match timeout(
                deadline.saturating_duration_since(Instant::now()),
                socket.recv_from(&mut buffer)
            ).await {
                Ok(Ok((n, addr))) => {
                    if n > 0 {
                        discovered.push(ServiceInfo {
                            name: String::from_utf8_lossy(&buffer[..n]).to_string(),
                            host: addr.ip().to_string(),
                            port: addr.port(),
                        });
                    }
                }
                _ => break,
            }
        }
        
        Ok(discovered)
    }
    
    /// Try to connect with exponential backoff
    async fn connect_with_backoff(&mut self) -> TransportResult<()> {
        let max_attempts = self.base.config.max_reconnect_attempts;
        let base_delay = self.base.config.reconnect_delay_ms;
        
        while self.reconnect_attempts < max_attempts {
            match self.try_connect().await {
                Ok(()) => {
                    self.reconnect_attempts = 0;
                    return Ok(());
                }
                Err(e) => {
                    self.reconnect_attempts += 1;
                    
                    if self.reconnect_attempts >= max_attempts {
                        return Err(e);
                    }
                    
                    // Exponential backoff with jitter
                    use rand::Rng;
                    let delay = base_delay * 2u32.pow(self.reconnect_attempts - 1);
                    let jitter = rand::thread_rng().gen_range(0..delay / 4);
                    let total_delay = delay + jitter;
                    
                    tracing::warn!(
                        "UDP connection failed (attempt {}/{}), retrying in {}ms: {}",
                        self.reconnect_attempts,
                        max_attempts,
                        total_delay,
                        e
                    );
                    
                    tokio::time::sleep(Duration::from_millis(total_delay as u64)).await;
                }
            }
        }
        
        Err(TransportError::ConnectionFailed("Max reconnection attempts exceeded".into()))
    }
    
    /// Attempt a single connection
    async fn try_connect(&mut self) -> TransportResult<()> {
        // Parse remote address
        let remote_addr = format!("{}:{}", self.settings.host, self.settings.port)
            .parse::<SocketAddr>()
            .map_err(|e| TransportError::ConfigError(format!("Invalid address: {}", e)))?;
        
        // Bind to local address
        let local_addr = if self.settings.bind_port > 0 {
            format!("0.0.0.0:{}", self.settings.bind_port)
        } else {
            "0.0.0.0:0".to_string()
        };
        
        let socket = UdpSocket::bind(local_addr)
            .await
            .map_err(|e| TransportError::ConnectionFailed(format!("UDP bind failed: {}", e)))?;
        
        // Configure socket options
        if self.settings.broadcast {
            socket.set_broadcast(true)
                .map_err(|e| TransportError::ConfigError(format!("Failed to enable broadcast: {}", e)))?;
        }
        
        if self.settings.multicast {
            // TODO: Join multicast group if needed
            // This requires the specific multicast address
        }
        
        // For UDP, "connection" is just storing the remote address
        // Optionally send a handshake packet
        if self.base.config.require_handshake {
            let handshake_timeout = Duration::from_millis(self.base.config.connect_timeout_ms as u64);
            
            // Send handshake
            socket.send_to(b"CONNECT", remote_addr).await?;
            
            // Wait for acknowledgment
            let mut buffer = vec![0u8; 16];
            match timeout(handshake_timeout, socket.recv_from(&mut buffer)).await {
                Ok(Ok((n, addr))) if addr == remote_addr && &buffer[..n] == b"ACCEPT" => {
                    // Handshake successful
                }
                Ok(Ok(_)) => {
                    return Err(TransportError::ConnectionFailed("Invalid handshake response".into()));
                }
                _ => {
                    return Err(TransportError::Timeout(format!("Handshake timeout after {}ms", 
                        self.base.config.connect_timeout_ms)));
                }
            }
        }
        
        self.socket = Some(Arc::new(socket));
        self.remote_addr = Some(remote_addr);
        
        tracing::info!("Connected to UDP {}:{}", self.settings.host, self.settings.port);
        Ok(())
    }
}

#[async_trait]
impl Transport for UdpTransport {
    fn transport_type(&self) -> TransportType {
        self.base.transport_type
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn is_connected(&self) -> bool {
        self.socket.is_some() && self.remote_addr.is_some()
    }
    
    async fn connect(&self) -> TransportResult<()> {
        if self.is_connected() {
            return Err(TransportError::AlreadyConnected);
        }
        
        self.base.set_state(ConnectionState::Connecting).await;
        
        // Connect with exponential backoff
        match self.connect_with_backoff().await {
            Ok(()) => {
                self.base.set_state(ConnectionState::Connected).await;
                self.base.update_stats(|stats| {
                    stats.reconnect_count += self.reconnect_attempts;
                }).await;
                Ok(())
            }
            Err(e) => {
                self.base.set_state(ConnectionState::Error).await;
                Err(e)
            }
        }
    }
    
    async fn disconnect(&self) -> TransportResult<()> {
        if !self.is_connected() {
            return Ok(());
        }
        
        // Send disconnect packet if configured
        if self.base.config.require_handshake {
            if let (Some(socket), Some(remote_addr)) = (&self.socket, &self.remote_addr) {
                let _ = socket.send_to(b"DISCONNECT", remote_addr).await;
            }
        }
        
        // Clean up all resources
        self.cleanup_resources().await?;
        self.remote_addr = None;
        self.base.set_state(ConnectionState::Disconnected).await;
        self.reconnect_attempts = 0;
        
        tracing::info!("Disconnected from UDP {}:{}", self.settings.host, self.settings.port);
        Ok(())
    }
    
    async fn send(&self, data: &[u8]) -> TransportResult<()> {
        let start = Instant::now();
        
        // Check MTU limit
        if data.len() > self.settings.mtu {
            return Err(TransportError::ConfigError(format!(
                "Data size {} exceeds MTU {}",
                data.len(),
                self.settings.mtu
            )));
        }
        
        if let (Some(socket), Some(remote_addr)) = (&self.socket, &self.remote_addr) {
            let write_timeout = Duration::from_millis(self.base.config.write_timeout_ms as u64);
            
            let bytes_sent = timeout(write_timeout, socket.send_to(data, remote_addr))
                .await
                .map_err(|_| TransportError::Timeout(format!("Write timeout after {}ms", self.base.config.write_timeout_ms)))?
                .map_err(|e| TransportError::IoError(e))?;
            
            self.base.update_stats(|stats| {
                stats.bytes_sent += bytes_sent as u64;
                stats.transactions_success += 1;
            }).await;
            
            // Enforce minimum latency requirement (100ms for UDP)
            self.base.enforce_latency(start).await?;
            
            Ok(())
        } else {
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            
            // Try to reconnect if auto-reconnect is enabled
            if self.base.config.auto_reconnect {
                // Trigger automatic reconnection
                self.trigger_auto_reconnection().await;
                
                // For send, we'll return an error and let the next attempt succeed
                return Err(TransportError::NotConnected);
            }
            
            Err(TransportError::NotConnected)
        }
    }
    
    async fn receive(&self, timeout_duration: Duration) -> TransportResult<Vec<u8>> {
        let start = Instant::now();
        
        if let Some(socket) = &self.socket {
            let mut buffer = vec![0u8; self.base.config.read_buffer_size];
            
            let (n, addr) = timeout(timeout_duration, socket.recv_from(&mut buffer))
                .await
                .map_err(|_| TransportError::Timeout(format!("Read timeout after {:?}", timeout_duration)))?
                .map_err(|e| TransportError::IoError(e))?;
            
            // Verify sender if we have a remote address set
            if let Some(remote_addr) = &self.remote_addr {
                if addr != *remote_addr && !self.settings.accept_any_source {
                    return Err(TransportError::InvalidData(format!(
                        "Received data from unexpected source: {}",
                        addr
                    )));
                }
            }
            
            buffer.truncate(n);
            
            self.base.update_stats(|stats| {
                stats.bytes_received += n as u64;
            }).await;
            
            // Enforce minimum latency
            self.base.enforce_latency(start).await?;
            
            Ok(buffer)
        } else {
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            
            // Try to reconnect if auto-reconnect is enabled
            if self.base.config.auto_reconnect {
                self.base.set_state(ConnectionState::Reconnecting).await;
                self.connect().await?;
                return self.receive(timeout_duration).await;
            }
            
            Err(TransportError::NotConnected)
        }
    }
    
    fn stats(&self) -> TransportStats {
        TransportStats::default()
    }
    
    async fn reset(&self) -> TransportResult<()> {
        // For UDP, we can flush any pending data by reading without blocking
        if let Some(socket) = &self.socket {
            let mut discard = vec![0u8; 1024];
            while let Ok(Ok(_)) = timeout(
                Duration::from_millis(10),
                socket.recv_from(&mut discard)
            ).await {
                // Discard data
            }
        }
        
        Ok(())
    }
    
    fn config(&self) -> &TransportConfig {
        &self.base.config
    }
    
    async fn cleanup_resources(&self) -> TransportResult<()> {
        // Cancel any active reconnection attempts
        self.base.cancel_reconnection().await;
        
        // Signal shutdown to any cooperative tasks
        self.cleanup_flag.store(true, Ordering::Relaxed);
        
        // Abort all spawned tasks
        for handle in self.task_handles.drain(..) {
            handle.abort();
        }
        
        // Drop the socket
        self.socket = None;
        self.remote_addr = None;
        
        // Reset the cleanup flag for next connection
        self.cleanup_flag.store(false, Ordering::Relaxed);
        
        // Reset reconnect attempts counter
        self.reconnect_attempts = 0;
        
        // Update state
        self.base.set_state(ConnectionState::Disconnected).await;
        
        tracing::debug!("UDP transport resources cleaned up");
        Ok(())
    }
}

/// Information about discovered UDP service
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub host: String,
    pub port: u16,
}

/// UDP server for receiving datagrams
pub struct UdpServer {
    socket: Arc<UdpSocket>,
    port: u16,
}

impl UdpServer {
    /// Create a new UDP server
    pub async fn new(port: u16) -> TransportResult<Self> {
        let addr = format!("0.0.0.0:{}", port)
            .parse::<SocketAddr>()
            .map_err(|e| TransportError::ConfigError(format!("Invalid address: {}", e)))?;
        
        let socket = UdpSocket::bind(addr)
            .await
            .map_err(|e| TransportError::IoError(e))?;
        
        // Get the actual port if we used 0
        let actual_port = if port == 0 {
            socket.local_addr()
                .map_err(|e| TransportError::IoError(e))?
                .port()
        } else {
            port
        };
        
        Ok(UdpServer {
            socket: Arc::new(socket),
            port: actual_port,
        })
    }
    
    /// Receive datagram and wrap sender in UdpTransport
    pub async fn receive_from(&self) -> TransportResult<(Vec<u8>, UdpTransport)> {
        let mut buffer = vec![0u8; 65507]; // Max UDP packet size
        
        let (n, peer_addr) = self.socket.recv_from(&mut buffer)
            .await
            .map_err(|e| TransportError::IoError(e))?;
        
        buffer.truncate(n);
        
        let config = TransportConfig {
            transport_type: TransportType::Udp,
            address: peer_addr.to_string(),
            settings: crate::transport::common::TransportSettings::Udp(UdpSettings {
                host: peer_addr.ip().to_string(),
                port: peer_addr.port(),
                bind_port: 0, // Use any available port
                broadcast: false,
                multicast: false,
                mtu: 1472, // Standard MTU
                accept_any_source: false,
            }),
            ..Default::default()
        };
        
        let mut transport = UdpTransport::new(config)?;
        transport.socket = Some(self.socket.clone());
        transport.remote_addr = Some(peer_addr);
        transport.base.set_state(ConnectionState::Connected).await;
        
        Ok((buffer, transport))
    }
    
    /// Enable broadcast on server socket
    pub fn set_broadcast(&self, broadcast: bool) -> TransportResult<()> {
        self.socket.set_broadcast(broadcast)
            .map_err(|e| TransportError::ConfigError(format!("Failed to set broadcast: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::common::TransportSettings;
    
    #[tokio::test]
    async fn test_udp_transport_creation() {
        let config = TransportConfig {
            transport_type: TransportType::Udp,
            address: "127.0.0.1:9090".to_string(),
            settings: TransportSettings::Udp(UdpSettings {
                host: "127.0.0.1".to_string(),
                port: 9090,
                bind_port: 0,
                broadcast: false,
                multicast: false,
                mtu: 1472,
                accept_any_source: true,
            }),
            ..Default::default()
        };
        
        let transport = UdpTransport::new(config);
        assert!(transport.is_ok());
    }
    
    #[tokio::test]
    async fn test_udp_server_client() {
        // Start server
        let server = UdpServer::new(0).await.unwrap(); // Use port 0 for random port
        let port = server.port;
        
        // Create client
        let config = TransportConfig {
            transport_type: TransportType::Udp,
            address: format!("127.0.0.1:{}", port),
            settings: TransportSettings::Udp(UdpSettings {
                host: "127.0.0.1".to_string(),
                port,
                bind_port: 0,
                broadcast: false,
                multicast: false,
                mtu: 1472,
                accept_any_source: true,
            }),
            auto_reconnect: false,
            require_handshake: false, // Skip handshake for test
            ..Default::default()
        };
        
        let mut client = UdpTransport::new(config).unwrap();
        
        // Connect should succeed
        client.connect().await.unwrap();
        assert!(client.is_connected());
        
        // Send data
        client.send(b"Hello UDP").await.unwrap();
        
        // Server receives data
        let (data, mut server_transport) = server.receive_from().await.unwrap();
        assert_eq!(data, b"Hello UDP");
        
        // Server sends response
        server_transport.send(b"UDP Response").await.unwrap();
        
        // Client receives response
        let response = client.receive(Duration::from_secs(1)).await.unwrap();
        assert_eq!(response, b"UDP Response");
        
        // Cleanup
        client.disconnect().await.unwrap();
        server_transport.disconnect().await.unwrap();
    }
}