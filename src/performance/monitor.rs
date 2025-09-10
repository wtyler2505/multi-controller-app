// Performance monitoring system with runtime checks

use super::{
    metrics::{SystemMetrics, ProcessMetrics, ResourceUsage},
    budget::{ResourceBudget, BudgetEnforcer, BudgetViolation},
};
use crate::logging::{LogLevel, LoggingSystem};

use sysinfo::{System, Pid};

use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tracing::{info, warn, error};

/// Startup phase tracker for detailed performance monitoring
#[derive(Debug, Clone)]
pub struct StartupPhase {
    pub name: String,
    pub start_time: Instant,
    pub duration: Option<Duration>,
    pub description: String,
}

/// Detailed startup performance validator with phase tracking
#[derive(Debug)]
pub struct StartupValidator {
    start_time: Instant,
    phases: Vec<StartupPhase>,
    current_phase: Option<String>,
}

impl StartupValidator {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            phases: Vec::new(),
            current_phase: None,
        }
    }
    
    /// Begin a new startup phase
    pub fn begin_phase(&mut self, name: &str, description: &str) {
        // End current phase if exists
        if let Some(current) = &self.current_phase {
            self.end_current_phase();
        }
        
        let phase = StartupPhase {
            name: name.to_string(),
            start_time: Instant::now(),
            duration: None,
            description: description.to_string(),
        };
        
        self.phases.push(phase);
        self.current_phase = Some(name.to_string());
        
        tracing::info!("Starting phase: {} - {}", name, description);
    }
    
    /// End the current phase
    pub fn end_current_phase(&mut self) {
        if let Some(ref current_name) = self.current_phase {
            if let Some(phase) = self.phases.iter_mut().find(|p| p.name == *current_name) {
                let duration = phase.start_time.elapsed();
                phase.duration = Some(duration);
                tracing::info!("Completed phase: {} in {:?}", current_name, duration);
            }
        }
        self.current_phase = None;
    }
    
    /// Get total startup time
    pub fn total_time(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    /// Get phase breakdown
    pub fn get_phases(&self) -> &[StartupPhase] {
        &self.phases
    }
    
    /// Generate startup performance report
    pub fn generate_report(&self) -> StartupReport {
        let total_duration = self.total_time();
        let mut phase_times = Vec::new();
        let mut slowest_phase = None;
        let mut slowest_duration = Duration::from_millis(0);
        
        for phase in &self.phases {
            if let Some(duration) = phase.duration {
                phase_times.push((phase.name.clone(), duration));
                
                if duration > slowest_duration {
                    slowest_duration = duration;
                    slowest_phase = Some(phase.name.clone());
                }
            }
        }
        
        StartupReport {
            total_duration,
            phase_count: self.phases.len(),
            phase_times,
            slowest_phase,
            slowest_duration,
            exceeded_budget: total_duration > Duration::from_secs(2),
        }
    }
}

/// Startup performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupReport {
    pub total_duration: Duration,
    pub phase_count: usize,
    pub phase_times: Vec<(String, Duration)>,
    pub slowest_phase: Option<String>,
    pub slowest_duration: Duration,
    pub exceeded_budget: bool,
}

/// Performance monitor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfig {
    /// Polling interval in milliseconds
    pub poll_interval_ms: u64,
    
    /// Enable system metrics collection
    pub collect_system_metrics: bool,
    
    /// Enable process metrics collection
    pub collect_process_metrics: bool,
    
    /// Maximum number of samples to retain
    pub max_samples: usize,
    
    /// Enable automatic alerts
    pub enable_alerts: bool,
    
    /// Alert cooldown period in seconds
    pub alert_cooldown_seconds: u64,
    
    /// Resource budget configuration
    pub budget: ResourceBudget,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            poll_interval_ms: 1000,        // Poll every second
            collect_system_metrics: true,
            collect_process_metrics: true,
            max_samples: 3600,             // 1 hour at 1Hz
            enable_alerts: true,
            alert_cooldown_seconds: 60,    // Don't repeat alerts for 1 minute
            budget: ResourceBudget::default(),
        }
    }
}

/// Performance alert types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceAlert {
    /// Memory usage alert
    Memory {
        current_mb: f64,
        limit_mb: f64,
        severity: AlertSeverity,
    },
    
    /// CPU usage alert
    Cpu {
        current_percent: f32,
        limit_percent: f32,
        is_idle: bool,
        severity: AlertSeverity,
    },
    
    /// Startup time alert
    StartupTime {
        duration_ms: u64,
        limit_ms: u64,
    },
    
    /// Generic performance issue
    Generic {
        message: String,
        severity: AlertSeverity,
    },
}

