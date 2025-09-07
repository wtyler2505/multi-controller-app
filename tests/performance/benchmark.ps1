#Requires -Version 5.1
<#
.SYNOPSIS
    Performance benchmark framework for Multi-Controller App prototypes
    
.DESCRIPTION
    Measures startup time, CPU usage, and memory consumption for both C# Native AOT and Rust prototypes.
    Compares results against performance budgets and outputs structured JSON reports.
    
.PARAMETER ExecutablePath
    Path to the executable to benchmark
    
.PARAMETER BenchmarkType
    Type of benchmark: 'csharp' or 'rust'
    
.PARAMETER OutputPath
    Path to save the JSON benchmark report
    
.PARAMETER ShowDetails
    Show detailed progress information
    
.PARAMETER MonitorDuration
    Duration in seconds to monitor CPU/memory (default: 60)
    
.EXAMPLE
    .\benchmark.ps1 -ExecutablePath "..\..\apps\desktop\bin\Release\net8.0-windows\publish\MultiControllerApp.exe" -BenchmarkType "csharp"
    
.EXAMPLE
    .\benchmark.ps1 -ExecutablePath "target\release\multi_controller_app.exe" -BenchmarkType "rust" -ShowDetails
#>

[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string]$ExecutablePath,
    
    [Parameter(Mandatory = $true)]
    [ValidateSet('csharp', 'rust', 'csharp-prototype', 'rust-prototype')]
    [string]$BenchmarkType,
    
    [string]$OutputPath = "benchmark-report-$(Get-Date -Format 'yyyyMMdd-HHmmss').json",
    
    [switch]$ShowDetails,
    
    [int]$MonitorDuration = 60
)

# Performance budgets
$BUDGETS = @{
    StartupTimeMs = 2000      # < 2 seconds
    IdleCpuPercent = 2.0      # <= 2%
    BaseMemoryMB = 150        # <= 150 MB
    MaxMemoryWithChartsMB = 220  # <= 220 MB with charts
}

# Helper function for console output
function Write-Progress-Info {
    param([string]$Message)
    if ($ShowDetails) {
        Write-Host "[PERF] $Message" -ForegroundColor Cyan
    }
}

