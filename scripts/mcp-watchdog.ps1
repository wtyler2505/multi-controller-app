[CmdletBinding()]
param(
    [string]$ServerName = "taskmaster-ai",
    [int]$MaxRuntimeMinutes = 60,
    [int]$RestartDelaySeconds = 5,
    [switch]$ShowDetails
)

# MCP Server Watchdog - Auto-restart on memory issues
# Prevents JavaScript heap out of memory errors

Write-Host "[WATCHDOG] Starting MCP Server Watchdog for: $ServerName" -ForegroundColor Green
Write-Host "[WATCHDOG] Max runtime: $MaxRuntimeMinutes minutes" -ForegroundColor Yellow
Write-Host "[WATCHDOG] Restart delay: $RestartDelaySeconds seconds" -ForegroundColor Yellow

$iteration = 0

while ($true) {
    $iteration++
    $startTime = Get-Date
    
    Write-Host "`n[WATCHDOG] Iteration $iteration - Starting $ServerName at $startTime" -ForegroundColor Cyan
    
    # Start the MCP server process
    $processArgs = @{
        FilePath = "npm.cmd"
        ArgumentList = "run", "mcp:$ServerName"
        WorkingDirectory = (Split-Path -Parent $PSScriptRoot)
        PassThru = $true
        NoNewWindow = $true
    }
    
    try {
        $process = Start-Process @processArgs
        
        if ($ShowDetails) {
            Write-Host "[WATCHDOG] Process started with PID: $($process.Id)" -ForegroundColor Gray
        }
        
        # Wait for the specified runtime or until process exits
        $timeoutMs = $MaxRuntimeMinutes * 60 * 1000
        $hasExited = $process.WaitForExit($timeoutMs)
        
        if (!$hasExited) {
            # Process still running after timeout - force restart
            Write-Host "[WATCHDOG] Runtime limit reached. Stopping process..." -ForegroundColor Yellow
            
            try {
                Stop-Process -Id $process.Id -Force -ErrorAction Stop
                Write-Host "[WATCHDOG] Process stopped successfully" -ForegroundColor Green
            }
            catch {
                Write-Host "[WATCHDOG] Warning: Failed to stop process: $_" -ForegroundColor Red
            }
        }
        else {
            # Process exited on its own
            $exitCode = $process.ExitCode
            $runtime = (Get-Date) - $startTime
            
            if ($exitCode -eq 0) {
                Write-Host "[WATCHDOG] Process exited normally after $($runtime.TotalMinutes) minutes" -ForegroundColor Green
            }
            else {
                Write-Host "[WATCHDOG] Process crashed with exit code $exitCode after $($runtime.TotalMinutes) minutes" -ForegroundColor Red
                
                # Check if it was a memory error
                if ($exitCode -eq -1073741819 -or $exitCode -eq 134) {
                    Write-Host "[WATCHDOG] Detected memory allocation failure!" -ForegroundColor Red
                }
            }
        }
    }
    catch {
        Write-Host "[WATCHDOG] Error starting/monitoring process: $_" -ForegroundColor Red
    }
    
    # Wait before restarting
    Write-Host "[WATCHDOG] Waiting $RestartDelaySeconds seconds before restart..." -ForegroundColor Yellow
    Start-Sleep -Seconds $RestartDelaySeconds
}