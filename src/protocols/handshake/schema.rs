//! Handshake Protocol Message Schema
//! 
//! This module defines the complete JSON schema for the handshake protocol,
//! supporting device identification, capability negotiation, and version compatibility.
//! 
//! # Protocol Overview
//! 
//! The handshake protocol consists of four main message types:
//! 
//! 1. **IDENTIFY** - Initial device identification request
//! 2. **CAPABILITIES** - Device capability advertisement 
//! 3. **VERSION** - Protocol version negotiation
//! 4. **ERROR** - Error reporting and diagnostics
//! 
//! # Schema Evolution Strategy
//! 
//! - Uses semantic versioning (major.minor.patch)
//! - Backward compatibility maintained within major versions
//! - Optional fields added for extensions without breaking changes
//! - Schema validation enforces required fields and constraints
//! 
//! # Usage Example
//! 
//! ```rust
//! use crate::protocols::handshake::schema::*;
//! 
//! // Create IDENTIFY command
//! let identify = IdentifyCommand {
//!     command: "IDENTIFY".to_string(),
//!     protocol_version: "1.0.0".to_string(),
//!     session_id: uuid::Uuid::new_v4(),
//!     capabilities_requested: vec!["basic_io".to_string(), "telemetry".to_string()],
//!     timestamp: Some(chrono::Utc::now().to_rfc3339()),
//!     client_info: Some(ClientInfo {
//!         name: "Multi-Controller App".to_string(),
//!         version: "0.1.0".to_string(),
//!         platform: "Windows".to_string(),
//!     }),
//! };
//! 
//! let json = serde_json::to_string(&identify)?;
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Protocol version used for this schema implementation
pub const PROTOCOL_VERSION: &str = "1.0.0";

/// Maximum length for string fields to prevent DoS attacks
pub const MAX_STRING_LENGTH: usize = 1024;

/// Maximum number of capabilities that can be requested/advertised
pub const MAX_CAPABILITIES: usize = 100;

/// Maximum size for custom parameters
pub const MAX_PARAMETERS: usize = 50;

// ============================================================================
// Core Message Types
// ============================================================================

/// Base trait for all handshake messages
/// Provides common validation and serialization functionality
pub trait HandshakeMessage: Serialize + for<'de> Deserialize<'de> + Clone + std::fmt::Debug {
    /// Validate the message according to schema rules
    fn validate(&self) -> Result<(), ValidationError>;
    
    /// Get the message type identifier
    fn message_type(&self) -> &str;
    
    /// Get the protocol version this message supports
    fn protocol_version(&self) -> &str;
}

// ============================================================================
// IDENTIFY Command and Response
// ============================================================================

/// IDENTIFY command sent from client to device to initiate handshake
/// 
/// This is the first message in the handshake protocol. The client sends
/// this to identify itself and request specific capabilities from the device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdentifyCommand {
    /// Command type - always "IDENTIFY"
    pub command: String,
    
    /// Protocol version the client supports (semantic version)
    pub protocol_version: String,
    
    /// Unique session identifier for this handshake attempt
    pub session_id: Uuid,
    
    /// List of capabilities the client wants to use
    /// Examples: ["basic_io", "telemetry", "scripting", "file_transfer"]
    pub capabilities_requested: Vec<String>,
    
    /// Optional ISO 8601 timestamp when message was created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    
    /// Optional client information for debugging and logging
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_info: Option<ClientInfo>,
    
    /// Optional authentication token or credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    
    /// Optional custom parameters for extended functionality
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub custom_params: HashMap<String, serde_json::Value>,
}

impl HandshakeMessage for IdentifyCommand {
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate command type
        if self.command != "IDENTIFY" {
            return Err(ValidationError::InvalidCommand {
                expected: "IDENTIFY".to_string(),
                actual: self.command.clone(),
            });
        }
        
        // Validate protocol version format
        validate_semver(&self.protocol_version)?;
        
        // Validate capabilities list
        if self.capabilities_requested.len() > MAX_CAPABILITIES {
            return Err(ValidationError::TooManyCapabilities {
                count: self.capabilities_requested.len(),
                max: MAX_CAPABILITIES,
            });
        }
        
