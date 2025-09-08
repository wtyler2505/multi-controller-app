---
model: claude-sonnet-4-20250514
category: project-management
priority: medium
tags: ["project-management", "github"]
description: Issue Status
allowed-tools: Bash, Read, LS
---

# Issue Status

Check issue status (open/closed) and current state.

## Usage
```
/pm:issue-status <issue_number>
```

## Instructions

You are checking the current status of a GitHub issue and providing a quick status report for: **Issue #$ARGUMENTS**

### 1. Fetch Issue Status
Use GitHub CLI to get current status:
```bash
gh issue view #$ARGUMENTS --json state,title,labels,assignees,updatedAt
```

### 2. Status Display
Show concise status information:
```
ðŸŽ« Issue #$ARGUMENTS: {Title}
   
ðŸ“Š Status: {OPEN/CLOSED}
   Last update: {timestamp}
   Assignee: {assignee or "Unassigned"}
   
ðŸ·ï¸ Labels: {label1}, {label2}, {label3}
```

### 3. Epic Context
If issue is part of an epic:
```
ðŸ“š Epic Context:
   Epic: {epic_name}
   Epic progress: {completed_tasks}/{total_tasks} tasks complete
   This task: {task_position} of {total_tasks}
```

### 4. Local Sync Status
Check if local files are in sync:
```
ðŸ’¾ Local Sync:
   Local file: {exists/missing}
   Last local update: {timestamp}
   Sync status: {in_sync/needs_sync/local_ahead/remote_ahead}
```

### 5. Quick Status Indicators
Use clear visual indicators:
- ðŸŸ¢ Open and ready
- ðŸŸ¡ Open with blockers  
- ðŸ”´ Open and overdue
- âœ… Closed and complete
- âŒ Closed without completion

### 6. Actionable Next Steps
Based on status, suggest actions:
```
ðŸš€ Suggested Actions:
   - Start work: /pm:issue-start $ARGUMENTS
   - Sync updates: /pm:issue-sync $ARGUMENTS
   - Close issue: gh issue close #$ARGUMENTS
   - Reopen issue: gh issue reopen #$ARGUMENTS
```

### 7. Batch Status
If checking multiple issues, support comma-separated list:
```
/pm:issue-status 123,124,125
```

Keep the output concise but informative, perfect for quick status checks during development of Issue #$ARGUMENTS.



