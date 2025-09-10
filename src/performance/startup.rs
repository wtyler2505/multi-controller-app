//! Enterprise-grade startup phase tracking with microsecond precision
//! 
//! This module provides detailed startup phase tracking and analysis for
//! identifying bottlenecks and ensuring the 2-second startup budget is met.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use tracing::{info, warn, error};

/// Startup phases with detailed tracking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StartupPhase {
    /// Pre-initialization (before main)
    PreInit,
    /// Core system initialization
    CoreInit,
    /// Device manager setup
    DeviceManagerInit,
    /// Transport layer initialization
    TransportInit,
    /// UI framework initialization
    UiInit,
    /// Plugin system loading
    PluginLoad,
    /// Telemetry system setup
    TelemetryInit,
    /// Performance monitor initialization
    PerformanceInit,
    /// Application ready
    Ready,
}

impl StartupPhase {
    /// Get phase display name
    pub fn display_name(&self) -> &str {
        match self {
            Self::PreInit => "Pre-initialization",
            Self::CoreInit => "Core initialization",
            Self::DeviceManagerInit => "Device manager",
            Self::TransportInit => "Transport layer",
            Self::UiInit => "UI framework",
            Self::PluginLoad => "Plugin loading",
            Self::TelemetryInit => "Telemetry system",
            Self::PerformanceInit => "Performance monitor",
            Self::Ready => "Application ready",
        }
    }
    
    /// Get expected duration budget for phase
    pub fn expected_duration_ms(&self) -> u64 {
        match self {
            Self::PreInit => 50,
            Self::CoreInit => 100,
            Self::DeviceManagerInit => 200,
            Self::TransportInit => 300,
            Self::UiInit => 500,
            Self::PluginLoad => 400,
            Self::TelemetryInit => 100,
            Self::PerformanceInit => 50,
            Self::Ready => 50,
        }
    }
}

/// Timing information for a single phase
#[derive(Debug, Clone)]
pub struct PhaseTimings {
    pub phase: StartupPhase,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub duration: Option<Duration>,
    pub sub_timings: HashMap<String, Duration>,
    pub metadata: HashMap<String, String>,
}

