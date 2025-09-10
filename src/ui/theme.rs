use egui::{Context, Style, Visuals, Color32, Stroke, Rounding};

/// Windows 10 theme configuration
pub struct Windows10Theme {
    pub dark_mode: bool,
    pub accent_color: Color32,
}

impl Default for Windows10Theme {
    fn default() -> Self {
        Self {
            dark_mode: true,
            accent_color: Color32::from_rgb(0, 120, 215), // Windows 10 default blue
        }
    }
}

impl Windows10Theme {
    /// Apply Windows 10 theme to egui context
    pub fn apply(&self, ctx: &Context) {
        let mut style = Style::default();
        
        if self.dark_mode {
            style.visuals = self.dark_visuals();
        } else {
            style.visuals = self.light_visuals();
        }
        
        // Common Windows 10 styling
        style.spacing.item_spacing = egui::vec2(8.0, 4.0);
        style.spacing.button_padding = egui::vec2(10.0, 6.0);
        style.spacing.menu_margin = egui::Margin::same(6.0);
        style.spacing.indent = 18.0;
        
        ctx.set_style(style);
    }
    
    fn dark_visuals(&self) -> Visuals {
        Visuals {
            dark_mode: true,
            
            // Window colors
            window_fill: Color32::from_rgb(32, 32, 32),
            window_stroke: Stroke::new(1.0, Color32::from_rgb(48, 48, 48)),
            window_rounding: Rounding::same(0.0), // Windows 10 has sharp corners
            window_shadow: egui::epaint::Shadow::NONE, // egui 0.29 doesn't have small_dark
            
            // Panel colors
            panel_fill: Color32::from_rgb(40, 40, 40),
            faint_bg_color: Color32::from_rgb(48, 48, 48),
            extreme_bg_color: Color32::from_rgb(24, 24, 24),
            code_bg_color: Color32::from_rgb(56, 56, 56),
            
            // Text colors
            override_text_color: Some(Color32::from_rgb(255, 255, 255)),
            
            // Hyperlink
            hyperlink_color: self.accent_color,
            
            // Selection
            selection: egui::style::Selection {
                bg_fill: self.accent_color,
                stroke: Stroke::new(1.0, Color32::from_rgb(0, 150, 255)),
            },
            
            // Widgets
            widgets: Self::dark_widgets(self.accent_color),
            
            ..Default::default()
        }
    }
    
    fn light_visuals(&self) -> Visuals {
        Visuals {
            dark_mode: false,
            
            // Window colors
            window_fill: Color32::from_rgb(243, 243, 243),
            window_stroke: Stroke::new(1.0, Color32::from_rgb(229, 229, 229)),
            window_rounding: Rounding::same(0.0), // Windows 10 has sharp corners
            window_shadow: egui::epaint::Shadow::NONE, // egui 0.29 doesn't have small_light
            
            // Panel colors  
            panel_fill: Color32::from_rgb(251, 251, 251),
            faint_bg_color: Color32::from_rgb(238, 238, 238),
            extreme_bg_color: Color32::from_rgb(255, 255, 255),
            code_bg_color: Color32::from_rgb(230, 230, 230),
            
            // Text colors
            override_text_color: Some(Color32::from_rgb(0, 0, 0)),
            
            // Hyperlink
            hyperlink_color: self.accent_color,
            
            // Selection
            selection: egui::style::Selection {
                bg_fill: self.accent_color,
                stroke: Stroke::new(1.0, Color32::from_rgb(0, 103, 192)),
            },
            
            // Widgets
            widgets: Self::light_widgets(self.accent_color),
            
            ..Default::default()
        }
    }
    
    fn dark_widgets(accent: Color32) -> egui::style::Widgets {
        egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                weak_bg_fill: Color32::from_rgb(50, 50, 50),
                bg_fill: Color32::from_rgb(60, 60, 60),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(80, 80, 80)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                weak_bg_fill: Color32::from_rgb(50, 50, 50),
                bg_fill: Color32::from_rgb(60, 60, 60),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(80, 80, 80)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                weak_bg_fill: Color32::from_rgb(60, 60, 60),
                bg_fill: Color32::from_rgb(70, 70, 70),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(90, 90, 90)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
            active: egui::style::WidgetVisuals {
                weak_bg_fill: accent,
                bg_fill: accent,
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(0, 150, 255)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
            open: egui::style::WidgetVisuals {
                weak_bg_fill: Color32::from_rgb(40, 40, 40),
                bg_fill: Color32::from_rgb(48, 48, 48),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(70, 70, 70)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
        }
    }
    
    fn light_widgets(accent: Color32) -> egui::style::Widgets {
        egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                weak_bg_fill: Color32::from_rgb(245, 245, 245),
                bg_fill: Color32::from_rgb(240, 240, 240),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(220, 220, 220)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(0, 0, 0)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                weak_bg_fill: Color32::from_rgb(245, 245, 245),
                bg_fill: Color32::from_rgb(240, 240, 240),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(220, 220, 220)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(0, 0, 0)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                weak_bg_fill: Color32::from_rgb(235, 235, 235),
                bg_fill: Color32::from_rgb(230, 230, 230),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(210, 210, 210)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(0, 0, 0)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
            active: egui::style::WidgetVisuals {
                weak_bg_fill: accent,
                bg_fill: accent,
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(0, 103, 192)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
            open: egui::style::WidgetVisuals {
                weak_bg_fill: Color32::from_rgb(240, 240, 240),
                bg_fill: Color32::from_rgb(238, 238, 238),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(220, 220, 220)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(0, 0, 0)),
                rounding: Rounding::same(2.0), // Windows 10 has less rounding
                expansion: 0.0,
            },
        }
    }
}