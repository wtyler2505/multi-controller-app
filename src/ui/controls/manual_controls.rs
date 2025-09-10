use super::widgets::{
    ControlWidget, ControlValue, WidgetMetadata, 
    PrecisionSlider, MultiStateToggle, EmergencyStopWidget, 
    SelectionDropdown, TextInput
};
use egui::{Color32, Ui};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::mpsc;
use uuid::Uuid;

/// State management for manual controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualControlState {
    pub emergency_stop_engaged: bool,
    pub widget_values: HashMap<String, ControlValue>,
    pub last_update: SystemTime,
    pub device_session_id: Option<Uuid>,
    pub control_authority: ControlAuthority,
    pub safety_interlocks: HashMap<String, bool>,
}

/// Control authority levels
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControlAuthority {
    None,
    ReadOnly,
    Limited,
    Full,
    Emergency,
}

impl ManualControlState {
    pub fn new() -> Self {
        Self {
            emergency_stop_engaged: false,
            widget_values: HashMap::new(),
            last_update: SystemTime::now(),
            device_session_id: None,
            control_authority: ControlAuthority::None,
            safety_interlocks: HashMap::new(),
        }
    }

    pub fn update_widget_value(&mut self, widget_id: &str, value: ControlValue) {
        self.widget_values.insert(widget_id.to_string(), value);
        self.last_update = SystemTime::now();
    }

    pub fn get_widget_value(&self, widget_id: &str) -> Option<&ControlValue> {
        self.widget_values.get(widget_id)
    }

    pub fn engage_emergency_stop(&mut self) {
        self.emergency_stop_engaged = true;
        self.control_authority = ControlAuthority::Emergency;
        self.last_update = SystemTime::now();
        tracing::warn!("Emergency stop engaged via manual control");
    }

    pub fn disengage_emergency_stop(&mut self) {
        self.emergency_stop_engaged = false;
        self.control_authority = ControlAuthority::Full;
        self.last_update = SystemTime::now();
        tracing::info!("Emergency stop disengaged");
    }

    pub fn set_safety_interlock(&mut self, name: &str, engaged: bool) {
        self.safety_interlocks.insert(name.to_string(), engaged);
        self.last_update = SystemTime::now();
    }

    pub fn is_safety_interlock_engaged(&self, name: &str) -> bool {
        self.safety_interlocks.get(name).copied().unwrap_or(false)
    }

    pub fn can_control(&self) -> bool {
        !self.emergency_stop_engaged 
            && matches!(self.control_authority, ControlAuthority::Limited | ControlAuthority::Full)
            && !self.safety_interlocks.values().any(|&engaged| engaged)
    }
}

impl Default for ManualControlState {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result for control values
#[derive(Debug, Clone)]
pub enum ValidationResult {
    Valid,
    Clamped { original: ControlValue, clamped: ControlValue },
    Invalid { reason: String },
}

/// Value validator trait
pub trait ValueValidator: Send + Sync {
    fn validate_range(&self, value: f64, min: f64, max: f64) -> ValidationResult;
    fn validate_precision(&self, value: f64, precision: usize) -> ValidationResult;
    fn validate_enum(&self, value: &str, allowed: &[String]) -> ValidationResult;
    fn sanitize_input(&self, input: &str) -> String;
}

/// Standard value validator implementation
pub struct StandardValidator;

impl ValueValidator for StandardValidator {
    fn validate_range(&self, value: f64, min: f64, max: f64) -> ValidationResult {
        if value < min {
            ValidationResult::Clamped { 
                original: ControlValue::Float(value), 
                clamped: ControlValue::Float(min) 
            }
        } else if value > max {
            ValidationResult::Clamped { 
                original: ControlValue::Float(value), 
                clamped: ControlValue::Float(max) 
            }
        } else if value.is_nan() || value.is_infinite() {
            ValidationResult::Invalid { 
                reason: "Value must be a finite number".to_string() 
            }
        } else {
            ValidationResult::Valid
        }
    }

