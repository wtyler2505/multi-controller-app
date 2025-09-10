# Launch Vibe-tree for Multi-Controller App parallel development
# Integrates with TaskMaster and Cipher memory

param(
    [string]$TaskId = "",
    [string]$WorktreeType = "task",
    [switch]$CreateNew,
    [switch]$LaunchOnly
)

$VibeTreePath = "C:\Users\wtyle\AppData\Local\Programs\vibetree\VibeTree.exe"
$ProjectRoot = Split-Path -Parent $PSScriptRoot
$WorktreeBase = Join-Path (Split-Path -Parent $ProjectRoot) "mc-app-worktrees"

function Start-VibeTree {
    Write-Host "[INFO] Launching Vibe-tree..." -ForegroundColor Cyan
    
    if (Test-Path $VibeTreePath) {
        Start-Process $VibeTreePath
        Write-Host "[OK] Vibe-tree launched" -ForegroundColor Green
    } else {
        Write-Host "[ERROR] Vibe-tree not found at: $VibeTreePath" -ForegroundColor Red
        exit 1
    }
}

function Create-TaskWorktree {
    param([string]$Id)
    
    $WorktreePath = Join-Path $WorktreeBase "task-$Id"
    $BranchName = "feature/task-$Id"
    
    if (Test-Path $WorktreePath) {
        Write-Host "[INFO] Worktree already exists: $WorktreePath" -ForegroundColor Yellow
        return $WorktreePath
    }
    
    Write-Host "[INFO] Creating worktree for task $Id..." -ForegroundColor Cyan
    
    # Create worktree directory if needed
    if (!(Test-Path $WorktreeBase)) {
        New-Item -ItemType Directory -Path $WorktreeBase -Force | Out-Null
    }
    
    # Create git worktree
    Push-Location $ProjectRoot
    git worktree add $WorktreePath -b $BranchName
    Pop-Location
    
    # Copy Cipher memory config
    $CipherSource = Join-Path $ProjectRoot ".cipher"
    $CipherDest = Join-Path $WorktreePath ".cipher"
    
    if (Test-Path $CipherSource) {
        Write-Host "[INFO] Linking Cipher memory..." -ForegroundColor Cyan
        # Create symbolic link to share memory
        cmd /c mklink /D $CipherDest $CipherSource 2>$null
    }
    
    Write-Host "[OK] Worktree created: $WorktreePath" -ForegroundColor Green
    return $WorktreePath
}

# Main execution
if ($LaunchOnly) {
    Start-VibeTree
    exit 0
}

if ($CreateNew -and $TaskId) {
    $worktree = Create-TaskWorktree -Id $TaskId
    Write-Host "[INFO] Worktree ready at: $worktree" -ForegroundColor Cyan
    Write-Host "[TIP] Start Claude in worktree: cd $worktree && claude" -ForegroundColor Yellow
}

Start-VibeTree

# Display active worktrees
Write-Host "`n[ACTIVE WORKTREES]" -ForegroundColor Cyan
Push-Location $ProjectRoot
git worktree list
Pop-Location