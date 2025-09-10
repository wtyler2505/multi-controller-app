use multi_controller_app::ui::controls::*;
use std::time::Duration;

#[test]
fn test_precision_slider_creation() {
    let slider = PrecisionSlider::new("test", "Test Slider", 0.0, 100.0, "%");
    assert_eq!(slider.id(), "test");
    
    let value = slider.get_value();
    if let ControlValue::Float(v) = value {
        assert!(v >= 0.0 && v <= 100.0);
    } else {
        panic!("Expected Float value");
    }
}

#[test] 
fn test_precision_slider_validation() {
    let slider = PrecisionSlider::new("test", "Test", 0.0, 10.0, "");
    
    // Valid value
    assert!(slider.validate(&ControlValue::Float(5.0)));
    
    // Invalid values
    assert!(!slider.validate(&ControlValue::Float(-1.0)));
    assert!(!slider.validate(&ControlValue::Float(11.0)));
    assert!(!slider.validate(&ControlValue::Float(f64::NAN)));
    assert!(!slider.validate(&ControlValue::String("invalid".to_string())));
}

#[test]
fn test_precision_slider_set_value() {
    let mut slider = PrecisionSlider::new("test", "Test", 0.0, 10.0, "");
    
    // Valid set
    assert!(slider.set_value(ControlValue::Float(7.5)).is_ok());
    if let ControlValue::Float(v) = slider.get_value() {
        assert_eq!(v, 7.5);
    }
    
    // Clamped value
    assert!(slider.set_value(ControlValue::Float(15.0)).is_ok());
    if let ControlValue::Float(v) = slider.get_value() {
        assert_eq!(v, 10.0); // Should be clamped to max
    }
    
    // Invalid type
    assert!(slider.set_value(ControlValue::String("invalid".to_string())).is_err());
}

#[test]
fn test_emergency_stop_widget() {
    let mut emergency_stop = EmergencyStopWidget::new("estop", "Emergency Stop");
    
    // Initially not engaged
    assert!(!emergency_stop.is_engaged());
    assert_eq!(emergency_stop.get_value(), ControlValue::Boolean(false));
    
    // Engage emergency stop
    emergency_stop.engage();
    assert!(emergency_stop.is_engaged());
    assert_eq!(emergency_stop.get_value(), ControlValue::Boolean(true));
    
    // Reset emergency stop
    emergency_stop.reset();
    assert!(!emergency_stop.is_engaged());
    assert_eq!(emergency_stop.get_value(), ControlValue::Boolean(false));
}

#[test]
fn test_multi_state_toggle() {
    let states = vec!["Off".to_string(), "On".to_string(), "Auto".to_string()];
    let mut toggle = MultiStateToggle::new("mode", "Mode", states.clone());
    
    // Initially at first state
    assert_eq!(toggle.current_state(), "Off");
    assert_eq!(toggle.get_value(), ControlValue::Selection("Off".to_string()));
    
    // Set to specific state
    assert!(toggle.set_value(ControlValue::Selection("Auto".to_string())).is_ok());
    assert_eq!(toggle.current_state(), "Auto");
    
    // Set by index
    assert!(toggle.set_value(ControlValue::Integer(1)).is_ok());
    assert_eq!(toggle.current_state(), "On");
    
    // Invalid state
    assert!(toggle.set_value(ControlValue::Selection("Invalid".to_string())).is_err());
    
    // Invalid index
    assert!(toggle.set_value(ControlValue::Integer(10)).is_err());
}

#[test]
fn test_selection_dropdown() {
    let options = vec!["Option1".to_string(), "Option2".to_string(), "Option3".to_string()];
    let mut dropdown = SelectionDropdown::new("select", "Select", options.clone());
    
    // Initially at first option
    assert_eq!(dropdown.selected_option(), "Option1");
    
    // Set valid option
    assert!(dropdown.set_value(ControlValue::Selection("Option2".to_string())).is_ok());
    assert_eq!(dropdown.selected_option(), "Option2");
    
    // Validate options
    assert!(dropdown.validate(&ControlValue::Selection("Option1".to_string())));
    assert!(!dropdown.validate(&ControlValue::Selection("Invalid".to_string())));
}

#[test]
fn test_text_input() {
    let mut input = TextInput::new("name", "Name")
        .with_max_length(10)
        .with_placeholder("Enter name");
    
    // Set valid text
    assert!(input.set_value(ControlValue::String("Hello".to_string())).is_ok());
    assert_eq!(input.text(), "Hello");
    
    // Max length enforcement
    assert!(input.set_value(ControlValue::String("VeryLongTextThatExceedsLimit".to_string())).is_ok());
    assert_eq!(input.text().len(), 10); // Should be truncated
}

