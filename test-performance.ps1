# Test Performance Script
$sw = [System.Diagnostics.Stopwatch]::StartNew()
$p = Start-Process -FilePath "target\debug\multi-controller-app.exe" -PassThru -WindowStyle Hidden
Start-Sleep -Milliseconds 500
$memory = $p.WorkingSet64 / 1MB
$p.Kill() 2>$null
$sw.Stop()

Write-Host "==================="
Write-Host "Performance Results"
Write-Host "==================="
Write-Host "Startup time: $($sw.ElapsedMilliseconds) ms"
Write-Host "Memory usage: $([math]::Round($memory, 2)) MB"
Write-Host ""
Write-Host "Performance Budgets:"
Write-Host "- Target startup: <2000ms"
Write-Host "- Target memory: <=150MB"
Write-Host ""
if ($sw.ElapsedMilliseconds -lt 2000) {
    Write-Host "[PASS] Startup time within budget" -ForegroundColor Green
} else {
    Write-Host "[FAIL] Startup time exceeds budget" -ForegroundColor Red
}
if ($memory -le 150) {
    Write-Host "[PASS] Memory usage within budget" -ForegroundColor Green
} else {
    Write-Host "[FAIL] Memory usage exceeds budget" -ForegroundColor Red
}