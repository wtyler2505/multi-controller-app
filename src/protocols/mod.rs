//! Communication Protocols
//! 
//! This module provides protocol implementations for device communication,
//! including handshake protocols, command protocols, and data protocols.
//! 
//! # Protocols
//! 
//! - `handshake` - Device identification and session establishment

pub mod handshake;

// Re-export commonly used types
pub use handshake::{
    HandshakeMessage,
    IdentifyCommand,
    IdentifyResponse,
    HandshakeResult,
    HandshakeError,
    PROTOCOL_VERSION,
};