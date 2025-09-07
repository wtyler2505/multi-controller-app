#Requires -Version 5.1
<#
.SYNOPSIS
    Run comprehensive performance comparison of C# and Rust prototypes
    
.DESCRIPTION
    Directly benchmarks both prototypes using the core benchmark.ps1 script
    and generates a detailed comparison analysis.
    
.PARAMETER MonitorDuration
    Duration in seconds to monitor each prototype (default: 60)
    
.PARAMETER ShowDetails
    Show detailed information during execution
    
.EXAMPLE
    .\run-prototype-comparison.ps1 -ShowDetails
    
.EXAMPLE
    .\run-prototype-comparison.ps1 -MonitorDuration 30
#>

[CmdletBinding()]
param(
    [int]$MonitorDuration = 60,
    [switch]$ShowDetails
)

$ErrorActionPreference = "Stop"

# Prototype paths
$CSharpExePath = "C:\Users\wtyle\multi-controller-csharp\prototypes\csharp-winui\bin\Release\net8.0-windows10.0.19041.0\win-x64\publish\MultiControllerPrototype.exe"
$RustExePath = "C:\Users\wtyle\multi-controller-rust\prototypes\rust-egui\target\release\multi-controller-rust-prototype.exe"

$timestamp = Get-Date -Format 'yyyyMMdd-HHmmss'

Write-Host ""
Write-Host "============================================================" -ForegroundColor White
Write-Host "       MULTI-CONTROLLER PROTOTYPE PERFORMANCE COMPARISON   " -ForegroundColor White
Write-Host "============================================================" -ForegroundColor White
Write-Host ""
Write-Host "Performance Budgets:" -ForegroundColor Yellow
Write-Host "  Startup Time: < 2000ms" -ForegroundColor Gray
Write-Host "  Idle CPU Usage: <= 2.0%" -ForegroundColor Gray  
Write-Host "  Base Memory: <= 150MB" -ForegroundColor Gray
Write-Host ""

# Results tracking
$results = @{
    csharpPath = ""
    rustPath = ""
    csharpSuccess = $false
    rustSuccess = $false
    startTime = Get-Date
}

# Test C# prototype
Write-Host "================= C# NATIVE AOT PROTOTYPE =================" -ForegroundColor Cyan
Write-Host ""

if (Test-Path $CSharpExePath) {
    $csharpExeInfo = Get-Item $CSharpExePath
    Write-Host "[C#] Executable: $($csharpExeInfo.Name)" -ForegroundColor Cyan
    Write-Host "[C#] Size: $([math]::Round($csharpExeInfo.Length / 1MB, 2))MB" -ForegroundColor Cyan
    Write-Host "[C#] Location: $CSharpExePath" -ForegroundColor Gray
    Write-Host ""
    
    try {
        $csharpReportPath = Join-Path $PSScriptRoot "csharp-prototype-benchmark-$timestamp.json"
        $benchmarkScript = Join-Path $PSScriptRoot "benchmark.ps1"
        
        $csharpArgs = @(
            "-ExecutablePath", $CSharpExePath
            "-BenchmarkType", "csharp-prototype"
            "-OutputPath", $csharpReportPath
            "-MonitorDuration", $MonitorDuration
        )
        
        if ($ShowDetails) { $csharpArgs += "-ShowDetails" }
        
        & powershell -ExecutionPolicy Bypass -File $benchmarkScript @csharpArgs
        
        if ($LASTEXITCODE -eq 0) {
            $results.csharpSuccess = $true
            $results.csharpPath = $csharpReportPath
            Write-Host "[C#] Benchmark completed successfully" -ForegroundColor Green
        } else {
            Write-Host "[C#] Benchmark failed with exit code: $LASTEXITCODE" -ForegroundColor Red
        }
        
    } catch {
        Write-Host "[C#] Benchmark failed with exception: $($_.Exception.Message)" -ForegroundColor Red
    }
} else {
    Write-Host "[C#] Prototype executable not found: $CSharpExePath" -ForegroundColor Red
    Write-Host "[C#] Please build the C# prototype first" -ForegroundColor Yellow
}

Write-Host ""

# Test Rust prototype
Write-Host "==================== RUST PROTOTYPE ====================" -ForegroundColor Magenta
Write-Host ""

