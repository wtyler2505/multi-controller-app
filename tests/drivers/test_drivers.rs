//! Mock driver implementations for comprehensive testing
//! 
//! Provides various mock drivers with configurable behavior for testing
//! different scenarios including failures, capabilities, and transport types.

use async_trait::async_trait;
use std::sync::{Arc, atomic::{AtomicU32, AtomicBool, Ordering}};
use std::collections::HashMap;
use tokio::sync::{RwLock, Mutex};
use serde_json::{Value, json};

use multi_controller_app::device::{
    DeviceDriver, DeviceSession, DeviceResult, DeviceError,
    Transport, TransportType, DriverCapabilities, 
    session::{SessionStatistics, StreamData, SubscriptionHandle}
};

/// Mock Arduino driver for testing
pub struct MockArduinoDriver {
    name: String,
    version: String,
    failure_mode: Arc<AtomicU32>, // Operations until failure (0 = never fail)
    operation_count: Arc<AtomicU32>,
}

impl MockArduinoDriver {
    pub fn new(name: &str) -> Self {
        MockArduinoDriver {
            name: name.to_string(),
            version: "1.0.0-test".to_string(),
            failure_mode: Arc::new(AtomicU32::new(0)),
            operation_count: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn with_failure_after(name: &str, operations: u32) -> Self {
        MockArduinoDriver {
            name: name.to_string(),
            version: "1.0.0-test".to_string(),
            failure_mode: Arc::new(AtomicU32::new(operations)),
            operation_count: Arc::new(AtomicU32::new(0)),
        }
    }
}

#[async_trait]
impl DeviceDriver for MockArduinoDriver {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn supported_transports(&self) -> Vec<TransportType> {
        vec![TransportType::Serial]
    }
    
    async fn probe_async(&self, _transport: Arc<dyn Transport>) -> DeviceResult<bool> {
        // Always successful for Arduino-like devices
        Ok(true)
    }
    
    async fn open_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<Box<dyn DeviceSession>> {
        let session = MockArduinoSession::new(
            transport, 
            &self.name, 
            self.failure_mode.clone(),
            self.operation_count.clone()
        );
        Ok(Box::new(session))
    }
    
    fn capabilities(&self) -> DriverCapabilities {
        DriverCapabilities {
            hot_plug: true,
            telemetry: true,
            pwm: true,
            gpio: true,
            analog_input: true,
            serial_passthrough: true,
            firmware_update: false,
            requires_auth: false,
            max_data_rate: Some(115200 / 10),
            min_latency_ms: Some(50),
        }
    }
}

/// Mock Raspberry Pi driver for testing SSH/network scenarios
pub struct MockRaspberryPiDriver {
    name: String,
    version: String,
}

impl MockRaspberryPiDriver {
    pub fn new(name: &str) -> Self {
        MockRaspberryPiDriver {
            name: name.to_string(),
            version: "1.0.0-test".to_string(),
        }
    }
}

#[async_trait]
impl DeviceDriver for MockRaspberryPiDriver {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn supported_transports(&self) -> Vec<TransportType> {
        vec![TransportType::SSH, TransportType::Serial, TransportType::TCP]
    }
    
    async fn probe_async(&self, _transport: Arc<dyn Transport>) -> DeviceResult<bool> {
        Ok(true)
    }
    
    async fn open_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<Box<dyn DeviceSession>> {
        let session = MockRaspberryPiSession::new(transport, &self.name);
        Ok(Box::new(session))
    }
    
    fn capabilities(&self) -> DriverCapabilities {
        DriverCapabilities {
            hot_plug: true,
            telemetry: true,
            pwm: true,
            gpio: true,
            analog_input: true,
            serial_passthrough: true,
            firmware_update: true,
            requires_auth: true,
            max_data_rate: Some(1_000_000), // 1MB/s over network
            min_latency_ms: Some(20), // Lower latency over network
        }
    }
}

/// Mock generic driver supporting all transport types
pub struct MockGenericDriver {
    name: String,
    version: String,
}

impl MockGenericDriver {
    pub fn new(name: &str) -> Self {
        MockGenericDriver {
            name: name.to_string(),
            version: "1.0.0-test".to_string(),
        }
    }
}

#[async_trait]
impl DeviceDriver for MockGenericDriver {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn supported_transports(&self) -> Vec<TransportType> {
        vec![
            TransportType::Serial,
            TransportType::SSH,
            TransportType::TCP,
            TransportType::UDP,
        ]
    }
    
    async fn probe_async(&self, _transport: Arc<dyn Transport>) -> DeviceResult<bool> {
        Ok(true)
    }
    
    async fn open_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<Box<dyn DeviceSession>> {
        let session = MockGenericSession::new(transport, &self.name);
        Ok(Box::new(session))
    }
    
    fn capabilities(&self) -> DriverCapabilities {
        DriverCapabilities {
            hot_plug: true,
            telemetry: true,
            pwm: false,
            gpio: true,
            analog_input: false,
            serial_passthrough: true,
            firmware_update: true,
            requires_auth: false,
            max_data_rate: Some(10_000_000), // 10MB/s
            min_latency_ms: Some(10),
        }
    }
}

/// Mock Arduino session implementation
pub struct MockArduinoSession {
    transport: Arc<dyn Transport>,
    session_id: String,
    device_name: String,
    active: Arc<AtomicBool>,
    pin_states: Arc<RwLock<HashMap<u8, bool>>>,
    statistics: Arc<Mutex<SessionStatistics>>,
    failure_mode: Arc<AtomicU32>,
    operation_count: Arc<AtomicU32>,
}

impl MockArduinoSession {
    fn new(
        transport: Arc<dyn Transport>, 
        device_name: &str,
        failure_mode: Arc<AtomicU32>,
        operation_count: Arc<AtomicU32>
    ) -> Self {
        MockArduinoSession {
            transport,
            session_id: uuid::Uuid::new_v4().to_string(),
            device_name: device_name.to_string(),
            active: Arc::new(AtomicBool::new(true)),
            pin_states: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(Mutex::new(SessionStatistics::default())),
            failure_mode,
            operation_count,
        }
    }

    fn should_fail(&self) -> bool {
        let failure_after = self.failure_mode.load(Ordering::Relaxed);
        if failure_after == 0 {
            return false;
        }
        
        let current_count = self.operation_count.fetch_add(1, Ordering::Relaxed) + 1;
        current_count % failure_after == 0
    }
}

#[async_trait]
impl DeviceSession for MockArduinoSession {
    fn session_id(&self) -> &str {
        &self.session_id
    }
    
    fn device_name(&self) -> &str {
        &self.device_name
    }
    
    async fn invoke_async(&mut self, endpoint: &str, args: Vec<Value>) -> DeviceResult<Value> {
        if !self.active.load(Ordering::Relaxed) {
            return Err(DeviceError::DeviceNotConnected("Session closed".into()));
        }

        // Check for simulated failure
        if self.should_fail() {
            return Err(DeviceError::Unknown("Simulated device failure".into()));
        }

        // Update statistics
        {
            let mut stats = self.statistics.lock().await;
            stats.commands_sent += 1;
        }

        match endpoint {
            "digitalWrite" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let value = args.get(1)
                    .and_then(|v| v.as_bool())
                    .ok_or_else(|| DeviceError::Unknown("Missing value argument".into()))?;
                
                let mut pins = self.pin_states.write().await;
                pins.insert(pin, value);
                
                Ok(json!({ "success": true, "pin": pin, "value": value }))
            }
            
            "digitalRead" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let pins = self.pin_states.read().await;
                let value = pins.get(&pin).copied().unwrap_or(false);
                
                Ok(json!({ "value": value, "pin": pin }))
            }
            
            "analogRead" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                // Simulate analog reading
                let value = 512 + (pin as u16 * 10); // Deterministic value based on pin
                
                Ok(json!({ "value": value, "pin": pin }))
            }

            "readTemperature" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u8;
                
                let sensor_type = args.get(1)
                    .and_then(|v| v.as_str())
                    .unwrap_or("DHT22");
                
                Ok(json!({
                    "temperature_c": 22.5 + (pin as f64 * 0.1),
                    "sensor_type": sensor_type,
                    "humidity": if sensor_type == "DHT22" { Some(65.0) } else { None }
                }))
            }
            
