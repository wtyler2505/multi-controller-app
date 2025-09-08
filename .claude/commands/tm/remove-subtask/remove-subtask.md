---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management", "tasks"]
description: Command for remove-subtask operations
---

Remove a subtask from its parent task.

Arguments: $ARGUMENTS

Parse subtask ID to remove, with option to convert to standalone task.

## Removing Subtasks

Remove a subtask and optionally convert it back to a standalone task.

## Argument Parsing

- "remove subtask 5.1"
- "delete 5.1"
- "convert 5.1 to task" â†’ remove and convert
- "5.1 standalone" â†’ convert to standalone

## Execution Options

### 1. Delete Subtask
```bash
task-master remove-subtask --id=<parentId.subtaskId>
```

### 2. Convert to Standalone
```bash
task-master remove-subtask --id=<parentId.subtaskId> --convert
```

## Pre-Removal Checks

1. **Validate Subtask**
   - Verify subtask exists
   - Check completion status
   - Review dependencies

2. **Impact Analysis**
   - Other subtasks that depend on it
   - Parent task implications
   - Data that will be lost

## Removal Process

### For Deletion:
1. Confirm if subtask has work done
2. Update parent task estimates
3. Remove subtask and its data
4. Clean up dependencies

### For Conversion:
1. Assign new standalone task ID
2. Preserve all task data
3. Update dependency references
4. Maintain task history

## Smart Features

- Warn if subtask is in-progress
- Show impact on parent task
- Preserve important data
- Update related estimates

## Example Flows

```
/project:tm/remove-subtask 5.1
â†’ Warning: Subtask #5.1 is in-progress
â†’ This will delete all subtask data
â†’ Parent task #5 will be updated
Confirm deletion? (y/n)

/project:tm/remove-subtask 5.1 convert
â†’ Converting subtask #5.1 to standalone task #89
â†’ Preserved: All task data and history
â†’ Updated: 2 dependency references
â†’ New task #89 is now independent
```

## Post-Removal

- Update parent task status
- Recalculate estimates
- Show updated hierarchy
- Suggest next actions


