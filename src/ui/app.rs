use egui::{Context, Ui, CentralPanel, SidePanel, TopBottomPanel, ScrollArea};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, mpsc};
use serde_json::{json, Value};
use crate::device::{DeviceManager, DeviceSession};
use crate::device::session::StreamData;
use crate::transport::{TransportFactory, TransportConfig, TransportType};
use crate::ui::panels::{PerformancePanel, TelemetryPanel, LogPanel};
use crate::logging::{LogLevel, LogEntry};
use crate::telemetry::{TelemetrySystem, TelemetryConfig, TelemetryChannel, TelemetrySample, SampleType, SampleValue, ChannelConfig};
use crate::performance::{PerformanceMonitor, MonitorConfig, PerformanceAlert};
use crate::logging::LoggingSystem;
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, UNIX_EPOCH, Instant};
use serde::{Serialize, Deserialize};

/// Device connection info
#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub name: String,
    pub transport_type: TransportType,
    pub address: String,
    pub session_id: Option<String>,
    pub connected: bool,
}

/// Device configuration profile
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub name: String,
    pub digital_states: HashMap<u8, bool>,
    pub pwm_values: HashMap<u8, u8>,
    pub servo_positions: HashMap<u8, u8>,
    pub created: u64, // timestamp
}

/// Main application state for the Multi-Controller App
pub struct MultiControllerApp {
    /// Device manager for handling device connections
    device_manager: Arc<DeviceManager>,
    
    /// Available transports/devices
    available_devices: Vec<DeviceInfo>,
    
    /// Active device sessions (session_id -> device_id mapping)
    active_sessions: HashMap<String, String>,
    
    /// Currently selected device ID
    selected_device: Option<String>,
    
    /// Current active device session
    current_session: Option<Arc<Mutex<Box<dyn DeviceSession>>>>,
    
    /// Current active tab
    active_tab: Tab,
    
    /// UI state
    sidebar_width: f32,
    dark_mode: bool,
    
    /// Runtime handle for spawning tasks
    runtime: Arc<tokio::runtime::Runtime>,
    
    /// Channel for receiving device updates
    device_update_rx: mpsc::UnboundedReceiver<DeviceUpdateEvent>,
    device_update_tx: mpsc::UnboundedSender<DeviceUpdateEvent>,
    
    /// Channel for sending device commands
    command_tx: mpsc::UnboundedSender<DeviceCommand>,
    command_rx: mpsc::UnboundedReceiver<DeviceCommand>,
    
    /// Channel for receiving device responses
    response_tx: mpsc::UnboundedSender<DeviceResponse>,
    response_rx: mpsc::UnboundedReceiver<DeviceResponse>,
    
    /// Telemetry system for managing data channels
    telemetry_system: Arc<TelemetrySystem>,
    
    /// Panel states
    telemetry_panel: TelemetryPanel,
    log_panel: LogPanel,
    performance_panel: PerformancePanel,
    
    /// Digital pin states (for manual tab)
    digital_pin_states: HashMap<u8, bool>,
    /// PWM values (for manual tab)
    pwm_values: HashMap<u8, u8>,
    /// Analog input values (for manual tab)
    analog_values: HashMap<u8, u16>,
    /// Servo positions (for manual tab)
    servo_positions: HashMap<u8, u8>,
    
    /// Telemetry data buffers
    telemetry_buffers: HashMap<String, VecDeque<(f64, f64)>>, // stream_name -> (timestamp, value)
    
    /// Scripts tab state
    scripts: HashMap<String, String>, // script_name -> script_content
    current_script: String,
    selected_script: Option<String>,
    script_output: Vec<String>,
    
    /// Profiles tab state
    profiles: HashMap<String, DeviceProfile>,
    current_profile_name: String,
    
    /// Performance monitoring
    performance_monitor: Arc<PerformanceMonitor>,
    
    /// Logging system
    logging_system: Arc<LoggingSystem>,
    
    /// Startup time tracking
    startup_instant: Option<Instant>,
}

/// Events for device updates
#[derive(Debug)]
enum DeviceUpdateEvent {
    DeviceDiscovered(DeviceInfo),
    DeviceConnected(String, String), // device_id, session_id
    DeviceDisconnected(String),
    DeviceRemoved(String),
}

/// Commands to send to devices
#[derive(Debug, Clone)]
pub enum DeviceCommand {
    DigitalWrite { pin: u8, value: bool },
    AnalogWrite { pin: u8, value: u8 },
    DigitalRead { pin: u8 },
    AnalogRead { pin: u8 },
    SetServo { index: u8, position: u8 },
    ExecuteScript { script: String },
    SubscribeToStream { stream: String },
    UnsubscribeFromStream { stream: String },
    CustomCommand { endpoint: String, args: Vec<Value> },
}

/// Responses from devices
#[derive(Debug, Clone)]
pub enum DeviceResponse {
    DigitalValue { pin: u8, value: bool },
    AnalogValue { pin: u8, value: u16 },
    StreamData { stream: String, data: Value, timestamp: u64 },
    CommandResult { success: bool, data: Option<Value> },
    Error { message: String },
    Log { level: LogLevel, message: String },
}

/// Available tabs in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Manual,
    Scripts,
    Telemetry,
    Logs,
    Profiles,
    Performance,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Manual
    }
}

