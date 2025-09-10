# Smart Search Templates for Cipher Memory
# Context-aware search patterns for different development scenarios

param(
    [Parameter(Position=0)]
    [string]$Query = "",
    
    [Parameter(Mandatory=$false)]
    [ValidateSet("auto", "task", "debug", "implement", "review", "performance")]
    [string]$Context = "auto",
    
    [Parameter(Mandatory=$false)]
    [int]$MaxResults = 5,
    
    [Parameter(Mandatory=$false)]
    [switch]$Detailed
)

$ProjectRoot = Split-Path -Parent $PSScriptRoot

function Get-CurrentContext {
    # Auto-detect context based on current state
    
    # Check if in git repository
    $GitBranch = git branch --show-current 2>$null
    
    # Check current directory
    $CurrentPath = Get-Location
    $RelativePath = $CurrentPath.Path.Replace($ProjectRoot, "").TrimStart("\")
    
    # Check for error state (recent failed command)
    $LastExitCode = $LASTEXITCODE
    
    # Determine context
    if ($LastExitCode -ne 0) {
        return "debug"
    }
    elseif ($RelativePath -like "tests*") {
        return "test"
    }
    elseif ($RelativePath -like "drivers*" -or $RelativePath -like "transports*") {
        return "implement"
    }
    elseif ($GitBranch -like "*review*" -or $GitBranch -like "*pr-*") {
        return "review"
    }
    else {
        return "task"
    }
}

function Build-SearchQuery {
    param(
        [string]$BaseQuery,
        [string]$SearchContext
    )
    
    $Queries = @()
    
    switch ($SearchContext) {
        "task" {
            # Starting a task - need context and examples
            $TaskId = Get-CurrentTaskId
            if ($TaskId) {
                $Queries += "task $TaskId"
            }
            $Queries += "$BaseQuery implementation"
            $Queries += "$BaseQuery pattern"
            $Queries += "$BaseQuery example"
            
            # Get component from current directory
            $Component = Get-ComponentFromPath
            if ($Component) {
                $Queries += "$Component $BaseQuery"
            }
        }
        
        "debug" {
            # Debugging - need error patterns and solutions
            $Queries += "error $BaseQuery"
            $Queries += "$BaseQuery troubleshooting"
            $Queries += "$BaseQuery fix"
            $Queries += "symptom $BaseQuery"
            $Queries += "$BaseQuery resolved"
            
            # Check for specific error patterns
            if ($BaseQuery -match "timeout") {
                $Queries += "50ms serial timeout"
                $Queries += "100ms network timeout"
            }
            if ($BaseQuery -match "memory") {
                $Queries += "memory leak"
                $Queries += "ring buffer overflow"
            }
        }
        
        "implement" {
            # Implementation - need patterns and interfaces
            $Queries += "$BaseQuery pattern"
            $Queries += "interface $BaseQuery"
            $Queries += "$BaseQuery example"
            $Queries += "async $BaseQuery"
            
            # Add Rust-specific patterns
            $Queries += "tokio $BaseQuery"
            $Queries += "trait $BaseQuery"
            $Queries += "impl $BaseQuery"
        }
        
        "review" {
            # Code review - need decisions and standards
            $Queries += "decision $BaseQuery"
            $Queries += "$BaseQuery best practice"
            $Queries += "$BaseQuery architecture"
            $Queries += "safety $BaseQuery"
            $Queries += "$BaseQuery standard"
        }
        
        "performance" {
            # Performance optimization - need metrics and baselines
            $Queries += "performance $BaseQuery"
            $Queries += "$BaseQuery optimization"
            $Queries += "$BaseQuery benchmark"
            $Queries += "metric $BaseQuery"
            $Queries += "$BaseQuery bottleneck"
            
            # Add specific performance targets
            $Queries += "2s startup"
            $Queries += "150MB RAM"
            $Queries += "2% CPU"
        }
        
        default {
            # Generic search
            $Queries += $BaseQuery
        }
    }
    
    return $Queries
}

function Get-CurrentTaskId {
    # Try to get task ID from various sources
    
    # Check environment variable
    if ($env:CURRENT_TASK_ID) {
        return $env:CURRENT_TASK_ID
    }
    
    # Check git branch
    $Branch = git branch --show-current 2>$null
    if ($Branch -match "task-(\d+\.?\d*)") {
        return $matches[1]
    }
    
    # Check recent commits
    $RecentCommit = git log -1 --pretty=%B 2>$null
    if ($RecentCommit -match "task (\d+\.?\d*)") {
        return $matches[1]
    }
    
    return $null
}

function Get-ComponentFromPath {
    $CurrentPath = (Get-Location).Path
    $RelativePath = $CurrentPath.Replace($ProjectRoot, "").TrimStart("\")
    
    if ($RelativePath -match "^(drivers|transports|apps)\\([^\\]+)") {
        return $matches[2]
    }
    
    return $null
}

function Search-CipherKnowledgeGraph {
    param([string[]]$Queries)
    
    Write-Host "Searching Knowledge Graph..." -ForegroundColor Cyan
    
    $Results = @()
    foreach ($Q in $Queries) {
        Write-Verbose "  Query: $Q"
        
        # This would call actual Cipher MCP in production
        # For now, simulate the search
        $Results += @{
            Query = $Q
            Type = "KnowledgeGraph"
            Matches = @()
        }
    }
    
    return $Results
}

function Search-CipherEmbeddings {
    param([string[]]$Queries)
    
    Write-Host "Searching with Embeddings..." -ForegroundColor Cyan
    
    $Results = @()
    foreach ($Q in $Queries) {
        Write-Verbose "  Query: $Q"
        
        # This would call actual Cipher MCP in production
        $Results += @{
            Query = $Q
            Type = "Embeddings"
            Matches = @()
            Similarity = 0.0
        }
    }
    
    return $Results
}

function Search-CipherReflection {
    param([string[]]$Queries)
    
    Write-Host "Searching Reflection Memory..." -ForegroundColor Cyan
    
    $Results = @()
    foreach ($Q in $Queries) {
        Write-Verbose "  Query: $Q"
        
        # This would call actual Cipher MCP in production
        $Results += @{
            Query = $Q
            Type = "Reflection"
            Matches = @()
        }
    }
    
    return $Results
}

function Format-SearchResults {
    param($GraphResults, $EmbeddingResults, $ReflectionResults)
    
    Write-Host "`n======================================" -ForegroundColor Green
    Write-Host "         SEARCH RESULTS" -ForegroundColor Green
    Write-Host "======================================" -ForegroundColor Green
    
    # Knowledge Graph Results
    if ($GraphResults.Count -gt 0) {
        Write-Host "`n[Knowledge Graph Entities]" -ForegroundColor Yellow
        foreach ($Result in $GraphResults | Select-Object -First $MaxResults) {
            Write-Host "  * $($Result.Query)" -ForegroundColor White
            if ($Detailed -and $Result.Matches) {
                foreach ($Match in $Result.Matches) {
                    Write-Host "    - $Match" -ForegroundColor Gray
                }
            }
        }
    }
    
    # Embedding Results
    if ($EmbeddingResults.Count -gt 0) {
        Write-Host "`n[Semantic Search Results]" -ForegroundColor Yellow
        foreach ($Result in $EmbeddingResults | Select-Object -First $MaxResults) {
            Write-Host "  * $($Result.Query) (Similarity: $($Result.Similarity))" -ForegroundColor White
            if ($Detailed -and $Result.Matches) {
                foreach ($Match in $Result.Matches) {
                    Write-Host "    - $Match" -ForegroundColor Gray
                }
            }
        }
    }
    
    # Reflection Results
    if ($ReflectionResults.Count -gt 0) {
        Write-Host "`n[Reasoning Patterns]" -ForegroundColor Yellow
        foreach ($Result in $ReflectionResults | Select-Object -First $MaxResults) {
            Write-Host "  * $($Result.Query)" -ForegroundColor White
            if ($Detailed -and $Result.Matches) {
                foreach ($Match in $Result.Matches) {
                    Write-Host "    - $Match" -ForegroundColor Gray
                }
            }
        }
    }
    
    Write-Host "`n======================================" -ForegroundColor Green
}

# Main Execution
if ($Context -eq "auto") {
    $Context = Get-CurrentContext
    Write-Host "Auto-detected context: $Context" -ForegroundColor DarkGray
}

if ([string]::IsNullOrWhiteSpace($Query)) {
    Write-Host "Cipher Memory Search" -ForegroundColor Cyan
    Write-Host "===================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\Search-CipherMemory.ps1 [-Query] <search> [-Context <type>] [-MaxResults <n>] [-Detailed]"
    Write-Host ""
    Write-Host "Contexts:"
    Write-Host "  auto       - Auto-detect based on current state (default)"
    Write-Host "  task       - Starting a new task"
    Write-Host "  debug      - Troubleshooting an error"
    Write-Host "  implement  - Implementing a feature"
    Write-Host "  review     - Code review"
    Write-Host "  performance - Performance optimization"
    Write-Host ""
    Write-Host "Examples:"
    Write-Host '  .\Search-CipherMemory.ps1 "serial timeout"'
    Write-Host '  .\Search-CipherMemory.ps1 "ring buffer" -Context implement'
    Write-Host '  .\Search-CipherMemory.ps1 "emergency stop" -Context review -Detailed'
    exit 0
}

Write-Host "Searching for: '$Query'" -ForegroundColor White
Write-Host "Context: $Context" -ForegroundColor White
Write-Host ""

# Build context-aware queries
$SearchQueries = Build-SearchQuery -BaseQuery $Query -SearchContext $Context

Write-Host "Generated search patterns:" -ForegroundColor DarkGray
foreach ($Q in $SearchQueries | Select-Object -First 3) {
    Write-Host "  - $Q" -ForegroundColor DarkGray
}

# Execute parallel searches
$GraphResults = Search-CipherKnowledgeGraph -Queries $SearchQueries
$EmbeddingResults = Search-CipherEmbeddings -Queries $SearchQueries
$ReflectionResults = Search-CipherReflection -Queries $SearchQueries

# Format and display results
Format-SearchResults -GraphResults $GraphResults -EmbeddingResults $EmbeddingResults -ReflectionResults $ReflectionResults

# Log search for learning
$SearchLog = Join-Path $ProjectRoot ".cipher\search-history.log"
$LogEntry = "$(Get-Date -Format 'o') | Query: $Query | Context: $Context | Results: $($GraphResults.Count + $EmbeddingResults.Count + $ReflectionResults.Count)"
$LogEntry | Add-Content -Path $SearchLog

Write-Host "`n[Search logged for continuous improvement]" -ForegroundColor DarkGray