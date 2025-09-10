// Resource budget enforcement and monitoring

use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

/// Resource budget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceBudget {
    /// Maximum RAM usage in MB
    pub max_memory_mb: f64,
    
    /// Maximum CPU usage percentage during idle (0-100)
    pub max_idle_cpu_percent: f32,
    
    /// Maximum CPU usage percentage during active operation (0-100)
    pub max_active_cpu_percent: f32,
    
    /// Maximum startup time in milliseconds
    pub max_startup_ms: u64,
    
    /// Warning threshold as percentage of max (0-1)
    pub warning_threshold: f32,
    
    /// Critical threshold as percentage of max (0-1)
    pub critical_threshold: f32,
}

impl Default for ResourceBudget {
    fn default() -> Self {
        Self {
            max_memory_mb: 150.0,           // Task 17 requirement
            max_idle_cpu_percent: 2.0,      // Task 17 requirement
            max_active_cpu_percent: 10.0,   // Reasonable for active operation
            max_startup_ms: 2000,            // Task 17 requirement
            warning_threshold: 0.8,          // Warn at 80%
            critical_threshold: 0.95,        // Critical at 95%
        }
    }
}

impl ResourceBudget {
    /// Check if memory usage is within budget
    pub fn check_memory(&self, current_mb: f64) -> BudgetStatus {
        let ratio = current_mb / self.max_memory_mb;
        
        if ratio >= 1.0 {
            BudgetStatus::Exceeded
        } else if ratio >= self.critical_threshold as f64 {
            BudgetStatus::Critical
        } else if ratio >= self.warning_threshold as f64 {
            BudgetStatus::Warning
        } else {
            BudgetStatus::Ok
        }
    }
    
    /// Check if CPU usage is within budget
    pub fn check_cpu(&self, current_percent: f32, is_idle: bool) -> BudgetStatus {
        let max = if is_idle {
            self.max_idle_cpu_percent
        } else {
            self.max_active_cpu_percent
        };
        
        let ratio = current_percent / max;
        
        if ratio >= 1.0 {
            BudgetStatus::Exceeded
        } else if ratio >= self.critical_threshold {
            BudgetStatus::Critical
        } else if ratio >= self.warning_threshold {
            BudgetStatus::Warning
        } else {
            BudgetStatus::Ok
        }
    }
    
    /// Check if startup time is within budget
    pub fn check_startup(&self, startup_ms: u64) -> BudgetStatus {
        if startup_ms > self.max_startup_ms {
            BudgetStatus::Exceeded
        } else {
            BudgetStatus::Ok
        }
    }
}

/// Budget status levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetStatus {
    /// Within acceptable limits
    Ok,
    /// Approaching limit (warning threshold)
    Warning,
    /// Very close to limit (critical threshold)
    Critical,
    /// Exceeded the limit
    Exceeded,
}

/// Budget violation details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetViolation {
    /// Timestamp of violation
    pub timestamp: u64,
    
    /// Type of resource violated
    pub resource_type: ResourceType,
    
    /// Current value
    pub current_value: f64,
    
    /// Budget limit
    pub budget_limit: f64,
    
    /// Violation severity
    pub status: BudgetStatus,
    
    /// Additional context
    pub context: String,
}

impl BudgetViolation {
    /// Create a new violation record
    pub fn new(
        resource_type: ResourceType,
        current_value: f64,
        budget_limit: f64,
        status: BudgetStatus,
        context: String,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        Self {
            timestamp,
            resource_type,
            current_value,
            budget_limit,
            status,
            context,
        }
    }
    
    /// Format as log message
    pub fn to_log_message(&self) -> String {
        format!(
            "[{}] {} violation: {:.2} / {:.2} ({})",
            self.status.as_str(),
            self.resource_type.as_str(),
            self.current_value,
            self.budget_limit,
            self.context
        )
    }
}

/// Resource type for violations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Memory,
    CpuIdle,
    CpuActive,
    StartupTime,
}

impl ResourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceType::Memory => "Memory",
            ResourceType::CpuIdle => "CPU (Idle)",
            ResourceType::CpuActive => "CPU (Active)",
            ResourceType::StartupTime => "Startup Time",
        }
    }
}

impl BudgetStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            BudgetStatus::Ok => "OK",
            BudgetStatus::Warning => "WARNING",
            BudgetStatus::Critical => "CRITICAL",
            BudgetStatus::Exceeded => "EXCEEDED",
        }
    }
}

/// Budget enforcement engine
pub struct BudgetEnforcer {
    budget: ResourceBudget,
    violations: Vec<BudgetViolation>,
    startup_time: Option<Duration>,
    is_idle: bool,
}

impl BudgetEnforcer {
    /// Create a new budget enforcer
    pub fn new(budget: ResourceBudget) -> Self {
        Self {
            budget,
            violations: Vec::new(),
            startup_time: None,
            is_idle: true,
        }
    }
    
    /// Create with default budget
    pub fn default() -> Self {
        Self::new(ResourceBudget::default())
    }
    
    /// Set startup time
    pub fn set_startup_time(&mut self, duration: Duration) {
        self.startup_time = Some(duration);
        
        let status = self.budget.check_startup(duration.as_millis() as u64);
        if status != BudgetStatus::Ok {
            let violation = BudgetViolation::new(
                ResourceType::StartupTime,
                duration.as_millis() as f64,
                self.budget.max_startup_ms as f64,
                status,
                "Startup time exceeded budget".to_string(),
            );
            self.violations.push(violation);
        }
    }
    
