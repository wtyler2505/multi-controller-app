use async_trait::async_trait;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::task::{JoinHandle, spawn_blocking};
use rand::Rng;
use uuid::Uuid;
use crate::transport::{
    Transport, TransportBase, TransportConfig, TransportError, TransportResult, 
    TransportStats, TransportType, ConnectionState
};
use crate::transport::common::SerialSettings;

// Type alias for SerialConfig
type SerialConfig = SerialSettings;

// Conversion traits for serialport enums
impl From<crate::transport::common::DataBits> for serialport::DataBits {
    fn from(bits: crate::transport::common::DataBits) -> Self {
        use crate::transport::common::DataBits;
        match bits {
            DataBits::Five => serialport::DataBits::Five,
            DataBits::Six => serialport::DataBits::Six,
            DataBits::Seven => serialport::DataBits::Seven,
            DataBits::Eight => serialport::DataBits::Eight,
        }
    }
}

impl From<crate::transport::common::StopBits> for serialport::StopBits {
    fn from(bits: crate::transport::common::StopBits) -> Self {
        use crate::transport::common::StopBits;
        match bits {
            StopBits::One => serialport::StopBits::One,
            StopBits::Two => serialport::StopBits::Two,
        }
    }
}

impl From<crate::transport::common::Parity> for serialport::Parity {
    fn from(parity: crate::transport::common::Parity) -> Self {
        use crate::transport::common::Parity;
        match parity {
            Parity::None => serialport::Parity::None,
            Parity::Odd => serialport::Parity::Odd,
            Parity::Even => serialport::Parity::Even,
        }
    }
}

impl From<crate::transport::common::FlowControl> for serialport::FlowControl {
    fn from(flow: crate::transport::common::FlowControl) -> Self {
        use crate::transport::common::FlowControl;
        match flow {
            FlowControl::None => serialport::FlowControl::None,
            FlowControl::Software => serialport::FlowControl::Software,
            FlowControl::Hardware => serialport::FlowControl::Hardware,
        }
    }
}

/// Information about a discovered serial port
#[derive(Debug, Clone)]
pub struct PortInfo {
    pub name: String,
    pub device_type: String,
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub serial_number: Option<String>,
}

// Known microcontroller vendor IDs
const ARDUINO_VID: u16 = 0x2341;  // Official Arduino
const FTDI_VID: u16 = 0x0403;     // FTDI chip (common in dev boards)
const CH340_VID: u16 = 0x1A86;    // CH340 chip (common in clones)
const CP210X_VID: u16 = 0x10C4;   // Silicon Labs CP210x
const TEENSY_VID: u16 = 0x16C0;   // Teensy boards
const STM32_VID: u16 = 0x0483;    // STMicroelectronics

/// Check if a USB device is likely a microcontroller
fn is_microcontroller_device(info: &serialport::UsbPortInfo) -> bool {
    matches!(info.vid, 
        ARDUINO_VID | FTDI_VID | CH340_VID | CP210X_VID | TEENSY_VID | STM32_VID
    )
}

