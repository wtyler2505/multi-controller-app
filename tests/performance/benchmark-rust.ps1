#Requires -Version 5.1
<#
.SYNOPSIS
    Rust specific performance benchmark helper
    
.DESCRIPTION
    Builds and benchmarks the Rust Multi-Controller App prototype.
    Handles Cargo build configuration, executable location, and Rust-specific measurements.
    
.PARAMETER Profile
    Build profile: 'release' or 'dev' (default: release)
    
.PARAMETER Target
    Target triple for cross-compilation (default: current platform)
    
.PARAMETER ShowDetails
    Show detailed build and benchmark information
    
.PARAMETER SkipBuild
    Skip the build step and benchmark existing executable
    
.PARAMETER MonitorDuration
    Duration in seconds to monitor CPU/memory (default: 60)
    
.PARAMETER Features
    Comma-separated list of Cargo features to enable
    
.EXAMPLE
    .\benchmark-rust.ps1 -ShowDetails
    
.EXAMPLE
    .\benchmark-rust.ps1 -Profile release -MonitorDuration 120
    
.EXAMPLE
    .\benchmark-rust.ps1 -SkipBuild -ShowDetails
    
.EXAMPLE
    .\benchmark-rust.ps1 -Features "gui,telemetry" -ShowDetails
#>

[CmdletBinding()]
param(
    [ValidateSet('release', 'dev')]
    [string]$Profile = 'release',
    
    [string]$Target = '',
    
    [switch]$ShowDetails,
    
    [switch]$SkipBuild,
    
    [int]$MonitorDuration = 60,
    
    [string]$Features = ''
)

# Project paths
$ProjectRoot = Resolve-Path "$PSScriptRoot\..\.."
$RustProjectPath = Join-Path $ProjectRoot "apps\rust-app"  # Assuming future Rust app location
$CargoToml = Join-Path $RustProjectPath "Cargo.toml"

Write-Host "[PERF-RUST] Rust Performance Benchmark" -ForegroundColor Cyan
Write-Host "[PERF-RUST] Profile: $Profile" -ForegroundColor Cyan
if ($Target) { Write-Host "[PERF-RUST] Target: $Target" -ForegroundColor Cyan }
if ($Features) { Write-Host "[PERF-RUST] Features: $Features" -ForegroundColor Cyan }

# Check if Rust project exists
if (-not (Test-Path $CargoToml)) {
    Write-Host "[PERF-RUST] Rust project not found at: $RustProjectPath" -ForegroundColor Yellow
    Write-Host "[PERF-RUST] Creating placeholder Rust project structure..." -ForegroundColor Yellow
    
    # Create basic Rust project structure for future implementation
    New-Item -ItemType Directory -Path $RustProjectPath -Force | Out-Null
    
    $cargoContent = @"
[package]
name = "multi-controller-app"
version = "0.1.0"
edition = "2021"

[dependencies]
egui = "0.24"
eframe = "0.24"
tokio = { version = "1.0", features = ["full"] }
tokio-serial = "5.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[features]
default = ["gui"]
gui = []
telemetry = []
charts = []

[[bin]]
name = "multi-controller-app"
path = "src/main.rs"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
opt-level = 0
debug = true
"@

    $mainContent = @"
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    
    println!("[PERF] Rust Multi-Controller App starting...");
    
    // Simulate application initialization
    std::thread::sleep(std::time::Duration::from_millis(500));
    
    let startup_time = start_time.elapsed();
    println!("[PERF] Startup time: {}ms", startup_time.as_millis());
    
    if startup_time.as_millis() > 2000 {
        eprintln!("[PERF] WARNING: Startup time exceeds 2s budget");
        std::process::exit(1);
    }
    
    println!("[PERF] Application ready");
    
    // Keep running for benchmarking
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        // In a real app, this would be the main event loop
    }
}
"@

    Set-Content -Path $CargoToml -Value $cargoContent -Encoding UTF8
    New-Item -ItemType Directory -Path (Join-Path $RustProjectPath "src") -Force | Out-Null
    Set-Content -Path (Join-Path $RustProjectPath "src\main.rs") -Value $mainContent -Encoding UTF8
    
    Write-Host "[PERF-RUST] Created placeholder Rust project for benchmarking" -ForegroundColor Green
}

