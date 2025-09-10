---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Command for epic-oneshot operations
allowed-tools: Read, LS, mcp__taskmaster-ai__get_tasks, mcp__desktop-commander__list_directory
argument-hint: <feature_name> | --skip-validation | --parallel

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-orchestration", "workflow-automation", "command-chaining"]
    complexity-factors: ["multi-command-coordination", "parallel-execution", "error-handling"]
    specialized-tools: ["epic-management", "workflow-orchestration", "command-chaining"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["epic-orchestration", "workflow-automation"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-orchestration + workflow-automation + command-chaining"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "orchestration-patterns + workflow-knowledge + chaining-strategies"
    
    knowledge-preparation:
      - domain: "epic-orchestration"
      - pattern-search: "orchestration-strategies + workflow-patterns + automation-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["workflow-initialization", "command-coordination", "parallel-execution"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "orchestration-strategies + coordination-approaches + automation-decisions"
      - pattern-recognition: "epic-orchestration-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["orchestration-results", "workflow-insights", "automation-techniques"]
      - knowledge-extraction: "orchestration-methodologies + workflow-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["orchestration-relationships", "workflow-dependencies", "automation-connections"]
      - cross-reference: "related-orchestration-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "orchestration-knowledge + workflow-patterns"
      - continuous-learning: "epic-orchestration-optimization"

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
    - workflow-initialization
    - command-coordination
    - parallel-execution
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-oneshot"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "orchestration-results + workflow-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["orchestration-patterns", "workflow-techniques", "automation-strategies"]
  learn-from: ["epic-decompose", "epic-sync", "workflow-management"]
  contribute-to: "epic-orchestration-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-epic-prerequisites
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-command-coordination
    - continuous-memory-updates
    - real-time-workflow-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - orchestration-pattern-extraction
---

# Epic Oneshot

Decompose epic into tasks and sync to GitHub in one operation.

## Usage
```
/pm:epic-oneshot <feature_name>
```

## Instructions

### 1. Validate Prerequisites

Check that epic exists and hasn't been processed:
```bash
# Epic must exist
test -f .claude/epics/$ARGUMENTS/epic.md || echo "âŒ Epic not found. Run: /pm:prd-parse $ARGUMENTS"

# Check for existing tasks
if ls .claude/epics/$ARGUMENTS/[0-9]*.md 2>/dev/null | grep -q .; then
  echo "âš ï¸ Tasks already exist. This will create duplicates."
  echo "Delete existing tasks or use /pm:epic-sync instead."
  exit 1
fi

# Check if already synced
if grep -q "github:" .claude/epics/$ARGUMENTS/epic.md; then
  echo "âš ï¸ Epic already synced to GitHub."
  echo "Use /pm:epic-sync to update."
  exit 1
fi
```

### 2. Execute Decompose

Simply run the decompose command:
```
Running: /pm:epic-decompose $ARGUMENTS
```

This will:
- Read the epic
- Create task files (using parallel agents if appropriate)
- Update epic with task summary

### 3. Execute Sync

Immediately follow with sync:
```
Running: /pm:epic-sync $ARGUMENTS
```

This will:
- Create epic issue on GitHub
- Create sub-issues (using parallel agents if appropriate)
- Rename task files to issue IDs
- Create worktree

### 4. Output

```
ðŸš€ Epic Oneshot Complete: $ARGUMENTS

Step 1: Decomposition âœ“
  - Tasks created: {count}
  
Step 2: GitHub Sync âœ“
  - Epic: #{number}
  - Sub-issues created: {count}
  - Worktree: ../epic-$ARGUMENTS

Ready for development!
  Start work: /pm:epic-start $ARGUMENTS
  Or single task: /pm:issue-start {task_number}
```

## Important Notes

This is simply a convenience wrapper that runs:
1. `/pm:epic-decompose` 
2. `/pm:epic-sync`

Both commands handle their own error checking, parallel execution, and validation. This command just orchestrates them in sequence.

Use this when you're confident the epic is ready and want to go from epic to GitHub issues in one step.