        for capability in &self.capabilities_requested {
            validate_string_length(capability, "capability")?;
        }
        
        // Validate custom parameters count
        if self.custom_params.len() > MAX_PARAMETERS {
            return Err(ValidationError::TooManyParameters {
                count: self.custom_params.len(),
                max: MAX_PARAMETERS,
            });
        }
        
        // Validate client info if present
        if let Some(ref client_info) = self.client_info {
            client_info.validate()?;
        }
        
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "IDENTIFY"
    }
    
    fn protocol_version(&self) -> &str {
        &self.protocol_version
    }
}

/// IDENTIFY response sent from device to client
/// 
/// The device responds with its identification, capabilities, and whether
/// it accepts the requested session.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdentifyResponse {
    /// Response status - "OK" for success, "ERROR" for failure
    pub status: String,
    
    /// Unique device identifier (persistent across reboots)
    pub device_id: String,
    
    /// Device type identifier (e.g., "Arduino_Uno", "ESP32", "Custom")
    pub device_type: String,
    
    /// Device firmware version (semantic version)
    pub firmware_version: String,
    
    /// Protocol version the device supports
    pub protocol_version: String,
    
    /// Capabilities available on this device
    pub capabilities: Vec<Capability>,
    
    /// Whether the session was accepted
    pub session_accepted: bool,
    
    /// Optional error message if status is "ERROR" or session not accepted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    
    /// Optional error code for programmatic handling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    
    /// Optional device information for debugging and display
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_info: Option<DeviceInfo>,
    
    /// Optional timestamp when response was generated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    
    /// Session ID from the original request (for correlation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,
    
    /// Optional custom parameters for extended functionality
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub custom_params: HashMap<String, serde_json::Value>,
}

impl HandshakeMessage for IdentifyResponse {
    fn validate(&self) -> Result<(), ValidationError> {
        // Validate status
        if !matches!(self.status.as_str(), "OK" | "ERROR") {
            return Err(ValidationError::InvalidStatus {
                status: self.status.clone(),
            });
        }
        
        // Validate required fields
        validate_string_length(&self.device_id, "device_id")?;
        validate_string_length(&self.device_type, "device_type")?;
        validate_semver(&self.firmware_version)?;
        validate_semver(&self.protocol_version)?;
        
        // Validate capabilities
        if self.capabilities.len() > MAX_CAPABILITIES {
            return Err(ValidationError::TooManyCapabilities {
                count: self.capabilities.len(),
                max: MAX_CAPABILITIES,
            });
        }
        
        for capability in &self.capabilities {
            capability.validate()?;
        }
        
        // If status is ERROR, error_message should be present
        if self.status == "ERROR" && self.error_message.is_none() {
            return Err(ValidationError::MissingErrorMessage);
        }
        
        // Validate device info if present
        if let Some(ref device_info) = self.device_info {
            device_info.validate()?;
        }
        
        // Validate custom parameters count
        if self.custom_params.len() > MAX_PARAMETERS {
            return Err(ValidationError::TooManyParameters {
                count: self.custom_params.len(),
                max: MAX_PARAMETERS,
            });
        }
        
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "IDENTIFY_RESPONSE"
    }
    
    fn protocol_version(&self) -> &str {
        &self.protocol_version
    }
}

// ============================================================================
// CAPABILITIES Messages
// ============================================================================

/// CAPABILITIES request to query specific capability details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapabilitiesRequest {
    /// Command type - always "CAPABILITIES"
    pub command: String,
    
    /// Session ID for correlation
    pub session_id: Uuid,
    
    /// List of specific capabilities to query (empty = query all)
    pub capabilities_requested: Vec<String>,
    
    /// Optional timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl HandshakeMessage for CapabilitiesRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.command != "CAPABILITIES" {
            return Err(ValidationError::InvalidCommand {
                expected: "CAPABILITIES".to_string(),
                actual: self.command.clone(),
            });
        }
        
        if self.capabilities_requested.len() > MAX_CAPABILITIES {
            return Err(ValidationError::TooManyCapabilities {
                count: self.capabilities_requested.len(),
                max: MAX_CAPABILITIES,
            });
        }
        
        for capability in &self.capabilities_requested {
            validate_string_length(capability, "capability")?;
        }
        
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "CAPABILITIES"
    }
    
    fn protocol_version(&self) -> &str {
        PROTOCOL_VERSION
    }
}

