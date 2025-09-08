---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management", "tasks"]
description: Command for remove-dependency operations
---

Remove a dependency between tasks.

Arguments: $ARGUMENTS

Parse the task IDs to remove dependency relationship.

## Removing Dependencies

Removes a dependency relationship, potentially unblocking tasks.

## Argument Parsing

Parse natural language or IDs:
- "remove dependency between 5 and 3"
- "5 no longer needs 3"
- "unblock 5 from 3"
- "5 3" â†’ remove dependency of 5 on 3

## Execution

```bash
task-master remove-dependency --id=<task-id> --depends-on=<dependency-id>
```

## Pre-Removal Checks

1. **Verify dependency exists**
2. **Check impact on task flow**
3. **Warn if it breaks logical sequence**
4. **Show what will be unblocked**

## Smart Analysis

Before removing:
- Show why dependency might have existed
- Check if removal makes tasks executable
- Verify no critical path disruption
- Suggest alternative dependencies

## Post-Removal

After removing:
1. Show updated task status
2. List newly unblocked tasks
3. Update project timeline
4. Suggest next actions

## Safety Features

- Confirm if removing critical dependency
- Show tasks that become immediately actionable
- Warn about potential issues
- Keep removal history

## Example

```
/project:tm/remove-dependency 5 from 3
â†’ Removed: Task #5 no longer depends on #3
â†’ Task #5 is now UNBLOCKED and ready to start
â†’ Warning: Consider if #5 still needs #2 completed first
```


