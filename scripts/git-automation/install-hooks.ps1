# Install git hooks for Multi-Controller App (Windows)

Write-Host "========================================" -ForegroundColor Blue
Write-Host "     Git Hooks Installation Script    " -ForegroundColor Blue
Write-Host "========================================" -ForegroundColor Blue

# Get the repository root
try {
    $RepoRoot = git rev-parse --show-toplevel 2>$null
    if ($LASTEXITCODE -ne 0) {
        throw "Not in a git repository"
    }
} catch {
    Write-Host "[ERROR] Not in a git repository" -ForegroundColor Red
    exit 1
}

Set-Location $RepoRoot

# Check if hooks directory exists
$HooksSource = "scripts\git-automation\hooks"
$HooksDest = ".git\hooks"

if (!(Test-Path $HooksSource)) {
    Write-Host "[ERROR] Hooks source directory not found: $HooksSource" -ForegroundColor Red
    exit 1
}

# Function to install a hook
function Install-Hook {
    param(
        [string]$HookName
    )
    
    $SourceFile = Join-Path $HooksSource $HookName
    $DestFile = Join-Path $HooksDest $HookName
    
    if (!(Test-Path $SourceFile)) {
        Write-Host "[WARNING] Hook not found: $HookName" -ForegroundColor Yellow
        return $false
    }
    
    # Backup existing hook if it exists
    if (Test-Path $DestFile) {
        $BackupFile = "$DestFile.backup"
        if (!(Test-Path $BackupFile)) {
            Copy-Item $DestFile $BackupFile
            Write-Host "   Backed up existing $HookName to $HookName.backup" -ForegroundColor Yellow
        }
    }
    
    # Copy hook file
    Copy-Item $SourceFile $DestFile -Force
    
    # Create a .bat wrapper for Windows Git
    $BatContent = "@echo off`nbash `"%~dp0$HookName`" %*"
    Set-Content -Path "$DestFile.bat" -Value $BatContent -Encoding ASCII
    
    Write-Host "[OK] Installed: $HookName" -ForegroundColor Green
    return $true
}

# Install hooks
Write-Host "`nInstalling hooks..." -ForegroundColor Blue
$hooks = @("pre-commit", "commit-msg", "pre-push")
$installedCount = 0

foreach ($hook in $hooks) {
    if (Install-Hook -HookName $hook) {
        $installedCount++
    }
}

# Verify installation
Write-Host "`nVerifying installation..." -ForegroundColor Blue
$verifiedCount = 0
foreach ($hook in $hooks) {
    $hookPath = Join-Path $HooksDest $hook
    if ((Test-Path $hookPath) -or (Test-Path "$hookPath.bat")) {
        $verifiedCount++
    }
}

Write-Host "[OK] $verifiedCount/3 hooks installed successfully" -ForegroundColor Green

# Configuration check
Write-Host "`nChecking configuration files..." -ForegroundColor Blue
if (Test-Path ".gitmeta\config\secrets-patterns.json") {
    Write-Host "[OK] Secrets patterns configured" -ForegroundColor Green
} else {
    Write-Host "[WARNING] Secrets patterns not found" -ForegroundColor Yellow
}

if (Test-Path ".gitmeta\config\performance-budgets.json") {
    Write-Host "[OK] Performance budgets configured" -ForegroundColor Green
} else {
    Write-Host "[WARNING] Performance budgets not found" -ForegroundColor Yellow
}

# Check for required tools
Write-Host "`nChecking required tools..." -ForegroundColor Blue
$nodeVersion = node --version 2>$null
if ($nodeVersion) {
    Write-Host "[OK] Node.js installed: $nodeVersion" -ForegroundColor Green
} else {
    Write-Host "[WARNING] Node.js not found (required for secrets scanner)" -ForegroundColor Yellow
}

$gitBashPath = Get-Command bash -ErrorAction SilentlyContinue
if ($gitBashPath) {
    Write-Host "[OK] Git Bash installed" -ForegroundColor Green
} else {
    Write-Host "[WARNING] Git Bash not found (required for hooks)" -ForegroundColor Yellow
}

# Final instructions
Write-Host "`n========================================" -ForegroundColor Blue
Write-Host "Git hooks installation complete!" -ForegroundColor Green
Write-Host "`nHooks installed:" -ForegroundColor Blue
Write-Host "  * pre-commit  - Runs secrets scanner and performance checks"
Write-Host "  * commit-msg  - Enforces conventional commits and task references"
Write-Host "  * pre-push    - Final validation before pushing to remote"
Write-Host "`nTo bypass hooks (use sparingly):" -ForegroundColor Blue
Write-Host "  git commit --no-verify"
Write-Host "  git push --no-verify"
Write-Host "`nTo uninstall hooks:" -ForegroundColor Blue
Write-Host "  Remove-Item .git\hooks\pre-commit* -Force"
Write-Host "  Remove-Item .git\hooks\commit-msg* -Force"
Write-Host "  Remove-Item .git\hooks\pre-push* -Force"
Write-Host "========================================`n" -ForegroundColor Blue