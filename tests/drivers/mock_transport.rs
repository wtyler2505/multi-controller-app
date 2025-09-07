/// Mock transport for driver testing
/// Provides controllable transport behavior for comprehensive driver testing

use async_trait::async_trait;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};

use multi_controller_app::transport::{Transport, TransportStats, TransportError, TransportResult};

/// Mock transport configuration
#[derive(Clone, Debug)]
pub struct MockTransportConfig {
    /// Simulated device type
    pub device_type: MockDeviceType,
    /// Whether probe commands succeed
    pub probe_succeeds: bool,
    /// Simulated latency for operations
    pub latency_ms: u64,
    /// Whether to simulate connection failures
    pub fail_on_connect: bool,
    /// Number of retries before succeeding
    pub retries_before_success: u32,
    /// Whether to track all commands sent
    pub record_commands: bool,
}

impl Default for MockTransportConfig {
    fn default() -> Self {
        Self {
            device_type: MockDeviceType::ArduinoUno,
            probe_succeeds: true,
            latency_ms: 10,
            fail_on_connect: false,
            retries_before_success: 0,
            record_commands: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MockDeviceType {
    ArduinoUno,
    ArduinoMega,
    RaspberryPi,
    Generic,
    Invalid,
}

/// Mock transport for testing device drivers
pub struct MockTransport {
    config: MockTransportConfig,
    connected: Arc<RwLock<bool>>,
    stats: Arc<RwLock<TransportStats>>,
    command_history: Arc<Mutex<Vec<Vec<u8>>>>,
    response_queue: Arc<Mutex<VecDeque<Vec<u8>>>>,
    retry_count: Arc<Mutex<u32>>,
    last_command: Arc<Mutex<Option<Vec<u8>>>>,
}

impl MockTransport {
    pub fn new(config: MockTransportConfig) -> Self {
        Self {
            config,
            connected: Arc::new(RwLock::new(false)),
            stats: Arc::new(RwLock::new(TransportStats::default())),
            command_history: Arc::new(Mutex::new(Vec::new())),
            response_queue: Arc::new(Mutex::new(VecDeque::new())),
            retry_count: Arc::new(Mutex::new(0)),
            last_command: Arc::new(Mutex::new(None)),
        }
    }

    /// Add a response to the queue
    pub async fn queue_response(&self, response: Vec<u8>) {
        let mut queue = self.response_queue.lock().await;
        queue.push_back(response);
    }

    /// Get command history
    pub async fn get_command_history(&self) -> Vec<Vec<u8>> {
        self.command_history.lock().await.clone()
    }

    /// Clear command history
    pub async fn clear_history(&self) {
        self.command_history.lock().await.clear();
    }

    /// Set connection state
    pub async fn set_connected(&self, connected: bool) {
        *self.connected.write().await = connected;
    }

    /// Generate device-specific probe response
    fn generate_probe_response(&self) -> Vec<u8> {
        match self.config.device_type {
            MockDeviceType::ArduinoUno => b"ARDUINO_UNO_V1\r\n".to_vec(),
            MockDeviceType::ArduinoMega => b"ARDUINO_MEGA_V1\r\n".to_vec(),
            MockDeviceType::RaspberryPi => b"RPI3B_V1\r\n".to_vec(),
            MockDeviceType::Generic => b"DEVICE_OK\r\n".to_vec(),
            MockDeviceType::Invalid => b"UNKNOWN\r\n".to_vec(),
        }
    }

    /// Generate command response based on command type
    fn generate_command_response(&self, command: &[u8]) -> Vec<u8> {
        let cmd_str = String::from_utf8_lossy(command);
        
        // Parse command and generate appropriate response
        if cmd_str.starts_with("PROBE") {
            if self.config.probe_succeeds {
                self.generate_probe_response()
            } else {
                b"ERROR\r\n".to_vec()
            }
        } else if cmd_str.starts_with("PIN_MODE") {
            b"OK\r\n".to_vec()
        } else if cmd_str.starts_with("DIGITAL_WRITE") {
            b"OK\r\n".to_vec()
        } else if cmd_str.starts_with("DIGITAL_READ") {
            b"VALUE:1\r\n".to_vec()
        } else if cmd_str.starts_with("ANALOG_READ") {
            b"VALUE:512\r\n".to_vec()
        } else if cmd_str.starts_with("PWM_WRITE") {
            b"OK\r\n".to_vec()
        } else if cmd_str.starts_with("HALL_CONFIG") {
            b"OK\r\n".to_vec()
        } else if cmd_str.starts_with("HALL_READ") {
            b"RPM:1250.5\r\n".to_vec()
        } else if cmd_str.starts_with("HALL_COUNT") {
            b"COUNT:12345\r\n".to_vec()
        } else if cmd_str.starts_with("HALL_RESET") {
            b"OK\r\n".to_vec()
        } else if cmd_str.starts_with("ULTRASONIC") {
            b"DISTANCE:25.5\r\n".to_vec()
        } else if cmd_str.starts_with("TEMP") {
            b"TEMP:22.5,HUM:65.0\r\n".to_vec()
        } else if cmd_str.starts_with("SERVO") {
            b"OK\r\n".to_vec()
        } else if cmd_str.starts_with("I2C_READ") {
            b"DATA:42,43,44\r\n".to_vec()
        } else if cmd_str.starts_with("IMU_READ") {
            b"ACC:0.02,-0.01,9.81,GYRO:0.5,-0.3,0.1\r\n".to_vec()
        } else if cmd_str.starts_with("PRESSURE") {
            b"PRES:1013.25,ALT:152.4,TEMP:23.8\r\n".to_vec()
        } else if cmd_str.starts_with("LOAD_CELL") {
            b"WEIGHT:1523.7,RAW:842156\r\n".to_vec()
        } else if cmd_str.starts_with("GAS") {
            b"SMOKE:125,LPG:85,CO:42\r\n".to_vec()
        } else if cmd_str.starts_with("GPS") {
            b"LAT:37.7749,LON:-122.4194,ALT:52.3,SAT:8\r\n".to_vec()
        } else {
            b"OK\r\n".to_vec()
        }
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn connect(&mut self) -> TransportResult<()> {
        // Simulate retry behavior
        let mut retries = self.retry_count.lock().await;
        
        if self.config.fail_on_connect && *retries < self.config.retries_before_success {
            *retries += 1;
            let mut stats = self.stats.write().await;
            stats.failed_connections += 1;
            return Err(TransportError::ConnectionFailed(
                format!("Mock connection failed (attempt {})", *retries)
            ));
        }
        
        // Simulate latency
        if self.config.latency_ms > 0 {
            tokio::time::sleep(Duration::from_millis(self.config.latency_ms)).await;
        }
        
        *self.connected.write().await = true;
        let mut stats = self.stats.write().await;
        stats.successful_connections += 1;
        
        Ok(())
    }

    async fn disconnect(&mut self) -> TransportResult<()> {
        *self.connected.write().await = false;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        // Use try_read to avoid blocking
        self.connected.try_read()
            .map(|guard| *guard)
            .unwrap_or(false)
    }

    async fn send(&mut self, data: &[u8]) -> TransportResult<()> {
        if !self.is_connected() {
            return Err(TransportError::NotConnected);
        }

        // Simulate latency
        if self.config.latency_ms > 0 {
            tokio::time::sleep(Duration::from_millis(self.config.latency_ms)).await;
        }

        // Record command if configured
        if self.config.record_commands {
            let mut history = self.command_history.lock().await;
            history.push(data.to_vec());
        }

        // Store last command
        *self.last_command.lock().await = Some(data.to_vec());

        // Update stats
        let mut stats = self.stats.write().await;
        stats.bytes_sent += data.len() as u64;
        stats.packets_sent += 1;

        Ok(())
    }

    async fn receive(&mut self, timeout: Duration) -> TransportResult<Vec<u8>> {
        if !self.is_connected() {
            return Err(TransportError::NotConnected);
        }

        // Check for queued responses first
        let mut queue = self.response_queue.lock().await;
        if let Some(response) = queue.pop_front() {
            let mut stats = self.stats.write().await;
            stats.bytes_received += response.len() as u64;
            stats.packets_received += 1;
            return Ok(response);
        }
        drop(queue);

        // Generate response based on last command
        let last_cmd = self.last_command.lock().await;
        if let Some(cmd) = last_cmd.as_ref() {
            // Simulate latency
            if self.config.latency_ms > 0 {
                tokio::time::sleep(Duration::from_millis(self.config.latency_ms)).await;
            }

            let response = self.generate_command_response(cmd);
            
            let mut stats = self.stats.write().await;
            stats.bytes_received += response.len() as u64;
            stats.packets_received += 1;
            
            Ok(response)
        } else {
            // No command sent yet, timeout
            tokio::time::sleep(timeout).await;
            Err(TransportError::Timeout)
        }
    }

    fn stats(&self) -> TransportStats {
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| TransportStats::default())
    }

    async fn flush(&mut self) -> TransportResult<()> {
        // Clear any pending data
        self.response_queue.lock().await.clear();
        *self.last_command.lock().await = None;
        Ok(())
    }

    fn transport_type(&self) -> crate::transport::TransportType {
        crate::transport::TransportType::Mock
    }
}

/// Test fixture for driver testing
pub struct DriverTestFixture {
    pub transport: Arc<Mutex<MockTransport>>,
    pub config: MockTransportConfig,
}

impl DriverTestFixture {
    pub fn new() -> Self {
        let config = MockTransportConfig::default();
        let transport = Arc::new(Mutex::new(MockTransport::new(config.clone())));
        Self { transport, config }
    }

    pub fn with_device(device_type: MockDeviceType) -> Self {
        let mut config = MockTransportConfig::default();
        config.device_type = device_type;
        let transport = Arc::new(Mutex::new(MockTransport::new(config.clone())));
        Self { transport, config }
    }

    pub fn with_failures(retries: u32) -> Self {
        let mut config = MockTransportConfig::default();
        config.fail_on_connect = true;
        config.retries_before_success = retries;
        let transport = Arc::new(Mutex::new(MockTransport::new(config.clone())));
        Self { transport, config }
    }

    pub async fn connect(&self) -> TransportResult<()> {
        self.transport.lock().await.connect().await
    }

    pub async fn queue_response(&self, response: Vec<u8>) {
        self.transport.lock().await.queue_response(response).await;
    }

    pub async fn get_command_history(&self) -> Vec<Vec<u8>> {
        self.transport.lock().await.get_command_history().await
    }

    pub async fn verify_command_sent(&self, expected: &[u8]) -> bool {
        let history = self.get_command_history().await;
        history.iter().any(|cmd| cmd == expected)
    }

    pub async fn verify_last_command(&self, expected: &[u8]) -> bool {
        let history = self.get_command_history().await;
        history.last().map(|cmd| cmd.as_slice() == expected).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_transport_basic() {
        let mut transport = MockTransport::new(MockTransportConfig::default());
        
        assert!(!transport.is_connected());
        transport.connect().await.expect("Connect failed");
        assert!(transport.is_connected());
        
        transport.send(b"PROBE\r\n").await.expect("Send failed");
        let response = transport.receive(Duration::from_secs(1)).await
            .expect("Receive failed");
        
        assert_eq!(response, b"ARDUINO_UNO_V1\r\n");
        
        transport.disconnect().await.expect("Disconnect failed");
        assert!(!transport.is_connected());
    }

    #[tokio::test]
    async fn test_mock_transport_retry() {
        let mut config = MockTransportConfig::default();
        config.fail_on_connect = true;
        config.retries_before_success = 2;
        
        let mut transport = MockTransport::new(config);
        
        // First two attempts should fail
        assert!(transport.connect().await.is_err());
        assert!(transport.connect().await.is_err());
        
        // Third attempt should succeed
        assert!(transport.connect().await.is_ok());
        assert!(transport.is_connected());
    }

    #[tokio::test]
    async fn test_mock_transport_command_history() {
        let mut transport = MockTransport::new(MockTransportConfig::default());
        transport.connect().await.unwrap();
        
        transport.send(b"CMD1\r\n").await.unwrap();
        transport.send(b"CMD2\r\n").await.unwrap();
        transport.send(b"CMD3\r\n").await.unwrap();
        
        let history = transport.get_command_history().await;
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], b"CMD1\r\n");
        assert_eq!(history[1], b"CMD2\r\n");
        assert_eq!(history[2], b"CMD3\r\n");
    }

    #[tokio::test]
    async fn test_fixture() {
        let fixture = DriverTestFixture::with_device(MockDeviceType::ArduinoMega);
        fixture.connect().await.unwrap();
        
        {
            let mut transport = fixture.transport.lock().await;
            transport.send(b"PROBE\r\n").await.unwrap();
            let response = transport.receive(Duration::from_secs(1)).await.unwrap();
            assert_eq!(response, b"ARDUINO_MEGA_V1\r\n");
        }
        
        assert!(fixture.verify_command_sent(b"PROBE\r\n").await);
        assert!(fixture.verify_last_command(b"PROBE\r\n").await);
    }
}