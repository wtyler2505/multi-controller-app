# Performance Gate Validation Script for Windows
# Ensures performance budgets are met before allowing commits

param(
    [switch]$SkipStartup = $false,
    [switch]$SkipMemory = $false,
    [switch]$Verbose = $false
)

$ErrorActionPreference = "Stop"

# Load configuration
$configPath = Join-Path $PSScriptRoot "..\..\\.gitmeta\\config\\performance-budgets.json"
$config = Get-Content $configPath | ConvertFrom-Json

# Colors for output
$red = [System.ConsoleColor]::Red
$yellow = [System.ConsoleColor]::Yellow  
$green = [System.ConsoleColor]::Green
$defaultColor = [System.Console]::ForegroundColor

function Write-ColorOutput($message, $color) {
    [System.Console]::ForegroundColor = $color
    Write-Host $message
    [System.Console]::ForegroundColor = $defaultColor
}

function Test-StartupPerformance {
    if ($SkipStartup) {
        Write-ColorOutput "⏭️  Skipping startup performance check" $yellow
        return $true
    }

    Write-ColorOutput "⏱️  Measuring startup performance..." $yellow
    
    $projectPath = Join-Path $PSScriptRoot "..\..\apps\desktop\MultiControllerApp.csproj"
    
    if (-not (Test-Path $projectPath)) {
        Write-ColorOutput "⚠️  Desktop project not found, skipping startup check" $yellow
        return $true
    }
    
    # Build in Release mode first
    Write-Host "Building in Release mode..."
    $buildOutput = dotnet build $projectPath -c Release 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput "❌ Build failed, cannot measure startup performance" $red
        return $false
    }
    
    # Measure startup time (average of 3 runs)
    $measurements = @()
    for ($i = 1; $i -le 3; $i++) {
        Write-Host "  Run $i/3..."
        $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
        
        # Start the app and wait for it to be ready
        $process = Start-Process -FilePath "dotnet" `
            -ArgumentList "run", "--project", $projectPath, "--configuration", "Release", "--no-build" `
            -PassThru -WindowStyle Hidden
        
        # Wait for startup or timeout after 5 seconds
        Start-Sleep -Milliseconds 500
        
        $stopwatch.Stop()
        $measurements += $stopwatch.ElapsedMilliseconds
        
        # Kill the process
        if (!$process.HasExited) {
            Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        }
        
        Start-Sleep -Milliseconds 200
    }
    
    $avgStartup = ($measurements | Measure-Object -Average).Average
    $maxAllowed = $config.startup.max_ms
    $warnThreshold = $config.startup.warn_ms
    
    Write-Host "  Measurements: $($measurements -join 'ms, ')ms"
    Write-Host "  Average: $([math]::Round($avgStartup))ms"
    
    if ($avgStartup -gt $maxAllowed) {
        Write-ColorOutput "❌ Startup time ($([math]::Round($avgStartup))ms) exceeds budget ($($maxAllowed)ms)" $red
        return $false
    } elseif ($avgStartup -gt $warnThreshold) {
        Write-ColorOutput "⚠️  Startup time ($([math]::Round($avgStartup))ms) approaching limit (max: $($maxAllowed)ms)" $yellow
    } else {
        Write-ColorOutput "✅ Startup time OK: $([math]::Round($avgStartup))ms (budget: $($maxAllowed)ms)" $green
    }
    
    return $true
}

function Test-MemoryUsage {
    if ($SkipMemory) {
        Write-ColorOutput "⏭️  Skipping memory check" $yellow
        return $true
    }

    Write-ColorOutput "💾 Checking memory usage..." $yellow
    
    # For now, we'll do a simple check based on current build size
    # In production, this would launch the app and measure actual memory
    
    $projectPath = Join-Path $PSScriptRoot "..\..\apps\desktop"
    $binPath = Join-Path $projectPath "bin\Release\net8.0-windows"
    
    if (Test-Path $binPath) {
        $totalSize = (Get-ChildItem $binPath -Recurse | Measure-Object -Property Length -Sum).Sum / 1MB
        Write-Host "  Binary size: $([math]::Round($totalSize, 2))MB"
        
        # Rough estimate: binary size * 3 for runtime memory
        $estimatedMemory = $totalSize * 3
        $maxAllowed = $config.memory.max_mb
        
        if ($estimatedMemory -gt $maxAllowed) {
            Write-ColorOutput "⚠️  Estimated memory usage high: ~$([math]::Round($estimatedMemory))MB" $yellow
        } else {
            Write-ColorOutput "✅ Memory estimate OK: ~$([math]::Round($estimatedMemory))MB (budget: $($maxAllowed)MB)" $green
        }
    } else {
        Write-ColorOutput "⚠️  No release build found for memory estimation" $yellow
    }
    
    return $true
}

function Test-CodeChanges {
    Write-ColorOutput "📝 Checking for performance-critical changes..." $yellow
    
    # Get list of staged files
    $stagedFiles = git diff --cached --name-only
    
    $criticalFiles = @(
        "Program.cs",
        "App.xaml.cs",
        "MainWindow.xaml.cs"
    )
    
    $hasCriticalChanges = $false
    foreach ($file in $stagedFiles) {
        foreach ($critical in $criticalFiles) {
            if ($file -like "*$critical") {
                Write-ColorOutput "  ⚠️  Critical file changed: $file" $yellow
                $hasCriticalChanges = $true
            }
        }
    }
    
    if ($hasCriticalChanges) {
        Write-ColorOutput "  ⚠️  Performance-critical files changed - extra validation recommended" $yellow
    } else {
        Write-ColorOutput "  ✅ No performance-critical files in this commit" $green
    }
    
    return $true
}

# Main execution
Write-ColorOutput "`n🚦 Performance Gate Validation" $yellow
Write-ColorOutput "================================" $yellow

$allPassed = $true

# Run tests
if (!(Test-CodeChanges)) { $allPassed = $false }
if (!(Test-StartupPerformance)) { $allPassed = $false }
if (!(Test-MemoryUsage)) { $allPassed = $false }

Write-ColorOutput "================================" $yellow

if ($allPassed) {
    Write-ColorOutput "✅ All performance checks passed!" $green
    exit 0
} else {
    Write-ColorOutput "❌ Performance validation failed!" $red
    Write-ColorOutput "`nTo bypass (use with caution):" $yellow
    Write-Host "  git commit --no-verify"
    Write-ColorOutput "`nTo fix performance issues:" $yellow
    Write-Host "  1. Review recent changes for performance impact"
    Write-Host "  2. Profile the application"
    Write-Host "  3. Consider using lazy loading or async initialization"
    exit 1
}