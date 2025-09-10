---
model: claude-sonnet-4-20250514
category: bridge-integration
priority: high
tags: ["bridge-integration"]
description: Bidirectional Task Master and GitHub synchronization
allowed-tools: Read, Write, LS, Bash, mcp__taskmaster-ai__get_tasks
argument-hint: [sync-direction] | --bidirectional | --to-github | --from-github

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["task-synchronization", "github-integration", "bridge-operations"]
    complexity-factors: ["bidirectional-sync", "data-mapping", "conflict-resolution"]
    specialized-tools: ["github-api", "task-management", "synchronization-tools"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["task-synchronization", "github-integration"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "task-synchronization + github-integration + bridge-operations"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "sync-patterns + integration-knowledge"
    
    knowledge-preparation:
      - domain: "task-synchronization"
      - pattern-search: "sync-strategies + integration-patterns + bridge-operations"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["sync-analysis", "data-mapping", "integration-execution"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "sync-strategies + mapping-approaches + integration-decisions"
      - pattern-recognition: "synchronization-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["sync-results", "integration-insights", "bridge-techniques"]
      - knowledge-extraction: "synchronization-methodologies + integration-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["sync-relationships", "integration-dependencies", "bridge-connections"]
      - cross-reference: "related-synchronization-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "synchronization-knowledge + integration-patterns"
      - continuous-learning: "sync-process-optimization"

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
    - sync-analysis
    - data-mapping
    - integration-execution
    - conflict-resolution
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "bridge-sync-all"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "sync-results + integration-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["sync-patterns", "integration-techniques", "bridge-strategies"]
  learn-from: ["tm-to-epic", "parallel-start", "github-sync-operations"]
  contribute-to: "task-synchronization-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-github-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-sync-operations
    - continuous-memory-updates
    - real-time-conflict-resolution
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - sync-pattern-extraction
---

# Bridge: Bidirectional Sync Task Master â†” GitHub

Synchronize Task Master tasks with GitHub Issues for team visibility.

## Usage
```
/bridge:sync-all
```

## Steps

1. **Export Task Master to GitHub**
   - Get all tasks from Task Master
   - For each task without GitHub Issue ID:
     ```bash
     gh issue create \
       --title "[Task {id}] {title}" \
       --body "{description}\n\nTask Master ID: {id}" \
       --label "task:multi-controller"
     ```
   - Store Issue number back in Task Master metadata

2. **Create Sub-Issues for Subtasks**
   - For tasks with subtasks:
     ```bash
     gh sub-issue create {parent-issue} \
       --title "[Subtask {id}] {title}" \
       --body "{details}"
     ```

3. **Update Existing Issues**
   - For tasks with Issue IDs:
     ```bash
     gh issue edit {issue-id} \
       --body "{updated-description}\n\nStatus: {status}"
     ```

4. **Import GitHub Comments**
   - For each issue with new comments:
     ```bash
     gh issue view {issue-id} --comments
     ```
   - Add as Task Master updates if relevant

5. **Sync Status**
   - Map Task Master status to GitHub:
     - pending â†’ open
     - in-progress â†’ open + "in-progress" label
     - review â†’ open + "review" label
     - done â†’ closed
     - blocked â†’ open + "blocked" label

6. **Generate Report**
   ```
   Synced Tasks:
   - Task 4 â†’ Issue #1245
   - Task 5 â†’ Issue #1246
   
   Updated Statuses:
   - Task 3: done â†’ Issue #1244: closed
   
   New Comments:
   - Issue #1243: 2 new comments imported
   ```

## Metadata Storage

Store GitHub Issue IDs in `.taskmaster/github-map.json`:
```json
{
  "tasks": {
    "4": 1245,
    "4.1": 1247,
    "5": 1246
  },
  "lastSync": "2025-08-25T06:00:00Z"
}
```

## Error Handling
- Rate limit detection and backoff
- Partial sync on failure
- Conflict resolution for concurrent edits


