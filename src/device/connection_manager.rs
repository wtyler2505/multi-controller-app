use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;
use tracing::{info, warn, error, debug};
use crate::device::{DeviceResult, DeviceError, DeviceDriver, DeviceSession, Transport};
use crate::transport::{TransportType, TransportConfig, TransportError};

/// Events for device connection lifecycle
#[derive(Debug, Clone)]
pub enum ConnectionEvent {
    /// Device detected (hot-plug or discovery)
    DeviceDetected {
        device_id: String,
        transport_type: TransportType,
        address: String,
        metadata: HashMap<String, String>,
    },
    
    /// Connection initiated
    ConnectionInitiated {
        device_id: String,
        session_id: String,
    },
    
    /// Connection established
    ConnectionEstablished {
        device_id: String,
        session_id: String,
        driver_name: String,
    },
    
    /// Connection lost
    ConnectionLost {
        device_id: String,
        session_id: String,
        reason: String,
    },
    
    /// Reconnection attempt
    ReconnectionAttempt {
        device_id: String,
        attempt: u32,
        max_attempts: u32,
    },
    
    /// Reconnection successful
    ReconnectionSuccessful {
        device_id: String,
        session_id: String,
        attempts: u32,
    },
    
    /// Device removed (unplugged)
    DeviceRemoved {
        device_id: String,
    },
    
    /// Connection error
    ConnectionError {
        device_id: String,
        error: String,
        recoverable: bool,
    },
}

/// Connection state for a device
#[derive(Debug, Clone)]
pub struct ConnectionState {
    pub device_id: String,
    pub transport_type: TransportType,
    pub address: String,
    pub session_id: Option<String>,
    pub driver_name: Option<String>,
    pub connected: bool,
    pub reconnect_attempts: u32,
    pub last_error: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Connection manager for device lifecycle
pub struct ConnectionManager {
    /// Active connections
    connections: Arc<RwLock<HashMap<String, ConnectionState>>>,
    
    /// Active sessions
    sessions: Arc<RwLock<HashMap<String, Box<dyn DeviceSession>>>>,
    
    /// Event channel
    event_tx: mpsc::UnboundedSender<ConnectionEvent>,
    event_rx: Arc<RwLock<mpsc::UnboundedReceiver<ConnectionEvent>>>,
    
    /// Reconnection configuration
    max_reconnect_attempts: u32,
    reconnect_delay_ms: u64,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new() -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        
        ConnectionManager {
            connections: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            event_tx,
            event_rx: Arc::new(RwLock::new(event_rx)),
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 1000,
        }
    }
    
    /// Get event receiver
    pub fn event_receiver(&self) -> mpsc::UnboundedReceiver<ConnectionEvent> {
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Clone existing events to new receiver
        let event_tx = tx.clone();
        let event_rx = self.event_rx.clone();
        
        tokio::spawn(async move {
            let mut rx = event_rx.write().await;
            while let Some(event) = rx.recv().await {
                let _ = event_tx.send(event);
            }
        });
        
        rx
    }
    
    /// Register a device detection
    pub async fn register_device(
        &self,
        transport_type: TransportType,
        address: String,
        metadata: HashMap<String, String>,
    ) -> String {
        let device_id = format!("{:?}_{}", transport_type, address);
        
        let state = ConnectionState {
            device_id: device_id.clone(),
            transport_type,
            address: address.clone(),
            session_id: None,
            driver_name: None,
            connected: false,
            reconnect_attempts: 0,
            last_error: None,
            metadata: metadata.clone(),
        };
        
        let mut connections = self.connections.write().await;
        connections.insert(device_id.clone(), state);
        
        // Send detection event
        let _ = self.event_tx.send(ConnectionEvent::DeviceDetected {
            device_id: device_id.clone(),
            transport_type,
            address,
            metadata,
        });
        
        info!("Registered device: {}", device_id);
        device_id
    }
    
    /// Connect to a device
    pub async fn connect_device(
        &self,
        device_id: &str,
        transport: Arc<dyn Transport>,
        driver: Arc<dyn DeviceDriver>,
    ) -> DeviceResult<String> {
        // Generate session ID
        let session_id = Uuid::new_v4().to_string();
        
        // Send connection initiated event
        let _ = self.event_tx.send(ConnectionEvent::ConnectionInitiated {
            device_id: device_id.to_string(),
            session_id: session_id.clone(),
        });
        
        // Open device session
        match driver.open_async(transport).await {
            Ok(session) => {
                // Update connection state
                let mut connections = self.connections.write().await;
                if let Some(state) = connections.get_mut(device_id) {
                    state.session_id = Some(session_id.clone());
                    state.driver_name = Some(driver.name().to_string());
                    state.connected = true;
                    state.reconnect_attempts = 0;
                    state.last_error = None;
                }
                
                // Store session
                let mut sessions = self.sessions.write().await;
                sessions.insert(session_id.clone(), session);
                
                // Send connection established event
                let _ = self.event_tx.send(ConnectionEvent::ConnectionEstablished {
                    device_id: device_id.to_string(),
                    session_id: session_id.clone(),
                    driver_name: driver.name().to_string(),
                });
                
                info!("Connected to device {} with session {}", device_id, session_id);
                Ok(session_id)
            }
            Err(e) => {
                // Update connection state with error
                let mut connections = self.connections.write().await;
                if let Some(state) = connections.get_mut(device_id) {
                    state.last_error = Some(e.to_string());
                }
                
                // Send error event
                let recoverable = !matches!(e, 
                    DeviceError::PermissionDenied(_) | 
                    DeviceError::UnsupportedDevice(_)
                );
                
                let _ = self.event_tx.send(ConnectionEvent::ConnectionError {
                    device_id: device_id.to_string(),
                    error: e.to_string(),
                    recoverable,
                });
                
                warn!("Failed to connect to device {}: {}", device_id, e);
                Err(e)
            }
        }
    }
    