    /// Set idle state
    pub fn set_idle(&mut self, is_idle: bool) {
        self.is_idle = is_idle;
    }
    
    /// Check current metrics against budget
    pub fn check_metrics(&mut self, memory_mb: f64, cpu_percent: f32) -> Vec<BudgetViolation> {
        let mut new_violations = Vec::new();
        
        // Check memory
        let memory_status = self.budget.check_memory(memory_mb);
        if memory_status != BudgetStatus::Ok {
            let violation = BudgetViolation::new(
                ResourceType::Memory,
                memory_mb,
                self.budget.max_memory_mb,
                memory_status,
                format!("{:.1} MB / {:.1} MB", memory_mb, self.budget.max_memory_mb),
            );
            self.violations.push(violation.clone());
            new_violations.push(violation);
        }
        
        // Check CPU
        let cpu_status = self.budget.check_cpu(cpu_percent, self.is_idle);
        if cpu_status != BudgetStatus::Ok {
            let resource_type = if self.is_idle {
                ResourceType::CpuIdle
            } else {
                ResourceType::CpuActive
            };
            
            let max_cpu = if self.is_idle {
                self.budget.max_idle_cpu_percent
            } else {
                self.budget.max_active_cpu_percent
            };
            
            let violation = BudgetViolation::new(
                resource_type,
                cpu_percent as f64,
                max_cpu as f64,
                cpu_status,
                format!("{:.1}% / {:.1}%", cpu_percent, max_cpu),
            );
            self.violations.push(violation.clone());
            new_violations.push(violation);
        }
        
        new_violations
    }
    
    /// Get all violations
    pub fn violations(&self) -> &[BudgetViolation] {
        &self.violations
    }
    
    /// Get recent violations (last N)
    pub fn recent_violations(&self, count: usize) -> Vec<&BudgetViolation> {
        let start = self.violations.len().saturating_sub(count);
        self.violations[start..].iter().collect()
    }
    
    /// Clear violation history
    pub fn clear_violations(&mut self) {
        self.violations.clear();
    }
    
    /// Get budget configuration
    pub fn budget(&self) -> &ResourceBudget {
        &self.budget
    }
    
    /// Update budget configuration
    pub fn set_budget(&mut self, budget: ResourceBudget) {
        self.budget = budget;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_budget_memory_check() {
        let budget = ResourceBudget::default();
        
        assert_eq!(budget.check_memory(50.0), BudgetStatus::Ok);
        assert_eq!(budget.check_memory(125.0), BudgetStatus::Warning); // 83%
        assert_eq!(budget.check_memory(145.0), BudgetStatus::Critical); // 97%
        assert_eq!(budget.check_memory(160.0), BudgetStatus::Exceeded);
    }
    
    #[test]
    fn test_budget_cpu_check() {
        let budget = ResourceBudget::default();
        
        // Idle CPU checks (max 2%)
        assert_eq!(budget.check_cpu(1.0, true), BudgetStatus::Ok);
        assert_eq!(budget.check_cpu(1.7, true), BudgetStatus::Warning); // 85%
        assert_eq!(budget.check_cpu(1.95, true), BudgetStatus::Critical); // 97.5%
        assert_eq!(budget.check_cpu(2.5, true), BudgetStatus::Exceeded);
        
        // Active CPU checks (max 10%)
        assert_eq!(budget.check_cpu(5.0, false), BudgetStatus::Ok);
        assert_eq!(budget.check_cpu(8.5, false), BudgetStatus::Warning); // 85%
        assert_eq!(budget.check_cpu(9.7, false), BudgetStatus::Critical); // 97%
        assert_eq!(budget.check_cpu(12.0, false), BudgetStatus::Exceeded);
    }
    
    #[test]
    fn test_budget_enforcer() {
        let mut enforcer = BudgetEnforcer::default();
        
        // Check within limits
        let violations = enforcer.check_metrics(100.0, 1.0);
        assert!(violations.is_empty());
        
        // Check memory warning
        let violations = enforcer.check_metrics(125.0, 1.0);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].resource_type, ResourceType::Memory);
        assert_eq!(violations[0].status, BudgetStatus::Warning);
        
        // Check CPU violation while idle
        let violations = enforcer.check_metrics(100.0, 3.0);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].resource_type, ResourceType::CpuIdle);
        assert_eq!(violations[0].status, BudgetStatus::Exceeded);
        
        // Check both violations
        let violations = enforcer.check_metrics(160.0, 3.0);
        assert_eq!(violations.len(), 2);
    }
    
    #[test]
    fn test_startup_time_check() {
        let mut enforcer = BudgetEnforcer::default();
        
        // Within budget
        enforcer.set_startup_time(Duration::from_millis(1500));
        assert_eq!(enforcer.violations().len(), 0);
        
        // Exceeded budget
        enforcer.set_startup_time(Duration::from_millis(2500));
        assert_eq!(enforcer.violations().len(), 1);
        assert_eq!(enforcer.violations()[0].resource_type, ResourceType::StartupTime);
    }
}