/// CAPABILITIES response with detailed capability information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapabilitiesResponse {
    /// Response status
    pub status: String,
    
    /// Session ID for correlation
    pub session_id: Uuid,
    
    /// Detailed capability information
    pub capabilities: Vec<Capability>,
    
    /// Optional error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    
    /// Optional timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl HandshakeMessage for CapabilitiesResponse {
    fn validate(&self) -> Result<(), ValidationError> {
        if !matches!(self.status.as_str(), "OK" | "ERROR") {
            return Err(ValidationError::InvalidStatus {
                status: self.status.clone(),
            });
        }
        
        if self.capabilities.len() > MAX_CAPABILITIES {
            return Err(ValidationError::TooManyCapabilities {
                count: self.capabilities.len(),
                max: MAX_CAPABILITIES,
            });
        }
        
        for capability in &self.capabilities {
            capability.validate()?;
        }
        
        if self.status == "ERROR" && self.error_message.is_none() {
            return Err(ValidationError::MissingErrorMessage);
        }
        
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "CAPABILITIES_RESPONSE"
    }
    
    fn protocol_version(&self) -> &str {
        PROTOCOL_VERSION
    }
}

// ============================================================================
// VERSION Negotiation Messages
// ============================================================================

/// VERSION request to negotiate protocol version
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionRequest {
    /// Command type - always "VERSION"
    pub command: String,
    
    /// Session ID for correlation
    pub session_id: Uuid,
    
    /// Preferred protocol version
    pub preferred_version: String,
    
    /// List of supported protocol versions (in preference order)
    pub supported_versions: Vec<String>,
    
    /// Optional timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl HandshakeMessage for VersionRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.command != "VERSION" {
            return Err(ValidationError::InvalidCommand {
                expected: "VERSION".to_string(),
                actual: self.command.clone(),
            });
        }
        
        validate_semver(&self.preferred_version)?;
        
        if self.supported_versions.is_empty() {
            return Err(ValidationError::EmptySupportedVersions);
        }
        
        for version in &self.supported_versions {
            validate_semver(version)?;
        }
        
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "VERSION"
    }
    
    fn protocol_version(&self) -> &str {
        &self.preferred_version
    }
}

/// VERSION response with negotiated protocol version
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionResponse {
    /// Response status
    pub status: String,
    
    /// Session ID for correlation
    pub session_id: Uuid,
    
    /// Negotiated protocol version (must be mutually supported)
    pub negotiated_version: String,
    
    /// List of versions the device supports
    pub supported_versions: Vec<String>,
    
    /// Optional error message if negotiation failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    
    /// Optional timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl HandshakeMessage for VersionResponse {
    fn validate(&self) -> Result<(), ValidationError> {
        if !matches!(self.status.as_str(), "OK" | "ERROR") {
            return Err(ValidationError::InvalidStatus {
                status: self.status.clone(),
            });
        }
        
        if self.status == "OK" {
            validate_semver(&self.negotiated_version)?;
        }
        
        if self.supported_versions.is_empty() {
            return Err(ValidationError::EmptySupportedVersions);
        }
        
        for version in &self.supported_versions {
            validate_semver(version)?;
        }
        
        if self.status == "ERROR" && self.error_message.is_none() {
            return Err(ValidationError::MissingErrorMessage);
        }
        
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "VERSION_RESPONSE"
    }
    
    fn protocol_version(&self) -> &str {
        &self.negotiated_version
    }
}

// ============================================================================
// ERROR Messages
// ============================================================================

/// Generic ERROR message for reporting issues during handshake
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorMessage {
    /// Message type - always "ERROR"
    pub message_type: String,
    
    /// Session ID for correlation (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,
    
    /// Error severity level
    pub severity: ErrorSeverity,
    
    /// Error category for programmatic handling
    pub category: ErrorCategory,
    
    /// Human-readable error message
    pub message: String,
    
    /// Optional error code for specific error types
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    
    /// Optional detailed error information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<HashMap<String, serde_json::Value>>,
    
    /// Optional timestamp when error occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    
    /// Whether the handshake can be retried
    pub retryable: bool,
}

