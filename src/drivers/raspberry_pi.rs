use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use serde_json::{Value, json};
use tokio::sync::{Mutex, mpsc};
use tracing::{info, debug, warn};

use crate::device::{
    DeviceDriver, DeviceSession, DeviceResult, DeviceError,
    Transport, TransportType, DriverCapabilities
};
use crate::device::session::{StreamData, SubscriptionHandle, SessionStatistics};

/// Raspberry Pi 3B+ GPIO constants
const RPI_GPIO_PINS: u8 = 40;  // Physical pins
const RPI_BCM_PINS: u8 = 28;   // BCM numbering (GPIO 0-27)

// BCM pins that support PWM
const RPI_PWM_PINS: &[u8] = &[12, 13, 18, 19];  // GPIO12, GPIO13, GPIO18, GPIO19

// I2C pins (BCM numbering)
const RPI_I2C_SDA: u8 = 2;   // GPIO2
const RPI_I2C_SCL: u8 = 3;   // GPIO3

// SPI pins (BCM numbering)
const RPI_SPI_MOSI: u8 = 10;  // GPIO10
const RPI_SPI_MISO: u8 = 9;   // GPIO9
const RPI_SPI_SCLK: u8 = 11;  // GPIO11
const RPI_SPI_CE0: u8 = 8;    // GPIO8
const RPI_SPI_CE1: u8 = 7;    // GPIO7

// UART pins (BCM numbering)
const RPI_UART_TXD: u8 = 14;  // GPIO14
const RPI_UART_RXD: u8 = 15;  // GPIO15

/// Command protocol for Raspberry Pi communication
const CMD_PROBE: &str = "PROBE";
const CMD_GPIO_MODE: &str = "GPIO_MODE";
const CMD_GPIO_WRITE: &str = "GPIO_WRITE";
const CMD_GPIO_READ: &str = "GPIO_READ";
const CMD_PWM_WRITE: &str = "PWM_WRITE";
const CMD_I2C_READ: &str = "I2C_READ";
const CMD_I2C_WRITE: &str = "I2C_WRITE";
const CMD_SPI_TRANSFER: &str = "SPI_TRANSFER";
const CMD_SYSTEM_INFO: &str = "SYSTEM_INFO";
const CMD_CPU_TEMP: &str = "CPU_TEMP";

const RESP_OK: &str = "OK";
const RESP_ERROR: &str = "ERROR";
const RESP_RPI_3B: &str = "RASPBERRY_PI_3B_V1";

/// Raspberry Pi 3B+ device driver
pub struct RaspberryPi3BDriver {
    name: String,
    version: String,
}

impl RaspberryPi3BDriver {
    pub fn new() -> Self {
        RaspberryPi3BDriver {
            name: "Raspberry Pi 3B+".to_string(),
            version: "1.0.0".to_string(),
        }
    }
    
    /// Detect Raspberry Pi over network (SSH/TCP)
    async fn detect_raspberry_pi(&self) -> DeviceResult<bool> {
        // In a real implementation, this would:
        // 1. Check for SSH availability on default port 22
        // 2. Try mDNS discovery for raspberrypi.local
        // 3. Check TCP connection on custom port
        debug!("Detecting Raspberry Pi 3B+ over network");
        Ok(true) // Simulated for now
    }
}

#[async_trait]
impl DeviceDriver for RaspberryPi3BDriver {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> &str {
        &self.version
    }
    
    fn supported_transports(&self) -> Vec<TransportType> {
        vec![TransportType::Tcp, TransportType::Ssh]
    }
    
    async fn probe_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<bool> {
        // Check if this is a Raspberry Pi
        if let Ok(is_pi) = self.detect_raspberry_pi().await {
            if !is_pi {
                debug!("No Raspberry Pi detected");
                return Ok(false);
            }
        }
        
        // Would send probe command to verify it's a Pi 3B+
        debug!("Raspberry Pi 3B+ device detected");
        Ok(true)
    }
    
    async fn open_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<Box<dyn DeviceSession>> {
        let session = RaspberryPi3BSession::new(transport);
        Ok(Box::new(session))
    }
    
