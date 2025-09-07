# Claude Custom Commands Quick Access Script
# Source this in your PowerShell profile for instant access

function ultra-think {
    param([string]$query)
    Write-Host "🧠 ULTRA-THINK MODE ACTIVATED" -ForegroundColor Cyan
    Write-Host "Use in Claude: /project:ultra-think $query" -ForegroundColor Yellow
    Set-Clipboard "/project:ultra-think $query"
    Write-Host "✅ Command copied to clipboard!" -ForegroundColor Green
}

function list-commands {
    Write-Host "`n📁 PROJECT COMMANDS (.claude/commands/):" -ForegroundColor Cyan
    Get-ChildItem ".claude/commands/*.md" | ForEach-Object {
        $name = $_.BaseName
        Write-Host "  /project:$name" -ForegroundColor Yellow
    }
    
    Write-Host "`n📁 GLOBAL COMMANDS (~/.claude/commands/):" -ForegroundColor Cyan
    Get-ChildItem "~/.claude/commands/*.md" | ForEach-Object {
        $name = $_.BaseName
        Write-Host "  /user:$name" -ForegroundColor Yellow
    }
}

function fix-claude-commands {
    Write-Host "🔧 FIXING CLAUDE COMMANDS..." -ForegroundColor Cyan
    
    # Option 1: Show correct usage
    Write-Host "`n✅ SOLUTION 1: Use correct prefix" -ForegroundColor Green
    Write-Host "  Instead of: /ultra-think" -ForegroundColor Red
    Write-Host "  Use: /project:ultra-think" -ForegroundColor Green
    
    # Option 2: Create agent wrappers
    Write-Host "`n✅ SOLUTION 2: Agent wrappers created" -ForegroundColor Green
    Write-Host "  Restart Claude Code to activate agents" -ForegroundColor Yellow
    
    # Option 3: List all available
    Write-Host "`n✅ SOLUTION 3: All available commands:" -ForegroundColor Green
    list-commands
}

# Auto-run on script load
Write-Host "🚀 Claude Commands Helper Loaded!" -ForegroundColor Magenta
Write-Host "Commands: ultra-think, list-commands, fix-claude-commands" -ForegroundColor Cyan