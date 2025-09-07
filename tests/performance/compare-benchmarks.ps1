#Requires -Version 5.1
<#
.SYNOPSIS
    Compare performance benchmark results between C# and Rust prototypes
    
.DESCRIPTION
    Analyzes and compares benchmark JSON reports from both C# Native AOT and Rust implementations.
    Provides detailed comparison metrics, visualizations, and recommendations.
    
.PARAMETER CSharpReport
    Path to the C# benchmark JSON report
    
.PARAMETER RustReport
    Path to the Rust benchmark JSON report
    
.PARAMETER OutputPath
    Path to save the comparison report (optional)
    
.PARAMETER ShowDetails
    Show detailed analysis and recommendations
    
.PARAMETER GenerateChart
    Generate a simple text-based comparison chart
    
.EXAMPLE
    .\compare-benchmarks.ps1 -CSharpReport "csharp-benchmark-20241224-143022.json" -RustReport "rust-benchmark-20241224-143122.json"
    
.EXAMPLE
    .\compare-benchmarks.ps1 -CSharpReport "reports\csharp.json" -RustReport "reports\rust.json" -ShowDetails -GenerateChart
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$CSharpReport,
    
    [Parameter(Mandatory = $true)]
    [string]$RustReport,
    
    [string]$OutputPath = "benchmark-comparison-$(Get-Date -Format 'yyyyMMdd-HHmmss').json",
    
    [switch]$ShowDetails,
    
    [switch]$GenerateChart
)

# Validate input files
if (-not (Test-Path $CSharpReport)) {
    Write-Error "C# report not found: $CSharpReport"
    exit 1
}

if (-not (Test-Path $RustReport)) {
    Write-Error "Rust report not found: $RustReport"
    exit 1
}

# Load reports
try {
    $csharpData = Get-Content $CSharpReport | ConvertFrom-Json
    $rustData = Get-Content $RustReport | ConvertFrom-Json
} catch {
    Write-Error "Failed to parse JSON reports: $($_.Exception.Message)"
    exit 1
}

# Helper function for percentage calculation
function Get-PercentageDifference {
    param(
        [double]$Value1,
        [double]$Value2,
        [int]$Decimals = 1
    )
    
    if ($Value1 -eq 0) { return "N/A" }
    $diff = (($Value2 - $Value1) / $Value1) * 100
    return "$([math]::Round($diff, $Decimals))%"
}

# Helper function for better/worse indication
function Get-PerformanceIndicator {
    param(
        [double]$Value1,
        [double]$Value2,
        [bool]$LowerIsBetter = $true
    )
    
    if ($Value1 -eq $Value2) { return "=" }
    
    if ($LowerIsBetter) {
        return if ($Value2 -lt $Value1) { "BETTER" } else { "WORSE" }
    } else {
        return if ($Value2 -gt $Value1) { "BETTER" } else { "WORSE" }
    }
}

# Extract key metrics
$csharpMetrics = @{
    StartupMs = $csharpData.results.startup.timeMs
    AvgCpu = $csharpData.results.cpu.averagePercent
    MaxCpu = $csharpData.results.cpu.maximumPercent
    AvgMemory = $csharpData.results.memory.averageMB
    MaxMemory = $csharpData.results.memory.maximumMB
    MinMemory = $csharpData.results.memory.minimumMB
    ExecutableSize = if ($csharpData.metadata.executableSizeMB) { $csharpData.metadata.executableSizeMB } else { 0 }
    BuildTime = if ($csharpData.metadata.buildTimeSeconds) { $csharpData.metadata.buildTimeSeconds } else { 0 }
    Passed = $csharpData.performance_gate.passed
}

$rustMetrics = @{
    StartupMs = $rustData.results.startup.timeMs
    AvgCpu = $rustData.results.cpu.averagePercent
    MaxCpu = $rustData.results.cpu.maximumPercent
    AvgMemory = $rustData.results.memory.averageMB
    MaxMemory = $rustData.results.memory.maximumMB
    MinMemory = $rustData.results.memory.minimumMB
    ExecutableSize = if ($rustData.metadata.executableSizeMB) { $rustData.metadata.executableSizeMB } else { 0 }
    BuildTime = if ($rustData.metadata.buildTimeSeconds) { $rustData.metadata.buildTimeSeconds } else { 0 }
    Passed = $rustData.performance_gate.passed
}

# Performance budgets for reference
$budgets = $csharpData.budgets

