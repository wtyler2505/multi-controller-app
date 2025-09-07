#Requires -Version 5.1
<#
.SYNOPSIS
    C# Native AOT specific performance benchmark helper
    
.DESCRIPTION
    Builds and benchmarks the C# Native AOT Multi-Controller App prototype.
    Handles build configuration, executable location, and C#-specific measurements.
    
.PARAMETER Configuration
    Build configuration: 'Release' or 'Debug' (default: Release)
    
.PARAMETER Runtime
    Target runtime identifier: 'win-x64', 'win-x86', 'win-arm64' (default: win-x64)
    
.PARAMETER ShowDetails
    Show detailed build and benchmark information
    
.PARAMETER SkipBuild
    Skip the build step and benchmark existing executable
    
.PARAMETER MonitorDuration
    Duration in seconds to monitor CPU/memory (default: 60)
    
.EXAMPLE
    .\benchmark-csharp.ps1 -ShowDetails
    
.EXAMPLE
    .\benchmark-csharp.ps1 -Configuration Release -Runtime win-x64 -MonitorDuration 120
    
.EXAMPLE
    .\benchmark-csharp.ps1 -SkipBuild -ShowDetails
#>

[CmdletBinding()]
param(
    [ValidateSet('Release', 'Debug')]
    [string]$Configuration = 'Release',
    
    [ValidateSet('win-x64', 'win-x86', 'win-arm64')]
    [string]$Runtime = 'win-x64',
    
    [switch]$ShowDetails,
    
    [switch]$SkipBuild,
    
    [int]$MonitorDuration = 60
)

# Project paths
$ProjectRoot = Resolve-Path "$PSScriptRoot\..\.."
$CSharpProjectPath = Join-Path $ProjectRoot "apps\desktop"
$ProjectFile = Join-Path $CSharpProjectPath "MultiControllerApp.csproj"

Write-Host "[PERF-CSHARP] C# Native AOT Performance Benchmark" -ForegroundColor Cyan
Write-Host "[PERF-CSHARP] Configuration: $Configuration, Runtime: $Runtime" -ForegroundColor Cyan

# Validate project file exists
if (-not (Test-Path $ProjectFile)) {
    Write-Error "[PERF-CSHARP] Project file not found: $ProjectFile"
    exit 1
}

# Build the project (unless skipped)
if (-not $SkipBuild) {
    Write-Host "[PERF-CSHARP] Building C# Native AOT project..." -ForegroundColor Yellow
    
    $buildStart = Get-Date
    
    # Clean first
    $cleanArgs = @(
        "clean"
        $ProjectFile
        "--configuration", $Configuration
        "--runtime", $Runtime
        "--verbosity", "minimal"
    )
    
    if ($ShowDetails) {
        Write-Host "[PERF-CSHARP] Running: dotnet $($cleanArgs -join ' ')" -ForegroundColor Gray
    }
    
    & dotnet @cleanArgs
    if ($LASTEXITCODE -ne 0) {
        Write-Error "[PERF-CSHARP] Clean failed with exit code $LASTEXITCODE"
        exit 1
    }
    
    # Publish with Native AOT
    $publishArgs = @(
        "publish"
        $ProjectFile
        "--configuration", $Configuration
        "--runtime", $Runtime
        "--self-contained", "true"
        "--verbosity", "minimal"
        "/p:PublishAot=true"
        "/p:PublishTrimmed=true"
        "/p:PublishSingleFile=true"
        "/p:StripSymbols=true"
        "/p:OptimizationPreference=Speed"
    )
    
    if ($ShowDetails) {
        Write-Host "[PERF-CSHARP] Running: dotnet $($publishArgs -join ' ')" -ForegroundColor Gray
        $publishArgs += "--verbosity", "normal"
    }
    
    & dotnet @publishArgs
    if ($LASTEXITCODE -ne 0) {
        Write-Error "[PERF-CSHARP] Publish failed with exit code $LASTEXITCODE"
        exit 1
    }
    
    $buildEnd = Get-Date
    $buildTime = ($buildEnd - $buildStart).TotalSeconds
    Write-Host "[PERF-CSHARP] Build completed in $([math]::Round($buildTime, 1)) seconds" -ForegroundColor Green
}

# Determine executable path
$PublishDir = Join-Path $CSharpProjectPath "bin\$Configuration\net8.0-windows\$Runtime\publish"
$ExecutablePath = Join-Path $PublishDir "MultiControllerApp.exe"

if (-not (Test-Path $ExecutablePath)) {
    Write-Error "[PERF-CSHARP] Executable not found: $ExecutablePath"
    Write-Host "[PERF-CSHARP] Expected publish directory: $PublishDir" -ForegroundColor Gray
    if (Test-Path $PublishDir) {
        Write-Host "[PERF-CSHARP] Contents of publish directory:" -ForegroundColor Gray
        Get-ChildItem $PublishDir | ForEach-Object { Write-Host "  $($_.Name)" -ForegroundColor Gray }
    }
    exit 1
}

