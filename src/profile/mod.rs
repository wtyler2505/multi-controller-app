// Profile management module with hot-reload support
pub mod config;
pub mod manager;
pub mod watcher;

pub use config::{Profile, ProfileConfig, DeviceSettings, UserSettings};
pub use manager::{ProfileManager, ProfileError};
pub use watcher::ProfileWatcher;

use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Result type for profile operations
pub type ProfileResult<T> = Result<T, ProfileError>;

/// Default profile directory relative to app data
pub fn default_profile_dir() -> PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        data_dir.join("multi-controller-app").join("profiles")
    } else {
        PathBuf::from("./profiles")
    }
}

/// Profile file extension
pub const PROFILE_EXTENSION: &str = "toml";

/// Default profile name
pub const DEFAULT_PROFILE: &str = "default";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profile_dir() {
        let dir = default_profile_dir();
        assert!(dir.to_str().is_some());
    }
}