impl HandshakeMessage for ErrorMessage {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.message_type != "ERROR" {
            return Err(ValidationError::InvalidCommand {
                expected: "ERROR".to_string(),
                actual: self.message_type.clone(),
            });
        }
        
        validate_string_length(&self.message, "error_message")?;
        
        if let Some(ref details) = self.details {
            if details.len() > MAX_PARAMETERS {
                return Err(ValidationError::TooManyParameters {
                    count: details.len(),
                    max: MAX_PARAMETERS,
                });
            }
        }
        
        Ok(())
    }
    
    fn message_type(&self) -> &str {
        "ERROR"
    }
    
    fn protocol_version(&self) -> &str {
        PROTOCOL_VERSION
    }
}

// ============================================================================
// Supporting Data Structures
// ============================================================================

/// Device capability description
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Capability {
    /// Capability name/identifier
    pub name: String,
    
    /// Capability version (semantic version)
    pub version: String,
    
    /// Human-readable description
    pub description: String,
    
    /// Whether this capability is enabled by default
    pub enabled_by_default: bool,
    
    /// Configuration parameters for this capability
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, serde_json::Value>,
    
    /// Dependencies on other capabilities
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
    
    /// Optional minimum required protocol version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_protocol_version: Option<String>,
}

impl Capability {
    fn validate(&self) -> Result<(), ValidationError> {
        validate_string_length(&self.name, "capability_name")?;
        validate_semver(&self.version)?;
        validate_string_length(&self.description, "capability_description")?;
        
        if self.parameters.len() > MAX_PARAMETERS {
            return Err(ValidationError::TooManyParameters {
                count: self.parameters.len(),
                max: MAX_PARAMETERS,
            });
        }
        
        for dependency in &self.dependencies {
            validate_string_length(dependency, "dependency")?;
        }
        
        if let Some(ref min_version) = self.min_protocol_version {
            validate_semver(min_version)?;
        }
        
        Ok(())
    }
}

/// Client information for debugging and logging
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientInfo {
    /// Client application name
    pub name: String,
    
    /// Client application version
    pub version: String,
    
    /// Client platform/OS
    pub platform: String,
    
    /// Optional additional client metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
}

impl ClientInfo {
    fn validate(&self) -> Result<(), ValidationError> {
        validate_string_length(&self.name, "client_name")?;
        validate_string_length(&self.version, "client_version")?;
        validate_string_length(&self.platform, "client_platform")?;
        
        if self.metadata.len() > MAX_PARAMETERS {
            return Err(ValidationError::TooManyParameters {
                count: self.metadata.len(),
                max: MAX_PARAMETERS,
            });
        }
        
        for (key, value) in &self.metadata {
            validate_string_length(key, "metadata_key")?;
            validate_string_length(value, "metadata_value")?;
        }
        
        Ok(())
    }
}

/// Device information for display and debugging
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceInfo {
    /// Device manufacturer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    
    /// Device model/product name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    
    /// Device serial number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    
    /// Hardware revision
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware_version: Option<String>,
    
    /// Available memory (bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_total: Option<u64>,
    
    /// Free memory (bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_free: Option<u64>,
    
    /// CPU/MCU type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_type: Option<String>,
    
    /// Clock speed (Hz)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_speed: Option<u64>,
    
    /// Additional device-specific metadata
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, serde_json::Value>,
}

impl DeviceInfo {
    fn validate(&self) -> Result<(), ValidationError> {
        if let Some(ref manufacturer) = self.manufacturer {
            validate_string_length(manufacturer, "manufacturer")?;
        }
        
        if let Some(ref model) = self.model {
            validate_string_length(model, "model")?;
        }
        
        if let Some(ref serial_number) = self.serial_number {
            validate_string_length(serial_number, "serial_number")?;
        }
        
        if let Some(ref hardware_version) = self.hardware_version {
            validate_string_length(hardware_version, "hardware_version")?;
        }
        
        if let Some(ref cpu_type) = self.cpu_type {
            validate_string_length(cpu_type, "cpu_type")?;
        }
        
        if self.metadata.len() > MAX_PARAMETERS {
            return Err(ValidationError::TooManyParameters {
                count: self.metadata.len(),
                max: MAX_PARAMETERS,
            });
        }
        
        Ok(())
    }
}

