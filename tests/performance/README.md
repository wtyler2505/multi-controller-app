# Multi-Controller App Performance Benchmark Framework

A comprehensive performance testing framework for comparing C# Native AOT and Rust prototypes of the Multi-Controller App.

## Overview

This framework measures and compares:
- **Startup time** (target: <2 seconds)
- **Idle CPU usage** (target: ≤2%)
- **Base RAM usage** (target: ≤150MB)
- **Executable size** (for distribution efficiency)
- **Build time** (for development velocity)

## Performance Budgets

| Metric | Budget | C# Target | Rust Target |
|--------|--------|-----------|-------------|
| Startup Time | <2 seconds | <2s | <2s |
| Idle CPU | ≤2% | ≤2% | ≤2% |
| Base Memory | ≤150MB | ≤150MB | ≤150MB |
| Memory + Charts | ≤220MB | ≤220MB | ≤220MB |

## Scripts Overview

### Core Scripts

- **`benchmark.ps1`** - Core benchmarking engine
- **`benchmark-csharp.ps1`** - C# Native AOT specific wrapper
- **`benchmark-rust.ps1`** - Rust specific wrapper
- **`compare-benchmarks.ps1`** - Comparison analysis tool
- **`run-all-benchmarks.ps1`** - Master orchestration script

### Quick Start

```powershell
# Run comprehensive benchmark of both prototypes
.\run-all-benchmarks.ps1 -ShowDetails

# Run only C# benchmark
.\benchmark-csharp.ps1 -ShowDetails

# Run only Rust benchmark  
.\benchmark-rust.ps1 -ShowDetails

# Compare existing reports
.\compare-benchmarks.ps1 -CSharpReport "csharp-report.json" -RustReport "rust-report.json" -ShowDetails -GenerateChart
```

## Detailed Usage

### 1. Core Benchmark Engine (`benchmark.ps1`)

Measures performance of any executable:

```powershell
# Basic usage
.\benchmark.ps1 -ExecutablePath "path\to\app.exe" -BenchmarkType "csharp"

# Extended monitoring
.\benchmark.ps1 -ExecutablePath "app.exe" -BenchmarkType "rust" -MonitorDuration 120 -ShowDetails

# Custom output path
.\benchmark.ps1 -ExecutablePath "app.exe" -BenchmarkType "csharp" -OutputPath "results\report.json"
```

**Parameters:**
- `-ExecutablePath` - Path to executable (required)
- `-BenchmarkType` - 'csharp' or 'rust' (required)  
- `-OutputPath` - JSON report path (optional)
- `-ShowDetails` - Verbose progress output
- `-MonitorDuration` - Monitoring duration in seconds (default: 60)

### 2. C# Native AOT Benchmark (`benchmark-csharp.ps1`)

Builds and benchmarks C# implementation:

```powershell
# Default Release build
.\benchmark-csharp.ps1 -ShowDetails

# Specific configuration and runtime
.\benchmark-csharp.ps1 -Configuration Release -Runtime win-x64 -MonitorDuration 120

# Skip build step
.\benchmark-csharp.ps1 -SkipBuild -ShowDetails
```

**Parameters:**
- `-Configuration` - 'Release' or 'Debug' (default: Release)
- `-Runtime` - 'win-x64', 'win-x86', 'win-arm64' (default: win-x64)
- `-ShowDetails` - Show build and benchmark details
- `-SkipBuild` - Skip build step
- `-MonitorDuration` - Monitoring duration (default: 60s)

### 3. Rust Benchmark (`benchmark-rust.ps1`)

Builds and benchmarks Rust implementation:

```powershell
# Default release build
.\benchmark-rust.ps1 -ShowDetails

# Development profile
.\benchmark-rust.ps1 -Profile dev -MonitorDuration 120

# With specific features
.\benchmark-rust.ps1 -Features "gui,telemetry" -ShowDetails
```

**Parameters:**
- `-Profile` - 'release' or 'dev' (default: release)
- `-Target` - Rust target triple (optional)
- `-ShowDetails` - Verbose output
- `-SkipBuild` - Skip cargo build
- `-MonitorDuration` - Monitoring duration (default: 60s)
- `-Features` - Comma-separated Cargo features

