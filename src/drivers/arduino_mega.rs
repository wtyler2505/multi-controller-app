use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use serde_json::{Value, json};
use tokio::sync::{Mutex, mpsc};
use serialport::{SerialPortType, SerialPortInfo};
use tracing::{info, debug, warn};

use crate::device::{
    DeviceDriver, DeviceSession, DeviceResult, DeviceError,
    Transport, TransportType, DriverCapabilities
};
use crate::device::session::{StreamData, SubscriptionHandle, SessionStatistics};

// Arduino USB Vendor IDs
const ARDUINO_VID: u16 = 0x2341;  // Official Arduino
const CH340_VID: u16 = 0x1A86;    // CH340 clone chip
const FTDI_VID: u16 = 0x0403;     // FTDI chip

// Arduino Product IDs
const ARDUINO_MEGA_PID: u16 = 0x0042;
const ARDUINO_MEGA_2560_PID: u16 = 0x0010;

// Command Protocol Constants
const CMD_PROBE: &str = "PROBE";
const CMD_PIN_MODE: &str = "PIN_MODE";
const CMD_DIGITAL_WRITE: &str = "DIGITAL_WRITE";
const CMD_DIGITAL_READ: &str = "DIGITAL_READ";
const CMD_ANALOG_READ: &str = "ANALOG_READ";
const CMD_PWM_WRITE: &str = "PWM_WRITE";
const CMD_HALL_CONFIG: &str = "HALL_CONFIG";
const CMD_HALL_READ: &str = "HALL_READ";

// Response codes
const RESP_OK: &str = "OK";
const RESP_ERROR: &str = "ERROR";

/// Arduino Mega 2560 specific constants
const MEGA_DIGITAL_PINS: u8 = 54;
const MEGA_ANALOG_PINS: u8 = 16;
const MEGA_PWM_PINS: &[u8] = &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 44, 45, 46];
const MEGA_INTERRUPT_PINS: &[u8] = &[2, 3, 18, 19, 20, 21];

/// Arduino Mega 2560 device driver
pub struct ArduinoMega2560Driver {
    name: String,
    version: String,
}

impl ArduinoMega2560Driver {
    pub fn new() -> Self {
        ArduinoMega2560Driver {
            name: "Arduino Mega 2560".to_string(),
            version: "1.0.0".to_string(),
        }
    }
    
    /// Detect Arduino Mega 2560 devices via USB VID/PID
    async fn detect_mega_usb(&self) -> DeviceResult<bool> {
        match serialport::available_ports() {
            Ok(ports) => {
                for port_info in ports {
                    debug!("Checking port: {:?}", port_info.port_name);
                    
                    if let SerialPortType::UsbPort(usb_info) = &port_info.port_type {
                        let vid = usb_info.vid;
                        let pid = usb_info.pid;
                        
                        debug!("USB device found - VID: 0x{:04X}, PID: 0x{:04X}", vid, pid);
                        
                        // Check for Arduino Mega 2560 specifically
                        if vid == ARDUINO_VID && (pid == ARDUINO_MEGA_PID || pid == ARDUINO_MEGA_2560_PID) {
                            info!("Arduino Mega 2560 detected on {} (VID: 0x{:04X}, PID: 0x{:04X})", 
                                  port_info.port_name, vid, pid);
                            return Ok(true);
                        }
                        
                        // Check clone chips that might be Mega 2560
                        if (vid == CH340_VID || vid == FTDI_VID) {
                            if let Some(product) = &usb_info.product {
                                if product.contains("Mega") || product.contains("2560") {
                                    info!("Arduino Mega 2560 clone detected on {}", port_info.port_name);
                                    return Ok(true);
                                }
                            }
                        }
                    }
                }
                Ok(false)
            }
            Err(e) => {
                warn!("Failed to enumerate serial ports: {}", e);
                Ok(true) // Continue with probe anyway
            }
        }
    }
}

#[async_trait]
impl DeviceDriver for ArduinoMega2560Driver {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn supported_transports(&self) -> Vec<TransportType> {
        vec![TransportType::Serial]
    }
    
    async fn probe_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<bool> {
        // First check if this is likely an Arduino Mega 2560 based on serial port info
        if let Ok(is_mega) = self.detect_mega_usb().await {
            if !is_mega {
                debug!("No Arduino Mega 2560 USB devices detected");
                return Ok(false);
            }
        }
        
        // Would send probe command to verify it's a Mega 2560
        debug!("Arduino Mega 2560 device detected via USB VID/PID");
        Ok(true)
    }
    