# Validate Cargo is available
try {
    & cargo --version | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "Cargo not found"
    }
} catch {
    Write-Error "[PERF-RUST] Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
}

# Build the project (unless skipped)
if (-not $SkipBuild) {
    Write-Host "[PERF-RUST] Building Rust project..." -ForegroundColor Yellow
    
    $buildStart = Get-Date
    Set-Location $RustProjectPath
    
    try {
        # Clean first
        $cleanArgs = @("clean")
        if ($ShowDetails) {
            Write-Host "[PERF-RUST] Running: cargo $($cleanArgs -join ' ')" -ForegroundColor Gray
        }
        
        & cargo @cleanArgs
        if ($LASTEXITCODE -ne 0) {
            throw "Clean failed with exit code $LASTEXITCODE"
        }
        
        # Build with specified profile
        $buildArgs = @("build")
        
        if ($Profile -eq "release") {
            $buildArgs += "--release"
        }
        
        if ($Target) {
            $buildArgs += "--target", $Target
        }
        
        if ($Features) {
            $buildArgs += "--features", $Features
        }
        
        if ($ShowDetails) {
            Write-Host "[PERF-RUST] Running: cargo $($buildArgs -join ' ')" -ForegroundColor Gray
        } else {
            $buildArgs += "--quiet"
        }
        
        & cargo @buildArgs
        if ($LASTEXITCODE -ne 0) {
            throw "Build failed with exit code $LASTEXITCODE"
        }
        
        $buildEnd = Get-Date
        $buildTime = ($buildEnd - $buildStart).TotalSeconds
        Write-Host "[PERF-RUST] Build completed in $([math]::Round($buildTime, 1)) seconds" -ForegroundColor Green
        
    } finally {
        Set-Location $ProjectRoot
    }
}

# Determine executable path
$targetDir = if ($Target) { $Target } else { "" }
$profileDir = if ($Profile -eq "release") { "release" } else { "debug" }

if ($targetDir) {
    $ExecutablePath = Join-Path $RustProjectPath "target\$targetDir\$profileDir\multi-controller-app.exe"
} else {
    $ExecutablePath = Join-Path $RustProjectPath "target\$profileDir\multi-controller-app.exe"
}

if (-not (Test-Path $ExecutablePath)) {
    Write-Error "[PERF-RUST] Executable not found: $ExecutablePath"
    
    # Try to find the executable with different naming conventions
    $targetBaseDir = if ($targetDir) { Join-Path $RustProjectPath "target\$targetDir\$profileDir" } else { Join-Path $RustProjectPath "target\$profileDir" }
    if (Test-Path $targetBaseDir) {
        Write-Host "[PERF-RUST] Contents of target directory:" -ForegroundColor Gray
        Get-ChildItem $targetBaseDir -Name "*.exe" | ForEach-Object { Write-Host "  $_" -ForegroundColor Gray }
        
        # Try common alternative names
        $alternatives = @("multi_controller_app.exe", "multi-controller.exe", "app.exe")
        foreach ($alt in $alternatives) {
            $altPath = Join-Path $targetBaseDir $alt
            if (Test-Path $altPath) {
                Write-Host "[PERF-RUST] Found alternative executable: $altPath" -ForegroundColor Yellow
                $ExecutablePath = $altPath
                break
            }
        }
    }
    
    if (-not (Test-Path $ExecutablePath)) {
        exit 1
    }
}

# Get executable information
$exeInfo = Get-Item $ExecutablePath
$executableSizeMB = [math]::Round($exeInfo.Length / 1MB, 2)
Write-Host "[PERF-RUST] Executable: $ExecutablePath" -ForegroundColor Cyan
Write-Host "[PERF-RUST] Size: ${executableSizeMB}MB" -ForegroundColor Cyan
Write-Host "[PERF-RUST] Modified: $($exeInfo.LastWriteTime)" -ForegroundColor Cyan

