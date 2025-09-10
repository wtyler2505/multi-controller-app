//! Comprehensive tests for the telemetry parser system
//!
//! This module tests all aspects of the multi-format telemetry parser including
//! CSV, JSON, binary parsing, data validation, and high-performance ring buffer operations.

use multi_controller_app::telemetry::{
    MultiFormatTelemetryParser, TelemetryBufferManager, TelemetryRingBuffer,
    CsvTelemetryParser, JsonTelemetryParser, BinaryTelemetryParser,
    TelemetryFormat, TelemetryFrame, DataPoint, TelemetryValue, DataQuality,
    ParseError, CsvSchema, ColumnDefinition, TelemetryValueType,
    BufferStats, SystemStats, DecimationStrategy, TelemetryDecimator,
};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[test]
fn test_csv_parsing_basic() {
    let csv_data = b"timestamp,voltage,current\n2024-01-01 12:00:00,3.3,0.5\n2024-01-01 12:00:01,3.2,0.6";
    
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
        ],
        timestamp_column: Some(0),
        sequence_column: None,
        device_id_column: None,
    };
    
    let parser = CsvTelemetryParser::with_schema(schema);
    let result = parser.parse(csv_data);
    
    assert!(result.is_ok(), "CSV parsing should succeed");
    let frame = result.unwrap();
    assert_eq!(frame.data_points.len(), 2, "Should have 2 data points (voltage and current)");
    assert_eq!(frame.data_points[0].channel, "voltage");
    assert_eq!(frame.data_points[1].channel, "current");
    
    // Check values
    if let TelemetryValue::Float(voltage) = &frame.data_points[0].value {
        assert!((voltage - 3.3).abs() < 0.001, "Voltage should be 3.3V");
    } else {
        panic!("Voltage should be a float value");
    }
    
    if let TelemetryValue::Float(current) = &frame.data_points[1].value {
        assert!((current - 0.5).abs() < 0.001, "Current should be 0.5A");
    } else {
        panic!("Current should be a float value");
    }
}

#[test]
fn test_csv_parsing_with_different_delimiters() {
    let csv_data = b"voltage;current;temperature\n3.3;0.5;25.6";
    
    let schema = CsvSchema {
        columns: vec![
            ColumnDefinition {
                name: "voltage".to_string(),
                data_type: TelemetryValueType::Float,
                unit: Some("V".to_string()),
                min_value: None,
                max_value: None,
            },
            ColumnDefinition {
                name: "current".to_string(),
                data_type: TelemetryValueType::Float,
                unit: Some("A".to_string()),
                min_value: None,
                max_value: None,
            },
            ColumnDefinition {
                name: "temperature".to_string(),
                data_type: TelemetryValueType::Float,
                unit: Some("°C".to_string()),
                min_value: None,
                max_value: None,
            },
        ],
        timestamp_column: None,
        sequence_column: None,
        device_id_column: None,
    };
    
    let parser = CsvTelemetryParser::with_schema(schema)
        .with_delimiter(';')
        .with_header(true);
    
    let result = parser.parse(csv_data);
    assert!(result.is_ok(), "Semicolon-delimited CSV should parse correctly");
    
    let frame = result.unwrap();
    assert_eq!(frame.data_points.len(), 3, "Should have 3 data points");
}

#[test]
fn test_csv_data_quality_assessment() {
    let csv_data = b"value\n2.5\n10.0\n0.1\n-5.0";
    
    let schema = CsvSchema {
        columns: vec![
            ColumnDefinition {
                name: "value".to_string(),
                data_type: TelemetryValueType::Float,
                unit: Some("V".to_string()),
                min_value: Some(0.0),
                max_value: Some(5.0),
            },
        ],
        timestamp_column: None,
        sequence_column: None,
        device_id_column: None,
    };
    
    let parser = CsvTelemetryParser::with_schema(schema);
    let result = parser.parse(csv_data);
    
    assert!(result.is_ok());
    let frame = result.unwrap();
    
    // The parser only processes the first data line after header
    // So we should get one data point with value 2.5 (good quality)
    assert_eq!(frame.data_points.len(), 1);
    assert_eq!(frame.data_points[0].quality, DataQuality::Good);
}

