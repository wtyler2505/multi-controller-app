//! Comprehensive tests for telemetry ring buffers
//! 
//! Requirements:
//! - Minimum 2,000 samples capacity
//! - Memory efficient (stay within 150MB total budget)
//! - Support 30 FPS update rate
//! - Thread-safe for concurrent access
//! - Data decimation for charting

use multi_controller_app::telemetry::{
    RingBuffer, TelemetrySample, SampleValue, TelemetryChannel, 
    ChannelConfig, SampleType, SampleMetadata, ChannelStats
};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

#[test]
fn test_ring_buffer_meets_minimum_capacity() {
    // Requirement: Minimum 2,000 samples
    let buffer = RingBuffer::<TelemetrySample>::new(2000);
    assert_eq!(buffer.capacity(), 2000);
    
    // Verify it can actually hold 2,000 samples
    for i in 0..2000 {
        buffer.push(TelemetrySample::new_f32(i as f32));
    }
    assert_eq!(buffer.len(), 2000);
    assert!(buffer.is_full());
}

#[test]
fn test_memory_usage_within_budget() {
    // Create a buffer with 2,000 samples
    let buffer = RingBuffer::<TelemetrySample>::new(2000);
    
    // Fill it completely
    for i in 0..2000 {
        let sample = TelemetrySample::with_metadata(
            SampleValue::Float32(i as f32),
            SampleMetadata::default()
                .source("TestDevice".to_string())
                .unit("V".to_string())
        );
        buffer.push(sample);
    }
    
    // Calculate memory usage
    let memory_bytes = buffer.memory_usage();
    let memory_mb = memory_bytes as f64 / (1024.0 * 1024.0);
    
    println!("Ring buffer memory usage: {:.2} MB for 2,000 samples", memory_mb);
    
    // Each TelemetrySample is approximately:
    // - 8 bytes for timestamp
    // - 32 bytes for SampleValue enum (worst case with Vector)
    // - Optional metadata: ~100 bytes when present
    // Total: ~140 bytes per sample * 2000 = 280KB base
    // With overhead: Should be under 1MB per channel
    
    assert!(memory_mb < 1.0, "Single channel should use less than 1MB");
    
    // Test with 10 channels (realistic scenario)
    let total_memory_mb = memory_mb * 10.0;
    println!("Total memory for 10 channels: {:.2} MB", total_memory_mb);
    assert!(total_memory_mb < 15.0, "10 channels should use less than 15MB");
}

#[test]
fn test_ring_buffer_wraparound_performance() {
    let buffer = RingBuffer::<TelemetrySample>::new(2000);
    
    // Measure time to fill buffer multiple times (wraparound)
    let start = Instant::now();
    
    for i in 0..10_000 {
        buffer.push(TelemetrySample::new_f32(i as f32));
    }
    
    let elapsed = start.elapsed();
    let ops_per_sec = 10_000.0 / elapsed.as_secs_f64();
    
    println!("Push operations per second: {:.0}", ops_per_sec);
    
    // Should handle at least 1000 ops/sec for 30 FPS with headroom
    assert!(ops_per_sec > 1000.0, "Must support at least 1000 ops/sec");
}

#[test]
fn test_concurrent_access_safety() {
    let buffer = Arc::new(RingBuffer::<TelemetrySample>::new(2000));
    let mut handles = vec![];
    
    // Spawn multiple reader threads
    for reader_id in 0..5 {
        let buf = buffer.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                let snapshot = buf.snapshot();
                let stats = buf.stats();
                
                // Verify data consistency
                assert!(snapshot.len() <= 2000);
                assert_eq!(stats.capacity, 2000);
                
                thread::sleep(Duration::from_micros(100));
            }
            println!("Reader {} completed", reader_id);
        }));
    }
    
    // Spawn writer thread
    let buf = buffer.clone();
    handles.push(thread::spawn(move || {
        for i in 0..5000 {
            buf.push(TelemetrySample::new_f32(i as f32));
            
            if i % 100 == 0 {
                thread::sleep(Duration::from_micros(10));
            }
        }
        println!("Writer completed");
    }));
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify final state
    assert_eq!(buffer.len(), 2000);
    assert!(buffer.is_full());
}