            _ => Err(DeviceError::Unknown(format!("Unknown endpoint: {}", endpoint))),
        }
    }
    
    async fn subscribe_async(
        &mut self,
        _stream: &str,
        _handler: tokio::sync::mpsc::UnboundedSender<StreamData>,
    ) -> DeviceResult<SubscriptionHandle> {
        // Mock subscription
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        Ok(SubscriptionHandle::new("mock_subscription".to_string(), tx))
    }
    
    async fn close_async(&mut self) -> DeviceResult<()> {
        self.active.store(false, Ordering::Relaxed);
        Ok(())
    }
    
    fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
    
    fn statistics(&self) -> SessionStatistics {
        self.statistics.try_lock()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| SessionStatistics::default())
    }
    
    async fn send_raw(&mut self, _data: &[u8]) -> DeviceResult<Vec<u8>> {
        if self.should_fail() {
            return Err(DeviceError::Unknown("Simulated raw send failure".into()));
        }
        
        Ok(b"OK\n".to_vec())
    }
}

/// Mock Raspberry Pi session implementation
pub struct MockRaspberryPiSession {
    transport: Arc<dyn Transport>,
    session_id: String,
    device_name: String,
    active: Arc<AtomicBool>,
    gpio_states: Arc<RwLock<HashMap<u8, bool>>>,
    statistics: Arc<Mutex<SessionStatistics>>,
}