impl MultiControllerApp {
    /// Create a new Multi-Controller App instance
    pub fn new(device_manager: Arc<DeviceManager>) -> Self {
        let runtime = Arc::new(tokio::runtime::Runtime::new().expect("Failed to create runtime"));
        let (tx, rx) = mpsc::unbounded_channel();
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (resp_tx, resp_rx) = mpsc::unbounded_channel();
        
        // Start device discovery
        let tx_clone = tx.clone();
        let rt = runtime.clone();
        std::thread::spawn(move || {
            rt.block_on(async {
                Self::start_device_discovery(tx_clone).await;
            });
        });
        
        // Initialize default values for controls
        let mut pwm_values = HashMap::new();
        for pin in [3u8, 5, 6, 9, 10, 11] {
            pwm_values.insert(pin, 128);
        }
        
        let mut servo_positions = HashMap::new();
        for i in 0..4 {
            servo_positions.insert(i, 90);
        }
        
        // Initialize telemetry system with optimized configuration
        let telemetry_config = TelemetryConfig {
            default_buffer_size: 2000,  // Minimum as per Task 9.1 requirements
            max_memory_bytes: 50 * 1024 * 1024,  // 50MB limit
            auto_memory_management: true,
            default_sample_rate: 30.0,  // 30 FPS for charts
        };
        let telemetry_system = Arc::new(TelemetrySystem::with_config(telemetry_config));
        
        // Create default telemetry channels
        let main_channel = telemetry_system.create_channel(
            "main_telemetry".to_string(),
            Some(ChannelConfig {
                buffer_size: 2000,
                sample_rate: 30.0,
                name: "main_telemetry".to_string(),
                sample_type: SampleType::Float32,
            })
        );
        
        // Create TelemetryPanel with the main channel
        let mut telemetry_panel = TelemetryPanel::new();
        telemetry_panel.set_channel(main_channel);
        
        // Initialize logging system
        let logging_system = Arc::new(LoggingSystem::default());
        
        // Initialize performance monitoring
        let mut monitor_config = MonitorConfig::default();
        monitor_config.budget.max_memory_mb = 150.0;  // Task 17 requirement
        monitor_config.budget.max_idle_cpu_percent = 2.0;  // Task 17 requirement
        monitor_config.budget.max_startup_ms = 2000;  // Task 17 requirement
        
        let mut performance_monitor = PerformanceMonitor::new(monitor_config);
        performance_monitor.set_logging_system(logging_system.clone());
        let performance_monitor = Arc::new(performance_monitor);
        
        // Register alert callback to log panel
        let log_panel_tx = tx.clone();
        let perf_monitor_clone = performance_monitor.clone();
        runtime.spawn(async move {
            perf_monitor_clone.register_alert_callback(move |alert| {
                // Send alert to UI through event channel
                let _ = log_panel_tx.send(DeviceUpdateEvent::DeviceDiscovered(
                    DeviceInfo {
                        name: format!("Performance Alert: {}", alert.to_log_message()),
                        transport_type: TransportType::Serial,
                        address: String::new(),
                        session_id: None,
                        connected: false,
                    }
                ));
            }).await;
        });
        
        // Start performance monitoring
        let perf_monitor_clone = performance_monitor.clone();
        runtime.spawn(async move {
            perf_monitor_clone.start().await;
        });
        
        // Create performance panel with monitor
        let performance_panel = PerformancePanel::new(performance_monitor.clone());
        
        Self {
            device_manager,
            available_devices: Vec::new(),
            active_sessions: HashMap::new(),
            selected_device: None,
            current_session: None,
            active_tab: Tab::default(),
            sidebar_width: 250.0,
            dark_mode: true,
            runtime,
            device_update_rx: rx,
            device_update_tx: tx,
            command_tx: cmd_tx,
            command_rx: cmd_rx,
            response_tx: resp_tx,
            response_rx: resp_rx,
            telemetry_system,
            telemetry_panel,
            log_panel: LogPanel::new(),
            performance_panel,
            digital_pin_states: HashMap::new(),
            pwm_values,
            analog_values: HashMap::new(),
            servo_positions,
            telemetry_buffers: HashMap::new(),
            scripts: HashMap::new(),
            current_script: String::new(),
            selected_script: None,
            script_output: Vec::new(),
            profiles: HashMap::new(),
            current_profile_name: String::new(),
            performance_monitor,
            logging_system,
            startup_instant: Some(Instant::now()),
        }
    }
    
    /// Validate startup performance (Task 17 requirement)
    pub async fn validate_startup_performance(&self) -> bool {
        self.performance_monitor.validate_startup_performance().await
    }
    
