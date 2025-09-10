//! UI Control Widgets - Minimal Implementation
//! 
//! This provides basic implementations to satisfy dependencies for the telemetry parser demo.

use egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlValue {
    Float(f64),
    Integer(i32),
    Boolean(bool),
    String(String),
}

pub trait ControlWidget: Send + Sync {
    fn id(&self) -> &str;
    fn render(&mut self, ui: &mut Ui) -> bool;
    fn get_value(&self) -> ControlValue;
    fn set_value(&mut self, value: ControlValue) -> Result<(), String>;
    fn validate(&self, value: &ControlValue) -> bool;
    fn metadata(&self) -> WidgetMetadata;
    fn has_changed(&self) -> bool;
    fn reset_changed(&mut self);
    fn emergency_stop(&mut self);
    fn validate_and_clamp(&mut self) -> bool;
}

#[derive(Debug, Clone)]
pub struct WidgetMetadata {
    pub id: String,
    pub widget_type: String,
    pub value: ControlValue,
}

pub struct PrecisionSlider {
    id: String,
    value: f64,
    min: f64,
    max: f64,
    label: String,
    unit: String,
    precision: usize,
    changed: bool,
}

impl PrecisionSlider {
    pub fn new(id: &str, label: &str, min: f64, max: f64, unit: &str) -> Box<dyn ControlWidget> {
        Box::new(Self { 
            id: id.to_string(), 
            value: min, 
            min, 
            max,
            label: label.to_string(),
            unit: unit.to_string(),
            precision: 2,
            changed: false,
        })
    }
}

impl ControlWidget for PrecisionSlider {
    fn id(&self) -> &str { &self.id }
    
    fn render(&mut self, ui: &mut Ui) -> bool {
        let mut temp_value = self.value as f32;
        let response = ui.add(egui::Slider::new(&mut temp_value, self.min as f32..=self.max as f32)
            .text(&self.label));
        
        if response.changed() {
            self.value = temp_value as f64;
            self.changed = true;
            true
        } else {
            false
        }
    }
    
    fn get_value(&self) -> ControlValue { ControlValue::Float(self.value) }
    
    fn set_value(&mut self, value: ControlValue) -> Result<(), String> {
        if let ControlValue::Float(v) = value { 
            self.value = v.clamp(self.min, self.max);
            self.changed = true;
            Ok(())
        } else {
            Err("Invalid value type for PrecisionSlider".to_string())
        }
    }
    
    fn validate(&self, value: &ControlValue) -> bool {
        if let ControlValue::Float(v) = value {
            *v >= self.min && *v <= self.max && v.is_finite()
        } else {
            false
        }
    }
    
    fn metadata(&self) -> WidgetMetadata {
        WidgetMetadata {
            id: self.id.clone(),
            widget_type: "PrecisionSlider".to_string(),
            value: self.get_value(),
        }
    }
    
    fn has_changed(&self) -> bool { self.changed }
    fn reset_changed(&mut self) { self.changed = false; }
    
    fn emergency_stop(&mut self) {
        self.value = (self.min + self.max) / 2.0;
        self.changed = true;
    }
    
    fn validate_and_clamp(&mut self) -> bool {
        let old_value = self.value;
        self.value = self.value.clamp(self.min, self.max);
        old_value != self.value
    }
}

pub struct MultiStateToggle {
    id: String,
    states: Vec<String>,
    current: usize,
    label: String,
    changed: bool,
}

impl MultiStateToggle {
    pub fn new(id: &str, label: &str, states: Vec<String>) -> Box<dyn ControlWidget> {
        Box::new(Self { 
            id: id.to_string(), 
            states, 
            current: 0,
            label: label.to_string(),
            changed: false,
        })
    }
}

impl ControlWidget for MultiStateToggle {
    fn id(&self) -> &str { &self.id }
    
    fn render(&mut self, ui: &mut Ui) -> bool { 
        let unknown_state = "Unknown".to_string();
        let current_state = self.states.get(self.current).unwrap_or(&unknown_state);
        let response = ui.button(format!("{}: {}", self.label, current_state));
        
        if response.clicked() {
            self.current = (self.current + 1) % self.states.len();
            self.changed = true;
            true
        } else {
            false
        }
    }
    