/// Error severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorSeverity {
    /// Informational message
    Info,
    /// Warning that doesn't prevent operation
    Warning,
    /// Error that prevents current operation
    Error,
    /// Critical error that may affect system stability
    Critical,
}

/// Error categories for programmatic handling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorCategory {
    /// Protocol parsing or validation error
    Protocol,
    /// Version compatibility error
    Version,
    /// Authentication or authorization error
    Authentication,
    /// Device capability error
    Capability,
    /// Session management error
    Session,
    /// Network or transport error
    Transport,
    /// Device hardware error
    Hardware,
    /// Configuration error
    Configuration,
    /// Unknown or unclassified error
    Unknown,
}

// ============================================================================
// Validation Support
// ============================================================================

/// Validation errors for schema validation
#[derive(Debug, thiserror::Error, Clone, PartialEq)]
pub enum ValidationError {
    #[error("Invalid command: expected '{expected}', got '{actual}'")]
    InvalidCommand { expected: String, actual: String },
    
    #[error("Invalid status: '{status}' (must be 'OK' or 'ERROR')")]
    InvalidStatus { status: String },
    
    #[error("Invalid semantic version: '{version}'")]
    InvalidSemver { version: String },
    
    #[error("String field '{field}' too long: {length} characters (max {max})")]
    StringTooLong { field: String, length: usize, max: usize },
    
    #[error("Too many capabilities: {count} (max {max})")]
    TooManyCapabilities { count: usize, max: usize },
    
    #[error("Too many parameters: {count} (max {max})")]
    TooManyParameters { count: usize, max: usize },
    
    #[error("Error message is required when status is 'ERROR'")]
    MissingErrorMessage,
    
    #[error("Supported versions list cannot be empty")]
    EmptySupportedVersions,
}

/// Validate a semantic version string
fn validate_semver(version: &str) -> Result<(), ValidationError> {
    // Basic semantic version validation (major.minor.patch)
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return Err(ValidationError::InvalidSemver {
            version: version.to_string(),
        });
    }
    
    for part in parts {
        if part.parse::<u32>().is_err() {
            return Err(ValidationError::InvalidSemver {
                version: version.to_string(),
            });
        }
    }
    
    Ok(())
}

/// Validate string length constraints
fn validate_string_length(value: &str, field_name: &str) -> Result<(), ValidationError> {
    if value.len() > MAX_STRING_LENGTH {
        return Err(ValidationError::StringTooLong {
            field: field_name.to_string(),
            length: value.len(),
            max: MAX_STRING_LENGTH,
        });
    }
    Ok(())
}

// ============================================================================
// Schema Examples and Utilities
// ============================================================================

/// Example message factory for testing and documentation
pub struct MessageExamples;

impl MessageExamples {
    /// Create example IDENTIFY command
    pub fn identify_command() -> IdentifyCommand {
        IdentifyCommand {
            command: "IDENTIFY".to_string(),
            protocol_version: "1.0.0".to_string(),
            session_id: Uuid::new_v4(),
            capabilities_requested: vec![
                "basic_io".to_string(),
                "telemetry".to_string(),
                "scripting".to_string(),
            ],
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            client_info: Some(ClientInfo {
                name: "Multi-Controller App".to_string(),
                version: "0.1.0".to_string(),
                platform: "Windows".to_string(),
                metadata: [
                    ("architecture".to_string(), "x86_64".to_string()),
                    ("build_type".to_string(), "release".to_string()),
                ].into_iter().collect(),
            }),
            auth_token: None,
            custom_params: HashMap::new(),
        }
    }
    
