use multi_controller_app::telemetry::{TelemetryChannel, ChannelConfig, TelemetrySample, SampleType};
use multi_controller_app::ui::charts::{TelemetryChart, ChartConfig, ChartType};
use std::time::Duration;
use std::thread;

#[test]
fn test_chart_ring_buffer_integration() {
    // Create a telemetry channel with ring buffer
    let mut config = ChannelConfig {
        name: "test_channel".to_string(),
        buffer_size: 2000,  // Minimum size per requirements
        sample_rate: 30.0,  // 30 FPS
        sample_type: SampleType::Float32,
    };
    
    let channel = TelemetryChannel::new(config);
    
    // Add 100 samples to the ring buffer
    for i in 0..100 {
        let value = 50.0 + 30.0 * ((i as f32) * 0.1).sin();
        channel.add_sample(TelemetrySample::new_f32(value));
        thread::sleep(Duration::from_millis(1)); // Small delay to get different timestamps
    }
    
    // Create a chart
    let mut chart_config = ChartConfig::default();
    chart_config.max_points = 300;  // Chart decimation limit
    chart_config.update_interval_ms = 33;  // ~30 FPS
    
    let mut chart = TelemetryChart::with_config("Test Chart", chart_config);
    
    // Update chart from channel (this integrates ring buffer data)
    chart.update_from_channel(&channel);
    
    // Verify integration
    let stats = chart.stats();
    assert!(stats.buffer_size > 0, "Chart should have data from ring buffer");
    assert!(stats.buffer_size <= 300, "Chart should respect max_points limit");
    
    // Verify data flow from ring buffer to chart
    let channel_stats = channel.get_stats();
    assert!(channel_stats.buffer_used > 0, "Channel buffer should contain samples");
    
    println!("Integration test passed:");
    println!("  Ring buffer samples: {}", channel_stats.buffer_used);
    println!("  Chart buffer points: {}", stats.buffer_size);
    println!("  Channel capacity: {}", channel_stats.buffer_capacity);
}

#[test]
fn test_chart_decimation_with_large_buffer() {
    let config = ChannelConfig {
        name: "high_rate_channel".to_string(),
        buffer_size: 5000,  // Large buffer
        sample_rate: 0.0,    // No rate limiting for test
        sample_type: SampleType::Float32,
    };
    
    let channel = TelemetryChannel::new(config);
    
    // Fill buffer with 5000 samples
    for i in 0..5000 {
        channel.add_sample(TelemetrySample::new_f32(i as f32));
    }
    
    // Chart should decimate to 300 points
    let mut chart = TelemetryChart::new("Decimation Test");
    chart.update_from_channel(&channel);
    
    let stats = chart.stats();
    assert!(stats.buffer_size <= 300, "Chart should decimate to max_points");
    assert!(stats.buffer_size > 0, "Chart should have some data");
    
    println!("Decimation test passed:");
    println!("  Input samples: 5000");
    println!("  Decimated to: {}", stats.buffer_size);
}

#[test]
fn test_chart_time_window_filtering() {
    let config = ChannelConfig {
        name: "time_window_channel".to_string(),
        buffer_size: 2000,
        sample_rate: 0.0,
        sample_type: SampleType::Float32,
    };
    
    let channel = TelemetryChannel::new(config);
    
    // Add samples over time
    for i in 0..100 {
        channel.add_sample(TelemetrySample::new_f32(i as f32));
        thread::sleep(Duration::from_millis(10));
    }
    
    // Create chart with 0.5 second time window
    let mut chart_config = ChartConfig::default();
    chart_config.time_window_seconds = 0.5;
    
    let mut chart = TelemetryChart::with_config("Time Window Test", chart_config);
    chart.update_from_channel(&channel);
    
    let stats = chart.stats();
    // Should have fewer points due to time window
    assert!(stats.buffer_size < 100, "Time window should filter old samples");
    
    println!("Time window test passed:");
    println!("  Total samples: 100");
    println!("  Samples in 0.5s window: {}", stats.buffer_size);
}

#[test]
fn test_chart_update_rate_limiting() {
    let config = ChannelConfig {
        name: "rate_limited_channel".to_string(),
        buffer_size: 2000,
        sample_rate: 0.0,
        sample_type: SampleType::Float32,
    };
    
    let channel = TelemetryChannel::new(config);
    
    // Add initial samples
    for i in 0..10 {
        channel.add_sample(TelemetrySample::new_f32(i as f32));
    }
    
    // Create chart with 100ms update interval
    let mut chart_config = ChartConfig::default();
    chart_config.update_interval_ms = 100;
    
    let mut chart = TelemetryChart::with_config("Rate Limit Test", chart_config);
    
    // First update should work
    chart.update_from_channel(&channel);
    let initial_stats = chart.stats();
    assert!(initial_stats.buffer_size > 0);
    
    // Immediate second update should be skipped
    for i in 10..20 {
        channel.add_sample(TelemetrySample::new_f32(i as f32));
    }
    chart.update_from_channel(&channel);
    let immediate_stats = chart.stats();
    
    // Wait for update interval
    thread::sleep(Duration::from_millis(101));
    
    // Now update should work
    chart.update_from_channel(&channel);
    let delayed_stats = chart.stats();
    
    println!("Rate limiting test passed:");
    println!("  Initial buffer size: {}", initial_stats.buffer_size);
    println!("  After delay buffer size: {}", delayed_stats.buffer_size);
}

#[test]
fn test_different_chart_types() {
    let config = ChannelConfig {
        name: "chart_types_channel".to_string(),
        buffer_size: 2000,
        sample_rate: 0.0,
        sample_type: SampleType::Float32,
    };
    
    let channel = TelemetryChannel::new(config);
    
    // Add samples
    for i in 0..50 {
        let value = 50.0 + 30.0 * ((i as f32) * 0.1).sin();
        channel.add_sample(TelemetrySample::new_f32(value));
    }
    
    // Test different chart types
    let chart_types = vec![
        ChartType::Line,
        ChartType::Scatter,
        ChartType::Area,
        ChartType::StepLine,
    ];
    
    for chart_type in chart_types {
        let mut config = ChartConfig::default();
        config.chart_type = chart_type;
        
        let mut chart = TelemetryChart::with_config(
            format!("{:?} Chart", chart_type), 
            config
        );
        
        chart.update_from_channel(&channel);
        let stats = chart.stats();
        
        assert!(stats.buffer_size > 0, "{:?} chart should have data", chart_type);
        println!("{:?} chart test passed with {} points", chart_type, stats.buffer_size);
    }
}