    /// Start device discovery task
    async fn start_device_discovery(tx: mpsc::UnboundedSender<DeviceUpdateEvent>) {
        loop {
            // Discover available transports
            if let Ok(transports) = TransportFactory::list_available().await {
                for transport_info in transports {
                    let device_info = DeviceInfo {
                        name: match transport_info.transport_type {
                            TransportType::Serial => format!("Serial Device ({})", transport_info.name),
                            TransportType::Tcp => format!("TCP Device ({})", transport_info.address),
                            TransportType::Udp => format!("UDP Device ({})", transport_info.address),
                            TransportType::Ssh => format!("SSH Device ({})", transport_info.address),
                        },
                        transport_type: transport_info.transport_type,
                        address: transport_info.address,
                        session_id: None,
                        connected: false,
                    };
                    
                    let _ = tx.send(DeviceUpdateEvent::DeviceDiscovered(device_info));
                }
            }
            
            // Scan every 5 seconds
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
    
    /// Main update function called every frame
    pub fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Process device update events
        while let Ok(event) = self.device_update_rx.try_recv() {
            match event {
                DeviceUpdateEvent::DeviceDiscovered(info) => {
                    // Check if device already exists
                    let device_id = format!("{}_{}", info.name, info.address);
                    if !self.available_devices.iter().any(|d| 
                        format!("{}_{}", d.name, d.address) == device_id
                    ) {
                        self.available_devices.push(info);
                    }
                }
                DeviceUpdateEvent::DeviceConnected(device_id, session_id) => {
                    if let Some(device) = self.available_devices.iter_mut()
                        .find(|d| format!("{}_{}", d.name, d.address) == device_id) 
                    {
                        device.connected = true;
                        device.session_id = Some(session_id.clone());
                    }
                    self.active_sessions.insert(session_id, device_id);
                }
                DeviceUpdateEvent::DeviceDisconnected(device_id) => {
                    if let Some(device) = self.available_devices.iter_mut()
                        .find(|d| format!("{}_{}", d.name, d.address) == device_id)
                    {
                        device.connected = false;
                        device.session_id = None;
                    }
                    self.current_session = None;
                }
                DeviceUpdateEvent::DeviceRemoved(device_id) => {
                    self.available_devices.retain(|d| 
                        format!("{}_{}", d.name, d.address) != device_id
                    );
                }
            }
        }
        
        // Process device responses
        while let Ok(response) = self.response_rx.try_recv() {
            match response {
                DeviceResponse::DigitalValue { pin, value } => {
                    self.digital_pin_states.insert(pin, value);
                }
                DeviceResponse::AnalogValue { pin, value } => {
                    self.analog_values.insert(pin, value);
                }
                DeviceResponse::StreamData { stream, data, timestamp } => {
                    // Parse telemetry data
                    if let Some(value) = data.as_f64() {
                        // Add to legacy buffer for backward compatibility
                        let entry = self.telemetry_buffers
                            .entry(stream.clone())
                            .or_insert_with(|| VecDeque::with_capacity(1000));
                        
                        let time = timestamp as f64 / 1000.0;
                        entry.push_back((time, value));
                        
                        // Keep only last 1000 samples
                        while entry.len() > 1000 {
                            entry.pop_front();
                        }
                        
                        // Feed data into the telemetry system
                        // Get or create channel for this stream
                        let channel = self.telemetry_system.get_channel(&stream)
                            .unwrap_or_else(|| {
                                self.telemetry_system.create_channel(
                                    stream.clone(),
                                    Some(ChannelConfig {
                                        buffer_size: 2000,
                                        sample_rate: 30.0,
                                        name: stream.clone(),
                                        sample_type: SampleType::Float32,
                                    })
                                )
                            });
                        
                        // Add sample to the channel
                        let sample = TelemetrySample::with_timestamp(
                            SampleValue::Float32(value as f32),
                            timestamp
                        );
                        channel.add_sample(sample);
                        
                        // If this is the main telemetry stream, update panel's channel
                        if stream == "telemetry" || stream == "main_telemetry" {
                            if self.telemetry_panel.channel.is_none() {
                                self.telemetry_panel.set_channel(channel);
                            }
                        }
                        
                        // Update telemetry panel (legacy compatibility)
                        self.telemetry_panel.add_data(value as f32);
                    }
                }
                DeviceResponse::Log { level, message } => {
                    self.log_panel.add_log(LogEntry {
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                        level,
                        message,
                        source: "Device".to_string(),
                        data: None,
                        thread_id: format!("{:?}", std::thread::current().id()),
                    });
                }
                DeviceResponse::CommandResult { success, data } => {
                    if !success {
                        self.log_panel.add_log(LogEntry {
                            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                            level: LogLevel::Error,
                            message: format!("Command failed: {:?}", data),
                            source: "Device".to_string(),
                            data: None,
                            thread_id: format!("{:?}", std::thread::current().id()),
                        });
                    }
                }
                DeviceResponse::Error { message } => {
                    self.log_panel.add_log(LogEntry {
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                        level: LogLevel::Error,
                        message,
                        source: "Device".to_string(),
                        data: None,
                        thread_id: format!("{:?}", std::thread::current().id()),
                    });
                }
            }
        }
        
        // Apply Windows 10 theme
        self.apply_theme(ctx);
        
        // Top panel with title bar
        self.render_title_bar(ctx);
        
        // Left sidebar for device selection
        self.render_device_sidebar(ctx);
        
        // Main content area with tabs
        self.render_main_content(ctx);
        
        // Bottom status bar
        self.render_status_bar(ctx);
    }
    
    /// Apply Windows 10-style theme
    fn apply_theme(&self, ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        
        if self.dark_mode {
            // Windows 10 Dark Mode colors
            style.visuals.window_fill = egui::Color32::from_rgb(32, 32, 32);
            style.visuals.panel_fill = egui::Color32::from_rgb(40, 40, 40);
            style.visuals.faint_bg_color = egui::Color32::from_rgb(48, 48, 48);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(24, 24, 24);
            style.visuals.code_bg_color = egui::Color32::from_rgb(56, 56, 56);
            
            // Text colors (egui 0.29 doesn't have text_color field directly)
            
            // Selection colors (Windows 11 blue accent)
            style.visuals.selection.bg_fill = egui::Color32::from_rgb(0, 120, 212);
            style.visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 150, 255));
            
            // Widget colors
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(60, 60, 60);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(70, 70, 70);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 120, 212);
        } else {
            // Windows 10 Light Mode colors
            style.visuals.window_fill = egui::Color32::from_rgb(243, 243, 243);
            style.visuals.panel_fill = egui::Color32::from_rgb(251, 251, 251);
            style.visuals.faint_bg_color = egui::Color32::from_rgb(238, 238, 238);
            style.visuals.extreme_bg_color = egui::Color32::from_rgb(255, 255, 255);
            style.visuals.code_bg_color = egui::Color32::from_rgb(230, 230, 230);
            
            // Text colors (egui 0.29 doesn't have text_color field directly)
            
            // Selection colors
            style.visuals.selection.bg_fill = egui::Color32::from_rgb(0, 120, 212);
            style.visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 103, 192));
            
            // Widget colors
            style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(240, 240, 240);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(230, 230, 230);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 120, 212);
        }
        
        // Windows 10 corner rounding (minimal)
        style.visuals.window_rounding = egui::Rounding::same(0.0);
        style.visuals.widgets.inactive.rounding = egui::Rounding::same(2.0);
        style.visuals.widgets.hovered.rounding = egui::Rounding::same(2.0);
        style.visuals.widgets.active.rounding = egui::Rounding::same(2.0);
        
        ctx.set_style(style);
    }
    
    /// Render the title bar
    fn render_title_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("title_bar").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading("🎮 Multi-Controller App");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Theme toggle
                    if ui.button(if self.dark_mode { "☀" } else { "🌙" }).clicked() {
                        self.dark_mode = !self.dark_mode;
                    }
                    
                    // Settings button
                    if ui.button("⚙").clicked() {
                        // TODO: Open settings
                    }
                });
            });
            ui.add_space(8.0);
        });
    }
    
    /// Render the device sidebar
    fn render_device_sidebar(&mut self, ctx: &Context) {
        SidePanel::left("device_sidebar")
            .resizable(true)
            .default_width(self.sidebar_width)
            .width_range(200.0..=400.0)
            .show(ctx, |ui| {
                ui.heading("Devices");
                ui.separator();
                
                ScrollArea::vertical().show(ui, |ui| {
                    // Show real discovered devices
                    let devices = self.available_devices.clone();
                    
                    for device in devices {
                        let device_id = format!("{}_{}", device.name, device.address);
                        let is_selected = self.selected_device.as_ref() == Some(&device_id);
                        
                        ui.horizontal(|ui| {
                            // Connection indicator
                            let status_color = if device.connected {
                                egui::Color32::from_rgb(0, 200, 0)
                            } else {
                                egui::Color32::from_rgb(128, 128, 128)
                            };
                            ui.colored_label(status_color, "●");
                            
                            // Device button
                            if ui.selectable_label(is_selected, &device.name).clicked() {
                                self.selected_device = Some(device_id.clone());
                            }
                        });
                        
                        if is_selected {
                            ui.indent("device_details", |ui| {
                                ui.label(format!("Type: {:?}", device.transport_type));
                                ui.label(format!("Address: {}", device.address));
                                ui.horizontal(|ui| {
                                    if device.connected {
                                        if ui.small_button("Disconnect").clicked() {
                                            self.disconnect_device(device_id.clone());
                                        }
                                    } else {
                                        if ui.small_button("Connect").clicked() {
                                            self.connect_device(device.clone());
                                        }
                                    }
                                    if ui.small_button("Configure").clicked() {
                                        // TODO: Open device config
                                    }
                                });
                            });
                        }
                        
                        ui.add_space(4.0);
                    }
                    
                    ui.separator();
                    
                    // Add device button
                    if ui.button("➕ Add Device").clicked() {
                        // TODO: Open add device dialog
                    }
                    
                    // Refresh button
                    if ui.button("🔄 Refresh").clicked() {
                        self.refresh_devices();
                    }
                });
            });
    }
    
    /// Connect to a device
    fn connect_device(&mut self, device: DeviceInfo) {
        let device_id = format!("{}_{}", device.name, device.address);
        let device_manager = self.device_manager.clone();
        let tx = self.device_update_tx.clone();
        let runtime = self.runtime.clone();
        
        runtime.spawn(async move {
            // Create transport config
            let config = TransportConfig {
                transport_type: device.transport_type,
                address: device.address.clone(),
                connect_timeout_ms: 5000,
                settings: match device.transport_type {
                    TransportType::Serial => crate::transport::common::TransportSettings::Serial(Default::default()),
                    TransportType::Tcp => crate::transport::common::TransportSettings::Tcp(Default::default()),
                    TransportType::Udp => crate::transport::common::TransportSettings::Udp(Default::default()),
                    TransportType::Ssh => crate::transport::common::TransportSettings::Ssh(Default::default()),
                },
                ..Default::default()
            };
            
            // Create transport
            if let Ok(mut transport) = TransportFactory::create(config).await {
                // Connect the transport first
                if transport.connect().await.is_ok() {
                    // Try to open device
                    if let Ok(session_id) = device_manager.open_device(
                        Arc::from(transport), 
                        Some(device_id.clone())
                    ).await {
                        let _ = tx.send(DeviceUpdateEvent::DeviceConnected(device_id, session_id));
                    } else {
                        tracing::error!("Failed to open device: {}", device_id);
                    }
                } else {
                    tracing::error!("Failed to connect transport for: {}", device_id);
                }
            } else {
                tracing::error!("Failed to create transport for: {}", device_id);
            }
        });
    }
    
    /// Disconnect from a device
    fn disconnect_device(&mut self, device_id: String) {
        let device_manager = self.device_manager.clone();
        let tx = self.device_update_tx.clone();
        let runtime = self.runtime.clone();
        
        runtime.spawn(async move {
            // Find session ID for this device
            if let Some(device) = device_manager.list_sessions().await.iter()
                .find(|s| s.contains(&device_id))
            {
                if device_manager.close_device(device).await.is_ok() {
                    let _ = tx.send(DeviceUpdateEvent::DeviceDisconnected(device_id));
                }
            }
        });
    }
    
    /// Refresh device list
    fn refresh_devices(&mut self) {
        self.available_devices.clear();
        // Discovery task will repopulate the list
    }
    
    /// Send a command to the current device
    fn send_device_command(&mut self, command: DeviceCommand) {
        if self.current_session.is_none() {
            self.log_panel.add_log(LogEntry {
                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                level: LogLevel::Warning,
                message: "No device connected".to_string(),
                source: "System".to_string(),
                data: None,
                thread_id: format!("{:?}", std::thread::current().id()),
            });
            return;
        }
        
        let session = self.current_session.clone();
        let response_tx = self.response_tx.clone();
        let runtime = self.runtime.clone();
        
        runtime.spawn(async move {
            if let Some(session_arc) = session {
                let mut session = session_arc.lock().await;
                
                let result = match command {
                    DeviceCommand::DigitalWrite { pin, value } => {
                        session.invoke_async(
                            "digitalWrite",
                            vec![json!(pin), json!(value)]
                        ).await
                    },
                    DeviceCommand::AnalogWrite { pin, value } => {
                        session.invoke_async(
                            "analogWrite",
                            vec![json!(pin), json!(value)]
                        ).await
                    },
                    DeviceCommand::DigitalRead { pin } => {
                        session.invoke_async(
                            "digitalRead",
                            vec![json!(pin)]
                        ).await
                    },
                    DeviceCommand::AnalogRead { pin } => {
                        session.invoke_async(
                            "analogRead",
                            vec![json!(pin)]
                        ).await
                    },
                    DeviceCommand::SetServo { index, position } => {
                        session.invoke_async(
                            "setServo",
                            vec![json!(index), json!(position)]
                        ).await
                    },
                    DeviceCommand::CustomCommand { endpoint, args } => {
                        session.invoke_async(&endpoint, args).await
                    },
                    _ => {
                        Ok(json!(null))
                    }
                };
                
                match result {
                    Ok(data) => {
                        let _ = response_tx.send(DeviceResponse::CommandResult {
                            success: true,
                            data: Some(data),
                        });
                    }
                    Err(e) => {
                        let _ = response_tx.send(DeviceResponse::Error {
                            message: e.to_string(),
                        });
                    }
                }
            }
        });
    }
    
    /// Execute the current script
    fn execute_script(&mut self) {
        let lines: Vec<String> = self.current_script.lines().map(|s| s.to_string()).collect();
        
        for line in lines {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            
            // Parse and execute commands
            if line.starts_with("digitalWrite(") && line.ends_with(")") {
                // Parse digitalWrite command
                let args = &line[13..line.len()-1];
                let parts: Vec<&str> = args.split(',').collect();
                if parts.len() == 2 {
                    if let Ok(pin) = parts[0].trim().parse::<u8>() {
                        let value = parts[1].trim().to_lowercase() == "true" || parts[1].trim() == "1";
                        self.send_device_command(DeviceCommand::DigitalWrite { pin, value });
                        self.script_output.push(format!("digitalWrite({}, {})", pin, value));
                    } else {
                        self.script_output.push(format!("Error: Invalid pin number in '{}'", line));
                    }
                } else {
                    self.script_output.push(format!("Error: Invalid arguments in '{}'", line));
                }
            } else if line.starts_with("analogWrite(") && line.ends_with(")") {
                // Parse analogWrite command
                let args = &line[12..line.len()-1];
                let parts: Vec<&str> = args.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(pin), Ok(value)) = (parts[0].trim().parse::<u8>(), parts[1].trim().parse::<u8>()) {
                        self.send_device_command(DeviceCommand::AnalogWrite { pin, value });
                        self.script_output.push(format!("analogWrite({}, {})", pin, value));
                    } else {
                        self.script_output.push(format!("Error: Invalid arguments in '{}'", line));
                    }
                } else {
                    self.script_output.push(format!("Error: Invalid arguments in '{}'", line));
                }
            } else if line.starts_with("setServo(") && line.ends_with(")") {
                // Parse setServo command
                let args = &line[9..line.len()-1];
                let parts: Vec<&str> = args.split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(index), Ok(position)) = (parts[0].trim().parse::<u8>(), parts[1].trim().parse::<u8>()) {
                        self.send_device_command(DeviceCommand::SetServo { index, position });
                        self.script_output.push(format!("setServo({}, {})", index, position));
                    } else {
                        self.script_output.push(format!("Error: Invalid arguments in '{}'", line));
                    }
                } else {
                    self.script_output.push(format!("Error: Invalid arguments in '{}'", line));
                }
            } else if line.starts_with("delay(") && line.ends_with(")") {
                // Parse delay command (note: this is synchronous and will block UI)
                let args = &line[6..line.len()-1];
                if let Ok(ms) = args.trim().parse::<u64>() {
                    self.script_output.push(format!("delay({}ms)", ms));
                    // Note: In a real implementation, we'd handle this asynchronously
                }
            } else {
                self.script_output.push(format!("Error: Unknown command '{}'", line));
            }
        }
        
        self.script_output.push("=== Script Execution Complete ===".to_string());
    }
    
    /// Render the main content area with tabs
    fn render_main_content(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            // Tab bar
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                
                let tabs = [
                    (Tab::Manual, "🎛 Manual"),
                    (Tab::Scripts, "📜 Scripts"),
                    (Tab::Telemetry, "📊 Telemetry"),
                    (Tab::Logs, "📝 Logs"),
                    (Tab::Profiles, "👤 Profiles"),
                    (Tab::Performance, "⚡ Performance"),
                ];
                
                for (tab, label) in tabs.iter() {
                    if ui.selectable_label(self.active_tab == *tab, *label).clicked() {
                        self.active_tab = *tab;
                    }
                    ui.add_space(4.0);
                }
            });
            
            ui.separator();
            
            // Tab content
            ScrollArea::both().show(ui, |ui| {
                match self.active_tab {
                    Tab::Manual => self.render_manual_tab(ui),
                    Tab::Scripts => self.render_scripts_tab(ui),
                    Tab::Telemetry => self.render_telemetry_tab(ui),
                    Tab::Logs => self.render_logs_tab(ui),
                    Tab::Profiles => self.render_profiles_tab(ui),
                    Tab::Performance => self.render_performance_tab(ui, ctx),
                }
            });
        });
    }
    
    /// Render the status bar
    fn render_status_bar(&self, ctx: &Context) {
        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Connection status
                if let Some(device) = &self.selected_device {
                    ui.label(format!("Connected: {}", device));
                } else {
                    ui.label("No device selected");
                }
                
                ui.separator();
                
                // Performance metrics
                ui.label(format!("FPS: {:.0}", 60.0)); // TODO: Real FPS
                ui.label(format!("CPU: {:.1}%", 1.5)); // TODO: Real CPU
                ui.label(format!("RAM: {} MB", 145)); // TODO: Real RAM
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
                });
            });
        });
    }
    
    /// Render Manual control tab
    fn render_manual_tab(&mut self, ui: &mut Ui) {
        ui.heading("Manual Control");
        ui.separator();
        
        if self.selected_device.is_none() {
            ui.label("Please select a device from the sidebar to access manual controls.");
            return;
        }
        
        // Control sections
        ui.collapsing("Digital I/O", |ui| {
            ui.horizontal_wrapped(|ui| {
                for pin in 0..14 {
                    let current_state = *self.digital_pin_states.get(&pin).unwrap_or(&false);
                    ui.vertical(|ui| {
                        ui.label(format!("D{}", pin));
                        
                        // Show current state
                        let color = if current_state {
                            egui::Color32::from_rgb(0, 200, 0)
                        } else {
                            egui::Color32::from_rgb(100, 100, 100)
                        };
                        ui.colored_label(color, if current_state { "HIGH" } else { "LOW" });
                        
                        if ui.button("HIGH").clicked() {
                            self.send_device_command(DeviceCommand::DigitalWrite { 
                                pin, 
                                value: true 
                            });
                            self.digital_pin_states.insert(pin, true);
                        }
                        if ui.button("LOW").clicked() {
                            self.send_device_command(DeviceCommand::DigitalWrite { 
                                pin, 
                                value: false 
                            });
                            self.digital_pin_states.insert(pin, false);
                        }
                    });
                }
            });
        });
        
        ui.collapsing("PWM Control", |ui| {
            for pin in [3u8, 5, 6, 9, 10, 11] {
                ui.horizontal(|ui| {
                    ui.label(format!("PWM{}: ", pin));
                    let mut value = *self.pwm_values.get(&pin).unwrap_or(&128);
                    let original_value = value;
                    if ui.add(egui::Slider::new(&mut value, 0..=255)).changed() {
                        self.pwm_values.insert(pin, value);
                        self.send_device_command(DeviceCommand::AnalogWrite { pin, value });
                    }
                    ui.label(format!("{}", value));
                });
            }
        });
        
        ui.collapsing("Analog Inputs", |ui| {
            for pin in 0..6 {
                ui.horizontal(|ui| {
                    ui.label(format!("A{}: ", pin));
                    let value = *self.analog_values.get(&pin).unwrap_or(&0);
                    let normalized = value as f32 / 1023.0;
                    ui.add(egui::ProgressBar::new(normalized).text(format!("{}", value)));
                    if ui.button("Read").clicked() {
                        self.send_device_command(DeviceCommand::AnalogRead { pin });
                    }
                });
            }
        });
        
        ui.collapsing("Servo Control", |ui| {
            for i in 0..4 {
                ui.horizontal(|ui| {
                    ui.label(format!("Servo {}: ", i));
                    let mut position = *self.servo_positions.get(&i).unwrap_or(&90);
                    if ui.add(egui::Slider::new(&mut position, 0..=180).suffix("°")).changed() {
                        self.servo_positions.insert(i, position);
                        self.send_device_command(DeviceCommand::SetServo { 
                            index: i, 
                            position 
                        });
                    }
                });
            }
        });
    }
    
    /// Render Scripts tab
    fn render_scripts_tab(&mut self, ui: &mut Ui) {
        ui.heading("Scripts");
        ui.separator();
        
        if self.selected_device.is_none() {
            ui.label("Please select a device to run scripts.");
            return;
        }
        
        // Script management controls
        ui.horizontal(|ui| {
            ui.label("Script:");
            
            // Script selector
            egui::ComboBox::from_label("")
                .selected_text(self.selected_script.as_ref().unwrap_or(&"New Script".to_string()))
                .show_ui(ui, |ui| {
                    if ui.selectable_value(&mut self.selected_script, None, "New Script").clicked() {
                        self.current_script.clear();
                        self.script_output.clear();
                    }
                    
                    for script_name in self.scripts.keys() {
                        let name = script_name.clone();
                        if ui.selectable_label(
                            self.selected_script.as_ref() == Some(&name),
                            &name
                        ).clicked() {
                            self.selected_script = Some(name.clone());
                            self.current_script = self.scripts.get(&name).unwrap_or(&String::new()).clone();
                            self.script_output.clear();
                        }
                    }
                });
            
            ui.separator();
            
            // Save button
            if ui.button("💾 Save").clicked() {
                if let Some(name) = &self.selected_script {
                    self.scripts.insert(name.clone(), self.current_script.clone());
                    self.script_output.push(format!("Script '{}' saved.", name));
                } else {
                    // Prompt for name
                    let name = format!("script_{}", self.scripts.len() + 1);
                    self.scripts.insert(name.clone(), self.current_script.clone());
                    self.selected_script = Some(name.clone());
                    self.script_output.push(format!("Script saved as '{}'.", name));
                }
            }
            
            // Execute button
            if ui.button("▶ Execute").clicked() {
                self.script_output.push("=== Executing Script ===".to_string());
                self.execute_script();
            }
            
            // Clear output button
            if ui.button("🗑 Clear Output").clicked() {
                self.script_output.clear();
            }
        });
        
        ui.separator();
        
        // Two-column layout: editor and output
        ui.columns(2, |columns| {
            // Left column: Script editor
            columns[0].heading("Script Editor");
            columns[0].separator();
            
            ScrollArea::vertical()
                .id_source("script_editor")
                .auto_shrink([false; 2])
                .show(&mut columns[0], |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.current_script)
                            .desired_width(f32::INFINITY)
                            .desired_rows(20)
                            .font(egui::TextStyle::Monospace)
                            .code_editor()
                    );
                });
            
            columns[0].separator();
            columns[0].label("Commands: digitalWrite(pin, value), analogWrite(pin, value), setServo(index, pos)");
            columns[0].label("Example: digitalWrite(13, true) // Turn on LED");
            
            // Right column: Output
            columns[1].heading("Output");
            columns[1].separator();
            
            ScrollArea::vertical()
                .id_source("script_output")
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(&mut columns[1], |ui| {
                    for line in &self.script_output {
                        ui.monospace(line);
                    }
                });
        });
    }
    
    /// Render Telemetry tab
    fn render_telemetry_tab(&mut self, ui: &mut Ui) {
        ui.heading("Telemetry");
        ui.separator();
        
        if self.selected_device.is_none() {
            ui.label("Please select a device to view telemetry.");
            return;
        }
        
        // Stream selection and channel management
        ui.horizontal(|ui| {
            ui.label("Streams:");
            if ui.button("Subscribe to Telemetry").clicked() {
                self.send_device_command(DeviceCommand::SubscribeToStream { 
                    stream: "telemetry".to_string() 
                });
            }
            if ui.button("Subscribe to Sensors").clicked() {
                self.send_device_command(DeviceCommand::SubscribeToStream { 
                    stream: "sensors".to_string() 
                });
            }
            
            ui.separator();
            
            // Channel selector
            let channels = self.telemetry_system.channel_names();
            if !channels.is_empty() {
                ui.label("Channel:");
                egui::ComboBox::from_label("")
                    .selected_text(self.telemetry_panel.channel.as_ref()
                        .map(|c| "main_telemetry")
                        .unwrap_or("Select Channel"))
                    .show_ui(ui, |ui| {
                        for channel_name in channels {
                            if ui.selectable_label(false, &channel_name).clicked() {
                                if let Some(channel) = self.telemetry_system.get_channel(&channel_name) {
                                    self.telemetry_panel.set_channel(channel);
                                }
                            }
                        }
                    });
            }
            
            // System stats
            ui.separator();
            let stats = self.telemetry_system.get_stats();
            ui.label(format!("Channels: {} | Memory: {:.1} MB", 
                stats.channel_count,
                stats.total_memory_bytes as f64 / (1024.0 * 1024.0)));
        });
        
        ui.separator();
        
        // Display telemetry panel with real-time charts
        self.telemetry_panel.show(ui);
        
        // Show all telemetry streams
        for (stream_name, buffer) in &self.telemetry_buffers {
            ui.collapsing(format!("Stream: {}", stream_name), |ui| {
                if buffer.is_empty() {
                    ui.label("No data received");
                } else {
                    // Show last value
                    if let Some((timestamp, value)) = buffer.back() {
                        ui.label(format!("Latest: {:.2} at {:.2}s", value, timestamp));
                    }
                    
                    // Simple line chart
                    let height = 100.0;
                    let (response, painter) = ui.allocate_painter(
                        egui::Vec2::new(ui.available_width(), height),
                        egui::Sense::hover()
                    );
                    
                    let rect = response.rect;
                    painter.rect_filled(
                        rect,
                        egui::Rounding::same(2.0),
                        egui::Color32::from_rgb(40, 40, 40)
                    );
                    
                    if buffer.len() > 1 {
                        let min_val = buffer.iter().map(|(_, v)| *v).fold(f64::INFINITY, f64::min);
                        let max_val = buffer.iter().map(|(_, v)| *v).fold(f64::NEG_INFINITY, f64::max);
                        let range = max_val - min_val;
                        
                        let points: Vec<egui::Pos2> = buffer
                            .iter()
                            .enumerate()
                            .map(|(i, (_, value))| {
                                let x = rect.left() + (i as f32 / buffer.len() as f32) * rect.width();
                                let normalized = if range > 0.0 {
                                    ((value - min_val) / range) as f32
                                } else {
                                    0.5
                                };
                                let y = rect.bottom() - (normalized * rect.height());
                                egui::pos2(x, y)
                            })
                            .collect();
                        
                        for window in points.windows(2) {
                            painter.line_segment(
                                [window[0], window[1]],
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 200, 255))
                            );
                        }
                    }
                }
            });
        }
    }
    
    /// Render Logs tab
    fn render_logs_tab(&mut self, ui: &mut Ui) {
        ui.heading("Logs");
        ui.separator();
        
        // Log controls
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.log_panel.auto_scroll, "Auto-scroll");
            if ui.button("Clear").clicked() {
                self.log_panel.clear();
            }
        });
        
        ui.separator();
        
        // Display logs from panel
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .stick_to_bottom(self.log_panel.auto_scroll)
            .show(ui, |ui| {
                self.log_panel.show(ui);
            });
    }
    
    /// Render Profiles tab
    fn render_profiles_tab(&mut self, ui: &mut Ui) {
        ui.heading("Profiles");
        ui.separator();
        
        if self.selected_device.is_none() {
            ui.label("Please select a device to manage profiles.");
            return;
        }
        
        // Profile management controls
        ui.horizontal(|ui| {
            ui.label("Profile Name:");
            ui.text_edit_singleline(&mut self.current_profile_name);
            
            // Save current state as profile
            if ui.button("💾 Save Current State").clicked() {
                if !self.current_profile_name.is_empty() {
                    let profile = DeviceProfile {
                        name: self.current_profile_name.clone(),
                        digital_states: self.digital_pin_states.clone(),
                        pwm_values: self.pwm_values.clone(),
                        servo_positions: self.servo_positions.clone(),
                        created: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    };
                    self.profiles.insert(self.current_profile_name.clone(), profile);
                    self.log_panel.add_log(LogEntry {
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                        level: LogLevel::Info,
                        message: format!("Profile '{}' saved", self.current_profile_name),
                        source: "Profile".to_string(),
                        data: None,
                        thread_id: format!("{:?}", std::thread::current().id()),
                    });
                }
            }
            
            // Load profile button
            if ui.button("📂 Load Profile").clicked() {
                // Will load the selected profile below
            }
            
            // Delete profile button
            if ui.button("🗑 Delete Profile").clicked() {
                if !self.current_profile_name.is_empty() {
                    self.profiles.remove(&self.current_profile_name);
                    self.log_panel.add_log(LogEntry {
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                        level: LogLevel::Info,
                        message: format!("Profile '{}' deleted", self.current_profile_name),
                        source: "Profile".to_string(),
                        data: None,
                        thread_id: format!("{:?}", std::thread::current().id()),
                    });
                    self.current_profile_name.clear();
                }
            }
        });
        
        ui.separator();
        
        // Two-column layout
        ui.columns(2, |columns| {
            // Left column: Profile list
            columns[0].heading("Saved Profiles");
            columns[0].separator();
            
            ScrollArea::vertical()
                .id_source("profile_list")
                .auto_shrink([false; 2])
                .show(&mut columns[0], |ui| {
                    let profiles = self.profiles.clone();
                    for (name, profile) in profiles {
                        let is_selected = self.current_profile_name == name;
                        
                        if ui.selectable_label(is_selected, &name).clicked() {
                            self.current_profile_name = name.clone();
                            
                            // Load the profile
                            self.digital_pin_states = profile.digital_states.clone();
                            self.pwm_values = profile.pwm_values.clone();
                            self.servo_positions = profile.servo_positions.clone();
                            
                            // Apply to device
                            for (&pin, &value) in &profile.digital_states {
                                self.send_device_command(DeviceCommand::DigitalWrite { pin, value });
                            }
                            for (&pin, &value) in &profile.pwm_values {
                                self.send_device_command(DeviceCommand::AnalogWrite { pin, value });
                            }
                            for (&index, &position) in &profile.servo_positions {
                                self.send_device_command(DeviceCommand::SetServo { index, position });
                            }
                            
                            self.log_panel.add_log(LogEntry {
                                timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64,
                                level: LogLevel::Info,
                                message: format!("Profile '{}' loaded and applied", name),
                                source: "Profile".to_string(),
                                data: None,
                                thread_id: format!("{:?}", std::thread::current().id()),
                            });
                        }
                        
                        // Show profile details on hover
                    }
                });
            
            // Right column: Profile details
            columns[1].heading("Current Configuration");
            columns[1].separator();
            
            ScrollArea::vertical()
                .id_source("profile_details")
                .auto_shrink([false; 2])
                .show(&mut columns[1], |ui| {
                    ui.collapsing("Digital Outputs", |ui| {
                        for (&pin, &state) in &self.digital_pin_states {
                            ui.horizontal(|ui| {
                                ui.label(format!("Pin {}: ", pin));
                                ui.label(if state { "HIGH" } else { "LOW" });
                            });
                        }
                    });
                    
                    ui.collapsing("PWM Outputs", |ui| {
                        for (&pin, &value) in &self.pwm_values {
                            ui.horizontal(|ui| {
                                ui.label(format!("Pin {}: ", pin));
                                ui.label(format!("{} ({}%)", value, (value as f32 / 255.0 * 100.0) as u8));
                            });
                        }
                    });
                    
                    ui.collapsing("Servo Positions", |ui| {
                        for (&index, &position) in &self.servo_positions {
                            ui.horizontal(|ui| {
                                ui.label(format!("Servo {}: ", index));
                                ui.label(format!("{}°", position));
                            });
                        }
                    });
                });
        });
    }
    
    /// Render Performance monitoring tab
    fn render_performance_tab(&mut self, ui: &mut Ui, ctx: &Context) {
        // Show the performance panel
        self.performance_panel.render(ctx, ui);
    }
}

impl eframe::App for MultiControllerApp {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.update(ctx, frame);
    }
}