### 4. Comparison Analysis (`compare-benchmarks.ps1`)

Compares two benchmark reports:

```powershell
# Basic comparison
.\compare-benchmarks.ps1 -CSharpReport "csharp.json" -RustReport "rust.json"

# Detailed analysis with chart
.\compare-benchmarks.ps1 -CSharpReport "csharp.json" -RustReport "rust.json" -ShowDetails -GenerateChart

# Custom output path
.\compare-benchmarks.ps1 -CSharpReport "csharp.json" -RustReport "rust.json" -OutputPath "comparison.json"
```

**Parameters:**
- `-CSharpReport` - Path to C# benchmark JSON (required)
- `-RustReport` - Path to Rust benchmark JSON (required)
- `-OutputPath` - Comparison report path (optional)
- `-ShowDetails` - Detailed analysis and recommendations
- `-GenerateChart` - Text-based comparison chart

### 5. Master Orchestration (`run-all-benchmarks.ps1`)

Runs complete benchmark suite:

```powershell
# Full benchmark suite
.\run-all-benchmarks.ps1 -ShowDetails

# Skip builds
.\run-all-benchmarks.ps1 -SkipBuild -MonitorDuration 120

# Custom output directory
.\run-all-benchmarks.ps1 -OutputDir "reports" -KeepReports

# Skip specific benchmarks
.\run-all-benchmarks.ps1 -SkipRust -ShowDetails
```

**Parameters:**
- `-SkipCSharp` - Skip C# benchmark
- `-SkipRust` - Skip Rust benchmark  
- `-SkipBuild` - Skip all build steps
- `-Configuration` - C# configuration (default: Release)
- `-RustProfile` - Rust profile (default: release)
- `-ShowDetails` - Verbose output
- `-MonitorDuration` - Per-benchmark duration (default: 60s)
- `-OutputDir` - Report directory (default: current)
- `-KeepReports` - Keep individual reports after comparison

## Output Format

### Individual Benchmark Reports

JSON structure for each benchmark:

```json
{
  "metadata": {
    "benchmarkType": "csharp|rust",
    "executablePath": "path/to/executable",
    "timestamp": "ISO timestamp",
    "monitorDuration": 60,
    "sampleCount": 60,
    "buildTimeSeconds": 45.2,
    "executableSizeMB": 12.5
  },
  "budgets": {
    "StartupTimeMs": 2000,
    "IdleCpuPercent": 2.0,
    "BaseMemoryMB": 150
  },
  "results": {
    "startup": {
      "timeMs": 1250,
      "passed": true,
      "budgetMs": 2000
    },
    "cpu": {
      "averagePercent": 1.2,
      "maximumPercent": 8.5,
      "passed": true,
      "samples": [1.1, 1.3, 1.2, ...]
    },
    "memory": {
      "minimumMB": 45.2,
      "averageMB": 52.1,
      "maximumMB": 58.3,
      "passed": true,
      "samples": [45.2, 50.1, 52.3, ...]
    }
  },
  "performance_gate": {
    "passed": true,
    "failures": []
  }
}
```

### Comparison Reports

JSON structure for comparisons:

```json
{
  "metadata": {
    "comparisonDate": "ISO timestamp",
    "csharpReportPath": "path/to/csharp.json",
    "rustReportPath": "path/to/rust.json"
  },
  "summary": {
    "csharpPassed": true,
    "rustPassed": true,
    "overallWinner": "both|csharp|rust|neither"
  },
  "comparison": {
    "startup": {
      "csharpMs": 1250,
      "rustMs": 980,
      "difference": "-21.6%",
      "winner": "rust"
    },
    "cpu": { ... },
    "memory": { ... }
  }
}
```

## Performance Gate Integration

### Exit Codes

- **0** - All performance budgets met
- **1** - One or more budgets exceeded

### CI/CD Integration