Write-Host ""
Write-Host "================ BENCHMARK COMPARISON ANALYSIS ================" -ForegroundColor White
Write-Host "C# Report: $CSharpReport" -ForegroundColor Gray
Write-Host "Rust Report: $RustReport" -ForegroundColor Gray
Write-Host "Comparison Date: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" -ForegroundColor Gray
Write-Host ""

# Overall Performance Gate Status
Write-Host "=== PERFORMANCE GATE STATUS ===" -ForegroundColor Yellow
if ($csharpMetrics.Passed) {
    Write-Host "C# Native AOT: PASSED" -ForegroundColor Green
} else {
    Write-Host "C# Native AOT: FAILED" -ForegroundColor Red
}

if ($rustMetrics.Passed) {
    Write-Host "Rust:          PASSED" -ForegroundColor Green
} else {
    Write-Host "Rust:          FAILED" -ForegroundColor Red
}
Write-Host ""

# Startup Time Comparison
Write-Host "=== STARTUP TIME COMPARISON ===" -ForegroundColor Yellow
Write-Host ("C# Native AOT:  {0,6}ms  {1}" -f $csharpMetrics.StartupMs, $(if ($csharpMetrics.StartupMs -le $budgets.StartupTimeMs) { "(PASS)" } else { "(FAIL)" })) -ForegroundColor $(if ($csharpMetrics.StartupMs -le $budgets.StartupTimeMs) { "Green" } else { "Red" })
Write-Host ("Rust:           {0,6}ms  {1}" -f $rustMetrics.StartupMs, $(if ($rustMetrics.StartupMs -le $budgets.StartupTimeMs) { "(PASS)" } else { "(FAIL)" })) -ForegroundColor $(if ($rustMetrics.StartupMs -le $budgets.StartupTimeMs) { "Green" } else { "Red" })
Write-Host ("Budget:         {0,6}ms" -f $budgets.StartupTimeMs) -ForegroundColor Gray

$startupDiff = Get-PercentageDifference $csharpMetrics.StartupMs $rustMetrics.StartupMs
$startupIndicator = Get-PerformanceIndicator $csharpMetrics.StartupMs $rustMetrics.StartupMs $true
Write-Host ("Difference:     {0,6}  ({1})" -f $startupDiff, $startupIndicator) -ForegroundColor $(if ($startupIndicator -eq "BETTER") { "Green" } elseif ($startupIndicator -eq "WORSE") { "Red" } else { "Gray" })
Write-Host ""

# CPU Usage Comparison
Write-Host "=== CPU USAGE COMPARISON ===" -ForegroundColor Yellow
Write-Host ("C# Avg CPU:     {0,6:F1}%  {1}" -f $csharpMetrics.AvgCpu, $(if ($csharpMetrics.AvgCpu -le $budgets.IdleCpuPercent) { "(PASS)" } else { "(FAIL)" })) -ForegroundColor $(if ($csharpMetrics.AvgCpu -le $budgets.IdleCpuPercent) { "Green" } else { "Red" })
Write-Host ("Rust Avg CPU:   {0,6:F1}%  {1}" -f $rustMetrics.AvgCpu, $(if ($rustMetrics.AvgCpu -le $budgets.IdleCpuPercent) { "(PASS)" } else { "(FAIL)" })) -ForegroundColor $(if ($rustMetrics.AvgCpu -le $budgets.IdleCpuPercent) { "Green" } else { "Red" })
Write-Host ("Budget:         {0,6:F1}%" -f $budgets.IdleCpuPercent) -ForegroundColor Gray

$cpuDiff = Get-PercentageDifference $csharpMetrics.AvgCpu $rustMetrics.AvgCpu
$cpuIndicator = Get-PerformanceIndicator $csharpMetrics.AvgCpu $rustMetrics.AvgCpu $true
Write-Host ("Difference:     {0,6}  ({1})" -f $cpuDiff, $cpuIndicator) -ForegroundColor $(if ($cpuIndicator -eq "BETTER") { "Green" } elseif ($cpuIndicator -eq "WORSE") { "Red" } else { "Gray" })

Write-Host ("C# Max CPU:     {0,6:F1}%" -f $csharpMetrics.MaxCpu) -ForegroundColor Gray
Write-Host ("Rust Max CPU:   {0,6:F1}%" -f $rustMetrics.MaxCpu) -ForegroundColor Gray
Write-Host ""

