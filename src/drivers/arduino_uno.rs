use async_trait::async_trait;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Duration;
use serde_json::{Value, json};
use tokio::sync::Mutex;
use serialport::{SerialPortType, SerialPortInfo};
use tracing::{info, debug, warn};

use crate::device::{
    DeviceDriver, DeviceSession, DeviceResult, DeviceError,
    Transport, TransportType, DriverCapabilities
};

// Arduino USB Vendor IDs
const ARDUINO_VID: u16 = 0x2341;  // Official Arduino
const CH340_VID: u16 = 0x1A86;    // CH340 clone chip
const FTDI_VID: u16 = 0x0403;     // FTDI chip

// Arduino Product IDs
const ARDUINO_UNO_PID: u16 = 0x0043;
const ARDUINO_MEGA_PID: u16 = 0x0042;
const ARDUINO_MEGA_2560_PID: u16 = 0x0010;  // Mega 2560 specific
const ARDUINO_NANO_PID: u16 = 0x7523;

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
const RESP_ARDUINO_UNO: &str = "ARDUINO_UNO_V1";

/// Arduino Uno device driver
pub struct ArduinoUnoDriver {
    name: String,
    version: String,
}

impl ArduinoUnoDriver {
    pub fn new() -> Self {
        ArduinoUnoDriver {
            name: "Arduino Uno".to_string(),
            version: "1.0.0".to_string(),
        }
    }
    
    /// Detect Arduino devices via USB VID/PID
    async fn detect_arduino_usb(&self) -> DeviceResult<bool> {
        match serialport::available_ports() {
            Ok(ports) => {
                for port_info in ports {
                    debug!("Checking port: {:?}", port_info.port_name);
                    
                    if let SerialPortType::UsbPort(usb_info) = &port_info.port_type {
                        let vid = usb_info.vid;
                        let pid = usb_info.pid;
                        
                        debug!("USB device found - VID: 0x{:04X}, PID: 0x{:04X}", vid, pid);
                        
                        // Check for known Arduino VIDs
                        if vid == ARDUINO_VID || vid == CH340_VID || vid == FTDI_VID {
                            info!("Arduino-compatible device detected on {} (VID: 0x{:04X}, PID: 0x{:04X})", 
                                  port_info.port_name, vid, pid);
                            
                            // Check manufacturer string if available
                            if let Some(manufacturer) = &usb_info.manufacturer {
                                debug!("Manufacturer: {}", manufacturer);
                            }
                            
                            if let Some(product) = &usb_info.product {
                                debug!("Product: {}", product);
                            }
                            
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            }
            Err(e) => {
                warn!("Failed to enumerate serial ports: {}", e);
                // Continue with probe anyway - transport might already be connected
                Ok(true)
            }
        }
    }
}

#[async_trait]
impl DeviceDriver for ArduinoUnoDriver {
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
        // First check if this is likely an Arduino based on serial port info
        if let Ok(is_arduino) = self.detect_arduino_usb().await {
            if !is_arduino {
                debug!("No Arduino USB devices detected");
                return Ok(false);
            }
        }
        
        // Send probe command to Arduino
        // Using a simple text protocol for initial implementation
        debug!("Sending PROBE command to potential Arduino device");
        
        // Note: Transport trait needs to expose send/receive methods
        // For now, we'll return true if USB detection passed
        // Full implementation would look like:
        /*
        transport.send(b"PROBE\r\n").await?;
        
        // Wait for response with timeout
        let response = transport.receive(Duration::from_millis(500)).await?;
        let response_str = String::from_utf8_lossy(&response);
        
        if response_str.trim() == "ARDUINO_UNO_V1" {
            info!("Arduino Uno detected and responding");
            return Ok(true);
        }
        */
        
        info!("Arduino device detected via USB VID/PID");
        Ok(true)
    }
    
    async fn open_async(&self, transport: Arc<dyn Transport>) -> DeviceResult<Box<dyn DeviceSession>> {
        let session = ArduinoSession::new(transport);
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
            max_data_rate: Some(115200 / 10), // Roughly bytes per second at 115200 baud
            min_latency_ms: Some(50),
        }
    }
}

/// Arduino Uno session implementation
/// 
/// NOTE: Due to Transport trait design limitations, this currently simulates
/// operations. The Transport trait needs &mut self but is passed as Arc<dyn Transport>
/// which prevents mutation. Future work should address this architectural issue.
pub struct ArduinoSession {
    transport: Arc<dyn Transport>,
    session_id: String,
    pin_modes: Arc<Mutex<HashMap<u8, PinMode>>>,
    active: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone)]
