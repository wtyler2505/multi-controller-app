// Profile file watcher for hot-reload functionality
use super::manager::ProfileManager;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// Profile watcher for hot-reload functionality
pub struct ProfileWatcher {
    watcher: Option<RecommendedWatcher>,
    receiver: Option<Receiver<notify::Result<Event>>>,
    manager: Arc<ProfileManager>,
    watch_thread: Option<thread::JoinHandle<()>>,
    running: Arc<RwLock<bool>>,
}

impl ProfileWatcher {
    /// Create a new profile watcher
    pub fn new(manager: Arc<ProfileManager>) -> Self {
        Self {
            watcher: None,
            receiver: None,
            manager,
            watch_thread: None,
            running: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Start watching for profile changes
    pub fn start(&mut self, profile_dir: PathBuf) -> Result<(), notify::Error> {
        // Check if already running
        if let Ok(running) = self.running.read() {
            if *running {
                warn!("Profile watcher already running");
                return Ok(());
            }
        }
        
        // Create channel for events
        let (tx, rx) = channel();
        
        // Create watcher
        let mut watcher = RecommendedWatcher::new(
            move |res: notify::Result<Event>| {
                if let Err(e) = tx.send(res) {
                    error!("Failed to send watch event: {}", e);
                }
            },
            notify::Config::default()
                .with_poll_interval(Duration::from_secs(1))
        )?;
        
        // Watch the profile directory
        watcher.watch(&profile_dir, RecursiveMode::NonRecursive)?;
        
        info!("Started watching profile directory: {:?}", profile_dir);
        
        // Store watcher and receiver
        self.watcher = Some(watcher);
        self.receiver = Some(rx);
        
        // Start processing thread
        self.start_processing_thread();
        
        // Mark as running
        if let Ok(mut running) = self.running.write() {
            *running = true;
        }
        
        Ok(())
    }
    
    /// Start the event processing thread
    fn start_processing_thread(&mut self) {
        let receiver = self.receiver.take();
        let manager = Arc::clone(&self.manager);
        let running = Arc::clone(&self.running);
        
        if let Some(rx) = receiver {
            let handle = thread::spawn(move || {
                Self::process_events(rx, manager, running);
            });
            
            self.watch_thread = Some(handle);
        }
    }
    
    /// Process file system events
    fn process_events(
        receiver: Receiver<notify::Result<Event>>,
        manager: Arc<ProfileManager>,
        running: Arc<RwLock<bool>>,
    ) {
        loop {
            // Check if we should stop
            if let Ok(r) = running.read() {
                if !*r {
                    break;
                }
            }
            
            // Wait for events with timeout
            match receiver.recv_timeout(Duration::from_millis(500)) {
                Ok(Ok(event)) => {
                    Self::handle_event(event, &manager);
                }
                Ok(Err(e)) => {
                    error!("Watch error: {}", e);
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Timeout is normal, continue
                    continue;
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    // Channel disconnected, stop watching
                    info!("Watch channel disconnected, stopping");
                    break;
                }
            }
        }
        
        debug!("Profile watcher thread exiting");
    }
    
    /// Handle a file system event
    fn handle_event(event: Event, manager: &Arc<ProfileManager>) {
        match event.kind {
            EventKind::Modify(_) => {
                for path in event.paths {
                    if Self::is_profile_file(&path) {
                        Self::handle_profile_change(&path, manager);
                    }
                }
            }
            EventKind::Create(_) => {
                for path in event.paths {
                    if Self::is_profile_file(&path) {
                        info!("New profile detected: {:?}", path);
                        // Rescan profiles to pick up new file
                        if let Err(e) = manager.scan_profiles() {
                            error!("Failed to scan profiles: {}", e);
                        }
                    }
                }
            }
            EventKind::Remove(_) => {
                for path in event.paths {
                    if Self::is_profile_file(&path) {
                        info!("Profile removed: {:?}", path);
                        // Rescan profiles to remove deleted file
                        if let Err(e) = manager.scan_profiles() {
                            error!("Failed to scan profiles: {}", e);
                        }
                    }
                }
            }
            _ => {
                // Ignore other events
            }
        }
    }
    
    /// Check if a path is a profile file
    fn is_profile_file(path: &PathBuf) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == crate::profile::PROFILE_EXTENSION)
            .unwrap_or(false)
    }
    
