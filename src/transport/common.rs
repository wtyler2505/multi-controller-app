use serde::{Serialize, Deserialize};
use std::fmt;
use std::error::Error;
use std::time::Duration;

/// Transport types supported by the application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportType {
    Serial,
    Tcp,
    Udp,
    Ssh,
}

impl fmt::Display for TransportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransportType::Serial => write!(f, "Serial"),
            TransportType::Tcp => write!(f, "TCP"),
            TransportType::Udp => write!(f, "UDP"),
            TransportType::Ssh => write!(f, "SSH"),
        }
    }
}

/// Transport-specific errors
#[derive(Debug)]
pub enum TransportError {
    /// Connection failed
    ConnectionFailed(String),
    
    /// Transport is not connected
    NotConnected,
    
    /// Already connected
    AlreadyConnected,
    
    /// Timeout occurred
    Timeout(String),
    
    /// I/O error
    IoError(std::io::Error),
    
    /// Configuration error
    ConfigError(String),
    
    /// Protocol error
    ProtocolError(String),
    
    /// Buffer overflow
    BufferOverflow,
    
    /// Invalid data received
    InvalidData(String),
    
    /// Feature not implemented
    NotImplemented(String),
    
    /// Hardware error
    HardwareError(String),
    
    /// Permission denied
    PermissionDenied(String),
    
    /// Resource unavailable
    ResourceUnavailable(String),
    
    /// Other error
    Other(String),
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransportError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            TransportError::NotConnected => write!(f, "Transport is not connected"),
            TransportError::AlreadyConnected => write!(f, "Transport is already connected"),
            TransportError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            TransportError::IoError(err) => write!(f, "I/O error: {}", err),
            TransportError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            TransportError::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            TransportError::BufferOverflow => write!(f, "Buffer overflow"),
            TransportError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            TransportError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            TransportError::HardwareError(msg) => write!(f, "Hardware error: {}", msg),
            TransportError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            TransportError::ResourceUnavailable(msg) => write!(f, "Resource unavailable: {}", msg),
            TransportError::Other(msg) => write!(f, "Transport error: {}", msg),
        }
    }
}

impl Error for TransportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TransportError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for TransportError {
    fn from(err: std::io::Error) -> Self {
        TransportError::IoError(err)
    }
}

/// Result type for transport operations
pub type TransportResult<T> = Result<T, TransportError>;

/// Transport configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    /// Transport type
    pub transport_type: TransportType,
    
    /// Connection address (port name, IP address, etc.)
    pub address: String,
    
    /// Connection timeout in milliseconds
    pub connect_timeout_ms: u32,
    
    /// Read timeout in milliseconds
    pub read_timeout_ms: u32,
    
    /// Write timeout in milliseconds
    pub write_timeout_ms: u32,
    
    /// Automatic reconnection
    pub auto_reconnect: bool,
    
    /// Maximum reconnection attempts
    pub max_reconnect_attempts: u32,
    
    /// Reconnection delay in milliseconds
    pub reconnect_delay_ms: u32,
    
    /// Buffer size for reading
    pub read_buffer_size: usize,
    
    /// Buffer size for writing
    pub write_buffer_size: usize,
    
    /// Require handshake on connection (for UDP)
    pub require_handshake: bool,
    
    /// Minimum latency to enforce between operations (optional)
    pub min_latency: Option<Duration>,
    
    /// Transport-specific settings
    pub settings: TransportSettings,
}

impl Default for TransportConfig {
    fn default() -> Self {
        TransportConfig {
            transport_type: TransportType::Serial,
            address: String::new(),
            connect_timeout_ms: 5000,
            read_timeout_ms: 1000,
            write_timeout_ms: 1000,
            auto_reconnect: true,
            max_reconnect_attempts: 3,
            reconnect_delay_ms: 1000,
            read_buffer_size: 4096,
            write_buffer_size: 4096,
            require_handshake: false,
            min_latency: None,
            settings: TransportSettings::Serial(SerialSettings::default()),
        }
    }
}

/// Transport-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransportSettings {
    Serial(SerialSettings),
    Tcp(TcpSettings),
    Udp(UdpSettings),
    Ssh(SshSettings),
}

/// Serial port settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialSettings {
    pub baud_rate: u32,
    pub data_bits: DataBits,
    pub stop_bits: StopBits,
    pub parity: Parity,
    pub flow_control: FlowControl,
}

impl Default for SerialSettings {
    fn default() -> Self {
        SerialSettings {
            baud_rate: 115200,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
            flow_control: FlowControl::None,
        }
    }
}

/// Data bits configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataBits {
    Five,
    Six,
    Seven,
    Eight,
}

/// Stop bits configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopBits {
    One,
    Two,
}

/// Parity configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Parity {
    None,
    Odd,
    Even,
}

/// Flow control configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FlowControl {
    None,
    Software,
    Hardware,
}

/// TCP settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpSettings {
    pub host: String,
    pub port: u16,
    pub no_delay: bool,
    pub keep_alive: bool,
    pub keep_alive_interval_ms: u32,
}

impl Default for TcpSettings {
    fn default() -> Self {
        TcpSettings {
            host: "localhost".to_string(),
            port: 8080,
            no_delay: true, // Disable Nagle's algorithm for low latency
            keep_alive: true,
            keep_alive_interval_ms: 10000,
        }
    }
}

/// UDP settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UdpSettings {
    pub host: String,
    pub port: u16,
    pub bind_port: u16,  // Local port to bind to (0 = any)
    pub broadcast: bool,
    pub multicast: bool,
    pub mtu: usize,  // Maximum transmission unit
    pub accept_any_source: bool,  // Accept datagrams from any source
}

impl Default for UdpSettings {
    fn default() -> Self {
        UdpSettings {
            host: "localhost".to_string(),
            port: 8080,
            bind_port: 0,  // Use any available port
            broadcast: false,
            multicast: false,
            mtu: 1472,  // Standard MTU for UDP over Ethernet
            accept_any_source: false,
        }
    }
}

/// SSH settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshSettings {
    pub username: String,
    pub key_path: Option<String>,
    pub password: Option<String>,  // Fallback to password auth if no key
    pub port: u16,
    pub compression: bool,
    pub strict_host_key_checking: bool,  // For known_hosts verification
    pub known_hosts_path: Option<String>,  // Custom known_hosts file
    pub key_passphrase: Option<String>,  // For encrypted private keys
}

impl Default for SshSettings {
    fn default() -> Self {
        SshSettings {
            username: "pi".to_string(),
            key_path: None,
            password: None,
            port: 22,
            compression: false,
            strict_host_key_checking: false,  // Relaxed for personal use
            known_hosts_path: None,
            key_passphrase: None,
        }
    }
}

