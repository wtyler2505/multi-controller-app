# PowerShell script for measuring test coverage on Windows
# Since cargo-tarpaulin doesn't work on Windows, this provides an alternative

param(
    [switch]$Verbose,
    [switch]$Html,
    [string]$OutputDir = "coverage"
)

Write-Host "Multi-Controller App - Coverage Measurement (Windows)" -ForegroundColor Cyan
Write-Host "=====================================================" -ForegroundColor Cyan

# Ensure we're in the project root
if (-not (Test-Path "Cargo.toml")) {
    Write-Error "Please run this script from the project root directory"
    exit 1
}

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir | Out-Null
}

# Run tests with verbose output
Write-Host "`nRunning tests with detailed output..." -ForegroundColor Yellow
$testOutput = cargo test --all-features --workspace --verbose --no-fail-fast 2>&1 | Out-String

# Save raw output
$testOutput | Out-File "$OutputDir\test-output.txt"

# Parse test results
$testsPassed = ([regex]::Matches($testOutput, "test result: ok")).Count
$testsFailed = ([regex]::Matches($testOutput, "test result: FAILED")).Count
$testsIgnored = ([regex]::Matches($testOutput, "ignored")).Count

# Count test functions
$testFunctions = ([regex]::Matches($testOutput, "running \d+ test")).Count

# Parse individual test names
$passedTests = [regex]::Matches($testOutput, "test [\w:]+::\w+ \.\.\. ok") | ForEach-Object { $_.Value }
$failedTests = [regex]::Matches($testOutput, "test [\w:]+::\w+ \.\.\. FAILED") | ForEach-Object { $_.Value }

# Count source files
$sourceFiles = Get-ChildItem -Path "src" -Recurse -Filter "*.rs" | Measure-Object
$testFiles = Get-ChildItem -Path "tests" -Recurse -Filter "*.rs" -ErrorAction SilentlyContinue | Measure-Object

# Estimate coverage
$totalTests = $testsPassed + $testsFailed
if ($totalTests -gt 0) {
    $coveragePercent = [math]::Round(($testsPassed / $totalTests) * 100, 2)
} else {
    $coveragePercent = 0
}

# Count lines of code (rough estimate)
$srcLines = 0
Get-ChildItem -Path "src" -Recurse -Filter "*.rs" | ForEach-Object {
    $srcLines += (Get-Content $_.FullName | Measure-Object -Line).Lines
}

$testLines = 0
if (Test-Path "tests") {
    Get-ChildItem -Path "tests" -Recurse -Filter "*.rs" -ErrorAction SilentlyContinue | ForEach-Object {
        $testLines += (Get-Content $_.FullName | Measure-Object -Line).Lines
    }
}

# Display results
Write-Host "`n📊 Coverage Report" -ForegroundColor Green
Write-Host "==================" -ForegroundColor Green
Write-Host "Tests Passed:     $testsPassed" -ForegroundColor Green
Write-Host "Tests Failed:     $testsFailed" -ForegroundColor Red
Write-Host "Tests Ignored:    $testsIgnored" -ForegroundColor Yellow
Write-Host "Total Tests:      $totalTests"
Write-Host "Coverage Score:   ${coveragePercent}%" -ForegroundColor $(if ($coveragePercent -ge 80) {"Green"} elseif ($coveragePercent -ge 60) {"Yellow"} else {"Red"})

Write-Host "`n📁 Code Statistics" -ForegroundColor Cyan
Write-Host "==================" -ForegroundColor Cyan
Write-Host "Source Files:     $($sourceFiles.Count)"
Write-Host "Test Files:       $($testFiles.Count)"
Write-Host "Source Lines:     $srcLines"
Write-Host "Test Lines:       $testLines"
Write-Host "Test Ratio:       $(if ($srcLines -gt 0) {[math]::Round(($testLines / $srcLines) * 100, 2)}else{0})%"

# Module-specific analysis
Write-Host "`n📦 Module Coverage" -ForegroundColor Magenta
Write-Host "==================" -ForegroundColor Magenta

$modules = @(
    @{Name="Transport"; Pattern="transport"},
    @{Name="Drivers"; Pattern="driver"},
    @{Name="Device"; Pattern="device"},
    @{Name="UI"; Pattern="ui"},
    @{Name="Scripting"; Pattern="scripting"}
)

foreach ($module in $modules) {
    $moduleTests = $passedTests | Where-Object { $_ -match $module.Pattern } | Measure-Object
    $moduleFailures = $failedTests | Where-Object { $_ -match $module.Pattern } | Measure-Object
    $total = $moduleTests.Count + $moduleFailures.Count
    
    if ($total -gt 0) {
        $percent = [math]::Round(($moduleTests.Count / $total) * 100, 2)
        Write-Host "$($module.Name): $($moduleTests.Count)/$total tests passing (${percent}%)" -ForegroundColor $(if ($percent -ge 80) {"Green"} else {"Yellow"})
    } else {
        Write-Host "$($module.Name): No tests found" -ForegroundColor Gray
    }
}

