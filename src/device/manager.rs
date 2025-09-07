use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use tokio::sync::mpsc;
use notify::{Watcher, RecursiveMode, Event};
use crate::device::{
    DeviceResult, DeviceError, DeviceDriver, DeviceSession, 
    Transport, PluginLoader, SafetyController, EmergencyStop
};
use crate::device::driver::DriverInfo;
use crate::device::safety::{HotPlugMonitor, HotPlugEvent};

/// Central device manager
/// Coordinates plugin loading, device detection, and session management
pub struct DeviceManager {
    /// Plugin loader
    plugin_loader: Arc<RwLock<PluginLoader>>,
    
    /// Loaded drivers
    drivers: Arc<RwLock<Vec<DriverInfo>>>,
    
    /// Active sessions
    sessions: Arc<RwLock<HashMap<String, Box<dyn DeviceSession>>>>,
    
    /// Safety controller
    safety: Arc<SafetyController>,
    
    /// Emergency stop
    emergency_stop: Arc<EmergencyStop>,
    
    /// Hot-plug monitor
    hotplug: HotPlugMonitor,
    hotplug_rx: Arc<RwLock<mpsc::UnboundedReceiver<HotPlugEvent>>>,
    
    /// File system watcher for plugin changes
    watcher: Arc<RwLock<Option<notify::RecommendedWatcher>>>,
}

