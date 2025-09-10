---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github", "status-monitoring", "progress-visualization"]
description: Issue Status - Comprehensive status monitoring and progress visualization

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["status-monitoring", "progress-visualization", "data-presentation"]
  complexity-level: "medium"
  selection-criteria:
    keyword-match: 0.85
    argument-analysis: 0.90
    project-context: 0.80
  preferred-agents: ["general-purpose"]
  fallback-agents: ["task-orchestrator"]
  confidence-threshold: 0.75

# Universal Cipher Memory Integration (MANDATORY)
cipher-memory-integration:
  enabled: true
  priority: "high"
  pre-execution-memory:
    context-search:
      - query-pattern: "status monitoring + progress visualization + data presentation"
      - search-depth: "comprehensive"
      - max-results: 10
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - monitoring-history: "mcp__cipher-memory__search_nodes"
      - visualization-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    monitoring-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - monitoring-summary: "mcp__cipher-memory__create_entities"
      - visualization-patterns: "mcp__cipher-memory__create_entities"
      - status-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - command-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - monitoring-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - monitoring-insights: "mcp__cipher-memory__create_entities"

# Universal Centralized Logging Integration (MANDATORY)
centralized-logging:
  enabled: true
  log-file: ".claude/execution-log.jsonl"
  log-components:
    execution-metadata: true
    agent-selection: true
    tool-chain: true
    memory-operations: true
    performance-metrics: true
    success-indicators: true
    monitoring-tracking: true
  logging-phases:
    pre-execution: true
    during-execution: true
    post-execution: true
    error-handling: true
  processing:
    real-time-write: true
    batch-processing: false
    error-recovery: true
    compression: false

# Cross-Command Learning Integration (MANDATORY)
cross-command-learning:
  enabled: true
  learning-domains: ["status-monitoring", "progress-visualization", "data-presentation"]
  pattern-sharing:
    success-patterns: "mcp__cipher-memory__create_entities"
    failure-patterns: "mcp__cipher-memory__create_entities"
    optimization-opportunities: "mcp__cipher-memory__add_observations"
  knowledge-synthesis:
    cross-domain-insights: "mcp__cipher-memory__create_relations"
    usage-pattern-analysis: "internal"
    performance-optimization: "internal"

# Workflow Integration (MANDATORY)
workflow-integration:
  enabled: true
  pre-execution:
    memory-context-loading: true
    cipher-search-patterns: true
    load-monitoring-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-monitoring-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

tool-chain: "status-monitoring-progress-visualization"
auto-deploy: true
parallel-execution: false
allowed-tools: ["Bash", "Read", "LS", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph"]
---

# Issue Status

Check issue status (open/closed) and current state.

## Usage
```
/pm:issue-status <issue_number>
```

## Instructions

You are checking the current status of a GitHub issue and providing a quick status report for: **Issue #$ARGUMENTS**

### 1. Fetch Issue Status
Use GitHub CLI to get current status:
```bash
gh issue view #$ARGUMENTS --json state,title,labels,assignees,updatedAt
```

### 2. Status Display
Show concise status information:
```
ðŸŽ« Issue #$ARGUMENTS: {Title}
   
ðŸ“Š Status: {OPEN/CLOSED}
   Last update: {timestamp}
   Assignee: {assignee or "Unassigned"}
   
ðŸ·ï¸ Labels: {label1}, {label2}, {label3}
```

### 3. Epic Context
If issue is part of an epic:
```
ðŸ“š Epic Context:
   Epic: {epic_name}
   Epic progress: {completed_tasks}/{total_tasks} tasks complete
   This task: {task_position} of {total_tasks}
```

### 4. Local Sync Status
Check if local files are in sync:
```
ðŸ’¾ Local Sync:
   Local file: {exists/missing}
   Last local update: {timestamp}
   Sync status: {in_sync/needs_sync/local_ahead/remote_ahead}
```

### 5. Quick Status Indicators
Use clear visual indicators:
- ðŸŸ¢ Open and ready
- ðŸŸ¡ Open with blockers  
- ðŸ”´ Open and overdue
- âœ… Closed and complete
- âŒ Closed without completion

### 6. Actionable Next Steps
Based on status, suggest actions:
```
ðŸš€ Suggested Actions:
   - Start work: /pm:issue-start $ARGUMENTS
   - Sync updates: /pm:issue-sync $ARGUMENTS
   - Close issue: gh issue close #$ARGUMENTS
   - Reopen issue: gh issue reopen #$ARGUMENTS
```

### 7. Batch Status
If checking multiple issues, support comma-separated list:
```
/pm:issue-status 123,124,125
```

Keep the output concise but informative, perfect for quick status checks during development of Issue #$ARGUMENTS.