# Memory Usage Comparison
Write-Host "=== MEMORY USAGE COMPARISON ===" -ForegroundColor Yellow
Write-Host ("C# Peak Memory: {0,6:F1}MB {1}" -f $csharpMetrics.MaxMemory, $(if ($csharpMetrics.MaxMemory -le $budgets.BaseMemoryMB) { "(PASS)" } else { "(FAIL)" })) -ForegroundColor $(if ($csharpMetrics.MaxMemory -le $budgets.BaseMemoryMB) { "Green" } else { "Red" })
Write-Host ("Rust Peak Mem:  {0,6:F1}MB {1}" -f $rustMetrics.MaxMemory, $(if ($rustMetrics.MaxMemory -le $budgets.BaseMemoryMB) { "(PASS)" } else { "(FAIL)" })) -ForegroundColor $(if ($rustMetrics.MaxMemory -le $budgets.BaseMemoryMB) { "Green" } else { "Red" })
Write-Host ("Budget:         {0,6:F1}MB" -f $budgets.BaseMemoryMB) -ForegroundColor Gray

$memoryDiff = Get-PercentageDifference $csharpMetrics.MaxMemory $rustMetrics.MaxMemory
$memoryIndicator = Get-PerformanceIndicator $csharpMetrics.MaxMemory $rustMetrics.MaxMemory $true
Write-Host ("Difference:     {0,6}  ({1})" -f $memoryDiff, $memoryIndicator) -ForegroundColor $(if ($memoryIndicator -eq "BETTER") { "Green" } elseif ($memoryIndicator -eq "WORSE") { "Red" } else { "Gray" })

Write-Host ("C# Avg Memory:  {0,6:F1}MB" -f $csharpMetrics.AvgMemory) -ForegroundColor Gray
Write-Host ("Rust Avg Mem:   {0,6:F1}MB" -f $rustMetrics.AvgMemory) -ForegroundColor Gray
Write-Host ""

# Executable Size Comparison
if ($csharpMetrics.ExecutableSize -gt 0 -and $rustMetrics.ExecutableSize -gt 0) {
    Write-Host "=== EXECUTABLE SIZE COMPARISON ===" -ForegroundColor Yellow
    Write-Host ("C# Executable:  {0,6:F1}MB" -f $csharpMetrics.ExecutableSize) -ForegroundColor Gray
    Write-Host ("Rust Executable:{0,6:F1}MB" -f $rustMetrics.ExecutableSize) -ForegroundColor Gray
    
    $sizeDiff = Get-PercentageDifference $csharpMetrics.ExecutableSize $rustMetrics.ExecutableSize
    $sizeIndicator = Get-PerformanceIndicator $csharpMetrics.ExecutableSize $rustMetrics.ExecutableSize $true
    Write-Host ("Difference:     {0,6}  ({1})" -f $sizeDiff, $sizeIndicator) -ForegroundColor $(if ($sizeIndicator -eq "BETTER") { "Green" } elseif ($sizeIndicator -eq "WORSE") { "Red" } else { "Gray" })
    Write-Host ""
}

# Build Time Comparison
if ($csharpMetrics.BuildTime -gt 0 -and $rustMetrics.BuildTime -gt 0) {
    Write-Host "=== BUILD TIME COMPARISON ===" -ForegroundColor Yellow
    Write-Host ("C# Build Time:  {0,6:F1}s" -f $csharpMetrics.BuildTime) -ForegroundColor Gray
    Write-Host ("Rust Build Time:{0,6:F1}s" -f $rustMetrics.BuildTime) -ForegroundColor Gray
    
    $buildDiff = Get-PercentageDifference $csharpMetrics.BuildTime $rustMetrics.BuildTime
    $buildIndicator = Get-PerformanceIndicator $csharpMetrics.BuildTime $rustMetrics.BuildTime $true
    Write-Host ("Difference:     {0,6}  ({1})" -f $buildDiff, $buildIndicator) -ForegroundColor $(if ($buildIndicator -eq "BETTER") { "Green" } elseif ($buildIndicator -eq "WORSE") { "Red" } else { "Gray" })
    Write-Host ""
}

