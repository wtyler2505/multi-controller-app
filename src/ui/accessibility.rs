use egui::{Context, Key, Response, Ui};

/// Accessibility helpers for the Multi-Controller App
pub struct AccessibilityHelpers;

impl AccessibilityHelpers {
    /// Add keyboard shortcut hints to a widget
    pub fn with_shortcut_hint(response: Response, shortcut: &str, description: &str) -> Response {
        response.on_hover_text(format!("{} ({})", description, shortcut))
    }
    
    /// Create an accessible label with proper heading level
    pub fn heading_with_level(ui: &mut Ui, text: &str, level: u8) {
        match level {
            1 => ui.heading(text),
            2 => ui.label(egui::RichText::new(text).heading().strong()),
            3 => ui.label(egui::RichText::new(text).strong()),
            _ => ui.label(text),
        };
    }
    
    /// Check if screen reader is active and add announcements
    pub fn announce_if_screen_reader(ctx: &Context, ui: &mut Ui, text: &str) {
        // egui 0.29 doesn't have screen_reader_active, use accessibility mode check
        if ctx.options(|opt| opt.screen_reader) {
            ui.label(text);
        }
    }
    
    /// Add ARIA-like role description to a widget
    pub fn with_role_description(response: Response, role: &str) -> Response {
        response.on_hover_text(format!("Role: {}", role))
    }
    
    /// Ensure proper focus order
    pub fn set_tab_index(ui: &mut Ui, index: u32) {
        // egui doesn't have explicit tab index, but we can use Id ordering
        ui.id().with(index);
    }
    
    /// Check for high contrast mode preference
    pub fn is_high_contrast_mode(ctx: &Context) -> bool {
        // Check system preference or user setting
        // For now, we'll check a custom setting
        ctx.data(|data| data.get_temp::<bool>(egui::Id::new("high_contrast")).unwrap_or(false))
    }
    
    /// Toggle high contrast mode
    pub fn toggle_high_contrast(ctx: &Context) {
        ctx.data_mut(|data| {
            let current = data.get_temp::<bool>(egui::Id::new("high_contrast")).unwrap_or(false);
            data.insert_temp(egui::Id::new("high_contrast"), !current);
        });
    }
    
    /// Apply high contrast colors if enabled
    pub fn apply_high_contrast_if_enabled(ctx: &Context) {
        if Self::is_high_contrast_mode(ctx) {
            let mut style = (*ctx.style()).clone();
            
            // High contrast colors
            style.visuals.override_text_color = Some(egui::Color32::WHITE);
            style.visuals.window_fill = egui::Color32::BLACK;
            style.visuals.panel_fill = egui::Color32::BLACK;
            style.visuals.faint_bg_color = egui::Color32::from_rgb(20, 20, 20);
            style.visuals.extreme_bg_color = egui::Color32::BLACK;
            style.visuals.code_bg_color = egui::Color32::from_rgb(10, 10, 10);
            
            // High contrast selection
            style.visuals.selection.bg_fill = egui::Color32::from_rgb(255, 255, 0);
            style.visuals.selection.stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
            
            // High contrast widgets
            style.visuals.widgets.noninteractive.weak_bg_fill = egui::Color32::from_rgb(30, 30, 30);
            style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(40, 40, 40);
            style.visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
            style.visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
            
            style.visuals.widgets.inactive = style.visuals.widgets.noninteractive;
            
            style.visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(60, 60, 60);
            style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(80, 80, 80);
            style.visuals.widgets.hovered.bg_stroke = egui::Stroke::new(2.0, egui::Color32::YELLOW);
            style.visuals.widgets.hovered.fg_stroke = egui::Stroke::new(2.0, egui::Color32::YELLOW);
            
            style.visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(100, 100, 0);
            style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(120, 120, 0);
            style.visuals.widgets.active.bg_stroke = egui::Stroke::new(2.0, egui::Color32::YELLOW);
            style.visuals.widgets.active.fg_stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
            
            style.visuals.widgets.open = style.visuals.widgets.active;
            
            ctx.set_style(style);
        }
    }
}

/// Keyboard shortcuts manager
pub struct KeyboardShortcuts;