#[test]
fn test_manual_control_state() {
    let mut state = ManualControlState::new();
    
    // Initial state
    assert!(!state.emergency_stop_engaged);
    assert_eq!(state.control_authority, ControlAuthority::None);
    assert!(!state.can_control());
    
    // Set authority to full
    state.control_authority = ControlAuthority::Full;
    assert!(state.can_control());
    
    // Engage emergency stop
    state.engage_emergency_stop();
    assert!(state.emergency_stop_engaged);
    assert_eq!(state.control_authority, ControlAuthority::Emergency);
    assert!(!state.can_control()); // Emergency stop prevents control
    
    // Set safety interlock
    state.disengage_emergency_stop();
    state.control_authority = ControlAuthority::Full;
    state.set_safety_interlock("door_open", true);
    assert!(!state.can_control()); // Safety interlock prevents control
    
    // Clear safety interlock
    state.set_safety_interlock("door_open", false);
    assert!(state.can_control());
}

#[test]
fn test_widget_value_storage() {
    let mut state = ManualControlState::new();
    
    // Store and retrieve widget values
    let value = ControlValue::Float(42.5);
    state.update_widget_value("test_widget", value.clone());
    
    assert_eq!(state.get_widget_value("test_widget"), Some(&value));
    assert_eq!(state.get_widget_value("nonexistent"), None);
    
    // Update existing value
    let new_value = ControlValue::Float(100.0);
    state.update_widget_value("test_widget", new_value.clone());
    assert_eq!(state.get_widget_value("test_widget"), Some(&new_value));
}

#[test]
fn test_manual_control_manager() {
    let mut manager = ManualControlManager::new();
    
    // Initial state
    assert!(manager.is_enabled());
    assert!(!manager.is_emergency_stop_engaged());
    
    // Add widgets
    manager.add_widget(create_power_slider());
    manager.add_widget(create_frequency_slider());
    manager.add_widget(create_mode_toggle());
    
    // Check widgets are added
    assert!(manager.get_widget_by_id("power").is_some());
    assert!(manager.get_widget_by_id("frequency").is_some());
    assert!(manager.get_widget_by_id("mode").is_some());
    assert!(manager.get_widget_by_id("nonexistent").is_none());
    
    // Test widget grouping
    manager.create_widget_group("basic_controls".to_string(), vec![
        "power".to_string(),
        "frequency".to_string()
    ]);
    
    manager.add_widget_to_group("advanced", "mode");
    
    // Test emergency stop
    manager.emergency_stop_all();
    assert!(manager.is_emergency_stop_engaged());
    
    manager.reset_emergency_stop();
    assert!(!manager.is_emergency_stop_engaged());
}

#[test]
fn test_control_authority_levels() {
    let mut manager = ManualControlManager::new();
    
    // Test different authority levels
    manager.set_control_authority(ControlAuthority::None);
    let state = manager.get_state_snapshot();
    assert_eq!(state.control_authority, ControlAuthority::None);
    assert!(!state.can_control());
    
    manager.set_control_authority(ControlAuthority::ReadOnly);
    let state = manager.get_state_snapshot();
    assert_eq!(state.control_authority, ControlAuthority::ReadOnly);
    assert!(!state.can_control());
    
    manager.set_control_authority(ControlAuthority::Limited);
    let state = manager.get_state_snapshot();
    assert_eq!(state.control_authority, ControlAuthority::Limited);
    assert!(state.can_control());
    
    manager.set_control_authority(ControlAuthority::Full);
    let state = manager.get_state_snapshot();
    assert_eq!(state.control_authority, ControlAuthority::Full);
    assert!(state.can_control());
}

#[test]
fn test_safety_interlocks() {
    let manager = ManualControlManager::new();
    
    // Set various safety interlocks
    manager.set_safety_interlock("door_open", true);
    manager.set_safety_interlock("temperature_high", false);
    manager.set_safety_interlock("pressure_low", true);
    
    let state = manager.get_state_snapshot();
    assert!(state.is_safety_interlock_engaged("door_open"));
    assert!(!state.is_safety_interlock_engaged("temperature_high"));
    assert!(state.is_safety_interlock_engaged("pressure_low"));
    assert!(!state.is_safety_interlock_engaged("nonexistent"));
    
    // Cannot control with any safety interlock engaged
    manager.set_control_authority(ControlAuthority::Full);
    let state = manager.get_state_snapshot();
    assert!(!state.can_control());
    
    // Clear all interlocks
    manager.set_safety_interlock("door_open", false);
    manager.set_safety_interlock("pressure_low", false);
    let state = manager.get_state_snapshot();
    assert!(state.can_control());
}

