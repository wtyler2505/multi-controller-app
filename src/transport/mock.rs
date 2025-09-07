/// Mock transport implementation for testing
/// Provides configurable failure injection and deterministic behavior
use async_trait::async_trait;
use std::sync::{Arc, atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering}};
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, mpsc};
use crate::transport::{
    Transport, TransportConfig, TransportError, TransportResult, 
    TransportStats, TransportType
};

/// Configuration for mock transport behavior
#[derive(Debug, Clone)]
pub struct MockConfig {
    /// Number of connect attempts before succeeding
    pub connect_failures: u32,
    /// Number of send attempts before succeeding
    pub send_failures: u32,
    /// Number of receive attempts before succeeding  
    pub receive_failures: u32,
    /// Simulated latency for operations
    pub latency_ms: u64,
    /// Whether to simulate disconnection after N operations
    pub disconnect_after_ops: Option<u32>,
    /// Data to return on receive (if None, echo sent data)
    pub receive_data: Option<Vec<u8>>,
    /// Whether to enforce minimum latency
    pub enforce_latency: bool,
}

impl Default for MockConfig {
    fn default() -> Self {
        MockConfig {
            connect_failures: 0,
            send_failures: 0,
            receive_failures: 0,
            latency_ms: 10,
            disconnect_after_ops: None,
            receive_data: None,
            enforce_latency: true,
        }
    }
}

/// Mock transport for testing
pub struct MockTransport {
    name: String,
    config: TransportConfig,
    mock_config: Arc<RwLock<MockConfig>>,
    connected: Arc<AtomicBool>,
    stats: Arc<RwLock<TransportStats>>,
    
    // Failure injection counters
    connect_attempts: Arc<AtomicU32>,
    send_attempts: Arc<AtomicU32>,
    receive_attempts: Arc<AtomicU32>,
    total_operations: Arc<AtomicU32>,
    
    // Data handling
    send_buffer: Arc<RwLock<Vec<u8>>>,
    receive_channel: Option<mpsc::UnboundedReceiver<Vec<u8>>>,
    receive_sender: mpsc::UnboundedSender<Vec<u8>>,
    
    // Timing
    last_operation: Arc<RwLock<Option<Instant>>>,
}

