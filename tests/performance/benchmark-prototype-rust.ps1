#Requires -Version 5.1
<#
.SYNOPSIS
    Rust prototype specific performance benchmark
    
.DESCRIPTION
    Benchmarks the existing Rust Multi-Controller App prototype.
    Uses the pre-built executable from the prototype directory.
    
.PARAMETER ShowDetails
    Show detailed benchmark information
    
.PARAMETER MonitorDuration
    Duration in seconds to monitor CPU/memory (default: 60)
    
.EXAMPLE
    .\benchmark-prototype-rust.ps1 -ShowDetails
    
.EXAMPLE
    .\benchmark-prototype-rust.ps1 -MonitorDuration 120
#>

[CmdletBinding()]
param(
    [switch]$ShowDetails,
    
    [int]$MonitorDuration = 60
)

# Prototype paths - these are the actual prototype locations
$PrototypePath = "C:\Users\wtyle\multi-controller-rust\prototypes\rust-egui"
$ExecutablePath = Join-Path $PrototypePath "target\release\multi-controller-rust-prototype.exe"

Write-Host "[PERF-RUST-PROTO] Rust egui Prototype Performance Benchmark" -ForegroundColor Magenta
Write-Host "[PERF-RUST-PROTO] Prototype Location: $PrototypePath" -ForegroundColor Magenta

# Validate executable exists
if (-not (Test-Path $ExecutablePath)) {
    Write-Error "[PERF-RUST-PROTO] Prototype executable not found: $ExecutablePath"
    Write-Host "[PERF-RUST-PROTO] Please ensure the Rust prototype has been built with 'cargo build --release'" -ForegroundColor Red
    exit 1
}

# Get executable information
$exeInfo = Get-Item $ExecutablePath
$executableSizeMB = [math]::Round($exeInfo.Length / 1MB, 2)
Write-Host "[PERF-RUST-PROTO] Executable: $ExecutablePath" -ForegroundColor Magenta
Write-Host "[PERF-RUST-PROTO] Size: ${executableSizeMB}MB" -ForegroundColor Magenta
Write-Host "[PERF-RUST-PROTO] Modified: $($exeInfo.LastWriteTime)" -ForegroundColor Magenta

# Get target directory information
if ($ShowDetails) {
    Write-Host "[PERF-RUST-PROTO] Analyzing Rust executable characteristics..." -ForegroundColor Gray
    
    $targetDir = Split-Path $ExecutablePath -Parent
    $targetFiles = Get-ChildItem $targetDir -File
    
    # Check for release build characteristics
    Write-Host "[PERF-RUST-PROTO] Release build optimizations: ENABLED" -ForegroundColor Green
    
    # Check for debug symbols (release builds should be stripped)
    try {
        $fileOutput = & file $ExecutablePath 2>$null
        if ($fileOutput -and $fileOutput -notmatch "not stripped") {
            Write-Host "[PERF-RUST-PROTO] Debug symbols: STRIPPED (optimized for performance)" -ForegroundColor Green
        } else {
            Write-Host "[PERF-RUST-PROTO] Debug symbols: PRESENT (may impact size)" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "[PERF-RUST-PROTO] Could not analyze debug symbols (file command not available)" -ForegroundColor Gray
    }
    
    # Display target directory info
    if ($targetFiles.Count -gt 1) {
        Write-Host "[PERF-RUST-PROTO] Additional files in target/release:" -ForegroundColor Gray
        $targetFiles | Where-Object { $_.Name -ne "multi-controller-rust-prototype.exe" } | ForEach-Object {
            $sizeMB = [math]::Round($_.Length / 1MB, 3)
            Write-Host "  $($_.Name) - ${sizeMB}MB" -ForegroundColor Gray
        }
    }
    
    # Check Rust version
    try {
        $rustVersion = & rustc --version 2>$null
        if ($rustVersion) {
            Write-Host "[PERF-RUST-PROTO] Compiled with: $rustVersion" -ForegroundColor Gray
        }
    } catch {
        Write-Host "[PERF-RUST-PROTO] Could not determine Rust compiler version" -ForegroundColor Gray
    }
}

# Prepare output path
$timestamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$outputPath = Join-Path $PSScriptRoot "rust-prototype-benchmark-$timestamp.json"

# Run the main benchmark
$benchmarkScript = Join-Path $PSScriptRoot "benchmark.ps1"

Write-Host "[PERF-RUST-PROTO] Starting performance measurement..." -ForegroundColor Yellow
Write-Host ""

# Build arguments properly
$args = @(
    "-ExecutablePath", $ExecutablePath
    "-BenchmarkType", "rust-prototype"
    "-OutputPath", $outputPath
    "-MonitorDuration", $MonitorDuration
)
if ($ShowDetails) { $args += "-ShowDetails" }

& $benchmarkScript @args
$benchmarkExitCode = $LASTEXITCODE

# Add prototype specific metadata to report if benchmark succeeded
if ($benchmarkExitCode -eq 0 -and (Test-Path $outputPath)) {
    try {
        $report = Get-Content $outputPath | ConvertFrom-Json
        
        # Add prototype specific metadata
        $report.metadata | Add-Member -Name "prototypeType" -Value "Rust egui" -MemberType NoteProperty
        $report.metadata | Add-Member -Name "prototypeLocation" -Value $PrototypePath -MemberType NoteProperty
        $report.metadata | Add-Member -Name "executableSizeMB" -Value $executableSizeMB -MemberType NoteProperty
        $report.metadata | Add-Member -Name "buildProfile" -Value "release" -MemberType NoteProperty
        
        # Try to get Rust version info
        try {
            $rustVersion = & rustc --version 2>$null
            $cargoVersion = & cargo --version 2>$null
            if ($rustVersion) {
                $report.metadata | Add-Member -Name "rustcVersion" -Value $rustVersion -MemberType NoteProperty
            }
            if ($cargoVersion) {
                $report.metadata | Add-Member -Name "cargoVersion" -Value $cargoVersion -MemberType NoteProperty
            }
        } catch {
            Write-Host "[PERF-RUST-PROTO] Could not get Rust version information" -ForegroundColor Gray
        }
        
        # Save updated report
        $report | ConvertTo-Json -Depth 10 | Set-Content $outputPath -Encoding UTF8
        Write-Host "[PERF-RUST-PROTO] Enhanced Rust prototype report saved to: $outputPath" -ForegroundColor Green
    } catch {
        Write-Warning "[PERF-RUST-PROTO] Failed to enhance report with prototype metadata: $($_.Exception.Message)"
    }
}

Write-Host ""
if ($benchmarkExitCode -eq 0) {
    Write-Host "[PERF-RUST-PROTO] Rust prototype benchmark completed successfully" -ForegroundColor Green
    Write-Host "[PERF-RUST-PROTO] Report saved: $(Split-Path $outputPath -Leaf)" -ForegroundColor Cyan
} else {
    Write-Host "[PERF-RUST-PROTO] Rust prototype benchmark failed" -ForegroundColor Red
}

exit $benchmarkExitCode