    /// Create example successful IDENTIFY response
    pub fn identify_response_success() -> IdentifyResponse {
        IdentifyResponse {
            status: "OK".to_string(),
            device_id: "arduino_uno_001".to_string(),
            device_type: "Arduino_Uno".to_string(),
            firmware_version: "2.1.0".to_string(),
            protocol_version: "1.0.0".to_string(),
            capabilities: vec![
                Capability {
                    name: "basic_io".to_string(),
                    version: "1.0.0".to_string(),
                    description: "Basic digital and analog I/O operations".to_string(),
                    enabled_by_default: true,
                    parameters: [
                        ("digital_pins".to_string(), serde_json::Value::Number(serde_json::Number::from(14))),
                        ("analog_pins".to_string(), serde_json::Value::Number(serde_json::Number::from(6))),
                    ].into_iter().collect(),
                    dependencies: vec![],
                    min_protocol_version: Some("1.0.0".to_string()),
                },
                Capability {
                    name: "telemetry".to_string(),
                    version: "1.2.0".to_string(),
                    description: "Real-time sensor data collection".to_string(),
                    enabled_by_default: false,
                    parameters: [
                        ("max_sample_rate".to_string(), serde_json::Value::Number(serde_json::Number::from(1000))),
                        ("buffer_size".to_string(), serde_json::Value::Number(serde_json::Number::from(512))),
                    ].into_iter().collect(),
                    dependencies: vec!["basic_io".to_string()],
                    min_protocol_version: Some("1.0.0".to_string()),
                },
            ],
            session_accepted: true,
            error_message: None,
            error_code: None,
            device_info: Some(DeviceInfo {
                manufacturer: Some("Arduino".to_string()),
                model: Some("Uno R3".to_string()),
                serial_number: Some("SN123456789".to_string()),
                hardware_version: Some("3.0".to_string()),
                memory_total: Some(32768),
                memory_free: Some(28672),
                cpu_type: Some("ATmega328P".to_string()),
                clock_speed: Some(16_000_000),
                metadata: HashMap::new(),
            }),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            session_id: Some(Uuid::new_v4()),
            custom_params: HashMap::new(),
        }
    }
    
    /// Create example error response
    pub fn identify_response_error() -> IdentifyResponse {
        IdentifyResponse {
            status: "ERROR".to_string(),
            device_id: "unknown".to_string(),
            device_type: "Unknown".to_string(),
            firmware_version: "0.0.0".to_string(),
            protocol_version: "1.0.0".to_string(),
            capabilities: vec![],
            session_accepted: false,
            error_message: Some("Incompatible protocol version".to_string()),
            error_code: Some("PROTOCOL_VERSION_MISMATCH".to_string()),
            device_info: None,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            session_id: Some(Uuid::new_v4()),
            custom_params: HashMap::new(),
        }
    }
    
    /// Create example error message
    pub fn error_message() -> ErrorMessage {
        ErrorMessage {
            message_type: "ERROR".to_string(),
            session_id: Some(Uuid::new_v4()),
            severity: ErrorSeverity::Error,
            category: ErrorCategory::Protocol,
            message: "Invalid JSON format in handshake message".to_string(),
            error_code: Some("JSON_PARSE_ERROR".to_string()),
            details: Some([
                ("line".to_string(), serde_json::Value::Number(serde_json::Number::from(15))),
                ("column".to_string(), serde_json::Value::Number(serde_json::Number::from(23))),
                ("expected".to_string(), serde_json::Value::String("'}'".to_string())),
            ].into_iter().collect()),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            retryable: true,
        }
    }
}

// ============================================================================
// JSON Schema Generation (for documentation)
// ============================================================================

/// Generate JSON Schema for documentation and validation tools
pub mod json_schema {
    use super::*;
    
