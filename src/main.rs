mod device;
mod transport;
mod drivers;
mod telemetry;
mod ui;
mod performance;
mod logging;
mod profile;

use std::sync::Arc;
use tracing_subscriber;
use device::DeviceManager;
use performance::{MonitorConfig, PerformanceMonitor};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create performance monitor with startup tracking
    let mut monitor_config = MonitorConfig::default();
    monitor_config.budget.max_startup_ms = 2000; // 2 second budget
    let performance_monitor = Arc::new(PerformanceMonitor::new(monitor_config));
    
    // Begin startup phase tracking
    performance_monitor.begin_startup_phase("initialization", "Setting up tracing and basic systems").await;
    
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("multi_controller_app=debug")
        .init();
    
    tracing::info!("Multi-Controller App starting...");
    
    // Phase 2: Device Manager Setup
    performance_monitor.end_startup_phase().await;
    performance_monitor.begin_startup_phase("device_manager", "Creating and initializing device manager").await;
    
    // Create device manager
    let device_manager = Arc::new(DeviceManager::new("./drivers"));
    
    // Initialize device manager
    device_manager.initialize().await?;
    
    tracing::info!("Device manager initialized successfully");
    
    // Phase 3: UI Setup
    performance_monitor.end_startup_phase().await;
    performance_monitor.begin_startup_phase("ui_setup", "Configuring GUI application and viewport").await;
    
    // Launch the GUI application
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_decorations(true)
            .with_transparent(false)
            .with_resizable(true),
        ..Default::default()
    };
    
    let device_manager_clone = device_manager.clone();
    
    // Phase 4: App Launch
    performance_monitor.end_startup_phase().await;
    performance_monitor.begin_startup_phase("app_launch", "Creating application instance and starting GUI").await;
    
    eframe::run_native(
        "Multi-Controller App",
        native_options,
        Box::new(move |_cc| {
            Ok(Box::new(ui::MultiControllerApp::new(device_manager_clone)))
        }),
    ).map_err(|e| anyhow::anyhow!("Failed to launch GUI: {}", e))?;
    
    // Final phase complete
    performance_monitor.end_startup_phase().await;
    
    // Validate startup performance
    let startup_valid = performance_monitor.validate_startup_performance().await;
    if !startup_valid {
        tracing::warn!("Startup performance exceeded budget!");
    }
    
    // Generate and log startup report
    let startup_report = performance_monitor.get_startup_report().await;
    tracing::info!("Startup completed in {:?} with {} phases", 
                  startup_report.total_duration, 
                  startup_report.phase_count);
    
    if let Some(slowest) = startup_report.slowest_phase {
        tracing::info!("Slowest phase: {} ({:?})", slowest, startup_report.slowest_duration);
    }
    
    tracing::info!("Shutting down...");
    
    // Trigger emergency stop on shutdown
    device_manager.emergency_stop("Application shutdown".to_string()).await;
    
    Ok(())
}