    fn capabilities(&self) -> DriverCapabilities {
        DriverCapabilities {
            hot_plug: true,  // Network devices can hot-plug
            telemetry: true,
            pwm: true,
            gpio: true,
            analog_input: false,  // No built-in ADC
            serial_passthrough: true,
            firmware_update: true,  // Can update software
            requires_auth: true,  // SSH requires authentication
            max_data_rate: Some(100_000_000 / 8),  // 100 Mbps Ethernet
            min_latency_ms: Some(10),  // Network latency
        }
    }
}

/// Raspberry Pi 3B+ session implementation
pub struct RaspberryPi3BSession {
    transport: Arc<dyn Transport>,
    session_id: String,
    gpio_modes: Arc<Mutex<HashMap<u8, GpioMode>>>,
    pwm_channels: Arc<Mutex<HashMap<u8, PwmChannel>>>,
    active: Arc<Mutex<bool>>,
    stats: Arc<Mutex<SessionStatistics>>,
}

#[derive(Debug, Clone)]
enum GpioMode {
    Input,
    Output,
    PwmOutput,
    I2c,
    Spi,
    Uart,
    Alt0,
    Alt1,
    Alt2,
    Alt3,
    Alt4,
    Alt5,
}

#[derive(Debug, Clone)]
struct PwmChannel {
    frequency: u32,
    duty_cycle: f32,
}

impl RaspberryPi3BSession {
    fn new(transport: Arc<dyn Transport>) -> Self {
        RaspberryPi3BSession {
            transport,
            session_id: uuid::Uuid::new_v4().to_string(),
            gpio_modes: Arc::new(Mutex::new(HashMap::new())),
            pwm_channels: Arc::new(Mutex::new(HashMap::new())),
            active: Arc::new(Mutex::new(true)),
            stats: Arc::new(Mutex::new(SessionStatistics::new())),
        }
    }
    
    /// Validate BCM GPIO pin number
    fn validate_gpio_pin(&self, pin: u8) -> DeviceResult<()> {
        if pin >= RPI_BCM_PINS {
            Err(DeviceError::Unknown(format!("Invalid GPIO pin {} (Pi 3B+ has GPIO 0-27)", pin)))
        } else {
            Ok(())
        }
    }
    
    /// Check if pin supports hardware PWM
    fn supports_pwm(&self, pin: u8) -> bool {
        RPI_PWM_PINS.contains(&pin)
    }
    
    /// Check if pin is reserved for I2C
    fn is_i2c_pin(&self, pin: u8) -> bool {
        pin == RPI_I2C_SDA || pin == RPI_I2C_SCL
    }
    
    /// Check if pin is reserved for SPI
    fn is_spi_pin(&self, pin: u8) -> bool {
        pin == RPI_SPI_MOSI || pin == RPI_SPI_MISO || pin == RPI_SPI_SCLK ||
        pin == RPI_SPI_CE0 || pin == RPI_SPI_CE1
    }
    
    /// Check if pin is reserved for UART
    fn is_uart_pin(&self, pin: u8) -> bool {
        pin == RPI_UART_TXD || pin == RPI_UART_RXD
    }
    
    /// Send command (simulated due to Transport limitations)
    async fn send_command(&self, command: &str) -> DeviceResult<String> {
        debug!("Raspberry Pi command: {}", command);
        
        // Simulate responses
        let response = if command.starts_with(CMD_PROBE) {
            RESP_RPI_3B.to_string()
        } else if command.starts_with(CMD_GPIO_READ) {
            "VALUE:1".to_string()
        } else if command.starts_with(CMD_CPU_TEMP) {
            "TEMP:42.5".to_string()  // CPU temperature in Celsius
        } else if command.starts_with(CMD_SYSTEM_INFO) {
            json!({
                "model": "Raspberry Pi 3 Model B Plus Rev 1.3",
                "cpu": "BCM2837B0",
                "cores": 4,
                "ram_mb": 1024,
                "os": "Raspbian GNU/Linux 11",
                "kernel": "5.15.61-v7+",
                "uptime_seconds": 86400
            }).to_string()
        } else {
            RESP_OK.to_string()
        };
        
        Ok(response)
    }
}

#[async_trait]
impl DeviceSession for RaspberryPi3BSession {
    fn session_id(&self) -> &str {
        &self.session_id
    }
    
    fn device_name(&self) -> &str {
        "Raspberry Pi 3B+"
    }
    
