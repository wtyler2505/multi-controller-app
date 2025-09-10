use async_trait::async_trait;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use std::path::PathBuf;
use tokio::task::JoinHandle;
use crate::transport::{
    Transport, TransportBase, TransportConfig, TransportError, TransportResult, 
    TransportStats, TransportType, ConnectionState
};
use crate::transport::ssh_keys::{SshKeyManager, SshKeyInfo};

/// Authentication method for SSH
#[derive(Debug, Clone)]
enum AuthMethod {
    Password(String),
    Key {
        path: PathBuf,
        passphrase: Option<String>,
    },
}

/// SSH transport implementation for secure remote connections
pub struct SshTransport {
    base: TransportBase,
    session: Arc<Mutex<Option<MockSshSession>>>, // Thread-safe session management
    reconnect_attempts: Arc<Mutex<u32>>,         // Thread-safe mutable state
    max_reconnect_attempts: u32,                 // Immutable configuration
    base_reconnect_delay: Duration,              // Immutable configuration
    task_handles: Arc<Mutex<Vec<JoinHandle<()>>>>, // Track spawned tasks for cleanup
    cleanup_flag: Arc<AtomicBool>,               // Signal for cooperative shutdown
    key_manager: SshKeyManager,                  // SSH key discovery and management
    resolved_key: Option<SshKeyInfo>,            // Resolved SSH key for authentication
}

impl SshTransport {
    /// Create a new SSH transport
    pub fn new(config: TransportConfig) -> TransportResult<Self> {
        // Validate configuration
        if let crate::transport::common::TransportSettings::Ssh(ref settings) = config.settings {
            if settings.port == 0 {
                return Err(TransportError::ConfigError("Invalid SSH port".into()));
            }
            if settings.username.is_empty() {
                return Err(TransportError::ConfigError("SSH username required".into()));
            }
        } else {
            return Err(TransportError::ConfigError("Invalid settings for SSH transport".into()));
        }
        
        // Initialize key manager
        let mut key_manager = SshKeyManager::new();
        
        // Resolve SSH key for authentication
        let resolved_key = if let crate::transport::common::TransportSettings::Ssh(ref settings) = config.settings {
            if let Some(ref key_path) = settings.key_path {
                // Use specified key path
                match key_manager.load_key(&PathBuf::from(key_path)) {
                    Ok(key) => {
                        tracing::info!("Using specified SSH key: {}", key_path);
                        Some(key)
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load specified SSH key {}: {}", key_path, e);
                        None
                    }
                }
            } else {
                // Auto-discover best available key
                if let Some(key) = key_manager.find_best_key() {
                    tracing::info!("Auto-discovered SSH key: {}", key.path.display());
                    Some(key)
                } else {
                    tracing::info!("No SSH key found, will use password authentication if available");
                    None
                }
            }
        } else {
            None
        };
        
        Ok(SshTransport {
            base: TransportBase::new(
                format!("SSH:{}", config.address),
                TransportType::Ssh,
                config,
            ),
            session: Arc::new(Mutex::new(None)),
            reconnect_attempts: Arc::new(Mutex::new(0)),
            max_reconnect_attempts: 5, // Fewer attempts for SSH due to longer timeouts
            base_reconnect_delay: Duration::from_secs(1), // Longer base delay for SSH
            task_handles: Arc::new(Mutex::new(Vec::new())),
            cleanup_flag: Arc::new(AtomicBool::new(false)),
            key_manager,
            resolved_key,
        })
    }
    