    fn get_value(&self) -> ControlValue { ControlValue::Integer(self.current as i32) }
    
    fn set_value(&mut self, value: ControlValue) -> Result<(), String> {
        if let ControlValue::Integer(i) = value { 
            if i >= 0 && (i as usize) < self.states.len() {
                self.current = i as usize;
                self.changed = true;
                Ok(())
            } else {
                Err("Index out of range".to_string())
            }
        } else {
            Err("Invalid value type for MultiStateToggle".to_string())
        }
    }
    
    fn validate(&self, value: &ControlValue) -> bool {
        if let ControlValue::Integer(i) = value {
            *i >= 0 && (*i as usize) < self.states.len()
        } else {
            false
        }
    }
    
    fn metadata(&self) -> WidgetMetadata {
        WidgetMetadata {
            id: self.id.clone(),
            widget_type: "MultiStateToggle".to_string(),
            value: self.get_value(),
        }
    }
    
    fn has_changed(&self) -> bool { self.changed }
    fn reset_changed(&mut self) { self.changed = false; }
    
    fn emergency_stop(&mut self) {
        self.current = 0;  // Reset to first state
        self.changed = true;
    }
    
    fn validate_and_clamp(&mut self) -> bool {
        if self.current >= self.states.len() {
            self.current = 0;
            true
        } else {
            false
        }
    }
}

pub struct SelectionDropdown {
    id: String,
    selected: usize,
    options: Vec<String>,
    label: String,
    changed: bool,
}

impl SelectionDropdown {
    pub fn new(id: &str, label: &str, options: Vec<String>) -> Box<dyn ControlWidget> {
        Box::new(Self { 
            id: id.to_string(), 
            selected: 0,
            options,
            label: label.to_string(),
            changed: false,
        })
    }
}

impl ControlWidget for SelectionDropdown {
    fn id(&self) -> &str { &self.id }
    
    fn render(&mut self, ui: &mut Ui) -> bool { 
        let none_option = "None".to_string();
        let current_option = self.options.get(self.selected).unwrap_or(&none_option);
        let mut changed = false;
        
        egui::ComboBox::from_label(&self.label)
            .selected_text(current_option)
            .show_ui(ui, |ui| {
                for (i, option) in self.options.iter().enumerate() {
                    if ui.selectable_value(&mut self.selected, i, option).clicked() {
                        changed = true;
                        self.changed = true;
                    }
                }
            });
        
        changed
    }
    
    fn get_value(&self) -> ControlValue { ControlValue::Integer(self.selected as i32) }
    
    fn set_value(&mut self, value: ControlValue) -> Result<(), String> {
        if let ControlValue::Integer(i) = value { 
            if i >= 0 && (i as usize) < self.options.len() {
                self.selected = i as usize;
                self.changed = true;
                Ok(())
            } else {
                Err("Index out of range".to_string())
            }
        } else {
            Err("Invalid value type for SelectionDropdown".to_string())
        }
    }
    
    fn validate(&self, value: &ControlValue) -> bool {
        if let ControlValue::Integer(i) = value {
            *i >= 0 && (*i as usize) < self.options.len()
        } else {
            false
        }
    }
    
    fn metadata(&self) -> WidgetMetadata {
        WidgetMetadata {
            id: self.id.clone(),
            widget_type: "SelectionDropdown".to_string(),
            value: self.get_value(),
        }
    }
    
    fn has_changed(&self) -> bool { self.changed }
    fn reset_changed(&mut self) { self.changed = false; }
    
    fn emergency_stop(&mut self) {
        self.selected = 0;  // Reset to first option
        self.changed = true;
    }
    
    fn validate_and_clamp(&mut self) -> bool {
        if self.selected >= self.options.len() {
            self.selected = 0;
            true
        } else {
            false
        }
    }
}

pub struct TextInput {
    id: String,
    value: String,
    label: String,
    placeholder: String,
    changed: bool,
}