impl MockTransport {
    pub fn new(name: String, config: TransportConfig, mock_config: MockConfig) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        
        MockTransport {
            name,
            config,
            mock_config: Arc::new(RwLock::new(mock_config)),
            connected: Arc::new(AtomicBool::new(false)),
            stats: Arc::new(RwLock::new(TransportStats::default())),
            connect_attempts: Arc::new(AtomicU32::new(0)),
            send_attempts: Arc::new(AtomicU32::new(0)),
            receive_attempts: Arc::new(AtomicU32::new(0)),
            total_operations: Arc::new(AtomicU32::new(0)),
            send_buffer: Arc::new(RwLock::new(Vec::new())),
            receive_channel: Some(rx),
            receive_sender: tx,
            last_operation: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Update mock configuration during test
    pub async fn set_mock_config(&self, config: MockConfig) {
        *self.mock_config.write().await = config;
    }
    
    /// Inject data to be received
    pub async fn inject_receive_data(&self, data: Vec<u8>) -> TransportResult<()> {
        self.receive_sender.send(data)
            .map_err(|_| TransportError::IoError(
                std::io::Error::new(std::io::ErrorKind::BrokenPipe, "Mock channel closed")
            ))
    }
    
    /// Get the last sent data
    pub async fn get_sent_data(&self) -> Vec<u8> {
        self.send_buffer.read().await.clone()
    }
    
    /// Reset all counters
    pub fn reset_counters(&self) {
        self.connect_attempts.store(0, Ordering::Relaxed);
        self.send_attempts.store(0, Ordering::Relaxed);
        self.receive_attempts.store(0, Ordering::Relaxed);
        self.total_operations.store(0, Ordering::Relaxed);
    }
    
    async fn enforce_latency(&self) {
        let mock_cfg = self.mock_config.read().await;
        if mock_cfg.enforce_latency {
            let min_latency = Duration::from_millis(mock_cfg.latency_ms);
            
            let mut last_op = self.last_operation.write().await;
            if let Some(last) = *last_op {
                let elapsed = last.elapsed();
                if elapsed < min_latency {
                    tokio::time::sleep(min_latency - elapsed).await;
                }
            }
            *last_op = Some(Instant::now());
        }
    }
    
    async fn check_disconnect(&self) -> TransportResult<()> {
        let mock_cfg = self.mock_config.read().await;
        if let Some(max_ops) = mock_cfg.disconnect_after_ops {
            let ops = self.total_operations.load(Ordering::Relaxed);
            if ops >= max_ops {
                self.connected.store(false, Ordering::Relaxed);
                return Err(TransportError::ConnectionFailed(
                    "Simulated disconnection after operation limit".into()
                ));
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Transport for MockTransport {
    fn transport_type(&self) -> TransportType {
        TransportType::Serial // Default to Serial for testing
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }
    
    async fn connect(&mut self) -> TransportResult<()> {
        let attempts = self.connect_attempts.fetch_add(1, Ordering::Relaxed);
        let mock_cfg = self.mock_config.read().await;
        
        if attempts < mock_cfg.connect_failures {
            return Err(TransportError::ConnectionFailed(
                format!("Mock connect failure {}/{}", attempts + 1, mock_cfg.connect_failures)
            ));
        }
        
        self.connected.store(true, Ordering::Relaxed);
        self.stats.write().await.reconnect_count += 1;
        Ok(())
    }
    
    async fn disconnect(&mut self) -> TransportResult<()> {
        self.connected.store(false, Ordering::Relaxed);
        self.reset_counters();
        Ok(())
    }
    
    async fn send(&mut self, data: &[u8]) -> TransportResult<()> {
        if !self.is_connected() {
            return Err(TransportError::NotConnected);
        }
        
        self.enforce_latency().await;
        self.check_disconnect().await?;
        
        let attempts = self.send_attempts.fetch_add(1, Ordering::Relaxed);
        let mock_cfg = self.mock_config.read().await;
        
        if attempts < mock_cfg.send_failures {
            self.stats.write().await.transactions_failed += 1;
            return Err(TransportError::IoError(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Mock send failure {}/{}", attempts + 1, mock_cfg.send_failures)
                )
            ));
        }
        
        // Store sent data
        *self.send_buffer.write().await = data.to_vec();
        
        // Update stats
        let mut stats = self.stats.write().await;
        stats.bytes_sent += data.len() as u64;
        stats.transactions_success += 1;
        
        self.total_operations.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
    
    async fn receive(&mut self, timeout: Duration) -> TransportResult<Vec<u8>> {
        if !self.is_connected() {
            return Err(TransportError::NotConnected);
        }
        
        self.enforce_latency().await;
        self.check_disconnect().await?;
        
        let attempts = self.receive_attempts.fetch_add(1, Ordering::Relaxed);
        let mock_cfg = self.mock_config.read().await;
        
        if attempts < mock_cfg.receive_failures {
            self.stats.write().await.transactions_failed += 1;
            return Err(TransportError::Timeout("Mock receive timeout".into()));
        }
        
        // Return configured data or echo sent data
        let data = if let Some(ref configured_data) = mock_cfg.receive_data {
            configured_data.clone()
        } else if let Some(ref mut rx) = self.receive_channel {
            // Try to receive injected data
            match tokio::time::timeout(timeout, rx.recv()).await {
                Ok(Some(data)) => data,
                Ok(None) => return Err(TransportError::IoError(
                    std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Channel closed")
                )),
                Err(_) => {
                    // Echo sent data as fallback
                    self.send_buffer.read().await.clone()
                }
            }
        } else {
            self.send_buffer.read().await.clone()
        };
        
        // Update stats
        let mut stats = self.stats.write().await;
        stats.bytes_received += data.len() as u64;
        stats.transactions_success += 1;
        
        self.total_operations.fetch_add(1, Ordering::Relaxed);
        Ok(data)
    }
    
    fn stats(&self) -> TransportStats {
        // Use try_read to avoid blocking in async context
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| TransportStats::default())
    }
    
    async fn reset(&mut self) -> TransportResult<()> {
        self.reset_counters();
        *self.send_buffer.write().await = Vec::new();
        *self.stats.write().await = TransportStats::default();
        Ok(())
    }
    
    fn config(&self) -> &TransportConfig {
        &self.config
    }
    
    async fn cleanup_resources(&mut self) -> TransportResult<()> {
        self.connected.store(false, Ordering::Relaxed);
        self.reset_counters();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mock_transport_basic() {
        let config = TransportConfig::default();
        let mock_config = MockConfig::default();
        let mut transport = MockTransport::new("test".into(), config, mock_config);
        
        // Test connection
        assert!(!transport.is_connected());
        transport.connect().await.unwrap();
        assert!(transport.is_connected());
        
        // Test send/receive echo
        let data = b"Hello World";
        transport.send(data).await.unwrap();
        let received = transport.receive(Duration::from_secs(1)).await.unwrap();
        assert_eq!(received, data);
        
        // Test disconnect
        transport.disconnect().await.unwrap();
        assert!(!transport.is_connected());
    }
    
    #[tokio::test]
    async fn test_mock_transport_failures() {
        let config = TransportConfig::default();
        let mock_config = MockConfig {
            connect_failures: 2,
            send_failures: 1,
            receive_failures: 1,
            ..Default::default()
        };
        let mut transport = MockTransport::new("test".into(), config, mock_config);
        
        // Test connect failures
        assert!(transport.connect().await.is_err());
        assert!(transport.connect().await.is_err());
        assert!(transport.connect().await.is_ok()); // Third attempt succeeds
        
        // Test send failure then success
        assert!(transport.send(b"test").await.is_err());
        assert!(transport.send(b"test").await.is_ok());
        
        // Test receive failure then success  
        assert!(transport.receive(Duration::from_millis(100)).await.is_err());
        assert!(transport.receive(Duration::from_millis(100)).await.is_ok());
    }
    
    #[tokio::test]
    async fn test_mock_transport_disconnect_after_ops() {
        let config = TransportConfig::default();
        let mock_config = MockConfig {
            disconnect_after_ops: Some(3),
            ..Default::default()
        };
        let mut transport = MockTransport::new("test".into(), config, mock_config);
        
        transport.connect().await.unwrap();
        
        // First 3 operations succeed
        transport.send(b"1").await.unwrap();
        transport.send(b"2").await.unwrap();
        transport.send(b"3").await.unwrap();
        
        // Fourth operation triggers disconnect
        assert!(transport.send(b"4").await.is_err());
        assert!(!transport.is_connected());
    }
}