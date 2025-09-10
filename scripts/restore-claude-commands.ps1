# Restore Claude Code Custom Commands
# This script copies project commands to global directory for immediate availability

Write-Host "🚀 Restoring Claude Code Custom Commands..." -ForegroundColor Cyan

$projectCommands = "C:\Users\wtyle\multi-controller-app\.claude\commands"
$globalCommands = "$env:USERPROFILE\.claude\commands"

# Ensure global commands directory exists
if (!(Test-Path $globalCommands)) {
    New-Item -ItemType Directory -Path $globalCommands -Force | Out-Null
    Write-Host "✅ Created global commands directory" -ForegroundColor Green
}

# Get all .md files from project commands
$commandFiles = Get-ChildItem -Path $projectCommands -Filter "*.md" -File

$copiedCount = 0
foreach ($file in $commandFiles) {
    $targetPath = Join-Path $globalCommands $file.Name
    
    # Check if file exists in global
    if (Test-Path $targetPath) {
        Write-Host "⚠️  Skipping $($file.BaseName) - already exists in global" -ForegroundColor Yellow
    } else {
        Copy-Item $file.FullName -Destination $targetPath
        Write-Host "✅ Copied $($file.BaseName)" -ForegroundColor Green
        $copiedCount++
    }
}

Write-Host "`n📊 Summary:" -ForegroundColor Cyan
Write-Host "   Total project commands: $($commandFiles.Count)" -ForegroundColor White
Write-Host "   Newly copied: $copiedCount" -ForegroundColor White
Write-Host "   Already existed: $($commandFiles.Count - $copiedCount)" -ForegroundColor White

Write-Host "`n🎯 HOW TO USE YOUR COMMANDS:" -ForegroundColor Magenta
Write-Host "   Type '/' followed by command name:" -ForegroundColor White
Write-Host "   /ultra-think - Deep analysis mode" -ForegroundColor Gray
Write-Host "   /code-review - Code review" -ForegroundColor Gray
Write-Host "   /cipher-memory - Memory operations" -ForegroundColor Gray
Write-Host "   /debug-error - Debug assistance" -ForegroundColor Gray
Write-Host "`n   For namespaced access:" -ForegroundColor White
Write-Host "   /user:ultra-think - Global command" -ForegroundColor Gray
Write-Host "   /project:ultra-think - Project command" -ForegroundColor Gray

Write-Host ""
Write-Host "IMPORTANT: Restart Claude Code if commands do not appear immediately" -ForegroundColor Yellow