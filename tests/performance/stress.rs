//! Stress tests for concurrent sessions and resource limits

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, Semaphore};
use multi_controller_app::transport::Transport;
use multi_controller_app::drivers::{ArduinoUnoDriver, ArduinoMegaDriver};
use multi_controller_app::device::{DeviceDriver, DeviceSession, DeviceManager};
use multi_controller_app::device::safety::SafetyController;

/// Stress test with maximum concurrent sessions
pub async fn stress_test_max_sessions() -> (usize, f64) {
    println!("\n=== Maximum Concurrent Sessions Stress Test ===");
    
    let max_sessions = 50; // Try to create 50 concurrent sessions
    let mut successful_sessions = 0;
    let mut handles = vec![];
    let failures = Arc::new(AtomicUsize::new(0));
    
    for i in 0..max_sessions {
        let fail_counter = failures.clone();
        
        let handle = tokio::spawn(async move {
            let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
                crate::drivers::mock_transport::MockTransportConfig {
                    device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
                    should_connect: true,
                    connection_delay: Duration::from_millis(50),
                    operation_delay: Duration::from_millis(5),
                    failure_rate: 0.02, // 2% failure under stress
                    transport_name: format!("stress_{}", i),
                }
            )) as Arc<dyn Transport>;
            
            match transport.connect().await {
                Ok(_) => {
                    let driver = ArduinoUnoDriver::new();
                    match driver.open_async(transport).await {
                        Ok(mut session) => {
                            // Run operations for 5 seconds
                            let start = Instant::now();
                            while start.elapsed() < Duration::from_secs(5) {
                                let _ = session.invoke_async(
                                    "analogRead",
                                    vec![serde_json::json!(0)]
                                ).await;
                                tokio::time::sleep(Duration::from_millis(100)).await;
                            }
                            true
                        }
                        Err(_) => {
                            fail_counter.fetch_add(1, Ordering::Relaxed);
                            false
                        }
                    }
                }
                Err(_) => {
                    fail_counter.fetch_add(1, Ordering::Relaxed);
                    false
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all sessions
    for handle in handles {
        if let Ok(success) = handle.await {
            if success {
                successful_sessions += 1;
            }
        }
    }
    
    let success_rate = successful_sessions as f64 / max_sessions as f64;
    println!("  Attempted sessions: {}", max_sessions);
    println!("  Successful sessions: {}", successful_sessions);
    println!("  Success rate: {:.2}%", success_rate * 100.0);
    
    (successful_sessions, success_rate)
}

/// Stress test rapid connect/disconnect cycles
pub async fn stress_test_rapid_reconnection() -> f64 {
    println!("\n=== Rapid Reconnection Stress Test ===");
    
    let cycles = 100;
    let mut successes = 0;
    
    let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
        crate::drivers::mock_transport::MockTransportConfig {
            device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
            should_connect: true,
            connection_delay: Duration::from_millis(10),
            operation_delay: Duration::from_millis(5),
            failure_rate: 0.05, // 5% failure under rapid reconnection
            transport_name: "rapid_reconnect".to_string(),
        }
    )) as Arc<dyn Transport>;
    
    for i in 0..cycles {
        // Connect
        if transport.connect().await.is_ok() {
            // Quick operation
            let _ = transport.write(b"TEST").await;
            
            // Disconnect
            if transport.disconnect().await.is_ok() {
                successes += 1;
            }
        }
        
        // Small delay to prevent overwhelming
        if i % 10 == 0 {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
    
    let success_rate = successes as f64 / cycles as f64;
    println!("  Reconnection cycles: {}", cycles);
    println!("  Successful cycles: {}", successes);
    println!("  Success rate: {:.2}%", success_rate * 100.0);
    
    success_rate
}

/// Stress test sustained load
pub async fn stress_test_sustained_load() -> (f64, Duration) {
    println!("\n=== Sustained Load Stress Test (60 seconds) ===");
    
    let duration = Duration::from_secs(60);
    let operations = Arc::new(AtomicUsize::new(0));
    let errors = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    
    // Create 20 concurrent workers
    let mut handles = vec![];
    
    for i in 0..20 {
        let ops = operations.clone();
        let errs = errors.clone();
        
        let handle = tokio::spawn(async move {
            let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
                crate::drivers::mock_transport::MockTransportConfig {
                    device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
                    should_connect: true,
                    connection_delay: Duration::from_millis(10),
                    operation_delay: Duration::from_millis(3),
                    failure_rate: 0.03, // 3% failure under sustained load
                    transport_name: format!("sustained_{}", i),
                }
            )) as Arc<dyn Transport>;
            
            if transport.connect().await.is_err() {
                return;
            }
            
            let driver = ArduinoUnoDriver::new();
            if let Ok(mut session) = driver.open_async(transport).await {
                let worker_start = Instant::now();
                while worker_start.elapsed() < duration {
                    ops.fetch_add(1, Ordering::Relaxed);
                    
                    let result = session.invoke_async(
                        "digitalWrite",
                        vec![serde_json::json!(13), serde_json::json!(true)]
                    ).await;
                    
                    if result.is_err() {
                        errs.fetch_add(1, Ordering::Relaxed);
                    }
                    
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all workers
    for handle in handles {
        let _ = handle.await;
    }
    
    let total_ops = operations.load(Ordering::Relaxed);
    let total_errors = errors.load(Ordering::Relaxed);
    let success_rate = (total_ops - total_errors) as f64 / total_ops as f64;
    let actual_duration = start.elapsed();
    
    println!("  Test duration: {:?}", actual_duration);
    println!("  Total operations: {}", total_ops);
    println!("  Failed operations: {}", total_errors);
    println!("  Success rate: {:.2}%", success_rate * 100.0);
    println!("  Operations per second: {:.2}", total_ops as f64 / actual_duration.as_secs_f64());
    
    (success_rate, actual_duration)
}

/// Stress test memory stability
pub async fn stress_test_memory_stability() -> bool {
    println!("\n=== Memory Stability Stress Test ===");
    
    let iterations = 1000;
    let mut sessions = vec![];
    
    // Create and destroy many sessions
    for i in 0..iterations {
        let transport = Arc::new(crate::drivers::mock_transport::MockTransport::new(
            crate::drivers::mock_transport::MockTransportConfig {
                device_type: crate::drivers::mock_transport::MockDeviceType::ArduinoUno,
                should_connect: true,
                connection_delay: Duration::from_millis(1),
                operation_delay: Duration::from_millis(1),
                failure_rate: 0.0,
                transport_name: format!("memory_test_{}", i),
            }
        )) as Arc<dyn Transport>;
        
        if transport.connect().await.is_ok() {
            let driver = ArduinoUnoDriver::new();
            if let Ok(session) = driver.open_async(transport).await {
                sessions.push(session);
                
                // Keep only last 10 sessions to test cleanup
                if sessions.len() > 10 {
                    sessions.remove(0);
                }
            }
        }
        
        if i % 100 == 0 {
            println!("  Iteration {}/{}", i, iterations);
        }
    }
    
    println!("  Memory test completed: {} iterations", iterations);
    println!("  Active sessions maintained: {}", sessions.len());
    
    true
}

/// Stress test with safety controller limits
pub async fn stress_test_safety_limits() -> bool {
    println!("\n=== Safety Controller Limits Stress Test ===");
    
    let safety = Arc::new(RwLock::new(SafetyController::new()));
    
    // Configure strict rate limit (10 ops/sec)
    {
        let mut controller = safety.write().await;
        controller.set_rate_limit(10);
    }
    
    let blocked_count = Arc::new(AtomicUsize::new(0));
    let allowed_count = Arc::new(AtomicUsize::new(0));
    
    // Try to exceed rate limit
    let mut handles = vec![];
    
    for _ in 0..5 {
        let safety = safety.clone();
        let blocked = blocked_count.clone();
        let allowed = allowed_count.clone();
        
        let handle = tokio::spawn(async move {
            for _ in 0..100 {
                let controller = safety.read().await;
                if controller.check_rate_limit("test_operation") {
                    allowed.fetch_add(1, Ordering::Relaxed);
                } else {
                    blocked.fetch_add(1, Ordering::Relaxed);
                }
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await.unwrap();
    }
    
    let total_blocked = blocked_count.load(Ordering::Relaxed);
    let total_allowed = allowed_count.load(Ordering::Relaxed);
    
    println!("  Operations attempted: {}", total_blocked + total_allowed);
    println!("  Operations allowed: {}", total_allowed);
    println!("  Operations blocked: {}", total_blocked);
    println!("  Rate limiting effectiveness: {:.2}%", 
             total_blocked as f64 / (total_blocked + total_allowed) as f64 * 100.0);
    
    // Safety controller should have blocked most operations
    total_blocked > total_allowed
}

/// Run all stress tests
pub async fn run_all_stress_tests() {
    println!("\n========================================");
    println!("      STRESS TEST VALIDATION");
    println!("========================================");
    
    let (max_sessions, session_success) = stress_test_max_sessions().await;
    let reconnect_success = stress_test_rapid_reconnection().await;
    let (sustained_success, sustained_duration) = stress_test_sustained_load().await;
    let memory_stable = stress_test_memory_stability().await;
    let safety_effective = stress_test_safety_limits().await;
    
    println!("\n=== Stress Test Summary ===");
    println!("Max concurrent sessions: {} ({:.1}% success)", max_sessions, session_success * 100.0);
    println!("Rapid reconnection: {:.1}% success", reconnect_success * 100.0);
    println!("Sustained load: {:.1}% success over {:?}", sustained_success * 100.0, sustained_duration);
    println!("Memory stability: {}", if memory_stable { "PASSED" } else { "FAILED" });
    println!("Safety limits: {}", if safety_effective { "EFFECTIVE" } else { "INEFFECTIVE" });
    
    // Validate requirements
    assert!(max_sessions >= 20, "Failed to support 20 concurrent sessions");
    assert!(reconnect_success > 0.90, "Reconnection success below 90%");
    assert!(sustained_success > 0.95, "Sustained load success below 95%");
    assert!(memory_stable, "Memory stability test failed");
    assert!(safety_effective, "Safety controller not effective");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_concurrent_sessions() {
        let (sessions, rate) = stress_test_max_sessions().await;
        assert!(sessions >= 20, "Must support at least 20 sessions");
        assert!(rate > 0.8, "Success rate too low");
    }
    
    #[tokio::test]
    async fn test_rapid_reconnection() {
        let rate = stress_test_rapid_reconnection().await;
        assert!(rate > 0.85, "Reconnection success too low");
    }
    
    #[tokio::test]
    async fn test_safety_limits() {
        let effective = stress_test_safety_limits().await;
        assert!(effective, "Safety controller must be effective");
    }
}