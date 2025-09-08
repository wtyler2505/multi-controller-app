//! Multi-Controller Core
//! 
//! This crate provides the fundamental traits and types for the Multi-Controller App,
//! defining the interfaces for device drivers, transports, and session management.

pub mod transport;
pub mod driver;
pub mod session;
pub mod error;

pub use transport::{Transport, TransportType, TransportInfo, TransportConfig};
pub use driver::{DeviceDriver, DeviceInfo};
pub use session::{DeviceSession, SessionId, ConnectionState};
pub use error::{Result, MultiControllerError};

/// Re-export commonly used types
pub use uuid::Uuid;
pub use std::collections::HashMap;
pub use async_trait::async_trait;
pub use serde_json::Value;