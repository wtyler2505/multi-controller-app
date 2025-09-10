---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "bidirectional-sync", "data-integration", "state-synchronization"]
description: Comprehensive Project Sync - Bidirectional synchronization with conflict resolution and state management

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["bidirectional-sync", "data-integration", "conflict-resolution"]
  complexity-level: "complex"
  selection-criteria:
    keyword-match: 0.90
    argument-analysis: 0.95
    project-context: 0.85
  preferred-agents: ["general-purpose"]
  fallback-agents: ["task-orchestrator"]
  confidence-threshold: 0.80

# Universal Cipher Memory Integration (MANDATORY)
cipher-memory-integration:
  enabled: true
  priority: "high"
  pre-execution-memory:
    context-search:
      - query-pattern: "bidirectional sync + data integration + conflict resolution"
      - search-depth: "comprehensive"
      - max-results: 15
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - sync-history: "mcp__cipher-memory__search_nodes"
      - integration-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    sync-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - sync-summary: "mcp__cipher-memory__create_entities"
      - integration-patterns: "mcp__cipher-memory__create_entities"
      - conflict-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - command-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - sync-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - sync-insights: "mcp__cipher-memory__create_entities"

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
    sync-tracking: true
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
  learning-domains: ["bidirectional-sync", "data-integration", "conflict-resolution"]
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
    load-sync-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-sync-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

tool-chain: "bidirectional-sync-data-integration"
auto-deploy: true
parallel-execution: false
allowed-tools: ["Bash", "Read", "Write", "LS", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph"]
---

# Sync

Full bidirectional sync between local and GitHub.

## Usage
```
/pm:sync [epic_name]
```

If epic_name provided, sync only that epic. Otherwise sync all.

## Instructions

### 1. Pull from GitHub

Get current state of all issues:
```bash
# Get all epic and task issues
gh issue list --label "epic" --limit 1000 --json number,title,state,body,labels,updatedAt
gh issue list --label "task" --limit 1000 --json number,title,state,body,labels,updatedAt
```

### 2. Update Local from GitHub

For each GitHub issue:
- Find corresponding local file by issue number
- Compare states:
  - If GitHub state newer (updatedAt > local updated), update local
  - If GitHub closed but local open, close local
  - If GitHub reopened but local closed, reopen local
- Update frontmatter to match GitHub state

### 3. Push Local to GitHub

For each local task/epic:
- If has GitHub URL but GitHub issue not found, it was deleted - mark local as archived
- If no GitHub URL, create new issue (like epic-sync)
- If local updated > GitHub updatedAt, push changes:
  ```bash
  gh issue edit {number} --body-file {local_file}
  ```

### 4. Handle Conflicts

If both changed (local and GitHub updated since last sync):
- Show both versions
- Ask user: "Local and GitHub both changed. Keep: (local/github/merge)?"
- Apply user's choice

### 5. Update Sync Timestamps

Update all synced files with last_sync timestamp.

### 6. Output

```
ðŸ”„ Sync Complete

Pulled from GitHub:
  Updated: {count} files
  Closed: {count} issues
  
Pushed to GitHub:
  Updated: {count} issues
  Created: {count} new issues
  
Conflicts resolved: {count}

Status:
  âœ… All files synced
  {or list any sync failures}
```

## Important Notes

Follow `/rules/github-operations.md` for GitHub commands.
Follow `/rules/frontmatter-operations.md` for local updates.
Always backup before sync in case of issues.