/// Alert severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

impl PerformanceAlert {
    /// Convert to log message
    pub fn to_log_message(&self) -> String {
        match self {
            PerformanceAlert::Memory { current_mb, limit_mb, severity } => {
                format!(
                    "[{:?}] Memory usage: {:.1} MB / {:.1} MB limit",
                    severity, current_mb, limit_mb
                )
            }
            PerformanceAlert::Cpu { current_percent, limit_percent, is_idle, severity } => {
                let state = if *is_idle { "idle" } else { "active" };
                format!(
                    "[{:?}] CPU usage ({}): {:.1}% / {:.1}% limit",
                    severity, state, current_percent, limit_percent
                )
            }
            PerformanceAlert::StartupTime { duration_ms, limit_ms } => {
                format!(
                    "[Critical] Startup time: {} ms / {} ms limit",
                    duration_ms, limit_ms
                )
            }
            PerformanceAlert::Generic { message, severity } => {
                format!("[{:?}] {}", severity, message)
            }
        }
    }
}

/// Performance monitoring system
pub struct PerformanceMonitor {
    config: Arc<RwLock<MonitorConfig>>,
    system_metrics: Arc<RwLock<SystemMetrics>>,
    process_metrics: Arc<RwLock<ProcessMetrics>>,
    resource_usage: Arc<RwLock<ResourceUsage>>,
    budget_enforcer: Arc<RwLock<BudgetEnforcer>>,
    startup_validator: Arc<RwLock<StartupValidator>>,
    startup_time: Arc<RwLock<Option<Instant>>>,
    last_alert_time: Arc<RwLock<Option<Instant>>>,
    alert_callbacks: Arc<RwLock<Vec<Box<dyn Fn(PerformanceAlert) + Send + Sync>>>>,
    logging_system: Option<Arc<LoggingSystem>>,
    monitoring_task: Arc<RwLock<Option<tokio::task::JoinHandle<()>>>>,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(config: MonitorConfig) -> Self {
        let pid = std::process::id();
        
        Self {
            budget_enforcer: Arc::new(RwLock::new(BudgetEnforcer::new(config.budget.clone()))),
            config: Arc::new(RwLock::new(config)),
            system_metrics: Arc::new(RwLock::new(SystemMetrics::new())),
            process_metrics: Arc::new(RwLock::new(ProcessMetrics::new(pid))),
            resource_usage: Arc::new(RwLock::new(ResourceUsage::new())),
            startup_validator: Arc::new(RwLock::new(StartupValidator::new())),
            startup_time: Arc::new(RwLock::new(Some(Instant::now()))),
            last_alert_time: Arc::new(RwLock::new(None)),
            alert_callbacks: Arc::new(RwLock::new(Vec::new())),
            logging_system: None,
            monitoring_task: Arc::new(RwLock::new(None)),
        }
    }
    
    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(MonitorConfig::default())
    }
    
    /// Set logging system for integration
    pub fn set_logging_system(&mut self, logging: Arc<LoggingSystem>) {
        self.logging_system = Some(logging);
    }
    
    /// Register alert callback
    pub async fn register_alert_callback<F>(&self, callback: F)
    where
        F: Fn(PerformanceAlert) + Send + Sync + 'static,
    {
        let mut callbacks = self.alert_callbacks.write().await;
        callbacks.push(Box::new(callback));
    }
    
    /// Begin a startup phase
    pub async fn begin_startup_phase(&self, name: &str, description: &str) {
        let mut validator = self.startup_validator.write().await;
        validator.begin_phase(name, description);
    }
    
    /// End current startup phase
    pub async fn end_startup_phase(&self) {
        let mut validator = self.startup_validator.write().await;
        validator.end_current_phase();
    }
    
    /// Get startup report
    pub async fn get_startup_report(&self) -> StartupReport {
        let validator = self.startup_validator.read().await;
        validator.generate_report()
    }
    
    /// Validate startup performance
    pub async fn validate_startup_performance(&self) -> bool {
        // Use a write lock from the start to avoid deadlock
        let mut startup_instant = self.startup_time.write().await;
        
        if let Some(start) = *startup_instant {
            let elapsed = start.elapsed();
            let elapsed_ms = elapsed.as_millis() as u64;
            
            let config = self.config.read().await;
            let mut enforcer = self.budget_enforcer.write().await;
            enforcer.set_startup_time(elapsed);
            drop(enforcer); // Release lock before potential alert
            
            if elapsed_ms > config.budget.max_startup_ms {
                // Clear startup time before sending alert
                *startup_instant = None;
                drop(startup_instant); // Release lock before async operation
                
                let alert = PerformanceAlert::StartupTime {
                    duration_ms: elapsed_ms,
                    limit_ms: config.budget.max_startup_ms,
                };
                
                self.send_alert(alert).await;
                
                error!(
                    "Startup time {} ms exceeded {} ms budget",
                    elapsed_ms, config.budget.max_startup_ms
                );
                
                return false;
            }
            
            info!("Startup completed in {} ms", elapsed_ms);
            // Clear startup time after successful validation
            *startup_instant = None;
        }
        
        true
    }
    
    /// Start monitoring
    pub async fn start(&self) {
        let mut task_guard = self.monitoring_task.write().await;
        
        // Stop existing task if running
        if let Some(task) = task_guard.take() {
            task.abort();
        }
        
        let config = self.config.clone();
        let system_metrics = self.system_metrics.clone();
        let process_metrics = self.process_metrics.clone();
        let resource_usage = self.resource_usage.clone();
        let budget_enforcer = self.budget_enforcer.clone();
        let alert_callbacks = self.alert_callbacks.clone();
        let last_alert_time = self.last_alert_time.clone();
        let logging_system = self.logging_system.clone();
        
        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_millis(
                config.read().await.poll_interval_ms
            ));
            
            loop {
                interval.tick().await;
                
                let config = config.read().await.clone();
                
                // Collect metrics
                let (sys_metrics, proc_metrics) = Self::collect_metrics(
                    config.collect_system_metrics,
                    config.collect_process_metrics,
                ).await;
                
                // Update stored metrics
                if let Some(sys) = sys_metrics {
                    *system_metrics.write().await = sys;
                }
                
                if let Some(proc) = proc_metrics.clone() {
                    *process_metrics.write().await = proc.clone();
                    
                    // Update resource usage tracking
                    let mut usage = resource_usage.write().await;
                    usage.add_sample(proc.memory_mb(), proc.cpu_percent);
                    usage.trim_to_size(config.max_samples);
                    
                    // Check budget
                    let mut enforcer = budget_enforcer.write().await;
                    let violations = enforcer.check_metrics(proc.memory_mb(), proc.cpu_percent);
                    
                    // Send alerts for violations
                    if config.enable_alerts && !violations.is_empty() {
                        let should_alert = {
                            let last_alert = last_alert_time.read().await;
                            if let Some(last) = *last_alert {
                                last.elapsed().as_secs() >= config.alert_cooldown_seconds
                            } else {
                                true
                            }
                        };
                        
                        if should_alert {
                            *last_alert_time.write().await = Some(Instant::now());
                            
                            for violation in violations {
                                let alert = Self::violation_to_alert(&violation);
                                
                                // Log to system
                                if let Some(ref logging) = logging_system {
                                    let level = match alert {
                                        PerformanceAlert::Memory { severity, .. } |
                                        PerformanceAlert::Cpu { severity, .. } |
                                        PerformanceAlert::Generic { severity, .. } => {
                                            match severity {
                                                AlertSeverity::Info => LogLevel::Info,
                                                AlertSeverity::Warning => LogLevel::Warning,
                                                AlertSeverity::Critical => LogLevel::Error,
                                            }
                                        }
                                        PerformanceAlert::StartupTime { .. } => LogLevel::Error,
                                    };
                                    
                                    logging.log(
                                        level,
                                        "PerformanceMonitor",
                                        alert.to_log_message(),
                                        None::<Vec<u8>>,
                                    ).await;
                                }
                                
                                // Send to callbacks
                                let callbacks = alert_callbacks.read().await;
                                for callback in callbacks.iter() {
                                    callback(alert.clone());
                                }
                            }
                        }
                    }
                }
            }
        });
        
        *task_guard = Some(task);
        info!("Performance monitoring started");
    }
    
    /// Stop monitoring
    pub async fn stop(&self) {
        let mut task_guard = self.monitoring_task.write().await;
        if let Some(task) = task_guard.take() {
            task.abort();
            info!("Performance monitoring stopped");
        }
    }
    
    /// Collect current metrics using real system data
    async fn collect_metrics(
        collect_system: bool,
        collect_process: bool,
    ) -> (Option<SystemMetrics>, Option<ProcessMetrics>) {
        let mut sys_metrics = None;
        let mut proc_metrics = None;
        
        // Create system instance for data collection with specific Windows optimizations
        let mut system = System::new();
        
        if collect_system {
            // Refresh system information - use specific refresh for better Windows performance
            system.refresh_memory();
            system.refresh_cpu_all();
            
            let mut metrics = SystemMetrics::new();
            
            // Real system metrics using sysinfo with Windows-compatible calls
            metrics.total_memory = system.total_memory();
            metrics.available_memory = system.available_memory();
            
            // Ensure CPU info is available before accessing
            if !system.cpus().is_empty() {
                metrics.cpu_usage = system.global_cpu_usage();
                metrics.cpu_cores = system.cpus().len();
            } else {
                // Fallback for Windows systems where CPU info might not be immediately available
                tracing::debug!("CPU information not immediately available, using fallback");
                metrics.cpu_usage = 0.0;
                metrics.cpu_cores = num_cpus::get(); // Use num_cpus crate as fallback
            }
            
            // System uptime - Windows compatible
            metrics.uptime_seconds = System::uptime();
            
            tracing::debug!(
                "System metrics collected - Memory: {:.1} MB / {:.1} MB, CPU: {:.1}%, Cores: {}",
                (metrics.total_memory - metrics.available_memory) as f64 / (1024.0 * 1024.0),
                metrics.total_memory as f64 / (1024.0 * 1024.0),
                metrics.cpu_usage,
                metrics.cpu_cores
            );
            
            sys_metrics = Some(metrics);
        }
        
        if collect_process {
            let pid = sysinfo::Pid::from_u32(std::process::id());
            
            // Refresh only process information for better performance
            system.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]));
            
            let mut metrics = ProcessMetrics::new(std::process::id());
            
            if let Some(process) = system.process(pid) {
                // Real process metrics using sysinfo with proper Windows handling
                metrics.memory_bytes = process.memory();
                metrics.virtual_memory_bytes = process.virtual_memory();
                metrics.cpu_percent = process.cpu_usage();
                
                // Get thread count with Windows-safe handling
                let threads = process.tasks();
                metrics.thread_count = if let Some(tasks) = threads {
                    if tasks.is_empty() {
                        // On Windows, tasks() might be empty, fallback to a reasonable default
                        std::thread::available_parallelism()
                            .map(|n| n.get() as u32)
                            .unwrap_or(1)
                    } else {
                        tasks.len() as u32
                    }
                } else {
                    // No tasks info available
                    std::thread::available_parallelism()
                        .map(|n| n.get() as u32)
                        .unwrap_or(1)
                };
                
                // Process uptime calculation with better Windows time handling
                let start_time = process.start_time();
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                metrics.uptime_seconds = current_time.saturating_sub(start_time);
                
                tracing::debug!(
                    "Process metrics collected - Memory: {:.1} MB, Virtual: {:.1} MB, CPU: {:.1}%, Threads: {}",
                    metrics.memory_mb(),
                    metrics.virtual_memory_mb(),
                    metrics.cpu_percent,
                    metrics.thread_count
                );
            } else {
                // Enhanced error handling for Windows process lookup
                tracing::warn!(
                    "Failed to find current process (PID: {}) in system metrics - this may indicate a Windows permissions or timing issue",
                    std::process::id()
                );
                
                // Provide minimal fallback metrics to prevent system failure
                metrics.memory_bytes = 0;
                metrics.virtual_memory_bytes = 0;
                metrics.cpu_percent = 0.0;
                metrics.thread_count = 1; // Reasonable fallback
                metrics.uptime_seconds = 0;
            }
            
            proc_metrics = Some(metrics);
        }
        
        (sys_metrics, proc_metrics)
    }
    
    /// Convert violation to alert
    fn violation_to_alert(violation: &BudgetViolation) -> PerformanceAlert {
        use crate::performance::budget::{ResourceType, BudgetStatus};
        
        let severity = match violation.status {
            BudgetStatus::Ok => AlertSeverity::Info,
            BudgetStatus::Warning => AlertSeverity::Warning,
            BudgetStatus::Critical | BudgetStatus::Exceeded => AlertSeverity::Critical,
        };
        
        match violation.resource_type {
            ResourceType::Memory => PerformanceAlert::Memory {
                current_mb: violation.current_value,
                limit_mb: violation.budget_limit,
                severity,
            },
            ResourceType::CpuIdle => PerformanceAlert::Cpu {
                current_percent: violation.current_value as f32,
                limit_percent: violation.budget_limit as f32,
                is_idle: true,
                severity,
            },
            ResourceType::CpuActive => PerformanceAlert::Cpu {
                current_percent: violation.current_value as f32,
                limit_percent: violation.budget_limit as f32,
                is_idle: false,
                severity,
            },
            ResourceType::StartupTime => PerformanceAlert::StartupTime {
                duration_ms: violation.current_value as u64,
                limit_ms: violation.budget_limit as u64,
            },
        }
    }
    
    /// Send alert through all channels
    async fn send_alert(&self, alert: PerformanceAlert) {
        // Log the alert
        if let Some(ref logging) = self.logging_system {
            let level = match &alert {
                PerformanceAlert::Memory { severity, .. } |
                PerformanceAlert::Cpu { severity, .. } |
                PerformanceAlert::Generic { severity, .. } => {
                    match severity {
                        AlertSeverity::Info => LogLevel::Info,
                        AlertSeverity::Warning => LogLevel::Warning,
                        AlertSeverity::Critical => LogLevel::Error,
                    }
                }
                PerformanceAlert::StartupTime { .. } => LogLevel::Error,
            };
            
            logging.log(
                level,
                "PerformanceMonitor",
                alert.to_log_message(),
                None,
            ).await;
        }
        
        // Send to callbacks
        let callbacks = self.alert_callbacks.read().await;
        for callback in callbacks.iter() {
            callback(alert.clone());
        }
    }
    
    /// Get current metrics
    pub async fn current_metrics(&self) -> (SystemMetrics, ProcessMetrics) {
        let sys = self.system_metrics.read().await.clone();
        let proc = self.process_metrics.read().await.clone();
        (sys, proc)
    }
    
    /// Get resource usage history
    pub async fn resource_usage(&self) -> ResourceUsage {
        self.resource_usage.read().await.clone()
    }
    
    /// Get budget violations
    pub async fn violations(&self) -> Vec<BudgetViolation> {
        let enforcer = self.budget_enforcer.read().await;
        enforcer.violations().to_vec()
    }
    
    /// Update configuration
    pub async fn update_config(&self, config: MonitorConfig) {
        *self.config.write().await = config.clone();
        
        let mut enforcer = self.budget_enforcer.write().await;
        enforcer.set_budget(config.budget);
        
        // Restart monitoring with new config
        self.stop().await;
        self.start().await;
    }
    
    /// Set idle state
    pub async fn set_idle(&self, is_idle: bool) {
        let mut enforcer = self.budget_enforcer.write().await;
        enforcer.set_idle(is_idle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_monitor_creation() {
        let monitor = PerformanceMonitor::default();
        let (sys, proc) = monitor.current_metrics().await;
        
        assert!(sys.timestamp > 0);
        assert!(proc.timestamp > 0);
    }
    
    #[tokio::test]
    async fn test_startup_validation() {
        // Test 1: No startup time should pass
        {
            let mut config = MonitorConfig::default();
            config.budget.max_startup_ms = 5000; // 5 seconds
            let monitor = PerformanceMonitor::new(config);
            
            // Should pass as there's no startup time to validate
            let result = monitor.validate_startup_performance().await;
            assert!(result);
        }
        
        // Test 2: Valid startup time should pass
        {
            let mut config = MonitorConfig::default();
            config.budget.max_startup_ms = 5000; // 5 seconds
            let monitor = PerformanceMonitor::new(config);
            
            // Set a startup time
            *monitor.startup_time.write().await = Some(Instant::now());
            
            // Should pass as we're well under 5 seconds
            let result = monitor.validate_startup_performance().await;
            assert!(result);
            
            // Verify startup time was cleared
            assert!(monitor.startup_time.read().await.is_none());
        }
    }
    
    #[tokio::test]
    async fn test_alert_callback() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        
        let monitor = PerformanceMonitor::default();
        let alert_count = Arc::new(AtomicUsize::new(0));
        let count_clone = alert_count.clone();
        
        monitor.register_alert_callback(move |_alert| {
            count_clone.fetch_add(1, Ordering::SeqCst);
        }).await;
        
        // Trigger an alert
        let alert = PerformanceAlert::Generic {
            message: "Test alert".to_string(),
            severity: AlertSeverity::Info,
        };
        monitor.send_alert(alert).await;
        
        // Give callback time to execute
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        assert_eq!(alert_count.load(Ordering::SeqCst), 1);
    }
    
    #[tokio::test]
    async fn test_resource_usage_tracking() {
        let monitor = PerformanceMonitor::default();
        
        // Simulate some metrics collection
        let mut usage = monitor.resource_usage.write().await;
        usage.add_sample(100.0, 5.0);
        usage.add_sample(110.0, 6.0);
        usage.add_sample(105.0, 5.5);
        drop(usage);
        
        let usage = monitor.resource_usage().await;
        assert_eq!(usage.memory_samples.len(), 3);
        assert_eq!(usage.peak_memory_mb, 110.0);
        assert!((usage.avg_memory_mb - 105.0).abs() < 0.01);
    }
}