use async_trait::async_trait;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use rand::Rng;
use crate::transport::{
    Transport, TransportBase, TransportConfig, TransportError, TransportResult, 
    TransportStats, TransportType, ConnectionState
};

/// Serial port transport implementation
pub struct SerialTransport {
    base: TransportBase,
    port: Arc<Mutex<Option<SerialPortWrapper>>>, // Using Arc for shared access from monitor
    reconnect_attempts: u32,
    max_reconnect_attempts: u32,
    base_reconnect_delay: Duration,
    task_handles: Vec<JoinHandle<()>>,  // Track spawned tasks for cleanup
    cleanup_flag: Arc<AtomicBool>,      // Signal for cooperative shutdown
    connection_state: Arc<AtomicBool>,  // Track connection state for monitor
}

impl SerialTransport {
    /// Create a new serial transport
    pub fn new(config: TransportConfig) -> TransportResult<Self> {
        // Validate configuration
        if let crate::transport::common::TransportSettings::Serial(ref settings) = config.settings {
            if settings.baud_rate == 0 {
                return Err(TransportError::ConfigError("Invalid baud rate".into()));
            }
        } else {
            return Err(TransportError::ConfigError("Invalid settings for serial transport".into()));
        }
        
        let mut transport = SerialTransport {
            base: TransportBase::new(
                format!("Serial:{}", config.address),
                TransportType::Serial,
                config.clone(),
            ),
            port: Arc::new(Mutex::new(None)),
            reconnect_attempts: 0,
            max_reconnect_attempts: 10,
            base_reconnect_delay: Duration::from_millis(100),
            task_handles: Vec::new(),
            cleanup_flag: Arc::new(AtomicBool::new(false)),
            connection_state: Arc::new(AtomicBool::new(false)),
        };
        
        // Start monitoring loop immediately if auto-reconnect is enabled
        // This allows hot-plug detection even before first connection attempt
        if config.auto_reconnect {
            transport.start_connection_monitor();
        }
        
        Ok(transport)
    }
    
    /// List available serial ports
    pub fn list_ports() -> TransportResult<Vec<String>> {
        // TODO: Implement actual serial port enumeration
        // For now, return mock ports
        Ok(vec![
            "COM1".to_string(),
            "COM3".to_string(),
            "COM4".to_string(),
        ])
    }
    
    /// Probe if a device is connected to this port
    pub async fn probe_port(port_name: &str) -> TransportResult<bool> {
        // TODO: Implement actual probing
        // This would try to open the port and send a probe command
        Ok(port_name == "COM3") // Mock: pretend COM3 has a device
    }
    