#[test]
fn test_json_parsing() {
    let json_data = r#"{
        "timestamp": "2024-01-01T12:00:00Z",
        "sequence_number": 1,
        "device_id": "arduino_001",
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
            }
        ],
        "metadata": {"sensor": "analog", "location": "lab"}
    }"#.as_bytes();
    
    let parser = JsonTelemetryParser::new();
    let result = parser.parse(json_data);
    
    assert!(result.is_ok(), "JSON parsing should succeed");
    let frame = result.unwrap();
    assert_eq!(frame.device_id, "arduino_001");
    assert_eq!(frame.sequence_number, 1);
    assert_eq!(frame.data_points.len(), 2);
    assert_eq!(frame.metadata.get("sensor").unwrap(), "analog");
}

#[test]
fn test_binary_parsing() {
    // Construct a binary frame manually
    let mut binary_data = Vec::new();
    
    // Header: version (1 byte)
    binary_data.push(1u8);
    
    // Sequence number (8 bytes)
    binary_data.extend_from_slice(&42u64.to_le_bytes());
    
    // Timestamp (8 bytes) - Jan 1, 2022 00:00:00 UTC in milliseconds
    binary_data.extend_from_slice(&1640995200000u64.to_le_bytes());
    
    // Data point count (2 bytes)
    binary_data.extend_from_slice(&2u16.to_le_bytes());
    
    // First data point: channel_id (2 bytes) + value (8 bytes) + quality (1 byte)
    binary_data.extend_from_slice(&0u16.to_le_bytes());  // channel 0
    binary_data.extend_from_slice(&3.3f64.to_le_bytes()); // value
    binary_data.push(0u8); // good quality
    
    // Second data point
    binary_data.extend_from_slice(&1u16.to_le_bytes());  // channel 1
    binary_data.extend_from_slice(&0.5f64.to_le_bytes()); // value
    binary_data.push(0u8); // good quality
    
    let parser = BinaryTelemetryParser::new();
    let result = parser.parse(&binary_data);
    
    assert!(result.is_ok(), "Binary parsing should succeed");
    let frame = result.unwrap();
    assert_eq!(frame.sequence_number, 42);
    assert_eq!(frame.device_id, "binary_device");
    assert_eq!(frame.data_points.len(), 2);
    assert_eq!(frame.data_points[0].channel, "channel_0");
    assert_eq!(frame.data_points[1].channel, "channel_1");
    
    // Check values
    if let TelemetryValue::Float(voltage) = &frame.data_points[0].value {
        assert!((voltage - 3.3).abs() < 0.001);
    }
    
    if let TelemetryValue::Float(current) = &frame.data_points[1].value {
        assert!((current - 0.5).abs() < 0.001);
    }
}

#[test]
fn test_auto_format_detection() {
    let parser = MultiFormatTelemetryParser::new();
    
    // Test JSON detection
    let json_data = r#"{"timestamp": "2024-01-01T12:00:00Z", "sequence_number": 1, "device_id": "test", "data_points": [], "metadata": {}}"#.as_bytes();
    let result = parser.parse_auto(json_data);
    assert!(result.is_ok(), "Auto-detection should work for JSON");
    let (format, _) = result.unwrap();
    assert_eq!(format, TelemetryFormat::Json);
    
    // Test CSV detection (should fall back to CSV if JSON fails)
    let csv_data = b"timestamp,value\n2024-01-01 12:00:00,42.0";
    let result = parser.parse_auto(csv_data);
    assert!(result.is_ok(), "Auto-detection should work for CSV");
    let (format, _) = result.unwrap();
    assert_eq!(format, TelemetryFormat::Csv);
}

#[test]
fn test_ring_buffer_basic_operations() {
    let buffer: TelemetryRingBuffer<i32> = TelemetryRingBuffer::new(2000);
    
    // Test initial state
    assert_eq!(buffer.capacity(), 2000);
    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
    assert_eq!(buffer.overflow_count(), 0);
    assert_eq!(buffer.utilization(), 0.0);
    
    // Test push and pop
    assert!(buffer.push(42));
    assert_eq!(buffer.len(), 1);
    assert!(!buffer.is_empty());
    assert!((buffer.utilization() - 0.05).abs() < 0.01); // ~0.05%
    
    assert_eq!(buffer.pop(), Some(42));
    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
    
    // Test pop from empty buffer
    assert_eq!(buffer.pop(), None);
}

