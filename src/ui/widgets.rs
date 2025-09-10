use egui::{Ui, Response, Widget, Color32, Vec2, Rect, Sense, Painter, Stroke, FontId};

/// A custom LED indicator widget
pub struct LedIndicator {
    on: bool,
    color: Color32,
    size: f32,
}

impl LedIndicator {
    pub fn new(on: bool) -> Self {
        Self {
            on,
            color: if on { Color32::from_rgb(0, 255, 0) } else { Color32::from_rgb(64, 64, 64) },
            size: 12.0,
        }
    }
    
    pub fn with_color(mut self, color: Color32) -> Self {
        if self.on {
            self.color = color;
        }
        self
    }
    
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl Widget for LedIndicator {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.size);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());
        
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();
            let radius = self.size / 2.0;
            
            // Draw outer circle
            painter.circle_filled(center, radius, Color32::from_rgb(32, 32, 32));
            painter.circle_stroke(center, radius, Stroke::new(1.0, Color32::from_rgb(64, 64, 64)));
            
            // Draw inner LED
            if self.on {
                // Glow effect
                painter.circle_filled(center, radius * 0.8, self.color.gamma_multiply(0.3));
                painter.circle_filled(center, radius * 0.6, self.color);
                // Highlight
                let highlight_pos = center - Vec2::new(radius * 0.2, radius * 0.2);
                painter.circle_filled(highlight_pos, radius * 0.2, Color32::from_rgba_premultiplied(255, 255, 255, 100));
            } else {
                painter.circle_filled(center, radius * 0.7, self.color);
            }
        }
        
        response
    }
}

/// A custom knob/dial control widget
pub struct Knob {
    value: f32,
    min: f32,
    max: f32,
    size: f32,
    label: String,
}

impl Knob {
    pub fn new(value: f32, min: f32, max: f32) -> Self {
        Self {
            value,
            min,
            max,
            size: 40.0,
            label: String::new(),
        }
    }
    
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }
    
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

impl Widget for Knob {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.size);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());
        
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();
            let radius = self.size / 2.0;
            
            // Handle input
            if response.dragged() {
                let delta = response.drag_delta();
                let sensitivity = 0.01;
                let change = (delta.x - delta.y) * sensitivity;
                self.value = (self.value + change * (self.max - self.min))
                    .clamp(self.min, self.max);
            }
            
            // Draw knob base
            painter.circle_filled(center, radius, Color32::from_rgb(60, 60, 60));
            painter.circle_stroke(center, radius, Stroke::new(2.0, Color32::from_rgb(80, 80, 80)));
            
            // Draw indicator
            let normalized = (self.value - self.min) / (self.max - self.min);
            let angle = normalized * std::f32::consts::PI * 1.5 - std::f32::consts::PI * 0.75;
            let indicator_end = center + Vec2::new(
                angle.cos() * radius * 0.7,
                angle.sin() * radius * 0.7
            );
            painter.line_segment(
                [center, indicator_end],
                Stroke::new(3.0, Color32::from_rgb(0, 120, 215))
            );
            
            // Draw center dot
            painter.circle_filled(center, radius * 0.15, Color32::from_rgb(100, 100, 100));
            
            // Draw label if present
            if !self.label.is_empty() {
                let text_pos = center + Vec2::new(0.0, radius + 10.0);
                painter.text(
                    text_pos,
                    egui::Align2::CENTER_CENTER,
                    &self.label,
                    FontId::proportional(12.0),
                    Color32::from_rgb(200, 200, 200),
                );
            }
        }
        
        response
    }
}

/// A custom toggle switch widget (Windows 10 style)
pub struct ToggleSwitch {
    on: bool,
    size: Vec2,
}

impl ToggleSwitch {
    pub fn new(on: bool) -> Self {
        Self {
            on,
            size: Vec2::new(44.0, 20.0),
        }
    }
}

impl Widget for ToggleSwitch {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let (rect, mut response) = ui.allocate_exact_size(self.size, Sense::click());
        
        if response.clicked() {
            self.on = !self.on;
            response.mark_changed();
        }
        
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            
            // Track
            let track_color = if self.on {
                Color32::from_rgb(0, 120, 215)
            } else {
                Color32::from_rgb(100, 100, 100)
            };
            
            painter.rect_filled(
                rect,
                egui::Rounding::same(rect.height() / 2.0),
                track_color
            );
            
            // Thumb
            let thumb_radius = rect.height() / 2.0 - 2.0;
            let thumb_center_x = if self.on {
                rect.right() - thumb_radius - 2.0
            } else {
                rect.left() + thumb_radius + 2.0
            };
            
            painter.circle_filled(
                egui::pos2(thumb_center_x, rect.center().y),
                thumb_radius,
                Color32::from_rgb(255, 255, 255)
            );
        }
        
        response
    }
}

/// A custom progress ring widget
pub struct ProgressRing {
    progress: f32,
    size: f32,
    thickness: f32,
}

impl ProgressRing {
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            size: 40.0,
            thickness: 4.0,
        }
    }
    
    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
    
    pub fn with_thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }
}

impl Widget for ProgressRing {
    fn ui(self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.size);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::hover());
        
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();
            let radius = self.size / 2.0 - self.thickness / 2.0;
            
            // Background ring
            painter.circle_stroke(
                center,
                radius,
                Stroke::new(self.thickness, Color32::from_rgb(60, 60, 60))
            );
            
            // Progress arc
            if self.progress > 0.0 {
                use egui::epaint::PathShape;
                let n_points = (64.0 * self.progress) as usize + 1;
                let points: Vec<_> = (0..=n_points)
                    .map(|i| {
                        let angle = -std::f32::consts::FRAC_PI_2 
                            + (i as f32 / 64.0) * std::f32::consts::TAU * self.progress;
                        center + Vec2::new(angle.cos() * radius, angle.sin() * radius)
                    })
                    .collect();
                
                if points.len() >= 2 {
                    painter.add(PathShape::line(
                        points,
                        Stroke::new(self.thickness, Color32::from_rgb(0, 120, 215))
                    ));
                }
            }
            
            // Center text
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                format!("{:.0}%", self.progress * 100.0),
                FontId::proportional(14.0),
                Color32::from_rgb(200, 200, 200),
            );
        }
        
        response
    }
}