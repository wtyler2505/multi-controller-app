# MCP Server Health Check Script
# Brutal, comprehensive health verification for all MCP servers

param(
    [switch]$Verbose,
    [switch]$SaveReport,
    [string]$OutputPath = "./.claude/reports/mcp-health-$(Get-Date -Format 'yyyyMMdd-HHmmss').json"
)

# Configuration
$ProjectRoot = (Get-Location).Path
$McpConfigPath = Join-Path $ProjectRoot ".mcp.json"
$StartTime = Get-Date

Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "MCP FORENSIC HEALTH CHECK" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "Time: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" -ForegroundColor Gray
Write-Host "Mode: $(if ($Verbose) {'VERBOSE'} else {'STANDARD'})" -ForegroundColor Gray
Write-Host ""

# Results storage
$Results = @{
    timestamp = $StartTime.ToString("o")
    projectRoot = $ProjectRoot
    servers = @{}
    summary = @{
        total = 0
        healthy = 0
        warning = 0
        critical = 0
    }
    metrics = @{}
}

# Load MCP configuration
if (-not (Test-Path $McpConfigPath)) {
    Write-Host "CRITICAL: .mcp.json not found!" -ForegroundColor Red
    exit 1
}

$McpConfig = Get-Content $McpConfigPath | ConvertFrom-Json
$Servers = $McpConfig.mcpServers.PSObject.Properties

Write-Host "Found $($Servers.Count) MCP servers to check" -ForegroundColor Yellow
Write-Host ""

# Function: Test server process
function Test-ServerProcess {
    param($ServerName, $ServerConfig)
    
    $result = @{
        name = $ServerName
        status = "unknown"
        checks = @{}
        metrics = @{}
        issues = @()
    }
    
    Write-Host "[$ServerName]" -ForegroundColor Cyan -NoNewline
    
    # Check 1: Configuration validity
    Write-Host " Config" -NoNewline
    if ($ServerConfig.command -and $ServerConfig.args) {
        $result.checks.config = "pass"
        Write-Host "OK" -ForegroundColor Green -NoNewline
    } else {
        $result.checks.config = "fail"
        $result.issues += "Invalid configuration"
        Write-Host "X" -ForegroundColor Red -NoNewline
    }
    
    # Check 2: Executable exists
    Write-Host " Exec" -NoNewline
    if ($ServerConfig.command -eq "node") {
        $execPath = $ServerConfig.args[0]
        if (Test-Path $execPath) {
            $result.checks.executable = "pass"
            Write-Host "OK" -ForegroundColor Green -NoNewline
        } else {
            $result.checks.executable = "fail"
            $result.issues += "Executable not found: $execPath"
            Write-Host "X" -ForegroundColor Red -NoNewline
        }
    } elseif ($ServerConfig.command -eq "npx") {
        $result.checks.executable = "pass"
        Write-Host "OK" -ForegroundColor Green -NoNewline
    } else {
        $result.checks.executable = "unknown"
        Write-Host "?" -ForegroundColor Gray -NoNewline
    }
    
    # Check 3: Timeout configuration
    Write-Host " Timeout" -NoNewline
    if ($ServerConfig.timeout) {
        $result.metrics.timeout = $ServerConfig.timeout
        if ($ServerConfig.timeout -ge 15000 -and $ServerConfig.timeout -le 120000) {
            $result.checks.timeout = "pass"
            Write-Host "OK" -ForegroundColor Green -NoNewline
        } else {
            $result.checks.timeout = "warn"
            $result.issues += "Timeout outside recommended range (15-120s): $($ServerConfig.timeout)ms"
            Write-Host "!" -ForegroundColor Yellow -NoNewline
        }
    } else {
        $result.checks.timeout = "warn"
        $result.issues += "No timeout configured"
        Write-Host "!" -ForegroundColor Yellow -NoNewline
    }
    
    # Check 4: Environment variables
    Write-Host " Env" -NoNewline
    if ($ServerConfig.env) {
        $missingEnv = @()
        foreach ($envVar in $ServerConfig.env.PSObject.Properties) {
            if ($envVar.Value -match '\$\{(.+)\}') {
                $varName = $Matches[1]
                if (-not [Environment]::GetEnvironmentVariable($varName)) {
                    $missingEnv += $varName
                }
            }
        }
        if ($missingEnv.Count -eq 0) {
            $result.checks.environment = "pass"
            Write-Host "OK" -ForegroundColor Green -NoNewline
        } else {
            $result.checks.environment = "fail"
            $result.issues += "Missing environment variables: $($missingEnv -join ', ')"
            Write-Host "X" -ForegroundColor Red -NoNewline
        }
    } else {
        $result.checks.environment = "pass"
        Write-Host "OK" -ForegroundColor Green -NoNewline
    }
    
    # Check 5: Process memory (if running)
    Write-Host " Memory" -NoNewline
    $nodeProcesses = Get-Process -Name node -ErrorAction SilentlyContinue
    if ($nodeProcesses) {
        $totalMemory = ($nodeProcesses | Measure-Object WorkingSet -Sum).Sum / 1MB
        $result.metrics.memory_mb = [math]::Round($totalMemory, 2)
        if ($totalMemory -lt 100) {
            $result.checks.memory = "pass"
            Write-Host "OK" -ForegroundColor Green -NoNewline
        } elseif ($totalMemory -lt 200) {
            $result.checks.memory = "warn"
            $result.issues += "High memory usage: $([math]::Round($totalMemory, 2))MB"
            Write-Host "!" -ForegroundColor Yellow -NoNewline
        } else {
            $result.checks.memory = "fail"
            $result.issues += "Excessive memory usage: $([math]::Round($totalMemory, 2))MB"
            Write-Host "X" -ForegroundColor Red -NoNewline
        }
    } else {
        $result.checks.memory = "unknown"
        Write-Host "?" -ForegroundColor Gray -NoNewline
    }
    
    # Determine overall status
    $failCount = ($result.checks.Values | Where-Object { $_ -eq "fail" }).Count
    $warnCount = ($result.checks.Values | Where-Object { $_ -eq "warn" }).Count
    
    if ($failCount -gt 0) {
        $result.status = "critical"
        Write-Host " [CRITICAL]" -ForegroundColor Red
        $Results.summary.critical++
    } elseif ($warnCount -gt 0) {
        $result.status = "warning"
        Write-Host " [WARNING]" -ForegroundColor Yellow
        $Results.summary.warning++
    } else {
        $result.status = "healthy"
        Write-Host " [HEALTHY]" -ForegroundColor Green
        $Results.summary.healthy++
    }
    
    if ($Verbose -and $result.issues.Count -gt 0) {
        foreach ($issue in $result.issues) {
            Write-Host "  └─ $issue" -ForegroundColor DarkYellow
        }
    }
    
    return $result
}