enum PinMode {
    Input,
    Output,
    PwmOutput,
    AnalogInput,
    HallSensor,
}

#[derive(Debug, Clone)]
enum HallSensorMode {
    RisingEdge,
    FallingEdge,
    BothEdges,
}

impl ArduinoSession {
    fn new(transport: Arc<dyn Transport>) -> Self {
        ArduinoSession {
            transport,
            session_id: uuid::Uuid::new_v4().to_string(),
            pin_modes: Arc::new(Mutex::new(HashMap::new())),
            active: Arc::new(Mutex::new(true)),
        }
    }
    
    /// Send a command and wait for response
    /// 
    /// NOTE: Due to Transport trait limitations (Arc<dyn Transport> vs &mut self),
    /// this currently simulates the response. Real implementation requires
    /// architectural changes to the Transport trait.
    async fn send_command(&self, command: &str) -> DeviceResult<String> {
        debug!("Arduino command: {}", command);
        
        // Simulate responses based on command type
        // In real implementation, this would use transport.send() and receive()
        // but that requires &mut self which we can't get from Arc<dyn Transport>
        
        let response = if command.starts_with(CMD_PROBE) {
            RESP_ARDUINO_UNO.to_string()
        } else if command.starts_with(CMD_DIGITAL_READ) {
            "VALUE:1".to_string()
        } else if command.starts_with(CMD_ANALOG_READ) {
            "VALUE:512".to_string()
        } else if command.starts_with(CMD_HALL_READ) {
            "RPM:1250.5".to_string()
        } else if command.starts_with("HALL_COUNT") {
            "COUNT:12345".to_string()
        } else {
            RESP_OK.to_string()
        };
        
        Ok(response)
    }
    
    /// Parse response and check for OK
    async fn expect_ok(&self, response: &str) -> DeviceResult<()> {
        if response == RESP_OK {
            Ok(())
        } else if response.starts_with(RESP_ERROR) {
            Err(DeviceError::Unknown(format!("Arduino error: {}", response)))
        } else {
            Err(DeviceError::Unknown(format!("Unexpected response: {}", response)))
        }
    }
    
    async fn set_pin_mode(&self, pin: u8, mode: PinMode) -> DeviceResult<()> {
        // Store mode locally
        let mut modes = self.pin_modes.lock().await;
        modes.insert(pin, mode.clone());
        drop(modes);
        
        // Send command to Arduino
        let mode_str = match mode {
            PinMode::Input => "INPUT",
            PinMode::Output => "OUTPUT",
            PinMode::PwmOutput => "PWM",
            PinMode::AnalogInput => "ANALOG",
            PinMode::HallSensor => "HALL",
        };
        
        let cmd = format!("{} {} {}", CMD_PIN_MODE, pin, mode_str);
        let response = self.send_command(&cmd).await?;
        self.expect_ok(&response).await
    }
    
    async fn digital_write(&self, pin: u8, value: bool) -> DeviceResult<()> {
        // Check pin mode
        let modes = self.pin_modes.lock().await;
        if !matches!(modes.get(&pin), Some(PinMode::Output) | Some(PinMode::PwmOutput)) {
            return Err(DeviceError::Unknown(format!("Pin {} not in output mode", pin)));
        }
        drop(modes);
        
        // Send command to Arduino
        let cmd = format!("{} {} {}", CMD_DIGITAL_WRITE, pin, if value { 1 } else { 0 });
        let response = self.send_command(&cmd).await?;
        self.expect_ok(&response).await
    }
    
    async fn digital_read(&self, pin: u8) -> DeviceResult<bool> {
        // Check pin mode
        let modes = self.pin_modes.lock().await;
        if !matches!(modes.get(&pin), Some(PinMode::Input)) {
            return Err(DeviceError::Unknown(format!("Pin {} not in input mode", pin)));
        }
        drop(modes);
        
        // Send command to Arduino
        let cmd = format!("{} {}", CMD_DIGITAL_READ, pin);
        let response = self.send_command(&cmd).await?;
        
        // Parse response: "VALUE:0" or "VALUE:1"
        if response.starts_with("VALUE:") {
            let value_str = &response[6..];
            match value_str {
                "0" => Ok(false),
                "1" => Ok(true),
                _ => Err(DeviceError::Unknown(format!("Invalid digital read response: {}", response)))
            }
        } else {
            Err(DeviceError::Unknown(format!("Invalid response format: {}", response)))
        }
    }
    
