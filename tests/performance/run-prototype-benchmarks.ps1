#Requires -Version 5.1
<#
.SYNOPSIS
    Master script to run performance benchmarks for both prototypes
    
.DESCRIPTION
    Orchestrates performance testing for both C# Native AOT and Rust prototypes.
    Uses the existing pre-built prototype executables and generates comprehensive comparison.
    
.PARAMETER SkipCSharp
    Skip C# Native AOT prototype benchmark
    
.PARAMETER SkipRust
    Skip Rust prototype benchmark
    
.PARAMETER ShowDetails
    Show detailed information during execution
    
.PARAMETER MonitorDuration
    Duration in seconds to monitor each prototype (default: 60)
    
.PARAMETER OutputDir
    Directory to save all benchmark reports (default: current directory)
    
.PARAMETER KeepReports
    Keep individual benchmark reports after comparison
    
.EXAMPLE
    .\run-prototype-benchmarks.ps1 -ShowDetails
    
.EXAMPLE
    .\run-prototype-benchmarks.ps1 -MonitorDuration 120 -OutputDir "reports" -KeepReports
    
.EXAMPLE
    .\run-prototype-benchmarks.ps1 -SkipRust -ShowDetails
#>

[CmdletBinding()]
param(
    [switch]$SkipCSharp,
    
    [switch]$SkipRust,
    
    [switch]$ShowDetails,
    
    [int]$MonitorDuration = 60,
    
    [string]$OutputDir = $PSScriptRoot,
    
    [switch]$KeepReports
)

$ErrorActionPreference = "Stop"

# Ensure output directory exists
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
    Write-Host "[MASTER] Created output directory: $OutputDir" -ForegroundColor Green
}

$OutputDir = Resolve-Path $OutputDir
$timestamp = Get-Date -Format 'yyyyMMdd-HHmmss'

Write-Host ""
Write-Host "============================================================" -ForegroundColor White
Write-Host "     MULTI-CONTROLLER APP PROTOTYPE PERFORMANCE BENCHMARK  " -ForegroundColor White
Write-Host "============================================================" -ForegroundColor White
Write-Host ""
Write-Host "Configuration:" -ForegroundColor Cyan
Write-Host "  Monitor Duration: $MonitorDuration seconds" -ForegroundColor Gray
Write-Host "  Output Directory: $OutputDir" -ForegroundColor Gray
Write-Host "  Skip C#: $SkipCSharp" -ForegroundColor Gray
Write-Host "  Skip Rust: $SkipRust" -ForegroundColor Gray
Write-Host ""

# Performance Budgets
Write-Host "Performance Budgets:" -ForegroundColor Yellow
Write-Host "  Startup Time: < 2000ms" -ForegroundColor Gray
Write-Host "  Idle CPU Usage: <= 2.0%" -ForegroundColor Gray  
Write-Host "  Base Memory: <= 150MB" -ForegroundColor Gray
Write-Host ""

# Track benchmark results
$benchmarkResults = @{
    csharpPath = ""
    rustPath = ""
    csharpSuccess = $false
    rustSuccess = $false
    startTime = Get-Date
}

# C# Native AOT Prototype Benchmark
if (-not $SkipCSharp) {
    Write-Host "================= C# NATIVE AOT PROTOTYPE =================" -ForegroundColor Cyan
    Write-Host ""
    
    try {
        $csharpScript = Join-Path $PSScriptRoot "benchmark-prototype-csharp.ps1"
        $csharpArgs = @(
            "-MonitorDuration", $MonitorDuration
        )
        
        if ($ShowDetails) { $csharpArgs += "-ShowDetails" }
        
        # Run C# prototype benchmark
        & $csharpScript @csharpArgs
        $csharpExitCode = $LASTEXITCODE
        
        if ($csharpExitCode -eq 0) {
            Write-Host "[MASTER] C# prototype benchmark completed successfully" -ForegroundColor Green
            
            # Find the generated report
            $csharpReportPattern = Join-Path $PSScriptRoot "csharp-prototype-benchmark-*.json"
            $csharpReports = Get-ChildItem -Path $csharpReportPattern | Sort-Object LastWriteTime -Descending
            
            if ($csharpReports.Count -gt 0) {
                $benchmarkResults.csharpPath = $csharpReports[0].FullName
                $benchmarkResults.csharpSuccess = $true
                Write-Host "[MASTER] C# report found: $($csharpReports[0].Name)" -ForegroundColor Green
            } else {
                Write-Warning "[MASTER] C# prototype benchmark report not found"
            }
        } else {
            Write-Host "[MASTER] C# prototype benchmark failed with exit code: $csharpExitCode" -ForegroundColor Red
        }
        
    } catch {
        Write-Error "[MASTER] C# prototype benchmark failed with exception: $($_.Exception.Message)"
    }
    
    Write-Host ""
} else {
    Write-Host "[MASTER] Skipping C# Native AOT prototype benchmark" -ForegroundColor Yellow
    Write-Host ""
}