impl PhaseTimings {
    /// Create new phase timing
    pub fn new(phase: StartupPhase) -> Self {
        Self {
            phase,
            start_time: Instant::now(),
            end_time: None,
            duration: None,
            sub_timings: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Complete the phase
    pub fn complete(&mut self) {
        let end = Instant::now();
        self.end_time = Some(end);
        self.duration = Some(end - self.start_time);
    }
    
    /// Add sub-timing
    pub fn add_sub_timing(&mut self, name: String, duration: Duration) {
        self.sub_timings.insert(name, duration);
    }
    
    /// Add metadata
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Get duration in microseconds
    pub fn duration_micros(&self) -> u128 {
        self.duration.map(|d| d.as_micros()).unwrap_or(0)
    }
    
    /// Get duration in milliseconds
    pub fn duration_millis(&self) -> u128 {
        self.duration.map(|d| d.as_millis()).unwrap_or(0)
    }
    
    /// Check if phase exceeded budget
    pub fn is_over_budget(&self) -> bool {
        if let Some(duration) = self.duration {
            duration.as_millis() as u64 > self.phase.expected_duration_ms()
        } else {
            false
        }
    }
}

/// Startup performance tracker with enterprise features
pub struct StartupTracker {
    phases: Arc<RwLock<HashMap<StartupPhase, PhaseTimings>>>,
    phase_order: Arc<RwLock<Vec<StartupPhase>>>,
    start_time: Instant,
    total_duration: Arc<RwLock<Option<Duration>>>,
    phase_callbacks: Arc<RwLock<Vec<Box<dyn Fn(&PhaseTimings) + Send + Sync>>>>,
    current_phase: Arc<RwLock<Option<StartupPhase>>>,
    startup_budget_ms: u64,
}

impl StartupTracker {
    /// Create new startup tracker
    pub fn new() -> Self {
        Self::with_budget(2000) // 2 second default budget
    }
    
    /// Create with custom budget
    pub fn with_budget(budget_ms: u64) -> Self {
        Self {
            phases: Arc::new(RwLock::new(HashMap::new())),
            phase_order: Arc::new(RwLock::new(Vec::new())),
            start_time: Instant::now(),
            total_duration: Arc::new(RwLock::new(None)),
            phase_callbacks: Arc::new(RwLock::new(Vec::new())),
            current_phase: Arc::new(RwLock::new(None)),
            startup_budget_ms: budget_ms,
        }
    }
    
    /// Start a phase
    pub fn start_phase(&self, phase: StartupPhase) {
        let mut phases = self.phases.write();
        let mut order = self.phase_order.write();
        let mut current = self.current_phase.write();
        
        // Complete previous phase if exists
        if let Some(prev_phase) = *current {
            if let Some(timing) = phases.get_mut(&prev_phase) {
                if timing.end_time.is_none() {
                    timing.complete();
                    
                    // Log if over budget
                    if timing.is_over_budget() {
                        warn!(
                            "Startup phase '{}' took {}ms (budget: {}ms)",
                            prev_phase.display_name(),
                            timing.duration_millis(),
                            prev_phase.expected_duration_ms()
                        );
                    }
                }
            }
        }
        
        // Start new phase
        let timing = PhaseTimings::new(phase);
        phases.insert(phase, timing.clone());
        order.push(phase);
        *current = Some(phase);
        
        info!("Starting phase: {}", phase.display_name());
        
        // Notify callbacks
        let callbacks = self.phase_callbacks.read();
        for callback in callbacks.iter() {
            callback(&timing);
        }
    }
    
    /// Complete current phase
    pub fn complete_phase(&self, phase: StartupPhase) {
        let mut phases = self.phases.write();
        let mut current = self.current_phase.write();
        
        if let Some(timing) = phases.get_mut(&phase) {
            timing.complete();
            
            let duration_ms = timing.duration_millis();
            let budget_ms = phase.expected_duration_ms();
            
            if timing.is_over_budget() {
                warn!(
                    "Phase '{}' exceeded budget: {}ms > {}ms",
                    phase.display_name(),
                    duration_ms,
                    budget_ms
                );
            } else {
                info!(
                    "Phase '{}' completed: {}ms / {}ms",
                    phase.display_name(),
                    duration_ms,
                    budget_ms
                );
            }
            
            // Notify callbacks
            let callbacks = self.phase_callbacks.read();
            for callback in callbacks.iter() {
                callback(timing);
            }
        }
        
        // Clear current phase if it matches
        if *current == Some(phase) {
            *current = None;
        }
    }
    
    /// Add sub-timing to current phase
    pub fn add_sub_timing(&self, name: String, duration: Duration) {
        let current = self.current_phase.read();
        if let Some(phase) = *current {
            let mut phases = self.phases.write();
            if let Some(timing) = phases.get_mut(&phase) {
                timing.add_sub_timing(name, duration);
            }
        }
    }
    
    /// Add metadata to current phase
    pub fn add_metadata(&self, key: String, value: String) {
        let current = self.current_phase.read();
        if let Some(phase) = *current {
            let mut phases = self.phases.write();
            if let Some(timing) = phases.get_mut(&phase) {
                timing.add_metadata(key, value);
            }
        }
    }
    
    /// Complete startup tracking
    pub fn complete_startup(&self) {
        // Complete any remaining phase
        let current = self.current_phase.read().clone();
        if let Some(phase) = current {
            self.complete_phase(phase);
        }
        
        // Calculate total duration
        let total = Instant::now() - self.start_time;
        *self.total_duration.write() = Some(total);
        
        let total_ms = total.as_millis() as u64;
        
        if total_ms > self.startup_budget_ms {
            error!(
                "Startup exceeded budget: {}ms > {}ms",
                total_ms, self.startup_budget_ms
            );
        } else {
            info!(
                "Startup completed successfully: {}ms / {}ms",
                total_ms, self.startup_budget_ms
            );
        }
    }
    
    /// Register phase callback
    pub fn register_callback<F>(&self, callback: F)
    where
        F: Fn(&PhaseTimings) + Send + Sync + 'static,
    {
        self.phase_callbacks.write().push(Box::new(callback));
    }
    
    /// Generate startup report
    pub fn generate_report(&self) -> StartupReport {
        let phases = self.phases.read();
        let order = self.phase_order.read();
        let total_duration = self.total_duration.read();
        
        let mut phase_reports = Vec::new();
        let mut total_ms = 0u128;
        let mut bottlenecks = Vec::new();
        
        for phase in order.iter() {
            if let Some(timing) = phases.get(phase) {
                let duration_ms = timing.duration_millis();
                total_ms += duration_ms;
                
                let phase_report = PhaseReport {
                    phase: *phase,
                    duration_ms,
                    duration_micros: timing.duration_micros(),
                    budget_ms: phase.expected_duration_ms(),
                    over_budget: timing.is_over_budget(),
                    sub_timings: timing.sub_timings.clone(),
                    metadata: timing.metadata.clone(),
                };
                
                if phase_report.over_budget {
                    bottlenecks.push(phase_report.clone());
                }
                
                phase_reports.push(phase_report);
            }
        }
        
        // Sort bottlenecks by impact
        bottlenecks.sort_by_key(|b| std::cmp::Reverse(b.duration_ms - b.budget_ms as u128));
        
        StartupReport {
            total_duration_ms: total_duration.map(|d| d.as_millis() as u64).unwrap_or(total_ms as u64),
            budget_ms: self.startup_budget_ms,
            budget_exceeded: total_ms as u64 > self.startup_budget_ms,
            phases: phase_reports,
            bottlenecks: bottlenecks.clone(),
            recommendations: generate_recommendations(&bottlenecks),
        }
    }
}

/// Phase performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseReport {
    pub phase: StartupPhase,
    pub duration_ms: u128,
    pub duration_micros: u128,
    pub budget_ms: u64,
    pub over_budget: bool,
    pub sub_timings: HashMap<String, Duration>,
    pub metadata: HashMap<String, String>,
}

/// Complete startup performance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupReport {
    pub total_duration_ms: u64,
    pub budget_ms: u64,
    pub budget_exceeded: bool,
    pub phases: Vec<PhaseReport>,
    pub bottlenecks: Vec<PhaseReport>,
    pub recommendations: Vec<String>,
}

