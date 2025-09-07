// Profile configuration structures
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Main profile structure containing all settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    /// Profile metadata
    pub metadata: ProfileMetadata,
    
    /// User-specific settings
    pub user: UserSettings,
    
    /// Device-specific settings
    pub device: DeviceSettings,
    
    /// Telemetry settings
    pub telemetry: TelemetrySettings,
    
    /// UI preferences
    pub ui: UiSettings,
    
    /// Transport configurations
    pub transports: Vec<TransportProfile>,
    
    /// Custom key-value pairs for extensions
    #[serde(default)]
    pub custom: HashMap<String, toml::Value>,
}

/// Profile metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileMetadata {
    pub name: String,
    pub version: String,
    pub created_at: String,
    pub modified_at: String,
    pub description: Option<String>,
}

/// User-specific settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserSettings {
    pub username: Option<String>,
    pub preferences: UserPreferences,
    pub workspace_paths: Vec<PathBuf>,
}

/// User preferences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub auto_save: bool,
    pub auto_connect: bool,
    pub notification_level: NotificationLevel,
}

/// Notification level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NotificationLevel {
    None,
    Errors,
    Warnings,
    Info,
    Debug,
}

/// Device-specific settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceSettings {
    pub default_device_type: Option<String>,
    pub auto_detect: bool,
    pub scan_interval_ms: u32,
    pub reconnect_attempts: u32,
    pub reconnect_delay_ms: u32,
    pub device_configs: Vec<DeviceConfig>,
}

/// Individual device configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeviceConfig {
    pub name: String,
    pub device_type: String,
    pub transport: String,
    pub address: String,
    pub settings: HashMap<String, toml::Value>,
}

/// Telemetry settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelemetrySettings {
    pub enabled: bool,
    pub buffer_size: usize,
    pub sample_rate: f32,
    pub auto_export: bool,
    pub export_format: ExportFormat,
    pub export_path: Option<PathBuf>,
    pub channels: Vec<ChannelProfile>,
}

/// Export format for telemetry data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Json,
    Csv,
    Binary,
    MessagePack,
}

/// Telemetry channel profile
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelProfile {
    pub name: String,
    pub enabled: bool,
    pub buffer_size: usize,
    pub sample_rate: f32,
    pub color: Option<String>,
}

/// UI settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UiSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub window_x: Option<i32>,
    pub window_y: Option<i32>,
    pub maximized: bool,
    pub panel_layout: PanelLayout,
    pub chart_settings: ChartSettings,
}

/// Panel layout configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PanelLayout {
    pub show_device_panel: bool,
    pub show_telemetry_panel: bool,
    pub show_log_panel: bool,
    pub show_script_panel: bool,
    pub panel_sizes: HashMap<String, f32>,
}

/// Chart display settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChartSettings {
    pub update_interval_ms: u32,
    pub max_points: usize,
    pub auto_scale: bool,
    pub show_legend: bool,
    pub show_tooltips: bool,
    pub show_grid: bool,
}

/// Transport profile configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransportProfile {
    pub name: String,
    pub transport_type: String,
    pub settings: HashMap<String, toml::Value>,
}

/// Profile configuration for the manager
#[derive(Debug, Clone)]
pub struct ProfileConfig {
    pub profile_dir: PathBuf,
    pub auto_save: bool,
    pub watch_for_changes: bool,
    pub backup_on_save: bool,
    pub max_backups: usize,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            metadata: ProfileMetadata {
                name: "default".to_string(),
                version: "1.0.0".to_string(),
                created_at: chrono::Utc::now().to_rfc3339(),
                modified_at: chrono::Utc::now().to_rfc3339(),
                description: Some("Default profile".to_string()),
            },
            user: UserSettings {
                username: None,
                preferences: UserPreferences {
                    theme: "dark".to_string(),
                    language: "en".to_string(),
                    auto_save: true,
                    auto_connect: false,
                    notification_level: NotificationLevel::Info,
                },
                workspace_paths: vec![],
            },
            device: DeviceSettings {
                default_device_type: None,
                auto_detect: true,
                scan_interval_ms: 5000,
                reconnect_attempts: 3,
                reconnect_delay_ms: 1000,
                device_configs: vec![],
            },
            telemetry: TelemetrySettings {
                enabled: true,
                buffer_size: 2000,
                sample_rate: 30.0,
                auto_export: false,
                export_format: ExportFormat::Json,
                export_path: None,
                channels: vec![],
            },
            ui: UiSettings {
                window_width: 1280,
                window_height: 720,
                window_x: None,
                window_y: None,
                maximized: false,
                panel_layout: PanelLayout {
                    show_device_panel: true,
                    show_telemetry_panel: true,
                    show_log_panel: true,
                    show_script_panel: false,
                    panel_sizes: HashMap::new(),
                },
                chart_settings: ChartSettings {
                    update_interval_ms: 33,  // ~30 FPS
                    max_points: 300,
                    auto_scale: true,
                    show_legend: true,
                    show_tooltips: true,
                    show_grid: true,
                },
            },
            transports: vec![],
            custom: HashMap::new(),
        }
    }
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            profile_dir: super::default_profile_dir(),
            auto_save: true,
            watch_for_changes: true,
            backup_on_save: true,
            max_backups: 5,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_default() {
        let profile = Profile::default();
        assert_eq!(profile.metadata.name, "default");
        assert_eq!(profile.ui.chart_settings.update_interval_ms, 33);
    }

    #[test]
    fn test_profile_serialization() {
        let profile = Profile::default();
        let toml_str = toml::to_string(&profile).unwrap();
        let deserialized: Profile = toml::from_str(&toml_str).unwrap();
        assert_eq!(profile, deserialized);
    }
}