impl MockRaspberryPiSession {
    fn new(transport: Arc<dyn Transport>, device_name: &str) -> Self {
        MockRaspberryPiSession {
            transport,
            session_id: uuid::Uuid::new_v4().to_string(),
            device_name: device_name.to_string(),
            active: Arc::new(AtomicBool::new(true)),
            gpio_states: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(Mutex::new(SessionStatistics::default())),
        }
    }
}

#[async_trait]
impl DeviceSession for MockRaspberryPiSession {
    fn session_id(&self) -> &str {
        &self.session_id
    }
    
    fn device_name(&self) -> &str {
        &self.device_name
    }
    
    async fn invoke_async(&mut self, endpoint: &str, args: Vec<Value>) -> DeviceResult<Value> {
        if !self.active.load(Ordering::Relaxed) {
            return Err(DeviceError::DeviceNotConnected("Session closed".into()));
        }

        {
            let mut stats = self.statistics.lock().await;
            stats.commands_sent += 1;
        }

        match endpoint {
            "gpioWrite" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let value = args.get(1)
                    .and_then(|v| v.as_bool())
                    .ok_or_else(|| DeviceError::Unknown("Missing value argument".into()))?;
                
                let mut gpio = self.gpio_states.write().await;
                gpio.insert(pin, value);
                
                Ok(json!({ "success": true, "gpio": pin, "value": value }))
            }
            
            "gpioRead" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let gpio = self.gpio_states.read().await;
                let value = gpio.get(&pin).copied().unwrap_or(false);
                
                Ok(json!({ "value": value, "gpio": pin }))
            }
            
            "systemInfo" => {
                Ok(json!({
                    "platform": "linux",
                    "architecture": "arm64",
                    "model": "Raspberry Pi 4 Model B",
                    "memory": "4GB",
                    "cpu_temp": 45.2
                }))
            }
            
            "executeCommand" => {
                let command = args.get(0)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DeviceError::Unknown("Missing command argument".into()))?;
                
                // Simulate command execution
                let output = match command {
                    "uptime" => "up 5 days, 12:34",
                    "whoami" => "pi",
                    "pwd" => "/home/pi",
                    _ => "command not found",
                };
                
