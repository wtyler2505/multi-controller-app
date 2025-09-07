// Profile manager for save/load operations
use super::config::{Profile, ProfileConfig};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use thiserror::Error;
use tracing::{debug, error, info, warn};

/// Profile management errors
#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] toml::ser::Error),
    
    #[error("Deserialization error: {0}")]
    DeserializationError(#[from] toml::de::Error),
    
    #[error("Profile already exists: {0}")]
    ProfileAlreadyExists(String),
    
    #[error("Invalid profile name: {0}")]
    InvalidProfileName(String),
    
    #[error("Lock poisoned")]
    LockPoisoned,
}

/// Profile manager for handling save/load operations
pub struct ProfileManager {
    config: ProfileConfig,
    profiles: Arc<RwLock<HashMap<String, Profile>>>,
    current_profile: Arc<RwLock<Option<String>>>,
    change_callbacks: Arc<RwLock<Vec<Box<dyn Fn(&Profile) + Send + Sync>>>>,
}

impl ProfileManager {
    /// Create a new profile manager
    pub fn new(config: ProfileConfig) -> Result<Self, ProfileError> {
        // Ensure profile directory exists
        if !config.profile_dir.exists() {
            fs::create_dir_all(&config.profile_dir)?;
        }
        
        let manager = Self {
            config,
            profiles: Arc::new(RwLock::new(HashMap::new())),
            current_profile: Arc::new(RwLock::new(None)),
            change_callbacks: Arc::new(RwLock::new(Vec::new())),
        };
        
        // Load existing profiles
        manager.scan_profiles()?;
        
        Ok(manager)
    }
    
    /// Scan and load all profiles from the profile directory
    pub fn scan_profiles(&self) -> Result<(), ProfileError> {
        let mut profiles = self.profiles.write()
            .map_err(|_| ProfileError::LockPoisoned)?;
        
        profiles.clear();
        
        let entries = fs::read_dir(&self.config.profile_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some(super::PROFILE_EXTENSION) {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    match self.load_profile_from_path(&path) {
                        Ok(profile) => {
                            info!("Loaded profile: {}", name);
                            profiles.insert(name.to_string(), profile);
                        }
                        Err(e) => {
                            error!("Failed to load profile {}: {}", name, e);
                        }
                    }
                }
            }
        }
        
        // If no profiles exist, create default
        if profiles.is_empty() {
            let default_profile = Profile::default();
            profiles.insert(super::DEFAULT_PROFILE.to_string(), default_profile);
            self.save_profile_internal(super::DEFAULT_PROFILE, &Profile::default())?;
        }
        
