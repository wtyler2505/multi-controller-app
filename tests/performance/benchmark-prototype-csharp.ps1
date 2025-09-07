#Requires -Version 5.1
<#
.SYNOPSIS
    C# Native AOT prototype specific performance benchmark
    
.DESCRIPTION
    Benchmarks the existing C# Native AOT Multi-Controller App prototype.
    Uses the pre-built executable from the prototype directory.
    
.PARAMETER ShowDetails
    Show detailed benchmark information
    
.PARAMETER MonitorDuration
    Duration in seconds to monitor CPU/memory (default: 60)
    
.EXAMPLE
    .\benchmark-prototype-csharp.ps1 -ShowDetails
    
.EXAMPLE
    .\benchmark-prototype-csharp.ps1 -MonitorDuration 120
#>

[CmdletBinding()]
param(
    [switch]$ShowDetails,
    
    [int]$MonitorDuration = 60
)

# Prototype paths - these are the actual prototype locations
$PrototypePath = "C:\Users\wtyle\multi-controller-csharp\prototypes\csharp-winui"
$PublishDir = Join-Path $PrototypePath "bin\Release\net8.0-windows10.0.19041.0\win-x64\publish"
$ExecutablePath = Join-Path $PublishDir "MultiControllerPrototype.exe"

Write-Host "[PERF-CSHARP-PROTO] C# Native AOT Prototype Performance Benchmark" -ForegroundColor Cyan
Write-Host "[PERF-CSHARP-PROTO] Prototype Location: $PrototypePath" -ForegroundColor Cyan

# Validate executable exists
if (-not (Test-Path $ExecutablePath)) {
    Write-Error "[PERF-CSHARP-PROTO] Prototype executable not found: $ExecutablePath"
    Write-Host "[PERF-CSHARP-PROTO] Please ensure the C# prototype has been built" -ForegroundColor Red
    exit 1
}

# Get executable information
$exeInfo = Get-Item $ExecutablePath
$executableSizeMB = [math]::Round($exeInfo.Length / 1MB, 2)
Write-Host "[PERF-CSHARP-PROTO] Executable: $ExecutablePath" -ForegroundColor Cyan
Write-Host "[PERF-CSHARP-PROTO] Size: ${executableSizeMB}MB" -ForegroundColor Cyan
Write-Host "[PERF-CSHARP-PROTO] Modified: $($exeInfo.LastWriteTime)" -ForegroundColor Cyan

# Get publish directory information
if ($ShowDetails) {
    Write-Host "[PERF-CSHARP-PROTO] Analyzing executable characteristics..." -ForegroundColor Gray
    
    $publishFiles = Get-ChildItem $PublishDir -File
    $totalSize = ($publishFiles | Measure-Object -Property Length -Sum).Sum / 1MB
    Write-Host "[PERF-CSHARP-PROTO] Total publish size: $([math]::Round($totalSize, 2))MB" -ForegroundColor Gray
    Write-Host "[PERF-CSHARP-PROTO] File count: $($publishFiles.Count)" -ForegroundColor Gray
    
    # Check for Native AOT characteristics
    $netRuntimeFiles = $publishFiles | Where-Object { $_.Name -like "*.dll" -and $_.Name -like "*runtime*" }
    if ($netRuntimeFiles.Count -eq 0) {
        Write-Host "[PERF-CSHARP-PROTO] Native AOT verification: PASSED (no runtime DLLs)" -ForegroundColor Green
    } else {
        Write-Host "[PERF-CSHARP-PROTO] Native AOT verification: WARNING ($($netRuntimeFiles.Count) runtime DLLs)" -ForegroundColor Yellow
    }
    
    # Single file check
    if ($publishFiles.Count -eq 1) {
        Write-Host "[PERF-CSHARP-PROTO] Single file publish: PASSED" -ForegroundColor Green
    } else {
        Write-Host "[PERF-CSHARP-PROTO] Single file publish: MULTIPLE FILES ($($publishFiles.Count))" -ForegroundColor Yellow
        if ($ShowDetails) {
            Write-Host "[PERF-CSHARP-PROTO] Files in publish directory:" -ForegroundColor Gray
            $publishFiles | Sort-Object Length -Descending | ForEach-Object {
                $sizeMB = [math]::Round($_.Length / 1MB, 2)
                Write-Host "  $($_.Name) - ${sizeMB}MB" -ForegroundColor Gray
            }
        }
    }
}

# Prepare output path
$timestamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$outputPath = Join-Path $PSScriptRoot "csharp-prototype-benchmark-$timestamp.json"

# Run the main benchmark
$benchmarkScript = Join-Path $PSScriptRoot "benchmark.ps1"
$benchmarkArgs = @(
    "-ExecutablePath", $ExecutablePath
    "-BenchmarkType", "csharp-prototype"
    "-OutputPath", $outputPath
    "-MonitorDuration", $MonitorDuration
)

if ($ShowDetails) {
    $benchmarkArgs += "-ShowDetails"
}

Write-Host "[PERF-CSHARP-PROTO] Starting performance measurement..." -ForegroundColor Yellow
Write-Host ""

# Build arguments properly
$args = @(
    "-ExecutablePath", $ExecutablePath
    "-BenchmarkType", "csharp-prototype"
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
        $report.metadata | Add-Member -Name "prototypeType" -Value "C# Native AOT WinUI" -MemberType NoteProperty
        $report.metadata | Add-Member -Name "prototypeLocation" -Value $PrototypePath -MemberType NoteProperty
        $report.metadata | Add-Member -Name "executableSizeMB" -Value $executableSizeMB -MemberType NoteProperty
        $report.metadata | Add-Member -Name "publishDirectory" -Value $PublishDir -MemberType NoteProperty
        $report.metadata | Add-Member -Name "totalPublishSizeMB" -Value ([math]::Round(($publishFiles | Measure-Object -Property Length -Sum).Sum / 1MB, 2)) -MemberType NoteProperty
        $report.metadata | Add-Member -Name "fileCount" -Value $publishFiles.Count -MemberType NoteProperty
        $report.metadata | Add-Member -Name "nativeAot" -Value $true -MemberType NoteProperty
        $report.metadata | Add-Member -Name "singleFile" -Value ($publishFiles.Count -eq 1) -MemberType NoteProperty
        
        # Save updated report
        $report | ConvertTo-Json -Depth 10 | Set-Content $outputPath -Encoding UTF8
        Write-Host "[PERF-CSHARP-PROTO] Enhanced C# prototype report saved to: $outputPath" -ForegroundColor Green
    } catch {
        Write-Warning "[PERF-CSHARP-PROTO] Failed to enhance report with prototype metadata: $($_.Exception.Message)"
    }
}

Write-Host ""
if ($benchmarkExitCode -eq 0) {
    Write-Host "[PERF-CSHARP-PROTO] C# Native AOT prototype benchmark completed successfully" -ForegroundColor Green
    Write-Host "[PERF-CSHARP-PROTO] Report saved: $(Split-Path $outputPath -Leaf)" -ForegroundColor Cyan
} else {
    Write-Host "[PERF-CSHARP-PROTO] C# Native AOT prototype benchmark failed" -ForegroundColor Red
}

exit $benchmarkExitCode