                Ok(json!({
                    "command": command,
                    "output": output,
                    "exit_code": if output == "command not found" { 127 } else { 0 }
                }))
            }
            
            _ => Err(DeviceError::Unknown(format!("Unknown endpoint: {}", endpoint))),
        }
    }
    
    async fn subscribe_async(
        &mut self,
        stream: &str,
        _handler: tokio::sync::mpsc::UnboundedSender<StreamData>,
    ) -> DeviceResult<SubscriptionHandle> {
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        Ok(SubscriptionHandle::new(format!("pi_{}", stream), tx))
    }
    
    async fn close_async(&mut self) -> DeviceResult<()> {
        self.active.store(false, Ordering::Relaxed);
        Ok(())
    }
    
    fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
    
    fn statistics(&self) -> SessionStatistics {
        self.statistics.try_lock()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| SessionStatistics::default())
    }
    
    async fn send_raw(&mut self, data: &[u8]) -> DeviceResult<Vec<u8>> {
        // Echo back the data with a prefix
        let mut response = b"Pi: ".to_vec();
        response.extend_from_slice(data);
        Ok(response)
    }
}

/// Mock generic session implementation
pub struct MockGenericSession {
    transport: Arc<dyn Transport>,
    session_id: String,
    device_name: String,
    active: Arc<AtomicBool>,
    statistics: Arc<Mutex<SessionStatistics>>,
    data_store: Arc<RwLock<HashMap<String, Value>>>,
}

impl MockGenericSession {
    fn new(transport: Arc<dyn Transport>, device_name: &str) -> Self {
        MockGenericSession {
            transport,
            session_id: uuid::Uuid::new_v4().to_string(),
            device_name: device_name.to_string(),
            active: Arc::new(AtomicBool::new(true)),
            statistics: Arc::new(Mutex::new(SessionStatistics::default())),
            data_store: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl DeviceSession for MockGenericSession {
    fn session_id(&self) -> &str {
        &self.session_id
    }
    
    fn device_name(&self) -> &str {
        &self.device_name
    }
    
    async fn invoke_async(&mut self, endpoint: &str, args: Vec<Value>) -> DeviceResult<Value> {
        if !self.active.load(Ordering::Relaxed) {
            return Err(DeviceError::DeviceNotConnected("Session closed".into()));
        }

        {
            let mut stats = self.statistics.lock().await;
            stats.commands_sent += 1;
        }

        match endpoint {
            "set" => {
                let key = args.get(0)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DeviceError::Unknown("Missing key argument".into()))?;
                
                let value = args.get(1)
                    .ok_or_else(|| DeviceError::Unknown("Missing value argument".into()))?;
                
                let mut store = self.data_store.write().await;
                store.insert(key.to_string(), value.clone());
                
                Ok(json!({ "success": true, "key": key }))
            }
            
            "get" => {
                let key = args.get(0)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DeviceError::Unknown("Missing key argument".into()))?;
                
                let store = self.data_store.read().await;
                let value = store.get(key).cloned().unwrap_or(Value::Null);
                
                Ok(json!({ "key": key, "value": value }))
            }
            
            "list" => {
                let store = self.data_store.read().await;
                let keys: Vec<&String> = store.keys().collect();
                
                Ok(json!({ "keys": keys }))
            }
            
            "status" => {
                Ok(json!({
                    "active": self.is_active(),
                    "session_id": self.session_id(),
                    "device_name": self.device_name(),
                    "transport_type": format!("{:?}", self.transport.transport_type())
                }))
            }
            
            _ => Err(DeviceError::Unknown(format!("Unknown endpoint: {}", endpoint))),
        }
    }
    
    async fn subscribe_async(
        &mut self,
        stream: &str,
        _handler: tokio::sync::mpsc::UnboundedSender<StreamData>,
    ) -> DeviceResult<SubscriptionHandle> {
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        Ok(SubscriptionHandle::new(format!("generic_{}", stream), tx))
    }
    
    async fn close_async(&mut self) -> DeviceResult<()> {
        self.active.store(false, Ordering::Relaxed);
        Ok(())
    }
    
    fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
    
    fn statistics(&self) -> SessionStatistics {
        self.statistics.try_lock()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| SessionStatistics::default())
    }
    
    async fn send_raw(&mut self, data: &[u8]) -> DeviceResult<Vec<u8>> {
        // Simple echo with JSON wrapper
        let response = json!({
            "echo": String::from_utf8_lossy(data),
            "length": data.len()
        });
        
        Ok(response.to_string().into_bytes())
    }
}