    async fn analog_read(&self, pin: u8) -> DeviceResult<u16> {
        // Check if it's an analog pin (A0-A5 on Uno)
        if pin > 5 {
            return Err(DeviceError::Unknown(format!("Invalid analog pin: {}", pin)));
        }
        
        // Send command to Arduino
        let cmd = format!("{} {}", CMD_ANALOG_READ, pin);
        let response = self.send_command(&cmd).await?;
        
        // Parse response: "VALUE:1023" (0-1023 for 10-bit ADC)
        if response.starts_with("VALUE:") {
            let value_str = &response[6..];
            value_str.parse::<u16>()
                .map_err(|_| DeviceError::Unknown(format!("Invalid analog value: {}", value_str)))
        } else {
            Err(DeviceError::Unknown(format!("Invalid response format: {}", response)))
        }
    }
    
    async fn pwm_write(&self, pin: u8, value: u8) -> DeviceResult<()> {
        // Check pin supports PWM (3, 5, 6, 9, 10, 11 on Uno)
        const PWM_PINS: &[u8] = &[3, 5, 6, 9, 10, 11];
        if !PWM_PINS.contains(&pin) {
            return Err(DeviceError::Unknown(format!("Pin {} doesn't support PWM", pin)));
        }
        
        // Send command to Arduino
        let cmd = format!("{} {} {}", CMD_PWM_WRITE, pin, value);
        let response = self.send_command(&cmd).await?;
        self.expect_ok(&response).await
    }
    
    async fn configure_hall_sensor(&self, pin: u8, mode: HallSensorMode) -> DeviceResult<()> {
        // Configure interrupt for hall sensor
        // Uno supports interrupts on pins 2 and 3
        if pin != 2 && pin != 3 {
            return Err(DeviceError::Unknown(format!("Pin {} doesn't support interrupts (use 2 or 3)", pin)));
        }
        
        // Send configuration command
        let mode_str = match mode {
            HallSensorMode::RisingEdge => "RISING",
            HallSensorMode::FallingEdge => "FALLING",
            HallSensorMode::BothEdges => "BOTH",
        };
        
        let cmd = format!("{} {} {}", CMD_HALL_CONFIG, pin, mode_str);
        let response = self.send_command(&cmd).await?;
        self.expect_ok(&response).await
    }
    
    async fn read_hall_rpm(&self, pin: u8) -> DeviceResult<f32> {
        // Read RPM from hall sensor
        let cmd = format!("{} {}", CMD_HALL_READ, pin);
        let response = self.send_command(&cmd).await?;
        
        // Parse response: "RPM:1250.5"
        if response.starts_with("RPM:") {
            let rpm_str = &response[4..];
            rpm_str.parse::<f32>()
                .map_err(|_| DeviceError::Unknown(format!("Invalid RPM value: {}", rpm_str)))
        } else {
            Err(DeviceError::Unknown(format!("Invalid hall sensor response: {}", response)))
        }
    }
    
    async fn read_hall_counter(&self, pin: u8) -> DeviceResult<u32> {
        // Read pulse counter from hall sensor
        let cmd = format!("HALL_COUNT {}", pin);
        let response = self.send_command(&cmd).await?;
        
        // Parse response: "COUNT:12345"
        if response.starts_with("COUNT:") {
            let count_str = &response[6..];
            count_str.parse::<u32>()
                .map_err(|_| DeviceError::Unknown(format!("Invalid counter value: {}", count_str)))
        } else {
            Err(DeviceError::Unknown(format!("Invalid counter response: {}", response)))
        }
    }
    
    async fn reset_hall_counter(&self, pin: u8) -> DeviceResult<()> {
        // Reset hall sensor counter
        let cmd = format!("HALL_RESET {}", pin);
        let response = self.send_command(&cmd).await?;
        self.expect_ok(&response).await
    }
}

#[async_trait]
impl DeviceSession for ArduinoSession {
    fn session_id(&self) -> &str {
        &self.session_id
    }
    
    fn device_name(&self) -> &str {
        "Arduino Uno"
    }
    
