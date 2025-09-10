//! Handshake Protocol Implementation
//! 
//! This module implements the complete handshake protocol for device communication,
//! including message schema, state machine, timeout handling, and compatibility validation.
//! 
//! # Overview
//! 
//! The handshake protocol provides a robust, extensible mechanism for:
//! - Device identification and authentication
//! - Capability negotiation and discovery
//! - Protocol version compatibility
//! - Session establishment and management
//! - Comprehensive error handling and recovery
//! 
//! # Components
//! 
//! - `schema` - Complete JSON message schema with validation
//! - Future: `state_machine` - Handshake state management (Task 28.2)
//! - Future: `timeout` - Timeout enforcement and retry logic (Task 28.3)  
//! - Future: `compatibility` - Version compatibility checking (Task 28.4)
//! - Future: `feedback` - User feedback and status reporting (Task 28.5)

pub mod schema;

// Re-export commonly used types for convenience
pub use schema::{
    HandshakeMessage,
    IdentifyCommand,
    IdentifyResponse,
    CapabilitiesRequest,
    CapabilitiesResponse,
    VersionRequest,
    VersionResponse,
    ErrorMessage,
    Capability,
    ClientInfo,
    DeviceInfo,
    ErrorSeverity,
    ErrorCategory,
    ValidationError,
    MessageExamples,
    PROTOCOL_VERSION,
    MAX_STRING_LENGTH,
    MAX_CAPABILITIES,
    MAX_PARAMETERS,
};

/// Handshake protocol result type
pub type HandshakeResult<T> = Result<T, HandshakeError>;

/// Comprehensive handshake error types
#[derive(Debug, thiserror::Error, Clone)]
pub enum HandshakeError {
    /// Message validation failed
    #[error("Message validation failed: {0}")]
    Validation(#[from] ValidationError),
    
    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(String),
    
    /// Transport layer error during handshake
    #[error("Transport error: {0}")]
    Transport(String),
    
    /// Handshake timeout (5 seconds)
    #[error("Handshake timed out")]
    Timeout,
    
    /// Protocol version incompatibility
    #[error("Incompatible protocol version: device {device_version}, client supports {client_versions:?}")]
    IncompatibleProtocol {
        device_version: String,
        client_versions: Vec<String>,
    },
    
    /// Device firmware incompatibility
    #[error("Incompatible firmware: {device_type} v{device_version}, minimum required: {minimum_required}")]
    IncompatibleFirmware {
        device_type: String,
        device_version: String,
        minimum_required: String,
    },
    
    /// Device rejected the handshake
    #[error("Device rejected handshake: {reason}")]
    DeviceRejection { reason: String },
    
    /// Required capability not available
    #[error("Missing required capability: {capability}")]
    MissingCapability { capability: String },
    
    /// Malformed device response
    #[error("Malformed device response: {details}")]
    MalformedResponse { details: String },
    
    /// Session management error
    #[error("Session error: {message}")]
    Session { message: String },
}

impl HandshakeError {
    /// Create a JSON error from serde_json::Error
    pub fn from_json_error(err: serde_json::Error) -> Self {
        Self::Json(err.to_string())
    }
    
    /// Create a transport error
    pub fn transport<E: std::fmt::Display>(err: E) -> Self {
        Self::Transport(err.to_string())
    }
    
    /// Create a malformed response error
    pub fn malformed_response<S: Into<String>>(details: S) -> Self {
        Self::MalformedResponse {
            details: details.into(),
        }
    }
    
    /// Check if this error is recoverable (handshake can be retried)
    pub fn is_recoverable(&self) -> bool {
        match self {
            HandshakeError::Timeout => true,
            HandshakeError::Transport(_) => true,
            HandshakeError::MalformedResponse { .. } => true,
            HandshakeError::Session { .. } => true,
            HandshakeError::Json(_) => false,  // Protocol error
            HandshakeError::Validation(_) => false,  // Protocol error
            HandshakeError::IncompatibleProtocol { .. } => false,  // Version mismatch
            HandshakeError::IncompatibleFirmware { .. } => false,  // Version mismatch
            HandshakeError::DeviceRejection { .. } => false,  // Device decision
            HandshakeError::MissingCapability { .. } => false,  // Capability mismatch
        }
    }
    
