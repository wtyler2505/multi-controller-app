# Git Sync Status Dashboard for Multi-Controller App
# Shows comprehensive synchronization status between local and remote

param(
    [switch]$AutoFix = $false,
    [switch]$Detailed = $false,
    [switch]$Watch = $false,
    [int]$WatchInterval = 5
)

$ErrorActionPreference = "Continue"

# ANSI color codes
$colors = @{
    Reset = "`e[0m"
    Red = "`e[31m"
    Green = "`e[32m"
    Yellow = "`e[33m"
    Blue = "`e[34m"
    Magenta = "`e[35m"
    Cyan = "`e[36m"
    White = "`e[37m"
    BoldRed = "`e[1;31m"
    BoldGreen = "`e[1;32m"
    BoldYellow = "`e[1;33m"
    BoldBlue = "`e[1;34m"
}

function Write-ColorLine($text, $color = "White") {
    Write-Host "$($colors[$color])$text$($colors['Reset'])"
}

function Get-GitStatus {
    $status = @{
        Branch = ""
        Remote = ""
        Ahead = 0
        Behind = 0
        Staged = @()
        Modified = @()
        Untracked = @()
        Conflicts = @()
        Stashes = 0
        LastFetch = $null
        RemoteUrl = ""
        IsDetached = $false
    }
    
    # Get current branch
    try {
        $status.Branch = git symbolic-ref --short HEAD 2>$null
        if (!$status.Branch) {
            $status.IsDetached = $true
            $status.Branch = git rev-parse --short HEAD 2>$null
        }
    } catch {
        $status.Branch = "unknown"
    }
    
    # Get remote tracking branch
    if (!$status.IsDetached) {
        $status.Remote = git rev-parse --abbrev-ref --symbolic-full-name "@{u}" 2>$null
    }
    
    # Get ahead/behind counts
    if ($status.Remote) {
        $counts = git rev-list --left-right --count "$($status.Remote)...HEAD" 2>$null
        if ($counts) {
            $parts = $counts -split '\s+'
            $status.Behind = [int]$parts[0]
            $status.Ahead = [int]$parts[1]
        }
    }
    
    # Get file status
    $gitStatus = git status --porcelain 2>$null
    foreach ($line in $gitStatus) {
        if ($line) {
            $code = $line.Substring(0, 2)
            $file = $line.Substring(3)
            
            switch -Regex ($code) {
                '^(M.|.M|MM)' { $status.Modified += $file }
                '^(A.|AM)' { $status.Staged += $file }
                '^\?\?' { $status.Untracked += $file }
                '^(DD|AU|UD|UA|DU|AA|UU)' { $status.Conflicts += $file }
            }
        }
    }
    
    # Get stash count
    $stashes = git stash list 2>$null
    if ($stashes) {
        $status.Stashes = ($stashes | Measure-Object).Count
    }
    
    # Get last fetch time
    $fetchHead = ".git/FETCH_HEAD"
    if (Test-Path $fetchHead) {
        $status.LastFetch = (Get-Item $fetchHead).LastWriteTime
    }
    
    # Get remote URL
    $status.RemoteUrl = git config --get remote.origin.url 2>$null
    
    return $status
}

