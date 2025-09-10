---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management", "tasks"]
description: Command for expand-task operations

# Agent and Tool Integration
assigned-agent: task-orchestrator
required-tools:
  - "mcp__taskmaster-ai__expand_task"
  - "mcp__taskmaster-ai__analyze_project_complexity"
  - "mcp__clear-thought__sequentialthinking"
tool-chain: task-coordination
auto-deploy: true
parallel-execution: true

# Workflow Configuration
pre-execution:
  validate-tools: true
  load-context: true
post-execution:
  store-results: true
  update-tasks: true
---

Break down a complex task into subtasks.

Arguments: $ARGUMENTS (task ID)

## Intelligent Task Expansion

Analyzes a task and creates detailed subtasks for better manageability.

## Execution

```bash
task-master expand --id=$ARGUMENTS
```

## Expansion Process

1. **Task Analysis**
   - Review task complexity
   - Identify components
   - Detect technical challenges
   - Estimate time requirements

2. **Subtask Generation**
   - Create 3-7 subtasks typically
   - Each subtask 1-4 hours
   - Logical implementation order
   - Clear acceptance criteria

3. **Smart Breakdown**
   - Setup/configuration tasks
   - Core implementation
   - Testing components
   - Integration steps
   - Documentation updates

## Enhanced Features

Based on task type:
- **Feature**: Setup â†’ Implement â†’ Test â†’ Integrate
- **Bug Fix**: Reproduce â†’ Diagnose â†’ Fix â†’ Verify
- **Refactor**: Analyze â†’ Plan â†’ Refactor â†’ Validate

## Post-Expansion

After expansion:
1. Show subtask hierarchy
2. Update time estimates
3. Suggest implementation order
4. Highlight critical path


