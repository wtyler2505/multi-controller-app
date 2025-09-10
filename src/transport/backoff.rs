use std::time::Duration;
use rand::Rng;

/// Configuration for exponential backoff retry logic
#[derive(Debug, Clone)]
pub struct ExponentialBackoff {
    /// Initial delay in milliseconds (default: 1000ms)
    initial_delay_ms: u64,
    
    /// Maximum delay in milliseconds (default: 30000ms)
    max_delay_ms: u64,
    
    /// Multiplication factor for each retry (default: 2.0)
    factor: f64,
    
    /// Maximum number of attempts (0 = unlimited)
    max_attempts: u32,
    
    /// Enable jitter to prevent thundering herd (default: true)
    enable_jitter: bool,
    
    /// Current attempt number
    current_attempt: u32,
}

impl ExponentialBackoff {
    /// Create a new exponential backoff configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Builder method to set initial delay
    pub fn with_initial_delay(mut self, ms: u64) -> Self {
        self.initial_delay_ms = ms;
        self
    }
    
    /// Builder method to set maximum delay
    pub fn with_max_delay(mut self, ms: u64) -> Self {
        self.max_delay_ms = ms;
        self
    }
    
    /// Builder method to set backoff factor
    pub fn with_factor(mut self, factor: f64) -> Self {
        self.factor = factor;
        self
    }
    
    /// Builder method to set maximum attempts
    pub fn with_max_attempts(mut self, attempts: u32) -> Self {
        self.max_attempts = attempts;
        self
    }
    
    /// Builder method to enable/disable jitter
    pub fn with_jitter(mut self, enable: bool) -> Self {
        self.enable_jitter = enable;
        self
    }
    
    /// Reset the backoff to initial state
    pub fn reset(&mut self) {
        self.current_attempt = 0;
    }
    
    /// Check if we should retry based on attempt count
    pub fn should_retry(&self) -> bool {
        self.max_attempts == 0 || self.current_attempt < self.max_attempts
    }
    
    /// Get the next delay duration and increment attempt counter
    pub fn next_delay(&mut self) -> Option<Duration> {
        if !self.should_retry() {
            return None;
        }
        
        self.current_attempt += 1;
        
        // Calculate exponential delay
        let base_delay = if self.current_attempt == 1 {
            self.initial_delay_ms
        } else {
            let exponential = self.initial_delay_ms as f64 * self.factor.powi((self.current_attempt - 1) as i32);
            exponential.min(self.max_delay_ms as f64) as u64
        };
        
        // Apply jitter if enabled (0-25% of base delay)
        let final_delay = if self.enable_jitter {
            let jitter_range = base_delay / 4;
            let jitter = rand::thread_rng().gen_range(0..=jitter_range);
            base_delay + jitter
        } else {
            base_delay
        };
        
        Some(Duration::from_millis(final_delay))
    }
    
    /// Get current attempt number
    pub fn current_attempt(&self) -> u32 {
        self.current_attempt
    }
    
    /// Get remaining attempts (returns None if unlimited)
    pub fn remaining_attempts(&self) -> Option<u32> {
        if self.max_attempts == 0 {
            None
        } else {
            Some(self.max_attempts.saturating_sub(self.current_attempt))
        }
    }
    
    /// Create backoff from transport config values
    pub fn from_config(max_attempts: u32, initial_delay_ms: u32) -> Self {
        Self {
            initial_delay_ms: initial_delay_ms as u64,
            max_delay_ms: 30000, // Fixed at 30s as per requirements
            factor: 2.0,
            max_attempts,
            enable_jitter: true,
            current_attempt: 0,
        }
    }
}

impl Default for ExponentialBackoff {
    fn default() -> Self {
        Self {
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            factor: 2.0,
            max_attempts: 10,
            enable_jitter: true,
            current_attempt: 0,
        }
    }
}

/// Helper to determine if an error is retryable
pub fn is_retryable_error(error: &crate::transport::TransportError) -> bool {
    use crate::transport::TransportError;
    
    match error {
        // Non-retryable errors (permanent failures)
        TransportError::ConfigError(_) |
        TransportError::PermissionDenied(_) |
        TransportError::InvalidData(_) |
        TransportError::NotImplemented(_) => false,
        
        // Retryable errors (temporary failures)
        TransportError::ConnectionFailed(_) |
        TransportError::NotConnected |
        TransportError::Timeout(_) |
        TransportError::IoError(_) |
        TransportError::HardwareError(_) |
        TransportError::AlreadyConnected |
        TransportError::BufferOverflow |
        TransportError::ProtocolError(_) |
        TransportError::ResourceUnavailable(_) |
        TransportError::Other(_) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_exponential_backoff_sequence() {
        let mut backoff = ExponentialBackoff::new()
            .with_initial_delay(1000)
            .with_max_delay(30000)
            .with_factor(2.0)
            .with_jitter(false) // Disable jitter for deterministic test
            .with_max_attempts(7);
        
        // Expected sequence: 1s, 2s, 4s, 8s, 16s, 30s (capped), 30s (capped)
        let expected = vec![1000, 2000, 4000, 8000, 16000, 30000, 30000];
        
        for expected_ms in expected {
            let delay = backoff.next_delay().expect("Should have delay");
            assert_eq!(delay.as_millis(), expected_ms);
        }
        
        // Should return None after max attempts
        assert!(backoff.next_delay().is_none());
    }
    
    #[test]
    fn test_backoff_with_jitter() {
        let mut backoff = ExponentialBackoff::new()
            .with_initial_delay(1000)
            .with_jitter(true)
            .with_max_attempts(5);
        
        for _ in 0..5 {
            let delay = backoff.next_delay().expect("Should have delay");
            let delay_ms = delay.as_millis();
            
            // With jitter, delay should be within expected range
            // For 1000ms base, jitter adds 0-250ms
            if backoff.current_attempt() == 1 {
                assert!(delay_ms >= 1000 && delay_ms <= 1250);
            }
        }
    }
    
    #[test]
    fn test_backoff_reset() {
        let mut backoff = ExponentialBackoff::new()
            .with_max_attempts(3);
        
        // Use up some attempts
        backoff.next_delay();
        backoff.next_delay();
        assert_eq!(backoff.current_attempt(), 2);
        
        // Reset should go back to 0
        backoff.reset();
        assert_eq!(backoff.current_attempt(), 0);
        assert!(backoff.should_retry());
    }
    
    #[test]
    fn test_unlimited_attempts() {
        let mut backoff = ExponentialBackoff::new()
            .with_max_attempts(0); // 0 means unlimited
        
        // Should always be able to retry
        for _ in 0..100 {
            assert!(backoff.should_retry());
            backoff.next_delay();
        }
        
        assert!(backoff.should_retry());
        assert_eq!(backoff.remaining_attempts(), None);
    }
    
    #[test]
    fn test_retryable_errors() {
        use crate::transport::TransportError;
        
        // Retryable errors
        assert!(is_retryable_error(&TransportError::ConnectionFailed("test".into())));
        assert!(is_retryable_error(&TransportError::NotConnected));
        assert!(is_retryable_error(&TransportError::Timeout("test".into())));
        
        // Non-retryable errors
        assert!(!is_retryable_error(&TransportError::ConfigError("test".into())));
        assert!(!is_retryable_error(&TransportError::PermissionDenied("test".into())));
    }
}