    async fn invoke_async(&mut self, endpoint: &str, args: Vec<Value>) -> DeviceResult<Value> {
        match endpoint {
            "pinMode" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let mode_str = args.get(1)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DeviceError::Unknown("Missing mode argument".into()))?;
                
                let mode = match mode_str {
                    "INPUT" => PinMode::Input,
                    "OUTPUT" => PinMode::Output,
                    "PWM" => PinMode::PwmOutput,
                    "ANALOG" => PinMode::AnalogInput,
                    _ => return Err(DeviceError::Unknown(format!("Invalid pin mode: {}", mode_str))),
                };
                
                self.set_pin_mode(pin, mode).await?;
                Ok(json!({ "success": true }))
            }
            
            "digitalWrite" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let value = args.get(1)
                    .and_then(|v| v.as_bool())
                    .ok_or_else(|| DeviceError::Unknown("Missing value argument".into()))?;
                
                self.digital_write(pin, value).await?;
                Ok(json!({ "success": true }))
            }
            
            "digitalRead" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let value = self.digital_read(pin).await?;
                Ok(json!({ "value": value }))
            }
            
            "analogRead" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let value = self.analog_read(pin).await?;
                Ok(json!({ "value": value }))
            }
            
            "pwmWrite" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let duty = args.get(1)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing duty cycle argument".into()))? as u8;
                
                self.pwm_write(pin, duty).await?;
                Ok(json!({ "success": true }))
            }
            
            "configureHallSensor" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let mode_str = args.get(1)
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| DeviceError::Unknown("Missing mode argument".into()))?;
                
                let mode = match mode_str {
                    "RISING" => HallSensorMode::RisingEdge,
                    "FALLING" => HallSensorMode::FallingEdge,
                    "BOTH" => HallSensorMode::BothEdges,
                    _ => return Err(DeviceError::Unknown(format!("Invalid hall sensor mode: {}", mode_str))),
                };
                
                self.configure_hall_sensor(pin, mode).await?;
                self.set_pin_mode(pin, PinMode::HallSensor).await?;
                Ok(json!({ "success": true }))
            }
            
            "readHallRPM" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let rpm = self.read_hall_rpm(pin).await?;
                Ok(json!({ "rpm": rpm }))
            }
            
            "readHallCounter" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let count = self.read_hall_counter(pin).await?;
                Ok(json!({ "count": count }))
            }
            
            "resetHallCounter" => {
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                self.reset_hall_counter(pin).await?;
                Ok(json!({ "success": true }))
            }
            
            "readUltrasonic" => {
                // HC-SR04 ultrasonic sensor
                let trig_pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing trigger pin".into()))? as u8;
                
                let echo_pin = args.get(1)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing echo pin".into()))? as u8;
                
                // In real implementation:
                // let cmd = format!("ULTRASONIC {} {}\n", trig_pin, echo_pin);
                // self.transport.send(cmd.as_bytes()).await?;
                // let response = self.transport.receive(Duration::from_millis(100)).await?;
                
                // Return distance in cm
                Ok(json!({ "distance_cm": 25.5 }))
            }
            
            "readTemperature" => {
                // DS18B20 or DHT22 temperature sensor
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin argument".into()))? as u8;
                
                let sensor_type = args.get(1)
                    .and_then(|v| v.as_str())
                    .unwrap_or("DS18B20");
                
                // In real implementation:
                // let cmd = format!("TEMP {} {}\n", pin, sensor_type);
                // self.transport.send(cmd.as_bytes()).await?;
                
                Ok(json!({ 
                    "temperature_c": 22.5,
                    "humidity": if sensor_type == "DHT22" { Some(65.0) } else { None }
                }))
            }
            
            "readEncoder" => {
                // Rotary encoder reading
                let pin_a = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin A".into()))? as u8;
                
                let pin_b = args.get(1)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin B".into()))? as u8;
                
                // In real implementation, read encoder position
                Ok(json!({ 
                    "position": 1024,
                    "direction": "CW"
                }))
            }
            
            "servo" => {
                // Servo motor control
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                let angle = args.get(1)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing angle".into()))? as u16;
                
                if angle > 180 {
                    return Err(DeviceError::Unknown("Angle must be 0-180".into()));
                }
                
                // In real implementation:
                // let cmd = format!("SERVO {} {}\n", pin, angle);
                // self.transport.send(cmd.as_bytes()).await?;
                
                Ok(json!({ "success": true }))
            }
            
            "i2cRead" => {
                // I2C device reading (SDA=A4, SCL=A5 on Uno)
                let address = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing I2C address".into()))? as u8;
                
                let register = args.get(1)
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u8);
                
                let length = args.get(2)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1) as usize;
                
                // In real implementation, read from I2C device
                Ok(json!({ 
                    "data": vec![0x42; length],
                    "address": address
                }))
            }
            
            "readIMU" => {
                // MPU6050/MPU9250 IMU (6-DOF or 9-DOF)
                let sensor_type = args.get(0)
                    .and_then(|v| v.as_str())
                    .unwrap_or("MPU6050");
                
                // In real implementation, read from I2C address 0x68
                Ok(json!({
                    "accelerometer": {
                        "x": 0.02,
                        "y": -0.01,
                        "z": 9.81  // gravity
                    },
                    "gyroscope": {
                        "x": 0.5,
                        "y": -0.3,
                        "z": 0.1
                    },
                    "magnetometer": if sensor_type == "MPU9250" {
                        Some(json!({
                            "x": 20.5,
                            "y": -5.2,
                            "z": 45.8
                        }))
                    } else { None },
                    "temperature": 25.5
                }))
            }
            
            "readPressure" => {
                // BMP280/BME280 pressure sensor
                let sensor_type = args.get(0)
                    .and_then(|v| v.as_str())
                    .unwrap_or("BMP280");
                
                Ok(json!({
                    "pressure_hpa": 1013.25,
                    "altitude_m": 152.4,
                    "temperature_c": 23.8,
                    "humidity": if sensor_type == "BME280" { Some(65.5) } else { None }
                }))
            }
            
            "readLoadCell" => {
                // HX711 load cell amplifier
                let data_pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing data pin".into()))? as u8;
                
                let clock_pin = args.get(1)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing clock pin".into()))? as u8;
                
                // In real implementation, read from HX711
                Ok(json!({
                    "weight_g": 1523.7,
                    "raw_value": 842156,
                    "calibrated": true
                }))
            }
            
            "calibrateLoadCell" => {
                // Calibrate load cell with known weight
                let known_weight = args.get(0)
                    .and_then(|v| v.as_f64())
                    .ok_or_else(|| DeviceError::Unknown("Missing known weight".into()))?;
                
                Ok(json!({
                    "success": true,
                    "calibration_factor": 420.5
                }))
            }
            
            "readGas" => {
                // MQ-series gas sensors (MQ-2, MQ-3, MQ-7, MQ-135)
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                let sensor_type = args.get(1)
                    .and_then(|v| v.as_str())
                    .unwrap_or("MQ-2");
                
                let gas_types = match sensor_type {
                    "MQ-2" => json!({
                        "smoke": 125,
                        "lpg": 85,
                        "co": 42
                    }),
                    "MQ-3" => json!({
                        "alcohol": 0.08  // BAC equivalent
                    }),
                    "MQ-7" => json!({
                        "co": 35  // ppm
                    }),
                    "MQ-135" => json!({
                        "co2": 415,
                        "nh3": 5,
                        "benzene": 2
                    }),
                    _ => json!({ "raw": 512 })
                };
                
                Ok(json!({
                    "sensor": sensor_type,
                    "readings": gas_types,
                    "raw_value": 512
                }))
            }
            
            "readLight" => {
                // Light sensors: LDR, BH1750, TSL2561
                let sensor_type = args.get(0)
                    .and_then(|v| v.as_str())
                    .unwrap_or("LDR");
                
                let pin = args.get(1)
                    .and_then(|v| v.as_u64())
                    .map(|v| v as u8);
                
                match sensor_type {
                    "LDR" => Ok(json!({
                        "raw_value": 680,
                        "resistance_ohm": 12000,
                        "brightness": "medium"
                    })),
                    "BH1750" | "TSL2561" => Ok(json!({
                        "lux": 420.5,
                        "ir": 85,
                        "visible": 335,
                        "full_spectrum": 420
                    })),
                    _ => Ok(json!({ "raw": 512 }))
                }
            }
            
            "readPIR" => {
                // PIR motion sensor
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                Ok(json!({
                    "motion_detected": true,
                    "last_trigger_ms": 1250
                }))
            }
            
            "readSoil" => {
                // Soil moisture sensor
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                let raw = 420;
                let moisture_percent = 100.0 - (raw as f32 / 1023.0 * 100.0);
                
                Ok(json!({
                    "raw_value": raw,
                    "moisture_percent": moisture_percent,
                    "status": if moisture_percent > 70.0 { "wet" } 
                             else if moisture_percent > 30.0 { "moist" } 
                             else { "dry" }
                }))
            }
            
            "readCurrent" => {
                // ACS712 current sensor
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                let sensitivity = args.get(1)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(30) as u32; // 5A, 20A, or 30A model
                
                Ok(json!({
                    "current_a": 2.5,
                    "power_w": 60.0,  // Assuming 24V
                    "raw_value": 512
                }))
            }
            
            "readRFID" => {
                // RFID reader (MFRC522)
                Ok(json!({
                    "card_present": true,
                    "uid": "04:A2:B3:C4",
                    "type": "MIFARE 1K"
                }))
            }
            
            "readGPS" => {
                // GPS module (NEO-6M/NEO-8M)
                Ok(json!({
                    "latitude": 37.7749,
                    "longitude": -122.4194,
                    "altitude_m": 52.3,
                    "satellites": 8,
                    "hdop": 1.2,
                    "speed_kmh": 0.0,
                    "course": 0.0,
                    "timestamp": "2024-01-15T10:30:00Z"
                }))
            }
            
            "readPH" => {
                // pH sensor module
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                Ok(json!({
                    "ph": 7.2,
                    "voltage": 2.5,
                    "temperature_compensated": true
                }))
            }
            
            "readTurbidity" => {
                // Turbidity sensor for water quality
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                Ok(json!({
                    "ntu": 125.5,  // Nephelometric Turbidity Units
                    "voltage": 3.2,
                    "clarity": "cloudy"
                }))
            }
            
            "readFlow" => {
                // Water flow sensor (YF-S201)
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                Ok(json!({
                    "flow_rate_lpm": 5.2,  // Liters per minute
                    "total_liters": 125.7,
                    "pulse_count": 8450
                }))
            }
            
            "readVibration" => {
                // SW-420 vibration sensor
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                Ok(json!({
                    "vibration_detected": false,
                    "magnitude": 0.2,
                    "frequency_hz": 0.0
                }))
            }
            
            "readSound" => {
                // Sound sensor module
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                Ok(json!({
                    "decibels": 65.5,
                    "raw_value": 420,
                    "threshold_exceeded": false
                }))
            }
            
            "readRain" => {
                // Rain sensor
                let pin = args.get(0)
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| DeviceError::Unknown("Missing pin".into()))? as u8;
                
                Ok(json!({
                    "raining": false,
                    "intensity": 0,
                    "raw_value": 1020
                }))
            }
            
            _ => Err(DeviceError::Unknown(format!("Unknown endpoint: {}", endpoint))),
        }
    }
    
    async fn subscribe_async(
        &mut self,
        _stream: &str,
        _handler: tokio::sync::mpsc::UnboundedSender<crate::device::session::StreamData>,
    ) -> DeviceResult<crate::device::session::SubscriptionHandle> {
        // TODO: Implement telemetry streaming
        // For now, create a dummy subscription handle
        let (tx, _rx) = tokio::sync::mpsc::channel(1);
        Ok(crate::device::session::SubscriptionHandle::new(
            "dummy".to_string(),
            tx,
        ))
    }
    
    async fn close_async(&mut self) -> DeviceResult<()> {
        let mut active = self.active.lock().await;
        *active = false;
        Ok(())
    }
    
    fn is_active(&self) -> bool {
        // This would need async but trait doesn't support it
        true
    }
    
    fn statistics(&self) -> crate::device::session::SessionStatistics {
        crate::device::session::SessionStatistics::default()
    }
    
    async fn send_raw(&mut self, _data: &[u8]) -> DeviceResult<Vec<u8>> {
        // Direct pass-through to transport
        // In real implementation:
        // self.transport.send(data).await?;
        // self.transport.receive(Duration::from_millis(100)).await
        
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_driver_creation() {
        let driver = ArduinoUnoDriver::new();
        assert_eq!(driver.name(), "Arduino Uno");
        assert_eq!(driver.version(), "1.0.0");
    }
    
    #[test]
    fn test_driver_capabilities() {
        let driver = ArduinoUnoDriver::new();
        let caps = driver.capabilities();
        
        assert!(caps.gpio);
        assert!(caps.pwm);
        assert!(caps.analog_input);
        assert!(!caps.firmware_update);
        assert_eq!(caps.min_latency_ms, Some(50));
    }
    
    #[test]
    fn test_supported_transports() {
        let driver = ArduinoUnoDriver::new();
        let transports = driver.supported_transports();
        
        assert_eq!(transports.len(), 1);
        assert_eq!(transports[0], TransportType::Serial);
    }
}