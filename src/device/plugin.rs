use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::fs;
use libloading::{Library, Symbol};
use crate::device::{DeviceResult, DeviceError, DeviceDriver};
use crate::device::driver::{DriverInfo, DriverPriority};

/// Plugin manifest structure (TOML/JSON format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Plugin metadata
    pub plugin: PluginMetadata,
    
    /// Driver configuration
    pub driver: DriverConfig,
    
    /// Dependencies
    #[serde(default)]
    pub dependencies: Vec<String>,
    
    /// Platform-specific settings
    #[serde(default)]
    pub platform: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub license: String,
    #[serde(default)]
    pub homepage: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriverConfig {
    /// Entry point function name
    #[serde(default = "default_entry_point")]
    pub entry_point: String,
    
    /// Driver priority
    #[serde(default)]
    pub priority: u8,
    
    /// Supported devices (vendor:product IDs)
    #[serde(default)]
    pub devices: Vec<String>,
    
    /// Required transports
    pub transports: Vec<String>,
    
    /// Capabilities
    #[serde(default)]
    pub capabilities: HashMap<String, serde_json::Value>,
}

fn default_entry_point() -> String {
    "create_driver".to_string()
}

/// Plugin loader for dynamic driver loading
pub struct PluginLoader {
    /// Base directory for plugins
    plugin_dir: PathBuf,
    
    /// Loaded libraries (kept alive)
    libraries: Vec<Library>,
    
    /// Loaded drivers
    drivers: Vec<DriverInfo>,
}

impl PluginLoader {
    /// Create a new plugin loader
    pub fn new<P: AsRef<Path>>(plugin_dir: P) -> Self {
        PluginLoader {
            plugin_dir: plugin_dir.as_ref().to_path_buf(),
            libraries: Vec::new(),
            drivers: Vec::new(),
        }
    }
    
    /// Load all plugins from the plugin directory
    pub async fn load_all(&mut self) -> DeviceResult<Vec<DriverInfo>> {
        let mut loaded = Vec::new();
        
        // Scan plugin directory
        let mut entries = fs::read_dir(&self.plugin_dir).await
            .map_err(|e| DeviceError::PluginLoadError(format!("Failed to read plugin dir: {}", e)))?;
        
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| DeviceError::PluginLoadError(e.to_string()))? {
            
            let path = entry.path();
            if path.is_dir() {
                // Try to load plugin from subdirectory
                if let Ok(driver) = self.load_plugin(&path).await {
                    loaded.push(driver);
                }
            }
        }
        
        Ok(loaded)
    }
    
    /// Load a single plugin from a directory
    pub async fn load_plugin(&mut self, plugin_path: &Path) -> DeviceResult<DriverInfo> {
        // Look for manifest file (manifest.toml or manifest.json)
        let manifest = self.load_manifest(plugin_path).await?;
        
        // Determine library path based on platform
        let lib_name = if cfg!(windows) {
            format!("{}.dll", manifest.plugin.name)
        } else if cfg!(macos) {
            format!("lib{}.dylib", manifest.plugin.name)
        } else {
            format!("lib{}.so", manifest.plugin.name)
        };
        
        let lib_path = plugin_path.join(&lib_name);
        
        // Load the dynamic library
        let library = unsafe {
            Library::new(&lib_path)
                .map_err(|e| DeviceError::PluginLoadError(
                    format!("Failed to load library {}: {}", lib_path.display(), e)
                ))?
        };
        
        // Get the entry point function
        let driver: Arc<dyn DeviceDriver> = unsafe {
            let entry_point: Symbol<fn() -> Arc<dyn DeviceDriver>> = 
                library.get(manifest.driver.entry_point.as_bytes())
                    .map_err(|e| DeviceError::PluginLoadError(
                        format!("Entry point '{}' not found: {}", manifest.driver.entry_point, e)
                    ))?;
            
            entry_point()
        };
        
        // Create driver info
        let priority = match manifest.driver.priority {
            0..=25 => DriverPriority::Low,
            26..=75 => DriverPriority::Normal,
            76..=150 => DriverPriority::High,
            _ => DriverPriority::Critical,
        };
        
        let driver_info = DriverInfo::new(driver.clone())
            .with_priority(priority);
        
        // Store library to keep it loaded
        self.libraries.push(library);
        self.drivers.push(driver_info.clone());
        
        tracing::info!("Loaded plugin: {} v{}", manifest.plugin.name, manifest.plugin.version);
        
        Ok(driver_info)
    }
    
    /// Load manifest from plugin directory
    async fn load_manifest(&self, plugin_path: &Path) -> DeviceResult<PluginManifest> {
        // Try TOML first
        let toml_path = plugin_path.join("manifest.toml");
        if toml_path.exists() {
            let content = fs::read_to_string(&toml_path).await
                .map_err(|e| DeviceError::InvalidManifest(e.to_string()))?;
            
            return toml::from_str(&content)
                .map_err(|e| DeviceError::InvalidManifest(format!("TOML parse error: {}", e)));
        }
        
        // Try JSON
        let json_path = plugin_path.join("manifest.json");
        if json_path.exists() {
            let content = fs::read_to_string(&json_path).await
                .map_err(|e| DeviceError::InvalidManifest(e.to_string()))?;
            
            return serde_json::from_str(&content)
                .map_err(|e| DeviceError::InvalidManifest(format!("JSON parse error: {}", e)));
        }
        
        Err(DeviceError::InvalidManifest(
            format!("No manifest found in {}", plugin_path.display())
        ))
    }
    
    /// Get all loaded drivers
    pub fn drivers(&self) -> &[DriverInfo] {
        &self.drivers
    }
    
    /// Find a driver by name
    pub fn find_driver(&self, name: &str) -> Option<&DriverInfo> {
        self.drivers.iter().find(|d| d.name == name)
    }
}

/// Plugin API version for compatibility checking
pub const PLUGIN_API_VERSION: &str = "0.1.0";

/// Macro for creating plugin entry point
#[macro_export]
macro_rules! export_driver {
    ($driver_type:ty) => {
        #[no_mangle]
        pub extern "C" fn create_driver() -> Arc<dyn DeviceDriver> {
            Arc::new(<$driver_type>::new())
        }
    };
}