impl KeyboardShortcuts {
    /// Check for standard navigation shortcuts
    pub fn check_navigation(ctx: &Context) -> Option<NavigationAction> {
        ctx.input(|i| {
            // F6 - Cycle through panels
            if i.key_pressed(Key::F6) {
                return Some(NavigationAction::CyclePanels);
            }
            
            // Tab navigation
            if i.key_pressed(Key::Tab) {
                if i.modifiers.shift {
                    return Some(NavigationAction::PreviousControl);
                } else if i.modifiers.ctrl {
                    return Some(NavigationAction::NextTab);
                } else {
                    return Some(NavigationAction::NextControl);
                }
            }
            
            // Alt shortcuts
            if i.modifiers.alt {
                if i.key_pressed(Key::D) {
                    return Some(NavigationAction::FocusDevicePanel);
                }
                if i.key_pressed(Key::M) {
                    return Some(NavigationAction::FocusMainPanel);
                }
                if i.key_pressed(Key::H) {
                    return Some(NavigationAction::ToggleHighContrast);
                }
            }
            
            // Function keys
            if i.key_pressed(Key::F5) {
                return Some(NavigationAction::Refresh);
            }
            
            if i.key_pressed(Key::F1) {
                return Some(NavigationAction::ShowHelp);
            }
            
            // Escape to cancel/clear
            if i.key_pressed(Key::Escape) {
                return Some(NavigationAction::ClearSelection);
            }
            
            // Number keys for tabs (with Ctrl)
            if i.modifiers.ctrl {
                if i.key_pressed(Key::Num1) {
                    return Some(NavigationAction::GoToTab(0));
                }
                if i.key_pressed(Key::Num2) {
                    return Some(NavigationAction::GoToTab(1));
                }
                if i.key_pressed(Key::Num3) {
                    return Some(NavigationAction::GoToTab(2));
                }
                if i.key_pressed(Key::Num4) {
                    return Some(NavigationAction::GoToTab(3));
                }
                if i.key_pressed(Key::Num5) {
                    return Some(NavigationAction::GoToTab(4));
                }
            }
            
            None
        })
    }
}

/// Navigation actions from keyboard shortcuts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationAction {
    CyclePanels,
    NextControl,
    PreviousControl,
    NextTab,
    FocusDevicePanel,
    FocusMainPanel,
    ToggleHighContrast,
    Refresh,
    ShowHelp,
    ClearSelection,
    GoToTab(usize),
}

/// Focus management helper
pub struct FocusManager;

impl FocusManager {
    /// Request focus for a specific widget
    pub fn request_focus(ctx: &Context, id: egui::Id) {
        ctx.memory_mut(|mem| {
            mem.request_focus(id);
        });
    }
    
    /// Check if a widget has focus
    pub fn has_focus(ctx: &Context, id: egui::Id) -> bool {
        ctx.memory(|mem| mem.has_focus(id))
    }
    
    /// Clear focus
    pub fn clear_focus(ctx: &Context) {
        ctx.memory_mut(|mem| {
            mem.surrender_focus(egui::Id::new("null"));
        });
    }
    
    /// Create a focus trap region
    pub fn focus_trap(ui: &mut Ui, first_id: egui::Id, last_id: egui::Id) {
        // When Tab is pressed on last element, focus first
        // When Shift+Tab is pressed on first element, focus last
        if ui.input(|i| i.key_pressed(Key::Tab)) {
            let ctx = ui.ctx();
            if FocusManager::has_focus(ctx, last_id) && !ui.input(|i| i.modifiers.shift) {
                FocusManager::request_focus(ctx, first_id);
            } else if FocusManager::has_focus(ctx, first_id) && ui.input(|i| i.modifiers.shift) {
                FocusManager::request_focus(ctx, last_id);
            }
        }
    }
}

/// Screen reader announcements
pub struct ScreenReaderAnnouncer;

impl ScreenReaderAnnouncer {
    /// Announce a message if screen reader is active
    pub fn announce(ctx: &Context, ui: &mut Ui, message: &str, priority: AnnouncementPriority) {
        // egui 0.29 doesn't have screen_reader_active, use accessibility mode check
        if ctx.options(|opt| opt.screen_reader) {
            match priority {
                AnnouncementPriority::Polite => {
                    ui.label(message);
                }
                AnnouncementPriority::Assertive => {
                    ui.label(egui::RichText::new(message).strong());
                }
            }
        }
    }
    
    /// Announce state change
    pub fn announce_state_change(ctx: &Context, ui: &mut Ui, component: &str, old_state: &str, new_state: &str) {
        Self::announce(
            ctx,
            ui,
            &format!("{} changed from {} to {}", component, old_state, new_state),
            AnnouncementPriority::Polite
        );
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AnnouncementPriority {
    Polite,
    Assertive,
}