#[test]
fn test_30_fps_update_rate() {
    let config = ChannelConfig {
        name: "test_channel".to_string(),
        buffer_size: 2000,
        sample_rate: 30.0,
        sample_type: SampleType::Float32,
    };
    
    let channel = TelemetryChannel::new(config);
    
    // Simulate 30 FPS updates for 1 second
    let start = Instant::now();
    let target_frame_time = Duration::from_millis(33); // ~30 FPS
    
    for frame in 0..30 {
        let frame_start = Instant::now();
        
        // Add 10 samples per frame (300 samples/sec)
        for i in 0..10 {
            let sample = TelemetrySample::new_f32((frame * 10 + i) as f32);
            channel.add_sample(sample);
        }
        
        // Get chart data (with decimation)
        let chart_data = channel.chart_data(300); // Limit to 300 points
        
        // Verify we're not exceeding point limit
        assert!(chart_data.len() <= 300, "Chart data must be decimated to 300 points");
        
        // Maintain frame rate
        let frame_elapsed = frame_start.elapsed();
        if frame_elapsed < target_frame_time {
            thread::sleep(target_frame_time - frame_elapsed);
        }
    }
    
    let total_elapsed = start.elapsed();
    let actual_fps = 30.0 / total_elapsed.as_secs_f32();
    
    println!("Actual FPS: {:.1}", actual_fps);
    
    // Allow 10% variance in frame rate
    assert!(actual_fps > 27.0 && actual_fps < 33.0, "Must maintain ~30 FPS");
    
    // Verify all samples were processed
    let stats = channel.get_stats();
    assert_eq!(stats.total_samples, 300);
}

#[test]
fn test_batch_operations_efficiency() {
    let buffer = RingBuffer::<TelemetrySample>::new(2000);
    
    // Create batch of samples
    let batch: Vec<TelemetrySample> = (0..100)
        .map(|i| TelemetrySample::new_f32(i as f32))
        .collect();
    
    // Measure batch vs individual operations
    let start_batch = Instant::now();
    for _ in 0..10 {
        buffer.push_batch(&batch);
    }
    let batch_time = start_batch.elapsed();
    
    buffer.clear();
    
    let start_individual = Instant::now();
    for _ in 0..10 {
        for sample in &batch {
            buffer.push(sample.clone());
        }
    }
    let individual_time = start_individual.elapsed();
    
    println!("Batch time: {:?}, Individual time: {:?}", batch_time, individual_time);
    
    // Batch should be significantly faster
    assert!(batch_time < individual_time, "Batch operations should be faster");
}

#[test]
fn test_data_decimation_accuracy() {
    let buffer = RingBuffer::<TelemetrySample>::new(2000);
    
    // Fill buffer with known pattern
    for i in 0..2000 {
        let value = (i as f32 / 100.0).sin() * 50.0 + 50.0; // Sine wave 0-100
        buffer.push(TelemetrySample::new_f32(value));
    }
    
    // Test different decimation levels
    let full_snapshot = buffer.snapshot();
    assert_eq!(full_snapshot.len(), 2000);
    
    let decimated_300 = simple_decimate(&full_snapshot, 300);
    assert!(decimated_300.len() <= 300);
    
    let decimated_100 = simple_decimate(&full_snapshot, 100);
    assert!(decimated_100.len() <= 100);
    
    // Verify decimation preserves key features (peaks and valleys)
    let find_max = |samples: &[TelemetrySample]| {
        samples.iter()
            .filter_map(|s| s.as_f32())
            .fold(f32::NEG_INFINITY, f32::max)
    };
    
    let find_min = |samples: &[TelemetrySample]| {
        samples.iter()
            .filter_map(|s| s.as_f32())
            .fold(f32::INFINITY, f32::min)
    };
    
    let full_max = find_max(&full_snapshot);
    let full_min = find_min(&full_snapshot);
    
    let dec_max = find_max(&decimated_300);
    let dec_min = find_min(&decimated_300);
    
    // Decimation should preserve extremes within 5%
    assert!((full_max - dec_max).abs() < 5.0);
    assert!((full_min - dec_min).abs() < 5.0);
}

