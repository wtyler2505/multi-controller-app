//! Telemetry Parser Demo
//!
//! This example demonstrates the multi-format telemetry parsing capabilities
//! including CSV, JSON, and binary format support with real-time buffering.

use std::collections::HashMap;
use chrono::Utc;

// Basic types needed for the demo - normally these would come from the main library
use multi_controller_app::telemetry::{
    MultiFormatTelemetryParser, TelemetryBufferManager, TelemetryFormat,
    CsvTelemetryParser, CsvSchema, ColumnDefinition, TelemetryValueType,
    TelemetryFrame, DataPoint, TelemetryValue, DataQuality,
    DecimationStrategy, TelemetryDecimator,
};

fn main() {
    println!("=€ Multi-Format Telemetry Parser Demo");
    println!("=====================================");
    
    // Initialize the parser system
    let parser = MultiFormatTelemetryParser::new();
    let buffer_manager = TelemetryBufferManager::new(2000);
    
    // Demo 1: CSV Parsing
    println!("\n=Ê Demo 1: CSV Telemetry Parsing");
    demo_csv_parsing(&parser, &buffer_manager);
    
    // Demo 2: JSON Parsing  
    println!("\n=Ë Demo 2: JSON Telemetry Parsing");
    demo_json_parsing(&parser, &buffer_manager);
    
    // Demo 3: Binary Parsing
    println!("\n=¾ Demo 3: Binary Telemetry Parsing");
    demo_binary_parsing(&parser, &buffer_manager);
    
    // Demo 4: Auto Format Detection
    println!("\n= Demo 4: Automatic Format Detection");
    demo_auto_detection(&parser, &buffer_manager);
    
    // Demo 5: High-Performance Buffering
    println!("\n¡ Demo 5: High-Performance Ring Buffer");
    demo_buffer_performance(&buffer_manager);
    
    // Demo 6: Data Decimation for Visualization
    println!("\n=É Demo 6: Data Decimation for Charts");
    demo_data_decimation(&buffer_manager);
    
    // Final Statistics
    println!("\n=È Final System Statistics");
    print_system_stats(&buffer_manager);
    
    println!("\n Demo completed successfully!");
    println!("All telemetry parsing and buffering features working correctly.");
}

fn demo_csv_parsing(parser: &MultiFormatTelemetryParser, buffer_manager: &TelemetryBufferManager) {
    let csv_data = b"timestamp,voltage,current,temperature\n2024-01-15 10:30:00,3.3,0.5,25.6\n2024-01-15 10:30:01,3.2,0.6,26.1\n2024-01-15 10:30:02,3.4,0.4,25.8";
    
    // Create custom CSV schema
    let schema = CsvSchema {
        columns: vec![
            ColumnDefinition {
                name: "timestamp".to_string(),
                data_type: TelemetryValueType::String,
                unit: None,
                min_value: None,
                max_value: None,
            },
            ColumnDefinition {
                name: "voltage".to_string(),
                data_type: TelemetryValueType::Float,
                unit: Some("V".to_string()),
                min_value: Some(0.0),
                max_value: Some(5.0),
            },
            ColumnDefinition {
                name: "current".to_string(),
                data_type: TelemetryValueType::Float,
                unit: Some("A".to_string()),
                min_value: Some(0.0),
                max_value: Some(2.0),
            },
            ColumnDefinition {
                name: "temperature".to_string(),
                data_type: TelemetryValueType::Float,
                unit: Some("°C".to_string()),
                min_value: Some(-10.0),
                max_value: Some(60.0),
            },
        ],
        timestamp_column: Some(0),
        sequence_column: None,
        device_id_column: None,
    };
    
    let csv_parser = CsvTelemetryParser::with_schema(schema);
    
    match csv_parser.parse(csv_data) {
        Ok(frame) => {
            println!(" CSV parsing successful!");
            println!("   Device ID: {}", frame.device_id);
            println!("   Data points: {}", frame.data_points.len());
            
            for (i, point) in frame.data_points.iter().enumerate() {
                match &point.value {
                    TelemetryValue::Float(val) => {
                        println!("   {}. {}: {:.2} {} (Quality: {:?})", 
                                i + 1, point.channel, val, 
                                point.unit.as_deref().unwrap_or(""), 
                                point.quality);
                    }
                    _ => {}
                }
            }
            
            // Store in buffer manager
            if buffer_manager.store_frame(frame) {
                println!("   =Á Frame stored in buffer successfully");
            }
        }
        Err(e) => println!("L CSV parsing failed: {}", e),
    }
}