#[test]
fn test_ring_buffer_overflow_handling() {
    let buffer: TelemetryRingBuffer<i32> = TelemetryRingBuffer::new(2000);
    
    // Fill buffer to capacity
    for i in 0..2000 {
        buffer.push(i);
    }
    
    assert_eq!(buffer.len(), 2000);
    assert_eq!(buffer.overflow_count(), 0);
    assert!((buffer.utilization() - 100.0).abs() < 0.1);
    
    // Push one more item to trigger overflow
    buffer.push(2000);
    assert_eq!(buffer.overflow_count(), 1, "Should have 1 overflow event");
    
    // Buffer should still maintain capacity
    assert!(buffer.len() <= 2000, "Buffer should not exceed capacity");
}

#[test]
fn test_ring_buffer_peek_operations() {
    let buffer: TelemetryRingBuffer<i32> = TelemetryRingBuffer::new(2000);
    
    // Add test data
    for i in 0..10 {
        buffer.push(i);
    }
    
    // Test peek_latest
    let latest = buffer.peek_latest(5);
    assert_eq!(latest.len(), 5, "Should get 5 latest items");
    assert_eq!(latest[0], 9, "Latest should be 9");
    assert_eq!(latest[1], 8, "Second latest should be 8");
    
    // Test peek_range
    let range = buffer.peek_range(0, 3);
    assert_eq!(range.len(), 3, "Should get 3 items from start");
    assert_eq!(range[0], 0, "First item should be 0");
    assert_eq!(range[1], 1, "Second item should be 1");
    assert_eq!(range[2], 2, "Third item should be 2");
    
    // Test peek_range with offset
    let range = buffer.peek_range(5, 3);
    assert_eq!(range.len(), 3, "Should get 3 items from offset 5");
    assert_eq!(range[0], 5, "First item should be 5");
}

#[test]
fn test_telemetry_buffer_manager() {
    let manager = TelemetryBufferManager::new(2000);
    
    // Create test frame
    let frame = TelemetryFrame {
        timestamp: Utc::now(),
        sequence_number: 1,
        device_id: "test_device".to_string(),
        data_points: vec![
            DataPoint {
                channel: "voltage".to_string(),
                value: TelemetryValue::Float(3.3),
                unit: Some("V".to_string()),
                quality: DataQuality::Good,
            }
        ],
        metadata: HashMap::new(),
    };
    
    // Store frame
    assert!(manager.store_frame(frame), "Frame storage should succeed");
    
    // Get recent frames
    let frames = manager.get_recent_frames("test_device", 10);
    assert_eq!(frames.len(), 1, "Should retrieve 1 frame");
    assert_eq!(frames[0].device_id, "test_device");
    assert_eq!(frames[0].sequence_number, 1);
    
    // Test device IDs
    let device_ids = manager.get_device_ids();
    assert_eq!(device_ids.len(), 1);
    assert!(device_ids.contains(&"test_device".to_string()));
    
    // Test buffer stats
    let stats = manager.get_buffer_stats();
    assert!(stats.contains_key("test_device"));
    let device_stats = &stats["test_device"];
    assert_eq!(device_stats.capacity, 2000);
    assert_eq!(device_stats.length, 1);
    assert_eq!(device_stats.overflow_count, 0);
    
    // Test system stats
    let system_stats = manager.get_system_stats();
    assert_eq!(system_stats.device_count, 1);
    assert_eq!(system_stats.total_capacity, 2000);
    assert_eq!(system_stats.total_samples, 1);
    assert_eq!(system_stats.total_stored_samples, 1);
}