#[test]
fn test_memory_pressure_handling() {
    // Create multiple channels to simulate real usage
    let channels: Vec<TelemetryChannel> = (0..10)
        .map(|i| {
            let config = ChannelConfig {
                name: format!("channel_{}", i),
                buffer_size: 2000,
                sample_rate: 30.0,
                sample_type: SampleType::Float32,
            };
            TelemetryChannel::new(config)
        })
        .collect();
    
    // Fill all channels
    for channel in &channels {
        for i in 0..2000 {
            channel.add_sample(TelemetrySample::new_f32(i as f32));
        }
    }
    
    // Calculate total memory usage
    let total_memory: usize = channels.iter()
        .map(|c| c.get_stats().memory_bytes)
        .sum();
    
    let total_mb = total_memory as f64 / (1024.0 * 1024.0);
    
    println!("Total memory for 10 full channels: {:.2} MB", total_mb);
    
    // Should be well within our telemetry budget (allowing rest for app)
    assert!(total_mb < 20.0, "Telemetry should use less than 20MB total");
}

#[test]
fn test_pruning_for_memory_management() {
    let buffer = RingBuffer::<TelemetrySample>::new(2000);
    
    // Fill buffer
    for i in 0..2000 {
        buffer.push(TelemetrySample::new_f32(i as f32));
    }
    
    let initial_memory = buffer.memory_usage();
    
    // Prune 25% of oldest data
    buffer.prune_oldest(25);
    
    assert_eq!(buffer.len(), 1500);
    
    // Verify newest data is preserved
    let snapshot = buffer.snapshot();
    let first_value = snapshot[0].as_f32().unwrap();
    assert_eq!(first_value, 500.0); // Oldest 500 samples removed
}

// Helper function for decimation testing
fn simple_decimate(samples: &[TelemetrySample], target_count: usize) -> Vec<TelemetrySample> {
    if samples.len() <= target_count {
        return samples.to_vec();
    }
    
    let step = samples.len() / target_count;
    samples
        .iter()
        .step_by(step.max(1))
        .cloned()
        .collect()
}

#[test]
fn test_performance_metrics() {
    println!("\n=== TELEMETRY RING BUFFER PERFORMANCE METRICS ===");
    
    let buffer = RingBuffer::<TelemetrySample>::new(2000);
    
    // Test write performance
    let start = Instant::now();
    for i in 0..100_000 {
        buffer.push(TelemetrySample::new_f32(i as f32));
    }
    let write_duration = start.elapsed();
    let writes_per_sec = 100_000.0 / write_duration.as_secs_f64();
    
    println!("Write performance: {:.0} samples/sec", writes_per_sec);
    
    // Test read performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = buffer.snapshot();
    }
    let read_duration = start.elapsed();
    let reads_per_sec = 1000.0 / read_duration.as_secs_f64();
    
    println!("Read performance: {:.0} snapshots/sec", reads_per_sec);
    
    // Memory efficiency
    let memory_per_sample = buffer.memory_usage() / buffer.capacity();
    println!("Memory per sample: {} bytes", memory_per_sample);
    
    // Stats calculation performance
    let start = Instant::now();
    for _ in 0..10000 {
        let _ = buffer.stats();
    }
    let stats_duration = start.elapsed();
    let stats_per_sec = 10000.0 / stats_duration.as_secs_f64();
    
    println!("Stats calculation: {:.0} ops/sec", stats_per_sec);
    
    println!("==============================================\n");
    
    // Assert minimum performance requirements
    assert!(writes_per_sec > 10_000.0, "Must support >10K writes/sec");
    assert!(reads_per_sec > 100.0, "Must support >100 reads/sec");
    assert!(memory_per_sample < 200, "Must use <200 bytes per sample");
}