    /// Disconnect a device
    pub async fn disconnect_device(&self, device_id: &str) -> DeviceResult<()> {
        let mut connections = self.connections.write().await;
        
        if let Some(state) = connections.get_mut(device_id) {
            if let Some(session_id) = &state.session_id {
                // Close session
                let mut sessions = self.sessions.write().await;
                if let Some(mut session) = sessions.remove(session_id) {
                    session.close_async().await?;
                }
                
                // Send disconnection event
                let _ = self.event_tx.send(ConnectionEvent::ConnectionLost {
                    device_id: device_id.to_string(),
                    session_id: session_id.clone(),
                    reason: "User requested disconnect".to_string(),
                });
            }
            
            // Update state
            state.connected = false;
            state.session_id = None;
            
            info!("Disconnected device: {}", device_id);
        }
        
        Ok(())
    }
    
    /// Handle device removal (hot-unplug)
    pub async fn handle_device_removed(&self, device_id: &str) {
        let mut connections = self.connections.write().await;
        
        if let Some(state) = connections.remove(device_id) {
            // Close session if active
            if let Some(session_id) = &state.session_id {
                let mut sessions = self.sessions.write().await;
                if let Some(mut session) = sessions.remove(session_id) {
                    let _ = session.close_async().await;
                }
            }
            
            // Send removal event
            let _ = self.event_tx.send(ConnectionEvent::DeviceRemoved {
                device_id: device_id.to_string(),
            });
            
            info!("Device removed: {}", device_id);
        }
    }
    
    /// Handle connection lost (unexpected disconnect)
    pub async fn handle_connection_lost(
        &self,
        device_id: &str,
        reason: String,
        auto_reconnect: bool,
    ) {
        let mut connections = self.connections.write().await;
        
        if let Some(state) = connections.get_mut(device_id) {
            let session_id = state.session_id.clone();
            state.connected = false;
            
            if let Some(session_id) = session_id {
                // Send connection lost event
                let _ = self.event_tx.send(ConnectionEvent::ConnectionLost {
                    device_id: device_id.to_string(),
                    session_id: session_id.clone(),
                    reason: reason.clone(),
                });
                
                // Remove session
                let mut sessions = self.sessions.write().await;
                sessions.remove(&session_id);
            }
            
            warn!("Connection lost for device {}: {}", device_id, reason);
            
            // Trigger reconnection if enabled
            if auto_reconnect && state.reconnect_attempts < self.max_reconnect_attempts {
                drop(connections); // Release lock before spawning task
                self.trigger_reconnection(device_id).await;
            }
        }
    }
    
    /// Trigger automatic reconnection
    async fn trigger_reconnection(&self, device_id: &str) {
        let connections = self.connections.clone();
        let event_tx = self.event_tx.clone();
        let max_attempts = self.max_reconnect_attempts;
        let delay_ms = self.reconnect_delay_ms;
        let device_id = device_id.to_string();
        
        tokio::spawn(async move {
            let mut attempt = 0;
            
            loop {
                attempt += 1;
                
                // Check if we should continue
                let should_continue = {
                    let conns = connections.read().await;
                    if let Some(state) = conns.get(&device_id) {
                        !state.connected && attempt <= max_attempts
                    } else {
                        false
                    }
                };
                
                if !should_continue {
                    break;
                }
                
                // Send reconnection attempt event
                let _ = event_tx.send(ConnectionEvent::ReconnectionAttempt {
                    device_id: device_id.clone(),
                    attempt,
                    max_attempts,
                });
                
                // Wait with exponential backoff
                let delay = delay_ms * 2u64.pow(attempt - 1);
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                
                // TODO: Actual reconnection logic would go here
                // This would involve recreating the transport and calling connect_device
                
                // For now, we just update the attempt counter
                let mut conns = connections.write().await;
                if let Some(state) = conns.get_mut(&device_id) {
                    state.reconnect_attempts = attempt;
                }
            }
        });
    }
    
    /// Get current connection states
    pub async fn get_connection_states(&self) -> Vec<ConnectionState> {
        let connections = self.connections.read().await;
        connections.values().cloned().collect()
    }
    
    /// Get active session
    pub async fn get_session(&self, session_id: &str) -> Option<Arc<RwLock<Box<dyn DeviceSession>>>> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).map(|_| {
            // This is a limitation - we can't return a reference to the session
            // Would need to refactor to use Arc<RwLock<>> for sessions
            Arc::new(RwLock::new(Box::new(()) as Box<dyn DeviceSession>))
        })
    }
    
    /// Check if a device is connected
    pub async fn is_connected(&self, device_id: &str) -> bool {
        let connections = self.connections.read().await;
        connections.get(device_id).map(|s| s.connected).unwrap_or(false)
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_connection_manager() {
        let manager = ConnectionManager::new();
        
        // Register a device
        let device_id = manager.register_device(
            TransportType::Serial,
            "COM3".to_string(),
            HashMap::new(),
        ).await;
        
        assert!(!manager.is_connected(&device_id).await);
        
        // Get connection states
        let states = manager.get_connection_states().await;
        assert_eq!(states.len(), 1);
        assert_eq!(states[0].device_id, device_id);
    }
}