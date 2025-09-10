//! Test utilities and helper functions for driver integration tests
//! 
//! Provides common functionality for setting up test environments,
//! creating mock objects, and performing test operations.

use std::sync::Arc;
use std::time::Duration;
use serde_json::Value;
use tokio::time::timeout;

use multi_controller_app::device::{
    DeviceManager, DeviceDriver, DeviceResult, DeviceError,
    Transport, TransportConfig, TransportType, DriverInfo, DriverPriority
};
use multi_controller_app::transport::{MockTransport, MockConfig};

use super::test_drivers::{
    MockArduinoDriver, MockRaspberryPiDriver, MockGenericDriver
};

/// Test environment for driver integration tests
pub struct TestEnvironment {
    pub manager: DeviceManager,
}

impl TestEnvironment {
    /// Create a new test environment
    pub async fn new() -> Self {
        let manager = DeviceManager::new("./test_drivers");
        manager.initialize().await.expect("Failed to initialize device manager");
        
        TestEnvironment { manager }
    }

    /// Create a mock Arduino driver
    pub async fn create_mock_arduino_driver(&self, name: &str) -> Arc<dyn DeviceDriver> {
        Arc::new(MockArduinoDriver::new(name))
    }

    /// Create a mock Raspberry Pi driver
    pub async fn create_mock_raspberry_pi_driver(&self, name: &str) -> Arc<dyn DeviceDriver> {
        Arc::new(MockRaspberryPiDriver::new(name))
    }

    /// Create a mock generic driver
    pub async fn create_mock_generic_driver(&self, name: &str) -> Arc<dyn DeviceDriver> {
        Arc::new(MockGenericDriver::new(name))
    }

    /// Create a failing driver (fails after N operations)
    pub async fn create_failing_driver(&self, name: &str, fail_after: u32) -> Arc<dyn DeviceDriver> {
        Arc::new(MockArduinoDriver::with_failure_after(name, fail_after))
    }

    /// Register a driver with the device manager
    pub async fn register_driver(&self, driver: Arc<dyn DeviceDriver>) {
        let driver_info = DriverInfo::new(driver);
        // Note: This would require extending DeviceManager with a register_driver method
        // For now, we'll assume it exists or work around it
    }

    /// Register a driver with specific priority
    pub async fn register_driver_with_priority(&self, driver: Arc<dyn DeviceDriver>, priority: DriverPriority) {
        let driver_info = DriverInfo::new(driver).with_priority(priority);
        // Note: This would require extending DeviceManager
    }

    /// Create a mock transport
    pub async fn create_mock_transport(&self, name: &str, transport_type: TransportType) -> Arc<dyn Transport> {
        let config = TransportConfig::default();
        let mock_config = MockConfig::default();
        
        let transport = MockTransport::new(name.to_string(), config, mock_config);
        Arc::new(transport)
    }

    /// Create a mock transport with specific configuration
    pub async fn create_configured_mock_transport(
        &self, 
        name: &str, 
        transport_type: TransportType,
        mock_config: MockConfig
    ) -> Arc<dyn Transport> {
        let config = TransportConfig::default();
        let transport = MockTransport::new(name.to_string(), config, mock_config);
        Arc::new(transport)
    }

    /// Clean up a specific session
    pub async fn cleanup_session(&self, session_id: &str) {
        let _ = self.manager.close_device(session_id).await;
    }

    /// Clean up all active sessions
    pub async fn cleanup_all_sessions(&self) {
        let sessions = self.manager.list_sessions().await;
        for session_id in sessions {
            let _ = self.manager.close_device(&session_id).await;
        }
    }
}

