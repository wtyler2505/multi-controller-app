---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github", "issue-display", "information-presentation"]
description: Issue Show - Comprehensive issue information display and analysis

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["issue-display", "information-presentation", "data-visualization"]
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
      - query-pattern: "issue display + information presentation + data visualization"
      - search-depth: "comprehensive"
      - max-results: 10
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - display-history: "mcp__cipher-memory__search_nodes"
      - presentation-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    display-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - display-summary: "mcp__cipher-memory__create_entities"
      - presentation-patterns: "mcp__cipher-memory__create_entities"
      - visualization-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - command-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - display-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - presentation-insights: "mcp__cipher-memory__create_entities"

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
    display-tracking: true
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
  learning-domains: ["issue-display", "information-presentation", "data-visualization"]
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
    load-display-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-display-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

tool-chain: "information-presentation-data-visualization"
auto-deploy: true
parallel-execution: false
allowed-tools: ["Bash", "Read", "LS", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph"]
---

# Issue Show

Display issue and sub-issues with detailed information.

## Usage
```
/pm:issue-show <issue_number>
```

## Instructions

You are displaying comprehensive information about a GitHub issue and related sub-issues for: **Issue #$ARGUMENTS**

### 1. Fetch Issue Data
- Use `gh issue view #$ARGUMENTS` to get GitHub issue details
- Look for local task file: first check `.claude/epics/*/$ARGUMENTS.md` (new naming)
- If not found, search for file with `github:.*issues/$ARGUMENTS` in frontmatter (old naming)
- Check for related issues and sub-tasks

### 2. Issue Overview
Display issue header:
```
ðŸŽ« Issue #$ARGUMENTS: {Issue Title}
   Status: {open/closed}
   Labels: {labels}
   Assignee: {assignee}
   Created: {creation_date}
   Updated: {last_update}
   
ðŸ“ Description:
{issue_description}
```

### 3. Local File Mapping
If local task file exists:
```
ðŸ“ Local Files:
   Task file: .claude/epics/{epic_name}/{task_file}
   Updates: .claude/epics/{epic_name}/updates/$ARGUMENTS/
   Last local update: {timestamp}
```

### 4. Sub-Issues and Dependencies
Show related issues:
```
ðŸ”— Related Issues:
   Parent Epic: #{epic_issue_number}
   Dependencies: #{dep1}, #{dep2}
   Blocking: #{blocked1}, #{blocked2}
   Sub-tasks: #{sub1}, #{sub2}
```

### 5. Recent Activity
Display recent comments and updates:
```
ðŸ’¬ Recent Activity:
   {timestamp} - {author}: {comment_preview}
   {timestamp} - {author}: {comment_preview}
   
   View full thread: gh issue view #$ARGUMENTS --comments
```

### 6. Progress Tracking
If task file exists, show progress:
```
âœ… Acceptance Criteria:
   âœ… Criterion 1 (completed)
   ðŸ”„ Criterion 2 (in progress)
   â¸ï¸ Criterion 3 (blocked)
   â–¡ Criterion 4 (not started)
```

### 7. Quick Actions
```
ðŸš€ Quick Actions:
   Start work: /pm:issue-start $ARGUMENTS
   Sync updates: /pm:issue-sync $ARGUMENTS
   Add comment: gh issue comment #$ARGUMENTS --body "your comment"
   View in browser: gh issue view #$ARGUMENTS --web
```

### 8. Error Handling
- Handle invalid issue numbers gracefully
- Check for network/authentication issues
- Provide helpful error messages and alternatives

Provide comprehensive issue information to help developers understand context and current status for Issue #$ARGUMENTS.



