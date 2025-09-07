//! Performance validation test suite
//! 
//! Comprehensive benchmarks for latency, throughput, and resource usage

pub mod benchmark;
pub mod latency;
pub mod throughput;
pub mod stress;
pub mod metrics;

use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Performance metrics collector
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub latency_p50: Duration,
    pub latency_p95: Duration,
    pub latency_p99: Duration,
    pub throughput_ops_per_sec: f64,
    pub success_rate: f64,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: f32,
    pub concurrent_sessions: usize,
}

/// Test configuration for performance benchmarks
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub warmup_iterations: usize,
    pub test_iterations: usize,
    pub concurrent_sessions: usize,
    pub operation_timeout: Duration,
    pub measure_resources: bool,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        BenchmarkConfig {
            warmup_iterations: 100,
            test_iterations: 1000,
            concurrent_sessions: 20,
            operation_timeout: Duration::from_millis(1000),
            measure_resources: true,
        }
    }
}

/// Measure operation latency
pub async fn measure_latency<F, Fut>(
    operation: F,
    iterations: usize,
) -> Vec<Duration>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let mut latencies = Vec::with_capacity(iterations);
    
    for _ in 0..iterations {
        let start = Instant::now();
        operation().await;
        latencies.push(start.elapsed());
    }
    
    latencies.sort_unstable();
    latencies
}

/// Calculate percentiles from sorted latencies
pub fn calculate_percentiles(latencies: &[Duration]) -> (Duration, Duration, Duration) {
    if latencies.is_empty() {
        return (Duration::ZERO, Duration::ZERO, Duration::ZERO);
    }
    
    let p50_idx = latencies.len() / 2;
    let p95_idx = (latencies.len() as f64 * 0.95) as usize;
    let p99_idx = (latencies.len() as f64 * 0.99) as usize;
    
    (
        latencies[p50_idx],
        latencies[p95_idx.min(latencies.len() - 1)],
        latencies[p99_idx.min(latencies.len() - 1)],
    )
}

/// Measure throughput in operations per second
pub async fn measure_throughput<F, Fut>(
    operation: F,
    duration: Duration,
) -> f64
where
    F: Fn() -> Fut + Clone + Send + 'static,
    Fut: std::future::Future<Output = bool> + Send,
{
    let start = Instant::now();
    let mut operations = 0;
    let mut successes = 0;
    
    while start.elapsed() < duration {
        if operation().await {
            successes += 1;
        }
        operations += 1;
    }
    
    let elapsed_secs = start.elapsed().as_secs_f64();
    operations as f64 / elapsed_secs
}

/// Run benchmark with configuration
pub async fn run_benchmark<F, Fut>(
    name: &str,
    config: BenchmarkConfig,
    operation: F,
) -> PerformanceMetrics
where
    F: Fn() -> Fut + Clone + Send + 'static,
    Fut: std::future::Future<Output = bool> + Send,
{
    println!("Running benchmark: {}", name);
    println!("  Warmup: {} iterations", config.warmup_iterations);
    println!("  Test: {} iterations", config.test_iterations);
    println!("  Concurrent sessions: {}", config.concurrent_sessions);
    
    // Warmup phase
    for _ in 0..config.warmup_iterations {
        let _ = operation().await;
    }
    
    // Measure latency
    let latencies = measure_latency(
        || async { operation().await; },
        config.test_iterations
    ).await;
    
    let (p50, p95, p99) = calculate_percentiles(&latencies);
    
    // Measure throughput
    let throughput = measure_throughput(
        operation.clone(),
        Duration::from_secs(10)
    ).await;
    
    // Calculate success rate
    let mut successes = 0;
    for _ in 0..100 {
        if operation().await {
            successes += 1;
        }
    }
    let success_rate = successes as f64 / 100.0;
    
    // TODO: Implement actual resource measurement
    let cpu_usage = 15.0; // Placeholder
    let memory_usage = 50.0; // Placeholder
    
    let metrics = PerformanceMetrics {
        latency_p50: p50,
        latency_p95: p95,
        latency_p99: p99,
        throughput_ops_per_sec: throughput,
        success_rate,
        cpu_usage_percent: cpu_usage,
        memory_usage_mb: memory_usage,
        concurrent_sessions: config.concurrent_sessions,
    };
    
    println!("Benchmark '{}' completed:", name);
    println!("  Latency P50: {:?}", metrics.latency_p50);
    println!("  Latency P95: {:?}", metrics.latency_p95);
    println!("  Latency P99: {:?}", metrics.latency_p99);
    println!("  Throughput: {:.2} ops/sec", metrics.throughput_ops_per_sec);
    println!("  Success rate: {:.2}%", metrics.success_rate * 100.0);
    
    metrics
}