if (Test-Path $RustExePath) {
    $rustExeInfo = Get-Item $RustExePath
    Write-Host "[RUST] Executable: $($rustExeInfo.Name)" -ForegroundColor Magenta
    Write-Host "[RUST] Size: $([math]::Round($rustExeInfo.Length / 1MB, 2))MB" -ForegroundColor Magenta
    Write-Host "[RUST] Location: $RustExePath" -ForegroundColor Gray
    Write-Host ""
    
    try {
        $rustReportPath = Join-Path $PSScriptRoot "rust-prototype-benchmark-$timestamp.json"
        $benchmarkScript = Join-Path $PSScriptRoot "benchmark.ps1"
        
        $rustArgs = @(
            "-ExecutablePath", $RustExePath
            "-BenchmarkType", "rust-prototype"
            "-OutputPath", $rustReportPath
            "-MonitorDuration", $MonitorDuration
        )
        
        if ($ShowDetails) { $rustArgs += "-ShowDetails" }
        
        & powershell -ExecutionPolicy Bypass -File $benchmarkScript @rustArgs
        
        if ($LASTEXITCODE -eq 0) {
            $results.rustSuccess = $true
            $results.rustPath = $rustReportPath
            Write-Host "[RUST] Benchmark completed successfully" -ForegroundColor Green
        } else {
            Write-Host "[RUST] Benchmark failed with exit code: $LASTEXITCODE" -ForegroundColor Red
        }
        
    } catch {
        Write-Host "[RUST] Benchmark failed with exception: $($_.Exception.Message)" -ForegroundColor Red
    }
} else {
    Write-Host "[RUST] Prototype executable not found: $RustExePath" -ForegroundColor Red
    Write-Host "[RUST] Please build the Rust prototype with 'cargo build --release'" -ForegroundColor Yellow
}

Write-Host ""

# Generate comparison if both succeeded
if ($results.csharpSuccess -and $results.rustSuccess) {
    Write-Host "=================== COMPARISON ANALYSIS ==================" -ForegroundColor White
    Write-Host ""
    
    try {
        $compareScript = Join-Path $PSScriptRoot "compare-benchmarks.ps1"
        $comparisonPath = Join-Path $PSScriptRoot "prototype-comparison-$timestamp.json"
        
        $compareArgs = @(
            "-CSharpReport", $results.csharpPath
            "-RustReport", $results.rustPath
            "-OutputPath", $comparisonPath
            "-GenerateChart"
        )
        
        if ($ShowDetails) { $compareArgs += "-ShowDetails" }
        
        & powershell -ExecutionPolicy Bypass -File $compareScript @compareArgs
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "[COMPARISON] Analysis completed successfully" -ForegroundColor Green
            Write-Host "[COMPARISON] Report saved: $(Split-Path $comparisonPath -Leaf)" -ForegroundColor Cyan
        } else {
            Write-Host "[COMPARISON] Analysis completed with warnings" -ForegroundColor Yellow
        }
        
    } catch {
        Write-Host "[COMPARISON] Analysis failed: $($_.Exception.Message)" -ForegroundColor Red
    }
}

# Final summary
$results.endTime = Get-Date
$totalDuration = ($results.endTime - $results.startTime).TotalSeconds

Write-Host ""
Write-Host "==================== EXECUTION SUMMARY ===================" -ForegroundColor White
Write-Host "Total Duration: $([math]::Round($totalDuration, 1)) seconds" -ForegroundColor Gray
Write-Host "Start Time: $($results.startTime.ToString('HH:mm:ss'))" -ForegroundColor Gray
Write-Host "End Time: $($results.endTime.ToString('HH:mm:ss'))" -ForegroundColor Gray
Write-Host ""

if ($results.csharpSuccess) {
    Write-Host "[OK] C# Native AOT prototype benchmark completed" -ForegroundColor Green
} else {
    Write-Host "[FAILED] C# Native AOT prototype benchmark failed" -ForegroundColor Red
}

if ($results.rustSuccess) {
    Write-Host "[OK] Rust prototype benchmark completed" -ForegroundColor Green
} else {
    Write-Host "[FAILED] Rust prototype benchmark failed" -ForegroundColor Red
}

if ($results.csharpSuccess -and $results.rustSuccess) {
    Write-Host "[OK] Prototype comparison analysis generated" -ForegroundColor Green
    Write-Host ""
    Write-Host "Key Findings Summary:" -ForegroundColor Yellow
    Write-Host "  - Review reports for detailed performance metrics" -ForegroundColor Gray
    Write-Host "  - Check performance budget compliance" -ForegroundColor Gray
    Write-Host "  - Consider optimization opportunities" -ForegroundColor Gray
    Write-Host "  - Use results for technology stack decision" -ForegroundColor Gray
}

Write-Host ""
Write-Host "Reports saved in: $PSScriptRoot" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor White

# Exit with appropriate code
if ($results.csharpSuccess -or $results.rustSuccess) {
    exit 0
} else {
    exit 1
}