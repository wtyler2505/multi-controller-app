---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Command for epic-refresh operations
allowed-tools: Read, Write, LS, mcp__taskmaster-ai__get_tasks, mcp__desktop-commander__read_multiple_files
argument-hint: <epic_name> | --force-sync | --calculate-only

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-progress-tracking", "status-synchronization", "progress-calculation"]
    complexity-factors: ["multi-task-aggregation", "status-synchronization", "progress-metrics"]
    specialized-tools: ["epic-management", "progress-tracking", "status-synchronization"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["progress-tracking", "status-synchronization"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-progress-tracking + status-synchronization + progress-calculation"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "tracking-patterns + synchronization-knowledge + calculation-strategies"
    
    knowledge-preparation:
      - domain: "progress-tracking"
      - pattern-search: "tracking-strategies + synchronization-patterns + calculation-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["status-analysis", "progress-calculation", "synchronization-operations"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "tracking-strategies + synchronization-approaches + calculation-decisions"
      - pattern-recognition: "progress-tracking-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["tracking-results", "synchronization-insights", "calculation-techniques"]
      - knowledge-extraction: "tracking-methodologies + synchronization-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["tracking-relationships", "synchronization-dependencies", "calculation-connections"]
      - cross-reference: "related-tracking-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "tracking-knowledge + synchronization-patterns"
      - continuous-learning: "progress-tracking-optimization"

# Centralized Logging Integration
logging-integration:
  enabled: true
  log-file: ".claude/command-execution.jsonl"
  
  # Comprehensive Execution Logging
  log-level: "comprehensive"
  
  capture-points:
    - command-initiation
    - agent-selection-process
    - memory-operations
    - status-analysis
    - progress-calculation
    - synchronization-operations
    - metric-updates
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-refresh"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "tracking-results + synchronization-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["tracking-patterns", "synchronization-techniques", "calculation-strategies"]
  learn-from: ["epic-status", "task-tracking", "progress-management"]
  contribute-to: "progress-tracking-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-epic-state
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-status-analysis
    - continuous-memory-updates
    - real-time-synchronization-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - tracking-pattern-extraction
---

# Epic Refresh

Update epic progress based on task states.

## Usage
```
/pm:epic-refresh <epic_name>
```

## Instructions

### 1. Count Task Status

Scan all task files in `.claude/epics/$ARGUMENTS/`:
- Count total tasks
- Count tasks with `status: closed`
- Count tasks with `status: open`
- Count tasks with work in progress

### 2. Calculate Progress

```
progress = (closed_tasks / total_tasks) * 100
```

Round to nearest integer.

### 3. Update GitHub Task List

If epic has GitHub issue, sync task checkboxes:

```bash
# Get epic issue number from epic.md frontmatter
epic_issue={extract_from_github_field}

if [ ! -z "$epic_issue" ]; then
  # Get current epic body
  gh issue view $epic_issue --json body -q .body > /tmp/epic-body.md
  
  # For each task, check its status and update checkbox
  for task_file in .claude/epics/$ARGUMENTS/[0-9]*.md; do
    task_issue=$(grep 'github:' $task_file | grep -oE '[0-9]+$')
    task_status=$(grep 'status:' $task_file | cut -d: -f2 | tr -d ' ')
    
    if [ "$task_status" = "closed" ]; then
      # Mark as checked
      sed -i "s/- \[ \] #$task_issue/- [x] #$task_issue/" /tmp/epic-body.md
    else
      # Ensure unchecked (in case manually checked)
      sed -i "s/- \[x\] #$task_issue/- [ ] #$task_issue/" /tmp/epic-body.md
    fi
  done
  
  # Update epic issue
  gh issue edit $epic_issue --body-file /tmp/epic-body.md
fi
```

### 4. Determine Epic Status

- If progress = 0% and no work started: `backlog`
- If progress > 0% and < 100%: `in-progress`
- If progress = 100%: `completed`

### 5. Update Epic

Get current datetime: `date -u +"%Y-%m-%dT%H:%M:%SZ"`

Update epic.md frontmatter:
```yaml
status: {calculated_status}
progress: {calculated_progress}%
updated: {current_datetime}
```

### 6. Output

```
ðŸ”„ Epic refreshed: $ARGUMENTS

Tasks:
  Closed: {closed_count}
  Open: {open_count}
  Total: {total_count}
  
Progress: {old_progress}% â†’ {new_progress}%
Status: {old_status} â†’ {new_status}
GitHub: Task list updated âœ“

{If complete}: Run /pm:epic-close $ARGUMENTS to close epic
{If in progress}: Run /pm:next to see priority tasks
```

## Important Notes

This is useful after manual task edits or GitHub sync.
Don't modify task files, only epic status.
Preserve all other frontmatter fields.