    /// Start a background task to monitor connection and trigger reconnection
    fn start_connection_monitor(&mut self) {
        let cleanup_flag = self.cleanup_flag.clone();
        let connection_state = self.connection_state.clone();
        let port = self.port.clone();
        let address = self.base.config.address.clone();
        let max_reconnect_attempts = self.max_reconnect_attempts;
        let base_reconnect_delay = self.base_reconnect_delay;
        
        let monitor_handle = tokio::spawn(async move {
            let mut reconnect_attempts = 0u32;
            let mut check_interval = Duration::from_millis(1000); // Default check interval
            
            while !cleanup_flag.load(Ordering::Relaxed) {
                tokio::time::sleep(check_interval).await;
                
                // Check if the port exists in the system
                let port_available = match serialport::available_ports() {
                    Ok(ports) => ports.iter().any(|p| p.port_name == address),
                    Err(_) => false
                };
                
                // Check our current connection state
                let was_connected = connection_state.load(Ordering::Relaxed);
                let have_port = {
                    let port_guard = port.lock().await;
                    port_guard.is_some()
                };
                
                // Handle state transitions
                match (was_connected, have_port, port_available) {
                    // Hot-plug detected! Device became available while disconnected
                    (false, false, true) => {
                        tracing::info!("Hot-plug detected! {} became available", address);
                        // Immediately try to connect to the newly available device
                        match SerialPortWrapper::new(&address) {
                            Ok(new_port) => {
                                let mut port_guard = port.lock().await;
                                *port_guard = Some(new_port);
                                connection_state.store(true, Ordering::Relaxed);
                                reconnect_attempts = 0;
                                tracing::info!("Hot-plug connection successful!");
                            }
                            Err(e) => {
                                tracing::warn!("Hot-plug connection failed: {}", e);
                            }
                        }
                    }
                    
                    // Active disconnection detected!
                    (true, true, false) => {
                        tracing::warn!("Disconnection detected! {} no longer available", address);
                        connection_state.store(false, Ordering::Relaxed);
                        
                        // Clear the port since it's no longer valid
                        let mut port_guard = port.lock().await;
                        *port_guard = None;
                    }
                    
                    // Connected and healthy - verify connection is still working
                    (true, true, true) => {
                        // Perform actual health check on the port
                        let mut port_guard = port.lock().await;
                        if let Some(ref mut serial_port) = *port_guard {
                            if !serial_port.check_health() {
                                // Port is unhealthy - mark as disconnected
                                tracing::warn!("Health check failed! Port {} is no longer responsive", address);
                                connection_state.store(false, Ordering::Relaxed);
                                *port_guard = None; // Clear the unhealthy port
                            }
                        }
                    }
                    
                    // Other states don't need action
                    _ => {}
                }
                
                // Check if we're disconnected and should try connecting/reconnecting
                if !connection_state.load(Ordering::Relaxed) && port_available {
                    // Port is available but we're not connected - try to connect
                    let port_guard = port.lock().await;
                    if port_guard.is_none() && reconnect_attempts < max_reconnect_attempts {
                        drop(port_guard); // Release lock before connection attempt
                        
                        reconnect_attempts += 1;
                        
                        // Calculate exponential backoff delay
                        let delay = base_reconnect_delay * 2u32.pow(reconnect_attempts - 1);
                        let jitter = rand::thread_rng().gen_range(0..delay.as_millis() as u64 / 4);
                        let total_delay = delay + Duration::from_millis(jitter);
                        
                        tracing::info!(
                            "Monitor detected disconnection. Attempting reconnect {} of {} after {:?}",
                            reconnect_attempts,
                            max_reconnect_attempts,
                            total_delay
                        );
                        
                        tokio::time::sleep(total_delay).await;
                        
                        // Attempt reconnection
                        match SerialPortWrapper::new(&address) {
                            Ok(new_port) => {
                                let mut port_guard = port.lock().await;
                                *port_guard = Some(new_port);
                                connection_state.store(true, Ordering::Relaxed);
                                reconnect_attempts = 0;
                                tracing::info!("Monitor successfully reconnected to serial port");
                            }
                            Err(e) => {
                                tracing::warn!("Monitor reconnect attempt {} failed: {}", reconnect_attempts, e);
                                
                                // Check if this is a permanent error
                                match e {
                                    TransportError::ConfigError(_) | 
                                    TransportError::PermissionDenied(_) => {
                                        tracing::error!("Permanent error detected, stopping reconnection attempts");
                                        break;
                                    }
                                    _ => continue,
                                }
                            }
                        }
                    }
                } else {
                    // Reset attempts when connected
                    reconnect_attempts = 0;
                }
                
                // Adjust check interval based on connection state
                if connection_state.load(Ordering::Relaxed) {
                    check_interval = Duration::from_millis(2000); // Check every 2s when connected
                } else {
                    check_interval = Duration::from_millis(1000); // Check every 1s when disconnected
                }
            }
            
            tracing::info!("Connection monitor stopped");
        });
        
        self.task_handles.push(monitor_handle);
        tracing::info!("Started connection monitor for serial transport");
    }
    
    /// Trigger automatic reconnection in the background
    async fn trigger_auto_reconnection(&self) {
        let address = self.base.config.address.clone();
        
        // Create a closure that attempts to connect
        let connect_fn = move || -> std::pin::Pin<Box<dyn std::future::Future<Output = TransportResult<()>> + Send>> {
            let addr = address.clone();
            Box::pin(async move {
                // TODO: Implement actual serial port connection
                // For now, simulate connection attempt
                let _mock_port = SerialPortWrapper::new(&addr)?;
                
                // In real implementation, we'd need to update self.port
                // This is challenging due to ownership - may need Arc<Mutex<>>
                tracing::info!("Mock serial reconnection to {}", addr);
                Ok(())
            })
        };
        
        // Trigger reconnection through TransportBase
        if let Err(e) = self.base.trigger_reconnection(connect_fn).await {
            tracing::error!("Failed to trigger reconnection: {}", e);
        }
    }
    