impl StartupReport {
    /// Export as JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut report = String::new();
        
        report.push_str("# Startup Performance Report\n\n");
        
        // Summary
        report.push_str("## Summary\n\n");
        report.push_str(&format!("- **Total Duration**: {} ms\n", self.total_duration_ms));
        report.push_str(&format!("- **Budget**: {} ms\n", self.budget_ms));
        report.push_str(&format!("- **Status**: {}\n\n", 
            if self.budget_exceeded { "❌ EXCEEDED" } else { "✅ PASSED" }
        ));
        
        // Phase breakdown
        report.push_str("## Phase Breakdown\n\n");
        report.push_str("| Phase | Duration (ms) | Budget (ms) | Status |\n");
        report.push_str("|-------|---------------|-------------|--------|\n");
        
        for phase in &self.phases {
            report.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                phase.phase.display_name(),
                phase.duration_ms,
                phase.budget_ms,
                if phase.over_budget { "❌" } else { "✅" }
            ));
        }
        
        // Bottlenecks
        if !self.bottlenecks.is_empty() {
            report.push_str("\n## Bottlenecks\n\n");
            for bottleneck in &self.bottlenecks {
                report.push_str(&format!(
                    "- **{}**: {} ms over budget\n",
                    bottleneck.phase.display_name(),
                    bottleneck.duration_ms - bottleneck.budget_ms as u128
                ));
            }
        }
        
        // Recommendations
        if !self.recommendations.is_empty() {
            report.push_str("\n## Recommendations\n\n");
            for rec in &self.recommendations {
                report.push_str(&format!("- {}\n", rec));
            }
        }
        
        report
    }
}

/// Generate recommendations based on bottlenecks
fn generate_recommendations(bottlenecks: &[PhaseReport]) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    for bottleneck in bottlenecks {
        match bottleneck.phase {
            StartupPhase::UiInit => {
                recommendations.push("Consider lazy-loading UI components".to_string());
                recommendations.push("Optimize widget initialization".to_string());
            }
            StartupPhase::PluginLoad => {
                recommendations.push("Load plugins asynchronously".to_string());
                recommendations.push("Implement plugin caching".to_string());
            }
            StartupPhase::TransportInit => {
                recommendations.push("Defer transport connection until needed".to_string());
                recommendations.push("Parallelize transport initialization".to_string());
            }
            StartupPhase::DeviceManagerInit => {
                recommendations.push("Cache device configurations".to_string());
                recommendations.push("Implement lazy device discovery".to_string());
            }
            _ => {}
        }
    }
    
    if bottlenecks.len() > 3 {
        recommendations.push("Consider overall application architecture refactoring".to_string());
    }
    
    recommendations
}

/// Global startup tracker instance
static STARTUP_TRACKER: once_cell::sync::Lazy<StartupTracker> = 
    once_cell::sync::Lazy::new(StartupTracker::new);

/// Get global startup tracker
pub fn tracker() -> &'static StartupTracker {
    &STARTUP_TRACKER
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;
    
    #[tokio::test]
    async fn test_phase_tracking() {
        let tracker = StartupTracker::new();
        
        // Track phases
        tracker.start_phase(StartupPhase::CoreInit);
        sleep(Duration::from_millis(50)).await;
        tracker.complete_phase(StartupPhase::CoreInit);
        
        tracker.start_phase(StartupPhase::UiInit);
        sleep(Duration::from_millis(100)).await;
        tracker.complete_phase(StartupPhase::UiInit);
        
        tracker.complete_startup();
        
        let report = tracker.generate_report();
        assert_eq!(report.phases.len(), 2);
        assert!(!report.budget_exceeded);
    }
    
    #[test]
    fn test_phase_budget() {
        let phase = StartupPhase::UiInit;
        assert_eq!(phase.expected_duration_ms(), 500);
        assert_eq!(phase.display_name(), "UI framework");
    }
    
    #[test]
    fn test_report_generation() {
        let tracker = StartupTracker::with_budget(100);
        
        tracker.start_phase(StartupPhase::CoreInit);
        tracker.complete_phase(StartupPhase::CoreInit);
        
        let report = tracker.generate_report();
        assert!(!report.phases.is_empty());
        
        // Test markdown generation
        let markdown = report.to_markdown();
        assert!(markdown.contains("Startup Performance Report"));
        
        // Test JSON generation
        let json = report.to_json().unwrap();
        assert!(json.contains("total_duration_ms"));
    }
}