# PowerShell Script Template - Multi-Controller App
# ASCII-only - NO emojis or Unicode special characters
# Follow this template to avoid common parameter conflicts

<#
.SYNOPSIS
    Brief description of what the script does

.DESCRIPTION
    Detailed description of the script's functionality

.PARAMETER ProjectRoot
    The root directory of the project

.PARAMETER ShowDetails
    Shows detailed output (alternative to -Verbose)

.PARAMETER DryRun
    Runs the script without making actual changes

.EXAMPLE
    .\script-name.ps1 -ShowDetails
    Runs the script with detailed output

.NOTES
    IMPORTANT: Never use these reserved parameter names:
    - Verbose (use ShowDetails, DetailedOutput, or ExtendedInfo instead)
    - Debug (use DebugMode or ShowDebug instead)
    - ErrorAction, WarningAction, ErrorVariable, WarningVariable
    - InformationAction, InformationVariable
    - WhatIf, Confirm (these are OK with SupportsShouldProcess)
#>

[CmdletBinding()]
param(
    [Parameter()]
    [string]$ProjectRoot = (Get-Location).Path,
    
    # NEVER use -Verbose as parameter name
    # Use one of these alternatives instead:
    [Parameter()]
    [switch]$ShowDetails,      # Preferred alternative to -Verbose
    # [switch]$DetailedOutput,  # Alternative option
    # [switch]$ExtendedInfo,    # Another alternative
    
    [Parameter()]
    [switch]$DryRun,           # OK - not a reserved parameter
    
    [Parameter()]
    [switch]$Force             # OK - not a reserved parameter
)

# If you need verbose output, use the built-in $VerbosePreference
if ($ShowDetails) {
    $VerbosePreference = 'Continue'
}

# Functions
function Write-Status {
    param(
        [string]$Message,
        [string]$Level = "INFO"
    )
    
    $color = switch ($Level) {
        "SUCCESS" { "Green" }
        "WARNING" { "Yellow" }
        "ERROR" { "Red" }
        "INFO" { "Cyan" }
        default { "White" }
    }
    
    if ($Level -eq "ERROR") {
        Write-Host "[$Level] $Message" -ForegroundColor $color
    }
    elseif ($ShowDetails -or $Level -ne "INFO") {
        Write-Host "[$Level] $Message" -ForegroundColor $color
    }
}

# Use Write-Verbose for detailed output (works with built-in -Verbose)
Write-Verbose "Starting script execution..."
Write-Verbose "Project Root: $ProjectRoot"

# Main script logic
try {
    Write-Status "Script starting..." "INFO"
    
    if ($DryRun) {
        Write-Status "Running in DRY RUN mode - no changes will be made" "WARNING"
    }
    
    # Your script logic here
    
    Write-Status "Script completed successfully" "SUCCESS"
    exit 0
}
catch {
    Write-Status "Script failed: $_" "ERROR"
    exit 1
}

# REMEMBER:
# - Always use ASCII-only characters (no emojis)
# - Test scripts with: powershell -ExecutionPolicy Bypass -File script.ps1
# - The built-in -Verbose parameter will work automatically with Write-Verbose
# - Use $ShowDetails for your custom verbose logic