# Install Cipher Memory Hooks
# Sets up git hooks and development environment integration

param(
    [switch]$Force
)

$ProjectRoot = Split-Path -Parent $PSScriptRoot
$GitHooksSource = Join-Path $ProjectRoot "scripts\git-hooks"
$GitHooksTarget = Join-Path $ProjectRoot ".git\hooks"

Write-Host "Installing Cipher Memory Hooks" -ForegroundColor Cyan
Write-Host "==============================" -ForegroundColor Cyan

# 1. Install Git Hooks
Write-Host "`n[1/3] Installing Git hooks..." -ForegroundColor Yellow

if (Test-Path $GitHooksTarget) {
    $Hooks = @("pre-commit", "post-commit")
    
    foreach ($Hook in $Hooks) {
        $Source = Join-Path $GitHooksSource $Hook
        $Target = Join-Path $GitHooksTarget $Hook
        
        if (Test-Path $Target) {
            if ($Force) {
                Write-Host "  - Overwriting existing $Hook hook" -ForegroundColor Yellow
                Copy-Item -Path $Source -Destination $Target -Force
            } else {
                Write-Host "  - $Hook hook already exists (use -Force to overwrite)" -ForegroundColor Gray
            }
        } else {
            Write-Host "  - Installing $Hook hook" -ForegroundColor Green
            Copy-Item -Path $Source -Destination $Target
        }
    }
} else {
    Write-Host "  [WARNING] .git/hooks directory not found" -ForegroundColor Red
    Write-Host "  Are you in a git repository?" -ForegroundColor Red
}

# 2. Create PowerShell Profile Integration
Write-Host "`n[2/3] Setting up PowerShell integration..." -ForegroundColor Yellow

$ProfileContent = @"

# Cipher Memory Integration
function Store-CodePattern {
    param([string]`$Pattern, [string]`$Description)
    & "$ProjectRoot\scripts\memory-hooks.ps1" -Event "code-pattern" -Context "`$Pattern::`$Description"
}

function Store-TestPass {
    param([string]`$TestName, [string]`$Solution)
    & "$ProjectRoot\scripts\memory-hooks.ps1" -Event "test-pass" -Context "`$TestName::`$Solution"
}

function Store-ErrorResolution {
    param([string]`$Error, [string]`$Resolution)
    & "$ProjectRoot\scripts\memory-hooks.ps1" -Event "error-resolved" -Context "`$Error::`$Resolution"
}