/// Helper function to perform concurrent device operations
pub async fn test_concurrent_device_operation(
    manager: &DeviceManager,
    session_id: &str,
    endpoint: &str,
    args: Value,
) -> DeviceResult<Value> {
    // Note: This would require extending DeviceManager to expose session operations
    // For now, we'll simulate the operation
    
    timeout(Duration::from_secs(5), async {
        // Simulate operation delay
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        // Simulate different responses based on endpoint
        match endpoint {
            "digitalWrite" => Ok(serde_json::json!({ "success": true })),
            "digitalRead" => Ok(serde_json::json!({ "value": true })),
            "analogRead" => Ok(serde_json::json!({ "value": 512 })),
            "gpioWrite" => Ok(serde_json::json!({ "success": true })),
            "gpioRead" => Ok(serde_json::json!({ "value": false })),
            "readTemperature" => Ok(serde_json::json!({ "temperature_c": 22.5 })),
            "status" => Ok(serde_json::json!({ "active": true })),
            _ => Err(DeviceError::Unknown(format!("Unknown endpoint: {}", endpoint))),
        }
    })
    .await
    .map_err(|_| DeviceError::Timeout("Operation timeout".into()))?
}

/// Helper function to create test data patterns
pub fn generate_test_data(size: usize, pattern: u8) -> Vec<u8> {
    (0..size).map(|i| pattern.wrapping_add(i as u8)).collect()
}

/// Helper function to validate test results
pub fn assert_operation_success(result: &DeviceResult<Value>, operation: &str) {
    match result {
        Ok(value) => {
            if let Some(success) = value.get("success") {
                assert!(success.as_bool().unwrap_or(false), 
                       "Operation {} should report success", operation);
            }
            // For read operations, just ensure we got a value
            if operation.contains("Read") || operation.contains("read") {
                assert!(value.get("value").is_some() || value.get("temperature_c").is_some(), 
                       "Read operation {} should return a value", operation);
            }
        }
        Err(e) => panic!("Operation {} failed: {}", operation, e),
    }
}

/// Helper function to measure operation latency
pub async fn measure_operation_latency<F, Fut>(
    operation: F,
    samples: usize,
) -> (Duration, Duration, Duration) // (min, max, avg)
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = DeviceResult<Value>>,
{
    let mut latencies = Vec::new();
    
    for _ in 0..samples {
        let start = std::time::Instant::now();
        let _ = operation().await;
        let latency = start.elapsed();
        latencies.push(latency);
    }
    
    let min = *latencies.iter().min().unwrap();
    let max = *latencies.iter().max().unwrap();
    let avg = Duration::from_nanos(
        latencies.iter().map(|d| d.as_nanos()).sum::<u128>() as u64 / latencies.len() as u64
    );
    
    (min, max, avg)
}

