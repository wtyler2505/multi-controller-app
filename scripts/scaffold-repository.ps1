# Repository Scaffolding Script for Multi-Controller App
# ASCII-only PowerShell script - no emojis or special Unicode characters
# Ensures idempotency and Windows filesystem compatibility

param(
    [Parameter()]
    [string]$ProjectRoot = (Get-Location).Path,
    
    [Parameter()]
    [switch]$ShowDetails,
    
    [Parameter()]
    [switch]$DryRun
)

# Configuration
$RequiredDirectories = @(
    "app",
    "drivers",
    "transports", 
    "docs",
    "docs/decisions",
    "docs/architecture",
    "docs/commands",
    "tests",
    "tests/unit",
    "tests/integration",
    "tests/e2e",
    "scripts",
    "scripts/git-automation",
    "profiles",
    "ref",
    ".claude",
    ".claude/commands",
    ".taskmaster",
    ".taskmaster/tasks",
    ".taskmaster/docs",
    ".gitmeta",
    ".gitmeta/config",
    ".gitmeta/templates"
)

$RequiredFiles = @{
    ".gitignore" = @"
# Node
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*
lerna-debug.log*
.npm
*.tsbuildinfo

# Build outputs
dist/
build/
out/
*.exe
*.dll
*.pdb
*.ilk
obj/
bin/

# IDE
.vscode/
.idea/
*.swp
*.swo
*~
.DS_Store
Thumbs.db

# Environment
.env
.env.local
.env.*.local

# Logs
logs/
*.log

# Testing
coverage/
.nyc_output/

# Temporary
temp/
tmp/
*.tmp

# OS
Desktop.ini
$RECYCLE.BIN/

# Project specific
*.local.json
settings.local.json
"@

    ".mcp.json" = @"
{
  `"mcpServers`": {
    `"task-master-ai`": {
      `"command`": `"npx`",
      `"args`": [`"-y`", `"--package=task-master-ai`", `"task-master-ai`"],
      `"env`": {
        `"ANTHROPIC_API_KEY`": `"`${ANTHROPIC_API_KEY}`",
        `"PERPLEXITY_API_KEY`": `"`${PERPLEXITY_API_KEY}`"
      }
    },
    `"desktop-commander`": {
      `"command`": `"npx`",
      `"args`": [`"-y`", `"@wonderwhy-er/desktop-commander`"],
      `"env`": {}
    },
    `"clear-thought`": {
      `"command`": `"npx`",
      `"args`": [`"-y`", `"@chirag127/clear-thought-mcp-server`"],
      `"env`": {}
    },
    `"context7`": {
      `"command`": `"npx`",
      `"args`": [`"-y`", `"@upstash/context7-mcp`"],
      `"env`": {
        `"CONTEXT7_API_KEY`": `"`${CONTEXT7_API_KEY}`"
      }
    }
  }
}
"@

    ".env.example" = @"
# API Keys (Required)
ANTHROPIC_API_KEY=your_key_here
PERPLEXITY_API_KEY=your_key_here

# Optional API Keys
OPENAI_API_KEY=your_key_here
GOOGLE_API_KEY=your_key_here
MISTRAL_API_KEY=your_key_here
CONTEXT7_API_KEY=your_key_here

# Application Settings
NODE_ENV=development
LOG_LEVEL=info
PORT=3000

# Hardware Settings
DEFAULT_BAUD_RATE=115200
SERIAL_TIMEOUT_MS=5000
TCP_TIMEOUT_MS=10000
"@

    ".editorconfig" = @"
# EditorConfig is awesome: https://EditorConfig.org

# top-most EditorConfig file
root = true

# Unix-style newlines with a newline ending every file
[*]
end_of_line = lf
insert_final_newline = true
charset = utf-8
indent_style = space
indent_size = 2
trim_trailing_whitespace = true

# Markdown files
[*.md]
trim_trailing_whitespace = false

# C# files
[*.cs]
indent_size = 4

# PowerShell files
[*.ps1]
end_of_line = crlf
indent_size = 4

