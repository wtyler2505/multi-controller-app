use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, Mutex};
use tokio::task::JoinHandle;

pub mod serial;
pub mod tcp;
pub mod udp;
pub mod ssh;
pub mod ssh_keys;
pub mod common;
pub mod manifest;
pub mod monitor;
pub mod backoff;

#[cfg(test)]
pub mod mock;

#[cfg(test)]
mod tests;

// Re-export common types
pub use common::{TransportType, TransportError, TransportResult, TransportConfig};
pub use monitor::LatencyMonitor;

/// Core transport trait for device communication
/// Implements connection management, data transfer, and latency enforcement
/// 
/// Uses interior mutability pattern with &self methods to enable true sharing
/// via Arc<dyn Transport> across multiple drivers and components.
#[async_trait]
pub trait Transport: Send + Sync {
    /// Get transport type
    fn transport_type(&self) -> TransportType;
    
    /// Get transport name/identifier
    fn name(&self) -> &str;
    
    /// Check if currently connected
    fn is_connected(&self) -> bool;
    
    /// Connect to the transport
    async fn connect(&self) -> TransportResult<()>;
    
    /// Disconnect from the transport
    async fn disconnect(&self) -> TransportResult<()>;
    
    /// Send data with latency enforcement
    async fn send(&self, data: &[u8]) -> TransportResult<()>;
    
    /// Receive data with timeout
    async fn receive(&self, timeout: Duration) -> TransportResult<Vec<u8>>;
    
    /// Send and receive in one operation (common pattern)
    async fn transact(&self, data: &[u8], timeout: Duration) -> TransportResult<Vec<u8>> {
        self.send(data).await?;
        self.receive(timeout).await
    }
    
    /// Get transport statistics
    fn stats(&self) -> TransportStats;
    
    /// Reset transport (clear buffers, reset state)
    async fn reset(&self) -> TransportResult<()>;
    
    /// Get configuration
    fn config(&self) -> &TransportConfig;
    
    /// Clean up all resources (tasks, channels, etc.) on disconnect
    /// This method should abort all spawned tasks, drop Arc references,
    /// and ensure no memory leaks occur during reconnect cycles
    async fn cleanup_resources(&self) -> TransportResult<()>;
}

/// Transport statistics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransportStats {
    /// Total bytes sent
    pub bytes_sent: u64,
    
    /// Total bytes received
    pub bytes_received: u64,
    
    /// Number of successful transactions
    pub transactions_success: u64,
    
    /// Number of failed transactions
    pub transactions_failed: u64,
    
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    
    /// Maximum latency observed
    pub max_latency_ms: f64,
    
    /// Number of reconnections
    pub reconnect_count: u32,
    
    /// Last error message if any
    pub last_error: Option<String>,
    
    /// Connection uptime in seconds
    pub uptime_seconds: u64,
    
    /// Number of times latency enforcement was triggered
    pub latency_enforcements: u64,
    
    /// Last enforcement delay in milliseconds
    pub last_enforcement_ms: Option<f64>,
    
    /// Total cumulative enforcement delay in milliseconds
    pub total_enforcement_delay_ms: f64,
}

/// Transport connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Error,
}

/// Transport capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportCapabilities {
    /// Supports bidirectional communication
    pub bidirectional: bool,
    
    /// Supports broadcast/multicast
    pub broadcast: bool,
    
    /// Maximum packet size
    pub max_packet_size: usize,
    
    /// Supports flow control
    pub flow_control: bool,
    
    /// Supports out-of-band data
    pub out_of_band: bool,
    
    /// Minimum latency achievable (ms)
    pub min_latency_ms: u32,
    
    /// Supports encryption
    pub encryption: bool,
}

