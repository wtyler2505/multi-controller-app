// Telemetry panel for real-time data visualization

use std::sync::Arc;
use egui::Ui;
use crate::telemetry::TelemetryChannel;

pub struct TelemetryPanel {
    pub channel: Option<Arc<TelemetryChannel>>,
    data: Vec<f32>,
}

impl TelemetryPanel {
    pub fn new() -> Self {
        Self {
            channel: None,
            data: Vec::new(),
        }
    }
    
    pub fn set_channel(&mut self, channel: Arc<TelemetryChannel>) {
        self.channel = Some(channel);
        self.data.clear();
    }
    
    pub fn add_data(&mut self, value: f32) {
        self.data.push(value);
        if self.data.len() > 1000 {
            self.data.remove(0);
        }
    }
    
    pub fn show(&mut self, ui: &mut Ui) {
        ui.label("Telemetry Panel");
        if let Some(_channel) = &self.channel {
            // TODO: Add channel name display when getter is available
            ui.label(format!("Data points: {}", self.data.len()));
        }
    }
}