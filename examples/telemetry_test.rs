//! Telemetry chart test example
//! 
//! This example demonstrates real-time telemetry charting at 30 FPS
//! with simulated data streams.

use multi_controller_app::ui::app::MultiControllerApp;
use multi_controller_app::device::DeviceManager;
use multi_controller_app::telemetry::{TelemetrySystem, TelemetryConfig, TelemetrySample};
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("multi_controller_app=debug".parse()?)
            .add_directive("telemetry_test=info".parse()?))
        .init();

    // Start the UI
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Multi-Controller App - Telemetry Test"),
        ..Default::default()
    };

    // Create device manager
    let device_manager = Arc::new(DeviceManager::new("plugins"));
    
    // Create runtime for spawning test data generator
    let rt = tokio::runtime::Runtime::new()?;
    
    // Clone for the data generator task
    let device_manager_clone = device_manager.clone();
    
    // Spawn test data generator
    rt.spawn(async move {
        generate_test_telemetry_data().await;
    });

    // Run the app
    eframe::run_native(
        "Multi-Controller App - Telemetry Test",
        native_options,
        Box::new(|_cc| {
            Ok(Box::new(MultiControllerApp::new(device_manager)))
        }),
    )?;

    Ok(())
}

/// Generate test telemetry data at 30 Hz
async fn generate_test_telemetry_data() {
    tracing::info!("Starting test telemetry data generator");
    
    // Create telemetry system
    let config = TelemetryConfig {
        default_buffer_size: 2000,
        max_memory_bytes: 50 * 1024 * 1024,
        auto_memory_management: true,
        default_sample_rate: 30.0,
    };
    let telemetry_system = Arc::new(TelemetrySystem::with_config(config));
    
    // Create test channels
    let sine_channel = telemetry_system.create_channel(
        "sine_wave".to_string(),
        None
    );
    
    let square_channel = telemetry_system.create_channel(
        "square_wave".to_string(),
        None
    );
    
    let random_channel = telemetry_system.create_channel(
        "random_data".to_string(),
        None
    );
    
    let mut tick = 0u64;
    let mut interval = time::interval(Duration::from_millis(33)); // ~30 Hz
    
    loop {
        interval.tick().await;
        
        let time = tick as f32 * 0.033; // Time in seconds
        
        // Generate sine wave (50 + 30 * sin(t))
        let sine_value = 50.0 + 30.0 * (time * 2.0 * std::f32::consts::PI * 0.5).sin();
        sine_channel.add_sample(TelemetrySample::new_f32(sine_value));
        
        // Generate square wave
        let square_value = if (tick / 30) % 2 == 0 { 80.0 } else { 20.0 };
        square_channel.add_sample(TelemetrySample::new_f32(square_value));
        
        // Generate random data
        let random_value = 50.0 + (rand::random::<f32>() - 0.5) * 40.0;
        random_channel.add_sample(TelemetrySample::new_f32(random_value));
        
        tick += 1;
        
        // Log every second
        if tick % 30 == 0 {
            let stats = telemetry_system.get_stats();
            tracing::debug!(
                "Telemetry stats: {} channels, {:.2} MB memory", 
                stats.channel_count,
                stats.total_memory_bytes as f64 / (1024.0 * 1024.0)
            );
        }
    }
}