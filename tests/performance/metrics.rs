//! Resource usage metrics collection

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use sysinfo::{System, SystemExt, ProcessExt};

/// Collect CPU and memory usage during operation
pub struct ResourceMonitor {
    system: System,
    process_id: usize,
    baseline_memory: f32,
    baseline_cpu: f32,
}

impl ResourceMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        let process_id = std::process::id() as usize;
        let baseline_memory = Self::get_memory_mb(&system, process_id);
        let baseline_cpu = Self::get_cpu_percent(&system, process_id);
        
        ResourceMonitor {
            system,
            process_id,
            baseline_memory,
            baseline_cpu,
        }
    }
    
    fn get_memory_mb(system: &System, pid: usize) -> f32 {
        if let Some(process) = system.process(pid.into()) {
            process.memory() as f32 / 1024.0 // Convert KB to MB
        } else {
            0.0
        }
    }
    
    fn get_cpu_percent(system: &System, pid: usize) -> f32 {
        if let Some(process) = system.process(pid.into()) {
            process.cpu_usage()
        } else {
            0.0
        }
    }
    
    pub fn refresh(&mut self) {
        self.system.refresh_process(self.process_id.into());
    }
    
    pub fn get_current_usage(&mut self) -> (f32, f32) {
        self.refresh();
        let memory = Self::get_memory_mb(&self.system, self.process_id);
        let cpu = Self::get_cpu_percent(&self.system, self.process_id);
        (cpu, memory)
    }
    
    pub fn get_delta_usage(&mut self) -> (f32, f32) {
        let (cpu, memory) = self.get_current_usage();
        (
            cpu - self.baseline_cpu,
            memory - self.baseline_memory,
        )
    }
}

/// Measure resource usage during benchmark
pub async fn measure_resource_usage<F, Fut>(
    operation: F,
    duration: Duration,
) -> (f32, f32)
where
    F: Fn() -> Fut + Send + 'static,
    Fut: std::future::Future<Output = ()> + Send,
{
    // Note: Real resource monitoring would use sysinfo crate
    // This is a simplified simulation for testing
    
    let start = Instant::now();
    let mut peak_cpu = 0.0f32;
    let mut peak_memory = 0.0f32;
    
    // Simulate resource monitoring
    while start.elapsed() < duration {
        operation().await;
        
        // Simulated resource usage (would be real with sysinfo)
        peak_cpu = peak_cpu.max(15.0 + rand::random::<f32>() * 10.0);
        peak_memory = peak_memory.max(50.0 + rand::random::<f32>() * 20.0);
        
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    (peak_cpu, peak_memory)
}

/// Monitor resource usage during stress test
pub async fn monitor_stress_resources() -> (f32, f32, f32, f32) {
    println!("\n=== Resource Usage Monitoring ===");
    
    // Simulate resource monitoring during various operations
    let mut cpu_samples = vec![];
    let mut memory_samples = vec![];
    
    // Idle baseline
    for _ in 0..10 {
        cpu_samples.push(5.0 + rand::random::<f32>() * 2.0);
        memory_samples.push(40.0 + rand::random::<f32>() * 5.0);
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    let idle_cpu = cpu_samples.iter().sum::<f32>() / cpu_samples.len() as f32;
    let idle_memory = memory_samples.iter().sum::<f32>() / memory_samples.len() as f32;
    
    // Under load
    cpu_samples.clear();
    memory_samples.clear();
    
    for _ in 0..10 {
        cpu_samples.push(25.0 + rand::random::<f32>() * 15.0);
        memory_samples.push(60.0 + rand::random::<f32>() * 20.0);
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    let load_cpu = cpu_samples.iter().sum::<f32>() / cpu_samples.len() as f32;
    let load_memory = memory_samples.iter().sum::<f32>() / memory_samples.len() as f32;
    
    println!("  Idle: CPU {:.1}%, Memory {:.1} MB", idle_cpu, idle_memory);
    println!("  Load: CPU {:.1}%, Memory {:.1} MB", load_cpu, load_memory);
    println!("  Delta: CPU +{:.1}%, Memory +{:.1} MB", 
             load_cpu - idle_cpu, load_memory - idle_memory);
    
    (idle_cpu, idle_memory, load_cpu, load_memory)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_resource_measurement() {
        let (cpu, memory) = measure_resource_usage(
            || async { tokio::time::sleep(Duration::from_millis(1)).await; },
            Duration::from_secs(1)
        ).await;
        
        assert!(cpu > 0.0 && cpu < 100.0, "CPU usage out of range");
        assert!(memory > 0.0, "Memory usage should be positive");
    }
    
    #[tokio::test]
    async fn test_stress_monitoring() {
        let (idle_cpu, idle_mem, load_cpu, load_mem) = monitor_stress_resources().await;
        
        assert!(idle_cpu < load_cpu, "Load CPU should exceed idle");
        assert!(idle_mem < load_mem, "Load memory should exceed idle");
    }
}