//! Throughput benchmarks for high-volume operations

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use multi_controller_app::transport::Transport;
use multi_controller_app::drivers::ArduinoUnoDriver;
use multi_controller_app::device::{DeviceDriver, DeviceSession};
use super::{measure_throughput, BenchmarkConfig};

/// Benchmark driver operations throughput
pub async fn benchmark_driver_throughput() -> f64 {
    println!("\n=== Driver Operations Throughput Benchmark ===");
    
    let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
        crate::drivers::mock_transport::MockTransportConfig {
            device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
            should_connect: true,
            connection_delay: Duration::from_millis(10),
            operation_delay: Duration::from_millis(1), // Fast operations
            failure_rate: 0.0,
            transport_name: "throughput_bench".to_string(),
        }
    )) as Arc<dyn Transport>;
    
    transport.connect().await.expect("Failed to connect");
    
    let driver = ArduinoUnoDriver::new();
    let session = Arc::new(RwLock::new(
        driver.open_async(transport).await.expect("Failed to open session")
    ));
    
    let ops_per_sec = measure_throughput(|| {
        let session = session.clone();
        async move {
            let mut sess = session.write().await;
            sess.invoke_async(
                "digitalWrite",
                vec![serde_json::json!(13), serde_json::json!(true)]
            ).await.is_ok()
        }
    }, Duration::from_secs(10)).await;
    
    println!("  Operations per second: {:.2}", ops_per_sec);
    ops_per_sec
}

/// Benchmark concurrent driver operations
pub async fn benchmark_concurrent_throughput(concurrent_sessions: usize) -> f64 {
    println!("\n=== Concurrent Operations Throughput ({}x) ===", concurrent_sessions);
    
    let operations = Arc::new(AtomicUsize::new(0));
    let successes = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    let duration = Duration::from_secs(10);
    
    let mut handles = vec![];
    let semaphore = Arc::new(Semaphore::new(concurrent_sessions));
    
    for i in 0..concurrent_sessions {
        let ops = operations.clone();
        let succ = successes.clone();
        let sem = semaphore.clone();
        
        let handle = tokio::spawn(async move {
            let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
                crate::drivers::mock_transport::MockTransportConfig {
                    device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
                    should_connect: true,
                    connection_delay: Duration::from_millis(10),
                    operation_delay: Duration::from_millis(2),
                    failure_rate: 0.01, // 1% failure rate under load
                    transport_name: format!("concurrent_{}", i),
                }
            )) as Arc<dyn Transport>;
            
            transport.connect().await.expect("Failed to connect");
            
            let driver = ArduinoUnoDriver::new();
            let mut session = driver.open_async(transport).await.expect("Failed to open session");
            
            let start_time = Instant::now();
            while start_time.elapsed() < duration {
                let _permit = sem.acquire().await.unwrap();
                
                let result = session.invoke_async(
                    "analogRead",
                    vec![serde_json::json!(0)]
                ).await;
                
                ops.fetch_add(1, Ordering::Relaxed);
                if result.is_ok() {
                    succ.fetch_add(1, Ordering::Relaxed);
                }
                
                drop(_permit);
                tokio::time::sleep(Duration::from_micros(100)).await;
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }
    
    let total_ops = operations.load(Ordering::Relaxed);
    let total_successes = successes.load(Ordering::Relaxed);
    let elapsed = start.elapsed().as_secs_f64();
    
    let ops_per_sec = total_ops as f64 / elapsed;
    let success_rate = total_successes as f64 / total_ops as f64;
    
    println!("  Total operations: {}", total_ops);
    println!("  Successful operations: {}", total_successes);
    println!("  Operations per second: {:.2}", ops_per_sec);
    println!("  Success rate: {:.2}%", success_rate * 100.0);
    
    ops_per_sec
}

/// Benchmark telemetry streaming throughput
pub async fn benchmark_telemetry_throughput() -> f64 {
    println!("\n=== Telemetry Streaming Throughput Benchmark ===");
    
    let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
        crate::drivers::mock_transport::MockTransportConfig {
            device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
            should_connect: true,
            connection_delay: Duration::from_millis(10),
            operation_delay: Duration::from_micros(100), // Very fast for telemetry
            failure_rate: 0.0,
            transport_name: "telemetry_bench".to_string(),
        }
    )) as Arc<dyn Transport>;
    
    transport.connect().await.expect("Failed to connect");
    
    let samples_per_sec = measure_throughput(|| {
        let transport = transport.clone();
        async move {
            // Simulate telemetry read
            let data = b"TELEM:100,200,300\n";
            transport.write(data).await.is_ok() &&
            transport.read(100, Duration::from_millis(10)).await.is_ok()
        }
    }, Duration::from_secs(10)).await;
    
    println!("  Telemetry samples per second: {:.2}", samples_per_sec);
    samples_per_sec
}

