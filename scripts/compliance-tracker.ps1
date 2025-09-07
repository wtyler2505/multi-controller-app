# Compliance Tracker for Pre-Task Research Enforcement
# Monitors and reports on research compliance across the project

param(
    [Parameter()]
    [switch]$ShowDetails,
    
    [Parameter()]
    [switch]$GenerateReport,
    
    [Parameter()]
    [string]$Since = (Get-Date).AddDays(-7).ToString("yyyy-MM-dd"),
    
    [Parameter()]
    [string]$OutputFile = ".taskmaster/reports/compliance-report.md"
)

$ErrorActionPreference = "Stop"

# Color configuration
$colors = @{
    Success = "Green"
    Warning = "Yellow"
    Error = "Red"
    Info = "Cyan"
    Header = "Blue"
}

# Paths
$logFile = Join-Path $PSScriptRoot "..\\.taskmaster\\compliance.log"
$memoryDir = Join-Path $PSScriptRoot "..\\.cipher\\memory"
$tasksFile = Join-Path $PSScriptRoot "..\\.taskmaster\\tasks\\tasks.json"

# Initialize stats
$stats = @{
    TotalTasks = 0
    TasksWithResearch = 0
    TasksWithoutResearch = 0
    ComplianceRate = 0
    RecentViolations = @()
    TopPatterns = @()
}

Write-Host "`n===========================================" -ForegroundColor $colors.Header
Write-Host "    RESEARCH COMPLIANCE TRACKER" -ForegroundColor $colors.Header
Write-Host "===========================================" -ForegroundColor $colors.Header

# Function to check if task has research
function Test-TaskResearch {
    param([string]$TaskId)
    
    # Check Cipher memory for research
    $searchQuery = "task $TaskId research implementation approaches"
    $memorySearch = Get-ChildItem -Path $memoryDir -Filter "*.json" -ErrorAction SilentlyContinue |
        Select-String -Pattern $TaskId -ErrorAction SilentlyContinue
    
    # Check git commits for research mentions
    $gitLog = git log --grep="research.*$TaskId" --oneline 2>$null
    
    # Check if PRD or research docs exist
    $researchDoc = Get-ChildItem -Path ".taskmaster/docs" -Filter "*task-$TaskId*" -ErrorAction SilentlyContinue
    
    return ($memorySearch -or $gitLog -or $researchDoc)
}

# Load tasks
if (Test-Path $tasksFile) {
    $tasksData = Get-Content $tasksFile -Raw | ConvertFrom-Json
    $allTasks = @()
    
    # Flatten tasks and subtasks
    foreach ($task in $tasksData.tasks) {
        $allTasks += $task
        if ($task.subtasks) {
            foreach ($subtask in $task.subtasks) {
                $subtask | Add-Member -NotePropertyName "ParentId" -NotePropertyValue $task.id -Force
                $allTasks += $subtask
            }
        }
    }
    
    $stats.TotalTasks = $allTasks.Count
}

# Check research compliance for each task
Write-Host "`n[Checking Tasks]" -ForegroundColor $colors.Info

foreach ($task in $allTasks) {
    $hasResearch = Test-TaskResearch -TaskId $task.id
    
    if ($hasResearch) {
        $stats.TasksWithResearch++
        if ($ShowDetails) {
            Write-Host "  [OK] Task $($task.id): $($task.title)" -ForegroundColor $colors.Success
        }
    } else {
        $stats.TasksWithoutResearch++
        if ($task.status -in @("in-progress", "done")) {
            $stats.RecentViolations += @{
                TaskId = $task.id
                Title = $task.title
                Status = $task.status
            }
            Write-Host "  [VIOLATION] Task $($task.id): $($task.title) [$($task.status)]" -ForegroundColor $colors.Error
        } elseif ($ShowDetails) {
            Write-Host "  [PENDING] Task $($task.id): $($task.title)" -ForegroundColor $colors.Warning
        }
    }
}

# Calculate compliance rate
if ($stats.TotalTasks -gt 0) {
    $stats.ComplianceRate = [math]::Round(($stats.TasksWithResearch / $stats.TotalTasks) * 100, 2)
}

# Analyze patterns from git history
Write-Host "`n[Analyzing Patterns]" -ForegroundColor $colors.Info

$gitPatterns = git log --since=$Since --grep="research" --oneline 2>$null | Measure-Object
$memoryPatterns = Get-ChildItem -Path $memoryDir -Filter "*.json" -ErrorAction SilentlyContinue |
    Where-Object { $_.LastWriteTime -gt (Get-Date $Since) } |
    Measure-Object

# Display summary
Write-Host "`n===========================================" -ForegroundColor $colors.Header
Write-Host "             COMPLIANCE SUMMARY" -ForegroundColor $colors.Header
Write-Host "===========================================" -ForegroundColor $colors.Header

