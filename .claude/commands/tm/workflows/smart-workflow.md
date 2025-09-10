---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management"]
description: Command for smart-workflow operations
---

Execute an intelligent workflow based on current project state and recent commands.

This command analyzes:
1. Recent commands you've run
2. Current project state
3. Time of day / day of week
4. Your working patterns

Arguments: $ARGUMENTS

## Intelligent Workflow Selection

Based on context, I'll determine the best workflow:

### Context Analysis
- Previous command executed
- Current task states
- Unfinished work from last session
- Your typical patterns

### Smart Execution

If last command was:
- `status` â†’ Likely starting work â†’ Run daily standup
- `complete` â†’ Task finished â†’ Find next task
- `list pending` â†’ Planning â†’ Suggest sprint planning
- `expand` â†’ Breaking down work â†’ Show complexity analysis
- `init` â†’ New project â†’ Show onboarding workflow

If no recent commands:
- Morning? â†’ Daily standup workflow
- Many pending tasks? â†’ Sprint planning
- Tasks blocked? â†’ Dependency resolution
- Friday? â†’ Weekly review

### Workflow Composition

I'll chain appropriate commands:
1. Analyze current state
2. Execute primary workflow
3. Suggest follow-up actions
4. Prepare environment for coding

### Learning Mode

This command learns from your patterns:
- Track command sequences
- Note time preferences
- Remember common workflows
- Adapt to your style

Example flows detected:
- Morning: standup â†’ next â†’ start
- After lunch: status â†’ continue task
- End of day: complete â†’ commit â†’ status