impl DeviceManager {
    /// Create a new device manager
    pub fn new(plugin_dir: &str) -> Self {
        let emergency_stop = Arc::new(EmergencyStop::new());
        let safety = Arc::new(SafetyController::new(emergency_stop.clone()));
        let (hotplug, hotplug_rx) = HotPlugMonitor::new();
        
        DeviceManager {
            plugin_loader: Arc::new(RwLock::new(PluginLoader::new(plugin_dir))),
            drivers: Arc::new(RwLock::new(Vec::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            safety,
            emergency_stop,
            hotplug,
            hotplug_rx: Arc::new(RwLock::new(hotplug_rx)),
            watcher: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Initialize the device manager
    pub async fn initialize(&self) -> DeviceResult<()> {
        // Load all plugins
        self.load_plugins().await?;
        
        // Start hot-plug monitoring
        self.start_hotplug_monitor().await;
        
        // Start plugin directory watching
        self.start_plugin_watcher().await?;
        
        tracing::info!("Device manager initialized");
        Ok(())
    }
    
    /// Load all plugins
    pub async fn load_plugins(&self) -> DeviceResult<Vec<DriverInfo>> {
        let mut loader = self.plugin_loader.write().await;
        let loaded = loader.load_all().await?;
        
        let mut drivers = self.drivers.write().await;
        drivers.extend(loaded.clone());
        
        tracing::info!("Loaded {} device drivers", drivers.len());
        Ok(loaded)
    }
    
    /// Probe for a device on a transport
    pub async fn probe_device(&self, transport: Arc<dyn Transport>) -> DeviceResult<Arc<dyn DeviceDriver>> {
        // Check emergency stop
        self.emergency_stop.guard().ensure_running()?;
        
        let drivers = self.drivers.read().await;
        
        // Sort by priority and probe
        let mut sorted_drivers = drivers.clone();
        sorted_drivers.sort_by_key(|d| std::cmp::Reverse(d.priority));
        
        for driver_info in sorted_drivers {
            match driver_info.driver.probe_async(transport.clone()).await {
                Ok(true) => {
                    tracing::info!("Device detected by driver: {}", driver_info.name);
                    return Ok(driver_info.driver.clone());
                }
                Ok(false) => continue,
                Err(e) => {
                    tracing::warn!("Probe failed for {}: {}", driver_info.name, e);
                    continue;
                }
            }
        }
        
        Err(DeviceError::DeviceNotFound("No driver recognized the device".into()))
    }
    
    /// Open a device session
    pub async fn open_device(
        &self, 
        transport: Arc<dyn Transport>,
        session_id: Option<String>
    ) -> DeviceResult<String> {
        // Check emergency stop
        self.emergency_stop.guard().ensure_running()?;
        
        // Rate limit device opening
        self.safety.check_rate_limit("open_device").await?;
        
        // Probe for appropriate driver
        let driver = self.probe_device(transport.clone()).await?;
        
        // Open session
        let session = driver.open_async(transport).await?;
        
        // Generate session ID
        let id = session_id.unwrap_or_else(|| {
            format!("session_{}", uuid::Uuid::new_v4())
        });
        
        // Store session
        let mut sessions = self.sessions.write().await;
        sessions.insert(id.clone(), session);
        
        tracing::info!("Opened device session: {}", id);
        Ok(id)
    }
    
    /// Close a device session
    pub async fn close_device(&self, session_id: &str) -> DeviceResult<()> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(mut session) = sessions.remove(session_id) {
            session.close_async().await?;
            tracing::info!("Closed device session: {}", session_id);
            Ok(())
        } else {
            Err(DeviceError::DeviceNotFound(format!("Session not found: {}", session_id)))
        }
    }
    
    /// Get an active session
    pub async fn get_session(&self, session_id: &str) -> Option<Box<dyn DeviceSession>> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).map(|s| {
            // This would need proper cloning/boxing in real implementation
            // For now, returning None as sessions can't be easily cloned
            None
        }).flatten()
    }
    
    /// List active sessions
    pub async fn list_sessions(&self) -> Vec<String> {
        let sessions = self.sessions.read().await;
        sessions.keys().cloned().collect()
    }
    
    /// Trigger emergency stop
    pub async fn emergency_stop(&self, reason: String) {
        self.emergency_stop.trigger(crate::device::safety::StopReason::UserRequested).await;
        
        // Close all sessions
        let session_ids = self.list_sessions().await;
        for id in session_ids {
            let _ = self.close_device(&id).await;
        }
    }
    
    /// Reset emergency stop
    pub async fn reset_emergency_stop(&self) {
        self.emergency_stop.reset().await;
        self.safety.reset_violations();
    }
    
    /// Start hot-plug monitoring
    async fn start_hotplug_monitor(&self) {
        let rx = self.hotplug_rx.clone();
        let safety = self.safety.clone();
        
        tokio::spawn(async move {
            let mut rx = rx.write().await;
            while let Some(event) = rx.recv().await {
                // Rate limit hot-plug events
                if safety.check_rate_limit("hotplug").await.is_err() {
                    tracing::warn!("Hot-plug event rate limited");
                    continue;
                }
                
                match event {
                    HotPlugEvent::DeviceAdded(id) => {
                        tracing::info!("Device added: {}", id);
                        // Trigger auto-detection logic here
                    }
                    HotPlugEvent::DeviceRemoved(id) => {
                        tracing::info!("Device removed: {}", id);
                        // Trigger cleanup logic here
                    }
                }
            }
        });
    }
    
    /// Start plugin directory watcher
    async fn start_plugin_watcher(&self) -> DeviceResult<()> {
        let (tx, mut rx) = mpsc::channel(100);
        
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        }).map_err(|e| DeviceError::Unknown(e.to_string()))?;
        
        // Get plugin directory from loader
        let loader = self.plugin_loader.read().await;
        
        // Watch plugin directory
        watcher.watch(
            std::path::Path::new("./drivers"), 
            RecursiveMode::NonRecursive
        ).map_err(|e| DeviceError::Unknown(e.to_string()))?;
        
        *self.watcher.write().await = Some(watcher);
        
        // Handle file system events
        let plugin_loader = self.plugin_loader.clone();
        let drivers = self.drivers.clone();
        
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                if event.kind.is_create() || event.kind.is_modify() {
                    tracing::info!("Plugin directory changed, reloading...");
                    // Reload plugins (in production, would be more sophisticated)
                    if let Ok(loaded) = plugin_loader.write().await.load_all().await {
                        *drivers.write().await = loaded;
                    }
                }
            }
        });
        
        Ok(())
    }
    
    /// Get safety controller
    pub fn safety(&self) -> Arc<SafetyController> {
        self.safety.clone()
    }
    
    /// Get emergency stop handle
    pub fn emergency_stop_handle(&self) -> Arc<EmergencyStop> {
        self.emergency_stop.clone()
    }
}

// Add uuid for session IDs
use uuid;