# Function: System metrics
function Get-SystemMetrics {
    Write-Host ""
    Write-Host "System Metrics:" -ForegroundColor Cyan
    
    $metrics = @{}
    
    # Total Node.js processes
    $nodeProcs = Get-Process -Name node -ErrorAction SilentlyContinue
    if ($nodeProcs) {
        $metrics.node_process_count = $nodeProcs.Count
        $metrics.total_node_memory_mb = [math]::Round(($nodeProcs | Measure-Object WorkingSet -Sum).Sum / 1MB, 2)
        $metrics.total_node_cpu = [math]::Round(($nodeProcs | Measure-Object CPU -Sum).Sum, 2)
        
        Write-Host "  Node Processes: $($metrics.node_process_count)" -ForegroundColor Gray
        Write-Host "  Total Memory: $($metrics.total_node_memory_mb) MB" -ForegroundColor Gray
        Write-Host "  Total CPU Time: $($metrics.total_node_cpu)s" -ForegroundColor Gray
    } else {
        Write-Host "  No Node.js processes found" -ForegroundColor Yellow
    }
    
    # Claude Code process
    $claudeProc = Get-Process -Name Cursor -ErrorAction SilentlyContinue
    if ($claudeProc) {
        $metrics.claude_memory_mb = [math]::Round($claudeProc.WorkingSet / 1MB, 2)
        Write-Host "  Claude Code Memory: $($metrics.claude_memory_mb) MB" -ForegroundColor Gray
    }
    
    # Ollama status (for embeddings)
    try {
        $ollamaTest = Invoke-RestMethod -Uri "http://localhost:11434/api/tags" -TimeoutSec 2 -ErrorAction SilentlyContinue
        $metrics.ollama_status = "running"
        Write-Host "  Ollama: Running" -ForegroundColor Green
    } catch {
        $metrics.ollama_status = "not_running"
        Write-Host "  Ollama: Not Running" -ForegroundColor Yellow
    }
    
    return $metrics
}

# Main execution
Write-Host "Starting health checks..." -ForegroundColor Yellow
Write-Host "-" * 40

# Check each server
foreach ($server in $Servers) {
    $Results.servers[$server.Name] = Test-ServerProcess -ServerName $server.Name -ServerConfig $server.Value
    $Results.summary.total++
}

# Get system metrics
$Results.metrics = Get-SystemMetrics

# Calculate execution time
$EndTime = Get-Date
$Duration = $EndTime - $StartTime
$Results.execution_time_ms = [math]::Round($Duration.TotalMilliseconds, 2)

# Summary
Write-Host ""
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "SUMMARY" -ForegroundColor Cyan
Write-Host "=====================================" -ForegroundColor Cyan
Write-Host "Total Servers: $($Results.summary.total)" -ForegroundColor White
Write-Host "Healthy: $($Results.summary.healthy)" -ForegroundColor Green
Write-Host "Warning: $($Results.summary.warning)" -ForegroundColor Yellow
Write-Host "Critical: $($Results.summary.critical)" -ForegroundColor Red
Write-Host "Execution Time: $($Results.execution_time_ms)ms" -ForegroundColor Gray

# Overall health score
$healthScore = [math]::Round(($Results.summary.healthy / $Results.summary.total) * 100, 1)
Write-Host ""
Write-Host "Overall Health Score: " -NoNewline
if ($healthScore -ge 80) {
    Write-Host "$healthScore%" -ForegroundColor Green
} elseif ($healthScore -ge 60) {
    Write-Host "$healthScore%" -ForegroundColor Yellow
} else {
    Write-Host "$healthScore%" -ForegroundColor Red
}

# Save report if requested
if ($SaveReport) {
    $reportDir = Split-Path $OutputPath -Parent
    if (-not (Test-Path $reportDir)) {
        New-Item -ItemType Directory -Path $reportDir -Force | Out-Null
    }
    $Results | ConvertTo-Json -Depth 10 | Set-Content $OutputPath
    Write-Host ""
    Write-Host ('Report saved to: ' + $OutputPath) -ForegroundColor Green
}

# Exit code based on health
if ($Results.summary.critical -gt 0) {
    exit 2  # Critical issues found
} elseif ($Results.summary.warning -gt 0) {
    exit 1  # Warnings found
} else {
    exit 0  # All healthy
}