function Start-TaskMemory {
    param([string]`$TaskId)
    & "$ProjectRoot\scripts\memory-hooks.ps1" -Event "task-start" -Context "`$TaskId"
}

function Complete-TaskMemory {
    param([string]`$TaskId, [string]`$Summary)
    & "$ProjectRoot\scripts\memory-hooks.ps1" -Event "task-complete" -Context "`$TaskId::`$Summary"
}

Write-Host "Cipher Memory commands loaded: Store-CodePattern, Store-TestPass, Store-ErrorResolution, Start-TaskMemory, Complete-TaskMemory" -ForegroundColor DarkGray
"@

$ProfilePath = $PROFILE.CurrentUserCurrentHost
$ProfileDir = Split-Path -Parent $ProfilePath

if (-not (Test-Path $ProfileDir)) {
    New-Item -ItemType Directory -Path $ProfileDir -Force | Out-Null
}

if (Test-Path $ProfilePath) {
    $CurrentContent = Get-Content $ProfilePath -Raw
    if ($CurrentContent -notmatch "Cipher Memory Integration") {
        Write-Host "  - Adding Cipher commands to PowerShell profile" -ForegroundColor Green
        Add-Content -Path $ProfilePath -Value $ProfileContent
    } else {
        Write-Host "  - Cipher commands already in PowerShell profile" -ForegroundColor Gray
    }
} else {
    Write-Host "  - Creating PowerShell profile with Cipher commands" -ForegroundColor Green
    Set-Content -Path $ProfilePath -Value $ProfileContent
}

# 3. Create VS Code Tasks Integration
Write-Host "`n[3/3] Creating VS Code task integration..." -ForegroundColor Yellow

$VsCodeDir = Join-Path $ProjectRoot ".vscode"
if (-not (Test-Path $VsCodeDir)) {
    New-Item -ItemType Directory -Path $VsCodeDir -Force | Out-Null
}

$TasksFile = Join-Path $VsCodeDir "tasks.json"
$TasksContent = @"
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Cipher: Start Task Memory",
            "type": "shell",
            "command": "powershell",
            "args": [
                "-File",
                "./scripts/memory-hooks.ps1",
                "-Event",
                "task-start",
                "-Context",
                "\${input:taskId}"
            ],
            "problemMatcher": []
        },
        {
            "label": "Cipher: Store Code Pattern",
            "type": "shell",
            "command": "powershell",
            "args": [
                "-File",
                "./scripts/memory-hooks.ps1",
                "-Event",
                "code-pattern",
                "-Context",
                "\${input:patternDescription}"
            ],
            "problemMatcher": []
        },
        {
            "label": "Cipher: Store Test Solution",
            "type": "shell",
            "command": "powershell",
            "args": [
                "-File",
                "./scripts/memory-hooks.ps1",
                "-Event",
                "test-pass",
                "-Context",
                "\${input:testSolution}"
            ],
            "problemMatcher": []
        },
        {
            "label": "Cipher: Complete Task",
            "type": "shell",
            "command": "powershell",
            "args": [
                "-File",
                "./scripts/memory-hooks.ps1",
                "-Event",
                "task-complete",
                "-Context",
                "\${input:taskSummary}"
            ],
            "problemMatcher": []
        }
    ],
    "inputs": [
        {
            "id": "taskId",
            "type": "promptString",
            "description": "Enter task ID (e.g., 4.2)"
        },
        {
            "id": "patternDescription",
            "type": "promptString",
            "description": "Pattern::Description (e.g., AsyncTransport::Tokio async transport pattern)"
        },
        {
            "id": "testSolution",
            "type": "promptString",
            "description": "TestName::Solution (e.g., SerialTimeout::Added buffer management)"
        },
        {
            "id": "taskSummary",
            "type": "promptString",
            "description": "TaskId::Summary (e.g., 4.2::Implemented serial transport)"
        }
    ]
}
"@

if (-not (Test-Path $TasksFile) -or $Force) {
    Write-Host "  - Creating VS Code tasks for Cipher memory" -ForegroundColor Green
    Set-Content -Path $TasksFile -Value $TasksContent
} else {
    Write-Host "  - VS Code tasks already exist (use -Force to overwrite)" -ForegroundColor Gray
}

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "Cipher Memory Hooks Installation Complete!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan

Write-Host "`nAvailable Commands:" -ForegroundColor Yellow
Write-Host "  PowerShell:" -ForegroundColor Cyan
Write-Host "    - Store-CodePattern" -ForegroundColor White
Write-Host "    - Store-TestPass" -ForegroundColor White
Write-Host "    - Store-ErrorResolution" -ForegroundColor White
Write-Host "    - Start-TaskMemory" -ForegroundColor White
Write-Host "    - Complete-TaskMemory" -ForegroundColor White

Write-Host "`n  Git Hooks:" -ForegroundColor Cyan
Write-Host "    - pre-commit: Extracts code patterns" -ForegroundColor White
Write-Host "    - post-commit: Links to task memories" -ForegroundColor White

Write-Host "`n  VS Code Tasks:" -ForegroundColor Cyan
Write-Host "    - Run with Ctrl+Shift+P > Tasks: Run Task" -ForegroundColor White

Write-Host "`n[NOTE] Restart PowerShell to load new commands" -ForegroundColor Yellow
Write-Host "[NOTE] Git hooks will activate on next commit" -ForegroundColor Yellow