    fn validate_precision(&self, value: f64, precision: usize) -> ValidationResult {
        let scale = 10.0_f64.powi(precision as i32);
        let rounded = (value * scale).round() / scale;

        if (value - rounded).abs() > f64::EPSILON {
            ValidationResult::Clamped { 
                original: ControlValue::Float(value), 
                clamped: ControlValue::Float(rounded) 
            }
        } else {
            ValidationResult::Valid
        }
    }

    fn validate_enum(&self, value: &str, allowed: &[String]) -> ValidationResult {
        if allowed.contains(&value.to_string()) {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid {
                reason: format!("Value '{}' not in allowed set: {:?}", value, allowed)
            }
        }
    }

    fn sanitize_input(&self, input: &str) -> String {
        // Remove potentially dangerous characters
        input.chars()
            .filter(|c| c.is_ascii_alphanumeric() || ".,-_+ ()".contains(*c))
            .collect()
    }
}

/// Events emitted by the manual control system
#[derive(Debug, Clone)]
pub enum ControlEvent {
    EmergencyStop { engaged: bool },
    ValueChanged { widget_id: String, value: ControlValue },
    ValidationError { widget_id: String, error: String },
    SafetyInterlock { name: String, engaged: bool },
    AuthorityChanged { new_authority: ControlAuthority },
}

/// Manual control manager
pub struct ManualControlManager {
    widgets: Vec<Box<dyn ControlWidget>>,
    state: Arc<RwLock<ManualControlState>>,
    emergency_stop: EmergencyStopWidget,
    validator: Box<dyn ValueValidator>,
    event_sender: Option<mpsc::UnboundedSender<ControlEvent>>,
    update_interval: Duration,
    last_render: Instant,
    widget_groups: HashMap<String, Vec<String>>, // Group name -> widget IDs
    enabled: bool,
}

impl ManualControlManager {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            state: Arc::new(RwLock::new(ManualControlState::new())),
            emergency_stop: EmergencyStopWidget::new("emergency_stop", "Emergency Stop"),
            validator: Box::new(StandardValidator),
            event_sender: None,
            update_interval: Duration::from_millis(16), // ~60 FPS
            last_render: Instant::now(),
            widget_groups: HashMap::new(),
            enabled: true,
        }
    }

    pub fn with_validator(mut self, validator: Box<dyn ValueValidator>) -> Self {
        self.validator = validator;
        self
    }

    pub fn with_event_channel(mut self) -> (Self, mpsc::UnboundedReceiver<ControlEvent>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        self.event_sender = Some(sender);
        (self, receiver)
    }

    pub fn with_update_rate(mut self, fps: u32) -> Self {
        self.update_interval = Duration::from_millis(1000 / fps as u64);
        self
    }

    pub fn add_widget(&mut self, widget: Box<dyn ControlWidget>) {
        // Initialize widget value in state
        let widget_id = widget.id().to_string();
        let initial_value = widget.get_value();
        
        if let Ok(mut state) = self.state.write() {
            state.update_widget_value(&widget_id, initial_value);
        }

        self.widgets.push(widget);
    }

    pub fn add_widget_to_group(&mut self, group_name: &str, widget_id: &str) {
        self.widget_groups
            .entry(group_name.to_string())
            .or_insert_with(Vec::new)
            .push(widget_id.to_string());
    }

    pub fn create_widget_group(&mut self, group_name: &str, widget_ids: Vec<String>) {
        self.widget_groups.insert(group_name.to_string(), widget_ids);
    }

    pub fn get_widget_by_id(&self, widget_id: &str) -> Option<&Box<dyn ControlWidget>> {
        self.widgets.iter().find(|w| w.id() == widget_id)
    }

    pub fn get_widget_by_id_mut(&mut self, widget_id: &str) -> Option<&mut Box<dyn ControlWidget>> {
        self.widgets.iter_mut().find(|w| w.id() == widget_id)
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_control_authority(&self, authority: ControlAuthority) {
        if let Ok(mut state) = self.state.write() {
            let old_authority = state.control_authority.clone();
            state.control_authority = authority.clone();
            state.last_update = SystemTime::now();

            if let Some(sender) = &self.event_sender {
                let _ = sender.send(ControlEvent::AuthorityChanged { 
                    new_authority: authority 
                });
            }

            tracing::info!("Control authority changed from {:?} to {:?}", old_authority, state.control_authority);
        }
    }

    pub fn set_safety_interlock(&self, name: &str, engaged: bool) {
        if let Ok(mut state) = self.state.write() {
            state.set_safety_interlock(name, engaged);

            if let Some(sender) = &self.event_sender {
                let _ = sender.send(ControlEvent::SafetyInterlock { 
                    name: name.to_string(), 
                    engaged 
                });
            }

            tracing::info!("Safety interlock '{}' {}", name, if engaged { "engaged" } else { "disengaged" });
        }
    }

    pub fn render(&mut self, ui: &mut Ui) -> bool {
        if !self.enabled {
            ui.label("Manual controls disabled");
            return false;
        }

        let mut any_changed = false;
        let now = Instant::now();

        // Rate limiting for performance
        if now.duration_since(self.last_render) < self.update_interval {
            return false;
        }
        self.last_render = now;

        let can_control = self.state.read().map(|s| s.can_control()).unwrap_or(false);

        // Emergency stop always rendered first and prominently
        ui.separator();
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            
            if self.emergency_stop.render(ui) {
                let engaged = self.emergency_stop.is_engaged();
                
                if let Ok(mut state) = self.state.write() {
                    if engaged {
                        state.engage_emergency_stop();
                        // Signal all widgets to emergency stop
                        for widget in &mut self.widgets {
                            widget.emergency_stop();
                        }
                    } else {
                        state.disengage_emergency_stop();
                    }
                }

                if let Some(sender) = &self.event_sender {
                    let _ = sender.send(ControlEvent::EmergencyStop { engaged });
                }
                
                any_changed = true;
            }
            
            ui.add_space(10.0);
        });
        ui.separator();

        // Show control status
        ui.horizontal(|ui| {
            let state = self.state.read().unwrap();
            let status_text = if state.emergency_stop_engaged {
                "⚠ EMERGENCY STOP ACTIVE"
            } else if !can_control {
                "� CONTROLS RESTRICTED"
            } else {
                " CONTROLS ENABLED"
            };
            
            let color = if state.emergency_stop_engaged {
                Color32::RED
            } else if !can_control {
                Color32::YELLOW
            } else {
                Color32::GREEN
            };
            
            ui.colored_label(color, status_text);
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("Authority: {:?}", state.control_authority));
            });
        });

        ui.separator();

        // Render control widgets
        ui.columns(2, |columns| {
            let widgets_per_column = (self.widgets.len() + 1) / 2;
            
            for (i, widget) in self.widgets.iter_mut().enumerate() {
                let column_index = i / widgets_per_column;
                let ui = &mut columns[column_index.min(1)];
                
                ui.add_enabled_ui(can_control, |ui| {
                    ui.group(|ui| {
                    
                    if widget.render(ui) {
                        // Widget value changed
                        let widget_id = widget.id().to_string();
                        let value = widget.get_value();
                        
                        // Validate the new value
                        if widget.validate(&value) {
                            // Update state
                            if let Ok(mut state) = self.state.write() {
                                state.update_widget_value(&widget_id, value.clone());
                            }
                            
                            // Send event
                            if let Some(sender) = &self.event_sender {
                                let _ = sender.send(ControlEvent::ValueChanged { 
                                    widget_id: widget_id.clone(), 
                                    value 
                                });
                            }
                        } else {
                            // Validation failed, revert to last known good value
                            if let Ok(state) = self.state.read() {
                                if let Some(last_value) = state.get_widget_value(&widget_id) {
                                    let _ = widget.set_value(last_value.clone());
                                }
                            }
                            
                            if let Some(sender) = &self.event_sender {
                                let _ = sender.send(ControlEvent::ValidationError { 
                                    widget_id, 
                                    error: "Value validation failed".to_string() 
                                });
                            }
                        }
                        
                        any_changed = true;
                    }
                    });
                });
                
                ui.add_space(5.0);
            }
        });

        any_changed
    }

    pub fn render_widget_group(&mut self, ui: &mut Ui, group_name: &str) -> bool {
        if !self.enabled {
            return false;
        }

        let widget_ids = self.widget_groups.get(group_name).cloned().unwrap_or_default();
        let mut any_changed = false;
        let can_control = self.state.read().map(|s| s.can_control()).unwrap_or(false);

        ui.group(|ui| {
            ui.label(group_name);
            ui.separator();
            
            ui.add_enabled_ui(can_control, |ui| {
            
            for widget_id in &widget_ids {
                if let Some(widget) = self.get_widget_by_id_mut(widget_id) {
                    if widget.render(ui) {
                        let value = widget.get_value();
                        
                        if widget.validate(&value) {
                            if let Ok(mut state) = self.state.write() {
                                state.update_widget_value(widget_id, value.clone());
                            }
                            
                            if let Some(sender) = &self.event_sender {
                                let _ = sender.send(ControlEvent::ValueChanged { 
                                    widget_id: widget_id.clone(), 
                                    value 
                                });
                            }
                        }
                        
                        any_changed = true;
                    }
                    ui.add_space(3.0);
                }
            }
            });
        });

        any_changed
    }

    pub fn emergency_stop_all(&mut self) {
        // Engage emergency stop
        self.emergency_stop.engage();
        
        // Update state
        if let Ok(mut state) = self.state.write() {
            state.engage_emergency_stop();
        }
        
        // Emergency stop all widgets
        for widget in &mut self.widgets {
            widget.emergency_stop();
        }
        
        // Send event
        if let Some(sender) = &self.event_sender {
            let _ = sender.send(ControlEvent::EmergencyStop { engaged: true });
        }
        
        tracing::warn!("Emergency stop activated for all manual controls");
    }

    pub fn reset_emergency_stop(&mut self) {
        self.emergency_stop.reset();
        
        if let Ok(mut state) = self.state.write() {
            state.disengage_emergency_stop();
        }
        
        if let Some(sender) = &self.event_sender {
            let _ = sender.send(ControlEvent::EmergencyStop { engaged: false });
        }
        
        tracing::info!("Emergency stop reset for manual controls");
    }

    pub fn get_state_snapshot(&self) -> ManualControlState {
        self.state.read().unwrap().clone()
    }

    pub fn is_emergency_stop_engaged(&self) -> bool {
        self.state.read().map(|s| s.emergency_stop_engaged).unwrap_or(false)
    }

    pub fn set_device_session(&self, session_id: Option<Uuid>) {
        if let Ok(mut state) = self.state.write() {
            state.device_session_id = session_id;
            state.last_update = SystemTime::now();
        }
    }

    pub fn validate_all_widgets(&mut self) -> bool {
        let mut all_valid = true;
        
        for widget in &mut self.widgets {
            if widget.validate_and_clamp() {
                let widget_id = widget.id().to_string();
                let value = widget.get_value();
                
                if let Ok(mut state) = self.state.write() {
                    state.update_widget_value(&widget_id, value.clone());
                }
                
                tracing::debug!("Widget '{}' value clamped during validation", widget_id);
            }
            
            let current_value = widget.get_value();
            if !widget.validate(&current_value) {
                all_valid = false;
                tracing::warn!("Widget '{}' has invalid value: {:?}", widget.id(), current_value);
            }
        }
        
        all_valid
    }

    pub fn reset_all_changed_flags(&mut self) {
        for widget in &mut self.widgets {
            widget.reset_changed();
        }
        self.emergency_stop.reset_changed();
    }

    pub fn get_changed_widgets(&self) -> Vec<String> {
        let mut changed = Vec::new();
        
        for widget in &self.widgets {
            if widget.has_changed() {
                changed.push(widget.id().to_string());
            }
        }
        
        if self.emergency_stop.has_changed() {
            changed.push(self.emergency_stop.id().to_string());
        }
        
        changed
    }

    pub fn export_widget_metadata(&self) -> HashMap<String, WidgetMetadata> {
        let mut metadata = HashMap::new();
        
        for widget in &self.widgets {
            metadata.insert(widget.id().to_string(), widget.metadata());
        }
        
        metadata.insert(self.emergency_stop.id().to_string(), self.emergency_stop.metadata());
        
        metadata
    }
}