    async fn invoke_async(&mut self, endpoint: &str, args: Vec<Value>) -> DeviceResult<Value> {
        match endpoint {
            "gpioMode" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                self.validate_gpio_pin(pin)?;
                
                let mode_str = args.get(1)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DeviceError::Unknown("Missing mode argument".into()))?;
                
                // Check for special pin reservations
                let mode = match mode_str {
                    "INPUT" => GpioMode::Input,
                    "OUTPUT" => GpioMode::Output,
                    "PWM" => {
                        if !self.supports_pwm(pin) {
                            return Err(DeviceError::Unknown(
                                format!("GPIO{} doesn't support hardware PWM (use GPIO {:?})", pin, RPI_PWM_PINS)
                            ));
                        }
                        GpioMode::PwmOutput
                    },
                    "I2C" => {
                        if !self.is_i2c_pin(pin) {
                            return Err(DeviceError::Unknown(
                                format!("GPIO{} is not an I2C pin (use GPIO2/3)", pin)
                            ));
                        }
                        GpioMode::I2c
                    },
                    "SPI" => {
                        if !self.is_spi_pin(pin) {
                            return Err(DeviceError::Unknown(
                                format!("GPIO{} is not a SPI pin", pin)
                            ));
                        }
                        GpioMode::Spi
                    },
                    "UART" => {
                        if !self.is_uart_pin(pin) {
                            return Err(DeviceError::Unknown(
                                format!("GPIO{} is not a UART pin (use GPIO14/15)", pin)
                            ));
                        }
                        GpioMode::Uart
                    },
                    _ => return Err(DeviceError::Unknown(format!("Invalid GPIO mode: {}", mode_str))),
                };
                
                let mut modes = self.gpio_modes.lock().await;
                modes.insert(pin, mode);
                
                let cmd = format!("{} {} {}", CMD_GPIO_MODE, pin, mode_str);
                self.send_command(&cmd).await?;
                
                Ok(json!({ "success": true }))
            }
            
            "gpioWrite" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                self.validate_gpio_pin(pin)?;
                
                let value = args.get(1)
                    .and_then(|v| v.as_bool())
                    .ok_or_else(|| DeviceError::Unknown("Missing value argument".into()))?;
                
                let cmd = format!("{} {} {}", CMD_GPIO_WRITE, pin, if value { 1 } else { 0 });
                self.send_command(&cmd).await?;
                