    async fn open_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<Box<dyn DeviceSession>> {
        let session = ArduinoMega2560Session::new(transport);
        Ok(Box::new(session))
    }
    
    fn capabilities(&self) -> DriverCapabilities {
        DriverCapabilities {
            hot_plug: false,
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

/// Arduino Mega 2560 session implementation
pub struct ArduinoMega2560Session {
    transport: Arc<dyn Transport>,
    session_id: String,
    pin_modes: Arc<Mutex<HashMap<u8, PinMode>>>,
    active: Arc<Mutex<bool>>,
    stats: Arc<Mutex<SessionStatistics>>,
}

#[derive(Debug, Clone)]
enum PinMode {
    Input,
    Output,
    PwmOutput,
    AnalogInput,
    Interrupt,
}

impl ArduinoMega2560Session {
    fn new(transport: Arc<dyn Transport>) -> Self {
        ArduinoMega2560Session {
            transport,
            session_id: uuid::Uuid::new_v4().to_string(),
            pin_modes: Arc::new(Mutex::new(HashMap::new())),
            active: Arc::new(Mutex::new(true)),
            stats: Arc::new(Mutex::new(SessionStatistics::new())),
        }
    }
    
    /// Validate pin number for Mega 2560
    fn validate_digital_pin(&self, pin: u8) -> DeviceResult<()> {
        if pin >= MEGA_DIGITAL_PINS {
            Err(DeviceError::Unknown(format!("Invalid digital pin {} (Mega 2560 has 0-53)", pin)))
        } else {
            Ok(())
        }
    }
    
    /// Validate analog pin for Mega 2560
    fn validate_analog_pin(&self, pin: u8) -> DeviceResult<()> {
        if pin >= MEGA_ANALOG_PINS {
            Err(DeviceError::Unknown(format!("Invalid analog pin {} (Mega 2560 has A0-A15)", pin)))
        } else {
            Ok(())
        }
    }
    
    /// Check if pin supports PWM
    fn supports_pwm(&self, pin: u8) -> bool {
        MEGA_PWM_PINS.contains(&pin)
    }
    
    /// Check if pin supports interrupts
    fn supports_interrupt(&self, pin: u8) -> bool {
        MEGA_INTERRUPT_PINS.contains(&pin)
    }
    
    /// Send command (simulated due to Transport limitations)
    async fn send_command(&self, command: &str) -> DeviceResult<String> {
        debug!("Mega 2560 command: {}", command);
        
        // Simulate responses
        let response = if command.starts_with(CMD_PROBE) {
            "ARDUINO_MEGA_2560_V1".to_string()
        } else if command.starts_with(CMD_DIGITAL_READ) {
            "VALUE:1".to_string()
        } else if command.starts_with(CMD_ANALOG_READ) {
            "VALUE:512".to_string()
        } else {
            RESP_OK.to_string()
        };
        
        Ok(response)
    }
}

#[async_trait]
impl DeviceSession for ArduinoMega2560Session {
    fn session_id(&self) -> &str {
        &self.session_id
    }
    
    fn device_name(&self) -> &str {
        "Arduino Mega 2560"
    }
    
    async fn invoke_async(&mut self, endpoint: &str, args: Vec<Value>) -> DeviceResult<Value> {
        match endpoint {
            "pinMode" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                self.validate_digital_pin(pin)?;
                
                let mode_str = args.get(1)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DeviceError::Unknown("Missing mode argument".into()))?;
                
                let mode = match mode_str {
                    "INPUT" => PinMode::Input,
                    "OUTPUT" => PinMode::Output,
                    "PWM" => {
                        if !self.supports_pwm(pin) {
                            return Err(DeviceError::Unknown(format!("Pin {} doesn't support PWM", pin)));
                        }
                        PinMode::PwmOutput
                    },
                    "ANALOG" => PinMode::AnalogInput,
                    "INTERRUPT" => {
                        if !self.supports_interrupt(pin) {
                            return Err(DeviceError::Unknown(format!("Pin {} doesn't support interrupts", pin)));
                        }
                        PinMode::Interrupt
                    },
                    _ => return Err(DeviceError::Unknown(format!("Invalid pin mode: {}", mode_str))),
                };
                
                let mut modes = self.pin_modes.lock().await;
                modes.insert(pin, mode);
                
                let cmd = format!("{} {} {}", CMD_PIN_MODE, pin, mode_str);
                let response = self.send_command(&cmd).await?;
                
                if response == RESP_OK {
                    Ok(json!({ "success": true }))
                } else {
                    Err(DeviceError::Unknown(format!("Failed to set pin mode: {}", response)))
                }
            }
            
            "digitalWrite" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                self.validate_digital_pin(pin)?;
                
                let value = args.get(1)
                    .and_then(|v| v.as_bool())
                    .ok_or_else(|| DeviceError::Unknown("Missing value argument".into()))?;
                
                let cmd = format!("{} {} {}", CMD_DIGITAL_WRITE, pin, if value { 1 } else { 0 });
                self.send_command(&cmd).await?;
                
                Ok(json!({ "success": true }))
            }
            
            "digitalRead" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                self.validate_digital_pin(pin)?;
                
                let cmd = format!("{} {}", CMD_DIGITAL_READ, pin);
                let response = self.send_command(&cmd).await?;
                
                // Parse response
                let value = response.contains("1");
                Ok(json!({ "value": value }))
            }
            
            "analogRead" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                self.validate_analog_pin(pin)?;
                
                let cmd = format!("{} {}", CMD_ANALOG_READ, pin);
                let response = self.send_command(&cmd).await?;
                
                // Parse response: "VALUE:512"
                let value = if response.starts_with("VALUE:") {
                    response[6..].parse::<u16>().unwrap_or(512)
                } else {
                    512
                };
                
                Ok(json!({ "value": value }))
            }
            
