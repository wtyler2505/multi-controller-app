# Shell Context Detector
# Detects the current shell environment and provides appropriate commands
# ASCII-only PowerShell script - no emojis or special Unicode characters

param(
    [Parameter(Position=0)]
    [string]$Command = "",
    
    [Parameter()]
    [switch]$DetectOnly,
    
    [Parameter()]
    [switch]$ShowDetails
)

function Get-CurrentShell {
    <#
    .SYNOPSIS
        Detects the current shell environment
    .DESCRIPTION
        Returns the shell type: PowerShell, Bash, CMD, or Unknown
    #>
    
    # Check if we're in PowerShell
    if ($PSVersionTable) {
        return "PowerShell"
    }
    
    # Check for bash
    if ($env:BASH_VERSION) {
        return "Bash"
    }
    
    # Check for cmd.exe
    if ($env:COMSPEC -like "*cmd.exe") {
        return "CMD"
    }
    
    # Check parent process
    try {
        $parentProcess = Get-Process -Id $PID -ErrorAction SilentlyContinue
        if ($parentProcess.ProcessName -like "*powershell*") {
            return "PowerShell"
        }
        elseif ($parentProcess.ProcessName -like "*bash*") {
            return "Bash"
        }
        elseif ($parentProcess.ProcessName -like "*cmd*") {
            return "CMD"
        }
    }
    catch {
        # Silent fail
    }
    
    return "Unknown"
}

function Get-ShellSpecificCommand {
    param(
        [string]$BaseCommand,
        [string]$Shell
    )
    
    # Command translation map
    $commandMap = @{
        "list-aot" = @{
            "PowerShell" = 'dotnet new list | Select-String "aot"'
            "Bash" = 'dotnet new list | grep -i "aot"'
            "CMD" = 'dotnet new list | findstr /i "aot"'
        }
        "find-files" = @{
            "PowerShell" = 'Get-ChildItem -Recurse -Filter'
            "Bash" = 'find . -name'
            "CMD" = 'dir /s /b'
        }
        "grep-content" = @{
            "PowerShell" = 'Select-String -Pattern'
            "Bash" = 'grep -r'
            "CMD" = 'findstr /s'
        }
        "list-processes" = @{
            "PowerShell" = 'Get-Process'
            "Bash" = 'ps aux'
            "CMD" = 'tasklist'
        }
        "environment-vars" = @{
            "PowerShell" = 'Get-ChildItem Env:'
            "Bash" = 'env'
            "CMD" = 'set'
        }
        "current-directory" = @{
            "PowerShell" = 'Get-Location'
            "Bash" = 'pwd'
            "CMD" = 'cd'
        }
        "clear-screen" = @{
            "PowerShell" = 'Clear-Host'
            "Bash" = 'clear'
            "CMD" = 'cls'
        }
    }
    
    if ($commandMap.ContainsKey($BaseCommand)) {
        if ($commandMap[$BaseCommand].ContainsKey($Shell)) {
            return $commandMap[$BaseCommand][$Shell]
        }
    }
    
    return $null
}

function Test-CommandAvailability {
    param(
        [string]$Command
    )
    
    try {
        $null = Get-Command $Command -ErrorAction Stop
        return $true
    }
    catch {
        return $false
    }
}

function Write-VerboseInfo {
    param([string]$Message)
    
    if ($ShowDetails) {
        Write-Host "[INFO] $Message" -ForegroundColor Cyan
    }
}

# Main execution
$currentShell = Get-CurrentShell
Write-VerboseInfo "Detected shell: $currentShell"

if ($DetectOnly) {
    Write-Host "Current Shell: $currentShell"
    
    # Additional environment info
    Write-Host "=== Environment Details ==="
    Write-Host "OS: $([System.Environment]::OSVersion.Platform)"
    Write-Host "OS Version: $([System.Environment]::OSVersion.Version)"
    Write-Host "PowerShell Version: $($PSVersionTable.PSVersion)"
    Write-Host "Process ID: $PID"
    
    # Check for common tools
    Write-Host "`n=== Available Tools ==="
    $tools = @("git", "dotnet", "node", "npm", "cargo", "rustc", "python", "pip")
    foreach ($tool in $tools) {
        if (Test-CommandAvailability $tool) {
            Write-Host "[OK] $tool is available" -ForegroundColor Green
        }
        else {
            Write-Host "[NOT FOUND] $tool" -ForegroundColor Yellow
        }
    }
    
    exit 0
}

# If a command was provided, translate it
if ($Command) {
    $translatedCommand = Get-ShellSpecificCommand -BaseCommand $Command -Shell $currentShell
    
    if ($translatedCommand) {
        Write-Host "Shell-specific command for '$Command' in $currentShell :"
        Write-Host $translatedCommand -ForegroundColor Green
        
        if ($ShowDetails) {
            # Show alternatives for other shells
            Write-Host "`nAlternatives for other shells:"
            @("PowerShell", "Bash", "CMD") | Where-Object { $_ -ne $currentShell } | ForEach-Object {
                $altCommand = Get-ShellSpecificCommand -BaseCommand $Command -Shell $_
                if ($altCommand) {
                    Write-Host "$_ : $altCommand" -ForegroundColor DarkGray
                }
            }
        }
    }
    else {
        Write-Host "No shell-specific translation found for: $Command" -ForegroundColor Yellow
        Write-Host "The command might work as-is or may need manual translation."
    }
}
else {
    # Show usage information
    Write-Host "Shell Context Detector - Usage Guide"
    Write-Host "===================================="
    Write-Host ""
    Write-Host "Detect current shell:"
    Write-Host "  .\shell-context-detector.ps1 -DetectOnly"
    Write-Host ""
    Write-Host "Translate command for current shell:"
    Write-Host "  .\shell-context-detector.ps1 -Command 'list-aot'"
    Write-Host ""
    Write-Host "Available command translations:"
    Write-Host "  - list-aot         : Find AOT templates"
    Write-Host "  - find-files       : Search for files"
    Write-Host "  - grep-content     : Search file contents"
    Write-Host "  - list-processes   : Show running processes"
    Write-Host "  - environment-vars : Display environment variables"
    Write-Host "  - current-directory: Show current directory"
    Write-Host "  - clear-screen     : Clear terminal screen"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -ShowDetails      : Show additional information"
    Write-Host "  -DetectOnly       : Only detect shell, don't translate"
}