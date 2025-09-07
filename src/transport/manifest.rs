use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use tokio::fs;
use crate::transport::{
    TransportType, TransportConfig, TransportResult, TransportError,
    Transport, TransportFactory
};
use crate::transport::common::{
    TransportSettings, SerialSettings, TcpSettings, UdpSettings, SshSettings
};
use std::sync::Arc;

/// Transport manifest - the "menu" of all available transports
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportManifest {
    /// Version of the manifest format
    pub version: String,
    
    /// List of all configured transports
    pub transports: Vec<TransportEntry>,
    
    /// Auto-discovery settings
    #[serde(default)]
    pub discovery: DiscoveryConfig,
    
    /// Global defaults
    #[serde(default)]
    pub defaults: ManifestDefaults,
}

/// Individual transport entry in the manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportEntry {
    /// Unique identifier for this transport
    pub id: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Device type (arduino, esp32, raspberry_pi, etc.)
    pub device_type: String,
    
    /// Transport type (serial, tcp, udp, ssh)
    pub transport_type: TransportType,
    
    /// Connection details
    pub connection: ConnectionDetails,
    
    /// What this transport can do
    pub capabilities: Vec<String>,
    
    /// Priority (higher = preferred)
    #[serde(default)]
    pub priority: u8,
    
    /// Is this transport currently enabled?
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Fallback transport ID if this one fails
    pub fallback: Option<String>,
    
    /// Performance requirements
    #[serde(default)]
    pub performance: PerformanceRequirements,
}

/// Connection details for a transport
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionDetails {
    Serial {
        /// Port name or pattern (e.g., "COM3", "/dev/ttyUSB*")
        port: String,
        baud_rate: u32,
        #[serde(default)]
        auto_detect: bool,
    },
    Tcp {
        host: String,
        port: u16,
        #[serde(default)]
        mdns_service: Option<String>,  // For auto-discovery
    },
    Udp {
        host: String,
        port: u16,
        #[serde(default)]
        broadcast: bool,
        #[serde(default)]
        bind_port: u16,
    },
    Ssh {
        host: String,
        port: u16,
        username: String,
        auth: SshAuthConfig,
    },
}

/// SSH authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum SshAuthConfig {
    Password { password: String },
    Key { path: String, passphrase: Option<String> },
    Agent,
}

/// Auto-discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscoveryConfig {
    /// Enable auto-discovery
    #[serde(default)]
    pub enabled: bool,
    
    /// Scan for serial ports
    #[serde(default)]
    pub serial_scan: bool,
    
    /// mDNS discovery for network devices
    #[serde(default)]
    pub mdns: bool,
    
    /// UDP broadcast discovery
    #[serde(default)]
    pub broadcast: bool,
    
    /// Discovery timeout in milliseconds
    #[serde(default = "default_discovery_timeout")]
    pub timeout_ms: u64,
}

/// Global defaults for the manifest
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManifestDefaults {
    /// Default reconnect attempts
    #[serde(default = "default_reconnect_attempts")]
    pub max_reconnect_attempts: u32,
    
    /// Default reconnect delay
    #[serde(default = "default_reconnect_delay")]
    pub reconnect_delay_ms: u32,
    
    /// Auto-reconnect by default
    #[serde(default = "default_true")]
    pub auto_reconnect: bool,
    
    /// Default read timeout
    #[serde(default = "default_read_timeout")]
    pub read_timeout_ms: u32,
}

/// Performance requirements for a transport
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceRequirements {
    /// Maximum latency in milliseconds
    pub max_latency_ms: Option<u32>,
    
    /// Minimum throughput in bytes/sec
    pub min_throughput: Option<u64>,
    
    /// Required reliability (0.0 - 1.0)
    pub reliability: Option<f32>,
}

/// Transport manifest manager - handles loading, saving, and discovery
pub struct ManifestManager {
    manifest: TransportManifest,
    manifest_path: PathBuf,
    discovered_transports: HashMap<String, TransportEntry>,
}

impl ManifestManager {
    /// Load manifest from file
    pub async fn load(path: impl AsRef<Path>) -> TransportResult<Self> {
        let path = path.as_ref().to_path_buf();
        
        let content = fs::read_to_string(&path)
            .await
            .map_err(|e| TransportError::ConfigError(format!("Failed to read manifest: {}", e)))?;
        
        let manifest: TransportManifest = toml::from_str(&content)
            .map_err(|e| TransportError::ConfigError(format!("Invalid manifest format: {}", e)))?;
        
        Ok(ManifestManager {
            manifest,
            manifest_path: path,
            discovered_transports: HashMap::new(),
        })
    }
    