                Ok(json!({ "success": true }))
            }
            
            "gpioRead" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                self.validate_gpio_pin(pin)?;
                
                let cmd = format!("{} {}", CMD_GPIO_READ, pin);
                let response = self.send_command(&cmd).await?;
                
                let value = response.contains("1");
                Ok(json!({ "value": value }))
            }
            
            "pwmWrite" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                if !self.supports_pwm(pin) {
                    return Err(DeviceError::Unknown(
                        format!("GPIO{} doesn't support hardware PWM", pin)
                    ));
                }
                
                let frequency = args.get(1)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1000) as u32;  // Default 1kHz
                
                let duty_cycle = args.get(2)
                    .and_then(|v| v.as_f64())
                    .unwrap_or(50.0) as f32;  // Default 50%
                
                if duty_cycle < 0.0 || duty_cycle > 100.0 {
                    return Err(DeviceError::Unknown("Duty cycle must be 0-100%".into()));
                }
                
                let mut channels = self.pwm_channels.lock().await;
                channels.insert(pin, PwmChannel { frequency, duty_cycle });
                
                let cmd = format!("{} {} {} {}", CMD_PWM_WRITE, pin, frequency, duty_cycle);
                self.send_command(&cmd).await?;
                
                Ok(json!({ "success": true }))
            }
            
            "i2cRead" => {
                let address = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing I2C address".into()))? as u8;
                
                let register = args.get(1)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u8;
                
                let length = args.get(2)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as usize;
                
                let cmd = format!("{} 0x{:02X} 0x{:02X} {}", CMD_I2C_READ, address, register, length);
                let response = self.send_command(&cmd).await?;
                
                // Simulate data response
                let data = vec![0x42u8; length];
                Ok(json!({ "data": data }))
            }
            
            "i2cWrite" => {
                let address = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing I2C address".into()))? as u8;
                
                let register = args.get(1)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing register".into()))? as u8;
                
                let data = args.get(2)
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| DeviceError::Unknown("Missing data array".into()))?;
                
                let cmd = format!("{} 0x{:02X} 0x{:02X} {:?}", CMD_I2C_WRITE, address, register, data);
                self.send_command(&cmd).await?;
                
                Ok(json!({ "success": true }))
            }
            
            "spiTransfer" => {
                let chip_select = args.get(0)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u8;  // CE0 by default
                
                let data = args.get(1)
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| DeviceError::Unknown("Missing data array".into()))?;
                
                let cmd = format!("{} {} {:?}", CMD_SPI_TRANSFER, chip_select, data);
                let response = self.send_command(&cmd).await?;
                
                // Simulate received data
                let received = vec![0x00u8; data.len()];
                Ok(json!({ "received": received }))
            }
            
            "getCpuTemperature" => {
                let cmd = CMD_CPU_TEMP.to_string();
                let response = self.send_command(&cmd).await?;
                
                // Parse "TEMP:42.5"
                let temp = if response.starts_with("TEMP:") {
                    response[5..].parse::<f32>().unwrap_or(0.0)
                } else {
                    0.0
                };
                
                Ok(json!({ "temperature_celsius": temp }))
            }
            
            "getSystemInfo" => {
                let cmd = CMD_SYSTEM_INFO.to_string();
                let response = self.send_command(&cmd).await?;
                
                // Try to parse as JSON
                if let Ok(info) = serde_json::from_str::<Value>(&response) {
                    Ok(info)
                } else {
                    Ok(json!({ "raw": response }))
                }
            }
            
            "executeCommand" => {
                // Execute arbitrary Linux command (with caution!)
                let command = args.get(0)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DeviceError::Unknown("Missing command".into()))?;
                
                // Safety check - disallow dangerous commands
                let blocked_commands = ["rm", "dd", "mkfs", "format", "shutdown", "reboot"];
                for blocked in blocked_commands {
                    if command.contains(blocked) {
                        return Err(DeviceError::Unknown(format!("Blocked command: {}", blocked)));
                    }
                }
                
                // Would execute command via SSH
                debug!("Would execute command: {}", command);
                
                Ok(json!({ "output": "Command execution simulated" }))
            }
            
            _ => Err(DeviceError::Unknown(format!("Unknown endpoint: {}", endpoint)))
        }
    }
    
    async fn subscribe_async(
        &mut self,
        stream: &str,
        handler: mpsc::UnboundedSender<StreamData>,
    ) -> DeviceResult<SubscriptionHandle> {
        // Could implement GPIO event streaming, CPU monitoring, etc.
        let (unsub_tx, _unsub_rx) = mpsc::channel(1);
        let handle = SubscriptionHandle::new(
            format!("rpi_stream_{}", uuid::Uuid::new_v4()),
            unsub_tx
        );
        debug!("Subscription created for stream: {}", stream);
        Ok(handle)
    }
    
    async fn close_async(&mut self) -> DeviceResult<()> {
        let mut active = self.active.lock().await;
        *active = false;
        Ok(())
    }
    
    fn is_active(&self) -> bool {
        // Check if session is still active
        self.active.try_lock().map(|guard| *guard).unwrap_or(false)
    }
    
    fn statistics(&self) -> SessionStatistics {
        // Return current statistics
        self.stats.try_lock()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| SessionStatistics::default())
    }
    
    async fn send_raw(&mut self, data: &[u8]) -> DeviceResult<Vec<u8>> {
        // Send raw command to Raspberry Pi
        // NOTE: This would use SSH/TCP transport in real implementation
        debug!("RPi raw send: {} bytes", data.len());
        
        // Update statistics
        let mut stats = self.stats.lock().await;
        stats.bytes_sent += data.len() as u64;
        stats.commands_sent += 1;
        stats.last_activity = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Parse command and simulate response
        let cmd = String::from_utf8_lossy(data);
        if cmd.starts_with("GPIO_READ") {
            Ok(b"1\n".to_vec())
        } else if cmd.starts_with("I2C_READ") {
            Ok(vec![0x00, 0x42])  // Example I2C data
        } else {
            Ok(b"OK\n".to_vec())
        }
    }
}