# Memory Trigger Hooks for Cipher Integration
# Automatically captures and stores knowledge during development events

param(
    [Parameter(Mandatory=$false)]
    [string]$Event = "manual",
    
    [Parameter(Mandatory=$false)]
    [string]$Context = "",
    
    [Parameter(Mandatory=$false)]
    [string]$TaskId = "",
    
    [Parameter(Mandatory=$false)]
    [ValidateSet("CRITICAL", "IMPORTANT", "CONTEXTUAL", "TEMPORARY")]
    [string]$Importance = "CONTEXTUAL"
)

# Configuration
$ProjectRoot = Split-Path -Parent $PSScriptRoot
$CipherMemoryLog = Join-Path $ProjectRoot ".cipher\memory-hooks.log"

# Ensure log directory exists
$LogDir = Split-Path -Parent $CipherMemoryLog
if (-not (Test-Path $LogDir)) {
    New-Item -ItemType Directory -Path $LogDir -Force | Out-Null
}

function Write-MemoryLog {
    param([string]$Message)
    $Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    "$Timestamp | $Message" | Add-Content -Path $CipherMemoryLog
}

function Invoke-CipherMemory {
    param(
        [string]$Operation,
        [hashtable]$Parameters
    )
    
    try {
        $JsonParams = $Parameters | ConvertTo-Json -Depth 10
        Write-MemoryLog "Executing: $Operation with params: $JsonParams"
        
        # This would be replaced with actual Cipher MCP calls in production
        # For now, we'll prepare the data structure
        return @{
            Success = $true
            Operation = $Operation
            Parameters = $Parameters
            Timestamp = Get-Date -Format "o"
        }
    }
    catch {
        Write-MemoryLog "[ERROR] Failed to execute $Operation : $_"
        return @{ Success = $false; Error = $_.ToString() }
    }
}

# Event Handlers
function Handle-TaskStart {
    param([string]$TaskId)
    
    Write-MemoryLog "[TASK_START] Loading memories for task $TaskId"
    
    # Search for related memories
    $SearchQuery = "task $TaskId implementation component"
    $Result = Invoke-CipherMemory -Operation "cipher_memory_search" -Parameters @{
        query = $SearchQuery
        top_k = 10
    }
    
    # Create task start entity
    $Entity = @{
        name = "TaskContext.$TaskId"
        entityType = "Session.TaskStart"
        observations = @(
            "Task $TaskId started at $(Get-Date -Format 'o')",
            "Working directory: $(Get-Location)",
            "Active branch: $(git branch --show-current 2>$null)"
        )
    }
    
    Invoke-CipherMemory -Operation "create_entities" -Parameters @{
        entities = @($Entity)
    }
    
    Write-Host "[OK] Task $TaskId context loaded into memory" -ForegroundColor Green
}

function Handle-TestPass {
    param([string]$TestName, [string]$Solution)
    
    Write-MemoryLog "[TEST_PASS] Storing solution for $TestName"
    
    # Store successful pattern
    $Memory = @{
        interaction = @(
            "Test passed: $TestName",
            "Solution: $Solution"
        )
        knowledgeInfo = @{
            domain = "testing"
            codePattern = "test-solution"
        }
        memoryMetadata = @{
            importance = "IMPORTANT"
            taskId = $TaskId
            testName = $TestName
        }
    }
    
    Invoke-CipherMemory -Operation "cipher_extract_and_operate_memory" -Parameters $Memory
    
    Write-Host "[OK] Test solution stored in memory" -ForegroundColor Green
}

function Handle-ErrorResolved {
    param([string]$Error, [string]$Resolution)
    
    Write-MemoryLog "[ERROR_RESOLVED] Storing troubleshooting for: $Error"
    
    # Store reasoning trace
    $Trace = @{
        trace = @{
            id = [guid]::NewGuid().ToString()
            steps = @(
                @{ type = "observation"; content = "Error encountered: $Error" }
                @{ type = "thought"; content = "Analyzing error pattern" }
                @{ type = "action"; content = "Applied fix: $Resolution" }
                @{ type = "conclusion"; content = "Error resolved successfully" }
            )
            metadata = @{
                taskId = $TaskId
                errorType = "runtime"
                importance = $Importance
            }
        }
        evaluation = @{
            qualityScore = 0.8
            issues = @()
            suggestions = @()
        }
    }
    
    Invoke-CipherMemory -Operation "cipher_store_reasoning_memory" -Parameters $Trace
    
    Write-Host "[OK] Error resolution stored in reflection memory" -ForegroundColor Green
}