    /// Create a default manifest
    pub fn create_default() -> Self {
        let manifest = TransportManifest {
            version: "1.0".to_string(),
            transports: vec![
                // Default Arduino on serial
                TransportEntry {
                    id: "arduino_primary".to_string(),
                    name: "Arduino Uno (Primary)".to_string(),
                    device_type: "arduino_uno".to_string(),
                    transport_type: TransportType::Serial,
                    connection: ConnectionDetails::Serial {
                        port: "COM3".to_string(),
                        baud_rate: 115200,
                        auto_detect: true,
                    },
                    capabilities: vec![
                        "digital_io".to_string(),
                        "analog_read".to_string(),
                        "pwm".to_string(),
                        "hall_sensor".to_string(),
                        "imu".to_string(),
                        "temperature".to_string(),
                    ],
                    priority: 10,
                    enabled: true,
                    fallback: Some("esp32_wifi".to_string()),
                    performance: PerformanceRequirements {
                        max_latency_ms: Some(50),
                        min_throughput: None,
                        reliability: Some(0.99),
                    },
                },
                // ESP32 over WiFi
                TransportEntry {
                    id: "esp32_wifi".to_string(),
                    name: "ESP32 (WiFi)".to_string(),
                    device_type: "esp32".to_string(),
                    transport_type: TransportType::Tcp,
                    connection: ConnectionDetails::Tcp {
                        host: "192.168.1.100".to_string(),
                        port: 8080,
                        mdns_service: Some("_esp32._tcp.local".to_string()),
                    },
                    capabilities: vec![
                        "wifi".to_string(),
                        "bluetooth".to_string(),
                        "digital_io".to_string(),
                        "analog_read".to_string(),
                        "pwm".to_string(),
                        "sensors".to_string(),
                    ],
                    priority: 8,
                    enabled: true,
                    fallback: None,
                    performance: PerformanceRequirements {
                        max_latency_ms: Some(100),
                        min_throughput: Some(10000),
                        reliability: Some(0.95),
                    },
                },
            ],
            discovery: DiscoveryConfig {
                enabled: true,
                serial_scan: true,
                mdns: true,
                broadcast: false,
                timeout_ms: 5000,
            },
            defaults: ManifestDefaults::default(),
        };
        
        ManifestManager {
            manifest,
            manifest_path: PathBuf::from("transport-manifest.toml"),
            discovered_transports: HashMap::new(),
        }
    }
    
    /// Save manifest to file
    pub async fn save(&self) -> TransportResult<()> {
        let content = toml::to_string_pretty(&self.manifest)
            .map_err(|e| TransportError::ConfigError(format!("Failed to serialize manifest: {}", e)))?;
        
        fs::write(&self.manifest_path, content)
            .await
            .map_err(|e| TransportError::ConfigError(format!("Failed to write manifest: {}", e)))?;
        
        Ok(())
    }
    
    /// Get all enabled transports sorted by priority
    pub fn get_enabled_transports(&self) -> Vec<&TransportEntry> {
        let mut transports: Vec<&TransportEntry> = self.manifest.transports
            .iter()
            .filter(|t| t.enabled)
            .collect();
        
        transports.sort_by(|a, b| b.priority.cmp(&a.priority));
        transports
    }
    
    /// Find transport by ID
    pub fn get_transport(&self, id: &str) -> Option<&TransportEntry> {
        self.manifest.transports
            .iter()
            .find(|t| t.id == id)
    }
    
    /// Create TransportConfig from manifest entry
    pub fn create_config(&self, entry: &TransportEntry) -> TransportResult<TransportConfig> {
        let settings = match &entry.connection {
            ConnectionDetails::Serial { port, baud_rate, .. } => {
                TransportSettings::Serial(SerialSettings {
                    baud_rate: *baud_rate,
                    ..Default::default()
                })
            }
            ConnectionDetails::Tcp { host, port, .. } => {
                TransportSettings::Tcp(TcpSettings {
                    host: host.clone(),
                    port: *port,
                    ..Default::default()
                })
            }
            ConnectionDetails::Udp { host, port, broadcast, bind_port } => {
                TransportSettings::Udp(UdpSettings {
                    host: host.clone(),
                    port: *port,
                    broadcast: *broadcast,
                    bind_port: *bind_port,
                    ..Default::default()
                })
            }
            ConnectionDetails::Ssh { host: _, port, username, auth } => {
                let key_path = match auth {
                    SshAuthConfig::Key { path, .. } => Some(path.clone()),
                    _ => None,
                };
                
                TransportSettings::Ssh(SshSettings {
                    username: username.clone(),
                    key_path,
                    password: None,  // Manifest doesn't store passwords
                    port: *port,
                    compression: false,
                    strict_host_key_checking: false,
                    known_hosts_path: None,
                    key_passphrase: None,  // Manifest doesn't store passphrases
                })
            }
        };
        
        let address = match &entry.connection {
            ConnectionDetails::Serial { port, .. } => port.clone(),
            ConnectionDetails::Tcp { host, port, .. } => format!("{}:{}", host, port),
            ConnectionDetails::Udp { host, port, .. } => format!("{}:{}", host, port),
            ConnectionDetails::Ssh { host, port, .. } => format!("{}:{}", host, port),
        };
        
        Ok(TransportConfig {
            transport_type: entry.transport_type,
            address,
            settings,
            auto_reconnect: self.manifest.defaults.auto_reconnect,
            max_reconnect_attempts: self.manifest.defaults.max_reconnect_attempts,
            reconnect_delay_ms: self.manifest.defaults.reconnect_delay_ms,
            read_timeout_ms: self.manifest.defaults.read_timeout_ms,
            ..Default::default()
        })
    }
    
