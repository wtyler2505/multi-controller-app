---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github", "integration", "bidirectional-sync"]
description: Issue Sync - Bidirectional synchronization with incremental updates

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["bidirectional-sync", "data-synchronization", "incremental-updates"]
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
      - query-pattern: "bidirectional sync + data synchronization + incremental updates"
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
      - sync-metrics: "mcp__cipher-memory__add_observations"
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
  learning-domains: ["bidirectional-sync", "data-synchronization", "incremental-updates"]
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

*Task completed: 100% | Synced at {timestamp}*
```

### 9. Output Summary
```
â˜ï¸ Synced updates to GitHub Issue #$ARGUMENTS

ðŸ“ Update summary:
   Progress items: {progress_count}
   Technical notes: {notes_count}
   Commits referenced: {commit_count}

ðŸ“Š Current status:
   Task completion: {task_completion}%
   Epic progress: {epic_progress}%
   Completed criteria: {completed}/{total}

ðŸ”— View update: gh issue view #$ARGUMENTS --comments
```

### 10. Frontmatter Maintenance
- Always update task file frontmatter with current timestamp
- Track completion percentages in progress files
- Update epic progress when tasks complete
- Maintain sync timestamps for audit trail

### 11. Incremental Sync Detection

**Prevent Duplicate Comments:**
1. Add sync markers to local files after each sync:
   ```markdown
   <!-- SYNCED: 2024-01-15T10:30:00Z -->
   ```
2. Only sync content added after the last marker
3. If no new content, skip sync with message: "No updates since last sync"

### 12. Comment Size Management

**Handle GitHub's Comment Limits:**
- Max comment size: 65,536 characters
- If update exceeds limit:
  1. Split into multiple comments
  2. Or summarize with link to full details
  3. Warn user: "âš ï¸ Update truncated due to size. Full details in local files."

### 13. Error Handling

**Common Issues and Recovery:**

1. **Network Error:**
   - Message: "âŒ Failed to post comment: network error"
   - Solution: "Check internet connection and retry"
   - Keep local updates intact for retry

2. **Rate Limit:**
   - Message: "âŒ GitHub rate limit exceeded"
   - Solution: "Wait {minutes} minutes or use different token"
   - Save comment locally for later sync

3. **Permission Denied:**
   - Message: "âŒ Cannot comment on issue (permission denied)"
   - Solution: "Check repository access permissions"

4. **Issue Locked:**
   - Message: "âš ï¸ Issue is locked for comments"
   - Solution: "Contact repository admin to unlock"

### 14. Epic Progress Calculation

When updating epic progress:
1. Count total tasks in epic directory
2. Count tasks with `status: closed` in frontmatter
3. Calculate: `progress = (closed_tasks / total_tasks) * 100`
4. Round to nearest integer
5. Update epic frontmatter only if percentage changed

### 15. Post-Sync Validation

After successful sync:
- [ ] Verify comment posted on GitHub
- [ ] Confirm frontmatter updated with sync timestamp
- [ ] Check epic progress updated if task completed
- [ ] Validate no data corruption in local files

This creates a transparent audit trail of development progress that stakeholders can follow in real-time for Issue #$ARGUMENTS, while maintaining accurate frontmatter across all project files.



