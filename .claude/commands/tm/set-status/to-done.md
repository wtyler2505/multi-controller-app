---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management"]
description: Command for to-done operations

# Agent and Tool Integration
assigned-agent: task-checker
required-tools:
  - "mcp__taskmaster-ai__set_task_status"
  - "mcp__taskmaster-ai__get_task"
  - "mcp__cipher-memory__create_entities"
tool-chain: task-coordination
auto-deploy: true

# Workflow Configuration
pre-execution:
  validate-tools: true
  load-context: true
post-execution:
  store-results: true
  update-tasks: true
  generate-report: true
---

Mark a task as completed.

Arguments: $ARGUMENTS (task ID)

## Completing a Task

This command validates task completion and updates project state intelligently.

## Pre-Completion Checks

1. Verify test strategy was followed
2. Check if all subtasks are complete
3. Validate acceptance criteria met
4. Ensure code is committed

## Execution

```bash
task-master set-status --id=$ARGUMENTS --status=done
```

## Post-Completion Actions

1. **Update Dependencies**
   - Identify newly unblocked tasks
   - Update sprint progress
   - Recalculate project timeline

2. **Documentation**
   - Generate completion summary
   - Update CLAUDE.md with learnings
   - Log implementation approach

3. **Next Steps**
   - Show newly available tasks
   - Suggest logical next task
   - Update velocity metrics

## Celebration & Learning

- Show impact of completion
- Display unblocked work
- Recognize achievement
- Capture lessons learned