    /// Auto-discover transports
    pub async fn discover(&mut self) -> TransportResult<Vec<TransportEntry>> {
        let mut discovered = Vec::new();
        
        if !self.manifest.discovery.enabled {
            return Ok(discovered);
        }
        
        // Discover serial ports
        if self.manifest.discovery.serial_scan {
            if let Ok(ports) = crate::transport::serial::SerialTransport::list_ports() {
                for port in ports {
                    let entry = TransportEntry {
                        id: format!("discovered_serial_{}", port.replace("/", "_").replace("\\", "_")),
                        name: format!("Serial Device ({})", port),
                        device_type: "unknown".to_string(),
                        transport_type: TransportType::Serial,
                        connection: ConnectionDetails::Serial {
                            port: port.clone(),
                            baud_rate: 115200,
                            auto_detect: true,
                        },
                        capabilities: vec!["unknown".to_string()],
                        priority: 5,
                        enabled: false, // Discovered but not enabled by default
                        fallback: None,
                        performance: PerformanceRequirements::default(),
                    };
                    
                    discovered.push(entry.clone());
                    self.discovered_transports.insert(entry.id.clone(), entry);
                }
            }
        }
        
        // TODO: Implement mDNS discovery
        // TODO: Implement broadcast discovery
        
        Ok(discovered)
    }
    
    /// Connect to the best available transport
    pub async fn connect_best(&self) -> TransportResult<Box<dyn Transport>> {
        let transports = self.get_enabled_transports();
        
        for entry in transports {
            tracing::info!("Trying transport: {} ({})", entry.name, entry.id);
            
            let config = self.create_config(entry)?;
            match TransportFactory::create(config).await {
                Ok(mut transport) => {
                    match transport.connect().await {
                        Ok(()) => {
                            tracing::info!("Successfully connected to: {}", entry.name);
                            return Ok(transport);
                        }
                        Err(e) => {
                            tracing::warn!("Failed to connect to {}: {}", entry.name, e);
                            
                            // Try fallback if available
                            if let Some(fallback_id) = &entry.fallback {
                                if let Some(fallback) = self.get_transport(fallback_id) {
                                    tracing::info!("Trying fallback: {}", fallback.name);
                                    let fallback_config = self.create_config(fallback)?;
                                    if let Ok(mut transport) = TransportFactory::create(fallback_config).await {
                                        if transport.connect().await.is_ok() {
                                            tracing::info!("Connected to fallback: {}", fallback.name);
                                            return Ok(transport);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to create transport {}: {}", entry.name, e);
                }
            }
        }
        
        Err(TransportError::ConnectionFailed("No transports available".into()))
    }
}

// Default value functions for serde
fn default_true() -> bool { true }
fn default_discovery_timeout() -> u64 { 5000 }
fn default_reconnect_attempts() -> u32 { 3 }
fn default_reconnect_delay() -> u32 { 1000 }
fn default_read_timeout() -> u32 { 1000 }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_default_manifest() {
        let manager = ManifestManager::create_default();
        assert_eq!(manager.manifest.version, "1.0");
        assert_eq!(manager.manifest.transports.len(), 2);
        
        // Check Arduino entry
        let arduino = &manager.manifest.transports[0];
        assert_eq!(arduino.id, "arduino_primary");
        assert_eq!(arduino.device_type, "arduino_uno");
        assert_eq!(arduino.priority, 10);
        
        // Check ESP32 entry
        let esp32 = &manager.manifest.transports[1];
        assert_eq!(esp32.id, "esp32_wifi");
        assert_eq!(esp32.device_type, "esp32");
        assert_eq!(esp32.priority, 8);
    }
    
    #[test]
    fn test_get_enabled_transports() {
        let manager = ManifestManager::create_default();
        let enabled = manager.get_enabled_transports();
        
        assert_eq!(enabled.len(), 2);
        // Should be sorted by priority (Arduino first)
        assert_eq!(enabled[0].id, "arduino_primary");
        assert_eq!(enabled[1].id, "esp32_wifi");
    }
    
    #[test]
    fn test_create_config_from_entry() {
        let manager = ManifestManager::create_default();
        let arduino = &manager.manifest.transports[0];
        
        let config = manager.create_config(arduino).unwrap();
        assert_eq!(config.transport_type, TransportType::Serial);
        assert_eq!(config.address, "COM3");
        
        match config.settings {
            TransportSettings::Serial(settings) => {
                assert_eq!(settings.baud_rate, 115200);
            }
            _ => panic!("Expected serial settings"),
        }
    }
}