# Helper function for error output
function Write-Error-Info {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

# Helper function for success output
function Write-Success-Info {
    param([string]$Message)
    Write-Host "[OK] $Message" -ForegroundColor Green
}

# Helper function for warning output
function Write-Warning-Info {
    param([string]$Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

# Validate executable exists
if (-not (Test-Path $ExecutablePath)) {
    Write-Error-Info "Executable not found: $ExecutablePath"
    exit 1
}

Write-Progress-Info "Starting performance benchmark for $BenchmarkType prototype"
Write-Progress-Info "Executable: $ExecutablePath"
Write-Progress-Info "Monitor duration: $MonitorDuration seconds"

# Initialize performance counters
try {
    $cpuCounter = New-Object System.Diagnostics.PerformanceCounter("Processor", "% Processor Time", "_Total")
    $cpuCounter.NextValue() | Out-Null  # First call always returns 0
} catch {
    Write-Error-Info "Failed to initialize CPU performance counter: $($_.Exception.Message)"
    exit 1
}

# Start the benchmark
$benchmarkStart = Get-Date
$startupTimer = [System.Diagnostics.Stopwatch]::StartNew()

Write-Progress-Info "Launching executable for cold start measurement..."

# Launch the process
$processStartInfo = New-Object System.Diagnostics.ProcessStartInfo
$processStartInfo.FileName = $ExecutablePath
$processStartInfo.UseShellExecute = $false
$processStartInfo.RedirectStandardOutput = $true
$processStartInfo.RedirectStandardError = $true
$processStartInfo.CreateNoWindow = $true

try {
    $process = [System.Diagnostics.Process]::Start($processStartInfo)
    Write-Progress-Info "Process started with PID: $($process.Id)"
} catch {
    Write-Error-Info "Failed to start process: $($_.Exception.Message)"
    exit 1
}

# Wait for process to be ready (looking for window or specific output)
$maxWaitTime = 10000  # 10 seconds max
$checkInterval = 100  # Check every 100ms
$waitedTime = 0
$processReady = $false

while (-not $processReady -and $waitedTime -lt $maxWaitTime -and -not $process.HasExited) {
    Start-Sleep -Milliseconds $checkInterval
    $waitedTime += $checkInterval
    
    # For Windows apps, check if main window handle is available
    if ($process.MainWindowHandle -ne [IntPtr]::Zero) {
        $processReady = $true
        Write-Progress-Info "Process window detected, startup complete"
    }
    # Fallback: assume ready after 1 second if no window (console app)
    elseif ($waitedTime -ge 1000) {
        $processReady = $true
        Write-Progress-Info "Process assumed ready (no window detected)"
    }
}

$startupTimer.Stop()
$startupTime = $startupTimer.ElapsedMilliseconds

if ($process.HasExited) {
    Write-Error-Info "Process exited during startup measurement"
    $exitCode = $process.ExitCode
    $stdout = $process.StandardOutput.ReadToEnd()
    $stderr = $process.StandardError.ReadToEnd()
    Write-Error-Info "Exit code: $exitCode"
    if ($stdout) { Write-Error-Info "STDOUT: $stdout" }
    if ($stderr) { Write-Error-Info "STDERR: $stderr" }
    exit 1
}

Write-Progress-Info "Startup time measured: ${startupTime}ms"

# Initialize monitoring arrays
$cpuSamples = @()
$memorySamples = @()
$timestamps = @()

Write-Progress-Info "Starting $MonitorDuration second monitoring period..."

# Monitor CPU and memory for specified duration
$monitorStart = Get-Date
$sampleInterval = 1  # Sample every 1 second

for ($i = 0; $i -lt $MonitorDuration; $i++) {
    try {
        # Sample current state
        $currentTime = Get-Date
        $timestamps += $currentTime
        
        # Get CPU usage (system-wide)
        $systemCpuUsage = $cpuCounter.NextValue()
        
        # Get process-specific metrics
        $process.Refresh()
        if ($process.HasExited) {
            Write-Warning-Info "Process exited during monitoring at $i seconds"
            break
        }
        
        $processMemoryMB = [math]::Round($process.WorkingSet64 / 1MB, 2)
        
        # Get process CPU usage
        $processCpuCounter = $null
        try {
            $processCpuCounter = New-Object System.Diagnostics.PerformanceCounter("Process", "% Processor Time", $process.ProcessName)
            $processCpuUsage = $processCpuCounter.NextValue()
        } catch {
            $processCpuUsage = 0  # Fallback if process counter fails
        } finally {
            if ($processCpuCounter) { $processCpuCounter.Dispose() }
        }
        
        $cpuSamples += $processCpuUsage
        $memorySamples += $processMemoryMB
        
        if ($ShowDetails -and ($i % 10 -eq 0 -or $i -eq ($MonitorDuration - 1))) {
            Write-Progress-Info "Sample $($i + 1)/$MonitorDuration - CPU: $([math]::Round($processCpuUsage, 1))%, Memory: ${processMemoryMB}MB"
        }
        
        # Check for budget violations during monitoring
        if ($processMemoryMB -gt $BUDGETS.BaseMemoryMB) {
            Write-Warning-Info "Memory budget exceeded: ${processMemoryMB}MB > $($BUDGETS.BaseMemoryMB)MB"
        }
        
        Start-Sleep -Seconds $sampleInterval
    } catch {
        Write-Warning-Info "Error during monitoring at sample $($i + 1): $($_.Exception.Message)"
    }
}

# Clean up process
try {
    if (-not $process.HasExited) {
        Write-Progress-Info "Terminating process..."
        $process.Kill()
        $process.WaitForExit(5000)  # Wait up to 5 seconds
    }
} catch {
    Write-Warning-Info "Error terminating process: $($_.Exception.Message)"
} finally {
    $process.Dispose()
}

# Calculate metrics
$benchmarkEnd = Get-Date
$totalBenchmarkTime = ($benchmarkEnd - $benchmarkStart).TotalSeconds

if ($cpuSamples.Count -eq 0 -or $memorySamples.Count -eq 0) {
    Write-Error-Info "No samples collected during monitoring"
    exit 1
}

$avgCpu = [math]::Round(($cpuSamples | Measure-Object -Average).Average, 2)
$maxCpu = [math]::Round(($cpuSamples | Measure-Object -Maximum).Maximum, 2)
$minMemory = [math]::Round(($memorySamples | Measure-Object -Minimum).Minimum, 2)
$maxMemory = [math]::Round(($memorySamples | Measure-Object -Maximum).Maximum, 2)
$avgMemory = [math]::Round(($memorySamples | Measure-Object -Average).Average, 2)

# Performance gate checks
$startupPassed = $startupTime -le $BUDGETS.StartupTimeMs
$cpuPassed = $avgCpu -le $BUDGETS.IdleCpuPercent
$memoryPassed = $maxMemory -le $BUDGETS.BaseMemoryMB

Write-Progress-Info "Performance analysis complete"

# Output results
Write-Host ""
Write-Host "============== PERFORMANCE BENCHMARK RESULTS ==============" -ForegroundColor White
Write-Host "Benchmark Type: $BenchmarkType" -ForegroundColor White
Write-Host "Executable: $ExecutablePath" -ForegroundColor White
Write-Host "Timestamp: $benchmarkStart" -ForegroundColor White
Write-Host ""

# Startup time results
if ($startupPassed) {
    Write-Success-Info "Startup Time: ${startupTime}ms (PASSED - Budget: $($BUDGETS.StartupTimeMs)ms)"
} else {
    Write-Warning-Info "Startup Time: ${startupTime}ms (FAILED - Budget: $($BUDGETS.StartupTimeMs)ms)"
}

# CPU usage results  
if ($cpuPassed) {
    Write-Success-Info "Average CPU: ${avgCpu}% (PASSED - Budget: $($BUDGETS.IdleCpuPercent)%)"
} else {
    Write-Warning-Info "Average CPU: ${avgCpu}% (FAILED - Budget: $($BUDGETS.IdleCpuPercent)%)"
}

Write-Host "Maximum CPU: ${maxCpu}%" -ForegroundColor Gray

# Memory usage results
if ($memoryPassed) {
    Write-Success-Info "Peak Memory: ${maxMemory}MB (PASSED - Budget: $($BUDGETS.BaseMemoryMB)MB)"
} else {
    Write-Warning-Info "Peak Memory: ${maxMemory}MB (FAILED - Budget: $($BUDGETS.BaseMemoryMB)MB)"
}

Write-Host "Average Memory: ${avgMemory}MB" -ForegroundColor Gray
Write-Host "Minimum Memory: ${minMemory}MB" -ForegroundColor Gray

# Overall result
$overallPassed = $startupPassed -and $cpuPassed -and $memoryPassed
if ($overallPassed) {
    Write-Success-Info "OVERALL RESULT: PASSED - All performance budgets met"
} else {
    Write-Warning-Info "OVERALL RESULT: FAILED - Some performance budgets exceeded"
}

Write-Host "Monitor Duration: $MonitorDuration seconds ($($cpuSamples.Count) samples)" -ForegroundColor Gray
Write-Host "Total Benchmark Time: $([math]::Round($totalBenchmarkTime, 1)) seconds" -ForegroundColor Gray
Write-Host "=========================================================" -ForegroundColor White

# Create detailed JSON report
$report = @{
    metadata = @{
        benchmarkType = $BenchmarkType
        executablePath = $ExecutablePath
        timestamp = $benchmarkStart.ToString("yyyy-MM-ddTHH:mm:ss.fffZ")
        monitorDuration = $MonitorDuration
        sampleCount = $cpuSamples.Count
        totalBenchmarkTime = [math]::Round($totalBenchmarkTime, 2)
        powershellVersion = $PSVersionTable.PSVersion.ToString()
        osVersion = [System.Environment]::OSVersion.ToString()
    }
    budgets = $BUDGETS
    results = @{
        startup = @{
            timeMs = $startupTime
            passed = $startupPassed
            budgetMs = $BUDGETS.StartupTimeMs
        }
        cpu = @{
            averagePercent = $avgCpu
            maximumPercent = $maxCpu
            passed = $cpuPassed
            budgetPercent = $BUDGETS.IdleCpuPercent
            samples = $cpuSamples
        }
        memory = @{
            minimumMB = $minMemory
            averageMB = $avgMemory
            maximumMB = $maxMemory
            passed = $memoryPassed
            budgetMB = $BUDGETS.BaseMemoryMB
            samples = $memorySamples
        }
    }
    performance_gate = @{
        passed = $overallPassed
        failures = @()
    }
}

# Add failure details
if (-not $startupPassed) {
    $report.performance_gate.failures += "Startup time ${startupTime}ms exceeds budget $($BUDGETS.StartupTimeMs)ms"
}
if (-not $cpuPassed) {
    $report.performance_gate.failures += "Average CPU ${avgCpu}% exceeds budget $($BUDGETS.IdleCpuPercent)%"
}
if (-not $memoryPassed) {
    $report.performance_gate.failures += "Peak memory ${maxMemory}MB exceeds budget $($BUDGETS.BaseMemoryMB)MB"
}

# Save JSON report
try {
    $jsonReport = $report | ConvertTo-Json -Depth 10 -Compress:$false
    $jsonReport | Out-File -FilePath $OutputPath -Encoding UTF8
    Write-Progress-Info "Benchmark report saved to: $OutputPath"
} catch {
    Write-Error-Info "Failed to save benchmark report: $($_.Exception.Message)"
    exit 1
}

Write-Host ""

# Exit with appropriate code
if ($overallPassed) {
    Write-Progress-Info "Benchmark completed successfully"
    exit 0
} else {
    Write-Error-Info "Benchmark failed - Performance budgets exceeded"
    exit 1
}