# Memory Enforcement Hook for Multi-Controller App
# Ensures Cipher memory operations are performed during development

param(
    [Parameter(Position=0)]
    [string]$Action = "check",
    
    [Parameter(Position=1)]
    [string]$Context = ""
)

$ErrorActionPreference = "Stop"
$MemoryLogFile = Join-Path $env:USERPROFILE ".cipher\memory-log.json"
$SessionFile = Join-Path $env:USERPROFILE ".cipher\current-session.json"

function Initialize-MemoryLog {
    if (-not (Test-Path (Split-Path $MemoryLogFile -Parent))) {
        New-Item -ItemType Directory -Path (Split-Path $MemoryLogFile -Parent) -Force | Out-Null
    }
    
    if (-not (Test-Path $MemoryLogFile)) {
        @{
            sessions = @()
            totalOperations = 0
            lastOperation = $null
        } | ConvertTo-Json | Set-Content $MemoryLogFile
    }
    
    # Initialize session tracking
    if (-not (Test-Path $SessionFile)) {
        @{
            sessionId = [Guid]::NewGuid().ToString()
            startTime = (Get-Date).ToString("o")
            operations = @()
            errorsFixes = 0
            patternsStored = 0
            searchesPerformed = 0
        } | ConvertTo-Json | Set-Content $SessionFile
    }
}

function Get-CurrentSession {
    if (Test-Path $SessionFile) {
        return Get-Content $SessionFile -Raw | ConvertFrom-Json
    }
    return $null
}

function Update-Session {
    param(
        [string]$OperationType,
        [string]$Details
    )
    
    $session = Get-CurrentSession
    if ($session) {
        $operation = @{
            type = $OperationType
            timestamp = (Get-Date).ToString("o")
            details = $Details
        }
        
        # Update counters based on operation type
        switch ($OperationType) {
            "error_fix" { $session.errorsFixes++ }
            "pattern_store" { $session.patternsStored++ }
            "memory_search" { $session.searchesPerformed++ }
        }
        
        $session.operations += $operation
        $session | ConvertTo-Json -Depth 10 | Set-Content $SessionFile
    }
}

function Test-MemoryCompliance {
    param(
        [string]$RequiredOperation
    )
    
    $session = Get-CurrentSession
    if (-not $session) {
        return $false
    }
    
    $timeSinceStart = (Get-Date) - [DateTime]::Parse($session.startTime)
    
    switch ($RequiredOperation) {
        "pre-commit" {
            # Must have at least 1 memory operation before committing
            return ($session.operations.Count -gt 0)
        }
        "error-fix" {
            # Must store error fix in memory
            $recentErrorFix = $session.operations | 
                Where-Object { $_.type -eq "error_fix" } | 
                Select-Object -Last 1
            
            if ($recentErrorFix) {
                $timeSinceFix = (Get-Date) - [DateTime]::Parse($recentErrorFix.timestamp)
                return ($timeSinceFix.TotalMinutes -lt 5)
            }
            return $false
        }
        "task-complete" {
            # Must have stored at least one pattern for task completion
            return ($session.patternsStored -gt 0)
        }
        "periodic" {
            # Every 10 operations should have at least 1 memory operation
            $toolUseCount = 10  # This would be tracked separately
            $expectedMemoryOps = [Math]::Floor($toolUseCount / 10)
            return ($session.operations.Count -ge $expectedMemoryOps)
        }
        default {
            return $true
        }
    }
}

function Show-ComplianceStatus {
    $session = Get-CurrentSession
    if (-not $session) {
        Write-Host "[WARNING] No session found - initializing..." -ForegroundColor Yellow
        Initialize-MemoryLog
        return
    }
    
    Write-Host ""
    Write-Host "=== Memory Compliance Status ===" -ForegroundColor Cyan
    Write-Host "Session ID: $($session.sessionId.Substring(0, 8))..."
    Write-Host "Operations performed: $($session.operations.Count)"
    Write-Host "Errors fixed & stored: $($session.errorsFixes)"
    Write-Host "Patterns stored: $($session.patternsStored)"
    Write-Host "Memory searches: $($session.searchesPerformed)"
    
    $timeSinceStart = (Get-Date) - [DateTime]::Parse($session.startTime)
    $expectedOps = [Math]::Floor($timeSinceStart.TotalMinutes / 10)
    
    if ($session.operations.Count -lt $expectedOps) {
        Write-Host "[VIOLATION] Should have $expectedOps memory ops by now!" -ForegroundColor Red
    } else {
        Write-Host "[OK] Memory compliance maintained" -ForegroundColor Green
    }
    Write-Host ""
}

function Invoke-MemoryCheck {
    param(
        [string]$CheckType
    )
    
    switch ($CheckType) {
        "pre-commit" {
            if (-not (Test-MemoryCompliance "pre-commit")) {
                Write-Host "[BLOCKED] Cannot commit without memory operations!" -ForegroundColor Red
                Write-Host "Run one of these first:" -ForegroundColor Yellow
                Write-Host "  - npm run memory:store-pattern" 
                Write-Host "  - npm run memory:store-error"
                Write-Host "  - npm run memory:search -- 'query'"
                exit 1
            }
        }
        "pre-build" {
            Show-ComplianceStatus
        }
        default {
            Show-ComplianceStatus
        }
    }
}

# Main execution
Initialize-MemoryLog

switch ($Action) {
    "check" {
        Invoke-MemoryCheck $Context
    }
    "log" {
        Update-Session $Context $Context
        Write-Host "[OK] Memory operation logged: $Context" -ForegroundColor Green
    }
    "status" {
        Show-ComplianceStatus
    }
    "reset" {
        Remove-Item $SessionFile -Force -ErrorAction SilentlyContinue
        Initialize-MemoryLog
        Write-Host "[OK] Session reset" -ForegroundColor Green
    }
    default {
        Write-Host "Usage: memory-enforcement.ps1 <action> [context]"
        Write-Host "  Actions: check, log, status, reset"
    }
}