# Rust Prototype Benchmark
if (-not $SkipRust) {
    Write-Host "==================== RUST PROTOTYPE ====================" -ForegroundColor Magenta
    Write-Host ""
    
    try {
        $rustScript = Join-Path $PSScriptRoot "benchmark-prototype-rust.ps1"
        $rustArgs = @(
            "-MonitorDuration", $MonitorDuration
        )
        
        if ($ShowDetails) { $rustArgs += "-ShowDetails" }
        
        # Run Rust prototype benchmark
        & $rustScript @rustArgs
        $rustExitCode = $LASTEXITCODE
        
        if ($rustExitCode -eq 0) {
            Write-Host "[MASTER] Rust prototype benchmark completed successfully" -ForegroundColor Green
            
            # Find the generated report
            $rustReportPattern = Join-Path $PSScriptRoot "rust-prototype-benchmark-*.json"
            $rustReports = Get-ChildItem -Path $rustReportPattern | Sort-Object LastWriteTime -Descending
            
            if ($rustReports.Count -gt 0) {
                $benchmarkResults.rustPath = $rustReports[0].FullName
                $benchmarkResults.rustSuccess = $true
                Write-Host "[MASTER] Rust report found: $($rustReports[0].Name)" -ForegroundColor Green
            } else {
                Write-Warning "[MASTER] Rust prototype benchmark report not found"
            }
        } else {
            Write-Host "[MASTER] Rust prototype benchmark failed with exit code: $rustExitCode" -ForegroundColor Red
        }
        
    } catch {
        Write-Error "[MASTER] Rust prototype benchmark failed with exception: $($_.Exception.Message)"
    }
    
    Write-Host ""
} else {
    Write-Host "[MASTER] Skipping Rust prototype benchmark" -ForegroundColor Yellow
    Write-Host ""
}

# Comparison and Analysis
if ($benchmarkResults.csharpSuccess -and $benchmarkResults.rustSuccess) {
    Write-Host "=================== COMPARISON ANALYSIS ==================" -ForegroundColor White
    Write-Host ""
    
    try {
        $compareScript = Join-Path $PSScriptRoot "compare-benchmarks.ps1"
        $comparisonPath = Join-Path $OutputDir "prototype-comparison-$timestamp.json"
        
        $compareArgs = @(
            "-CSharpReport", $benchmarkResults.csharpPath
            "-RustReport", $benchmarkResults.rustPath
            "-OutputPath", $comparisonPath
            "-GenerateChart"
        )
        
        if ($ShowDetails) { $compareArgs += "-ShowDetails" }
        
        & $compareScript @compareArgs
        $compareExitCode = $LASTEXITCODE
        
        if ($compareExitCode -eq 0) {
            Write-Host "[MASTER] Comparison analysis completed successfully" -ForegroundColor Green
        } else {
            Write-Host "[MASTER] Comparison analysis completed with warnings" -ForegroundColor Yellow
        }
        
    } catch {
        Write-Error "[MASTER] Comparison analysis failed: $($_.Exception.Message)"
    }
    
} elseif ($benchmarkResults.csharpSuccess -or $benchmarkResults.rustSuccess) {
    Write-Host "================= SINGLE PROTOTYPE RESULT =================" -ForegroundColor Yellow
    Write-Host ""
    
    if ($benchmarkResults.csharpSuccess) {
        Write-Host "[MASTER] Only C# prototype benchmark completed successfully" -ForegroundColor Cyan
        Write-Host "[MASTER] C# report: $(Split-Path $benchmarkResults.csharpPath -Leaf)" -ForegroundColor Gray
    } else {
        Write-Host "[MASTER] Only Rust prototype benchmark completed successfully" -ForegroundColor Magenta
        Write-Host "[MASTER] Rust report: $(Split-Path $benchmarkResults.rustPath -Leaf)" -ForegroundColor Gray
    }
    
    Write-Host "[MASTER] Run both prototypes for comparison analysis" -ForegroundColor Yellow
    
} else {
    Write-Host "================== BENCHMARK FAILURE ==================" -ForegroundColor Red
    Write-Host ""
    Write-Host "[MASTER] No prototype benchmarks completed successfully" -ForegroundColor Red
    Write-Host "[MASTER] Check the error messages above for details" -ForegroundColor Red
    Write-Host ""
    Write-Host "Common issues:" -ForegroundColor Yellow
    Write-Host "  - C# prototype not built: Build the csharp-winui prototype first" -ForegroundColor Gray
    Write-Host "  - Rust prototype not built: Run 'cargo build --release' in rust-egui prototype" -ForegroundColor Gray
    Write-Host "  - Missing dependencies: Check runtime requirements" -ForegroundColor Gray
}