    /// Attempt to reconnect with exponential backoff
    pub async fn reconnect(&mut self) -> TransportResult<()> {
        if self.is_connected() {
            return Ok(());
        }
        
        while self.reconnect_attempts < self.max_reconnect_attempts {
            self.reconnect_attempts += 1;
            
            // Calculate delay with exponential backoff and jitter
            let delay = self.base_reconnect_delay * 2u32.pow(self.reconnect_attempts - 1);
            let jitter = rand::thread_rng().gen_range(0..delay.as_millis() as u64 / 4);
            let total_delay = delay + Duration::from_millis(jitter);
            
            tracing::info!(
                "Attempting reconnect {} of {} after {:?}",
                self.reconnect_attempts,
                self.max_reconnect_attempts,
                total_delay
            );
            
            // Wait before attempting reconnect
            tokio::time::sleep(total_delay).await;
            
            // Try to connect
            match self.connect().await {
                Ok(_) => {
                    tracing::info!("Successfully reconnected to serial port");
                    self.reconnect_attempts = 0;
                    return Ok(());
                }
                Err(e) => {
                    tracing::warn!("Reconnect attempt {} failed: {}", self.reconnect_attempts, e);
                    
                    // If this is a permanent error, don't retry
                    match e {
                        TransportError::ConfigError(_) | 
                        TransportError::PermissionDenied(_) => {
                            return Err(e);
                        }
                        _ => continue,
                    }
                }
            }
        }
        
        Err(TransportError::ConnectionFailed(format!(
            "Failed to reconnect after {} attempts",
            self.max_reconnect_attempts
        )))
    }
}