    /// Handle a profile file change
    fn handle_profile_change(path: &PathBuf, manager: &Arc<ProfileManager>) {
        if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
            // Skip backup files
            if name.contains("backup") || name.contains("~") {
                return;
            }
            
            info!("Profile changed: {}", name);
            
            // Add a small delay to ensure file write is complete
            thread::sleep(Duration::from_millis(100));
            
            // Reload the profile
            match manager.reload_profile(name) {
                Ok(()) => {
                    info!("Successfully hot-reloaded profile: {}", name);
                }
                Err(e) => {
                    error!("Failed to reload profile {}: {}", name, e);
                }
            }
        }
    }
    
    /// Stop watching for changes
    pub fn stop(&mut self) {
        info!("Stopping profile watcher");
        
        // Mark as not running
        if let Ok(mut running) = self.running.write() {
            *running = false;
        }
        
        // Drop the watcher to stop watching
        self.watcher = None;
        
        // Wait for thread to finish
        if let Some(handle) = self.watch_thread.take() {
            if let Err(e) = handle.join() {
                error!("Failed to join watcher thread: {:?}", e);
            }
        }
        
        info!("Profile watcher stopped");
    }
}

impl Drop for ProfileWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::config::{Profile, ProfileConfig};
    use crate::profile::manager::ProfileManager;
    use std::fs;
    use tempfile::TempDir;
    
    #[test]
    fn test_profile_watcher_creation() {
        let temp_dir = TempDir::new().unwrap();
        let config = ProfileConfig {
            profile_dir: temp_dir.path().to_path_buf(),
            auto_save: false,
            watch_for_changes: false,
            backup_on_save: false,
            max_backups: 3,
        };
        
        let manager = Arc::new(ProfileManager::new(config).unwrap());
        let _watcher = ProfileWatcher::new(manager);
    }
    
    #[test]
    fn test_hot_reload() {
        let temp_dir = TempDir::new().unwrap();
        let profile_dir = temp_dir.path().to_path_buf();
        
        let config = ProfileConfig {
            profile_dir: profile_dir.clone(),
            auto_save: false,
            watch_for_changes: true,
            backup_on_save: false,
            max_backups: 3,
        };
        
        let manager = Arc::new(ProfileManager::new(config).unwrap());
        
        // Create a test profile
        let profile = Profile::default();
        manager.save_profile("test", profile).unwrap();
        manager.set_current_profile("test").unwrap();
        
        // Set up change detection
        let changed = Arc::new(RwLock::new(false));
        let changed_clone = Arc::clone(&changed);
        
        manager.on_profile_change(move |_profile| {
            if let Ok(mut c) = changed_clone.write() {
                *c = true;
            }
        }).unwrap();
        
        // Start watcher
        let mut watcher = ProfileWatcher::new(Arc::clone(&manager));
        watcher.start(profile_dir.clone()).unwrap();
        
        // Give watcher time to start
        thread::sleep(Duration::from_millis(500));
        
        // Modify the profile file
        let profile_path = profile_dir.join("test.toml");
        let mut modified_profile = manager.load_profile("test").unwrap();
        modified_profile.user.username = Some("hot_reload_test".to_string());
        
        let content = toml::to_string_pretty(&modified_profile).unwrap();
        fs::write(&profile_path, content).unwrap();
        
        // Wait for hot-reload to trigger
        thread::sleep(Duration::from_secs(2));
        
        // Check if change was detected
        let was_changed = changed.read().unwrap();
        assert!(*was_changed, "Hot-reload should have triggered");
        
        // Verify profile was reloaded
        let current = manager.current_profile().unwrap().unwrap();
        assert_eq!(current.user.username, Some("hot_reload_test".to_string()));
        
        // Stop watcher
        watcher.stop();
    }
}