//! Serial Transport Implementation
//! 
//! This crate provides a serial port transport implementation using the serialport-rs crate,
//! with async support, reconnection logic, and cross-platform compatibility.

use multi_controller_core::{
    Result, Transport, TransportType, TransportInfo, TransportConfig, 
    MultiControllerError, async_trait
};
use serialport::SerialPortBuilder;
use std::sync::{Arc, Mutex};
use std::io::{Read, Write};
use std::time::{Duration, Instant};
use tracing::{debug, warn, error, info};
use tokio::time::timeout;

/// Serial-specific configuration options
#[derive(Debug, Clone)]
pub struct SerialConfig {
    pub port_name: String,
    pub baud_rate: u32,
    pub data_bits: serialport::DataBits,
    pub stop_bits: serialport::StopBits,
    pub parity: serialport::Parity,
    pub flow_control: serialport::FlowControl,
    pub transport_config: TransportConfig,
}

impl SerialConfig {
    pub fn new(port_name: String) -> Self {
        Self {
            port_name,
            baud_rate: 115200,
            data_bits: serialport::DataBits::Eight,
            stop_bits: serialport::StopBits::One,
            parity: serialport::Parity::None,
            flow_control: serialport::FlowControl::None,
            transport_config: TransportConfig::default(),
        }
    }
    
    pub fn with_baud_rate(mut self, baud_rate: u32) -> Self {
        self.baud_rate = baud_rate;
        self
    }
}

/// Thread-safe wrapper for serial port
type SafeSerialPort = Arc<Mutex<Option<Box<dyn serialport::SerialPort + Send>>>>;

/// Serial port transport implementation
pub struct SerialTransport {
    config: SerialConfig,
    port: SafeSerialPort,
    connected: Arc<Mutex<bool>>,
    last_activity: Arc<Mutex<Option<Instant>>>,
    bytes_sent: Arc<Mutex<u64>>,
    bytes_received: Arc<Mutex<u64>>,
    reconnect_attempts: u32,
}

impl SerialTransport {
    /// Create a new serial transport instance
    pub fn new(config: SerialConfig) -> Self {
        Self {
            config,
            port: Arc::new(Mutex::new(None)),
            connected: Arc::new(Mutex::new(false)),
            last_activity: Arc::new(Mutex::new(None)),
            bytes_sent: Arc::new(Mutex::new(0)),
            bytes_received: Arc::new(Mutex::new(0)),
            reconnect_attempts: 0,
        }
    }
    
    /// Get available serial ports on the system
    pub fn available_ports() -> Result<Vec<String>> {
        match serialport::available_ports() {
            Ok(ports) => {
                let port_names: Vec<String> = ports
                    .into_iter()
                    .map(|port| port.port_name)
                    .collect();
                Ok(port_names)
            }
            Err(e) => {
                error!("Failed to enumerate serial ports: {}", e);
                Err(MultiControllerError::SerialPort(e.to_string()))
            }
        }
    }
    
    /// Create a serial port builder with the current configuration
    fn create_port_builder(&self) -> SerialPortBuilder {
        serialport::new(&self.config.port_name, self.config.baud_rate)
            .data_bits(self.config.data_bits)
            .stop_bits(self.config.stop_bits)
            .parity(self.config.parity)
            .flow_control(self.config.flow_control)
            .timeout(self.config.transport_config.read_timeout)
    }
    
    /// Attempt to connect with retry logic
    async fn connect_with_retry(&mut self) -> Result<()> {
        let max_attempts = self.config.transport_config.reconnect_attempts;
        
        for attempt in 1..=max_attempts {
            match self.attempt_connection().await {
                Ok(()) => {
                    self.reconnect_attempts = 0;
                    info!("Serial port {} connected successfully", self.config.port_name);
                    return Ok(());
                }
                Err(e) => {
                    self.reconnect_attempts = attempt;
                    warn!(
                        "Serial connection attempt {}/{} failed for {}: {}",
                        attempt, max_attempts, self.config.port_name, e
                    );
                    
                    if attempt < max_attempts {
                        tokio::time::sleep(self.config.transport_config.reconnect_delay).await;
                    }
                }
            }
        }
        
        Err(MultiControllerError::Connection(
            format!("Failed to connect to {} after {} attempts", 
                   self.config.port_name, max_attempts)
        ))
    }
    