/// Detect the specific device type from USB info
fn detect_device_type(info: &serialport::UsbPortInfo) -> String {
    match info.vid {
        ARDUINO_VID => {
            // Check specific Arduino models by PID
            match info.pid {
                0x0043 => "Arduino Uno".to_string(),
                0x0042 => "Arduino Mega".to_string(),
                0x0010 => "Arduino Mega 2560".to_string(),
                0x7523 => "Arduino Nano".to_string(),
                _ => "Arduino Device".to_string(),
            }
        }
        FTDI_VID => "FTDI Serial Device".to_string(),
        CH340_VID => "CH340 Serial Device".to_string(),
        CP210X_VID => "Silicon Labs CP210x".to_string(),
        TEENSY_VID => "Teensy".to_string(),
        STM32_VID => "STM32 Device".to_string(),
        _ => "Unknown USB Serial".to_string(),
    }
}

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
    
    /// List available serial ports with cross-platform support
    pub async fn list_ports() -> TransportResult<Vec<PortInfo>> {
        spawn_blocking(|| {
            let ports = serialport::available_ports()
                .map_err(|e| TransportError::HardwareError(format!("Failed to enumerate ports: {}", e)))?;
            
            let mut discovered = Vec::new();
            
            for port in ports {
                match port.port_type {
                    serialport::SerialPortType::UsbPort(ref info) => {
                        // Check for known microcontroller vendors
                        if is_microcontroller_device(info) {
                            discovered.push(PortInfo {
                                name: port.port_name.clone(),
                                device_type: detect_device_type(info),
                                vendor_id: Some(info.vid),
                                product_id: Some(info.pid),
                                manufacturer: info.manufacturer.clone(),
                                product: info.product.clone(),
                                serial_number: info.serial_number.clone(),
                            });
                        } else {
                            // Include other USB serial devices
                            discovered.push(PortInfo {
                                name: port.port_name.clone(),
                                device_type: "USB Serial".to_string(),
                                vendor_id: Some(info.vid),
                                product_id: Some(info.pid),
                                manufacturer: info.manufacturer.clone(),
                                product: info.product.clone(),
                                serial_number: info.serial_number.clone(),
                            });
                        }
                    }
                    _ => {
                        // Include all other serial ports for manual selection
                        discovered.push(PortInfo {
                            name: port.port_name.clone(),
                            device_type: "Serial Port".to_string(),
                            vendor_id: None,
                            product_id: None,
                            manufacturer: None,
                            product: None,
                            serial_number: None,
                        });
                    }
                }
            }
            
            // Sort by port name for consistent ordering
            discovered.sort_by(|a, b| a.name.cmp(&b.name));
            
            Ok(discovered)
        }).await
        .map_err(|e| TransportError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other, 
            format!("Task join error: {}", e)
        )))?
    }
    
    /// Probe if a device is connected to this port
    pub async fn probe_port(port_name: &str, config: &SerialConfig) -> TransportResult<bool> {
        // Try to open the port and send a probe command
        match SerialPortWrapper::new(port_name, config).await {
            Ok(port) => {
                // Send probe command
                match port.write(b"PROBE\r\n").await {
                    Ok(_) => {
                        // Wait for response
                        match port.read(Duration::from_millis(500)).await {
                            Ok(data) => {
                                let response = String::from_utf8_lossy(&data);
                                // Check for known responses
                                Ok(response.contains("OK") || 
                                   response.contains("ARDUINO") || 
                                   response.contains("READY"))
                            }
                            Err(_) => Ok(false) // No response, but port exists
                        }
                    }
                    Err(_) => Ok(false) // Port exists but can't write
                }
            }
            Err(_) => Ok(false) // Can't open port
        }
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
                        // Extract config from transport settings for reconnection
                        let config = SerialSettings::default(); // TODO: Get from actual config
                        match SerialPortWrapper::new(&address, &config).await {
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
                        if let Some(ref serial_port) = *port_guard {
                            if !serial_port.check_health().await {
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
                        let config = SerialSettings::default(); // TODO: Get from actual config
                        match SerialPortWrapper::new(&address, &config).await {
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
        let config = SerialSettings::default(); // TODO: Get from actual config
        
        // Create a closure that attempts to connect
        let connect_fn = move || -> std::pin::Pin<Box<dyn std::future::Future<Output = TransportResult<()>> + Send>> {
            let addr = address.clone();
            let cfg = config.clone();
            Box::pin(async move {
                // Attempt to create a new serial port connection
                let _new_port = SerialPortWrapper::new(&addr, &cfg).await?;
                
                // In real implementation, we'd need to update self.port
                // This is challenging due to ownership - may need Arc<Mutex<>>
                tracing::info!("Triggered serial reconnection to {}", addr);
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
        
        // Extract serial settings from config
        let serial_config = if let crate::transport::common::TransportSettings::Serial(ref settings) = self.base.config.settings {
            settings.clone()
        } else {
            return Err(TransportError::ConfigError("Invalid settings for serial transport".into()));
        };
        
        // Connect to serial port using proper async patterns
        let serial_port = SerialPortWrapper::new(&self.base.config.address, &serial_config).await?;
        
        // Update the shared port
        {
            let mut port_guard = self.port.lock().await;
            *port_guard = Some(serial_port);
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
        
        tracing::info!("Connected to serial port: {} with session ID: {}", 
                     self.base.config.address, 
                     self.port.lock().await.as_ref().map(|p| p.session_id()).unwrap_or_default());
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
        
        let port_guard = self.port.lock().await;
        if let Some(ref port) = port_guard.as_ref() {
            // Use the port through its async interface
            match port.write(data).await {
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
        
        if !self.is_connected() {
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            return Err(TransportError::NotConnected);
        }
        
        let port_guard = self.port.lock().await;
        if let Some(ref port) = port_guard.as_ref() {
            // Use the async read method with timeout
            match port.read(timeout).await {
                Ok(data) => {
                    drop(port_guard);
                    
                    if !data.is_empty() {
                        self.base.update_stats(|stats| {
                            stats.bytes_received += data.len() as u64;
                        }).await;
                    }
                    
                    // Enforce minimum latency
                    self.base.enforce_latency(start).await?;
                    
                    // Complete the monitoring guard
                    guard.complete().await;
                    
                    Ok(data)
                }
                Err(e) => {
                    drop(port_guard);
                    
                    self.base.update_stats(|stats| {
                        stats.transactions_failed += 1;
                        stats.last_error = Some(e.to_string());
                    }).await;
                    
                    // Enforce minimum latency even on error
                    self.base.enforce_latency(start).await?;
                    
                    // Complete the monitoring guard
                    guard.complete().await;
                    
                    Err(e)
                }
            }
        } else {
            drop(port_guard);
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Port not available".into());
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
        let port_guard = self.port.lock().await;
        if let Some(ref port) = port_guard.as_ref() {
            port.flush().await?;
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

/// Wrapper around real serial port with proper async patterns
struct SerialPortWrapper {
    port: Arc<Mutex<Box<dyn serialport::SerialPort>>>,
    port_name: String,
    session_id: Uuid,
}

impl SerialPortWrapper {
    /// Create new serial port wrapper using spawn_blocking for I/O operations
    async fn new(port_name: &str, config: &SerialConfig) -> TransportResult<Self> {
        let port_name_clone = port_name.to_string();
        let baud_rate = config.baud_rate;
        let timeout_ms = 100u64; // Default timeout in ms
        let data_bits = config.data_bits.into();
        let stop_bits = config.stop_bits.into();
        let parity = config.parity.into();
        let flow_control = config.flow_control.into();
        
        // CRITICAL: Use spawn_blocking for serial port opening
        let port = spawn_blocking(move || {
            serialport::new(&port_name_clone, baud_rate)
                .timeout(Duration::from_millis(timeout_ms))
                .data_bits(data_bits)
                .parity(parity)
                .stop_bits(stop_bits)
                .flow_control(flow_control)
                .open()
                .map_err(|e| {
                    use serialport::ErrorKind;
                    match e.kind() {
                        ErrorKind::NoDevice => TransportError::ConnectionFailed(
                            format!("No device found on port {}", port_name_clone)
                        ),
                        ErrorKind::InvalidInput => TransportError::ConfigError(
                            format!("Invalid port name: {}", port_name_clone)
                        ),
                        _ => TransportError::ConnectionFailed(
                            format!("Failed to open port {}: {}", port_name_clone, e)
                        ),
                    }
                })
        }).await
        .map_err(|e| TransportError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other, 
            format!("Task join error: {}", e)
        )))??;
        
        Ok(SerialPortWrapper {
            port: Arc::new(Mutex::new(port)),
            port_name: port_name.to_string(),
            session_id: Uuid::new_v4(),
        })
    }
    
    /// Write data using spawn_blocking for async safety
    async fn write(&self, data: &[u8]) -> TransportResult<()> {
        use std::io::Write;
        
        let port = self.port.clone();
        let data = data.to_vec();
        
        // CRITICAL: Use spawn_blocking for serial write operations
        spawn_blocking(move || {
            let mut port_guard = port.blocking_lock();
            port_guard.write_all(&data).map_err(|e| {
                // IO errors often indicate disconnection on serial ports
                TransportError::IoError(e)
            })?;
            port_guard.flush().map_err(|e| TransportError::IoError(e))
        }).await
        .map_err(|e| TransportError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other, 
            format!("Task join error: {}", e)
        )))?
    }
    
    /// Read data using spawn_blocking for async safety
    async fn read(&self, timeout: Duration) -> TransportResult<Vec<u8>> {
        let port = self.port.clone();
        
        // CRITICAL: Use spawn_blocking for serial read operations
        spawn_blocking(move || {
            let mut port_guard = port.blocking_lock();
            let mut buf = vec![0u8; 1024]; // Larger buffer for better performance
            
            // Set timeout for this specific read
            port_guard.set_timeout(timeout)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            
            match port_guard.read(&mut buf) {
                Ok(n) => {
                    buf.truncate(n);
                    Ok(buf)
                }
                Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    // Timeout is not an error, just no data available
                    Ok(Vec::new())
                }
                Err(e) => {
                    // Log the error - IO errors often indicate disconnection
                    tracing::warn!("Read error (possible disconnection): {}", e);
                    Err(TransportError::IoError(e))
                }
            }
        }).await
        .map_err(|e| TransportError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other, 
            format!("Task join error: {}", e)
        )))?
    }
    
    /// Flush port using spawn_blocking
    async fn flush(&self) -> TransportResult<()> {
        let port = self.port.clone();
        
        spawn_blocking(move || {
            let mut port_guard = port.blocking_lock();
            port_guard.flush().map_err(|e| TransportError::IoError(e))
        }).await
        .map_err(|e| TransportError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other, 
            format!("Task join error: {}", e)
        )))?
    }
    
    /// Check port health using spawn_blocking
    async fn check_health(&self) -> bool {
        let port = self.port.clone();
        
        let result = spawn_blocking(move || {
            let mut port_guard = port.blocking_lock();
            
            // Try to flush the port - this will fail if device is disconnected
            // bytes_to_read() alone is not reliable for disconnection detection on Windows
            match port_guard.flush() {
                Ok(_) => {
                    // Also check if we can query port state
                    match port_guard.bytes_to_read() {
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
        }).await;
        
        result.unwrap_or(false)
    }
    
    /// Get session ID for tracking
    fn session_id(&self) -> Uuid {
        self.session_id
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