#[async_trait]
impl Transport for SerialTransport {
    fn transport_type(&self) -> TransportType {
        self.base.transport_type
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn is_connected(&self) -> bool {
        self.connection_state.load(Ordering::Relaxed)
    }
    
    async fn connect(&mut self) -> TransportResult<()> {
        if self.is_connected() {
            return Err(TransportError::AlreadyConnected);
        }
        
        self.base.set_state(ConnectionState::Connecting).await;
        
        // TODO: Implement actual serial port connection
        // For now, create a mock connection
        let mock_port = SerialPortWrapper::new(&self.base.config.address)?;
        
        // Update the shared port
        {
            let mut port_guard = self.port.lock().await;
            *port_guard = Some(mock_port);
        }
        
        // Set connection state
        self.connection_state.store(true, Ordering::Relaxed);
        
        self.base.set_state(ConnectionState::Connected).await;
        self.base.update_stats(|stats| {
            stats.reconnect_count = 0;
        }).await;
        
        // Start connection monitoring if auto-reconnect is enabled
        if self.base.config.auto_reconnect && self.task_handles.is_empty() {
            self.start_connection_monitor();
        }
        
        tracing::info!("Connected to serial port: {}", self.base.config.address);
        Ok(())
    }
    
    async fn disconnect(&mut self) -> TransportResult<()> {
        if !self.is_connected() {
            return Ok(());
        }
        
        // Clean up all resources before disconnecting
        self.cleanup_resources().await?;
        
        tracing::info!("Disconnected from serial port: {}", self.base.config.address);
        Ok(())
    }
    
    async fn send(&mut self, data: &[u8]) -> TransportResult<()> {
        let start = Instant::now();
        
        // Attempt reconnection if not connected and auto-reconnect is enabled
        if !self.is_connected() {
            if self.base.config.auto_reconnect {
                // Trigger automatic reconnection
                self.trigger_auto_reconnection().await;
                
                // For send, we'll return an error and let the next attempt succeed
                self.base.update_stats(|stats| {
                    stats.transactions_failed += 1;
                    stats.last_error = Some("Triggering reconnection".into());
                }).await;
                return Err(TransportError::NotConnected);
            } else {
                // Manual reconnection for backward compatibility
                self.reconnect().await?;
            }
        }
        
        // Start monitoring this operation after connection is ensured
        let guard = self.base.monitor.start_operation("serial_send");
        
        let mut port_guard = self.port.lock().await;
        if let Some(ref mut port) = port_guard.as_mut() {
            match port.write(data) {
                Ok(_) => {
                    drop(port_guard); // Explicitly drop the lock before async operations
                    
                    self.base.update_stats(|stats| {
                        stats.bytes_sent += data.len() as u64;
                        stats.transactions_success += 1;
                    }).await;
                    
                    // Enforce minimum latency requirement (50ms for serial)
                    self.base.enforce_latency(start).await?;
                    
                    // Complete the monitoring guard
                    guard.complete().await;
                    
                    Ok(())
                }
                Err(e) => {
                    let err_str = e.to_string();
                    drop(port_guard); // Explicitly drop the lock before modifying self
                    
                    self.base.update_stats(|stats| {
                        stats.transactions_failed += 1;
                        stats.last_error = Some(err_str.clone());
                    }).await;
                    
                    // Connection lost, clear port and trigger reconnection
                    {
                        let mut port_guard = self.port.lock().await;
                        *port_guard = None;
                    }
                    self.connection_state.store(false, Ordering::Relaxed);
                    self.base.set_state(ConnectionState::Disconnected).await;
                    
                    // Trigger automatic reconnection if enabled
                    if self.base.config.auto_reconnect && crate::transport::backoff::is_retryable_error(&e) {
                        self.trigger_auto_reconnection().await;
                    }
                    
                    Err(e)
                }
            }
        } else {
            drop(port_guard);
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            Err(TransportError::NotConnected)
        }
    }
    
    async fn receive(&mut self, timeout: Duration) -> TransportResult<Vec<u8>> {
        // Start monitoring this operation
        let guard = self.base.monitor.start_operation("serial_receive");
        let start = Instant::now();
        
        let mut port_guard = self.port.lock().await;
        if let Some(ref mut port) = port_guard.as_mut() {
            // Set up timeout
            let deadline = Instant::now() + timeout;
            
            // Try to read data
            let data = tokio::time::timeout_at(
                deadline.into(),
                port.read()
            ).await
                .map_err(|_| TransportError::Timeout(format!("Receive timeout after {:?}", timeout)))?;
            
            self.base.update_stats(|stats| {
                stats.bytes_received += data.len() as u64;
            }).await;
            
            // Enforce minimum latency
            self.base.enforce_latency(start).await?;
            
            // Complete the monitoring guard
            guard.complete().await;
            
            Ok(data)
        } else {
            drop(port_guard);
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            Err(TransportError::NotConnected)
        }
    }
    
    fn stats(&self) -> TransportStats {
        // This would need async but trait doesn't support it
        // Return default for now, real implementation would cache stats
        TransportStats::default()
    }
    
    async fn reset(&mut self) -> TransportResult<()> {
        let mut port_guard = self.port.lock().await;
        if let Some(ref mut port) = port_guard.as_mut() {
            port.flush()?;
            Ok(())
        } else {
            Err(TransportError::NotConnected)
        }
    }
    
    fn config(&self) -> &TransportConfig {
        &self.base.config
    }
    
    async fn cleanup_resources(&mut self) -> TransportResult<()> {
        // Cancel any active reconnection attempts
        self.base.cancel_reconnection().await;
        
        // Signal shutdown to any cooperative tasks
        self.cleanup_flag.store(true, Ordering::Relaxed);
        
        // Abort all spawned tasks
        for handle in self.task_handles.drain(..) {
            handle.abort();
        }
        
        // Drop the port connection
        {
            let mut port_guard = self.port.lock().await;
            *port_guard = None;
        }
        
        // Clear connection state
        self.connection_state.store(false, Ordering::Relaxed);
        
        // Reset the cleanup flag for next connection
        self.cleanup_flag.store(false, Ordering::Relaxed);
        
        // Reset reconnect attempts counter
        self.reconnect_attempts = 0;
        
        // Update state
        self.base.set_state(ConnectionState::Disconnected).await;
        
        tracing::debug!("Serial transport resources cleaned up");
        Ok(())
    }
}

/// Wrapper around real serial port
struct SerialPortWrapper {
    port: Box<dyn serialport::SerialPort>,
    port_name: String,
}

impl SerialPortWrapper {
    fn new(port_name: &str) -> TransportResult<Self> {
        // Actually try to open the port - will fail if no device
        let port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(100))
            .data_bits(serialport::DataBits::Eight)
            .parity(serialport::Parity::None)
            .stop_bits(serialport::StopBits::One)
            .flow_control(serialport::FlowControl::None)
            .open()
            .map_err(|e| {
                use serialport::ErrorKind;
                match e.kind() {
                    ErrorKind::NoDevice => TransportError::ConnectionFailed(
                        format!("No device found on port {}", port_name)
                    ),
                    ErrorKind::InvalidInput => TransportError::ConfigError(
                        format!("Invalid port name: {}", port_name)
                    ),
                    _ => TransportError::ConnectionFailed(
                        format!("Failed to open port {}: {}", port_name, e)
                    ),
                }
            })?;
        
        Ok(SerialPortWrapper {
            port,
            port_name: port_name.to_string(),
        })
    }
    
    fn write(&mut self, data: &[u8]) -> TransportResult<()> {
        use std::io::Write;
        self.port.write_all(data).map_err(|e| {
            // IO errors often indicate disconnection on serial ports
            TransportError::IoError(e)
        })
    }
    
    async fn read(&mut self) -> Vec<u8> {
        let mut buf = vec![0u8; 256];
        match self.port.read(&mut buf) {
            Ok(n) => {
                buf.truncate(n);
                buf
            }
            Err(e) => {
                // Log the error - IO errors often indicate disconnection
                tracing::warn!("Read error (possible disconnection): {}", e);
                // Return empty on timeout or error
                Vec::new()
            }
        }
    }
    
