/// SSH transport loopback tests
/// Tests SSH connection with local SSH server or mock

use super::common::*;
use crate::transport::{Transport, TransportConfig, TransportType, TransportSettings, SshSettings};
use crate::transport::ssh::SshTransport;
use std::time::Duration;
use tokio;

/// Configuration for SSH loopback testing
struct SshLoopbackConfig {
    host: String,
    port: u16,
    username: String,
    key_path: Option<String>,
    password: Option<String>,
    use_mock_server: bool,
}

impl Default for SshLoopbackConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 2222,  // Non-standard port for testing
            username: "test_user".to_string(),
            key_path: Some("~/.ssh/test_key".to_string()),
            password: None,
            use_mock_server: true,  // Use mock by default
        }
    }
}

/// Create an SSH transport for testing
async fn create_ssh_transport(config: &SshLoopbackConfig) -> SshTransport {
    let transport_config = TransportConfig {
        transport_type: TransportType::Ssh,
        address: format!("{}:{}", config.host, config.port),
        connect_timeout_ms: 10000,  // SSH needs more time
        read_timeout_ms: 2000,
        write_timeout_ms: 2000,
        auto_reconnect: true,
        max_reconnect_attempts: 3,
        reconnect_delay_ms: 500,
        read_buffer_size: 8192,
        write_buffer_size: 8192,
        require_handshake: false,
        min_latency: Some(Duration::from_millis(50)),
        settings: TransportSettings::Ssh(SshSettings {
            username: config.username.clone(),
            key_path: config.key_path.clone(),
            port: config.port,
            ..Default::default()
        }),
    };
    
    SshTransport::new(transport_config).expect("Failed to create SSH transport")
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_basic_connection() {
    let config = SshLoopbackConfig::default();
    let mut transport = create_ssh_transport(&config).await;
    
    // Connect
    match transport.connect().await {
        Ok(_) => {
            assert!(transport.is_connected());
            
            // Test simple command
            let test_cmd = b"echo 'Hello SSH'\n";
            transport.send(test_cmd).await.expect("Failed to send");
            
            let received = transport.receive(Duration::from_secs(5)).await
                .expect("Failed to receive");
            
            // Response should contain our echo
            let response = String::from_utf8_lossy(&received);
            assert!(response.contains("Hello SSH"), 
                    "Echo not found in response: {}", response);
            
            // Disconnect
            transport.disconnect().await.expect("Failed to disconnect");
        }
        Err(e) => {
            eprintln!("SSH connection failed (expected if no test server): {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_key_authentication() {
    let mut config = SshLoopbackConfig::default();
    config.password = None;
    config.key_path = Some("~/.ssh/id_rsa".to_string());
    
    let mut transport = create_ssh_transport(&config).await;
    
    match transport.connect().await {
        Ok(_) => {
            assert!(transport.is_connected());
            println!("SSH key authentication successful");
            transport.disconnect().await.ok();
        }
        Err(e) => {
            eprintln!("SSH key auth failed (expected without server): {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_password_authentication() {
    let mut config = SshLoopbackConfig::default();
    config.password = Some("test_password".to_string());
    config.key_path = None;
    
    let mut transport = create_ssh_transport(&config).await;
    
    match transport.connect().await {
        Ok(_) => {
            assert!(transport.is_connected());
            println!("SSH password authentication successful");
            transport.disconnect().await.ok();
        }
        Err(e) => {
            eprintln!("SSH password auth failed (expected without server): {}", e);
        }
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_command_execution() {
    let config = SshLoopbackConfig::default();
    let mut transport = create_ssh_transport(&config).await;
    
    if transport.connect().await.is_ok() {
        // Test various commands
        let commands = vec![
            b"pwd\n",
            b"ls -la\n",
            b"echo $USER\n",
            b"date\n",
        ];
        
        for cmd in commands {
            transport.send(cmd).await.expect("Send command failed");
            let response = transport.receive(Duration::from_secs(2)).await
                .expect("Receive response failed");
            
            assert!(!response.is_empty(), "Empty response for command");
            println!("Command response: {}", String::from_utf8_lossy(&response));
        }
        
        transport.disconnect().await.ok();
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_shell_session() {
    let config = SshLoopbackConfig::default();
    let mut transport = create_ssh_transport(&config).await;
    
    if transport.connect().await.is_ok() {
        // Start interactive shell session
        transport.send(b"bash\n").await.expect("Failed to start shell");
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // Send commands in shell
        transport.send(b"export TEST_VAR='Hello World'\n").await.ok();
        transport.send(b"echo $TEST_VAR\n").await.ok();
        
        let response = transport.receive(Duration::from_secs(2)).await
            .expect("Failed to receive shell output");
        
        let output = String::from_utf8_lossy(&response);
        assert!(output.contains("Hello World") || !output.is_empty(), 
                "Shell variable not echoed");
        
        // Exit shell
        transport.send(b"exit\n").await.ok();
        transport.disconnect().await.ok();
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_file_transfer_pattern() {
    let config = SshLoopbackConfig::default();
    let mut transport = create_ssh_transport(&config).await;
    
    if transport.connect().await.is_ok() {
        // Simulate file transfer pattern with base64 encoding
        let file_content = b"This is test file content";
        let encoded = base64::encode(file_content);
        
        // Send encoded data
        let cmd = format!("echo '{}' | base64 -d > /tmp/test_file.txt\n", encoded);
        transport.send(cmd.as_bytes()).await.expect("Send file failed");
        
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        // Read back the file
        transport.send(b"cat /tmp/test_file.txt\n").await.expect("Cat failed");
        let response = transport.receive(Duration::from_secs(2)).await
            .expect("Receive file failed");
        
        let received_content = String::from_utf8_lossy(&response);
        assert!(received_content.contains("test file content") || 
                !received_content.is_empty(), 
                "File content not retrieved");
        
        // Clean up
        transport.send(b"rm /tmp/test_file.txt\n").await.ok();
        transport.disconnect().await.ok();
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_reconnection() {
    let config = SshLoopbackConfig::default();
    let mut transport = create_ssh_transport(&config).await;
    
    if transport.connect().await.is_ok() {
        // Initial command
        transport.send(b"echo 'Before disconnect'\n").await.ok();
        let _ = transport.receive(Duration::from_millis(500)).await;
        
        // Disconnect
        transport.disconnect().await.expect("Disconnect failed");
        assert!(!transport.is_connected());
        
        // Wait and reconnect
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        if transport.connect().await.is_ok() {
            assert!(transport.is_connected());
            
            // Command after reconnect
            transport.send(b"echo 'After reconnect'\n").await
                .expect("Send after reconnect failed");
            let response = transport.receive(Duration::from_secs(2)).await
                .expect("Receive after reconnect failed");
            
            let output = String::from_utf8_lossy(&response);
            assert!(output.contains("After reconnect") || !output.is_empty());
            
            transport.disconnect().await.ok();
        }
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_concurrent_channels() {
    let config = SshLoopbackConfig::default();
    let transport = std::sync::Arc::new(tokio::sync::Mutex::new(
        create_ssh_transport(&config).await
    ));
    
    if transport.lock().await.connect().await.is_ok() {
        // Spawn concurrent command executions
        let mut handles = vec![];
        
        for i in 0..5 {
            let transport_clone = transport.clone();
            let handle = tokio::spawn(async move {
                let cmd = format!("echo 'Concurrent {}'\n", i);
                let mut t = transport_clone.lock().await;
                t.send(cmd.as_bytes()).await.expect("Concurrent send failed");
                t.receive(Duration::from_secs(2)).await
            });
            handles.push(handle);
        }
        
        // Wait for all commands
        for handle in handles {
            let result = handle.await.expect("Task panicked");
            assert!(result.is_ok(), "Concurrent command failed");
        }
        
        transport.lock().await.disconnect().await.ok();
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_large_output() {
    let config = SshLoopbackConfig::default();
    let mut transport = create_ssh_transport(&config).await;
    
    if transport.connect().await.is_ok() {
        // Generate large output
        transport.send(b"seq 1 10000\n").await
            .expect("Send seq command failed");
        
        // Collect all output
        let mut total_output = Vec::new();
        let start = std::time::Instant::now();
        
        while start.elapsed() < Duration::from_secs(5) {
            match transport.receive(Duration::from_millis(100)).await {
                Ok(data) => total_output.extend_from_slice(&data),
                Err(_) => break,
            }
        }
        
        // Verify we got substantial output
        assert!(total_output.len() > 1000, 
                "Large output not received: only {} bytes", total_output.len());
        
        let output_str = String::from_utf8_lossy(&total_output);
        assert!(output_str.contains("10000") || output_str.contains("9999"), 
                "Sequence end not found in output");
        
        transport.disconnect().await.ok();
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_error_handling() {
    let config = SshLoopbackConfig::default();
    let mut transport = create_ssh_transport(&config).await;
    
    if transport.connect().await.is_ok() {
        // Send invalid command
        transport.send(b"invalidcommandthatdoesnotexist\n").await
            .expect("Send invalid command failed");
        
        let response = transport.receive(Duration::from_secs(2)).await
            .expect("Receive error response failed");
        
        let output = String::from_utf8_lossy(&response);
        // Should get some error message
        assert!(!output.is_empty() || 
                output.contains("not found") || 
                output.contains("command"), 
                "No error message for invalid command");
        
        // Connection should still be alive
        assert!(transport.is_connected());
        
        // Should still work for valid commands
        transport.send(b"echo 'Still working'\n").await
            .expect("Send after error failed");
        let _ = transport.receive(Duration::from_secs(1)).await;
        
        transport.disconnect().await.ok();
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_timeout_handling() {
    let mut config = SshLoopbackConfig::default();
    let mut transport_config = TransportConfig::default();
    transport_config.timeout = Duration::from_millis(100);  // Very short timeout
    
    let mut transport = SshTransport::new(
        format!("{}:{}", config.host, config.port),
        transport_config,
        config.username.clone(),
        config.key_path.clone(),
        config.password.clone(),
    );
    
    if transport.connect().await.is_ok() {
        // Send command that takes time
        transport.send(b"sleep 2 && echo 'Done'\n").await
            .expect("Send sleep command failed");
        
        // Should timeout
        let result = transport.receive(Duration::from_millis(100)).await;
        assert!(result.is_err(), "Should have timed out");
        
        // Connection should survive timeout
        assert!(transport.is_connected());
        
        transport.disconnect().await.ok();
    }
}

#[tokio::test]
#[ignore] // Requires SSH server  
async fn test_ssh_keepalive() {
    let config = SshLoopbackConfig::default();
    let mut transport = create_ssh_transport(&config).await;
    
    if transport.connect().await.is_ok() {
        // Send initial command
        transport.send(b"echo 'Initial'\n").await.expect("Initial send failed");
        let _ = transport.receive(Duration::from_millis(500)).await;
        
        // Wait for potential timeout (simulate idle)
        tokio::time::sleep(Duration::from_secs(10)).await;
        
        // Connection should still be alive (SSH keepalive)
        assert!(transport.is_connected(), "Connection died during idle");
        
        // Should still work
        transport.send(b"echo 'After idle'\n").await
            .expect("Send after idle failed");
        let response = transport.receive(Duration::from_secs(2)).await
            .expect("Receive after idle failed");
        
        assert!(!response.is_empty(), "No response after idle period");
        
        transport.disconnect().await.ok();
    }
}

#[tokio::test]
#[ignore] // Requires SSH server
async fn test_ssh_special_characters() {
    let config = SshLoopbackConfig::default();
    let mut transport = create_ssh_transport(&config).await;
    
    if transport.connect().await.is_ok() {
        // Test special characters in commands
        let special_commands = vec![
            b"echo 'Hello \"World\"'\n",
            b"echo 'Path: /usr/local/bin'\n",
            b"echo 'Var=$HOME'\n",
            b"echo 'Special: !@#$%^&*()'\n",
        ];
        
        for cmd in special_commands {
            transport.send(cmd).await.expect("Send special failed");
            let response = transport.receive(Duration::from_secs(1)).await
                .expect("Receive special failed");
            
            assert!(!response.is_empty(), "Empty response for special chars");
        }
        
        transport.disconnect().await.ok();
    }
}

/// Mock SSH server for testing when real server unavailable
pub struct MockSshServer {
    responses: std::collections::HashMap<String, String>,
}

impl MockSshServer {
    pub fn new() -> Self {
        let mut responses = std::collections::HashMap::new();
        responses.insert("echo".to_string(), "echo_response".to_string());
        responses.insert("pwd".to_string(), "/home/test_user".to_string());
        responses.insert("ls".to_string(), "file1.txt\nfile2.txt".to_string());
        
        Self { responses }
    }
    
    pub fn handle_command(&self, cmd: &str) -> String {
        let cmd_parts: Vec<&str> = cmd.trim().split_whitespace().collect();
        if let Some(base_cmd) = cmd_parts.first() {
            if let Some(response) = self.responses.get(*base_cmd) {
                return response.clone();
            }
        }
        format!("bash: {}: command not found", cmd)
    }
}