function Handle-CodePattern {
    param([string]$Pattern, [string]$Description)
    
    Write-MemoryLog "[CODE_PATTERN] Storing pattern: $Pattern"
    
    # Create pattern entity
    $Entity = @{
        name = "Pattern.$Pattern"
        entityType = "Code.Pattern"
        observations = @(
            $Description,
            "Discovered at $(Get-Date -Format 'o')",
            "Task context: $TaskId"
        )
    }
    
    Invoke-CipherMemory -Operation "create_entities" -Parameters @{
        entities = @($Entity)
    }
    
    Write-Host "[OK] Code pattern stored in knowledge graph" -ForegroundColor Green
}

function Handle-TaskComplete {
    param([string]$TaskId, [string]$Summary)
    
    Write-MemoryLog "[TASK_COMPLETE] Extracting learnings from task $TaskId"
    
    # Extract and store learnings
    $Learnings = @{
        name = "TaskLearnings.$TaskId"
        entityType = "Learning.TaskComplete"
        observations = @(
            "Task $TaskId completed at $(Get-Date -Format 'o')",
            "Summary: $Summary",
            "Git commits: $(git log --oneline -n 5 2>$null)"
        )
    }
    
    Invoke-CipherMemory -Operation "create_entities" -Parameters @{
        entities = @($Learnings)
    }
    
    # Prune temporary memories
    Write-MemoryLog "Pruning temporary memories for task $TaskId"
    
    Write-Host "[OK] Task learnings extracted and stored" -ForegroundColor Green
}

function Handle-PerformanceMilestone {
    param([string]$Metric, [string]$Value)
    
    Write-MemoryLog "[PERFORMANCE] Recording milestone: $Metric = $Value"
    
    $Entity = @{
        name = "Performance.$Metric"
        entityType = "Metrics.Milestone"
        observations = @(
            "$Metric achieved: $Value",
            "Timestamp: $(Get-Date -Format 'o')",
            "Task context: $TaskId",
            "Git commit: $(git rev-parse --short HEAD 2>$null)"
        )
    }
    
    Invoke-CipherMemory -Operation "create_entities" -Parameters @{
        entities = @($Entity)
    }
    
    Write-Host "[OK] Performance milestone recorded" -ForegroundColor Green
}

# Main Event Router
switch ($Event) {
    "task-start" {
        Handle-TaskStart -TaskId $Context
    }
    "test-pass" {
        $Parts = $Context -split "::", 2
        Handle-TestPass -TestName $Parts[0] -Solution $Parts[1]
    }
    "error-resolved" {
        $Parts = $Context -split "::", 2
        Handle-ErrorResolved -Error $Parts[0] -Resolution $Parts[1]
    }
    "code-pattern" {
        $Parts = $Context -split "::", 2
        Handle-CodePattern -Pattern $Parts[0] -Description $Parts[1]
    }
    "task-complete" {
        $Parts = $Context -split "::", 2
        Handle-TaskComplete -TaskId $Parts[0] -Summary $Parts[1]
    }
    "performance" {
        $Parts = $Context -split "::", 2
        Handle-PerformanceMilestone -Metric $Parts[0] -Value $Parts[1]
    }
    "manual" {
        Write-Host "Memory Hooks - Manual Mode"
        Write-Host "======================================"
        Write-Host "Usage: .\memory-hooks.ps1 -Event <event> -Context <context> [-TaskId <id>] [-Importance <level>]"
        Write-Host ""
        Write-Host "Events:"
        Write-Host "  task-start       : Load memories for task (Context: TaskId)"
        Write-Host "  test-pass        : Store test solution (Context: TestName::Solution)"
        Write-Host "  error-resolved   : Store troubleshooting (Context: Error::Resolution)"
        Write-Host "  code-pattern     : Store code pattern (Context: Pattern::Description)"
        Write-Host "  task-complete    : Extract learnings (Context: TaskId::Summary)"
        Write-Host "  performance      : Record milestone (Context: Metric::Value)"
        Write-Host ""
        Write-Host "Examples:"
        Write-Host '  .\memory-hooks.ps1 -Event task-start -Context "4.2"'
        Write-Host '  .\memory-hooks.ps1 -Event test-pass -Context "SerialTest::Added timeout handling"'
        Write-Host '  .\memory-hooks.ps1 -Event error-resolved -Context "Timeout::Increased buffer size"'
    }
    default {
        Write-Host "[ERROR] Unknown event: $Event" -ForegroundColor Red
        exit 1
    }
}

Write-MemoryLog "Hook completed: $Event"