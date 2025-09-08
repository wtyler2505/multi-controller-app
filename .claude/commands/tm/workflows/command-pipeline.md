---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management"]
description: Command for command-pipeline operations
---

Execute a pipeline of commands based on a specification.

Arguments: $ARGUMENTS

## Command Pipeline Execution

Parse pipeline specification from arguments. Supported formats:

### Simple Pipeline
`init â†’ expand-all â†’ sprint-plan`

### Conditional Pipeline  
`status â†’ if:pending>10 â†’ sprint-plan â†’ else â†’ next`

### Iterative Pipeline
`for:pending-tasks â†’ expand â†’ complexity-check`

### Smart Pipeline Patterns

**1. Project Setup Pipeline**
```
init [prd] â†’ 
expand-all â†’ 
complexity-report â†’ 
sprint-plan â†’ 
show first-sprint
```

**2. Daily Work Pipeline**
```
standup â†’
if:in-progress â†’ continue â†’
else â†’ next â†’ start
```

**3. Task Completion Pipeline**
```
complete [id] â†’
git-commit â†’
if:blocked-tasks-freed â†’ show-freed â†’
next
```

**4. Quality Check Pipeline**
```
list in-progress â†’
for:each â†’ check-idle-time â†’
if:idle>1day â†’ prompt-update
```

### Pipeline Features

**Variables**
- Store results: `status â†’ $count=pending-count`
- Use in conditions: `if:$count>10`
- Pass between commands: `expand $high-priority-tasks`

**Error Handling**
- On failure: `try:complete â†’ catch:show-blockers`
- Skip on error: `optional:test-run`
- Retry logic: `retry:3:commit`

**Parallel Execution**
- Parallel branches: `[analyze | test | lint]`
- Join results: `parallel â†’ join:report`

### Execution Flow

1. Parse pipeline specification
2. Validate command sequence
3. Execute with state passing
4. Handle conditions and loops
5. Aggregate results
6. Show summary

This enables complex workflows like:
`parse-prd â†’ expand-all â†’ filter:complex>70 â†’ assign:senior â†’ sprint-plan:weighted`


