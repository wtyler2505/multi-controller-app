use crate::{Result, DeviceInfo, async_trait, Uuid};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Unique identifier for device sessions
pub type SessionId = Uuid;

/// Connection state of a device session
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Error,
}

/// Information about an active device session
#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub session_id: SessionId,
    pub device_info: DeviceInfo,
    pub state: ConnectionState,
    pub created_at: u64,
    pub last_activity: Option<u64>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl SessionInfo {
    pub fn new(session_id: SessionId, device_info: DeviceInfo) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
            
        Self {
            session_id,
            device_info,
            state: ConnectionState::Disconnected,
            created_at: now,
            last_activity: None,
            bytes_sent: 0,
            bytes_received: 0,
        }
    }
}

/// The DeviceSession trait defines the interface for active device connections
#[async_trait]
pub trait DeviceSession: Send + Sync {
    /// Get the unique session identifier
    fn session_id(&self) -> SessionId;
    
    /// Get information about this session
    fn session_info(&self) -> SessionInfo;
    
    /// Get the current connection state
    fn connection_state(&self) -> ConnectionState;
    
    /// Invoke a device endpoint with arguments
    async fn invoke(&mut self, endpoint: &str, args: Vec<serde_json::Value>) -> Result<serde_json::Value>;
    
    /// Subscribe to a data stream from the device
    async fn subscribe(&mut self, stream: &str, handler: Box<dyn Fn(&[u8]) + Send + Sync>) -> Result<SubscriptionHandle>;
    
    /// Unsubscribe from a data stream
    async fn unsubscribe(&mut self, handle: SubscriptionHandle) -> Result<()>;
    
    /// Send raw data to the device
    async fn send_raw(&mut self, data: &[u8]) -> Result<usize>;
    
    /// Receive raw data from the device
    async fn receive_raw(&mut self, buffer: &mut [u8]) -> Result<usize>;
    
    /// Perform cleanup operations before closing the session
    async fn cleanup_resources(&mut self) -> Result<()>;
    
    /// Close the device session
    async fn close(mut self: Box<Self>) -> Result<()> {
        self.cleanup_resources().await?;
        Ok(())
    }
}

/// Handle for managing data stream subscriptions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SubscriptionHandle(pub Uuid);

impl SubscriptionHandle {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}