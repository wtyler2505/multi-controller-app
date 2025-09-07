//! Driver integration test modules
//! 
//! This module contains comprehensive integration tests for the Multi-Controller App
//! driver system, covering scenarios such as multiple driver coordination,
//! hot-swapping, error recovery, failover, transport reconnection,
//! device manager integration, plugin loading, safety controller integration,
//! and performance under load.

pub mod driver_integration;
pub mod test_drivers;
pub mod test_utils;

// Re-export commonly used test utilities
pub use test_utils::*;
pub use test_drivers::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Basic smoke test to ensure test infrastructure works
    #[tokio::test]
    async fn test_infrastructure_smoke_test() {
        let test_env = TestEnvironment::new().await;
        
        // Test creating mock drivers
        let arduino_driver = test_env.create_mock_arduino_driver("SmokeTest_Arduino").await;
        let pi_driver = test_env.create_mock_raspberry_pi_driver("SmokeTest_Pi").await;
        let generic_driver = test_env.create_mock_generic_driver("SmokeTest_Generic").await;
        
        assert_eq!(arduino_driver.name(), "SmokeTest_Arduino");
        assert_eq!(pi_driver.name(), "SmokeTest_Pi");
        assert_eq!(generic_driver.name(), "SmokeTest_Generic");
        
        // Test creating mock transports
        let serial_transport = test_env.create_mock_transport("smoke_serial", multi_controller_app::device::TransportType::Serial).await;
        let ssh_transport = test_env.create_mock_transport("smoke_ssh", multi_controller_app::device::TransportType::SSH).await;
        
        assert_eq!(serial_transport.name(), "smoke_serial");
        assert_eq!(ssh_transport.name(), "smoke_ssh");
        
        println!("Test infrastructure smoke test passed");
    }
}