    /// Generate JSON schema documentation for all message types
    pub fn generate_schema_docs() -> String {
        format!(
            r#"
# Handshake Protocol JSON Schema Documentation

## Protocol Version: {}

## Message Types

### 1. IDENTIFY Command
```json
{}
```

### 2. IDENTIFY Response (Success)
```json
{}
```

### 3. IDENTIFY Response (Error)
```json
{}
```

### 4. ERROR Message
```json
{}
```

## Schema Constraints

- Maximum string length: {} characters
- Maximum capabilities: {}
- Maximum custom parameters: {}
- Protocol version format: semantic version (major.minor.patch)
- All timestamps: ISO 8601 format (RFC 3339)
- Session IDs: UUID v4 format

## Backward Compatibility

This schema maintains backward compatibility within major versions:
- New optional fields may be added in minor versions
- Required fields are never removed in minor/patch versions
- Major version increments indicate breaking changes
- Devices should ignore unknown fields for forward compatibility
"#,
            PROTOCOL_VERSION,
            serde_json::to_string_pretty(&MessageExamples::identify_command()).unwrap(),
            serde_json::to_string_pretty(&MessageExamples::identify_response_success()).unwrap(),
            serde_json::to_string_pretty(&MessageExamples::identify_response_error()).unwrap(),
            serde_json::to_string_pretty(&MessageExamples::error_message()).unwrap(),
            MAX_STRING_LENGTH,
            MAX_CAPABILITIES,
            MAX_PARAMETERS,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_identify_command_validation() {
        let cmd = MessageExamples::identify_command();
        assert!(cmd.validate().is_ok());
        
        // Test invalid command
        let mut invalid_cmd = cmd.clone();
        invalid_cmd.command = "INVALID".to_string();
        assert!(invalid_cmd.validate().is_err());
        
        // Test too many capabilities
        let mut too_many_caps = cmd.clone();
        too_many_caps.capabilities_requested = (0..MAX_CAPABILITIES + 1)
            .map(|i| format!("cap_{}", i))
            .collect();
        assert!(too_many_caps.validate().is_err());
    }
    
    #[test]
    fn test_identify_response_validation() {
        let resp = MessageExamples::identify_response_success();
        assert!(resp.validate().is_ok());
        
        let error_resp = MessageExamples::identify_response_error();
        assert!(error_resp.validate().is_ok());
        
        // Test missing error message when status is ERROR
        let mut invalid_error = error_resp.clone();
        invalid_error.error_message = None;
        assert!(invalid_error.validate().is_err());
    }
    
    #[test]
    fn test_semver_validation() {
        assert!(validate_semver("1.0.0").is_ok());
        assert!(validate_semver("2.5.10").is_ok());
        assert!(validate_semver("0.1.0").is_ok());
        
        assert!(validate_semver("1.0").is_err());
        assert!(validate_semver("1.0.0.1").is_err());
        assert!(validate_semver("v1.0.0").is_err());
        assert!(validate_semver("1.0.0-beta").is_err());
        assert!(validate_semver("abc").is_err());
    }
    
    #[test]
    fn test_string_length_validation() {
        let short_string = "a".repeat(10);
        assert!(validate_string_length(&short_string, "test").is_ok());
        
        let long_string = "a".repeat(MAX_STRING_LENGTH + 1);
        assert!(validate_string_length(&long_string, "test").is_err());
    }
    
    #[test]
    fn test_message_serialization() {
        let cmd = MessageExamples::identify_command();
        let json = serde_json::to_string(&cmd).unwrap();
        let deserialized: IdentifyCommand = serde_json::from_str(&json).unwrap();
        assert_eq!(cmd, deserialized);
        
        let resp = MessageExamples::identify_response_success();
        let json = serde_json::to_string(&resp).unwrap();
        let deserialized: IdentifyResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, deserialized);
    }
    
    #[test]
    fn test_capability_validation() {
        let capability = Capability {
            name: "test_cap".to_string(),
            version: "1.0.0".to_string(),
            description: "Test capability".to_string(),
            enabled_by_default: true,
            parameters: HashMap::new(),
            dependencies: vec![],
            min_protocol_version: Some("1.0.0".to_string()),
        };
        
        assert!(capability.validate().is_ok());
        
        // Test invalid version
        let mut invalid_cap = capability.clone();
        invalid_cap.version = "invalid".to_string();
        assert!(invalid_cap.validate().is_err());
    }
    
    #[test] 
    fn test_error_message_validation() {
        let error = MessageExamples::error_message();
        assert!(error.validate().is_ok());
        
        // Test invalid message type
        let mut invalid_error = error.clone();
        invalid_error.message_type = "INVALID".to_string();
        assert!(invalid_error.validate().is_err());
    }
}