# File Management
if ($benchmarkResults.csharpSuccess -or $benchmarkResults.rustSuccess) {
    Write-Host ""
    Write-Host "==================== FILE MANAGEMENT ====================" -ForegroundColor White
    
    # Move reports to output directory if different from current
    if ($OutputDir -ne $PSScriptRoot) {
        if ($benchmarkResults.csharpSuccess) {
            $csharpDestination = Join-Path $OutputDir (Split-Path $benchmarkResults.csharpPath -Leaf)
            Move-Item $benchmarkResults.csharpPath $csharpDestination -Force
            Write-Host "[MASTER] Moved C# report to: $csharpDestination" -ForegroundColor Green
            $benchmarkResults.csharpPath = $csharpDestination
        }
        
        if ($benchmarkResults.rustSuccess) {
            $rustDestination = Join-Path $OutputDir (Split-Path $benchmarkResults.rustPath -Leaf)
            Move-Item $benchmarkResults.rustPath $rustDestination -Force
            Write-Host "[MASTER] Moved Rust report to: $rustDestination" -ForegroundColor Green
            $benchmarkResults.rustPath = $rustDestination
        }
    }
    
    # Clean up individual reports if not keeping them
    if (-not $KeepReports) {
        if ($benchmarkResults.csharpSuccess -and $benchmarkResults.rustSuccess) {
            # Only clean up if we have a comparison report
            $comparisonExists = Test-Path (Join-Path $OutputDir "prototype-comparison-$timestamp.json")
            if ($comparisonExists) {
                Write-Host "[MASTER] Cleaning up individual reports (use -KeepReports to retain)" -ForegroundColor Gray
                if ($benchmarkResults.csharpPath) { Remove-Item $benchmarkResults.csharpPath -Force }
                if ($benchmarkResults.rustPath) { Remove-Item $benchmarkResults.rustPath -Force }
            }
        }
    } else {
        Write-Host "[MASTER] Keeping all individual benchmark reports" -ForegroundColor Gray
    }
}

# Final Summary
$benchmarkResults.endTime = Get-Date
$totalDuration = ($benchmarkResults.endTime - $benchmarkResults.startTime).TotalSeconds

Write-Host ""
Write-Host "==================== EXECUTION SUMMARY ===================" -ForegroundColor White
Write-Host "Total Duration: $([math]::Round($totalDuration, 1)) seconds" -ForegroundColor Gray
Write-Host "Start Time: $($benchmarkResults.startTime.ToString('yyyy-MM-dd HH:mm:ss'))" -ForegroundColor Gray
Write-Host "End Time: $($benchmarkResults.endTime.ToString('yyyy-MM-dd HH:mm:ss'))" -ForegroundColor Gray
Write-Host ""

if ($benchmarkResults.csharpSuccess) {
    Write-Host "[OK] C# Native AOT prototype benchmark completed" -ForegroundColor Green
} elseif (-not $SkipCSharp) {
    Write-Host "[FAILED] C# Native AOT prototype benchmark failed" -ForegroundColor Red
}

if ($benchmarkResults.rustSuccess) {
    Write-Host "[OK] Rust prototype benchmark completed" -ForegroundColor Green
} elseif (-not $SkipRust) {
    Write-Host "[FAILED] Rust prototype benchmark failed" -ForegroundColor Red
}

if ($benchmarkResults.csharpSuccess -and $benchmarkResults.rustSuccess) {
    Write-Host "[OK] Comparison analysis generated" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps for prototype evaluation:" -ForegroundColor Yellow
    Write-Host "  1. Review the comparison report for detailed analysis" -ForegroundColor Gray
    Write-Host "  2. Check performance budget compliance" -ForegroundColor Gray
    Write-Host "  3. Identify optimization opportunities" -ForegroundColor Gray
    Write-Host "  4. Make informed technology stack decision" -ForegroundColor Gray
    Write-Host "  5. Document findings in decision log" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Reports saved in: $OutputDir" -ForegroundColor Cyan
}

Write-Host "============================================================" -ForegroundColor White
Write-Host ""

# Exit with appropriate code
$overallSuccess = $benchmarkResults.csharpSuccess -or $benchmarkResults.rustSuccess
if ($overallSuccess) {
    exit 0
} else {
    exit 1
}