    /// Single connection attempt
    async fn attempt_connection(&mut self) -> Result<()> {
        debug!("Attempting to connect to serial port: {}", self.config.port_name);
        
        let builder = self.create_port_builder();
        
        // Use timeout for the connection attempt
        let port_result = timeout(
            self.config.transport_config.connection_timeout,
            tokio::task::spawn_blocking(move || builder.open())
        ).await;
        
        match port_result {
            Ok(open_result) => match open_result {
                Ok(port) => {
                    {
                        let mut port_guard = self.port.lock().unwrap();
                        *port_guard = Some(port);
                    }
                    {
                        let mut connected = self.connected.lock().unwrap();
                        *connected = true;
                    }
                    {
                        let mut activity = self.last_activity.lock().unwrap();
                        *activity = Some(Instant::now());
                    }
                    Ok(())
                }
                Err(e) => {
                    error!("Serial port open failed: {}", e);
                    Err(MultiControllerError::SerialPort(e.to_string()))
                }
            },
            Err(_) => {
                error!("Serial port connection timed out");
                Err(MultiControllerError::Timeout {
                    timeout_ms: self.config.transport_config.connection_timeout.as_millis() as u64,
                })
            }
        }
    }
}

#[async_trait]
impl Transport for SerialTransport {
    fn transport_type(&self) -> TransportType {
        TransportType::Serial
    }
    
    fn name(&self) -> &str {
        &self.config.port_name
    }
    
    fn is_connected(&self) -> bool {
        let connected = self.connected.lock().unwrap();
        let port = self.port.lock().unwrap();
        *connected && port.is_some()
    }
    
    async fn connect(&mut self) -> Result<()> {
        if self.is_connected() {
            debug!("Serial port {} already connected", self.config.port_name);
            return Ok(());
        }
        
        self.connect_with_retry().await
    }
    
    async fn disconnect(&mut self) -> Result<()> {
        debug!("Disconnecting from serial port: {}", self.config.port_name);
        
        {
            let mut port_guard = self.port.lock().unwrap();
            if let Some(mut port) = port_guard.take() {
                // Attempt to flush before closing
                if let Err(e) = port.flush() {
                    warn!("Failed to flush serial port before disconnect: {}", e);
                }
                // Port will be dropped here, closing the connection
            }
        }
        
        {
            let mut connected = self.connected.lock().unwrap();
            *connected = false;
        }
        
        info!("Disconnected from serial port: {}", self.config.port_name);
        Ok(())
    }
    
