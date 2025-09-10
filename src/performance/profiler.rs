//! Performance profiling with flame graph generation
//! 
//! Enterprise-grade profiling system for identifying performance bottlenecks

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};

/// Profile sample representing a single function call
#[derive(Debug, Clone)]
pub struct ProfileSample {
    pub function_name: String,
    pub module: String,
    pub start_time: Instant,
    pub duration: Duration,
    pub children: Vec<ProfileSample>,
    pub metadata: HashMap<String, String>,
}

impl ProfileSample {
    /// Create new profile sample
    pub fn new(function_name: String, module: String) -> Self {
        Self {
            function_name,
            module,
            start_time: Instant::now(),
            duration: Duration::ZERO,
            children: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Complete the sample
    pub fn complete(&mut self) {
        self.duration = Instant::now() - self.start_time;
    }
    
    /// Add child sample
    pub fn add_child(&mut self, child: ProfileSample) {
        self.children.push(child);
    }
    
    /// Calculate total time including children
    pub fn total_time(&self) -> Duration {
        let mut total = self.duration;
        for child in &self.children {
            total = total.saturating_add(child.total_time());
        }
        total
    }
    
    /// Calculate self time (excluding children)
    pub fn self_time(&self) -> Duration {
        let mut child_time = Duration::ZERO;
        for child in &self.children {
            child_time = child_time.saturating_add(child.duration);
        }
        self.duration.saturating_sub(child_time)
    }
}

/// Flame graph data for visualization
#[derive(Debug, Clone)]
pub struct FlameGraph {
    pub samples: Vec<ProfileSample>,
    pub total_duration: Duration,
    pub sample_count: usize,
    pub function_stats: HashMap<String, FunctionStats>,
}

/// Statistics for a single function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionStats {
    pub function_name: String,
    pub call_count: usize,
    pub total_time: Duration,
    pub self_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub avg_time: Duration,
}

/// Performance profiler
pub struct Profiler {
    samples: Arc<RwLock<Vec<ProfileSample>>>,
    current_stack: Arc<RwLock<Vec<ProfileSample>>>,
    enabled: Arc<RwLock<bool>>,
    max_samples: usize,
}

impl Profiler {
    /// Create new profiler
    pub fn new() -> Self {
        Self {
            samples: Arc::new(RwLock::new(Vec::new())),
            current_stack: Arc::new(RwLock::new(Vec::new())),
            enabled: Arc::new(RwLock::new(false)),
            max_samples: 10000,
        }
    }
    
    /// Enable profiling
    pub fn enable(&self) {
        *self.enabled.write() = true;
        self.clear();
    }
    
    /// Disable profiling
    pub fn disable(&self) {
        *self.enabled.write() = false;
    }
    
    /// Check if profiling is enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }
    
    /// Start profiling a function
    pub fn start_function(&self, function_name: String, module: String) -> ProfileGuard {
        if !self.is_enabled() {
            return ProfileGuard { profiler: None, sample: None };
        }
        
        let sample = ProfileSample::new(function_name, module);
        let mut stack = self.current_stack.write();
        stack.push(sample.clone());
        
        ProfileGuard {
            profiler: Some(self as *const Self),
            sample: Some(sample),
        }
    }
    
    /// Complete a function profile
    fn complete_function(&self, mut sample: ProfileSample) {
        sample.complete();
        
        let mut stack = self.current_stack.write();
        if let Some(pos) = stack.iter().position(|s| s.function_name == sample.function_name) {
            stack.remove(pos);
        }
        
        if stack.is_empty() {
            // Top-level sample
            let mut samples = self.samples.write();
            if samples.len() < self.max_samples {
                samples.push(sample);
            }
        } else {
            // Nested sample
            if let Some(parent) = stack.last_mut() {
                parent.add_child(sample);
            }
        }
    }
    
    /// Clear all samples
    pub fn clear(&self) {
        self.samples.write().clear();
        self.current_stack.write().clear();
    }
    
    /// Generate flame graph
    pub fn generate_flame_graph(&self) -> FlameGraph {
        let samples = self.samples.read().clone();
        let mut function_stats = HashMap::new();
        let mut total_duration = Duration::ZERO;
        
        // Calculate statistics
        for sample in &samples {
            total_duration = total_duration.saturating_add(sample.duration);
            collect_stats(&sample, &mut function_stats);
        }
        
        FlameGraph {
            samples,
            total_duration,
            sample_count: function_stats.values().map(|s| s.call_count).sum(),
            function_stats,
        }
    }
    
