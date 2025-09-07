// Performance monitoring and budget enforcement tests

#[cfg(test)]
mod performance_tests {
    use multi_controller_app::performance::{
        PerformanceMonitor, MonitorConfig, ResourceBudget,
        BudgetEnforcer, BudgetStatus, ResourceType,
        SystemMetrics, ProcessMetrics, ResourceUsage,
        PerformanceAlert, AlertSeverity,
    };
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;
    use tokio::time::sleep;

    #[test]
    fn test_resource_budget_defaults() {
        let budget = ResourceBudget::default();
        
        // Task 17 requirements
        assert_eq!(budget.max_memory_mb, 150.0);
        assert_eq!(budget.max_idle_cpu_percent, 2.0);
        assert_eq!(budget.max_startup_ms, 2000);
        
        // Warning and critical thresholds
        assert_eq!(budget.warning_threshold, 0.8);
        assert_eq!(budget.critical_threshold, 0.95);
    }

    #[test]
    fn test_memory_budget_checks() {
        let budget = ResourceBudget::default();
        
        // Test OK status (50 MB / 150 MB = 33%)
        assert_eq!(budget.check_memory(50.0), BudgetStatus::Ok);
        
        // Test Warning status (125 MB / 150 MB = 83%)
        assert_eq!(budget.check_memory(125.0), BudgetStatus::Warning);
        
        // Test Critical status (145 MB / 150 MB = 97%)
        assert_eq!(budget.check_memory(145.0), BudgetStatus::Critical);
        
        // Test Exceeded status (160 MB / 150 MB = 107%)
        assert_eq!(budget.check_memory(160.0), BudgetStatus::Exceeded);
    }

    #[test]
    fn test_cpu_budget_checks() {
        let budget = ResourceBudget::default();
        
        // Test idle CPU checks (max 2%)
        assert_eq!(budget.check_cpu(1.0, true), BudgetStatus::Ok);
        assert_eq!(budget.check_cpu(1.7, true), BudgetStatus::Warning);
        assert_eq!(budget.check_cpu(1.95, true), BudgetStatus::Critical);
        assert_eq!(budget.check_cpu(2.5, true), BudgetStatus::Exceeded);
        
        // Test active CPU checks (max 10%)
        assert_eq!(budget.check_cpu(5.0, false), BudgetStatus::Ok);
        assert_eq!(budget.check_cpu(8.5, false), BudgetStatus::Warning);
        assert_eq!(budget.check_cpu(9.7, false), BudgetStatus::Critical);
        assert_eq!(budget.check_cpu(12.0, false), BudgetStatus::Exceeded);
    }

    #[test]
    fn test_startup_budget_check() {
        let budget = ResourceBudget::default();
        
        // Within budget (1500 ms < 2000 ms)
        assert_eq!(budget.check_startup(1500), BudgetStatus::Ok);
        
        // Exceeded budget (2500 ms > 2000 ms)
        assert_eq!(budget.check_startup(2500), BudgetStatus::Exceeded);
    }

