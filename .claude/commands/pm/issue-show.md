---
model: claude-sonnet-4-20250514
category: project-management
priority: medium
tags: ["project-management", "github"]
description: Issue Show
allowed-tools: Bash, Read, LS
---

# Issue Show

Display issue and sub-issues with detailed information.

## Usage
```
/pm:issue-show <issue_number>
```

## Instructions

You are displaying comprehensive information about a GitHub issue and related sub-issues for: **Issue #$ARGUMENTS**

### 1. Fetch Issue Data
- Use `gh issue view #$ARGUMENTS` to get GitHub issue details
- Look for local task file: first check `.claude/epics/*/$ARGUMENTS.md` (new naming)
- If not found, search for file with `github:.*issues/$ARGUMENTS` in frontmatter (old naming)
- Check for related issues and sub-tasks

### 2. Issue Overview
Display issue header:
```
ðŸŽ« Issue #$ARGUMENTS: {Issue Title}
   Status: {open/closed}
   Labels: {labels}
   Assignee: {assignee}
   Created: {creation_date}
   Updated: {last_update}
   
ðŸ“ Description:
{issue_description}
```

### 3. Local File Mapping
If local task file exists:
```
ðŸ“ Local Files:
   Task file: .claude/epics/{epic_name}/{task_file}
   Updates: .claude/epics/{epic_name}/updates/$ARGUMENTS/
   Last local update: {timestamp}
```

### 4. Sub-Issues and Dependencies
Show related issues:
```
ðŸ”— Related Issues:
   Parent Epic: #{epic_issue_number}
   Dependencies: #{dep1}, #{dep2}
   Blocking: #{blocked1}, #{blocked2}
   Sub-tasks: #{sub1}, #{sub2}
```

### 5. Recent Activity
Display recent comments and updates:
```
ðŸ’¬ Recent Activity:
   {timestamp} - {author}: {comment_preview}
   {timestamp} - {author}: {comment_preview}
   
   View full thread: gh issue view #$ARGUMENTS --comments
```

### 6. Progress Tracking
If task file exists, show progress:
```
âœ… Acceptance Criteria:
   âœ… Criterion 1 (completed)
   ðŸ”„ Criterion 2 (in progress)
   â¸ï¸ Criterion 3 (blocked)
   â–¡ Criterion 4 (not started)
```

### 7. Quick Actions
```
ðŸš€ Quick Actions:
   Start work: /pm:issue-start $ARGUMENTS
   Sync updates: /pm:issue-sync $ARGUMENTS
   Add comment: gh issue comment #$ARGUMENTS --body "your comment"
   View in browser: gh issue view #$ARGUMENTS --web
```

### 8. Error Handling
- Handle invalid issue numbers gracefully
- Check for network/authentication issues
- Provide helpful error messages and alternatives

Provide comprehensive issue information to help developers understand context and current status for Issue #$ARGUMENTS.