Write-Host "`nOverall Statistics:" -ForegroundColor $colors.Info
Write-Host "  Total Tasks: $($stats.TotalTasks)"
Write-Host "  With Research: $($stats.TasksWithResearch)" -ForegroundColor $colors.Success
Write-Host "  Without Research: $($stats.TasksWithoutResearch)" -ForegroundColor $(if ($stats.TasksWithoutResearch -gt 0) { $colors.Warning } else { $colors.Success })
Write-Host "  Compliance Rate: $($stats.ComplianceRate)%" -ForegroundColor $(if ($stats.ComplianceRate -ge 80) { $colors.Success } elseif ($stats.ComplianceRate -ge 60) { $colors.Warning } else { $colors.Error })

if ($stats.RecentViolations.Count -gt 0) {
    Write-Host "`nRecent Violations (In-Progress/Done without research):" -ForegroundColor $colors.Error
    foreach ($violation in $stats.RecentViolations) {
        Write-Host "  - Task $($violation.TaskId): $($violation.Title) [$($violation.Status)]" -ForegroundColor $colors.Error
    }
}

Write-Host "`nActivity Since $Since`:" -ForegroundColor $colors.Info
Write-Host "  Research Commits: $($gitPatterns.Count)"
Write-Host "  Memory Entries: $($memoryPatterns.Count)"

# Enforcement status
Write-Host "`nEnforcement Mechanisms:" -ForegroundColor $colors.Info

$preCommitHook = Test-Path ".git/hooks/pre-commit"
$vsCodeTasks = Test-Path ".vscode/tasks.json"
$wrapperScript = Test-Path "scripts/task-master-wrapper.ps1"
$packageScripts = Select-String -Path "package.json" -Pattern "research:validate" -ErrorAction SilentlyContinue

Write-Host "  Git Pre-commit Hook: $(if ($preCommitHook) { '[ACTIVE]' } else { '[MISSING]' })" -ForegroundColor $(if ($preCommitHook) { $colors.Success } else { $colors.Error })
Write-Host "  VS Code Tasks: $(if ($vsCodeTasks) { '[CONFIGURED]' } else { '[MISSING]' })" -ForegroundColor $(if ($vsCodeTasks) { $colors.Success } else { $colors.Error })
Write-Host "  TaskMaster Wrapper: $(if ($wrapperScript) { '[INSTALLED]' } else { '[MISSING]' })" -ForegroundColor $(if ($wrapperScript) { $colors.Success } else { $colors.Error })
Write-Host "  Build-time Checks: $(if ($packageScripts) { '[CONFIGURED]' } else { '[MISSING]' })" -ForegroundColor $(if ($packageScripts) { $colors.Success } else { $colors.Error })

# Generate report if requested
if ($GenerateReport) {
    Write-Host "`n[Generating Report]" -ForegroundColor $colors.Info
    
    $report = @"
# Research Compliance Report
Generated: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")

## Summary
- **Total Tasks**: $($stats.TotalTasks)
- **Tasks with Research**: $($stats.TasksWithResearch)
- **Tasks without Research**: $($stats.TasksWithoutResearch)
- **Compliance Rate**: $($stats.ComplianceRate)%

## Violations
$(if ($stats.RecentViolations.Count -gt 0) {
    $stats.RecentViolations | ForEach-Object {
        "- Task $($_.TaskId): $($_.Title) [$($_.Status)]"
    } | Out-String
} else {
    "No violations found."
})

## Enforcement Status
- Git Pre-commit Hook: $(if ($preCommitHook) { '✅ Active' } else { '❌ Missing' })
- VS Code Tasks: $(if ($vsCodeTasks) { '✅ Configured' } else { '❌ Missing' })
- TaskMaster Wrapper: $(if ($wrapperScript) { '✅ Installed' } else { '❌ Missing' })
- Build-time Checks: $(if ($packageScripts) { '✅ Configured' } else { '❌ Missing' })

## Recommendations
$(if ($stats.ComplianceRate -lt 80) {
    "1. Increase research coverage for pending tasks
2. Run `task-master research` before implementation
3. Use `npm run research:enforce` to block non-compliant work"
} else {
    "Research compliance is good. Continue current practices."
})
"@
    
    # Ensure directory exists
    $reportDir = Split-Path $OutputFile -Parent
    if (!(Test-Path $reportDir)) {
        New-Item -ItemType Directory -Path $reportDir -Force | Out-Null
    }
    
    $report | Out-File -FilePath $OutputFile -Encoding utf8
    Write-Host "  Report saved to: $OutputFile" -ForegroundColor $colors.Success
}

# Exit code based on compliance
if ($stats.ComplianceRate -lt 60) {
    Write-Host "`n[WARNING] Low compliance rate detected!" -ForegroundColor $colors.Error
    exit 1
} else {
    Write-Host "`n[OK] Research compliance check passed" -ForegroundColor $colors.Success
    exit 0
}