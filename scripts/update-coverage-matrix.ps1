# Update Test Coverage Matrix Script
# Analyzes test files and updates the coverage matrix JSON

param(
    [string]$ProjectRoot = (Get-Location).Path,
    [string]$MatrixFile = "$ProjectRoot\tests\coverage-matrix.json",
    [switch]$GenerateReport
)

Write-Host "Test Coverage Matrix Updater" -ForegroundColor Cyan
Write-Host "=============================" -ForegroundColor Cyan

# Load existing matrix
if (Test-Path $MatrixFile) {
    $matrix = Get-Content $MatrixFile -Raw | ConvertFrom-Json
    Write-Host "Loaded existing matrix from: $MatrixFile" -ForegroundColor Green
} else {
    Write-Host "Matrix file not found: $MatrixFile" -ForegroundColor Red
    exit 1
}

# Count tests in each file
function Count-Tests {
    param([string]$FilePath)
    
    if (!(Test-Path $FilePath)) {
        return 0
    }
    
    $content = Get-Content $FilePath -Raw
    $testPattern = '#\[test\]|#\[tokio::test\]'
    $matches = [regex]::Matches($content, $testPattern)
    return $matches.Count
}

# Analyze test coverage
Write-Host "`nAnalyzing test files..." -ForegroundColor Yellow

$testDir = "$ProjectRoot\tests"
$totalTests = 0
$testsByCategory = @{}

# Unit tests
$unitTests = @(
    Get-ChildItem -Path "$testDir\drivers" -Filter "*.rs" -File
    Get-ChildItem -Path "$testDir" -Filter "*_test.rs" -File
) | Where-Object { $_.Name -ne "mod.rs" }

foreach ($file in $unitTests) {
    $count = Count-Tests $file.FullName
    $totalTests += $count
    if (!$testsByCategory.ContainsKey("unit")) {
        $testsByCategory["unit"] = 0
    }
    $testsByCategory["unit"] += $count
}

# Integration tests
$integrationTests = Get-ChildItem -Path "$testDir\integration" -Filter "*.rs" -File | Where-Object { $_.Name -ne "mod.rs" }
foreach ($file in $integrationTests) {
    $count = Count-Tests $file.FullName
    $totalTests += $count
    if (!$testsByCategory.ContainsKey("integration")) {
        $testsByCategory["integration"] = 0
    }
    $testsByCategory["integration"] += $count
}

# Loopback tests
$loopbackTests = Get-ChildItem -Path "$testDir\loopback" -Filter "*.rs" -File | Where-Object { $_.Name -ne "mod.rs" -and $_.Name -ne "common.rs" }
foreach ($file in $loopbackTests) {
    $count = Count-Tests $file.FullName
    $totalTests += $count
    if (!$testsByCategory.ContainsKey("loopback")) {
        $testsByCategory["loopback"] = 0
    }
    $testsByCategory["loopback"] += $count
}

# Performance tests
$performanceTests = Get-ChildItem -Path "$testDir\performance" -Filter "*.rs" -File | Where-Object { $_.Name -ne "mod.rs" -and $_.Name -ne "benchmark.rs" }
foreach ($file in $performanceTests) {
    $count = Count-Tests $file.FullName
    $totalTests += $count
    if (!$testsByCategory.ContainsKey("performance")) {
        $testsByCategory["performance"] = 0
    }
    $testsByCategory["performance"] += $count
}

Write-Host "Total tests found: $totalTests" -ForegroundColor Green

# Update matrix metadata
$matrix.metadata.totalTests = $totalTests
$matrix.metadata.lastUpdated = (Get-Date -Format "yyyy-MM-dd")

# Update test type counts
foreach ($type in $testsByCategory.Keys) {
    if ($matrix.testTypes.$type) {
        $matrix.testTypes.$type.count = $testsByCategory[$type]
    }
}

# Save updated matrix
$matrix | ConvertTo-Json -Depth 10 | Set-Content $MatrixFile
Write-Host "Updated matrix saved to: $MatrixFile" -ForegroundColor Green

# Generate report if requested
if ($GenerateReport) {
    Write-Host "`nGenerating Coverage Report..." -ForegroundColor Yellow
    
    $report = @"
# Test Coverage Matrix Report
Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Summary
- **Total Tests**: $totalTests
- **Current Coverage**: $($matrix.metadata.currentCoverage)%
- **Target Coverage**: $($matrix.metadata.targetCoverage)%
- **Status**: $(if ($matrix.metadata.currentCoverage -ge $matrix.metadata.targetCoverage) { "✅ TARGET MET" } else { "⚠️ BELOW TARGET" })

## Test Distribution
"@

    foreach ($type in $testsByCategory.Keys | Sort-Object) {
        $report += "`n- **$($type.ToUpper())**: $($testsByCategory[$type]) tests"
    }

    $report += @"

## Coverage by Requirement
| Requirement | Current | Target | Status |
|------------|---------|--------|--------|
"@

    foreach ($req in $matrix.requirements.PSObject.Properties) {
        $current = $req.Value.currentCoverage
        $target = $req.Value.coverageTarget
        $status = if ($current -ge $target) { "✅" } else { "❌" }
        $report += "`n| $($req.Value.name) | $current% | $target% | $status |"
    }

    $report += @"

## Priority Gaps
"@

    $gapCount = 1
    foreach ($gap in $matrix.prioritizedGaps | Select-Object -First 5) {
        $report += "`n$gapCount. **$($gap.description)** ($($gap.requirement))"
        $report += "`n   - Priority: $($gap.priority)"
        $report += "`n   - Effort: $($gap.estimatedEffort)"
        $report += "`n   - Impact: $($gap.coverageImpact)"
        $gapCount++
    }

    $reportFile = "$ProjectRoot\tests\coverage-matrix-report.md"
    $report | Set-Content $reportFile
    Write-Host "Report generated: $reportFile" -ForegroundColor Green
}

Write-Host "`nCoverage Matrix Update Complete!" -ForegroundColor Cyan