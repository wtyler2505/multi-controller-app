use crate::{Result, async_trait};
use std::time::Duration;

/// Represents different transport types supported by the system
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransportType {
    Serial,
    Tcp,
    Udp,
    Ssh,
}

/// Configuration for transport connections
#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub connection_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    pub reconnect_attempts: u32,
    pub reconnect_delay: Duration,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(5),
            read_timeout: Duration::from_millis(100),
            write_timeout: Duration::from_millis(50),
            reconnect_attempts: 3,
            reconnect_delay: Duration::from_millis(1000),
        }
    }
}

/// The Transport trait defines the interface for all communication transports
#[async_trait]
pub trait Transport: Send + Sync {
    /// Get the transport type
    fn transport_type(&self) -> TransportType;
    
    /// Get a human-readable name/identifier for this transport
    fn name(&self) -> &str;
    
    /// Check if the transport is currently connected
    fn is_connected(&self) -> bool;
    
    /// Connect to the transport endpoint
    async fn connect(&mut self) -> Result<()>;
    
    /// Disconnect from the transport endpoint
    async fn disconnect(&mut self) -> Result<()>;
    
    /// Send data through the transport
    async fn send(&mut self, data: &[u8]) -> Result<usize>;
    
    /// Receive data from the transport with timeout
    async fn receive(&mut self, buffer: &mut [u8]) -> Result<usize>;
    
    /// Flush any pending writes
    async fn flush(&mut self) -> Result<()>;
    
    /// Get transport-specific information
    fn info(&self) -> TransportInfo;
}

/// Information about a transport instance
#[derive(Debug, Clone)]
pub struct TransportInfo {
    pub transport_type: TransportType,
    pub endpoint: String,
    pub connected: bool,
    pub last_activity: Option<std::time::Instant>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}