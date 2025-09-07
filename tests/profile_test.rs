// Profile management integration tests
use multi_controller_app::profile::*;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Create a test configuration with a temporary directory
fn test_setup() -> (ProfileConfig, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let config = ProfileConfig {
        profile_dir: temp_dir.path().to_path_buf(),
        auto_save: true,
        watch_for_changes: true,
        backup_on_save: true,
        max_backups: 3,
    };
    (config, temp_dir)
}

#[test]
fn test_profile_default_values() {
    let profile = Profile::default();
    
    assert_eq!(profile.metadata.name, "default");
    assert_eq!(profile.metadata.version, "1.0.0");
    assert_eq!(profile.user.preferences.theme, "dark");
    assert_eq!(profile.telemetry.buffer_size, 2000);
    assert_eq!(profile.telemetry.sample_rate, 30.0);
    assert_eq!(profile.ui.chart_settings.update_interval_ms, 33);
    assert_eq!(profile.ui.chart_settings.max_points, 300);
}

#[test]
fn test_profile_serialization() {
    let mut profile = Profile::default();
    profile.user.username = Some("test_user".to_string());
    profile.device.default_device_type = Some("Arduino Uno".to_string());
    
    // Serialize to TOML
    let toml_str = toml::to_string_pretty(&profile).unwrap();
    assert!(toml_str.contains("test_user"));
    assert!(toml_str.contains("Arduino Uno"));
    
    // Deserialize back
    let deserialized: Profile = toml::from_str(&toml_str).unwrap();
    assert_eq!(deserialized.user.username, Some("test_user".to_string()));
    assert_eq!(deserialized.device.default_device_type, Some("Arduino Uno".to_string()));
}

#[test]
fn test_profile_manager_initialization() {
    let (config, _temp) = test_setup();
    let manager = ProfileManager::new(config).unwrap();
    
    // Should have default profile
    let profiles = manager.list_profiles().unwrap();
    assert!(profiles.contains(&"default".to_string()));
    
    // Default profile should be loadable
    let default_profile = manager.load_profile("default").unwrap();
    assert_eq!(default_profile.metadata.name, "default");
}

#[test]
fn test_profile_create_save_load() {
    let (config, _temp) = test_setup();
    let manager = ProfileManager::new(config).unwrap();
    
    // Create a new profile
    manager.create_profile("test_profile", None).unwrap();
    
    // Load and modify
    let mut profile = manager.load_profile("test_profile").unwrap();
    profile.user.username = Some("modified_user".to_string());
    profile.device.scan_interval_ms = 10000;
    
    // Save changes
    manager.save_profile("test_profile", profile.clone()).unwrap();
    
    // Reload and verify
    let loaded = manager.load_profile("test_profile").unwrap();
    assert_eq!(loaded.user.username, Some("modified_user".to_string()));
    assert_eq!(loaded.device.scan_interval_ms, 10000);
}

#[test]
fn test_profile_deletion() {
    let (config, _temp) = test_setup();
    let manager = ProfileManager::new(config).unwrap();
    
    // Create profile
    manager.create_profile("to_delete", None).unwrap();
    assert!(manager.list_profiles().unwrap().contains(&"to_delete".to_string()));
    
    // Delete profile
    manager.delete_profile("to_delete").unwrap();
    assert!(!manager.list_profiles().unwrap().contains(&"to_delete".to_string()));
    
    // Should not be able to load deleted profile
    assert!(manager.load_profile("to_delete").is_err());
}