            "pwmWrite" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                if !self.supports_pwm(pin) {
                    return Err(DeviceError::Unknown(format!("Pin {} doesn't support PWM on Mega 2560", pin)));
                }
                
                let duty = args.get(1)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing duty cycle argument".into()))? as u8;
                
                let cmd = format!("{} {} {}", CMD_PWM_WRITE, pin, duty);
                self.send_command(&cmd).await?;
                
                Ok(json!({ "success": true }))
            }
            
            "configureInterrupt" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                if !self.supports_interrupt(pin) {
                    return Err(DeviceError::Unknown(
                        format!("Pin {} doesn't support interrupts (use pins {:?})", pin, MEGA_INTERRUPT_PINS)
                    ));
                }
                
                let mode_str = args.get(1)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DeviceError::Unknown("Missing mode argument".into()))?;
                
                let cmd = format!("{} {} {}", CMD_HALL_CONFIG, pin, mode_str);
                self.send_command(&cmd).await?;
                
                Ok(json!({ "success": true }))
            }
            
            _ => Err(DeviceError::Unknown(format!("Unknown endpoint: {}", endpoint)))
        }
    }
    
    async fn subscribe_async(
        &mut self,
        stream: &str,
        handler: mpsc::UnboundedSender<StreamData>,
    ) -> DeviceResult<SubscriptionHandle> {
        // Implement telemetry streaming
        // For now, return a dummy subscription
        let (unsub_tx, _unsub_rx) = mpsc::channel(1);
        let handle = SubscriptionHandle::new(
            format!("mega_{}", uuid::Uuid::new_v4()),
            unsub_tx
        );
        Ok(handle)
    }
    
    async fn close_async(&mut self) -> DeviceResult<()> {
        let mut active = self.active.lock().await;
        *active = false;
        Ok(())
    }
    
    fn is_active(&self) -> bool {
        // Check if session is still active
        // Since we're using Arc<Mutex<bool>>, we need to try_lock
        self.active.try_lock().map(|guard| *guard).unwrap_or(false)
    }
    
    fn statistics(&self) -> SessionStatistics {
        // Return current statistics
        // Clone the stats since we can't hold the lock
        self.stats.try_lock()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| SessionStatistics::default())
    }
    
    async fn send_raw(&mut self, data: &[u8]) -> DeviceResult<Vec<u8>> {
        // Send raw data to device
        // NOTE: Transport trait limitations prevent actual implementation
        debug!("Mega 2560 raw send: {} bytes", data.len());
        
        // Update statistics
        let mut stats = self.stats.lock().await;
        stats.bytes_sent += data.len() as u64;
        stats.commands_sent += 1;
        stats.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Simulate response
        Ok(b"OK\r\n".to_vec())
    }
}