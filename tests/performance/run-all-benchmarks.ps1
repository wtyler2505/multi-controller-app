#Requires -Version 5.1
<#
.SYNOPSIS
    Master script to run all performance benchmarks and generate comparison
    
.DESCRIPTION
    Orchestrates performance testing for both C# Native AOT and Rust prototypes.
    Builds both implementations, runs benchmarks, and generates comprehensive comparison.
    
.PARAMETER SkipCSharp
    Skip C# Native AOT benchmark
    
.PARAMETER SkipRust
    Skip Rust benchmark
    
.PARAMETER SkipBuild
    Skip build steps for both implementations
    
.PARAMETER Configuration
    C# build configuration: 'Release' or 'Debug' (default: Release)
    
.PARAMETER RustProfile
    Rust build profile: 'release' or 'dev' (default: release)
    
.PARAMETER ShowDetails
    Show detailed information during execution
    
.PARAMETER MonitorDuration
    Duration in seconds to monitor each prototype (default: 60)
    
.PARAMETER OutputDir
    Directory to save all benchmark reports (default: current directory)
    
.PARAMETER KeepReports
    Keep individual benchmark reports after comparison
    
.EXAMPLE
    .\run-all-benchmarks.ps1 -ShowDetails
    
.EXAMPLE
    .\run-all-benchmarks.ps1 -MonitorDuration 120 -OutputDir "reports" -KeepReports
    
.EXAMPLE
    .\run-all-benchmarks.ps1 -SkipBuild -ShowDetails
#>