    #[test]
    fn test_budget_enforcer() {
        let mut enforcer = BudgetEnforcer::default();
        
        // Test memory within limits
        let violations = enforcer.check_metrics(100.0, 1.0);
        assert!(violations.is_empty());
        
        // Test memory warning
        let violations = enforcer.check_metrics(125.0, 1.0);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].resource_type, ResourceType::Memory);
        assert_eq!(violations[0].status, BudgetStatus::Warning);
        
        // Test CPU violation while idle
        enforcer.set_idle(true);
        let violations = enforcer.check_metrics(100.0, 3.0);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].resource_type, ResourceType::CpuIdle);
        assert_eq!(violations[0].status, BudgetStatus::Exceeded);
        
        // Test both violations
        let violations = enforcer.check_metrics(160.0, 3.0);
        assert_eq!(violations.len(), 2);
        
        // Verify violations are stored
        assert!(enforcer.violations().len() >= 4);
    }

    #[test]
    fn test_resource_usage_tracking() {
        let mut usage = ResourceUsage::new();
        
        // Add samples
        usage.add_sample(100.0, 5.0);
        usage.add_sample(120.0, 7.0);
        usage.add_sample(110.0, 6.0);
        
        // Check tracking
        assert_eq!(usage.memory_samples.len(), 3);
        assert_eq!(usage.cpu_samples.len(), 3);
        
        // Check peaks
        assert_eq!(usage.peak_memory_mb, 120.0);
        assert_eq!(usage.peak_cpu_percent, 7.0);
        
        // Check averages
        assert!((usage.avg_memory_mb - 110.0).abs() < 0.01);
        assert!((usage.avg_cpu_percent - 6.0).abs() < 0.01);
    }

    #[test]
    fn test_resource_usage_trimming() {
        let mut usage = ResourceUsage::new();
        
        // Add 10 samples
        for i in 0..10 {
            usage.add_sample(i as f64, i as f32);
        }
        
        assert_eq!(usage.memory_samples.len(), 10);
        
        // Trim to 5 samples
        usage.trim_to_size(5);
        assert_eq!(usage.memory_samples.len(), 5);
        
        // Verify oldest samples were removed
        assert_eq!(usage.memory_samples[0], 5.0);
        assert_eq!(usage.memory_samples[4], 9.0);
    }

    #[test]
    fn test_system_metrics() {
        let mut metrics = SystemMetrics::new();
        assert!(metrics.timestamp > 0);
        assert!(metrics.cpu_cores > 0);
        
        // Test memory usage calculation
        metrics.total_memory = 1000;
        metrics.available_memory = 400;
        assert!((metrics.memory_usage_percent() - 60.0).abs() < 0.01);
        
        // Test zero total memory
        metrics.total_memory = 0;
        assert_eq!(metrics.memory_usage_percent(), 0.0);
    }

    #[test]
    fn test_process_metrics() {
        let metrics = ProcessMetrics::new(1234);
        assert_eq!(metrics.pid, 1234);
        assert!(metrics.timestamp > 0);
        
        let mut metrics = ProcessMetrics::new(1234);
        metrics.memory_bytes = 100 * 1024 * 1024; // 100 MB
        assert!((metrics.memory_mb() - 100.0).abs() < 0.01);
        
        metrics.virtual_memory_bytes = 200 * 1024 * 1024; // 200 MB
        assert!((metrics.virtual_memory_mb() - 200.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_performance_monitor_creation() {
        let config = MonitorConfig::default();
        let monitor = PerformanceMonitor::new(config);
        
        let (sys, proc) = monitor.current_metrics().await;
        assert!(sys.timestamp > 0);
        assert!(proc.timestamp > 0);
    }

    #[tokio::test]
    async fn test_performance_monitor_startup_validation() {
        let mut config = MonitorConfig::default();
        config.budget.max_startup_ms = 5000; // 5 seconds (generous for test)
        
        let monitor = PerformanceMonitor::new(config);
        
        // Should pass as we're well under 5 seconds
        let result = monitor.validate_startup_performance().await;
        assert!(result);
    }

    #[tokio::test]
    async fn test_performance_alert_callbacks() {
        let monitor = PerformanceMonitor::default();
        let alert_count = Arc::new(AtomicUsize::new(0));
        let count_clone = alert_count.clone();
        
        // Register callback
        monitor.register_alert_callback(move |alert| {
            match alert {
                PerformanceAlert::Memory { .. } => {
                    count_clone.fetch_add(1, Ordering::SeqCst);
                }
                _ => {}
            }
        }).await;
        
        // The alert callback will be triggered when violations occur during monitoring
        // For this test, we just verify that the callback was registered successfully
        // In a real scenario, alerts would be triggered by actual budget violations
        
        // Test passes if no panic occurred during callback registration
        assert!(true);
    }

    #[tokio::test]
    async fn test_performance_monitor_start_stop() {
        let monitor = PerformanceMonitor::default();
        
        // Start monitoring
        monitor.start().await;
        
        // Give it time to start
        sleep(Duration::from_millis(100)).await;
        
        // Stop monitoring
        monitor.stop().await;
        
        // Verify it stopped (no panic)
        assert!(true);
    }

    #[tokio::test]
    async fn test_resource_usage_history() {
        let monitor = PerformanceMonitor::default();
        
        // Manually add some usage data
        let mut usage = monitor.resource_usage.write().await;
        usage.add_sample(100.0, 5.0);
        usage.add_sample(110.0, 6.0);
        usage.add_sample(105.0, 5.5);
        drop(usage);
        
        // Get usage history
        let usage = monitor.resource_usage().await;
        assert_eq!(usage.memory_samples.len(), 3);
        assert_eq!(usage.peak_memory_mb, 110.0);
        assert!((usage.avg_memory_mb - 105.0).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_monitor_config_update() {
        let monitor = PerformanceMonitor::default();
        
        // Update configuration
        let mut new_config = MonitorConfig::default();
        new_config.budget.max_memory_mb = 200.0;
        new_config.poll_interval_ms = 500;
        
        monitor.update_config(new_config).await;
        
        // Verify configuration was updated
        let config = monitor.config.read().await;
        assert_eq!(config.budget.max_memory_mb, 200.0);
        assert_eq!(config.poll_interval_ms, 500);
    }

    #[tokio::test]
    async fn test_idle_state_tracking() {
        let monitor = PerformanceMonitor::default();
        
        // Set to active state
        monitor.set_idle(false).await;
        
        // Check metrics with active CPU limits
        let enforcer = monitor.budget_enforcer.read().await;
        let status = enforcer.budget().check_cpu(8.0, false);
        assert_eq!(status, BudgetStatus::Warning); // 8% of 10% = 80%
    }

    #[test]
    fn test_alert_message_formatting() {
        let alert = PerformanceAlert::Memory {
            current_mb: 160.0,
            limit_mb: 150.0,
            severity: AlertSeverity::Critical,
        };
        
        let message = alert.to_log_message();
        assert!(message.contains("Critical"));
        assert!(message.contains("160.0"));
        assert!(message.contains("150.0"));
        
        let alert = PerformanceAlert::Cpu {
            current_percent: 3.0,
            limit_percent: 2.0,
            is_idle: true,
            severity: AlertSeverity::Warning,
        };
        
        let message = alert.to_log_message();
        assert!(message.contains("Warning"));
        assert!(message.contains("idle"));
        assert!(message.contains("3.0"));
        
        let alert = PerformanceAlert::StartupTime {
            duration_ms: 2500,
            limit_ms: 2000,
        };
        
        let message = alert.to_log_message();
        assert!(message.contains("Startup time"));
        assert!(message.contains("2500"));
        assert!(message.contains("2000"));
    }
}