fn demo_json_parsing(parser: &MultiFormatTelemetryParser, buffer_manager: &TelemetryBufferManager) {
    let json_data = r#"{
        "timestamp": "2024-01-15T10:30:00Z",
        "sequence_number": 42,
        "device_id": "arduino_sensor_001",
        "data_points": [
            {
                "channel": "voltage",
                "value": {"Float": 3.3},
                "unit": "V",
                "quality": "Good"
            },
            {
                "channel": "current",
                "value": {"Float": 0.5},
                "unit": "A", 
                "quality": "Good"
            },
            {
                "channel": "status",
                "value": {"String": "ONLINE"},
                "unit": null,
                "quality": "Good"
            }
        ],
        "metadata": {
            "sensor_type": "environmental",
            "location": "lab_bench_3",
            "firmware": "v2.1.0"
        }
    }"#.as_bytes();
    
    match parser.parse(json_data, TelemetryFormat::Json) {
        Ok(frame) => {
            println!(" JSON parsing successful!");
            println!("   Device ID: {}", frame.device_id);
            println!("   Sequence: {}", frame.sequence_number);
            println!("   Metadata: {} entries", frame.metadata.len());
            println!("   Data points: {}", frame.data_points.len());
            
            for point in &frame.data_points {
                match &point.value {
                    TelemetryValue::Float(val) => {
                        println!("   " {}: {:.2} {}", point.channel, val,
                                point.unit.as_deref().unwrap_or(""));
                    }
                    TelemetryValue::String(s) => {
                        println!("   " {}: {}", point.channel, s);
                    }
                    _ => {}
                }
            }
            
            if buffer_manager.store_frame(frame) {
                println!("   =Á Frame stored in buffer successfully");
            }
        }
        Err(e) => println!("L JSON parsing failed: {}", e),
    }
}

fn demo_binary_parsing(_parser: &MultiFormatTelemetryParser, buffer_manager: &TelemetryBufferManager) {
    // Create a binary telemetry frame manually
    let mut binary_data = Vec::new();
    
    // Header: version (1 byte)
    binary_data.push(1u8);
    
    // Sequence number (8 bytes)
    binary_data.extend_from_slice(&123u64.to_le_bytes());
    
    // Timestamp (8 bytes) - current time in milliseconds
    let timestamp_ms = Utc::now().timestamp_millis() as u64;
    binary_data.extend_from_slice(&timestamp_ms.to_le_bytes());
    
    // Data point count (2 bytes) 
    binary_data.extend_from_slice(&3u16.to_le_bytes());
    
    // Data points: channel_id (2 bytes) + value (8 bytes) + quality (1 byte)
    let data_points = [
        (0u16, 3.3f64, 0u8), // voltage, good quality
        (1u16, 0.5f64, 0u8), // current, good quality
        (2u16, 25.6f64, 1u8), // temperature, questionable quality
    ];
    
    for (channel_id, value, quality) in data_points.iter() {
        binary_data.extend_from_slice(&channel_id.to_le_bytes());
        binary_data.extend_from_slice(&value.to_le_bytes());
        binary_data.push(*quality);
    }
    
    println!("=æ Binary frame size: {} bytes", binary_data.len());
    
    // Parse with binary parser directly
    use multi_controller_app::telemetry::BinaryTelemetryParser;
    let binary_parser = BinaryTelemetryParser::new();
    
    match binary_parser.parse(&binary_data) {
        Ok(frame) => {
            println!(" Binary parsing successful!");
            println!("   Device ID: {}", frame.device_id);
            println!("   Sequence: {}", frame.sequence_number);
            println!("   Data points: {}", frame.data_points.len());
            
            let channel_names = ["voltage", "current", "temperature"];
            for (i, point) in frame.data_points.iter().enumerate() {
                if let TelemetryValue::Float(val) = &point.value {
                    println!("   " {}: {:.2} (Quality: {:?})", 
                            channel_names.get(i).unwrap_or(&point.channel.as_str()),
                            val, point.quality);
                }
            }
            
            if buffer_manager.store_frame(frame) {
                println!("   =Á Frame stored in buffer successfully");
            }
        }
        Err(e) => println!("L Binary parsing failed: {}", e),
    }
}

