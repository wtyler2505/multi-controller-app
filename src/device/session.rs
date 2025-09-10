use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::device::{DeviceResult, DeviceError};

/// Device session interface (equivalent to IDeviceSession)
/// Represents an active connection to a device
#[async_trait]
pub trait DeviceSession: Send + Sync {
    /// Get session ID
    fn session_id(&self) -> &str;
    
    /// Get device name/identifier
    fn device_name(&self) -> &str;
    
    /// Invoke a device endpoint/method
    /// Returns the response as a JSON value
    async fn invoke_async(&mut self, endpoint: &str, args: Vec<Value>) -> DeviceResult<Value>;
    
    /// Subscribe to a data stream from the device
    /// Returns a subscription handle that must be kept alive
    async fn subscribe_async(
        &mut self, 
        stream: &str,
        handler: mpsc::UnboundedSender<StreamData>,
    ) -> DeviceResult<SubscriptionHandle>;
    
    /// Close the session
    async fn close_async(&mut self) -> DeviceResult<()>;
    
    /// Check if session is active
    fn is_active(&self) -> bool;
    
    /// Get session statistics
    fn statistics(&self) -> SessionStatistics;
    
    /// Send raw command (for debugging/direct control)
    async fn send_raw(&mut self, data: &[u8]) -> DeviceResult<Vec<u8>>;
}

/// Device endpoint descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceEndpoint {
    /// Endpoint name (e.g., "set_pwm", "read_sensor")
    pub name: String,
    
    /// Human-readable description
    pub description: String,
    
    /// Parameter schema (JSON Schema format)
    pub parameters: Value,
    
    /// Return type schema
    pub returns: Value,
    
    /// Whether this endpoint modifies device state
    pub mutates_state: bool,
    
    /// Minimum interval between calls (ms)
    pub rate_limit_ms: Option<u32>,
}

/// Stream data packet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamData {
    /// Stream name
    pub stream: String,
    
    /// Timestamp (milliseconds since epoch)
    pub timestamp: u64,
    
    /// Data payload
    pub data: Value,
    
    /// Sequence number for ordering
    pub sequence: u64,
}

/// Subscription handle - drop to unsubscribe
pub struct SubscriptionHandle {
    id: String,
    unsubscribe: Option<mpsc::Sender<String>>,
}

impl SubscriptionHandle {
    pub fn new(id: String, unsubscribe: mpsc::Sender<String>) -> Self {
        SubscriptionHandle {
            id: id.clone(),
            unsubscribe: Some(unsubscribe),
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Drop for SubscriptionHandle {
    fn drop(&mut self) {
        if let Some(tx) = self.unsubscribe.take() {
            let _ = tx.try_send(self.id.clone());
        }
    }
}

/// Session statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SessionStatistics {
    /// Total bytes sent
    pub bytes_sent: u64,
    
    /// Total bytes received  
    pub bytes_received: u64,
    
    /// Total commands sent
    pub commands_sent: u64,
    
    /// Total responses received
    pub responses_received: u64,
    
    /// Number of errors
    pub error_count: u64,
    
    /// Average latency (ms)
    pub avg_latency_ms: f64,
    
    /// Min latency (ms)
    pub min_latency_ms: u64,
    
    /// Max latency (ms)
    pub max_latency_ms: u64,
    
    /// Session start time (unix timestamp)
    pub start_time: u64,
    
    /// Last activity time (unix timestamp)
    pub last_activity: u64,
}

impl SessionStatistics {
    pub fn new() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        SessionStatistics {
            start_time: now,
            last_activity: now,
            ..Default::default()
        }
    }
    
    pub fn update_latency(&mut self, latency_ms: u64) {
        self.min_latency_ms = if self.min_latency_ms == 0 {
            latency_ms
        } else {
            self.min_latency_ms.min(latency_ms)
        };
        
        self.max_latency_ms = self.max_latency_ms.max(latency_ms);
        
        // Update running average
        let n = self.responses_received as f64;
        self.avg_latency_ms = if n > 0.0 {
            (self.avg_latency_ms * n + latency_ms as f64) / (n + 1.0)
        } else {
            latency_ms as f64
        };
    }
}