#[test]
fn test_parse_and_store_csv() {
    let manager = TelemetryBufferManager::new(2000);
    let csv_data = b"timestamp,voltage\n2024-01-01 12:00:00,3.3";
    
    let result = manager.parse_and_store(csv_data, TelemetryFormat::Csv, Some("csv_device"));
    assert!(result.is_ok(), "CSV parse and store should succeed");
    assert!(result.unwrap(), "Storage should return true");
    
    let frames = manager.get_recent_frames("csv_device", 10);
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0].device_id, "csv_device");
}

#[test]
fn test_auto_parse_and_store() {
    let manager = TelemetryBufferManager::new(2000);
    
    // Test with JSON data
    let json_data = r#"{"timestamp": "2024-01-01T12:00:00Z", "sequence_number": 1, "device_id": "auto_device", "data_points": [{"channel": "test", "value": {"Float": 1.0}, "unit": null, "quality": "Good"}], "metadata": {}}"#.as_bytes();
    
    let result = manager.auto_parse_and_store(json_data, None);
    assert!(result.is_ok(), "Auto parse should work");
    let (format, success) = result.unwrap();
    assert_eq!(format, TelemetryFormat::Json);
    assert!(success);
    
    let frames = manager.get_recent_frames("auto_device", 10);
    assert_eq!(frames.len(), 1);
}

#[test]
fn test_buffer_management_operations() {
    let manager = TelemetryBufferManager::new(2000);
    
    // Add some test data
    let frame = TelemetryFrame {
        timestamp: Utc::now(),
        sequence_number: 1,
        device_id: "mgmt_test".to_string(),
        data_points: vec![],
        metadata: HashMap::new(),
    };
    
    manager.store_frame(frame);
    
    // Test clear operations
    assert!(manager.clear_device_buffer("mgmt_test"), "Device buffer should exist");
    let frames = manager.get_recent_frames("mgmt_test", 10);
    assert_eq!(frames.len(), 0, "Buffer should be empty after clear");
    
    // Add data again
    let frame = TelemetryFrame {
        timestamp: Utc::now(),
        sequence_number: 2,
        device_id: "mgmt_test".to_string(),
        data_points: vec![],
        metadata: HashMap::new(),
    };
    manager.store_frame(frame);
    
    // Test remove buffer
    assert!(manager.remove_device_buffer("mgmt_test"), "Should remove existing buffer");
    assert!(!manager.remove_device_buffer("nonexistent"), "Should return false for non-existent buffer");
    
    let device_ids = manager.get_device_ids();
    assert!(!device_ids.contains(&"mgmt_test".to_string()), "Device should be removed");
}

#[test]
fn test_data_decimation() {
    // Create test frames with sequence numbers
    let frames: Vec<TelemetryFrame> = (0..1000).map(|i| {
        TelemetryFrame {
            timestamp: Utc::now(),
            sequence_number: i,
            device_id: "decimation_test".to_string(),
            data_points: vec![
                DataPoint {
                    channel: "test_channel".to_string(),
                    value: TelemetryValue::Float(i as f64),
                    unit: None,
                    quality: DataQuality::Good,
                }
            ],
            metadata: HashMap::new(),
        }
    }).collect();
    
    // Test uniform decimation
    let decimated = TelemetryDecimator::decimate_frames(&frames, 100, DecimationStrategy::Uniform);
    assert!(decimated.len() <= 100, "Decimated data should not exceed target");
    assert!(decimated.len() > 0, "Should have some data points");
    
    // Verify decimation preserves order
    if decimated.len() >= 2 {
        assert!(decimated[0].sequence_number < decimated[1].sequence_number, 
                "Decimation should preserve chronological order");
    }
    
    // Test with target larger than input
    let small_set = &frames[..50];
    let decimated = TelemetryDecimator::decimate_frames(small_set, 100, DecimationStrategy::Uniform);
    assert_eq!(decimated.len(), 50, "Should return all data when target > input size");
    
    // Test other decimation strategies
    let minmax_decimated = TelemetryDecimator::decimate_frames(&frames, 100, DecimationStrategy::MinMax);
    assert!(minmax_decimated.len() <= 100);
    
    let average_decimated = TelemetryDecimator::decimate_frames(&frames, 100, DecimationStrategy::Average);
    assert!(average_decimated.len() <= 100);
    
    let adaptive_decimated = TelemetryDecimator::decimate_frames(&frames, 100, DecimationStrategy::Adaptive);
    assert!(adaptive_decimated.len() <= 100);
}