#[test]
fn test_value_validator() {
    let validator = StandardValidator;
    
    // Range validation
    assert!(matches!(validator.validate_range(5.0, 0.0, 10.0), ValidationResult::Valid));
    assert!(matches!(validator.validate_range(-1.0, 0.0, 10.0), ValidationResult::Clamped { .. }));
    assert!(matches!(validator.validate_range(15.0, 0.0, 10.0), ValidationResult::Clamped { .. }));
    assert!(matches!(validator.validate_range(f64::NAN, 0.0, 10.0), ValidationResult::Invalid { .. }));
    
    // Precision validation
    assert!(matches!(validator.validate_precision(1.23, 2), ValidationResult::Valid));
    assert!(matches!(validator.validate_precision(1.234, 2), ValidationResult::Clamped { .. }));
    
    // Enum validation
    let allowed = vec!["option1".to_string(), "option2".to_string()];
    assert!(matches!(validator.validate_enum("option1", &allowed), ValidationResult::Valid));
    assert!(matches!(validator.validate_enum("invalid", &allowed), ValidationResult::Invalid { .. }));
    
    // Input sanitization
    let sanitized = validator.sanitize_input("Hello, World! <script>alert('xss')</script>");
    assert!(!sanitized.contains('<'));
    assert!(!sanitized.contains('>'));
    assert!(sanitized.contains("Hello, World!"));
}

#[test]
fn test_widget_metadata() {
    let slider = PrecisionSlider::new("test", "Test Slider", 0.0, 100.0, "%")
        .with_precision(2)
        .with_description("Test slider widget");
    
    let metadata = slider.metadata();
    assert_eq!(metadata.widget_type, "PrecisionSlider");
    assert_eq!(metadata.label, "Test Slider");
    assert_eq!(metadata.description, Some("Test slider widget".to_string()));
    assert_eq!(metadata.min_value, Some(0.0));
    assert_eq!(metadata.max_value, Some(100.0));
    assert_eq!(metadata.unit, Some("%".to_string()));
    assert_eq!(metadata.precision, Some(2));
    assert!(!metadata.is_emergency_control);
    assert_eq!(metadata.update_rate_ms, 16);
}

#[test]
fn test_convenience_functions() {
    // Test all convenience widget creation functions
    let power = create_power_slider();
    assert_eq!(power.id(), "power");
    assert!(matches!(power.get_value(), ControlValue::Float(_)));
    
    let frequency = create_frequency_slider();
    assert_eq!(frequency.id(), "frequency");
    
    let mode = create_mode_toggle();
    assert_eq!(mode.id(), "mode");
    assert!(matches!(mode.get_value(), ControlValue::Selection(_)));
    
    let protocol = create_protocol_dropdown();
    assert_eq!(protocol.id(), "protocol");
    
    let name_input = create_device_name_input();
    assert_eq!(name_input.id(), "device_name");
    assert!(matches!(name_input.get_value(), ControlValue::String(_)));
}

#[test]
fn test_widget_emergency_stop() {
    let mut slider = PrecisionSlider::new("test", "Test", 0.0, 100.0, "")
        .with_emergency_value(25.0);
    
    // Set to some value
    slider.set_value(ControlValue::Float(75.0)).unwrap();
    assert_eq!(slider.get_value(), ControlValue::Float(75.0));
    
    // Emergency stop should reset to emergency value
    slider.emergency_stop();
    assert_eq!(slider.get_value(), ControlValue::Float(25.0));
    assert!(slider.has_changed());
}

#[test]
fn test_widget_validation_and_clamping() {
    let mut slider = PrecisionSlider::new("test", "Test", 0.0, 10.0, "");
    
    // Set invalid value manually (bypassing normal validation)
    slider.value = 15.0; // This would normally be prevented
    
    // Validate and clamp should fix it
    assert!(slider.validate_and_clamp());
    assert_eq!(slider.get_value(), ControlValue::Float(10.0));
    
    // Set NaN value
    slider.value = f64::NAN;
    assert!(slider.validate_and_clamp());
    assert_eq!(slider.get_value(), ControlValue::Float(5.0)); // Should be reset to middle value
}

#[cfg(feature = "hardware-tests")]
mod hardware_tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_performance_requirements() {
        let mut manager = ManualControlManager::new();
        
        // Add many widgets to test performance
        for i in 0..100 {
            let widget = Box::new(
                PrecisionSlider::new(&format!("slider_{}", i), &format!("Slider {}", i), 0.0, 100.0, "")
            );
            manager.add_widget(widget);
        }
        
        // Test update rate performance (should be <16ms for 60 FPS)
        let start = std::time::Instant::now();
        for _ in 0..60 {
            // Simulate 60 update cycles
            manager.validate_all_widgets();
            std::thread::sleep(Duration::from_millis(1));
        }
        let elapsed = start.elapsed();
        
        // Should complete 60 cycles in well under 1 second (16ms * 60 = 960ms)
        assert!(elapsed < Duration::from_millis(800), "Performance requirement not met: {:?}", elapsed);
    }
    
    #[test]
    fn test_emergency_stop_response_time() {
        let mut manager = ManualControlManager::new();
        
        // Test emergency stop response time (should be <1ms)
        let start = std::time::Instant::now();
        manager.emergency_stop_all();
        let elapsed = start.elapsed();
        
        assert!(elapsed < Duration::from_millis(1), "Emergency stop too slow: {:?}", elapsed);
        assert!(manager.is_emergency_stop_engaged());
    }
}