//! Performance validation test suite entry point

mod performance;
mod drivers;

use performance::benchmark::run_complete_benchmark;
use performance::latency::run_all_latency_benchmarks;
use performance::throughput::run_all_throughput_benchmarks;
use performance::stress::run_all_stress_tests;

#[tokio::main]
async fn main() {
    println!("Multi-Controller App Performance Validation");
    println!("===========================================\n");
    
    // Run complete benchmark suite
    let results = run_complete_benchmark().await;
    
    // Generate and save report
    let report = performance::benchmark::generate_report(&results);
    
    // Save to file
    std::fs::write("performance_validation_report.md", report)
        .expect("Failed to write report");
    
    // Save JSON results
    let json = serde_json::to_string_pretty(&results)
        .expect("Failed to serialize results");
    std::fs::write("performance_results.json", json)
        .expect("Failed to write JSON results");
    
    println!("\n✅ Performance validation complete!");
    println!("📊 Report saved to: performance_validation_report.md");
    println!("📝 Results saved to: performance_results.json");
    
    // Exit with appropriate code
    match results.overall_status {
        performance::benchmark::TestStatus::Passed => std::process::exit(0),
        performance::benchmark::TestStatus::Warning => std::process::exit(1),
        performance::benchmark::TestStatus::Failed => std::process::exit(2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_latency_suite() {
        run_all_latency_benchmarks().await;
    }
    
    #[tokio::test]
    async fn test_throughput_suite() {
        run_all_throughput_benchmarks().await;
    }
    
    #[tokio::test]
    async fn test_stress_suite() {
        run_all_stress_tests().await;
    }
    
    #[tokio::test]
    #[ignore] // Long running test
    async fn test_full_performance_validation() {
        let results = run_complete_benchmark().await;
        
        assert_eq!(
            results.overall_status,
            performance::benchmark::TestStatus::Passed,
            "Performance validation should pass all requirements"
        );
    }
}