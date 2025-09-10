use async_trait::async_trait;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use std::net::SocketAddr;
use tokio::net::{TcpStream, TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio::time::timeout;
use tokio::task::JoinHandle;

use crate::transport::{
    Transport, TransportBase, TransportConfig, TransportError, TransportResult,
    TransportStats, TransportType, ConnectionState,
};
use crate::transport::common::TcpSettings;

/// TCP transport implementation
pub struct TcpTransport {
    base: TransportBase,
    stream: Option<Mutex<TcpStream>>,
    settings: TcpSettings,
    reconnect_attempts: u32,
    task_handles: Vec<JoinHandle<()>>,  // Track spawned tasks for cleanup
    cleanup_flag: Arc<AtomicBool>,      // Signal for cooperative shutdown
}

impl TcpTransport {
    /// Create a new TCP transport
    pub fn new(config: TransportConfig) -> TransportResult<Self> {
        let settings = match config.settings {
            crate::transport::common::TransportSettings::Tcp(ref tcp) => tcp.clone(),
            _ => return Err(TransportError::ConfigError("Invalid settings for TCP transport".into())),
        };
        
        Ok(TcpTransport {
            base: TransportBase::new(
                format!("TCP:{}:{}", settings.host, settings.port),
                TransportType::Tcp,
                config,
            ),
            stream: None,
            settings,
            reconnect_attempts: 0,
            task_handles: Vec::new(),
            cleanup_flag: Arc::new(AtomicBool::new(false)),
        })
    }
    
    /// Scan for available TCP services using mDNS
    pub async fn discover_mdns(service_type: &str, timeout_ms: u64) -> TransportResult<Vec<ServiceInfo>> {
        // TODO: Implement mDNS discovery
        // For now, return empty list
        Ok(vec![])
    }
    
    /// Try to connect with exponential backoff using shared module
    async fn connect_with_backoff(&mut self) -> TransportResult<()> {
        let mut backoff = crate::transport::backoff::ExponentialBackoff::from_config(
            self.base.config.max_reconnect_attempts,
            self.base.config.reconnect_delay_ms,
        );
        
        while backoff.should_retry() {
            match self.try_connect().await {
                Ok(()) => {
                    self.reconnect_attempts = 0;
                    return Ok(());
                }
                Err(e) => {
                    // Check if error is retryable
                    if !crate::transport::backoff::is_retryable_error(&e) {
                        tracing::error!("Non-retryable TCP error: {}", e);
                        return Err(e);
                    }
                    
                    if let Some(delay) = backoff.next_delay() {
                        tracing::warn!(
                            "TCP connection failed (attempt {}/{}), retrying in {:?}: {}",
                            backoff.current_attempt(),
                            backoff.remaining_attempts().unwrap_or(999),
                            delay,
                            e
                        );
                        
                        tokio::time::sleep(delay).await;
                    } else {
                        return Err(TransportError::ConnectionFailed(
                            format!("Max reconnection attempts ({}) exceeded", backoff.current_attempt())
                        ));
                    }
                }
            }
        }
        
        Err(TransportError::ConnectionFailed("Max reconnection attempts exceeded".into()))
    }
    
    /// Attempt a single connection
    async fn try_connect(&mut self) -> TransportResult<()> {
        let addr = format!("{}:{}", self.settings.host, self.settings.port)
            .parse::<SocketAddr>()
            .map_err(|e| TransportError::ConfigError(format!("Invalid address: {}", e)))?;
        
        let connect_timeout = Duration::from_millis(self.base.config.connect_timeout_ms as u64);
        
        let stream = timeout(connect_timeout, TcpStream::connect(addr))
            .await
            .map_err(|_| TransportError::Timeout(format!("Connection timeout after {}ms", self.base.config.connect_timeout_ms)))?
            .map_err(|e| TransportError::ConnectionFailed(format!("TCP connect failed: {}", e)))?;
        
        // Configure socket options
        stream.set_nodelay(self.settings.no_delay)
            .map_err(|e| TransportError::ConfigError(format!("Failed to set TCP_NODELAY: {}", e)))?;
        
        // TODO: Configure keep-alive if supported
        
        self.stream = Some(Mutex::new(stream));
        
        tracing::info!("Connected to TCP {}:{}", self.settings.host, self.settings.port);
        Ok(())
    }
}

impl TcpTransport {
    async fn handle_not_connected(&mut self, timeout_duration: Duration) -> TransportResult<Vec<u8>> {
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

#[async_trait]
impl Transport for TcpTransport {
    fn transport_type(&self) -> TransportType {
        self.base.transport_type
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn is_connected(&self) -> bool {
        self.stream.is_some()
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
        
        // Clean up all resources before disconnecting
        self.cleanup_resources().await?;
        self.reconnect_attempts = 0;
        
        tracing::info!("Disconnected from TCP {}:{}", self.settings.host, self.settings.port);
        Ok(())
    }
    
    async fn send(&self, data: &[u8]) -> TransportResult<()> {
        let start = Instant::now();
        
        // Check connection and reconnect if needed (before creating guard)
        if self.stream.is_none() && self.base.config.auto_reconnect {
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            
            self.base.set_state(ConnectionState::Reconnecting).await;
            self.connect().await?;
            return self.send(data).await;
        }
        
        if let Some(ref stream) = self.stream {
            // Start monitoring this operation
            let guard = self.base.monitor.start_operation("tcp_send");
            let mut stream = stream.lock().await;
            
            let write_timeout = Duration::from_millis(self.base.config.write_timeout_ms as u64);
            
            timeout(write_timeout, stream.write_all(data))
                .await
                .map_err(|_| TransportError::Timeout(format!("Write timeout after {}ms", self.base.config.write_timeout_ms)))?
                .map_err(|e| TransportError::IoError(e))?;
            
            stream.flush().await?;
            
            self.base.update_stats(|stats| {
                stats.bytes_sent += data.len() as u64;
                stats.transactions_success += 1;
            }).await;
            
            // Enforce minimum latency requirement (100ms for TCP)
            self.base.enforce_latency(start).await?;
            
            // Complete the monitoring guard
            guard.complete().await;
            
            Ok(())
        } else {
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            
            Err(TransportError::NotConnected)
        }
    }
    
    async fn receive(&self, timeout_duration: Duration) -> TransportResult<Vec<u8>> {
        let start = Instant::now();
        
        // Handle the case where stream exists
        let result = if let Some(ref stream) = self.stream {
            let mut stream = stream.lock().await;
            
            let mut buffer = vec![0u8; self.base.config.read_buffer_size];
            
            let n = timeout(timeout_duration, stream.read(&mut buffer))
                .await
                .map_err(|_| TransportError::Timeout(format!("Read timeout after {:?}", timeout_duration)))?
                .map_err(|e| TransportError::IoError(e))?;
            
            if n == 0 {
                // Connection closed by peer - will handle after releasing lock
                None
            } else {
                buffer.truncate(n);
                
                self.base.update_stats(|stats| {
                    stats.bytes_received += n as u64;
                }).await;
                
                // Enforce minimum latency
                self.base.enforce_latency(start).await?;
                
                Some(buffer)
            }
        } else {
            return self.handle_not_connected(timeout_duration).await;
        };
        
        // Handle connection closed case
        if result.is_none() {
            self.stream = None;
            self.base.set_state(ConnectionState::Disconnected).await;
            return Err(TransportError::ConnectionFailed("Connection closed by peer".into()));
        }
        
        Ok(result.unwrap())
    }
    
    fn stats(&self) -> TransportStats {
        TransportStats::default()
    }
    
    async fn reset(&self) -> TransportResult<()> {
        // TCP doesn't have a buffer to flush like serial
        // But we can try to clear any pending data
        if let Some(ref stream) = self.stream {
            let mut stream = stream.lock().await;
            
            // Try to read and discard any pending data
            let mut discard = vec![0u8; 1024];
            while let Ok(Ok(n)) = timeout(
                Duration::from_millis(10),
                stream.read(&mut discard)
            ).await {
                if n == 0 {
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    fn config(&self) -> &TransportConfig {
        &self.base.config
    }
    
    async fn cleanup_resources(&self) -> TransportResult<()> {
        // Signal shutdown to any cooperative tasks
        self.cleanup_flag.store(true, Ordering::Relaxed);
        
        // Abort all spawned tasks
        for handle in self.task_handles.drain(..) {
            handle.abort();
        }
        
        // Properly shutdown and drop the TCP stream
        if let Some(stream) = self.stream.take() {
            let mut stream = stream.lock().await;
            let _ = stream.shutdown().await; // Ignore errors during cleanup
        }
        
        // Reset the cleanup flag for next connection
        self.cleanup_flag.store(false, Ordering::Relaxed);
        
        // Update state
        self.base.set_state(ConnectionState::Disconnected).await;
        
        tracing::debug!("TCP transport resources cleaned up");
        Ok(())
    }
}

/// Information about discovered TCP service
#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub txt_records: Vec<(String, String)>,
}

/// TCP server for accepting incoming connections
pub struct TcpServer {
    listener: TcpListener,
    port: u16,
}

impl TcpServer {
    /// Create a new TCP server
    pub async fn new(port: u16) -> TransportResult<Self> {
        let addr = format!("0.0.0.0:{}", port)
            .parse::<SocketAddr>()
            .map_err(|e| TransportError::ConfigError(format!("Invalid address: {}", e)))?;
        
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| TransportError::IoError(e))?;
        
        // Get the actual port if we used 0
        let actual_port = if port == 0 {
            listener.local_addr()
                .map_err(|e| TransportError::IoError(e))?
                .port()
        } else {
            port
        };
        
        Ok(TcpServer {
            listener,
            port: actual_port,
        })
    }
    
    /// Accept incoming connection and wrap in TcpTransport
    pub async fn accept(&self) -> TransportResult<TcpTransport> {
        let (stream, peer_addr) = self.listener.accept()
            .await
            .map_err(|e| TransportError::IoError(e))?;
        
        let config = TransportConfig {
            transport_type: TransportType::Tcp,
            address: peer_addr.to_string(),
            settings: crate::transport::common::TransportSettings::Tcp(TcpSettings {
                host: peer_addr.ip().to_string(),
                port: peer_addr.port(),
                no_delay: true,
                keep_alive: true,
                keep_alive_interval_ms: 10000,
            }),
            ..Default::default()
        };
        
        let mut transport = TcpTransport::new(config)?;
        transport.stream = Some(Mutex::new(stream));
        transport.base.set_state(ConnectionState::Connected).await;
        
        Ok(transport)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::common::TransportSettings;
    
    #[tokio::test]
    async fn test_tcp_transport_creation() {
        let config = TransportConfig {
            transport_type: TransportType::Tcp,
            address: "127.0.0.1:8080".to_string(),
            settings: TransportSettings::Tcp(TcpSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
                no_delay: true,
                keep_alive: true,
                keep_alive_interval_ms: 10000,
            }),
            ..Default::default()
        };
        
        let transport = TcpTransport::new(config);
        assert!(transport.is_ok());
    }
    
    #[tokio::test]
    async fn test_tcp_server_client() {
        // Start server
        let server = TcpServer::new(0).await.unwrap(); // Use port 0 for random port
        let port = server.port;
        
        // Spawn server accept task
        let server_handle = tokio::spawn(async move {
            server.accept().await
        });
        
        // Create client
        let config = TransportConfig {
            transport_type: TransportType::Tcp,
            address: format!("127.0.0.1:{}", port),
            settings: TransportSettings::Tcp(TcpSettings {
                host: "127.0.0.1".to_string(),
                port,
                no_delay: true,
                keep_alive: false,
                keep_alive_interval_ms: 0,
            }),
            auto_reconnect: false,
            ..Default::default()
        };
        
        let mut client = TcpTransport::new(config).unwrap();
        
        // Connect should succeed
        client.connect().await.unwrap();
        assert!(client.is_connected());
        
        // Server should accept connection
        let mut server_transport = server_handle.await.unwrap().unwrap();
        assert!(server_transport.is_connected());
        
        // Test data exchange
        client.send(b"Hello TCP").await.unwrap();
        let data = server_transport.receive(Duration::from_secs(1)).await.unwrap();
        assert_eq!(data, b"Hello TCP");
        
        // Cleanup
        client.disconnect().await.unwrap();
        server_transport.disconnect().await.unwrap();
    }
}