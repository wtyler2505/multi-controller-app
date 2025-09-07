/// Common test utilities and helpers
use std::time::Duration;
use tokio::time::timeout;

/// Test timeout helper with better error messages
pub async fn with_timeout<F, T>(duration: Duration, future: F, context: &str) -> T
where
    F: std::future::Future<Output = T>,
{
    timeout(duration, future)
        .await
        .unwrap_or_else(|_| panic!("Test timeout after {:?}: {}", duration, context))
}

/// Helper to create test data patterns
pub fn create_test_data(size: usize, pattern: u8) -> Vec<u8> {
    vec![pattern; size]
}

/// Verify data matches expected pattern
pub fn verify_test_data(data: &[u8], expected_pattern: u8) -> bool {
    data.iter().all(|&b| b == expected_pattern)
}

/// Generate incrementing test data for uniqueness
pub fn create_unique_test_data(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Helper to assert async results with context
#[macro_export]
macro_rules! assert_ok {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => panic!("{}: {:?}", $msg, err),
        }
    };
}

#[macro_export]
macro_rules! assert_err {
    ($expr:expr, $msg:expr) => {
        match $expr {
            Err(err) => err,
            Ok(_) => panic!("{}: Expected error but got Ok", $msg),
        }
    };
}

/// Test environment setup helper
pub struct TestEnv {
    pub test_name: String,
    pub start_time: std::time::Instant,
}

impl TestEnv {
    pub fn new(test_name: &str) -> Self {
        eprintln!("\n=== Starting test: {} ===", test_name);
        TestEnv {
            test_name: test_name.to_string(),
            start_time: std::time::Instant::now(),
        }
    }
    
    pub fn checkpoint(&self, msg: &str) {
        eprintln!("[{:>4.1}s] {}: {}", 
            self.start_time.elapsed().as_secs_f32(),
            self.test_name, 
            msg
        );
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        eprintln!("=== Test {} completed in {:.1}s ===\n",
            self.test_name,
            self.start_time.elapsed().as_secs_f32()
        );
    }
}

/// Helper to run async code in blocking context (for property tests)
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(future)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_test_data() {
        let data = create_test_data(10, 0xAB);
        assert_eq!(data.len(), 10);
        assert!(verify_test_data(&data, 0xAB));
    }
    
    #[test] 
    fn test_create_unique_test_data() {
        let data = create_unique_test_data(300);
        assert_eq!(data.len(), 300);
        assert_eq!(data[0], 0);
        assert_eq!(data[255], 255);
        assert_eq!(data[256], 0); // Wraps around
    }
    
    #[tokio::test]
    async fn test_timeout_helper() {
        // Should complete
        let result = with_timeout(
            Duration::from_millis(100),
            async { 42 },
            "test operation"
        ).await;
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_env_helper() {
        let env = TestEnv::new("sample_test");
        std::thread::sleep(Duration::from_millis(10));
        env.checkpoint("Did something");
        std::thread::sleep(Duration::from_millis(10));
        env.checkpoint("Did something else");
        // Drop will print completion
    }
}