[CmdletBinding()]
param(
    [switch]$SkipCSharp,
    
    [switch]$SkipRust,
    
    [switch]$SkipBuild,
    
    [ValidateSet('Release', 'Debug')]
    [string]$Configuration = 'Release',
    
    [ValidateSet('release', 'dev')]
    [string]$RustProfile = 'release',
    
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
Write-Host "==================== MULTI-CONTROLLER APP ===================" -ForegroundColor White
Write-Host "              COMPREHENSIVE PERFORMANCE BENCHMARK              " -ForegroundColor White
Write-Host "===============================================================" -ForegroundColor White
Write-Host ""
Write-Host "Configuration:" -ForegroundColor Cyan
Write-Host "  C# Configuration: $Configuration" -ForegroundColor Gray
Write-Host "  Rust Profile: $RustProfile" -ForegroundColor Gray
Write-Host "  Monitor Duration: $MonitorDuration seconds" -ForegroundColor Gray
Write-Host "  Output Directory: $OutputDir" -ForegroundColor Gray
Write-Host "  Skip Build: $SkipBuild" -ForegroundColor Gray
Write-Host ""

# Track benchmark results
$benchmarkResults = @{
    csharpPath = ""
    rustPath = ""
    csharpSuccess = $false
    rustSuccess = $false
    startTime = Get-Date
}

# C# Native AOT Benchmark
if (-not $SkipCSharp) {
    Write-Host "==================== C# NATIVE AOT BENCHMARK ==================" -ForegroundColor Cyan
    Write-Host ""
    
    try {
        $csharpScript = Join-Path $PSScriptRoot "benchmark-csharp.ps1"
        $csharpArgs = @(
            "-Configuration", $Configuration
            "-MonitorDuration", $MonitorDuration
        )
        
        if ($SkipBuild) { $csharpArgs += "-SkipBuild" }
        if ($ShowDetails) { $csharpArgs += "-ShowDetails" }
        
        # Run C# benchmark
        & $csharpScript @csharpArgs
        $csharpExitCode = $LASTEXITCODE
        
        if ($csharpExitCode -eq 0) {
            Write-Host "[MASTER] C# Native AOT benchmark completed successfully" -ForegroundColor Green
            
            # Find the generated report
            $csharpReportPattern = Join-Path $PSScriptRoot "csharp-benchmark-*.json"
            $csharpReports = Get-ChildItem -Path $csharpReportPattern | Sort-Object LastWriteTime -Descending
            
            if ($csharpReports.Count -gt 0) {
                $benchmarkResults.csharpPath = $csharpReports[0].FullName
                $benchmarkResults.csharpSuccess = $true
                Write-Host "[MASTER] C# report found: $($csharpReports[0].Name)" -ForegroundColor Green
            } else {
                Write-Warning "[MASTER] C# benchmark report not found"
            }
        } else {
            Write-Host "[MASTER] C# Native AOT benchmark failed with exit code: $csharpExitCode" -ForegroundColor Red
        }
        
    } catch {
        Write-Error "[MASTER] C# benchmark failed with exception: $($_.Exception.Message)"
    }
    
    Write-Host ""
} else {
    Write-Host "[MASTER] Skipping C# Native AOT benchmark" -ForegroundColor Yellow
    Write-Host ""
}

# Rust Benchmark
if (-not $SkipRust) {
    Write-Host "====================== RUST BENCHMARK ======================" -ForegroundColor Magenta
    Write-Host ""
    
    try {
        $rustScript = Join-Path $PSScriptRoot "benchmark-rust.ps1"
        $rustArgs = @(
            "-Profile", $RustProfile
            "-MonitorDuration", $MonitorDuration
        )
        
        if ($SkipBuild) { $rustArgs += "-SkipBuild" }
        if ($ShowDetails) { $rustArgs += "-ShowDetails" }
        
        # Run Rust benchmark
        & $rustScript @rustArgs
        $rustExitCode = $LASTEXITCODE
        
        if ($rustExitCode -eq 0) {
            Write-Host "[MASTER] Rust benchmark completed successfully" -ForegroundColor Green
            
            # Find the generated report
            $rustReportPattern = Join-Path $PSScriptRoot "rust-benchmark-*.json"
            $rustReports = Get-ChildItem -Path $rustReportPattern | Sort-Object LastWriteTime -Descending
            
            if ($rustReports.Count -gt 0) {
                $benchmarkResults.rustPath = $rustReports[0].FullName
                $benchmarkResults.rustSuccess = $true
                Write-Host "[MASTER] Rust report found: $($rustReports[0].Name)" -ForegroundColor Green
            } else {
                Write-Warning "[MASTER] Rust benchmark report not found"
            }
        } else {
            Write-Host "[MASTER] Rust benchmark failed with exit code: $rustExitCode" -ForegroundColor Red
        }
        
    } catch {
        Write-Error "[MASTER] Rust benchmark failed with exception: $($_.Exception.Message)"
    }
    
    Write-Host ""
} else {
    Write-Host "[MASTER] Skipping Rust benchmark" -ForegroundColor Yellow
    Write-Host ""
}

# Comparison and Analysis
if ($benchmarkResults.csharpSuccess -and $benchmarkResults.rustSuccess) {
    Write-Host "==================== COMPARISON ANALYSIS ===================" -ForegroundColor White
    Write-Host ""
    
    try {
        $compareScript = Join-Path $PSScriptRoot "compare-benchmarks.ps1"
        $comparisonPath = Join-Path $OutputDir "benchmark-comparison-$timestamp.json"
        
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
    Write-Host "==================== SINGLE BENCHMARK RESULT ===============" -ForegroundColor Yellow
    Write-Host ""
    
    if ($benchmarkResults.csharpSuccess) {
        Write-Host "[MASTER] Only C# benchmark completed successfully" -ForegroundColor Cyan
        Write-Host "[MASTER] C# report: $(Split-Path $benchmarkResults.csharpPath -Leaf)" -ForegroundColor Gray
    } else {
        Write-Host "[MASTER] Only Rust benchmark completed successfully" -ForegroundColor Magenta
        Write-Host "[MASTER] Rust report: $(Split-Path $benchmarkResults.rustPath -Leaf)" -ForegroundColor Gray
    }
    
    Write-Host "[MASTER] Run both benchmarks for comparison analysis" -ForegroundColor Yellow
    
} else {
    Write-Host "==================== BENCHMARK FAILURE ===================" -ForegroundColor Red
    Write-Host ""
    Write-Host "[MASTER] No benchmarks completed successfully" -ForegroundColor Red
    Write-Host "[MASTER] Check the error messages above for details" -ForegroundColor Red
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
            $comparisonExists = Test-Path (Join-Path $OutputDir "benchmark-comparison-$timestamp.json")
            if ($comparisonExists) {
                Write-Host "[MASTER] Cleaning up individual reports (use -KeepReports to retain them)" -ForegroundColor Gray
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
    Write-Host "[OK] C# Native AOT benchmark completed" -ForegroundColor Green
} elseif (-not $SkipCSharp) {
    Write-Host "[FAILED] C# Native AOT benchmark failed" -ForegroundColor Red
}

if ($benchmarkResults.rustSuccess) {
    Write-Host "[OK] Rust benchmark completed" -ForegroundColor Green
} elseif (-not $SkipRust) {
    Write-Host "[FAILED] Rust benchmark failed" -ForegroundColor Red
}

if ($benchmarkResults.csharpSuccess -and $benchmarkResults.rustSuccess) {
    Write-Host "[OK] Comparison analysis generated" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "  1. Review the comparison report for detailed analysis" -ForegroundColor Gray
    Write-Host "  2. Check for performance budget violations" -ForegroundColor Gray
    Write-Host "  3. Consider optimizations for failing metrics" -ForegroundColor Gray
    Write-Host "  4. Use results for technology stack decision" -ForegroundColor Gray
    Write-Host ""
    Write-Host "Reports saved in: $OutputDir" -ForegroundColor Cyan
}

Write-Host "=========================================================" -ForegroundColor White
Write-Host ""

# Exit with appropriate code
$overallSuccess = $benchmarkResults.csharpSuccess -or $benchmarkResults.rustSuccess
if ($overallSuccess) {
    exit 0
} else {
    exit 1
}