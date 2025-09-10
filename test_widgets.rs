//! Quick test of widget creation
use multi_controller_app::ui::controls::{PrecisionSlider, ControlWidget};

fn main() {
    println!("Testing widget creation...");
    
    // Test creating a precision slider
    let mut slider = PrecisionSlider::new("test", "Test Slider", 0.0, 100.0, "%");
    
    println!("Created slider with ID: {}", slider.id());
    println!("Initial value: {:?}", slider.get_value());
    
    // Test setting a value
    let new_value = multi_controller_app::ui::controls::ControlValue::Float(50.0);
    if let Ok(_) = slider.set_value(new_value) {
        println!("Set value to 50.0: {:?}", slider.get_value());
    }
    
    println!("Widget creation test passed!");
}
