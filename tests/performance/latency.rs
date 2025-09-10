//! Latency benchmarks for transport and driver operations

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use multi_controller_app::transport::{Transport, SerialTransport, TcpTransport, UdpTransport};
use multi_controller_app::drivers::ArduinoUnoDriver;
use multi_controller_app::device::{DeviceDriver, DeviceSession};
use super::{measure_latency, calculate_percentiles, BenchmarkConfig};

/// Benchmark serial transport latency
pub async fn benchmark_serial_latency() -> (Duration, Duration, Duration) {
    println!("\n=== Serial Transport Latency Benchmark ===");
    
    // Create mock serial transport
    let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
        crate::drivers::mock_transport::MockTransportConfig {
            device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
            should_connect: true,
            connection_delay: Duration::from_millis(10),
            operation_delay: Duration::from_millis(5),
            failure_rate: 0.0,
            transport_name: "serial_bench".to_string(),
        }
    )) as Arc<dyn Transport>;
    
    transport.connect().await.expect("Failed to connect");
    
    let latencies = measure_latency(|| async {
        let data = b"TEST_DATA";
        let _ = transport.write(data).await;
        let _ = transport.read(100, Duration::from_millis(100)).await;
    }, 1000).await;
    
    let (p50, p95, p99) = calculate_percentiles(&latencies);
    println!("  P50: {:?}, P95: {:?}, P99: {:?}", p50, p95, p99);
    
    (p50, p95, p99)
}

/// Benchmark TCP transport latency
pub async fn benchmark_tcp_latency() -> (Duration, Duration, Duration) {
    println!("\n=== TCP Transport Latency Benchmark ===");
    
    let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
        crate::drivers::mock_transport::MockTransportConfig {
            device_type: crate::drivers::mock_transport::MockDeviceType::Generic,
            should_connect: true,
            connection_delay: Duration::from_millis(15),
            operation_delay: Duration::from_millis(3),
            failure_rate: 0.0,
            transport_name: "tcp_bench".to_string(),
        }
    )) as Arc<dyn Transport>;
    
    transport.connect().await.expect("Failed to connect");
    
    let latencies = measure_latency(|| async {
        let data = b"TCP_TEST";
        let _ = transport.write(data).await;
        let _ = transport.read(100, Duration::from_millis(100)).await;
    }, 1000).await;
    
    let (p50, p95, p99) = calculate_percentiles(&latencies);
    println!("  P50: {:?}, P95: {:?}, P99: {:?}", p50, p95, p99);
    
    (p50, p95, p99)
}

/// Benchmark UDP transport latency
pub async fn benchmark_udp_latency() -> (Duration, Duration, Duration) {
    println!("\n=== UDP Transport Latency Benchmark ===");
    
    let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
        crate::drivers::mock_transport::MockTransportConfig {
            device_type: crate::drivers::mock_transport::MockDeviceType::Generic,
            should_connect: true,
            connection_delay: Duration::from_millis(5),
            operation_delay: Duration::from_millis(2),
            failure_rate: 0.0,
            transport_name: "udp_bench".to_string(),
        }
    )) as Arc<dyn Transport>;
    
    transport.connect().await.expect("Failed to connect");
    
    let latencies = measure_latency(|| async {
        let data = b"UDP_TEST";
        let _ = transport.write(data).await;
        let _ = transport.read(100, Duration::from_millis(100)).await;
    }, 1000).await;
    
    let (p50, p95, p99) = calculate_percentiles(&latencies);
    println!("  P50: {:?}, P95: {:?}, P99: {:?}", p50, p95, p99);
    
    (p50, p95, p99)
}