# Detailed Analysis
if ($ShowDetails) {
    Write-Host "=== DETAILED ANALYSIS ===" -ForegroundColor Yellow
    
    # Technology stack info
    Write-Host "Technology Stack:" -ForegroundColor White
    Write-Host "  C# Version: $($csharpData.metadata.powershellVersion)" -ForegroundColor Gray
    if ($csharpData.metadata.buildConfiguration) {
        Write-Host "  C# Config: $($csharpData.metadata.buildConfiguration) ($($csharpData.metadata.targetRuntime))" -ForegroundColor Gray
    }
    if ($rustData.metadata.rustcVersion) {
        Write-Host "  Rust Version: $($rustData.metadata.rustcVersion)" -ForegroundColor Gray
    }
    if ($rustData.metadata.buildProfile) {
        Write-Host "  Rust Profile: $($rustData.metadata.buildProfile)" -ForegroundColor Gray
    }
    Write-Host ""
    
    # Performance characteristics
    Write-Host "Performance Characteristics:" -ForegroundColor White
    
    # Memory efficiency
    $memoryEfficiency = if ($csharpMetrics.MaxMemory -lt $rustMetrics.MaxMemory) { "C# is more memory efficient" } 
                       elseif ($rustMetrics.MaxMemory -lt $csharpMetrics.MaxMemory) { "Rust is more memory efficient" } 
                       else { "Similar memory usage" }
    Write-Host "  Memory: $memoryEfficiency" -ForegroundColor Gray
    
    # CPU efficiency
    $cpuEfficiency = if ($csharpMetrics.AvgCpu -lt $rustMetrics.AvgCpu) { "C# is more CPU efficient" } 
                    elseif ($rustMetrics.AvgCpu -lt $csharpMetrics.AvgCpu) { "Rust is more CPU efficient" } 
                    else { "Similar CPU usage" }
    Write-Host "  CPU: $cpuEfficiency" -ForegroundColor Gray
    
    # Startup performance
    $startupEfficiency = if ($csharpMetrics.StartupMs -lt $rustMetrics.StartupMs) { "C# starts faster" } 
                        elseif ($rustMetrics.StartupMs -lt $csharpMetrics.StartupMs) { "Rust starts faster" } 
                        else { "Similar startup time" }
    Write-Host "  Startup: $startupEfficiency" -ForegroundColor Gray
    Write-Host ""
    
    # Recommendations
    Write-Host "Recommendations:" -ForegroundColor White
    
    # Performance gate status
    if ($csharpMetrics.Passed -and $rustMetrics.Passed) {
        Write-Host "  [OK] Both implementations meet performance budgets" -ForegroundColor Green
    } elseif ($csharpMetrics.Passed -and -not $rustMetrics.Passed) {
        Write-Host "  [RECOMMEND] Choose C# - meets performance budgets" -ForegroundColor Yellow
    } elseif (-not $csharpMetrics.Passed -and $rustMetrics.Passed) {
        Write-Host "  [RECOMMEND] Choose Rust - meets performance budgets" -ForegroundColor Yellow
    } else {
        Write-Host "  [WARNING] Neither implementation meets all budgets" -ForegroundColor Red
    }
    
    # Memory recommendations
    if ($csharpMetrics.MaxMemory -gt $budgets.BaseMemoryMB -or $rustMetrics.MaxMemory -gt $budgets.BaseMemoryMB) {
        Write-Host "  [ACTION] Memory optimization needed for production" -ForegroundColor Yellow
    }
    
    # CPU recommendations
    if ($csharpMetrics.AvgCpu -gt $budgets.IdleCpuPercent -or $rustMetrics.AvgCpu -gt $budgets.IdleCpuPercent) {
        Write-Host "  [ACTION] CPU usage optimization needed" -ForegroundColor Yellow
    }
    
    # Startup recommendations
    if ($csharpMetrics.StartupMs -gt $budgets.StartupTimeMs -or $rustMetrics.StartupMs -gt $budgets.StartupTimeMs) {
        Write-Host "  [ACTION] Startup time optimization needed" -ForegroundColor Yellow
    }
    
    Write-Host ""
}