    fn flush(&mut self) -> TransportResult<()> {
        self.port.flush().map_err(|e| TransportError::IoError(e))
    }
    
    fn check_health(&mut self) -> bool {
        // Try to flush the port - this will fail if device is disconnected
        // bytes_to_read() alone is not reliable for disconnection detection on Windows
        match self.port.flush() {
            Ok(_) => {
                // Also check if we can query port state
                match self.port.bytes_to_read() {
                    Ok(_) => true,
                    Err(e) => {
                        tracing::warn!("Health check failed on bytes_to_read: {}", e);
                        false
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Health check failed on flush: {}", e);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::common::{SerialSettings, TransportSettings};
    
    #[tokio::test]
    async fn test_serial_transport_creation() {
        let config = TransportConfig {
            transport_type: TransportType::Serial,
            address: "COM3".to_string(),
            settings: TransportSettings::Serial(SerialSettings::default()),
            ..Default::default()
        };
        
        let transport = SerialTransport::new(config);
        assert!(transport.is_ok());
    }
    
    #[tokio::test]
    async fn test_serial_connect_disconnect() {
        let config = TransportConfig {
            transport_type: TransportType::Serial,
            address: "COM3".to_string(),
            settings: TransportSettings::Serial(SerialSettings::default()),
            ..Default::default()
        };
        
        let mut transport = SerialTransport::new(config).unwrap();
        
        // Should not be connected initially
        assert!(!transport.is_connected());
        
        // Connect
        let result = transport.connect().await;
        assert!(result.is_ok());
        assert!(transport.is_connected());
        
        // Try to connect again (should fail)
        let result = transport.connect().await;
        assert!(matches!(result, Err(TransportError::AlreadyConnected)));
        
        // Disconnect
        let result = transport.disconnect().await;
        assert!(result.is_ok());
        assert!(!transport.is_connected());
    }
    
    #[tokio::test]
    async fn test_serial_send_receive() {
        let config = TransportConfig {
            transport_type: TransportType::Serial,
            address: "COM3".to_string(),
            settings: TransportSettings::Serial(SerialSettings::default()),
            ..Default::default()
        };
        
        let mut transport = SerialTransport::new(config).unwrap();
        
        // Connect first
        transport.connect().await.unwrap();
        
        // Send data
        let result = transport.send(b"PING").await;
        assert!(result.is_ok());
        
        // Receive response
        let result = transport.receive(Duration::from_secs(1)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"PONG");
    }
    
    #[tokio::test]
    async fn test_latency_enforcement() {
        let config = TransportConfig {
            transport_type: TransportType::Serial,
            address: "COM3".to_string(),
            settings: TransportSettings::Serial(SerialSettings::default()),
            ..Default::default()
        };
        
        let mut transport = SerialTransport::new(config).unwrap();
        transport.connect().await.unwrap();
        
        let start = Instant::now();
        transport.send(b"TEST").await.unwrap();
        let elapsed = start.elapsed();
        
        // Should enforce minimum 50ms latency for serial
        assert!(elapsed >= Duration::from_millis(50));
    }
    
    #[tokio::test]
    async fn test_exponential_backoff() {
        let config = TransportConfig {
            transport_type: TransportType::Serial,
            address: "COM3".to_string(),
            settings: TransportSettings::Serial(SerialSettings::default()),
            ..Default::default()
        };
        
        let mut transport = SerialTransport::new(config).unwrap();
        
        // First connect should succeed
        transport.connect().await.unwrap();
        assert_eq!(transport.reconnect_attempts, 0);
        
        // Disconnect
        transport.disconnect().await.unwrap();
        
        // Test reconnect with exponential backoff
        // Since COM3 succeeds, this should reconnect on first attempt
        let start = Instant::now();
        transport.reconnect().await.unwrap();
        let elapsed = start.elapsed();
        
        // First attempt should have 100ms base delay + jitter
        assert!(elapsed >= Duration::from_millis(100));
        assert!(elapsed < Duration::from_millis(200));
        assert_eq!(transport.reconnect_attempts, 0); // Reset after success
    }
    
    #[tokio::test]
    async fn test_auto_reconnect_on_send() {
        let config = TransportConfig {
            transport_type: TransportType::Serial,
            address: "COM3".to_string(),
            settings: TransportSettings::Serial(SerialSettings::default()),
            ..Default::default()
        };
        
        let mut transport = SerialTransport::new(config).unwrap();
        
        // Should not be connected initially
        assert!(!transport.is_connected());
        
        // Send should trigger auto-reconnect
        let result = transport.send(b"TEST").await;
        assert!(result.is_ok());
        assert!(transport.is_connected());
    }
}