/// Base transport implementation with common functionality
/// Uses interior mutability pattern to support sharing via Arc<dyn Transport>
pub struct TransportBase {
    pub name: Arc<String>,
    pub transport_type: TransportType,
    pub state: Arc<RwLock<ConnectionState>>,
    pub stats: Arc<RwLock<TransportStats>>,
    pub config: Arc<TransportConfig>,
    pub capabilities: TransportCapabilities,
    pub monitor: Arc<LatencyMonitor>,
    pub reconnection_task: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl TransportBase {
    pub fn new(name: String, transport_type: TransportType, config: TransportConfig) -> Self {
        let capabilities = match transport_type {
            TransportType::Serial => TransportCapabilities {
                bidirectional: true,
                broadcast: false,
                max_packet_size: 4096,
                flow_control: true,
                out_of_band: false,
                min_latency_ms: 50, // Performance requirement
                encryption: false,
            },
            TransportType::Tcp => TransportCapabilities {
                bidirectional: true,
                broadcast: false,
                max_packet_size: 65536,
                flow_control: true,
                out_of_band: true,
                min_latency_ms: 100, // Performance requirement
                encryption: true,
            },
            TransportType::Udp => TransportCapabilities {
                bidirectional: true,
                broadcast: true,
                max_packet_size: 65507,
                flow_control: false,
                out_of_band: false,
                min_latency_ms: 100,
                encryption: false,
            },
            TransportType::Ssh => TransportCapabilities {
                bidirectional: true,
                broadcast: false,
                max_packet_size: 32768,
                flow_control: true,
                out_of_band: false,
                min_latency_ms: 150,
                encryption: true,
            },
        };
        
        let monitor = Arc::new(LatencyMonitor::new(
            capabilities.min_latency_ms as f64
        ));
        
        TransportBase {
            name: Arc::new(name),
            transport_type,
            state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            stats: Arc::new(RwLock::new(TransportStats::default())),
            config: Arc::new(config),
            capabilities,
            monitor,
            reconnection_task: Arc::new(Mutex::new(None)),
        }
    }
    
    pub async fn update_stats<F>(&self, update_fn: F)
    where
        F: FnOnce(&mut TransportStats),
    {
        let mut stats = self.stats.write().await;
        update_fn(&mut stats);
    }
    
    pub async fn set_state(&self, new_state: ConnectionState) {
        let mut state = self.state.write().await;
        *state = new_state;
    }
    
    pub async fn get_state(&self) -> ConnectionState {
        let state = self.state.read().await;
        *state
    }
    
    /// Enforce latency requirements
    pub async fn enforce_latency(&self, start: std::time::Instant) -> TransportResult<()> {
        let elapsed = start.elapsed();
        let min_latency = Duration::from_millis(self.capabilities.min_latency_ms as u64);
        
        if elapsed < min_latency {
            let delay = min_latency - elapsed;
            let delay_ms = delay.as_millis();
            
            // Emit warning for visibility
            tracing::warn!(
                transport = %self.name,
                transport_type = ?self.transport_type,
                measured_ms = elapsed.as_millis(),
                required_ms = min_latency.as_millis(),
                delay_ms = delay_ms,
                "Latency budget enforcement: delaying {}ms to meet {}ms minimum",
                delay_ms, self.capabilities.min_latency_ms
            );
            
            // Update enforcement stats
            self.update_stats(|stats| {
                stats.latency_enforcements += 1;
                stats.last_enforcement_ms = Some(delay_ms as f64);
                stats.total_enforcement_delay_ms += delay_ms as f64;
            }).await;
            
            tokio::time::sleep(delay).await;
        }
        
        let elapsed_ms = elapsed.as_millis() as f64;
        self.update_stats(|stats| {
            stats.avg_latency_ms = (stats.avg_latency_ms * stats.transactions_success as f64
                + elapsed_ms) / (stats.transactions_success + 1) as f64;
            stats.max_latency_ms = stats.max_latency_ms.max(elapsed_ms);
        }).await;
        
        Ok(())
    }
    
    /// Trigger automatic reconnection with exponential backoff
    pub async fn trigger_reconnection<F>(&self, connect_fn: F) -> TransportResult<()>
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = TransportResult<()>> + Send>> + Send + 'static,
    {
        let mut task_guard = self.reconnection_task.lock().await;
        
        // Cancel existing reconnection task if any
        if let Some(handle) = task_guard.take() {
            handle.abort();
        }
        
        // Create exponential backoff from config
        let backoff = backoff::ExponentialBackoff::from_config(
            self.config.max_reconnect_attempts,
            self.config.reconnect_delay_ms,
        );
        
        let state_clone = self.state.clone();
        let stats_clone = self.stats.clone();
        let name = self.name.clone();
        
        // Spawn new reconnection task
        let handle = tokio::spawn(async move {
            let mut backoff = backoff;
            
            loop {
                // Check if we should continue trying
                if !backoff.should_retry() {
                    tracing::error!("Max reconnection attempts exceeded for {}", name);
                    let mut state = state_clone.write().await;
                    *state = ConnectionState::Error;
                    break;
                }
                
                // Get next delay
                if let Some(delay) = backoff.next_delay() {
                    tracing::info!(
                        "Reconnection attempt {}/{} for {} in {:?}",
                        backoff.current_attempt(),
                        backoff.remaining_attempts().unwrap_or(999),
                        name,
                        delay
                    );
                    
                    // Wait with backoff delay
                    tokio::time::sleep(delay).await;
                    
                    // Update state to Reconnecting
                    {
                        let mut state = state_clone.write().await;
                        *state = ConnectionState::Reconnecting;
                    }
                    
                    // Attempt reconnection
                    match connect_fn().await {
                        Ok(()) => {
                            tracing::info!("Successfully reconnected {}", name);
                            
                            // Update state to Connected
                            let mut state = state_clone.write().await;
                            *state = ConnectionState::Connected;
                            
                            // Update stats
                            let mut stats = stats_clone.write().await;
                            stats.reconnect_count += 1;
                            
                            break; // Success, exit loop
                        }
                        Err(e) => {
                            // Check if error is retryable
                            if !backoff::is_retryable_error(&e) {
                                tracing::error!("Non-retryable error for {}: {}", name, e);
                                let mut state = state_clone.write().await;
                                *state = ConnectionState::Error;
                                break;
                            }
                            
                            tracing::warn!(
                                "Reconnection attempt {} failed for {}: {}",
                                backoff.current_attempt(),
                                name,
                                e
                            );
                            
                            // Update stats
                            let mut stats = stats_clone.write().await;
                            stats.transactions_failed += 1;
                            stats.last_error = Some(e.to_string());
                        }
                    }
                }
            }
        });
        
        *task_guard = Some(handle);
        Ok(())
    }
    
    /// Cancel any active reconnection attempts
    pub async fn cancel_reconnection(&self) {
        let mut task_guard = self.reconnection_task.lock().await;
        if let Some(handle) = task_guard.take() {
            handle.abort();
            tracing::debug!("Cancelled reconnection task for {}", self.name);
        }
    }
}

/// Factory for creating transports
pub struct TransportFactory;

impl TransportFactory {
    /// Create a transport from configuration
    pub async fn create(config: TransportConfig) -> TransportResult<Box<dyn Transport>> {
        match config.transport_type {
            TransportType::Serial => {
                let transport = serial::SerialTransport::new(config)?;
                Ok(Box::new(transport))
            }
            TransportType::Tcp => {
                let transport = tcp::TcpTransport::new(config)?;
                Ok(Box::new(transport))
            }
            TransportType::Udp => {
                let transport = udp::UdpTransport::new(config)?;
                Ok(Box::new(transport))
            }
            TransportType::Ssh => {
                let transport = ssh::SshTransport::new(config)?;
                Ok(Box::new(transport))
            }
        }
    }
    
    /// List available transports on the system
    pub async fn list_available() -> TransportResult<Vec<TransportInfo>> {
        let mut available = Vec::new();
        
        // Check for serial ports
        if let Ok(ports) = serial::SerialTransport::list_ports().await {
            for port_info in ports {
                available.push(TransportInfo {
                    transport_type: TransportType::Serial,
                    name: port_info.name.clone(),
                    address: port_info.name,
                    available: true,
                });
            }
        }
        
        // TODO: Add network interface detection for TCP/UDP
        // TODO: Add SSH host detection
        
        Ok(available)
    }
}

/// Information about available transport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportInfo {
    pub transport_type: TransportType,
    pub name: String,
    pub address: String,
    pub available: bool,
}