/// Benchmark bulk data transfer throughput
pub async fn benchmark_bulk_transfer_throughput() -> f64 {
    println!("\n=== Bulk Data Transfer Throughput Benchmark ===");
    
    let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
        crate::drivers::mock_transport::MockTransportConfig {
            device_type: crate::drivers::mock_transport::MockDeviceType::Generic,
            should_connect: true,
            connection_delay: Duration::from_millis(10),
            operation_delay: Duration::from_micros(50),
            failure_rate: 0.0,
            transport_name: "bulk_bench".to_string(),
        }
    )) as Arc<dyn Transport>;
    
    transport.connect().await.expect("Failed to connect");
    
    // Create 1KB data packet
    let data_packet = vec![0xAA; 1024];
    let bytes_transferred = Arc::new(AtomicUsize::new(0));
    let bytes = bytes_transferred.clone();
    
    let start = Instant::now();
    let duration = Duration::from_secs(10);
    
    while start.elapsed() < duration {
        if transport.write(&data_packet).await.is_ok() {
            bytes.fetch_add(data_packet.len(), Ordering::Relaxed);
        }
    }
    
    let total_bytes = bytes_transferred.load(Ordering::Relaxed);
    let elapsed = start.elapsed().as_secs_f64();
    let mb_per_sec = (total_bytes as f64 / 1_048_576.0) / elapsed;
    
    println!("  Data transfer rate: {:.2} MB/s", mb_per_sec);
    println!("  Total bytes transferred: {}", total_bytes);
    
    mb_per_sec
}

/// Run all throughput benchmarks
pub async fn run_all_throughput_benchmarks() {
    println!("\n========================================");
    println!("    THROUGHPUT PERFORMANCE VALIDATION");
    println!("========================================");
    
    let single_throughput = benchmark_driver_throughput().await;
    let concurrent_10 = benchmark_concurrent_throughput(10).await;
    let concurrent_20 = benchmark_concurrent_throughput(20).await;
    let telemetry_throughput = benchmark_telemetry_throughput().await;
    let bulk_throughput = benchmark_bulk_transfer_throughput().await;
    
    println!("\n=== Throughput Summary ===");
    println!("Single session: {:.2} ops/sec", single_throughput);
    println!("10 concurrent: {:.2} ops/sec", concurrent_10);
    println!("20 concurrent: {:.2} ops/sec", concurrent_20);
    println!("Telemetry: {:.2} samples/sec", telemetry_throughput);
    println!("Bulk transfer: {:.2} MB/sec", bulk_throughput);
    
    // Validate against requirements (100+ ops/sec)
    assert!(single_throughput > 100.0, "Single session throughput below 100 ops/sec");
    assert!(concurrent_20 > 100.0, "Concurrent throughput below 100 ops/sec");
    assert!(telemetry_throughput > 1000.0, "Telemetry throughput below 1000 samples/sec");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_driver_throughput() {
        let throughput = benchmark_driver_throughput().await;
        assert!(throughput > 100.0, "Driver throughput below target");
    }
    
    #[tokio::test]
    async fn test_concurrent_throughput() {
        let throughput = benchmark_concurrent_throughput(5).await;
        assert!(throughput > 50.0, "Concurrent throughput too low");
    }
    
    #[tokio::test]
    async fn test_telemetry_throughput() {
        let throughput = benchmark_telemetry_throughput().await;
        assert!(throughput > 500.0, "Telemetry throughput too low");
    }
}