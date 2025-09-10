/// Scripting runtime module for device automation
/// 
/// Provides a sandboxed Rhai scripting environment for automating
/// device operations with strict resource limits and security controls.

mod engine;
mod sandbox;
mod api;
mod errors;
mod async_bridge;

pub use engine::{ScriptEngine, ScriptContext};
pub use sandbox::{SandboxConfig, ResourceLimits};
pub use api::{DeviceApi, ScriptDeviceHandle};
pub use errors::{ScriptError, ScriptResult};
pub use async_bridge::{AsyncBridge, DeviceHandle};

/// Re-export commonly used Rhai types
pub use rhai::{Dynamic, Scope};

#[cfg(test)]
mod tests;