impl TextInput {
    pub fn new(id: &str, label: &str) -> Box<dyn ControlWidget> {
        Box::new(Self { 
            id: id.to_string(), 
            value: String::new(),
            label: label.to_string(),
            placeholder: String::new(),
            changed: false,
        })
    }
}

impl ControlWidget for TextInput {
    fn id(&self) -> &str { &self.id }
    
    fn render(&mut self, ui: &mut Ui) -> bool { 
        ui.horizontal(|ui| {
            ui.label(&self.label);
            let response = ui.text_edit_singleline(&mut self.value);
            if response.changed() {
                self.changed = true;
                true
            } else {
                false
            }
        }).inner
    }
    
    fn get_value(&self) -> ControlValue { ControlValue::String(self.value.clone()) }
    
    fn set_value(&mut self, value: ControlValue) -> Result<(), String> {
        if let ControlValue::String(s) = value { 
            self.value = s;
            self.changed = true;
            Ok(())
        } else {
            Err("Invalid value type for TextInput".to_string())
        }
    }
    
    fn validate(&self, value: &ControlValue) -> bool {
        matches!(value, ControlValue::String(_))
    }
    
    fn metadata(&self) -> WidgetMetadata {
        WidgetMetadata {
            id: self.id.clone(),
            widget_type: "TextInput".to_string(),
            value: self.get_value(),
        }
    }
    
    fn has_changed(&self) -> bool { self.changed }
    fn reset_changed(&mut self) { self.changed = false; }
    
    fn emergency_stop(&mut self) {
        self.value.clear();
        self.changed = true;
    }
    
    fn validate_and_clamp(&mut self) -> bool {
        // Text input doesn't need clamping, just validation
        false
    }
}

pub struct EmergencyStopWidget {
    id: String,
    engaged: bool,
    label: String,
    changed: bool,
}

impl EmergencyStopWidget {
    pub fn new(id: &str, label: &str) -> Self {
        Self { 
            id: id.to_string(), 
            engaged: false,
            label: label.to_string(),
            changed: false,
        }
    }

    pub fn is_engaged(&self) -> bool {
        self.engaged
    }
    
    pub fn reset(&mut self) {
        self.engaged = false;
        self.changed = true;
    }
    
    pub fn engage(&mut self) {
        self.engaged = true;
        self.changed = true;
    }
}

impl ControlWidget for EmergencyStopWidget {
    fn id(&self) -> &str { &self.id }
    
    fn render(&mut self, ui: &mut Ui) -> bool { 
        let button_color = if self.engaged {
            egui::Color32::from_rgb(200, 50, 50)  // Bright red when engaged
        } else {
            egui::Color32::from_rgb(150, 30, 30)  // Darker red when not engaged
        };
        
        let button_text = if self.engaged { "STOP ENGAGED" } else { "EMERGENCY STOP" };
        
        let response = ui.add(
            egui::Button::new(button_text)
                .fill(button_color)
                .min_size(egui::Vec2::new(120.0, 40.0))
        );
        
        if response.clicked() {
            self.engaged = !self.engaged;
            self.changed = true;
            true
        } else {
            false
        }
    }
    
    fn get_value(&self) -> ControlValue { ControlValue::Boolean(self.engaged) }
    
    fn set_value(&mut self, value: ControlValue) -> Result<(), String> {
        if let ControlValue::Boolean(b) = value { 
            self.engaged = b;
            self.changed = true;
            Ok(())
        } else {
            Err("Invalid value type for EmergencyStopWidget".to_string())
        }
    }
    
    fn validate(&self, value: &ControlValue) -> bool {
        matches!(value, ControlValue::Boolean(_))
    }
    
    fn metadata(&self) -> WidgetMetadata {
        WidgetMetadata {
            id: self.id.clone(),
            widget_type: "EmergencyStopWidget".to_string(),
            value: self.get_value(),
        }
    }
    
    fn has_changed(&self) -> bool { self.changed }
    fn reset_changed(&mut self) { self.changed = false; }
    
    fn emergency_stop(&mut self) {
        self.engaged = true;  // Emergency stop should always engage
        self.changed = true;
    }
    
    fn validate_and_clamp(&mut self) -> bool {
        // Emergency stop doesn't need clamping
        false
    }
}