    /// Try to connect with exponential backoff using shared module
    async fn connect_with_backoff(&self) -> TransportResult<()> {
        let mut backoff = crate::transport::backoff::ExponentialBackoff::from_config(
            self.base.config.max_reconnect_attempts,
            self.base.config.reconnect_delay_ms,
        );
        
        while backoff.should_retry() {
            match self.try_connect().await {
                Ok(()) => {
                    *self.reconnect_attempts.lock().await = 0;
                    return Ok(());
                }
                Err(e) => {
                    // Check if error is retryable
                    if !crate::transport::backoff::is_retryable_error(&e) {
                        tracing::error!("Non-retryable SSH error: {}", e);
                        return Err(e);
                    }
                    
                    if let Some(delay) = backoff.next_delay() {
                        tracing::warn!(
                            "SSH connection failed (attempt {}/{}), retrying in {:?}: {}",
                            backoff.current_attempt(),
                            backoff.remaining_attempts().unwrap_or(999),
                            delay,
                            e
                        );
                        
                        tokio::time::sleep(delay).await;
                    } else {
                        return Err(TransportError::ConnectionFailed(
                            format!("Max reconnection attempts ({}) exceeded", backoff.current_attempt())
                        ));
                    }
                }
            }
        }
        
        Err(TransportError::ConnectionFailed("Max reconnection attempts exceeded".into()))
    }
    
    /// Attempt a single connection (extracted for reuse)
    async fn try_connect(&self) -> TransportResult<()> {
        // Extract SSH settings
        let settings = if let crate::transport::common::TransportSettings::Ssh(ref settings) = self.base.config.settings {
            settings.clone()
        } else {
            return Err(TransportError::ConfigError("Invalid SSH settings".into()));
        };
        
        // Determine authentication method
        let auth_method = if let Some(ref key_info) = self.resolved_key {
            // Use SSH key authentication
            if key_info.is_encrypted && settings.key_passphrase.is_none() {
                tracing::warn!("SSH key is encrypted but no passphrase provided");
            }
            AuthMethod::Key {
                path: key_info.path.clone(),
                passphrase: settings.key_passphrase.clone(),
            }
        } else if let Some(ref password) = settings.password {
            // Use password authentication
            AuthMethod::Password(password.clone())
        } else {
            // No authentication method available
            return Err(TransportError::PermissionDenied(
                "No SSH authentication method available (no key or password)".into()
            ));
        };
        
        // TODO: Implement actual SSH connection with real SSH library
        // For now, create a mock session with authentication info
        let mock_session = MockSshSession::new_with_auth(
            &self.base.config.address,
            &settings.username,
            auth_method,
            settings.port,
        )?;
        
        *self.session.lock().await = Some(mock_session);
        
        tracing::info!("Connected SSH session to {} using {}", 
            self.base.config.address,
            if self.resolved_key.is_some() { "key authentication" } else { "password authentication" }
        );
        Ok(())
    }
    
    /// Trigger automatic reconnection in the background
    async fn trigger_auto_reconnection(&self) {
        let address = self.base.config.address.clone();
        
        // Create a closure that attempts to connect
        let connect_fn = move || -> std::pin::Pin<Box<dyn std::future::Future<Output = TransportResult<()>> + Send>> {
            let addr = address.clone();
            Box::pin(async move {
                // TODO: Implement actual SSH connection
                // For now, simulate connection attempt
                tracing::info!("Mock SSH reconnection to {}", addr);
                Ok(())
            })
        };
        
        // Trigger reconnection through TransportBase
        if let Err(e) = self.base.trigger_reconnection(connect_fn).await {
            tracing::error!("Failed to trigger SSH reconnection: {}", e);
        }
    }
    
    /// Test SSH connection by running a simple command
    pub async fn test_connection(&self) -> TransportResult<bool> {
        if !self.is_connected() {
            return Ok(false);
        }
        
        // Try to run 'echo test' command
        match self.send(b"echo test\n").await {
            Ok(_) => {
                match self.receive(Duration::from_secs(5)).await {
                    Ok(response) => {
                        Ok(response.windows(4).any(|w| w == b"test"))
                    }
                    Err(_) => Ok(false)
                }
            }
            Err(_) => Ok(false)
        }
    }
}

#[async_trait]
impl Transport for SshTransport {
    fn transport_type(&self) -> TransportType {
        self.base.transport_type
    }
    
    fn name(&self) -> &str {
        &self.base.name
    }
    
    fn is_connected(&self) -> bool {
        self.session.lock().await.is_some()
    }
    