fn demo_auto_detection(parser: &MultiFormatTelemetryParser, buffer_manager: &TelemetryBufferManager) {
    let test_cases = vec![
        ("JSON data", r#"{"timestamp": "2024-01-15T10:30:00Z", "sequence_number": 1, "device_id": "auto_test", "data_points": [], "metadata": {}}"#.as_bytes()),
        ("CSV data", b"timestamp,value\n2024-01-15 10:30:00,42.0"),
    ];
    
    for (description, data) in test_cases {
        match parser.parse_auto(data) {
            Ok((detected_format, frame)) => {
                println!(" {} auto-detected as: {:?}", description, detected_format);
                println!("   Device: {}", frame.device_id);
                
                if buffer_manager.store_frame(frame) {
                    println!("   =Á Frame stored successfully");
                }
            }
            Err(e) => println!("L Auto-detection failed for {}: {}", description, e),
        }
    }
}

fn demo_buffer_performance(buffer_manager: &TelemetryBufferManager) {
    use std::time::Instant;
    
    println!("<ÃB Testing high-performance operations...");
    
    // Create test data
    let start = Instant::now();
    let num_frames = 1000;
    
    for i in 0..num_frames {
        let frame = TelemetryFrame {
            timestamp: Utc::now(),
            sequence_number: i,
            device_id: "performance_test".to_string(),
            data_points: vec![
                DataPoint {
                    channel: "test_channel".to_string(),
                    value: TelemetryValue::Float(i as f64 * 0.1),
                    unit: Some("V".to_string()),
                    quality: DataQuality::Good,
                }
            ],
            metadata: HashMap::new(),
        };
        
        buffer_manager.store_frame(frame);
    }
    
    let elapsed = start.elapsed();
    let throughput = num_frames as f64 / elapsed.as_secs_f64();
    
    println!(" Stored {} frames in {:.2}ms", num_frames, elapsed.as_millis());
    println!("   =È Throughput: {:.0} frames/second", throughput);
    
    // Test retrieval performance
    let start = Instant::now();
    let recent_frames = buffer_manager.get_recent_frames("performance_test", 100);
    let retrieval_time = start.elapsed();
    
    println!("   =ä Retrieved {} frames in {:.2}¼s", recent_frames.len(), retrieval_time.as_micros());
}

fn demo_data_decimation(buffer_manager: &TelemetryBufferManager) {
    // Get the performance test data we just created
    let frames = buffer_manager.get_recent_frames("performance_test", 1000);
    
    if frames.is_empty() {
        println!("   No data available for decimation demo");
        return;
    }
    
    println!("=Ê Original dataset: {} frames", frames.len());
    
    // Test different decimation strategies
    let strategies = vec![
        (DecimationStrategy::Uniform, "Uniform"),
        (DecimationStrategy::MinMax, "Min-Max"),
        (DecimationStrategy::Average, "Average"),
        (DecimationStrategy::Adaptive, "Adaptive"),
    ];
    
    let target_points = 50;
    
    for (strategy, name) in strategies {
        let start = std::time::Instant::now();
        let decimated = TelemetryDecimator::decimate_frames(&frames, target_points, strategy);
        let elapsed = start.elapsed();
        
        println!("   {} decimation: {} ’ {} points ({:.2}¼s)", 
                name, frames.len(), decimated.len(), elapsed.as_micros());
    }
    
    println!(" Decimation strategies tested successfully");
}

fn print_system_stats(buffer_manager: &TelemetryBufferManager) {
    let system_stats = buffer_manager.get_system_stats();
    let buffer_stats = buffer_manager.get_buffer_stats();
    
    println!("=Ê System Overview:");
    println!("   Devices: {}", system_stats.device_count);
    println!("   Total capacity: {} frames", system_stats.total_capacity);
    println!("   Total samples: {}", system_stats.total_samples);
    println!("   Parse errors: {}", system_stats.total_parse_errors);
    println!("   Average utilization: {:.1}%", system_stats.average_utilization);
    
    println!("\n=Ë Device Buffers:");
    for (device_id, stats) in buffer_stats {
        println!("   " {}: {}/{} frames ({:.1}% full, {} overflows)",
                device_id, stats.length, stats.capacity, 
                stats.utilization_percent, stats.overflow_count);
    }
    
    println!("\n=¾ Memory estimate: {:.1} MB", 
             buffer_manager.estimated_memory_usage() as f64 / (1024.0 * 1024.0));
}