/// Benchmark driver operation latency
pub async fn benchmark_driver_latency() -> (Duration, Duration, Duration) {
    println!("\n=== Driver Operation Latency Benchmark ===");
    
    let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
        crate::drivers::mock_transport::MockTransportConfig {
            device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
            should_connect: true,
            connection_delay: Duration::from_millis(10),
            operation_delay: Duration::from_millis(5),
            failure_rate: 0.0,
            transport_name: "driver_bench".to_string(),
        }
    )) as Arc<dyn Transport>;
    
    transport.connect().await.expect("Failed to connect");
    
    let driver = ArduinoUnoDriver::new();
    let mut session = driver.open_async(transport).await.expect("Failed to open session");
    
    let latencies = measure_latency(|| async {
        let _ = session.invoke_async(
            "digitalWrite",
            vec![serde_json::json!(13), serde_json::json!(true)]
        ).await;
    }, 1000).await;
    
    let (p50, p95, p99) = calculate_percentiles(&latencies);
    println!("  P50: {:?}, P95: {:?}, P99: {:?}", p50, p95, p99);
    
    (p50, p95, p99)
}

/// Benchmark reconnection latency
pub async fn benchmark_reconnection_latency() -> Duration {
    println!("\n=== Reconnection Latency Benchmark ===");
    
    let mut total_time = Duration::ZERO;
    let iterations = 10;
    
    for _ in 0..iterations {
        let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
            crate::drivers::mock_transport::MockTransportConfig {
                device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
                should_connect: true,
                connection_delay: Duration::from_millis(100),
                operation_delay: Duration::from_millis(5),
                failure_rate: 0.0,
                transport_name: "reconnect_bench".to_string(),
            }
        )) as Arc<dyn Transport>;
        
        // Connect
        transport.connect().await.expect("Failed to connect");
        
        // Disconnect
        transport.disconnect().await.expect("Failed to disconnect");
        
        // Measure reconnection time
        let start = Instant::now();
        transport.connect().await.expect("Failed to reconnect");
        total_time += start.elapsed();
    }
    
    let avg_time = total_time / iterations;
    println!("  Average reconnection time: {:?}", avg_time);
    
    avg_time
}

/// Run all latency benchmarks
pub async fn run_all_latency_benchmarks() {
    println!("\n========================================");
    println!("    LATENCY PERFORMANCE VALIDATION");
    println!("========================================");
    
    let serial_latency = benchmark_serial_latency().await;
    let tcp_latency = benchmark_tcp_latency().await;
    let udp_latency = benchmark_udp_latency().await;
    let driver_latency = benchmark_driver_latency().await;
    let reconnection_time = benchmark_reconnection_latency().await;
    
    println!("\n=== Latency Summary ===");
    println!("Serial Transport P50: {:?}", serial_latency.0);
    println!("TCP Transport P50: {:?}", tcp_latency.0);
    println!("UDP Transport P50: {:?}", udp_latency.0);
    println!("Driver Operation P50: {:?}", driver_latency.0);
    println!("Reconnection Time: {:?}", reconnection_time);
    
    // Validate against requirements
    assert!(serial_latency.0 < Duration::from_millis(100), "Serial latency exceeds 100ms target");
    assert!(tcp_latency.0 < Duration::from_millis(100), "TCP latency exceeds 100ms target");
    assert!(driver_latency.0 < Duration::from_millis(100), "Driver latency exceeds 100ms target");
    assert!(reconnection_time < Duration::from_secs(5), "Reconnection time exceeds 5s target");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_serial_latency_benchmark() {
        let (p50, p95, p99) = benchmark_serial_latency().await;
        assert!(p50 < Duration::from_millis(100));
        assert!(p95 < Duration::from_millis(200));
        assert!(p99 < Duration::from_millis(500));
    }
    
    #[tokio::test]
    async fn test_tcp_latency_benchmark() {
        let (p50, p95, p99) = benchmark_tcp_latency().await;
        assert!(p50 < Duration::from_millis(100));
        assert!(p95 < Duration::from_millis(200));
        assert!(p99 < Duration::from_millis(500));
    }
    
    #[tokio::test]
    async fn test_driver_latency_benchmark() {
        let (p50, p95, p99) = benchmark_driver_latency().await;
        assert!(p50 < Duration::from_millis(100));
        assert!(p95 < Duration::from_millis(200));
        assert!(p99 < Duration::from_millis(500));
    }
}