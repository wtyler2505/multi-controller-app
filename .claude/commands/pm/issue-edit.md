---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github", "issue-editing", "content-management"]
description: Issue Edit - Interactive issue modification with bidirectional sync

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["issue-editing", "content-management", "interactive-workflow"]
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
      - query-pattern: "issue edit + content management + interactive workflow"
      - search-depth: "comprehensive"
      - max-results: 10
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - editing-history: "mcp__cipher-memory__search_nodes"
      - workflow-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    interaction-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - editing-summary: "mcp__cipher-memory__create_entities"
      - workflow-patterns: "mcp__cipher-memory__create_entities"
      - interaction-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - command-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - editing-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - editing-insights: "mcp__cipher-memory__create_entities"

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
    interaction-tracking: true
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
  learning-domains: ["issue-editing", "content-management", "interactive-workflows"]
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
    load-editing-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-editing-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

tool-chain: "interactive-editing-content-management"
auto-deploy: true
parallel-execution: false
allowed-tools: ["Bash", "Read", "Write", "LS", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph"]
---

# Issue Edit

Edit issue details locally and on GitHub.

## Usage
```
/pm:issue-edit <issue_number>
```

## Instructions

### 1. Get Current Issue State

```bash
# Get from GitHub
gh issue view $ARGUMENTS --json title,body,labels

# Find local task file
# Search for file with github:.*issues/$ARGUMENTS
```

### 2. Interactive Edit

Ask user what to edit:
- Title
- Description/Body
- Labels
- Acceptance criteria (local only)
- Priority/Size (local only)

### 3. Update Local File

Get current datetime: `date -u +"%Y-%m-%dT%H:%M:%SZ"`

Update task file with changes:
- Update frontmatter `name` if title changed
- Update body content if description changed
- Update `updated` field with current datetime

### 4. Update GitHub

If title changed:
```bash
gh issue edit $ARGUMENTS --title "{new_title}"
```

If body changed:
```bash
gh issue edit $ARGUMENTS --body-file {updated_task_file}
```

If labels changed:
```bash
gh issue edit $ARGUMENTS --add-label "{new_labels}"
gh issue edit $ARGUMENTS --remove-label "{removed_labels}"
```

### 5. Output

```
âœ… Updated issue #$ARGUMENTS
  Changes:
    {list_of_changes_made}
  
Synced to GitHub: âœ…
```

## Important Notes

Always update local first, then GitHub.
Preserve frontmatter fields not being edited.
Follow `/rules/frontmatter-operations.md`.


