---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management", "tasks"]
description: Command for update-task operations
---

Update tasks with intelligent field detection and bulk operations.

Arguments: $ARGUMENTS

## Intelligent Task Updates

Parse arguments to determine update intent and execute smartly.

### 1. **Natural Language Processing**

Understand update requests like:
- "mark 23 as done" â†’ Update status to done
- "increase priority of 45" â†’ Set priority to high
- "add dependency on 12 to task 34" â†’ Add dependency
- "tasks 20-25 need review" â†’ Bulk status update
- "all API tasks high priority" â†’ Pattern-based update

### 2. **Smart Field Detection**

Automatically detect what to update:
- Status keywords: done, complete, start, pause, review
- Priority changes: urgent, high, low, deprioritize
- Dependency updates: depends on, blocks, after
- Assignment: assign to, owner, responsible
- Time: estimate, spent, deadline

### 3. **Bulk Operations**

Support for multiple task updates:
```
Examples:
- "complete tasks 12, 15, 18"
- "all pending auth tasks to in-progress"
- "increase priority for tasks blocking 45"
- "defer all documentation tasks"
```

### 4. **Contextual Validation**

Before updating, check:
- Status transitions are valid
- Dependencies don't create cycles
- Priority changes make sense
- Bulk updates won't break project flow

Show preview:
```
Update Preview:
â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
Tasks to update: #23, #24, #25
Change: status â†’ in-progress
Impact: Will unblock tasks #30, #31
Warning: Task #24 has unmet dependencies
```

### 5. **Smart Suggestions**

Based on update:
- Completing task? â†’ Show newly unblocked tasks
- Changing priority? â†’ Show impact on sprint
- Adding dependency? â†’ Check for conflicts
- Bulk update? â†’ Show summary of changes

### 6. **Workflow Integration**

After updates:
- Auto-update dependent task states
- Trigger status recalculation
- Update sprint/milestone progress
- Log changes with context

Result: Flexible, intelligent task updates with safety checks.