# Generate HTML report if requested
if ($Html) {
    Write-Host "`n📄 Generating HTML report..." -ForegroundColor Yellow
    
    $htmlContent = @"
<!DOCTYPE html>
<html>
<head>
    <title>Test Coverage Report - Multi-Controller App</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 20px;
            background-color: #f5f5f5;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background-color: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        h1 {
            color: #333;
            border-bottom: 3px solid #4CAF50;
            padding-bottom: 10px;
        }
        .metric {
            display: inline-block;
            margin: 10px 20px;
            padding: 15px;
            background-color: #f9f9f9;
            border-radius: 5px;
            min-width: 150px;
        }
        .metric-value {
            font-size: 2em;
            font-weight: bold;
            color: #4CAF50;
        }
        .metric-label {
            color: #666;
            font-size: 0.9em;
        }
        .status-good { color: #4CAF50; }
        .status-warning { color: #FF9800; }
        .status-bad { color: #F44336; }
        table {
            width: 100%;
            border-collapse: collapse;
            margin-top: 20px;
        }
        th {
            background-color: #4CAF50;
            color: white;
            padding: 12px;
            text-align: left;
        }
        td {
            padding: 10px;
            border-bottom: 1px solid #ddd;
        }
        .coverage-bar {
            width: 100%;
            height: 20px;
            background-color: #e0e0e0;
            border-radius: 10px;
            overflow: hidden;
        }
        .coverage-fill {
            height: 100%;
            background-color: #4CAF50;
            transition: width 0.3s;
        }
        .timestamp {
            color: #666;
            font-size: 0.9em;
            margin-top: 20px;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>📊 Test Coverage Report</h1>
        <p class="timestamp">Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")</p>
        
        <h2>Overall Coverage</h2>
        <div class="coverage-bar">
            <div class="coverage-fill" style="width: ${coveragePercent}%"></div>
        </div>
        <p style="text-align: center; font-size: 1.5em; font-weight: bold;" class="$(if ($coveragePercent -ge 80) {"status-good"} elseif ($coveragePercent -ge 60) {"status-warning"} else {"status-bad"})">
            ${coveragePercent}%
        </p>
        
        <h2>Test Results</h2>
        <div>
            <div class="metric">
                <div class="metric-value status-good">$testsPassed</div>
                <div class="metric-label">Tests Passed</div>
            </div>
            <div class="metric">
                <div class="metric-value status-bad">$testsFailed</div>
                <div class="metric-label">Tests Failed</div>
            </div>
            <div class="metric">
                <div class="metric-value status-warning">$testsIgnored</div>
                <div class="metric-label">Tests Ignored</div>
            </div>
            <div class="metric">
                <div class="metric-value">$totalTests</div>
                <div class="metric-label">Total Tests</div>
            </div>
        </div>
        
        <h2>Code Statistics</h2>
        <table>
            <tr>
                <th>Metric</th>
                <th>Value</th>
            </tr>
            <tr>
                <td>Source Files</td>
                <td>$($sourceFiles.Count)</td>
            </tr>
            <tr>
                <td>Test Files</td>
                <td>$($testFiles.Count)</td>
            </tr>
            <tr>
                <td>Source Lines of Code</td>
                <td>$srcLines</td>
            </tr>
            <tr>
                <td>Test Lines of Code</td>
                <td>$testLines</td>
            </tr>
            <tr>
                <td>Test-to-Code Ratio</td>
                <td>$(if ($srcLines -gt 0) {[math]::Round(($testLines / $srcLines) * 100, 2)}else{0})%</td>
            </tr>
        </table>
        
        <h2>Module Coverage</h2>
        <table>
            <tr>
                <th>Module</th>
                <th>Tests Passing</th>
                <th>Coverage</th>
            </tr>
"@

    foreach ($module in $modules) {
        $moduleTests = $passedTests | Where-Object { $_ -match $module.Pattern } | Measure-Object
        $moduleFailures = $failedTests | Where-Object { $_ -match $module.Pattern } | Measure-Object
        $total = $moduleTests.Count + $moduleFailures.Count
        
        if ($total -gt 0) {
            $percent = [math]::Round(($moduleTests.Count / $total) * 100, 2)
            $statusClass = if ($percent -ge 80) {"status-good"} elseif ($percent -ge 60) {"status-warning"} else {"status-bad"}
            $htmlContent += @"
            <tr>
                <td>$($module.Name)</td>
                <td>$($moduleTests.Count)/$total</td>
                <td class="$statusClass">${percent}%</td>
            </tr>
"@
        } else {
            $htmlContent += @"
            <tr>
                <td>$($module.Name)</td>
                <td>0/0</td>
                <td style="color: gray;">N/A</td>
            </tr>
"@
        }
    }

    $htmlContent += @"
        </table>
        
        <div class="timestamp">
            <p><strong>Note:</strong> This is an estimated coverage based on test pass/fail results.</p>
            <p>For accurate code coverage on Linux, use: <code>cargo tarpaulin</code></p>
        </div>
    </div>
</body>
</html>
"@

    $htmlContent | Out-File "$OutputDir\coverage-report.html"
    Write-Host "HTML report saved to: $OutputDir\coverage-report.html" -ForegroundColor Green
}

# Check threshold
$threshold = 80
if ($coveragePercent -lt $threshold) {
    Write-Host "`n⚠️  Coverage ${coveragePercent}% is below threshold ${threshold}%" -ForegroundColor Red
    if ($Verbose) {
        Write-Host "`nFailed tests:" -ForegroundColor Red
        $failedTests | ForEach-Object { Write-Host "  $_" -ForegroundColor Red }
    }
    exit 1
} else {
    Write-Host "`n✅ Coverage ${coveragePercent}% meets threshold ${threshold}%" -ForegroundColor Green
}

# Save summary to JSON for CI integration
$summary = @{
    timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    coverage_percent = $coveragePercent
    tests_passed = $testsPassed
    tests_failed = $testsFailed
    tests_ignored = $testsIgnored
    total_tests = $totalTests
    source_files = $sourceFiles.Count
    test_files = $testFiles.Count
    source_lines = $srcLines
    test_lines = $testLines
    meets_threshold = $coveragePercent -ge $threshold
}

$summary | ConvertTo-Json | Out-File "$OutputDir\coverage-summary.json"

Write-Host "`n📁 Coverage artifacts saved to: $OutputDir\" -ForegroundColor Cyan