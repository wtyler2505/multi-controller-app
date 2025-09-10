# TaskMaster Wrapper Script with Research Enforcement
# Wraps task-master commands to enforce research requirements
# Usage: ./scripts/task-master-wrapper.ps1 [command] [args]

param(
    [Parameter(Position=0)]
    [string]$Command,
    
    [Parameter(Position=1, ValueFromRemainingArguments)]
    [string[]]$Arguments
)

$ErrorActionPreference = "Stop"

# Commands that require research check
$researchRequiredCommands = @(
    "expand",
    "update",
    "update-task",
    "update-subtask",
    "set-status",
    "add-task",
    "add-subtask"
)

# Extract task ID from arguments
function Get-TaskIdFromArgs {
    param([string[]]$Args)
    
    foreach ($arg in $Args) {
        # Check for --id=X format
        if ($arg -match '--id=(\d+(?:\.\d+)?)') {
            return $matches[1]
        }
        # Check for plain number argument
        if ($arg -match '^\d+(?:\.\d+)?$') {
            return $arg
        }
    }
    
    # Try to get from branch name
    $branch = git rev-parse --abbrev-ref HEAD 2>$null
    if ($branch -match 'task[- ]?(\d+)') {
        return $matches[1]
    }
    
    return $null
}

# Main logic
Write-Host "[TaskMaster Wrapper] Command: $Command" -ForegroundColor Cyan

# Check if this command requires research
if ($Command -in $researchRequiredCommands) {
    $taskId = Get-TaskIdFromArgs -Args $Arguments
    
    if ($taskId) {
        Write-Host "[Research Check] Validating research for task $taskId..." -ForegroundColor Yellow
        
        # Run research check
        $scriptPath = Join-Path $PSScriptRoot "pre-task-research.ps1"
        if (Test-Path $scriptPath) {
            & $scriptPath -TaskId $taskId
            
            if ($LASTEXITCODE -ne 0) {
                Write-Host "[BLOCKED] Research required before running: task-master $Command" -ForegroundColor Red
                Write-Host "[HINT] Run research first: task-master research --query 'task $taskId implementation approaches'" -ForegroundColor Yellow
                exit 1
            }
        }
    }
}

# Special handling for 'next' command - add research reminder
if ($Command -eq "next") {
    Write-Host "================================================" -ForegroundColor Blue
    Write-Host "REMINDER: Research before implementation!" -ForegroundColor Blue
    Write-Host "Use: task-master research --query 'task X approaches'" -ForegroundColor Blue
    Write-Host "================================================" -ForegroundColor Blue
}

# Execute the actual task-master command
Write-Host "[Executing] task-master $Command $($Arguments -join ' ')" -ForegroundColor Green

if ($Command) {
    task-master $Command @Arguments
} else {
    task-master @Arguments
}

$exitCode = $LASTEXITCODE

# Post-execution checks
if ($exitCode -eq 0) {
    # If task was marked complete, verify research was done
    if ($Command -eq "set-status" -and $Arguments -contains "--status=done") {
        $taskId = Get-TaskIdFromArgs -Args $Arguments
        if ($taskId) {
            Write-Host "[Verification] Checking research compliance for completed task $taskId..." -ForegroundColor Yellow
            
            # Log completion
            $logFile = Join-Path $PSScriptRoot "..\\.taskmaster\\compliance.log"
            $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
            "$timestamp | Task $taskId | Completed with research check" | Out-File -Append -FilePath $logFile -Encoding utf8
        }
    }
}

exit $exitCode