    /// Get user-friendly error message for display
    pub fn user_friendly_message(&self) -> String {
        match self {
            HandshakeError::Timeout => {
                "Device did not respond within 5 seconds. Please check the connection and try again.".to_string()
            }
            HandshakeError::IncompatibleProtocol { device_version, .. } => {
                format!("Device protocol version {} is not supported. Please update the device firmware or use a compatible client version.", device_version)
            }
            HandshakeError::IncompatibleFirmware { device_type, device_version, minimum_required } => {
                format!("{} firmware v{} is too old. Minimum required version: {}", device_type, device_version, minimum_required)
            }
            HandshakeError::DeviceRejection { reason } => {
                format!("Device refused connection: {}", reason)
            }
            HandshakeError::MissingCapability { capability } => {
                format!("Device does not support required capability: {}", capability)
            }
            HandshakeError::Transport(msg) => {
                format!("Connection error: {}", msg)
            }
            HandshakeError::Session { message } => {
                format!("Session error: {}", message)
            }
            _ => {
                format!("Handshake failed: {}", self)
            }
        }
    }
    
    /// Get error category for programmatic handling
    pub fn category(&self) -> ErrorCategory {
        match self {
            HandshakeError::Validation(_) => ErrorCategory::Protocol,
            HandshakeError::Json(_) => ErrorCategory::Protocol,
            HandshakeError::Transport(_) => ErrorCategory::Transport,
            HandshakeError::Timeout => ErrorCategory::Transport,
            HandshakeError::IncompatibleProtocol { .. } => ErrorCategory::Version,
            HandshakeError::IncompatibleFirmware { .. } => ErrorCategory::Version,
            HandshakeError::DeviceRejection { .. } => ErrorCategory::Authentication,
            HandshakeError::MissingCapability { .. } => ErrorCategory::Capability,
            HandshakeError::MalformedResponse { .. } => ErrorCategory::Protocol,
            HandshakeError::Session { .. } => ErrorCategory::Session,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_handshake_error_recoverable() {
        assert!(HandshakeError::Timeout.is_recoverable());
        assert!(HandshakeError::transport("connection lost").is_recoverable());
        assert!(HandshakeError::malformed_response("invalid JSON").is_recoverable());
        
        assert!(!HandshakeError::IncompatibleProtocol {
            device_version: "0.5.0".to_string(),
            client_versions: vec!["1.0.0".to_string()],
        }.is_recoverable());
        
        assert!(!HandshakeError::DeviceRejection {
            reason: "auth failed".to_string(),
        }.is_recoverable());
    }
    
    #[test]
    fn test_handshake_error_categories() {
        assert_eq!(HandshakeError::Timeout.category(), ErrorCategory::Transport);
        assert_eq!(
            HandshakeError::IncompatibleProtocol {
                device_version: "0.5.0".to_string(),
                client_versions: vec!["1.0.0".to_string()],
            }.category(),
            ErrorCategory::Version
        );
        assert_eq!(
            HandshakeError::MissingCapability {
                capability: "telemetry".to_string(),
            }.category(),
            ErrorCategory::Capability
        );
    }
    
    #[test]
    fn test_user_friendly_messages() {
        let timeout_msg = HandshakeError::Timeout.user_friendly_message();
        assert!(timeout_msg.contains("5 seconds"));
        assert!(timeout_msg.contains("check the connection"));
        
        let incompatible_msg = HandshakeError::IncompatibleProtocol {
            device_version: "0.5.0".to_string(),
            client_versions: vec!["1.0.0".to_string()],
        }.user_friendly_message();
        assert!(incompatible_msg.contains("not supported"));
        assert!(incompatible_msg.contains("0.5.0"));
    }
}