    async fn connect(&self) -> TransportResult<()> {
        if self.is_connected() {
            return Err(TransportError::AlreadyConnected);
        }
        
        self.base.set_state(ConnectionState::Connecting).await;
        
        // Connect with exponential backoff
        match self.connect_with_backoff().await {
            Ok(()) => {
                self.base.set_state(ConnectionState::Connected).await;
                self.base.update_stats(|stats| {
                    stats.reconnect_count += *self.reconnect_attempts.lock().await;
                }).await;
                Ok(())
            }
            Err(e) => {
                self.base.set_state(ConnectionState::Error).await;
                Err(e)
            }
        }
    }
    
    async fn disconnect(&self) -> TransportResult<()> {
        if !self.is_connected() {
            return Ok(());
        }
        
        // Clean up all resources before disconnecting
        self.cleanup_resources().await?;
        
        tracing::info!("Disconnected SSH session from {}", self.base.config.address);
        Ok(())
    }
    
    async fn send(&self, data: &[u8]) -> TransportResult<()> {
        let start = Instant::now();
        
        // Check connection and reconnect if needed (before creating guard)
        if !self.is_connected() && self.base.config.auto_reconnect {
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            
            self.base.set_state(ConnectionState::Reconnecting).await;
            self.connect().await?;
            return self.send(data).await;
        }
        
        let mut session_guard = self.session.lock().await;
        if let Some(ref mut session) = *session_guard {
            match session.execute(data) {
                Ok(_) => {
                    drop(session_guard); // Explicitly drop the lock before async operations
                    
                    self.base.update_stats(|stats| {
                        stats.bytes_sent += data.len() as u64;
                        stats.transactions_success += 1;
                    }).await;
                    
                    // Enforce minimum latency requirement (100ms for network)
                    self.base.enforce_latency(start).await?;
                    
                    Ok(())
                }
                Err(e) => {
                    let err_str = e.to_string();
                    drop(session_guard); // Explicitly drop the lock before modifying self
                    
                    self.base.update_stats(|stats| {
                        stats.transactions_failed += 1;
                        stats.last_error = Some(err_str.clone());
                    }).await;
                    
                    // Connection lost, clear session and trigger reconnection
                    *self.session.lock().await = None;
                    self.base.set_state(ConnectionState::Disconnected).await;
                    
                    // Trigger automatic reconnection if enabled
                    if self.base.config.auto_reconnect && crate::transport::backoff::is_retryable_error(&e) {
                        self.trigger_auto_reconnection().await;
                    }
                    
                    Err(e)
                }
            }
        } else {
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            Err(TransportError::NotConnected)
        }
    }
    
    async fn receive(&self, timeout: Duration) -> TransportResult<Vec<u8>> {
        let start = Instant::now();
        
        let mut session_guard = self.session.lock().await;
        if let Some(ref mut session) = *session_guard {
            
            // Set up timeout
            let deadline = Instant::now() + timeout;
            
            // Try to read data
            let data = session.read_output();
            drop(session_guard); // Release lock before async operations
            
            self.base.update_stats(|stats| {
                stats.bytes_received += data.len() as u64;
            }).await;
            
            // Enforce minimum latency
            self.base.enforce_latency(start).await?;
            
            Ok(data)
        } else {
            self.base.update_stats(|stats| {
                stats.transactions_failed += 1;
                stats.last_error = Some("Not connected".into());
            }).await;
            Err(TransportError::NotConnected)
        }
    }
    
    fn stats(&self) -> TransportStats {
        TransportStats::default()
    }
    
    async fn reset(&self) -> TransportResult<()> {
        let mut session_guard = self.session.lock().await;
        if let Some(ref mut session) = *session_guard {
            session.reset()?;
            Ok(())
        } else {
            Err(TransportError::NotConnected)
        }
    }
    
    fn config(&self) -> &TransportConfig {
        &self.base.config
    }
    