```yaml
# Example GitHub Actions integration
- name: Run Performance Benchmarks
  run: |
    cd tests/performance
    ./run-all-benchmarks.ps1 -ShowDetails
    if ($LASTEXITCODE -ne 0) {
      throw "Performance budgets exceeded"
    }
  shell: pwsh

- name: Upload Performance Reports
  uses: actions/upload-artifact@v4
  with:
    name: performance-reports
    path: tests/performance/*.json
```

### Pre-commit Hook Integration

```powershell
# Add to pre-commit hook
cd tests/performance
./benchmark-csharp.ps1 -SkipBuild -MonitorDuration 30
if ($LASTEXITCODE -ne 0) {
    Write-Error "Performance regression detected"
    exit 1
}
```

## Troubleshooting

### Common Issues

**1. Executable not found:**
```
[ERROR] Executable not found: path\to\app.exe
```
- Ensure the build completed successfully
- Check the executable path
- For C#: Look in `bin\Release\net8.0-windows\win-x64\publish\`
- For Rust: Look in `target\release\` or `target\debug\`

**2. Process exits during startup:**
```
[ERROR] Process exited during startup measurement
```
- Check application dependencies (runtime, libraries)
- Verify the executable is not corrupted
- Run manually first to identify issues
- Check stderr output in the error message

**3. Performance counter failures:**
```
[ERROR] Failed to initialize CPU performance counter
```
- Run PowerShell as Administrator
- Check Windows Performance Toolkit is installed
- Verify WMI service is running

**4. Rust project not found:**
```
[PERF-RUST] Rust project not found at: path
```
- Framework creates placeholder Rust project automatically
- Customize the generated `Cargo.toml` and `main.rs` for real testing
- Or point to existing Rust implementation

### Performance Optimization Tips

**Startup Time:**
- Use lazy loading for non-critical components
- Defer expensive initialization
- Reduce dependency injection overhead
- Profile with startup timer in code

**Memory Usage:**
- Monitor GC pressure in C#
- Use object pooling for frequent allocations  
- Profile with dotMemory or similar tools
- Consider struct vs class trade-offs

**CPU Usage:**
- Profile with dotTrace or perf tools
- Identify hot paths with profiler
- Optimize loops and frequent operations
- Consider async/await patterns

## Development Workflow

### 1. Initial Setup
```powershell
# Run baseline benchmarks
cd tests/performance
.\run-all-benchmarks.ps1 -ShowDetails
```

### 2. During Development
```powershell
# Quick performance check
.\benchmark-csharp.ps1 -ShowDetails -MonitorDuration 30

# Compare with previous results
.\compare-benchmarks.ps1 -CSharpReport baseline.json -RustReport current.json -ShowDetails
```

### 3. Pre-release Validation  
```powershell
# Full validation suite
.\run-all-benchmarks.ps1 -MonitorDuration 120 -OutputDir release-reports -ShowDetails
```

### 4. Regression Testing
```powershell
# Compare against baseline
.\compare-benchmarks.ps1 -CSharpReport baseline-csharp.json -RustReport current-csharp.json -GenerateChart
```

## Architecture Integration

The benchmark framework integrates with the Multi-Controller App architecture:

- **Performance Profiler Agent**: Use `Task` tool with `subagent_type: "performance-profiler"`
- **Task Master Integration**: Log benchmark results in task updates
- **Git Workflow**: Include performance validation in PR checks
- **Memory Steward**: Store performance baselines for comparison

## File Locations

```
tests/performance/
├── benchmark.ps1                 # Core benchmarking engine
├── benchmark-csharp.ps1         # C# specific wrapper
├── benchmark-rust.ps1           # Rust specific wrapper  
├── compare-benchmarks.ps1       # Comparison analysis
├── run-all-benchmarks.ps1       # Master orchestration
├── README.md                    # This documentation
├── *.json                       # Generated reports
└── reports/                     # Optional output directory
```

## Future Enhancements

- **Flamegraph integration** for detailed CPU profiling
- **Memory leak detection** with extended monitoring
- **Network latency measurement** for transport operations  
- **I/O performance testing** for serial/TCP operations
- **Stress testing** with concurrent device connections
- **Battery usage monitoring** for mobile deployments
- **GPU usage tracking** for chart rendering