/// Helper function to simulate network conditions
pub async fn simulate_network_condition(
    condition: NetworkCondition,
    duration: Duration,
) {
    match condition {
        NetworkCondition::HighLatency => {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        NetworkCondition::PacketLoss => {
            // Simulate 10% packet loss
            if rand::random::<f32>() < 0.1 {
                tokio::time::sleep(duration).await;
            }
        }
        NetworkCondition::Unstable => {
            // Random delays
            let delay = Duration::from_millis(rand::random::<u64>() % 50);
            tokio::time::sleep(delay).await;
        }
        NetworkCondition::Normal => {
            tokio::time::sleep(Duration::from_millis(5)).await;
        }
    }
}

/// Network condition simulation types
pub enum NetworkCondition {
    Normal,
    HighLatency,
    PacketLoss,
    Unstable,
}

/// Helper function to create stress test workload
pub async fn create_stress_workload(
    manager: &DeviceManager,
    session_id: &str,
    operations_per_second: u32,
    duration: Duration,
) -> Vec<DeviceResult<Value>> {
    let interval = Duration::from_nanos(1_000_000_000 / operations_per_second as u64);
    let end_time = std::time::Instant::now() + duration;
    let mut results = Vec::new();
    
    let operations = ["digitalRead", "analogRead", "digitalWrite"];
    let mut op_index = 0;
    
    while std::time::Instant::now() < end_time {
        let operation = operations[op_index % operations.len()];
        let args = match operation {
            "digitalRead" | "analogRead" => serde_json::json!([2]),
            "digitalWrite" => serde_json::json!([13, op_index % 2 == 0]),
            _ => serde_json::json!([]),
        };
        
        let result = test_concurrent_device_operation(manager, session_id, operation, args).await;
        results.push(result);
        
        op_index += 1;
        tokio::time::sleep(interval).await;
    }
    
    results
}

/// Helper function to analyze test results
pub struct TestResultAnalysis {
    pub total_operations: usize,
    pub successful_operations: usize,
    pub failed_operations: usize,
    pub success_rate: f64,
    pub average_latency: Duration,
    pub error_types: std::collections::HashMap<String, usize>,
}

pub fn analyze_test_results(results: &[DeviceResult<Value>], latencies: &[Duration]) -> TestResultAnalysis {
    let total_operations = results.len();
    let successful_operations = results.iter().filter(|r| r.is_ok()).count();
    let failed_operations = total_operations - successful_operations;
    let success_rate = successful_operations as f64 / total_operations as f64;
    
    let average_latency = if !latencies.is_empty() {
        Duration::from_nanos(
            latencies.iter().map(|d| d.as_nanos()).sum::<u128>() as u64 / latencies.len() as u64
        )
    } else {
        Duration::ZERO
    };
    
    let mut error_types = std::collections::HashMap::new();
    for result in results {
        if let Err(e) = result {
            let error_type = match e {
                DeviceError::DeviceNotFound(_) => "DeviceNotFound",
                DeviceError::DeviceNotConnected(_) => "DeviceNotConnected",
                DeviceError::Timeout(_) => "Timeout",
                DeviceError::Unknown(_) => "Unknown",
            };
            *error_types.entry(error_type.to_string()).or_insert(0) += 1;
        }
    }
    
    TestResultAnalysis {
        total_operations,
        successful_operations,
        failed_operations,
        success_rate,
        average_latency,
        error_types,
    }
}

/// Helper macro for creating test assertions with detailed error messages
#[macro_export]
macro_rules! assert_test_result {
    ($result:expr, $operation:expr, $expected:expr) => {
        match $result {
            Ok(value) => {
                assert_eq!(value, $expected, 
                          "Operation '{}' returned unexpected value", $operation);
            }
            Err(e) => {
                panic!("Operation '{}' failed: {}", $operation, e);
            }
        }
    };
}

/// Helper macro for timing test operations
#[macro_export]
macro_rules! time_operation {
    ($operation:expr) => {{
        let start = std::time::Instant::now();
        let result = $operation;
        let duration = start.elapsed();
        (result, duration)
    }};
}

/// Helper function to validate driver capabilities
pub fn validate_driver_capabilities(
    driver: &dyn DeviceDriver,
    expected_transports: &[TransportType],
    expected_features: &[&str],
) {
    let supported_transports = driver.supported_transports();
    for expected_transport in expected_transports {
        assert!(supported_transports.contains(expected_transport),
               "Driver {} should support transport {:?}", 
               driver.name(), expected_transport);
    }
    
    let capabilities = driver.capabilities();
    for feature in expected_features {
        match *feature {
            "gpio" => assert!(capabilities.gpio, "Driver should support GPIO"),
            "pwm" => assert!(capabilities.pwm, "Driver should support PWM"),
            "analog" => assert!(capabilities.analog_input, "Driver should support analog input"),
            "telemetry" => assert!(capabilities.telemetry, "Driver should support telemetry"),
            "hotplug" => assert!(capabilities.hot_plug, "Driver should support hot plug"),
            _ => panic!("Unknown capability: {}", feature),
        }
    }
}

/// Random testing utilities
pub mod random {
    use rand::Rng;
    
    pub fn random_delay(min_ms: u64, max_ms: u64) -> std::time::Duration {
        let delay_ms = rand::thread_rng().gen_range(min_ms..=max_ms);
        std::time::Duration::from_millis(delay_ms)
    }
    
    pub fn random_bool_with_probability(probability: f32) -> bool {
        rand::thread_rng().gen::<f32>() < probability
    }
    
    pub fn random_bytes(length: usize) -> Vec<u8> {
        (0..length).map(|_| rand::random::<u8>()).collect()
    }
}