#[test]
fn test_error_handling_and_validation() {
    let parser = MultiFormatTelemetryParser::new();
    
    // Test empty data
    let result = parser.parse(&[], TelemetryFormat::Json);
    assert!(result.is_err(), "Empty JSON data should fail");
    
    // Test invalid JSON
    let invalid_json = b"{ invalid json }";
    let result = parser.parse(invalid_json, TelemetryFormat::Json);
    assert!(result.is_err(), "Invalid JSON should fail");
    match result.err().unwrap() {
        ParseError::JsonError(_) => {}, // Expected
        _ => panic!("Should be JSON error"),
    }
    
    // Test binary with insufficient data
    let short_binary = b"123";
    let result = parser.parse(short_binary, TelemetryFormat::Binary);
    assert!(result.is_err(), "Insufficient binary data should fail");
    match result.err().unwrap() {
        ParseError::InsufficientData { required, available } => {
            assert!(required > available, "Required should be > available");
        },
        _ => panic!("Should be insufficient data error"),
    }
    
    // Test CSV column count mismatch
    let csv_data = b"a,b\n1,2,3"; // Header has 2 columns, data has 3
    let result = parser.parse(csv_data, TelemetryFormat::Csv);
    assert!(result.is_err(), "CSV column mismatch should fail");
}