#[test]
fn test_profile_backup_creation() {
    let (config, temp) = test_setup();
    let manager = ProfileManager::new(config).unwrap();
    
    // Create and modify profile multiple times
    manager.create_profile("backup_test", None).unwrap();
    
    for i in 0..5 {
        let mut profile = manager.load_profile("backup_test").unwrap();
        profile.user.username = Some(format!("user_{}", i));
        manager.save_profile("backup_test", profile).unwrap();
    }
    
    // Check backups exist
    let backup_dir = temp.path().join("backups");
    assert!(backup_dir.exists());
    
    let backups: Vec<_> = fs::read_dir(backup_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    
    // Should have max_backups number of backups (3)
    assert!(backups.len() <= 3);
}

#[test]
fn test_current_profile_management() {
    let (config, _temp) = test_setup();
    let manager = ProfileManager::new(config).unwrap();
    
    // Create profiles
    manager.create_profile("profile1", None).unwrap();
    manager.create_profile("profile2", None).unwrap();
    
    // Set current profile
    manager.set_current_profile("profile1").unwrap();
    assert_eq!(manager.current_profile_name(), Some("profile1".to_string()));
    
    // Switch to different profile
    manager.set_current_profile("profile2").unwrap();
    assert_eq!(manager.current_profile_name(), Some("profile2".to_string()));
    
    // Get current profile
    let current = manager.current_profile().unwrap().unwrap();
    assert_eq!(current.metadata.name, "profile2");
}

#[test]
fn test_profile_change_callbacks() {
    let (config, _temp) = test_setup();
    let manager = Arc::new(ProfileManager::new(config).unwrap());
    
    // Set up callback
    let callback_triggered = Arc::new(std::sync::RwLock::new(false));
    let callback_triggered_clone = Arc::clone(&callback_triggered);
    
    manager.on_profile_change(move |profile| {
        if let Ok(mut triggered) = callback_triggered_clone.write() {
            *triggered = true;
            assert_eq!(profile.metadata.name, "callback_test");
        }
    }).unwrap();
    
    // Create and set profile
    manager.create_profile("callback_test", None).unwrap();
    manager.set_current_profile("callback_test").unwrap();
    
    // Verify callback was triggered
    thread::sleep(Duration::from_millis(100));
    let triggered = callback_triggered.read().unwrap();
    assert!(*triggered);
}

#[test]
fn test_profile_hot_reload() {
    let (config, temp) = test_setup();
    let profile_dir = temp.path().to_path_buf();
    let manager = Arc::new(ProfileManager::new(config).unwrap());
    
    // Create initial profile
    let mut profile = Profile::default();
    profile.user.username = Some("initial".to_string());
    manager.save_profile("hot_reload_test", profile).unwrap();
    manager.set_current_profile("hot_reload_test").unwrap();
    
    // Set up change detection
    let reload_detected = Arc::new(std::sync::RwLock::new(false));
    let reload_detected_clone = Arc::clone(&reload_detected);
    let username_changed = Arc::new(std::sync::RwLock::new(String::new()));
    let username_changed_clone = Arc::clone(&username_changed);
    
    manager.on_profile_change(move |profile| {
        if let Ok(mut detected) = reload_detected_clone.write() {
            *detected = true;
        }
        if let Some(username) = &profile.user.username {
            if let Ok(mut stored) = username_changed_clone.write() {
                *stored = username.clone();
            }
        }
    }).unwrap();
    
    // Start watcher
    let mut watcher = ProfileWatcher::new(Arc::clone(&manager));
    watcher.start(profile_dir.clone()).unwrap();
    
    // Give watcher time to initialize
    thread::sleep(Duration::from_millis(500));
    
    // Modify profile file directly
    let profile_path = profile_dir.join("hot_reload_test.toml");
    let mut modified_profile = manager.load_profile("hot_reload_test").unwrap();
    modified_profile.user.username = Some("hot_reloaded".to_string());
    modified_profile.device.scan_interval_ms = 7500;
    
    let content = toml::to_string_pretty(&modified_profile).unwrap();
    fs::write(&profile_path, content).unwrap();
    
    // Wait for hot-reload
    thread::sleep(Duration::from_secs(2));
    
    // Verify hot-reload occurred
    let detected = reload_detected.read().unwrap();
    assert!(*detected, "Hot-reload should have been detected");
    
    let username = username_changed.read().unwrap();
    assert_eq!(*username, "hot_reloaded");
    
    // Verify profile was actually reloaded
    let current = manager.current_profile().unwrap().unwrap();
    assert_eq!(current.user.username, Some("hot_reloaded".to_string()));
    assert_eq!(current.device.scan_interval_ms, 7500);
    
    // Stop watcher
    watcher.stop();
}

#[test]
fn test_invalid_profile_names() {
    let (config, _temp) = test_setup();
    let manager = ProfileManager::new(config).unwrap();
    
    // Test invalid names
    let invalid_names = vec!["", "profile/with/slash", "profile\\with\\backslash", 
                             "profile:with:colon", "profile*with*star"];
    
    for name in invalid_names {
        let profile = Profile::default();
        assert!(manager.save_profile(name, profile).is_err());
    }
}

#[test]
fn test_profile_template_creation() {
    let (config, _temp) = test_setup();
    let manager = ProfileManager::new(config).unwrap();
    
    // Create template profile
    let mut template = Profile::default();
    template.user.preferences.theme = "light".to_string();
    template.device.scan_interval_ms = 2500;
    manager.save_profile("template", template).unwrap();
    
    // Create new profile from template
    manager.create_profile("from_template", Some("template")).unwrap();
    
    // Verify it has template values
    let created = manager.load_profile("from_template").unwrap();
    assert_eq!(created.user.preferences.theme, "light");
    assert_eq!(created.device.scan_interval_ms, 2500);
}

#[test]
fn test_concurrent_profile_access() {
    let (config, _temp) = test_setup();
    let manager = Arc::new(ProfileManager::new(config).unwrap());
    
    // Create test profile
    manager.create_profile("concurrent", None).unwrap();
    
    // Spawn multiple threads accessing the profile
    let mut handles = vec![];
    
    for i in 0..5 {
        let manager_clone = Arc::clone(&manager);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                // Retry logic for Windows file locking
                let mut retries = 0;
                loop {
                    // Try to load profile
                    match manager_clone.load_profile("concurrent") {
                        Ok(mut profile) => {
                            // Modify profile
                            profile.user.username = Some(format!("thread_{}_iter_{}", i, j));
                            
                            // Try to save profile with retries
                            match manager_clone.save_profile("concurrent", profile) {
                                Ok(_) => break,
                                Err(e) => {
                                    if retries < 3 {
                                        retries += 1;
                                        thread::sleep(Duration::from_millis(50));
                                        continue;
                                    } else {
                                        panic!("Failed to save profile after 3 retries: {:?}", e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            if retries < 3 {
                                retries += 1;
                                thread::sleep(Duration::from_millis(50));
                                continue;
                            } else {
                                panic!("Failed to load profile after 3 retries: {:?}", e);
                            }
                        }
                    }
                }
                
                thread::sleep(Duration::from_millis(20));
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify profile is still valid
    let final_profile = manager.load_profile("concurrent").unwrap();
    assert!(final_profile.user.username.is_some());
}