        Ok(())
    }
    
    /// Load a profile from a file path
    fn load_profile_from_path(&self, path: &Path) -> Result<Profile, ProfileError> {
        let content = fs::read_to_string(path)?;
        let profile: Profile = toml::from_str(&content)?;
        Ok(profile)
    }
    
    /// Save a profile (internal helper)
    fn save_profile_internal(&self, name: &str, profile: &Profile) -> Result<(), ProfileError> {
        let path = self.profile_path(name);
        
        // Create backup if enabled
        if self.config.backup_on_save && path.exists() {
            self.create_backup(&path)?;
        }
        
        let content = toml::to_string_pretty(profile)?;
        fs::write(path, content)?;
        
        Ok(())
    }
    
    /// Get the file path for a profile
    fn profile_path(&self, name: &str) -> PathBuf {
        self.config.profile_dir
            .join(format!("{}.{}", name, super::PROFILE_EXTENSION))
    }
    
    /// Create a backup of a profile file
    fn create_backup(&self, path: &Path) -> Result<(), ProfileError> {
        let backup_dir = self.config.profile_dir.join("backups");
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir)?;
        }
        
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let file_name = path.file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| ProfileError::InvalidProfileName("Invalid path".to_string()))?;
        
        let backup_name = format!("{}_{}", timestamp, file_name);
        let backup_path = backup_dir.join(backup_name);
        
        fs::copy(path, backup_path)?;
        
        // Clean up old backups
        self.cleanup_backups(&backup_dir)?;
        
        Ok(())
    }
    
    /// Clean up old backup files
    fn cleanup_backups(&self, backup_dir: &Path) -> Result<(), ProfileError> {
        let mut backups: Vec<_> = fs::read_dir(backup_dir)?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.path().extension().and_then(|s| s.to_str()) == Some(super::PROFILE_EXTENSION)
            })
            .collect();
        
        if backups.len() > self.config.max_backups {
            // Sort by modification time (oldest first)
            backups.sort_by_key(|entry| {
                entry.metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
            });
            
            // Remove oldest backups
            let to_remove = backups.len() - self.config.max_backups;
            for entry in backups.iter().take(to_remove) {
                if let Err(e) = fs::remove_file(entry.path()) {
                    warn!("Failed to remove old backup: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Load a profile by name
    pub fn load_profile(&self, name: &str) -> Result<Profile, ProfileError> {
        let profiles = self.profiles.read()
            .map_err(|_| ProfileError::LockPoisoned)?;
        
        profiles.get(name)
            .cloned()
            .ok_or_else(|| ProfileError::ProfileNotFound(name.to_string()))
    }
    
    /// Save a profile
    pub fn save_profile(&self, name: &str, mut profile: Profile) -> Result<(), ProfileError> {
        // Validate profile name
        if name.is_empty() || name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|']) {
            return Err(ProfileError::InvalidProfileName(name.to_string()));
        }
        
        // Update metadata
        profile.metadata.name = name.to_string();
        profile.metadata.modified_at = chrono::Utc::now().to_rfc3339();
        
        // Save to file
        self.save_profile_internal(name, &profile)?;
        
        // Update in-memory cache
        let mut profiles = self.profiles.write()
            .map_err(|_| ProfileError::LockPoisoned)?;
        profiles.insert(name.to_string(), profile.clone());
        
        // Trigger callbacks if this is the current profile
        let current = self.current_profile.read()
            .map_err(|_| ProfileError::LockPoisoned)?;
        
        if current.as_ref() == Some(&name.to_string()) {
            self.notify_changes(&profile);
        }
        
        info!("Saved profile: {}", name);
        Ok(())
    }
    
    /// Delete a profile
    pub fn delete_profile(&self, name: &str) -> Result<(), ProfileError> {
        // Don't delete the default profile
        if name == super::DEFAULT_PROFILE {
            return Err(ProfileError::InvalidProfileName(
                "Cannot delete default profile".to_string()
            ));
        }
        
        let path = self.profile_path(name);
        if path.exists() {
            // Create backup before deletion
            if self.config.backup_on_save {
                self.create_backup(&path)?;
            }
            fs::remove_file(path)?;
        }
        
        let mut profiles = self.profiles.write()
            .map_err(|_| ProfileError::LockPoisoned)?;
        profiles.remove(name);
        
        // If this was the current profile, switch to default
        let mut current = self.current_profile.write()
            .map_err(|_| ProfileError::LockPoisoned)?;
        
        if current.as_ref() == Some(&name.to_string()) {
            *current = Some(super::DEFAULT_PROFILE.to_string());
            
            // Load and notify default profile
            if let Some(default_profile) = profiles.get(super::DEFAULT_PROFILE) {
                self.notify_changes(default_profile);
            }
        }
        
        info!("Deleted profile: {}", name);
        Ok(())
    }
    
    /// Create a new profile from a template
    pub fn create_profile(&self, name: &str, template: Option<&str>) -> Result<(), ProfileError> {
        // Check if profile already exists
        let profiles = self.profiles.read()
            .map_err(|_| ProfileError::LockPoisoned)?;
        
        if profiles.contains_key(name) {
            return Err(ProfileError::ProfileAlreadyExists(name.to_string()));
        }
        
        drop(profiles); // Release read lock
        
        // Create profile from template or default
        let profile = if let Some(template_name) = template {
            self.load_profile(template_name)?
        } else {
            Profile::default()
        };
        
        self.save_profile(name, profile)?;
        
        Ok(())
    }
    
    /// Set the current active profile
    pub fn set_current_profile(&self, name: &str) -> Result<(), ProfileError> {
        let profile = self.load_profile(name)?;
        
        let mut current = self.current_profile.write()
            .map_err(|_| ProfileError::LockPoisoned)?;
        *current = Some(name.to_string());
        
        // Notify all callbacks
        self.notify_changes(&profile);
        
        info!("Switched to profile: {}", name);
        Ok(())
    }
    
    /// Get the current profile name
    pub fn current_profile_name(&self) -> Option<String> {
        self.current_profile.read()
            .ok()
            .and_then(|c| c.clone())
    }
    
    /// Get the current profile
    pub fn current_profile(&self) -> Result<Option<Profile>, ProfileError> {
        let current = self.current_profile.read()
            .map_err(|_| ProfileError::LockPoisoned)?;
        
        if let Some(name) = current.as_ref() {
            self.load_profile(name).map(Some)
        } else {
            Ok(None)
        }
    }
    
    /// List all available profiles
    pub fn list_profiles(&self) -> Result<Vec<String>, ProfileError> {
        let profiles = self.profiles.read()
            .map_err(|_| ProfileError::LockPoisoned)?;
        
        Ok(profiles.keys().cloned().collect())
    }
    
    /// Register a callback for profile changes
    pub fn on_profile_change<F>(&self, callback: F) -> Result<(), ProfileError>
    where
        F: Fn(&Profile) + Send + Sync + 'static,
    {
        let mut callbacks = self.change_callbacks.write()
            .map_err(|_| ProfileError::LockPoisoned)?;
        
        callbacks.push(Box::new(callback));
        Ok(())
    }
    
    /// Notify all callbacks of profile changes
    fn notify_changes(&self, profile: &Profile) {
        if let Ok(callbacks) = self.change_callbacks.read() {
            for callback in callbacks.iter() {
                callback(profile);
            }
        }
    }
    
    /// Reload a profile from disk (used by hot-reload)
    pub fn reload_profile(&self, name: &str) -> Result<(), ProfileError> {
        let path = self.profile_path(name);
        let profile = self.load_profile_from_path(&path)?;
        
        // Update in-memory cache
        let mut profiles = self.profiles.write()
            .map_err(|_| ProfileError::LockPoisoned)?;
        profiles.insert(name.to_string(), profile.clone());
        
        // If this is the current profile, notify changes
        let current = self.current_profile.read()
            .map_err(|_| ProfileError::LockPoisoned)?;
        
        if current.as_ref() == Some(&name.to_string()) {
            self.notify_changes(&profile);
            info!("Hot-reloaded profile: {}", name);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    fn test_config() -> (ProfileConfig, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let config = ProfileConfig {
            profile_dir: temp_dir.path().to_path_buf(),
            auto_save: true,
            watch_for_changes: false,
            backup_on_save: true,
            max_backups: 3,
        };
        (config, temp_dir)
    }
    
    #[test]
    fn test_profile_manager_creation() {
        let (config, _temp) = test_config();
        let manager = ProfileManager::new(config).unwrap();
        
        // Should have default profile
        let profiles = manager.list_profiles().unwrap();
        assert!(profiles.contains(&super::super::DEFAULT_PROFILE.to_string()));
    }
    
    #[test]
    fn test_save_load_profile() {
        let (config, _temp) = test_config();
        let manager = ProfileManager::new(config).unwrap();
        
        let mut profile = Profile::default();
        profile.user.username = Some("test_user".to_string());
        
        manager.save_profile("test", profile.clone()).unwrap();
        
        let loaded = manager.load_profile("test").unwrap();
        assert_eq!(loaded.user.username, Some("test_user".to_string()));
    }
    
    #[test]
    fn test_delete_profile() {
        let (config, _temp) = test_config();
        let manager = ProfileManager::new(config).unwrap();
        
        manager.create_profile("test", None).unwrap();
        assert!(manager.list_profiles().unwrap().contains(&"test".to_string()));
        
        manager.delete_profile("test").unwrap();
        assert!(!manager.list_profiles().unwrap().contains(&"test".to_string()));
    }
    
    #[test]
    fn test_current_profile() {
        let (config, _temp) = test_config();
        let manager = ProfileManager::new(config).unwrap();
        
        manager.create_profile("test", None).unwrap();
        manager.set_current_profile("test").unwrap();
        
        assert_eq!(manager.current_profile_name(), Some("test".to_string()));
    }
}