    /// Export as flame graph format
    pub fn export_flamegraph(&self) -> String {
        let mut output = String::new();
        let samples = self.samples.read();
        
        for sample in samples.iter() {
            export_sample(&sample, &mut output, Vec::new());
        }
        
        output
    }
}

/// RAII guard for profiling
pub struct ProfileGuard {
    profiler: Option<*const Profiler>,
    sample: Option<ProfileSample>,
}

impl Drop for ProfileGuard {
    fn drop(&mut self) {
        if let (Some(profiler), Some(sample)) = (self.profiler, self.sample.take()) {
            unsafe {
                (*profiler).complete_function(sample);
            }
        }
    }
}

/// Collect function statistics
fn collect_stats(sample: &ProfileSample, stats: &mut HashMap<String, FunctionStats>) {
    let key = format!("{}::{}", sample.module, sample.function_name);
    
    let stat = stats.entry(key.clone()).or_insert_with(|| FunctionStats {
        function_name: sample.function_name.clone(),
        call_count: 0,
        total_time: Duration::ZERO,
        self_time: Duration::ZERO,
        min_time: Duration::MAX,
        max_time: Duration::ZERO,
        avg_time: Duration::ZERO,
    });
    
    stat.call_count += 1;
    stat.total_time = stat.total_time.saturating_add(sample.total_time());
    stat.self_time = stat.self_time.saturating_add(sample.self_time());
    stat.min_time = stat.min_time.min(sample.duration);
    stat.max_time = stat.max_time.max(sample.duration);
    stat.avg_time = stat.total_time / stat.call_count as u32;
    
    // Recurse for children
    for child in &sample.children {
        collect_stats(child, stats);
    }
}

/// Export sample in flame graph format
fn export_sample(sample: &ProfileSample, output: &mut String, mut stack: Vec<String>) {
    stack.push(format!("{}::{}", sample.module, sample.function_name));
    
    // Output: stack;count
    let stack_str = stack.join(";");
    let count = sample.duration.as_micros();
    output.push_str(&format!("{} {}\n", stack_str, count));
    
    // Recurse for children
    for child in &sample.children {
        export_sample(child, output, stack.clone());
    }
}

/// Global profiler instance
static PROFILER: once_cell::sync::Lazy<Profiler> = 
    once_cell::sync::Lazy::new(Profiler::new);

/// Get global profiler
pub fn profiler() -> &'static Profiler {
    &PROFILER
}

/// Profile a code block
#[macro_export]
macro_rules! profile {
    ($name:expr) => {
        let _guard = $crate::performance::profiler::profiler()
            .start_function($name.to_string(), module_path!().to_string());
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_profiling() {
        let profiler = Profiler::new();
        profiler.enable();
        
        {
            let _guard = profiler.start_function("test_function".to_string(), "test".to_string());
            thread::sleep(Duration::from_millis(10));
        }
        
        let graph = profiler.generate_flame_graph();
        assert_eq!(graph.samples.len(), 1);
        assert!(graph.total_duration >= Duration::from_millis(10));
    }
    
    #[test]
    fn test_nested_profiling() {
        let profiler = Profiler::new();
        profiler.enable();
        
        {
            let _outer = profiler.start_function("outer".to_string(), "test".to_string());
            thread::sleep(Duration::from_millis(5));
            
            {
                let _inner = profiler.start_function("inner".to_string(), "test".to_string());
                thread::sleep(Duration::from_millis(5));
            }
        }
        
        let graph = profiler.generate_flame_graph();
        assert!(!graph.samples.is_empty());
        assert_eq!(graph.function_stats.len(), 2);
    }
    
    #[test]
    fn test_export_flamegraph() {
        let profiler = Profiler::new();
        profiler.enable();
        
        {
            let _guard = profiler.start_function("test".to_string(), "module".to_string());
            thread::sleep(Duration::from_millis(1));
        }
        
        let export = profiler.export_flamegraph();
        assert!(export.contains("module::test"));
    }
}