    async fn send(&mut self, data: &[u8]) -> Result<usize> {
        if !self.is_connected() {
            return Err(MultiControllerError::Connection(
                "Serial port not connected".to_string()
            ));
        }
        
        let data_clone = data.to_vec();
        let port_clone = self.port.clone();
        
        let write_result = timeout(
            self.config.transport_config.write_timeout,
            tokio::task::spawn_blocking(move || {
                let mut port_guard = port_clone.lock().unwrap();
                if let Some(ref mut port) = port_guard.as_mut() {
                    port.write(&data_clone)
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::NotConnected,
                        "Serial port not available"
                    ))
                }
            })
        ).await;
        
        match write_result {
            Ok(task_result) => match task_result {
                Ok(bytes_written) => {
                    {
                        let mut sent = self.bytes_sent.lock().unwrap();
                        *sent += bytes_written as u64;
                    }
                    {
                        let mut activity = self.last_activity.lock().unwrap();
                        *activity = Some(Instant::now());
                    }
                    debug!("Sent {} bytes to {}", bytes_written, self.config.port_name);
                    Ok(bytes_written)
                }
                Err(e) => {
                    error!("Serial write error: {}", e);
                    {
                        let mut connected = self.connected.lock().unwrap();
                        *connected = false;
                    }
                    Err(MultiControllerError::Io(e))
                }
            },
            Err(_) => {
                error!("Serial write timeout");
                Err(MultiControllerError::Timeout {
                    timeout_ms: self.config.transport_config.write_timeout.as_millis() as u64,
                })
            }
        }
    }
    
    async fn receive(&mut self, buffer: &mut [u8]) -> Result<usize> {
        if !self.is_connected() {
            return Err(MultiControllerError::Connection(
                "Serial port not connected".to_string()
            ));
        }
        
        let buffer_len = buffer.len();
        let port_clone = self.port.clone();
        
        let read_result = timeout(
            self.config.transport_config.read_timeout,
            tokio::task::spawn_blocking(move || {
                let mut port_guard = port_clone.lock().unwrap();
                if let Some(ref mut port) = port_guard.as_mut() {
                    let mut temp_buffer = vec![0u8; buffer_len];
                    match port.read(&mut temp_buffer) {
                        Ok(bytes_read) => Ok((bytes_read, temp_buffer)),
                        Err(e) => Err(e),
                    }
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::NotConnected,
                        "Serial port not available"
                    ))
                }
            })
        ).await;
        
        match read_result {
            Ok(task_result) => match task_result {
                Ok((bytes_read, temp_buffer)) => {
                    buffer[..bytes_read].copy_from_slice(&temp_buffer[..bytes_read]);
                    {
                        let mut received = self.bytes_received.lock().unwrap();
                        *received += bytes_read as u64;
                    }
                    {
                        let mut activity = self.last_activity.lock().unwrap();
                        *activity = Some(Instant::now());
                    }
                    debug!("Received {} bytes from {}", bytes_read, self.config.port_name);
                    Ok(bytes_read)
                }
                Err(e) => {
                    // Handle timeout as a special case (not an error for serial ports)
                    if e.kind() == std::io::ErrorKind::TimedOut {
                        Ok(0) // No data available
                    } else {
                        error!("Serial read error: {}", e);
                        {
                            let mut connected = self.connected.lock().unwrap();
                            *connected = false;
                        }
                        Err(MultiControllerError::Io(e))
                    }
                }
            },
            Err(_) => {
                error!("Serial read timeout");
                Err(MultiControllerError::Timeout {
                    timeout_ms: self.config.transport_config.read_timeout.as_millis() as u64,
                })
            }
        }
    }
    
    async fn flush(&mut self) -> Result<()> {
        if !self.is_connected() {
            return Err(MultiControllerError::Connection(
                "Serial port not connected".to_string()
            ));
        }
        
        let port_clone = self.port.clone();
        
        let flush_result = tokio::task::spawn_blocking(move || {
            let mut port_guard = port_clone.lock().unwrap();
            if let Some(ref mut port) = port_guard.as_mut() {
                port.flush()
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotConnected,
                    "Serial port not available"
                ))
            }
        }).await;
        
        match flush_result {
            Ok(task_result) => match task_result {
                Ok(()) => {
                    debug!("Flushed serial port: {}", self.config.port_name);
                    Ok(())
                }
                Err(e) => {
                    error!("Serial flush error: {}", e);
                    Err(MultiControllerError::Io(e))
                }
            },
            Err(e) => {
                error!("Serial flush task error: {}", e);
                Err(MultiControllerError::Connection("Flush task failed".to_string()))
            }
        }
    }
    
    fn info(&self) -> TransportInfo {
        let connected = {
            let connected = self.connected.lock().unwrap();
            *connected
        };
        let last_activity = {
            let activity = self.last_activity.lock().unwrap();
            *activity
        };
        let bytes_sent = {
            let sent = self.bytes_sent.lock().unwrap();
            *sent
        };
        let bytes_received = {
            let received = self.bytes_received.lock().unwrap();
            *received
        };
        
        TransportInfo {
            transport_type: TransportType::Serial,
            endpoint: self.config.port_name.clone(),
            connected,
            last_activity,
            bytes_sent,
            bytes_received,
        }
    }
}

// RAII cleanup implementation
impl Drop for SerialTransport {
    fn drop(&mut self) {
        if self.is_connected() {
            debug!("Dropping SerialTransport, ensuring cleanup for {}", self.config.port_name);
            // Clean up the port
            let mut port_guard = self.port.lock().unwrap();
            *port_guard = None;
            let mut connected = self.connected.lock().unwrap();
            *connected = false;
        }
    }
}