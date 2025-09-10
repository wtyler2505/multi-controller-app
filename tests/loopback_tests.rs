/// Integration tests for transport layer loopback functionality
/// 
/// These tests validate data integrity and reliability across all transport types
/// using loopback configurations (virtual ports, local connections, etc.)

mod loopback;

// Re-export test modules to ensure they're compiled and available
pub use loopback::*;

#[cfg(test)]
mod loopback_test_runner {
    use super::loopback;
    
    /// Ensure all loopback modules compile correctly
    #[test]
    fn loopback_modules_compile() {
        // This test ensures all loopback test modules are compiled
        // Individual tests are marked with #[ignore] since they require hardware
        assert!(true, "Loopback modules compiled successfully");
    }
    
    /// Basic validation that test utilities work
    #[tokio::test]
    async fn test_patterns_creation() {
        let patterns = loopback::common::TestPatterns::new();
        assert_eq!(patterns.simple.len(), 4);
        assert_eq!(patterns.large.len(), 8192);
        assert!(!patterns.edge_cases.is_empty());
    }
    
    /// Validate loopback configuration defaults
    #[test]
    fn test_loopback_config_defaults() {
        let config = loopback::common::LoopbackConfig::default();
        assert_eq!(config.timeout.as_secs(), 5);
        assert_eq!(config.retry_count, 3);
        assert!(config.verify_stats);
        assert!(config.test_reconnection);
    }
}