# Generate text-based chart
if ($GenerateChart) {
    Write-Host "=== PERFORMANCE COMPARISON CHART ===" -ForegroundColor Yellow
    
    # Normalize values for chart display (0-100 scale)
    $maxStartup = [math]::Max($csharpMetrics.StartupMs, $rustMetrics.StartupMs)
    $maxMemory = [math]::Max($csharpMetrics.MaxMemory, $rustMetrics.MaxMemory)
    $maxCpu = [math]::Max($csharpMetrics.AvgCpu, $rustMetrics.AvgCpu)
    
    function Get-BarChart {
        param([double]$Value, [double]$MaxValue, [int]$Width = 20)
        
        $normalizedValue = if ($MaxValue -gt 0) { $Value / $MaxValue } else { 0 }
        $barLength = [math]::Round($normalizedValue * $Width)
        $bar = "█" * $barLength + "░" * ($Width - $barLength)
        return $bar
    }
    
    Write-Host ""
    Write-Host "Startup Time (ms):" -ForegroundColor White
    Write-Host ("  C#:   {0,6}ms [{1}]" -f $csharpMetrics.StartupMs, $(Get-BarChart $csharpMetrics.StartupMs $maxStartup)) -ForegroundColor Cyan
    Write-Host ("  Rust: {0,6}ms [{1}]" -f $rustMetrics.StartupMs, $(Get-BarChart $rustMetrics.StartupMs $maxStartup)) -ForegroundColor Magenta
    Write-Host ""
    
    Write-Host "Memory Usage (MB):" -ForegroundColor White
    Write-Host ("  C#:   {0,6:F1}MB [{1}]" -f $csharpMetrics.MaxMemory, $(Get-BarChart $csharpMetrics.MaxMemory $maxMemory)) -ForegroundColor Cyan
    Write-Host ("  Rust: {0,6:F1}MB [{1}]" -f $rustMetrics.MaxMemory, $(Get-BarChart $rustMetrics.MaxMemory $maxMemory)) -ForegroundColor Magenta
    Write-Host ""
    
    Write-Host "CPU Usage (%):" -ForegroundColor White
    Write-Host ("  C#:   {0,6:F1}%  [{1}]" -f $csharpMetrics.AvgCpu, $(Get-BarChart $csharpMetrics.AvgCpu $maxCpu)) -ForegroundColor Cyan
    Write-Host ("  Rust: {0,6:F1}%  [{1}]" -f $rustMetrics.AvgCpu, $(Get-BarChart $rustMetrics.AvgCpu $maxCpu)) -ForegroundColor Magenta
    Write-Host ""
}

# Create comprehensive comparison report
$comparisonReport = @{
    metadata = @{
        comparisonDate = (Get-Date).ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
        csharpReportPath = $CSharpReport
        rustReportPath = $RustReport
        budgets = $budgets
    }
    summary = @{
        csharpPassed = $csharpMetrics.Passed
        rustPassed = $rustMetrics.Passed
        overallWinner = if ($csharpMetrics.Passed -and -not $rustMetrics.Passed) { "csharp" } 
                       elseif (-not $csharpMetrics.Passed -and $rustMetrics.Passed) { "rust" }
                       elseif ($csharpMetrics.Passed -and $rustMetrics.Passed) { "both" }
                       else { "neither" }
    }
    comparison = @{
        startup = @{
            csharpMs = $csharpMetrics.StartupMs
            rustMs = $rustMetrics.StartupMs
            difference = $startupDiff
            winner = if ($csharpMetrics.StartupMs -lt $rustMetrics.StartupMs) { "csharp" } 
                    elseif ($rustMetrics.StartupMs -lt $csharpMetrics.StartupMs) { "rust" } 
                    else { "tie" }
        }
        cpu = @{
            csharpAvgPercent = $csharpMetrics.AvgCpu
            rustAvgPercent = $rustMetrics.AvgCpu
            difference = $cpuDiff
            winner = if ($csharpMetrics.AvgCpu -lt $rustMetrics.AvgCpu) { "csharp" } 
                    elseif ($rustMetrics.AvgCpu -lt $csharpMetrics.AvgCpu) { "rust" } 
                    else { "tie" }
        }
        memory = @{
            csharpMaxMB = $csharpMetrics.MaxMemory
            rustMaxMB = $rustMetrics.MaxMemory
            difference = $memoryDiff
            winner = if ($csharpMetrics.MaxMemory -lt $rustMetrics.MaxMemory) { "csharp" } 
                    elseif ($rustMetrics.MaxMemory -lt $csharpMetrics.MaxMemory) { "rust" } 
                    else { "tie" }
        }
    }
    sourceReports = @{
        csharp = $csharpData
        rust = $rustData
    }
}

# Save comparison report
try {
    $comparisonReport | ConvertTo-Json -Depth 15 | Set-Content $OutputPath -Encoding UTF8
    Write-Host "[COMPARE] Comparison report saved to: $OutputPath" -ForegroundColor Green
} catch {
    Write-Warning "[COMPARE] Failed to save comparison report: $($_.Exception.Message)"
}

Write-Host "=================================================================" -ForegroundColor White
Write-Host ""

# Exit with appropriate code based on overall results
if ($comparisonReport.summary.overallWinner -eq "both") {
    Write-Host "[COMPARE] Comparison completed - Both implementations meet performance budgets" -ForegroundColor Green
    exit 0
} elseif ($comparisonReport.summary.overallWinner -ne "neither") {
    Write-Host "[COMPARE] Comparison completed - Clear performance winner identified" -ForegroundColor Green
    exit 0
} else {
    Write-Host "[COMPARE] Comparison completed - Performance optimization needed for both" -ForegroundColor Yellow
    exit 1
}