function Show-Dashboard {
    param($status)
    
    Clear-Host
    
    # Header
    Write-ColorLine "========================================" "Blue"
    Write-ColorLine "     Git Sync Status Dashboard" "BoldBlue"
    Write-ColorLine "========================================" "Blue"
    Write-Host ""
    
    # Branch info
    Write-Host -NoNewline "Branch: "
    Write-ColorLine $status.Branch "BoldCyan"
    
    if ($status.Remote) {
        Write-Host -NoNewline "Tracking: "
        Write-ColorLine $status.Remote "Cyan"
    } elseif (!$status.IsDetached) {
        Write-ColorLine "Not tracking any remote branch" "Yellow"
    }
    
    if ($status.RemoteUrl) {
        Write-Host -NoNewline "Remote: "
        Write-ColorLine $status.RemoteUrl "Cyan"
    }
    
    Write-Host ""
    
    # Sync status
    Write-ColorLine "Synchronization Status:" "BoldWhite"
    Write-ColorLine "------------------------" "White"
    
    if ($status.Ahead -eq 0 -and $status.Behind -eq 0) {
        Write-ColorLine "[OK] Up to date with remote" "Green"
    } else {
        if ($status.Ahead -gt 0) {
            Write-ColorLine "[!] $($status.Ahead) commit(s) ahead of remote" "Yellow"
            if ($Detailed) {
                $aheadCommits = git log --oneline "$($status.Remote)..HEAD" 2>$null
                foreach ($commit in $aheadCommits) {
                    Write-Host "     $commit"
                }
            }
        }
        
        if ($status.Behind -gt 0) {
            Write-ColorLine "[!] $($status.Behind) commit(s) behind remote" "Yellow"
            if ($Detailed) {
                $behindCommits = git log --oneline "HEAD..$($status.Remote)" 2>$null
                foreach ($commit in $behindCommits) {
                    Write-Host "     $commit"
                }
            }
        }
    }
    
    Write-Host ""
    
    # Working directory status
    Write-ColorLine "Working Directory:" "BoldWhite"
    Write-ColorLine "------------------" "White"
    
    $hasChanges = $false
    
    if ($status.Staged.Count -gt 0) {
        Write-ColorLine "[+] $($status.Staged.Count) file(s) staged" "Green"
        $hasChanges = $true
        if ($Detailed) {
            foreach ($file in $status.Staged) {
                Write-Host "     + $file" -ForegroundColor Green
            }
        }
    }
    
    if ($status.Modified.Count -gt 0) {
        Write-ColorLine "[*] $($status.Modified.Count) file(s) modified" "Yellow"
        $hasChanges = $true
        if ($Detailed) {
            foreach ($file in $status.Modified) {
                Write-Host "     * $file" -ForegroundColor Yellow
            }
        }
    }
    
    if ($status.Untracked.Count -gt 0) {
        Write-ColorLine "[?] $($status.Untracked.Count) untracked file(s)" "Magenta"
        $hasChanges = $true
        if ($Detailed) {
            foreach ($file in $status.Untracked) {
                Write-Host "     ? $file" -ForegroundColor Magenta
            }
        }
    }
    
    if ($status.Conflicts.Count -gt 0) {
        Write-ColorLine "[X] $($status.Conflicts.Count) conflict(s)" "BoldRed"
        $hasChanges = $true
        if ($Detailed) {
            foreach ($file in $status.Conflicts) {
                Write-Host "     X $file" -ForegroundColor Red
            }
        }
    }
    
    if (!$hasChanges) {
        Write-ColorLine "[OK] Working directory clean" "Green"
    }
    
    if ($status.Stashes -gt 0) {
        Write-Host ""
        Write-ColorLine "[i] $($status.Stashes) stash(es) saved" "Cyan"
    }
    
    Write-Host ""
    
    # Last fetch info
    if ($status.LastFetch) {
        $timeSinceFetch = (Get-Date) - $status.LastFetch
        $fetchAge = ""
        
        if ($timeSinceFetch.TotalMinutes -lt 60) {
            $fetchAge = "$([int]$timeSinceFetch.TotalMinutes) minutes ago"
        } elseif ($timeSinceFetch.TotalHours -lt 24) {
            $fetchAge = "$([int]$timeSinceFetch.TotalHours) hours ago"
        } else {
            $fetchAge = "$([int]$timeSinceFetch.TotalDays) days ago"
        }
        
        Write-Host -NoNewline "Last fetch: "
        
        if ($timeSinceFetch.TotalHours -gt 24) {
            Write-ColorLine $fetchAge "Yellow"
        } else {
            Write-ColorLine $fetchAge "Green"
        }
    } else {
        Write-ColorLine "Last fetch: Never" "Yellow"
    }
    
    Write-Host ""
    
    # Recommended actions
    Write-ColorLine "Recommended Actions:" "BoldWhite"
    Write-ColorLine "--------------------" "White"
    
    $actions = @()
    
    if ($status.Behind -gt 0) {
        $actions += "git pull --rebase"
    }
    
    if ($status.Ahead -gt 0) {
        $actions += "git push"
    }
    
    if ($status.Modified.Count -gt 0) {
        $actions += "git add <files> && git commit"
    }
    
    if ($status.Conflicts.Count -gt 0) {
        $actions += "Resolve conflicts, then git add <files>"
    }
    
    if ($status.Stashes -gt 0 -and $status.Modified.Count -eq 0) {
        $actions += "git stash pop (to restore stashed changes)"
    }
    
    if (!$status.Remote -and !$status.IsDetached) {
        $actions += "git push -u origin $($status.Branch)"
    }
    
    if ($actions.Count -eq 0) {
        Write-ColorLine "[OK] No actions needed - repository in sync!" "Green"
    } else {
        foreach ($action in $actions) {
            Write-Host "  * $action"
        }
        
        if ($AutoFix) {
            Write-Host ""
            Write-ColorLine "Auto-fix mode enabled. Executing safe operations..." "Yellow"
            
            # Fetch updates
            if ($status.Behind -gt 0 -or (Get-Date) - $status.LastFetch -gt [TimeSpan]::FromHours(1)) {
                Write-Host "Fetching updates..."
                git fetch 2>&1 | Out-Null
            }
            
            # Pull if behind and no local changes
            if ($status.Behind -gt 0 -and $status.Modified.Count -eq 0) {
                Write-Host "Pulling changes..."
                git pull --rebase 2>&1 | Out-Null
            }
            
            # Push if ahead
            if ($status.Ahead -gt 0) {
                Write-Host "Pushing changes..."
                git push 2>&1 | Out-Null
            }
            
            Write-ColorLine "Auto-fix complete!" "Green"
        }
    }
    
    Write-Host ""
    Write-ColorLine "========================================" "Blue"
    
    # Footer with commands
    if (!$Watch) {
        Write-Host ""
        Write-Host "Commands:" -ForegroundColor Cyan
        Write-Host "  -Detailed    Show file details"
        Write-Host "  -AutoFix     Auto-execute safe sync operations"
        Write-Host "  -Watch       Live monitoring mode"
        Write-Host ""
    } else {
        Write-Host "Watching... (Ctrl+C to stop)" -ForegroundColor Yellow
    }
}

# Main execution
if ($Watch) {
    while ($true) {
        $status = Get-GitStatus
        Show-Dashboard -status $status
        Start-Sleep -Seconds $WatchInterval
    }
} else {
    $status = Get-GitStatus
    Show-Dashboard -status $status
}