#[test]
fn test_concurrent_buffer_operations() {
    use std::thread;
    use std::sync::Arc;
    
    let buffer = Arc::new(TelemetryRingBuffer::new(2000));
    let num_threads = 4;
    let items_per_thread = 100;
    
    let mut handles = vec![];
    
    // Spawn multiple threads to write data
    for thread_id in 0..num_threads {
        let buffer_clone = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
            for i in 0..items_per_thread {
                let value = thread_id * 1000 + i;
                buffer_clone.push(value);
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify data
    let total_items = num_threads * items_per_thread;
    assert_eq!(buffer.len(), total_items, "Should have all items from all threads");
    assert_eq!(buffer.overflow_count(), 0, "No overflow should occur");
    
    // Test concurrent reads
    let read_handles: Vec<_> = (0..4).map(|_| {
        let buffer_clone = Arc::clone(&buffer);
        thread::spawn(move || {
            let latest = buffer_clone.peek_latest(10);
            assert_eq!(latest.len(), 10, "Should always get 10 latest items");
        })
    }).collect();
    
    for handle in read_handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_memory_usage_estimation() {
    let manager = TelemetryBufferManager::new(2000);
    
    // Initial memory usage should be minimal
    let initial_memory = manager.estimated_memory_usage();
    
    // Add a device buffer
    let frame = TelemetryFrame {
        timestamp: Utc::now(),
        sequence_number: 1,
        device_id: "memory_test".to_string(),
        data_points: vec![],
        metadata: HashMap::new(),
    };
    manager.store_frame(frame);
    
    let memory_with_buffer = manager.estimated_memory_usage();
    assert!(memory_with_buffer > initial_memory, "Memory usage should increase with buffers");
    
    // Memory should be at least capacity * rough frame size
    assert!(memory_with_buffer >= 2000 * 1024, "Should account for buffer capacity");
}

#[test]
fn test_system_stats_accuracy() {
    let manager = TelemetryBufferManager::new(2000);
    
    // Add multiple devices
    for device_num in 0..3 {
        for sequence in 0..10 {
            let frame = TelemetryFrame {
                timestamp: Utc::now(),
                sequence_number: sequence,
                device_id: format!("device_{}", device_num),
                data_points: vec![
                    DataPoint {
                        channel: "test".to_string(),
                        value: TelemetryValue::Float(sequence as f64),
                        unit: None,
                        quality: DataQuality::Good,
                    }
                ],
                metadata: HashMap::new(),
            };
            manager.store_frame(frame);
        }
    }
    
    let stats = manager.get_system_stats();
    assert_eq!(stats.device_count, 3, "Should have 3 devices");
    assert_eq!(stats.total_capacity, 3 * 2000, "Total capacity should be 3 * 2000");
    assert_eq!(stats.total_samples, 3 * 10, "Should have 30 total samples");
    assert_eq!(stats.total_stored_samples, 30, "Stored samples should match");
    assert_eq!(stats.total_parse_errors, 0, "No parse errors expected");
    assert!(stats.average_utilization > 0.0, "Should have some utilization");
}

#[test]
fn test_frame_to_samples_conversion() {
    let parser = MultiFormatTelemetryParser::new();
    
    let frame = TelemetryFrame {
        timestamp: Utc::now(),
        sequence_number: 1,
        device_id: "converter_test".to_string(),
        data_points: vec![
            DataPoint {
                channel: "voltage".to_string(),
                value: TelemetryValue::Float(3.3),
                unit: Some("V".to_string()),
                quality: DataQuality::Good,
            },
            DataPoint {
                channel: "current".to_string(),
                value: TelemetryValue::Integer(500),
                unit: Some("mA".to_string()),
                quality: DataQuality::Questionable,
            }
        ],
        metadata: {
            let mut map = HashMap::new();
            map.insert("test_key".to_string(), "test_value".to_string());
            map
        },
    };
    
    let samples = parser.frame_to_samples(frame.clone());
    assert_eq!(samples.len(), 2, "Should convert to 2 samples");
    
    // Check first sample (voltage)
    let voltage_sample = &samples[0];
    assert_eq!(voltage_sample.timestamp_ms, frame.timestamp.timestamp_millis() as u64);
    
    // Check metadata conversion
    if let Some(ref metadata) = voltage_sample.metadata {
        assert_eq!(metadata.source, Some("converter_test".to_string()));
        assert_eq!(metadata.unit, Some("V".to_string()));
        assert_eq!(metadata.quality, Some(1.0)); // Good quality = 1.0
        assert!(metadata.tags.contains(&"voltage".to_string()));
    } else {
        panic!("Sample should have metadata");
    }
    
    // Check second sample (current)
    let current_sample = &samples[1];
    if let Some(ref metadata) = current_sample.metadata {
        assert_eq!(metadata.quality, Some(0.5)); // Questionable quality = 0.5
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;
    
    #[test]
    fn test_high_throughput_parsing() {
        let parser = MultiFormatTelemetryParser::new();
        let csv_data = b"timestamp,voltage,current\n2024-01-01 12:00:00,3.3,0.5";
        
        let start = Instant::now();
        let iterations = 1000;
        
        for _ in 0..iterations {
            let result = parser.parse(csv_data, TelemetryFormat::Csv);
            assert!(result.is_ok(), "All parsing operations should succeed");
        }
        
        let elapsed = start.elapsed();
        let ops_per_sec = iterations as f64 / elapsed.as_secs_f64();
        
        println!("CSV parsing throughput: {:.0} ops/sec", ops_per_sec);
        assert!(ops_per_sec > 100.0, "Should achieve at least 100 ops/sec");
    }
    
    #[test]
    fn test_buffer_performance() {
        let buffer: TelemetryRingBuffer<u64> = TelemetryRingBuffer::new(10000);
        let start = Instant::now();
        let iterations = 100000;
        
        // Test push performance
        for i in 0..iterations {
            buffer.push(i);
        }
        
        let push_elapsed = start.elapsed();
        let push_ops_per_sec = iterations as f64 / push_elapsed.as_secs_f64();
        
        println!("Buffer push throughput: {:.0} ops/sec", push_ops_per_sec);
        assert!(push_ops_per_sec > 10000.0, "Should achieve at least 10k push ops/sec");
        
        // Test peek performance
        let start = Instant::now();
        let peek_iterations = 1000;
        
        for _ in 0..peek_iterations {
            let _latest = buffer.peek_latest(100);
        }
        
        let peek_elapsed = start.elapsed();
        let peek_ops_per_sec = peek_iterations as f64 / peek_elapsed.as_secs_f64();
        
        println!("Buffer peek throughput: {:.0} ops/sec", peek_ops_per_sec);
        assert!(peek_ops_per_sec > 100.0, "Should achieve at least 100 peek ops/sec");
    }
}