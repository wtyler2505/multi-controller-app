---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Epic Close
allowed-tools: Bash, Read, Write, LS
argument-hint: <epic_name> | --archive | --sync-github

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-completion", "project-closure", "github-integration"]
    complexity-factors: ["completion-validation", "status-synchronization", "archive-management"]
    specialized-tools: ["epic-management", "github-api", "completion-tracking"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "task-orchestrator"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "cipher-memory", "taskmaster-ai"]
    specialized-functions: ["epic-completion", "github-integration"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-completion + project-closure + github-integration"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "completion-patterns + closure-knowledge"
    
    knowledge-preparation:
      - domain: "epic-completion"
      - pattern-search: "completion-strategies + closure-patterns + sync-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["completion-validation", "status-synchronization", "archive-execution"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "completion-strategies + sync-approaches + archive-decisions"
      - pattern-recognition: "epic-completion-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["completion-results", "closure-insights", "sync-techniques"]
      - knowledge-extraction: "completion-methodologies + closure-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["completion-relationships", "closure-dependencies", "sync-connections"]
      - cross-reference: "related-completion-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "completion-knowledge + closure-patterns"
      - continuous-learning: "epic-completion-optimization"

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
    - completion-validation
    - status-synchronization
    - archive-execution
    - github-integration
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-close"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "completion-results + closure-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["completion-patterns", "closure-techniques", "sync-strategies"]
  learn-from: ["epic-status", "epic-sync", "task-completion"]
  contribute-to: "epic-completion-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-epic-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-completion-validation
    - continuous-memory-updates
    - real-time-sync-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - completion-pattern-extraction
---

# Epic Close

Mark an epic as complete when all tasks are done.

## Usage
```
/pm:epic-close <epic_name>
```

## Instructions

### 1. Verify All Tasks Complete

Check all task files in `.claude/epics/$ARGUMENTS/`:
- Verify all have `status: closed` in frontmatter
- If any open tasks found: "âŒ Cannot close epic. Open tasks remain: {list}"

### 2. Update Epic Status

Get current datetime: `date -u +"%Y-%m-%dT%H:%M:%SZ"`

Update epic.md frontmatter:
```yaml
status: completed
progress: 100%
updated: {current_datetime}
completed: {current_datetime}
```

### 3. Update PRD Status

If epic references a PRD, update its status to "complete".

### 4. Close Epic on GitHub

If epic has GitHub issue:
```bash
gh issue close {epic_issue_number} --comment "âœ… Epic completed - all tasks done"
```

### 5. Archive Option

Ask user: "Archive completed epic? (yes/no)"

If yes:
- Move epic directory to `.claude/epics/.archived/{epic_name}/`
- Create archive summary with completion date

### 6. Output

```
âœ… Epic closed: $ARGUMENTS
  Tasks completed: {count}
  Duration: {days_from_created_to_completed}
  
{If archived}: Archived to .claude/epics/.archived/

Next epic: Run /pm:next to see priority work
```

## Important Notes

Only close epics with all tasks complete.
Preserve all data when archiving.
Update related PRD status.