    async fn cleanup_resources(&self) -> TransportResult<()> {
        // Cancel any active reconnection attempts
        self.base.cancel_reconnection().await;
        
        // Signal shutdown to any cooperative tasks
        self.cleanup_flag.store(true, Ordering::Relaxed);
        
        // Abort all spawned tasks
        {
            let mut handles = self.task_handles.lock().await;
            for handle in handles.drain(..) {
                handle.abort();
            }
        }
        
        // Drop the SSH session
        *self.session.lock().await = None;
        
        // Reset the cleanup flag for next connection
        self.cleanup_flag.store(false, Ordering::Relaxed);
        
        // Reset reconnect attempts counter
        *self.reconnect_attempts.lock().await = 0;
        
        // Update state
        self.base.set_state(ConnectionState::Disconnected).await;
        
        tracing::debug!("SSH transport resources cleaned up");
        Ok(())
    }
}

/// Mock SSH session for testing (will be replaced with real implementation)
struct MockSshSession {
    host: String,
    username: String,
    auth_method: AuthMethod,
    port: u16,
    command_buffer: Vec<u8>,
    output_buffer: Vec<u8>,
}

impl MockSshSession {
    fn new(host: &str, username: &str, key_path: Option<&str>, port: u16) -> TransportResult<Self> {
        // Convert old API to new auth method for compatibility
        let auth_method = if let Some(key) = key_path {
            AuthMethod::Key {
                path: PathBuf::from(key),
                passphrase: None,
            }
        } else {
            AuthMethod::Password("default".to_string())
        };
        
        Self::new_with_auth(host, username, auth_method, port)
    }
    
    fn new_with_auth(host: &str, username: &str, auth_method: AuthMethod, port: u16) -> TransportResult<Self> {
        // Simulate connection failure for some hosts
        if host == "unreachable.local" {
            return Err(TransportError::ConnectionFailed("Host unreachable".into()));
        }
        
        // Simulate authentication validation
        match &auth_method {
            AuthMethod::Key { path, .. } => {
                // Simulate key validation
                if !path.exists() && !path.to_string_lossy().contains("mock") {
                    return Err(TransportError::PermissionDenied(
                        format!("SSH key not found: {}", path.display())
                    ));
                }
                tracing::debug!("Mock SSH: Using key authentication with {}", path.display());
            }
            AuthMethod::Password(password) => {
                // Simulate password validation
                if username == "root" && password == "default" {
                    return Err(TransportError::PermissionDenied(
                        "Root login with default password forbidden".into()
                    ));
                }
                tracing::debug!("Mock SSH: Using password authentication");
            }
        }
        
        Ok(MockSshSession {
            host: host.to_string(),
            username: username.to_string(),
            auth_method,
            port,
            command_buffer: Vec::new(),
            output_buffer: Vec::new(),
        })
    }
    
    fn execute(&mut self, command: &[u8]) -> TransportResult<()> {
        self.command_buffer.extend_from_slice(command);
        
        // Simulate command execution
        if command.starts_with(b"echo ") {
            // Echo command - return the text
            let text = &command[5..];
            self.output_buffer.extend_from_slice(text);
        } else if command.starts_with(b"gpio ") {
            // GPIO command simulation
            self.output_buffer.extend_from_slice(b"OK\n");
        } else if command.starts_with(b"reboot") {
            // Simulate connection loss on reboot
            return Err(TransportError::ConnectionFailed("Connection lost during reboot".into()));
        }
        
        Ok(())
    }
    
    fn read_output(&mut self) -> Vec<u8> {
        if !self.output_buffer.is_empty() {
            let output = self.output_buffer.clone();
            self.output_buffer.clear();
            return output;
        }
        
        // Return some mock telemetry data if no command output
        vec![0x01, 0x02, 0x03, 0x04]
    }
    
