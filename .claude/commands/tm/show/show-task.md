---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management", "tasks"]
description: Command for show-task operations

# Agent and Tool Integration
assigned-agent: task-executor
required-tools:
  - "mcp__taskmaster-ai__get_task"
  - "mcp__FileScopeMCP__find_important_files"
  - "mcp__cipher-memory__search_nodes"
tool-chain: task-coordination
auto-deploy: true

# Workflow Configuration
pre-execution:
  validate-tools: true
  load-context: true
post-execution:
  store-results: false
  update-tasks: false
---

Show detailed task information with rich context and insights.

Arguments: $ARGUMENTS

## Enhanced Task Display

Parse arguments to determine what to show and how.

### 1. **Smart Task Selection**

Based on $ARGUMENTS:
- Number â†’ Show specific task with full context
- "current" â†’ Show active in-progress task(s)
- "next" â†’ Show recommended next task
- "blocked" â†’ Show all blocked tasks with reasons
- "critical" â†’ Show critical path tasks
- Multiple IDs â†’ Comparative view

### 2. **Contextual Information**

For each task, intelligently include:

**Core Details**
- Full task information (id, title, description, details)
- Current status with history
- Test strategy and acceptance criteria
- Priority and complexity analysis

**Relationships**
- Dependencies (what it needs)
- Dependents (what needs it)
- Parent/subtask hierarchy
- Related tasks (similar work)

**Time Intelligence**
- Created/updated timestamps
- Time in current status
- Estimated vs actual time
- Historical completion patterns

### 3. **Visual Enhancements**

```
ðŸ“‹ Task #45: Implement User Authentication
â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”
Status: ðŸŸ¡ in-progress (2 hours)
Priority: ðŸ”´ High | Complexity: 73/100

Dependencies: âœ… #41, âœ… #42, â³ #43 (blocked)
Blocks: #46, #47, #52

Progress: â–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–‘â–‘ 80% complete

Recent Activity:
- 2h ago: Status changed to in-progress
- 4h ago: Dependency #42 completed
- Yesterday: Task expanded with 3 subtasks
```

### 4. **Intelligent Insights**

Based on task analysis:
- **Risk Assessment**: Complexity vs time remaining
- **Bottleneck Analysis**: Is this blocking critical work?
- **Recommendation**: Suggested approach or concerns
- **Similar Tasks**: How others completed similar work

### 5. **Action Suggestions**

Context-aware next steps:
- If blocked â†’ Show how to unblock
- If complex â†’ Suggest expansion
- If in-progress â†’ Show completion checklist
- If done â†’ Show dependent tasks ready to start

### 6. **Multi-Task View**

When showing multiple tasks:
- Common dependencies
- Optimal completion order
- Parallel work opportunities
- Combined complexity analysis


