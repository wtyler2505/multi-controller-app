use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::{RwLock, broadcast, mpsc};
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use crate::device::{DeviceResult, DeviceError};

/// Global emergency stop controller
/// Provides a centralized mechanism to halt all device operations
pub struct EmergencyStop {
    /// Emergency stop flag
    stopped: Arc<AtomicBool>,
    
    /// Broadcast channel for stop events
    stop_tx: broadcast::Sender<StopReason>,
    
    /// Stop reason
    reason: Arc<RwLock<Option<StopReason>>>,
}

#[derive(Debug, Clone)]
pub enum StopReason {
    UserRequested,
    SafetyViolation(String),
    SystemError(String),
    Timeout,
    Shutdown,
}

impl EmergencyStop {
    pub fn new() -> Self {
        let (stop_tx, _) = broadcast::channel(100);
        
        EmergencyStop {
            stopped: Arc::new(AtomicBool::new(false)),
            stop_tx,
            reason: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Trigger emergency stop
    pub async fn trigger(&self, reason: StopReason) {
        self.stopped.store(true, Ordering::SeqCst);
        *self.reason.write().await = Some(reason.clone());
        let _ = self.stop_tx.send(reason);
        
        tracing::warn!("EMERGENCY STOP triggered: {:?}", self.reason.read().await);
    }
    
    /// Reset emergency stop
    pub async fn reset(&self) {
        self.stopped.store(false, Ordering::SeqCst);
        *self.reason.write().await = None;
        
        tracing::info!("Emergency stop reset");
    }
    
    /// Check if stopped
    pub fn is_stopped(&self) -> bool {
        self.stopped.load(Ordering::SeqCst)
    }
    
    /// Get stop reason
    pub async fn get_reason(&self) -> Option<StopReason> {
        self.reason.read().await.clone()
    }
    
    /// Subscribe to stop events
    pub fn subscribe(&self) -> broadcast::Receiver<StopReason> {
        self.stop_tx.subscribe()
    }
    
    /// Create a guard that checks stop status
    pub fn guard(&self) -> EmergencyStopGuard {
        EmergencyStopGuard {
            stopped: self.stopped.clone(),
        }
    }
}

/// Guard for checking emergency stop status
pub struct EmergencyStopGuard {
    stopped: Arc<AtomicBool>,
}

impl EmergencyStopGuard {
    /// Check if operations should stop
    pub fn should_stop(&self) -> bool {
        self.stopped.load(Ordering::Relaxed)
    }
    
    /// Ensure not stopped, return error if stopped
    pub fn ensure_running(&self) -> DeviceResult<()> {
        if self.should_stop() {
            Err(DeviceError::SafetyViolation("Emergency stop is active".into()))
        } else {
            Ok(())
        }
    }
}

/// Safety controller for enforcing limits and constraints
pub struct SafetyController {
    /// Emergency stop
    emergency_stop: Arc<EmergencyStop>,
    
    /// Rate limiters per operation type
    rate_limiters: Arc<RwLock<HashMap<String, Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>>>>,
    
    /// Safety limits
    limits: Arc<RwLock<SafetyLimits>>,
    
    /// Violation counter
    violations: Arc<AtomicU64>,
}

use std::collections::HashMap;
use governor::state::{NotKeyed, InMemoryState};
use governor::clock::DefaultClock;

/// Safety limits configuration
#[derive(Debug, Clone)]
pub struct SafetyLimits {
    /// Maximum PWM duty cycle (0-100)
    pub max_pwm_duty: u8,
    
    /// Maximum PWM frequency (Hz)
    pub max_pwm_frequency: u32,
    
    /// Maximum current draw (mA)
    pub max_current_ma: u32,
    
    /// Maximum temperature (Celsius)
    pub max_temperature_c: f32,
    
    /// Minimum command interval (ms)
    pub min_command_interval_ms: u64,
    
    /// Maximum consecutive errors before stop
    pub max_consecutive_errors: u32,
    
    /// Enable automatic recovery
    pub auto_recovery: bool,
}

impl Default for SafetyLimits {
    fn default() -> Self {
        SafetyLimits {
            max_pwm_duty: 95,           // 95% max duty cycle
            max_pwm_frequency: 50_000,   // 50 kHz max
            max_current_ma: 2000,        // 2A max
            max_temperature_c: 85.0,     // 85°C max
            min_command_interval_ms: 10, // 10ms between commands
            max_consecutive_errors: 10,  // 10 errors before stop
            auto_recovery: true,
        }
    }
}

impl SafetyController {
    pub fn new(emergency_stop: Arc<EmergencyStop>) -> Self {
        SafetyController {
            emergency_stop,
            rate_limiters: Arc::new(RwLock::new(HashMap::new())),
            limits: Arc::new(RwLock::new(SafetyLimits::default())),
            violations: Arc::new(AtomicU64::new(0)),
        }
    }
    
    /// Check PWM parameters against safety limits
    pub async fn check_pwm(&self, duty_cycle: u8, frequency: u32) -> DeviceResult<()> {
        let limits = self.limits.read().await;
        
        if duty_cycle > limits.max_pwm_duty {
            self.record_violation().await;
            return Err(DeviceError::SafetyViolation(
                format!("PWM duty cycle {} exceeds limit {}", duty_cycle, limits.max_pwm_duty)
            ));
        }
        
        if frequency > limits.max_pwm_frequency {
            self.record_violation().await;
            return Err(DeviceError::SafetyViolation(
                format!("PWM frequency {} Hz exceeds limit {} Hz", frequency, limits.max_pwm_frequency)
            ));
        }
        
        Ok(())
    }
    
    /// Check rate limit for an operation
    pub async fn check_rate_limit(&self, operation: &str) -> DeviceResult<()> {
        let mut limiters = self.rate_limiters.write().await;
        
        let limiter = limiters.entry(operation.to_string()).or_insert_with(|| {
            // Default: 100 operations per second
            Arc::new(RateLimiter::direct(Quota::per_second(nonzero!(100u32))))
        });
        
        match limiter.check() {
            Ok(_) => Ok(()),
            Err(_) => {
                self.record_violation().await;
                Err(DeviceError::RateLimitExceeded)
            }
        }
    }
    
    /// Set custom rate limit for an operation
    pub async fn set_rate_limit(&self, operation: &str, per_second: u32) {
        let mut limiters = self.rate_limiters.write().await;
        
        limiters.insert(
            operation.to_string(),
            Arc::new(RateLimiter::direct(Quota::per_second(
                std::num::NonZeroU32::new(per_second).unwrap_or(nonzero!(1u32))
            )))
        );
    }
    
    /// Update safety limits
    pub async fn update_limits(&self, limits: SafetyLimits) {
        *self.limits.write().await = limits;
        tracing::info!("Safety limits updated");
    }
    
    /// Record a safety violation
    async fn record_violation(&self) {
        let count = self.violations.fetch_add(1, Ordering::SeqCst) + 1;
        let limits = self.limits.read().await;
        
        tracing::warn!("Safety violation #{}", count);
        
        // Check if we should trigger emergency stop
        if count >= limits.max_consecutive_errors as u64 {
            self.emergency_stop.trigger(
                StopReason::SafetyViolation(format!("{} violations exceeded limit", count))
            ).await;
        }
    }
    
    /// Reset violation counter
    pub fn reset_violations(&self) {
        self.violations.store(0, Ordering::SeqCst);
    }
    
    /// Get violation count
    pub fn violation_count(&self) -> u64 {
        self.violations.load(Ordering::SeqCst)
    }
}

/// Hot-plug monitor for device detection
pub struct HotPlugMonitor {
    watcher_tx: mpsc::UnboundedSender<HotPlugEvent>,
}

#[derive(Debug, Clone)]
pub enum HotPlugEvent {
    DeviceAdded(String),
    DeviceRemoved(String),
}

impl HotPlugMonitor {
    pub fn new() -> (Self, mpsc::UnboundedReceiver<HotPlugEvent>) {
        let (tx, rx) = mpsc::unbounded_channel();
        
        (HotPlugMonitor { watcher_tx: tx }, rx)
    }
    
    /// Report device addition
    pub fn device_added(&self, device_id: String) {
        let _ = self.watcher_tx.send(HotPlugEvent::DeviceAdded(device_id));
    }
    
    /// Report device removal
    pub fn device_removed(&self, device_id: String) {
        let _ = self.watcher_tx.send(HotPlugEvent::DeviceRemoved(device_id));
    }
}