impl Default for ManualControlManager {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience functions for creating common control widgets
pub fn create_power_slider() -> Box<dyn ControlWidget> {
    PrecisionSlider::new("power", "Power", 0.0, 100.0, "%")
}

pub fn create_frequency_slider() -> Box<dyn ControlWidget> {
    PrecisionSlider::new("frequency", "Frequency", 1.0, 1000.0, "Hz")
}

pub fn create_mode_toggle() -> Box<dyn ControlWidget> {
    MultiStateToggle::new("mode", "Mode", vec![
        "Manual".to_string(),
        "Auto".to_string(),
        "Test".to_string(),
        "Calibrate".to_string()
    ])
}

pub fn create_protocol_dropdown() -> Box<dyn ControlWidget> {
    SelectionDropdown::new("protocol", "Protocol", vec![
        "Serial".to_string(),
        "TCP".to_string(),
        "UDP".to_string(),
        "SSH".to_string()
    ])
}

pub fn create_device_name_input() -> Box<dyn ControlWidget> {
    TextInput::new("device_name", "Device Name")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manual_control_state_creation() {
        let state = ManualControlState::new();
        assert!(!state.emergency_stop_engaged);
        assert_eq!(state.control_authority, ControlAuthority::None);
        assert!(state.widget_values.is_empty());
    }

    #[test]
    fn test_emergency_stop_engagement() {
        let mut state = ManualControlState::new();
        state.engage_emergency_stop();
        assert!(state.emergency_stop_engaged);
        assert_eq!(state.control_authority, ControlAuthority::Emergency);
    }

    #[test]
    fn test_widget_value_storage() {
        let mut state = ManualControlState::new();
        let value = ControlValue::Float(42.5);
        state.update_widget_value("test_widget", value.clone());
        
        assert_eq!(state.get_widget_value("test_widget"), Some(&value));
    }

    #[test]
    fn test_safety_interlocks() {
        let mut state = ManualControlState::new();
        state.set_safety_interlock("door_open", true);
        assert!(state.is_safety_interlock_engaged("door_open"));
        assert!(!state.can_control());
    }

    #[test]
    fn test_value_validator() {
        let validator = StandardValidator;
        
        // Test range validation
        match validator.validate_range(5.0, 0.0, 10.0) {
            ValidationResult::Valid => {},
            _ => panic!("Expected valid range"),
        }
        
        match validator.validate_range(-1.0, 0.0, 10.0) {
            ValidationResult::Clamped { .. } => {},
            _ => panic!("Expected clamped result"),
        }
        
        // Test enum validation
        let allowed = vec!["option1".to_string(), "option2".to_string()];
        match validator.validate_enum("option1", &allowed) {
            ValidationResult::Valid => {},
            _ => panic!("Expected valid enum"),
        }
    }

    #[test]
    fn test_manual_control_manager() {
        let mut manager = ManualControlManager::new();
        assert!(manager.is_enabled());
        assert!(!manager.is_emergency_stop_engaged());
        
        // Add a test widget
        let widget = create_power_slider();
        manager.add_widget(widget);
        
        assert_eq!(manager.widgets.len(), 1);
        assert!(manager.get_widget_by_id("power").is_some());
    }
}