# MCP Server Health Check Script - Simplified Version
# Run: powershell -ExecutionPolicy Bypass -File scripts/check-mcp-health.ps1

$ErrorActionPreference = 'Continue'
$StartTime = Get-Date

Write-Host "====================================="
Write-Host "MCP SERVER HEALTH CHECK"
Write-Host "====================================="
Write-Host ""

# Load configuration
$ConfigPath = ".mcp.json"
if (-not (Test-Path $ConfigPath)) {
    Write-Host "ERROR: .mcp.json not found" -ForegroundColor Red
    exit 1
}

$Config = Get-Content $ConfigPath | ConvertFrom-Json
$Servers = $Config.mcpServers.PSObject.Properties

$TotalServers = 0
$HealthyServers = 0
$WarningServers = 0
$CriticalServers = 0

Write-Host "Checking $($Servers.Count) MCP servers..."
Write-Host ""

foreach ($Server in $Servers) {
    $TotalServers++
    $ServerName = $Server.Name
    $ServerConfig = $Server.Value
    $Issues = @()
    
    Write-Host "[$ServerName]" -ForegroundColor Cyan -NoNewline
    
    # Check executable
    if ($ServerConfig.command -eq "node") {
        $ExecPath = $ServerConfig.args[0]
        if (Test-Path $ExecPath) {
            Write-Host " [Exec:OK]" -ForegroundColor Green -NoNewline
        } else {
            Write-Host " [Exec:FAIL]" -ForegroundColor Red -NoNewline
            $Issues += "Executable not found"
        }
    } elseif ($ServerConfig.command -eq "npx") {
        Write-Host " [Exec:NPX]" -ForegroundColor Green -NoNewline
    }
    
    # Check timeout
    if ($ServerConfig.timeout) {
        if ($ServerConfig.timeout -ge 15000) {
            Write-Host " [Timeout:OK]" -ForegroundColor Green -NoNewline
        } else {
            Write-Host " [Timeout:LOW]" -ForegroundColor Yellow -NoNewline
            $Issues += "Timeout too low"
        }
    } else {
        Write-Host " [Timeout:NONE]" -ForegroundColor Yellow -NoNewline
        $Issues += "No timeout set"
    }
    
    # Determine status
    if ($Issues.Count -eq 0) {
        Write-Host " [HEALTHY]" -ForegroundColor Green
        $HealthyServers++
    } elseif ($Issues.Count -eq 1) {
        Write-Host " [WARNING]" -ForegroundColor Yellow
        $WarningServers++
    } else {
        Write-Host " [CRITICAL]" -ForegroundColor Red
        $CriticalServers++
    }
    
    # Show issues if any
    foreach ($Issue in $Issues) {
        Write-Host "  - $Issue" -ForegroundColor DarkYellow
    }
}

Write-Host ""
Write-Host "====================================="
Write-Host "SYSTEM METRICS"
Write-Host "====================================="

# Check Node.js processes
$NodeProcesses = Get-Process -Name node -ErrorAction SilentlyContinue
if ($NodeProcesses) {
    $NodeCount = $NodeProcesses.Count
    $NodeMemory = [math]::Round(($NodeProcesses | Measure-Object WorkingSet -Sum).Sum / 1MB, 2)
    Write-Host "Node Processes: $NodeCount"
    Write-Host "Total Node Memory: ${NodeMemory}MB"
} else {
    Write-Host "No Node.js processes found" -ForegroundColor Yellow
}

# Check Ollama
try {
    $null = Invoke-RestMethod -Uri "http://localhost:11434/api/tags" -TimeoutSec 2 -ErrorAction Stop
    Write-Host "Ollama: Running" -ForegroundColor Green
} catch {
    Write-Host "Ollama: Not Running" -ForegroundColor Yellow
}

# Check Claude Code
$CursorProcess = Get-Process -Name Cursor -ErrorAction SilentlyContinue
if ($CursorProcess) {
    $CursorMemory = [math]::Round($CursorProcess.WorkingSet / 1MB, 2)
    Write-Host "Claude Code Memory: ${CursorMemory}MB"
}

Write-Host ""
Write-Host "====================================="
Write-Host "SUMMARY"
Write-Host "====================================="
Write-Host "Total Servers: $TotalServers"
Write-Host "Healthy: $HealthyServers" -ForegroundColor Green
Write-Host "Warning: $WarningServers" -ForegroundColor Yellow
Write-Host "Critical: $CriticalServers" -ForegroundColor Red

# Calculate health score
if ($TotalServers -gt 0) {
    $HealthScore = [math]::Round(($HealthyServers / $TotalServers) * 100, 1)
    Write-Host ""
    Write-Host -NoNewline "Overall Health Score: "
    if ($HealthScore -ge 80) {
        Write-Host "${HealthScore}%" -ForegroundColor Green
    } elseif ($HealthScore -ge 60) {
        Write-Host "${HealthScore}%" -ForegroundColor Yellow
    } else {
        Write-Host "${HealthScore}%" -ForegroundColor Red
    }
}

# Execution time
$EndTime = Get-Date
$Duration = $EndTime - $StartTime
Write-Host ""
Write-Host "Execution Time: $([math]::Round($Duration.TotalMilliseconds, 2))ms" -ForegroundColor Gray

# Exit code
if ($CriticalServers -gt 0) {
    exit 2
} elseif ($WarningServers -gt 0) {
    exit 1
} else {
    exit 0
}