// Performance monitoring and enforcement module

pub mod monitor;
pub mod metrics;
pub mod budget;
pub mod startup;
pub mod profiler;

pub use monitor::{PerformanceMonitor, MonitorConfig, PerformanceAlert, AlertSeverity};
pub use metrics::{SystemMetrics, ProcessMetrics, ResourceUsage};
pub use budget::{ResourceBudget, BudgetEnforcer, BudgetViolation};
pub use startup::{StartupTracker, StartupPhase, StartupReport, PhaseTimings, tracker};
pub use profiler::{Profiler, profiler, FlameGraph, FunctionStats};