# Verify Rust build characteristics
if ($ShowDetails) {
    Write-Host "[PERF-RUST] Analyzing Rust executable characteristics..." -ForegroundColor Gray
    
    # Check if it's a release build (smaller size typically indicates release)
    if ($Profile -eq "release") {
        Write-Host "[PERF-RUST] Release build optimizations: ENABLED" -ForegroundColor Green
        
        # Check for debug symbols (release builds should be stripped)
        try {
            $fileOutput = & file $ExecutablePath 2>$null
            if ($fileOutput -and $fileOutput -notmatch "not stripped") {
                Write-Host "[PERF-RUST] Debug symbols: STRIPPED (good for performance)" -ForegroundColor Green
            } else {
                Write-Host "[PERF-RUST] Debug symbols: PRESENT (may impact size)" -ForegroundColor Yellow
            }
        } catch {
            Write-Host "[PERF-RUST] Could not analyze debug symbols (file command not available)" -ForegroundColor Gray
        }
    } else {
        Write-Host "[PERF-RUST] Development build: Debug symbols present" -ForegroundColor Gray
    }
    
    # Display enabled features if any
    if ($Features) {
        Write-Host "[PERF-RUST] Enabled features: $Features" -ForegroundColor Gray
    }
    
    # Check target directory contents
    $targetBaseDir = Split-Path $ExecutablePath -Parent
    $totalSize = (Get-ChildItem $targetBaseDir -File | Measure-Object -Property Length -Sum).Sum / 1MB
    Write-Host "[PERF-RUST] Target directory size: $([math]::Round($totalSize, 2))MB" -ForegroundColor Gray
}

# Prepare output path with Rust specific naming
$timestamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$outputPath = Join-Path $PSScriptRoot "rust-benchmark-$timestamp.json"

# Run the main benchmark
$benchmarkScript = Join-Path $PSScriptRoot "benchmark.ps1"
$benchmarkArgs = @(
    "-ExecutablePath", $ExecutablePath
    "-BenchmarkType", "rust"
    "-OutputPath", $outputPath
    "-MonitorDuration", $MonitorDuration
)

if ($ShowDetails) {
    $benchmarkArgs += "-ShowDetails"
}

Write-Host "[PERF-RUST] Starting performance measurement..." -ForegroundColor Yellow
Write-Host ""

& $benchmarkScript @benchmarkArgs
$benchmarkExitCode = $LASTEXITCODE

# Add Rust specific metadata to report if benchmark succeeded
if ($benchmarkExitCode -eq 0 -and (Test-Path $outputPath)) {
    try {
        $report = Get-Content $outputPath | ConvertFrom-Json
        
        # Add Rust specific metadata
        $report.metadata | Add-Member -Name "buildProfile" -Value $Profile -MemberType NoteProperty
        $report.metadata | Add-Member -Name "executableSizeMB" -Value $executableSizeMB -MemberType NoteProperty
        $report.metadata | Add-Member -Name "cargoVersion" -Value ((& cargo --version) -join ' ') -MemberType NoteProperty
        $report.metadata | Add-Member -Name "rustcVersion" -Value ((& rustc --version) -join ' ') -MemberType NoteProperty
        
        if ($Target) {
            $report.metadata | Add-Member -Name "targetTriple" -Value $Target -MemberType NoteProperty
        }
        
        if ($Features) {
            $report.metadata | Add-Member -Name "enabledFeatures" -Value $Features -MemberType NoteProperty
        }
        
        if (-not $SkipBuild) {
            $report.metadata | Add-Member -Name "buildTimeSeconds" -Value ([math]::Round($buildTime, 2)) -MemberType NoteProperty
        }
        
        # Save updated report
        $report | ConvertTo-Json -Depth 10 | Set-Content $outputPath -Encoding UTF8
        Write-Host "[PERF-RUST] Enhanced Rust report saved to: $outputPath" -ForegroundColor Green
    } catch {
        Write-Warning "[PERF-RUST] Failed to enhance report with Rust metadata: $($_.Exception.Message)"
    }
}

Write-Host ""
if ($benchmarkExitCode -eq 0) {
    Write-Host "[PERF-RUST] Rust benchmark completed successfully" -ForegroundColor Green
} else {
    Write-Host "[PERF-RUST] Rust benchmark failed" -ForegroundColor Red
}

exit $benchmarkExitCode