# Get executable information
$exeInfo = Get-Item $ExecutablePath
$executableSizeMB = [math]::Round($exeInfo.Length / 1MB, 2)
Write-Host "[PERF-CSHARP] Executable: $ExecutablePath" -ForegroundColor Cyan
Write-Host "[PERF-CSHARP] Size: ${executableSizeMB}MB" -ForegroundColor Cyan
Write-Host "[PERF-CSHARP] Modified: $($exeInfo.LastWriteTime)" -ForegroundColor Cyan

# Verify Native AOT compilation
if ($ShowDetails) {
    Write-Host "[PERF-CSHARP] Analyzing executable characteristics..." -ForegroundColor Gray
    
    # Check for .NET runtime dependencies (Native AOT should have minimal deps)
    $publishFiles = Get-ChildItem $PublishDir -File
    $netRuntimeFiles = $publishFiles | Where-Object { $_.Name -like "*.dll" -and $_.Name -like "*runtime*" }
    
    if ($netRuntimeFiles.Count -eq 0) {
        Write-Host "[PERF-CSHARP] Native AOT verification: PASSED (no runtime DLLs found)" -ForegroundColor Green
    } else {
        Write-Host "[PERF-CSHARP] Native AOT verification: WARNING (found $($netRuntimeFiles.Count) runtime DLLs)" -ForegroundColor Yellow
        $netRuntimeFiles | ForEach-Object { Write-Host "  $($_.Name)" -ForegroundColor Gray }
    }
    
    # Check total publish directory size
    $totalSize = ($publishFiles | Measure-Object -Property Length -Sum).Sum / 1MB
    Write-Host "[PERF-CSHARP] Total publish size: $([math]::Round($totalSize, 2))MB" -ForegroundColor Gray
    
    # Check if single file publish worked
    if ($publishFiles.Count -eq 1) {
        Write-Host "[PERF-CSHARP] Single file publish: PASSED" -ForegroundColor Green
    } else {
        Write-Host "[PERF-CSHARP] Single file publish: WARNING ($($publishFiles.Count) files)" -ForegroundColor Yellow
    }
}

# Prepare output path with C# specific naming
$timestamp = Get-Date -Format 'yyyyMMdd-HHmmss'
$outputPath = Join-Path $PSScriptRoot "csharp-benchmark-$timestamp.json"

# Run the main benchmark
$benchmarkScript = Join-Path $PSScriptRoot "benchmark.ps1"
$benchmarkArgs = @(
    "-ExecutablePath", $ExecutablePath
    "-BenchmarkType", "csharp"
    "-OutputPath", $outputPath
    "-MonitorDuration", $MonitorDuration
)

if ($ShowDetails) {
    $benchmarkArgs += "-ShowDetails"
}

Write-Host "[PERF-CSHARP] Starting performance measurement..." -ForegroundColor Yellow
Write-Host ""

& $benchmarkScript @benchmarkArgs
$benchmarkExitCode = $LASTEXITCODE

# Add C# specific metadata to report if benchmark succeeded
if ($benchmarkExitCode -eq 0 -and (Test-Path $outputPath)) {
    try {
        $report = Get-Content $outputPath | ConvertFrom-Json
        
        # Add C# specific metadata
        $report.metadata | Add-Member -Name "buildConfiguration" -Value $Configuration -MemberType NoteProperty
        $report.metadata | Add-Member -Name "targetRuntime" -Value $Runtime -MemberType NoteProperty
        $report.metadata | Add-Member -Name "executableSizeMB" -Value $executableSizeMB -MemberType NoteProperty
        $report.metadata | Add-Member -Name "publishDirectory" -Value $PublishDir -MemberType NoteProperty
        $report.metadata | Add-Member -Name "nativeAot" -Value $true -MemberType NoteProperty
        $report.metadata | Add-Member -Name "singleFile" -Value ($publishFiles.Count -eq 1) -MemberType NoteProperty
        
        if (-not $SkipBuild) {
            $report.metadata | Add-Member -Name "buildTimeSeconds" -Value ([math]::Round($buildTime, 2)) -MemberType NoteProperty
        }
        
        # Save updated report
        $report | ConvertTo-Json -Depth 10 | Set-Content $outputPath -Encoding UTF8
        Write-Host "[PERF-CSHARP] Enhanced C# report saved to: $outputPath" -ForegroundColor Green
    } catch {
        Write-Warning "[PERF-CSHARP] Failed to enhance report with C# metadata: $($_.Exception.Message)"
    }
}

Write-Host ""
if ($benchmarkExitCode -eq 0) {
    Write-Host "[PERF-CSHARP] C# Native AOT benchmark completed successfully" -ForegroundColor Green
} else {
    Write-Host "[PERF-CSHARP] C# Native AOT benchmark failed" -ForegroundColor Red
}

exit $benchmarkExitCode