# Batch files
[*.{cmd,bat}]
end_of_line = crlf
indent_size = 2
"@

    ".claude/settings.json" = @"
{
  `"allowedTools`": [
    `"Edit`",
    `"Read`",
    `"Write`",
    `"MultiEdit`",
    `"Bash(npm *)`",
    `"Bash(git *)`",
    `"Bash(dotnet *)`",
    `"Bash(cargo *)`",
    `"Bash(task-master *)`",
    `"mcp__*`"
  ],
  `"customInstructions`": `"Follow CLAUDE.md instructions strictly. Use Task Master for all task management.`"
}
"@

    "tests/README.md" = @"
# Test Directory Structure

## Organization

- **unit/** - Unit tests for individual components
- **integration/** - Integration tests for component interactions  
- **e2e/** - End-to-end tests for complete workflows

## Running Tests

\`\`\`bash
# Run all tests
npm test

# Run unit tests only
npm run test:unit

# Run integration tests
npm run test:integration

# Run e2e tests
npm run test:e2e

# Run with coverage
npm run test:coverage
\`\`\`

## Test Requirements

- All code must have corresponding tests
- Maintain minimum 80% coverage
- Tests must run in under 5 minutes
- Use descriptive test names
"@
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

function Test-DirectoryStructure {
    param([string]$Path)
    
    $missing = @()
    
    foreach ($dir in $RequiredDirectories) {
        $fullPath = Join-Path $Path $dir
        if (-not (Test-Path $fullPath)) {
            $missing += $dir
        }
    }
    
    return $missing
}

function Create-DirectoryStructure {
    param(
        [string]$Path,
        [string[]]$Directories
    )
    
    $created = 0
    
    foreach ($dir in $Directories) {
        $fullPath = Join-Path $Path $dir
        
        if ($DryRun) {
            Write-Status "Would create: $dir" "INFO"
        }
        else {
            try {
                if (-not (Test-Path $fullPath)) {
                    New-Item -ItemType Directory -Path $fullPath -Force | Out-Null
                    Write-Status "Created directory: $dir" "SUCCESS"
                    $created++
                }
                else {
                    Write-Status "Directory exists: $dir" "INFO"
                }
            }
            catch {
                Write-Status "Failed to create directory: $dir - $_" "ERROR"
            }
        }
    }
    
    return $created
}

function Create-RequiredFiles {
    param([string]$Path)
    
    $created = 0
    
    foreach ($file in $RequiredFiles.Keys) {
        $fullPath = Join-Path $Path $file
        
        if ($DryRun) {
            if (-not (Test-Path $fullPath)) {
                Write-Status "Would create: $file" "INFO"
            }
        }
        else {
            try {
                if (-not (Test-Path $fullPath)) {
                    # Ensure parent directory exists
                    $parent = Split-Path $fullPath -Parent
                    if (-not (Test-Path $parent)) {
                        New-Item -ItemType Directory -Path $parent -Force | Out-Null
                    }
                    
                    # Create file with content
                    $RequiredFiles[$file] | Set-Content -Path $fullPath -Encoding UTF8
                    Write-Status "Created file: $file" "SUCCESS"
                    $created++
                }
                else {
                    Write-Status "File exists: $file" "INFO"
                }
            }
            catch {
                Write-Status "Failed to create file: $file - $_" "ERROR"
            }
        }
    }
    
    return $created
}

function Test-WindowsCompatibility {
    param([string]$Path)
    
    $issues = @()
    
    # Check path length
    if ($Path.Length -gt 200) {
        $issues += "Base path is very long (>200 chars), may cause issues with deep nesting"
    }
    
    # Check for reserved names
    $reservedNames = @("CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4",
                      "COM5", "COM6", "COM7", "COM8", "COM9", "LPT1", "LPT2",
                      "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9")
    
    foreach ($dir in $RequiredDirectories) {
        $parts = $dir -split '[\\/]'
        foreach ($part in $parts) {
            if ($reservedNames -contains $part.ToUpper()) {
                $issues += "Directory name '$part' is a Windows reserved name"
            }
            if ($part -match '[<>:"|?*]') {
                $issues += "Directory name '$part' contains invalid Windows characters"
            }
        }
    }
    
    return $issues
}

function Show-Summary {
    param(
        [int]$DirsCreated,
        [int]$FilesCreated,
        [string[]]$Issues
    )
    
    Write-Host ""
    Write-Host "===== Repository Scaffolding Summary =====" -ForegroundColor Cyan
    Write-Host "Project Root: $ProjectRoot"
    Write-Host "Directories Created: $DirsCreated"
    Write-Host "Files Created: $FilesCreated"
    
    if ($Issues.Count -gt 0) {
        Write-Host ""
        Write-Host "Compatibility Issues Found:" -ForegroundColor Yellow
        foreach ($issue in $Issues) {
            Write-Host "  - $issue" -ForegroundColor Yellow
        }
    }
    else {
        Write-Host "No compatibility issues found" -ForegroundColor Green
    }
    
    Write-Host "==========================================" -ForegroundColor Cyan
}

# Main execution
Write-Host "Multi-Controller App Repository Scaffolding Script" -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host ""

# Validate project root
if (-not (Test-Path $ProjectRoot)) {
    Write-Status "Project root does not exist: $ProjectRoot" "ERROR"
    exit 1
}

Write-Status "Project Root: $ProjectRoot" "INFO"

if ($DryRun) {
    Write-Status "Running in DRY RUN mode - no changes will be made" "WARNING"
}

# Check Windows compatibility
Write-Status "Checking Windows filesystem compatibility..." "INFO"
$compatibilityIssues = Test-WindowsCompatibility -Path $ProjectRoot

if ($compatibilityIssues.Count -gt 0) {
    foreach ($issue in $compatibilityIssues) {
        Write-Status $issue "WARNING"
    }
}

# Check existing structure
Write-Status "Checking existing directory structure..." "INFO"
$missingDirs = Test-DirectoryStructure -Path $ProjectRoot

if ($missingDirs.Count -eq 0) {
    Write-Status "All required directories exist" "SUCCESS"
}
else {
    Write-Status "Missing directories: $($missingDirs.Count)" "WARNING"
}

# Create missing directories
$dirsCreated = 0
if ($missingDirs.Count -gt 0) {
    Write-Status "Creating missing directories..." "INFO"
    $dirsCreated = Create-DirectoryStructure -Path $ProjectRoot -Directories $missingDirs
}

# Create required files
Write-Status "Checking and creating required files..." "INFO"
$filesCreated = Create-RequiredFiles -Path $ProjectRoot

# Verify final structure
if (-not $DryRun) {
    Write-Status "Verifying final structure..." "INFO"
    $stillMissing = Test-DirectoryStructure -Path $ProjectRoot
    
    if ($stillMissing.Count -eq 0) {
        Write-Status "Repository structure is complete" "SUCCESS"
    }
    else {
        Write-Status "Some directories are still missing" "ERROR"
        foreach ($dir in $stillMissing) {
            Write-Status "  - $dir" "ERROR"
        }
    }
}

# Show summary
Show-Summary -DirsCreated $dirsCreated -FilesCreated $filesCreated -Issues $compatibilityIssues

# Exit code
if ($stillMissing.Count -eq 0 -or $DryRun) {
    exit 0
}
else {
    exit 1
}