    fn reset(&mut self) -> TransportResult<()> {
        self.command_buffer.clear();
        self.output_buffer.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::common::{SshSettings, TransportSettings};
    
    #[tokio::test]
    async fn test_ssh_transport_creation() {
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "pi".to_string(),
                key_path: Some("/home/user/.ssh/id_rsa".to_string()),
                password: None,
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let transport = SshTransport::new(config);
        assert!(transport.is_ok());
    }
    
    #[tokio::test]
    async fn test_ssh_validation() {
        // Test invalid port
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "pi".to_string(),
                key_path: None,
                password: None,
                port: 0,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let transport = SshTransport::new(config);
        assert!(transport.is_err());
        
        // Test empty username
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "".to_string(),
                key_path: None,
                password: None,
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let transport = SshTransport::new(config);
        assert!(transport.is_err());
    }
    
    #[tokio::test]
    async fn test_ssh_connect_disconnect() {
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "pi".to_string(),
                key_path: Some("/home/user/.ssh/id_rsa".to_string()),
                password: None,
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let mut transport = SshTransport::new(config).unwrap();
        
        // Should not be connected initially
        assert!(!transport.is_connected());
        
        // Connect
        let result = transport.connect().await;
        assert!(result.is_ok());
        assert!(transport.is_connected());
        
        // Try to connect again (should fail)
        let result = transport.connect().await;
        assert!(matches!(result, Err(TransportError::AlreadyConnected)));
        
        // Disconnect
        let result = transport.disconnect().await;
        assert!(result.is_ok());
        assert!(!transport.is_connected());
    }
    
    #[tokio::test]
    async fn test_ssh_command_execution() {
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "pi".to_string(),
                key_path: Some("/home/user/.ssh/id_rsa".to_string()),
                password: None,
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let mut transport = SshTransport::new(config).unwrap();
        transport.connect().await.unwrap();
        
        // Send echo command
        let result = transport.send(b"echo hello\n").await;
        assert!(result.is_ok());
        
        // Receive response
        let result = transport.receive(Duration::from_secs(1)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), b"hello\n");
    }
    
    #[tokio::test]
    async fn test_ssh_permission_denied() {
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "root".to_string(),
                key_path: None,
                password: Some("default".to_string()), // Default password for root = permission denied
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let mut transport = SshTransport::new(config).unwrap();
        let result = transport.connect().await;
        assert!(matches!(result, Err(TransportError::PermissionDenied(_))));
    }
    
    #[tokio::test]
    async fn test_ssh_key_authentication() {
        // Test with specified key path
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "pi".to_string(),
                key_path: Some("/mock/path/id_ed25519".to_string()),
                password: None,
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let transport = SshTransport::new(config).unwrap();
        assert!(transport.resolved_key.is_some());
    }
    
    #[tokio::test]
    async fn test_ssh_password_fallback() {
        // Test with password when no key is available
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "pi".to_string(),
                key_path: None,
                password: Some("secure_password".to_string()),
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let mut transport = SshTransport::new(config).unwrap();
        assert!(transport.resolved_key.is_none());
        
        // Should connect successfully with password
        let result = transport.connect().await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_ssh_no_auth_method() {
        // Test failure when no authentication method is available
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "pi".to_string(),
                key_path: None,
                password: None,  // No password and no key
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let mut transport = SshTransport::new(config).unwrap();
        let result = transport.connect().await;
        assert!(matches!(result, Err(TransportError::PermissionDenied(_))));
    }
    
    #[tokio::test]
    async fn test_ssh_auto_reconnect() {
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "pi".to_string(),
                key_path: Some("/home/user/.ssh/id_rsa".to_string()),
                password: None,
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let mut transport = SshTransport::new(config).unwrap();
        
        // Should not be connected initially
        assert!(!transport.is_connected());
        
        // Send should trigger auto-reconnect
        let result = transport.send(b"echo test\n").await;
        assert!(result.is_ok());
        assert!(transport.is_connected());
    }
    
    #[tokio::test]
    async fn test_ssh_test_connection() {
        let config = TransportConfig {
            transport_type: TransportType::Ssh,
            address: "192.168.1.100".to_string(),
            settings: TransportSettings::Ssh(SshSettings {
                username: "pi".to_string(),
                key_path: Some("/home/user/.ssh/id_rsa".to_string()),
                password: None,
                port: 22,
                compression: false,
                strict_host_key_checking: false,
                known_hosts_path: None,
                key_passphrase: None,
            }),
            ..Default::default()
        };
        
        let mut transport = SshTransport::new(config).unwrap();
        
        // Test when not connected
        let result = transport.test_connection().await.unwrap();
        assert!(!result);
        
        // Connect and test
        transport.connect().await.unwrap();
        let result = transport.test_connection().await.unwrap();
        assert!(result);
    }
}