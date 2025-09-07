// Log panel for displaying application logs

use egui::Ui;
use crate::logging::{LogLevel, LogEntry};

pub struct LogPanel {
    pub logs: Vec<LogEntry>,
    pub auto_scroll: bool,
    max_logs: usize,
}

impl LogPanel {
    pub fn new() -> Self {
        Self {
            logs: Vec::new(),
            auto_scroll: true,
            max_logs: 1000,
        }
    }
    
    pub fn add_log(&mut self, entry: LogEntry) {
        self.logs.push(entry);
        if self.logs.len() > self.max_logs {
            self.logs.remove(0);
        }
    }
    
    pub fn clear(&mut self) {
        self.logs.clear();
    }
    
    pub fn show(&mut self, ui: &mut Ui) {
        ui.label("Log Panel");
        ui.label(format!("Log entries: {}", self.logs.len()));
        
        // TODO: Add scrollable area with log entries
        egui::ScrollArea::vertical().show(ui, |ui| {
            for log in &self.logs {
                let color = match log.level {
                    LogLevel::Fatal => egui::Color32::from_rgb(255, 0, 128), // Bright red-pink
                    LogLevel::Error => egui::Color32::RED,
                    LogLevel::Warning => egui::Color32::YELLOW,
                    LogLevel::Info => egui::Color32::WHITE,
                    LogLevel::Debug => egui::Color32::GRAY,
                    LogLevel::Trace => egui::Color32::DARK_GRAY,
                };
                ui.colored_label(color, &log.message);
            }
        });
    }
}