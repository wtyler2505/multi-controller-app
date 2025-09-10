---
model: claude-sonnet-4-20250514
category: taskmaster-core
priority: critical
tags: ["taskmaster-core", "taskmaster", "central-hub", "task-orchestration", "command-reference"]
description: Task Master main command - comprehensive central hub for task management operations with full command orchestration

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["task-management", "command-orchestration", "workflow-coordination"]
  complexity-level: "complex"
  selection-criteria:
    keyword-match: 0.95
    argument-analysis: 0.95
    project-context: 0.90
  preferred-agents: ["task-orchestrator"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.85

# Universal Cipher Memory Integration (MANDATORY)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    context-search:
      - query-pattern: "task management + command orchestration + workflow coordination"
      - search-depth: "comprehensive"
      - max-results: 20
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - task-history: "mcp__cipher-memory__search_nodes"
      - command-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    orchestration-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - orchestration-summary: "mcp__cipher-memory__create_entities"
      - command-patterns: "mcp__cipher-memory__create_entities"
      - workflow-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - task-relationships: "mcp__cipher-memory__create_relations"
      - command-relationships: "mcp__cipher-memory__create_relations"
      - workflow-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - orchestration-insights: "mcp__cipher-memory__create_entities"

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
    orchestration-tracking: true
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
  learning-domains: ["task-management", "command-orchestration", "workflow-coordination"]
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
    load-task-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-orchestration-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

tool-chain: "task-management-command-orchestration"
auto-deploy: true
parallel-execution: false
allowed-tools: ["Bash", "Read", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph", "mcp__taskmaster-ai__*"]
---

# Task Master Command Reference

Comprehensive command structure for Task Master integration with Claude Code.

## Command Organization

Commands are organized hierarchically to match Task Master's CLI structure while providing enhanced Claude Code integration.

## Project Setup & Configuration

### `/project:tm/init`
- `init-project` - Initialize new project (handles PRD files intelligently)
- `init-project-quick` - Quick setup with auto-confirmation (-y flag)

### `/project:tm/models`
- `view-models` - View current AI model configuration
- `setup-models` - Interactive model configuration
- `set-main` - Set primary generation model
- `set-research` - Set research model
- `set-fallback` - Set fallback model

## Task Generation

### `/project:tm/parse-prd`
- `parse-prd` - Generate tasks from PRD document
- `parse-prd-with-research` - Enhanced parsing with research mode

### `/project:tm/generate`
- `generate-tasks` - Create individual task files from tasks.json

## Task Management

### `/project:tm/list`
- `list-tasks` - Smart listing with natural language filters
- `list-tasks-with-subtasks` - Include subtasks in hierarchical view
- `list-tasks-by-status` - Filter by specific status

### `/project:tm/set-status`
- `to-pending` - Reset task to pending
- `to-in-progress` - Start working on task
- `to-done` - Mark task complete
- `to-review` - Submit for review
- `to-deferred` - Defer task
- `to-cancelled` - Cancel task

### `/project:tm/sync-readme`
- `sync-readme` - Export tasks to README.md with formatting

### `/project:tm/update`
- `update-task` - Update tasks with natural language
- `update-tasks-from-id` - Update multiple tasks from a starting point
- `update-single-task` - Update specific task

### `/project:tm/add-task`
- `add-task` - Add new task with AI assistance

### `/project:tm/remove-task`
- `remove-task` - Remove task with confirmation

## Subtask Management

### `/project:tm/add-subtask`
- `add-subtask` - Add new subtask to parent
- `convert-task-to-subtask` - Convert existing task to subtask

### `/project:tm/remove-subtask`
- `remove-subtask` - Remove subtask (with optional conversion)

### `/project:tm/clear-subtasks`
- `clear-subtasks` - Clear subtasks from specific task
- `clear-all-subtasks` - Clear all subtasks globally

## Task Analysis & Breakdown

### `/project:tm/analyze-complexity`
- `analyze-complexity` - Analyze and generate expansion recommendations

### `/project:tm/complexity-report`
- `complexity-report` - Display complexity analysis report

### `/project:tm/expand`
- `expand-task` - Break down specific task
- `expand-all-tasks` - Expand all eligible tasks
- `with-research` - Enhanced expansion

## Task Navigation

### `/project:tm/next`
- `next-task` - Intelligent next task recommendation

### `/project:tm/show`
- `show-task` - Display detailed task information

### `/project:tm/status`
- `project-status` - Comprehensive project dashboard

## Dependency Management

### `/project:tm/add-dependency`
- `add-dependency` - Add task dependency

### `/project:tm/remove-dependency`
- `remove-dependency` - Remove task dependency

### `/project:tm/validate-dependencies`
- `validate-dependencies` - Check for dependency issues

### `/project:tm/fix-dependencies`
- `fix-dependencies` - Automatically fix dependency problems

## Workflows & Automation

### `/project:tm/workflows`
- `smart-workflow` - Context-aware intelligent workflow execution
- `command-pipeline` - Chain multiple commands together
- `auto-implement-tasks` - Advanced auto-implementation with code generation

## Utilities

### `/project:tm/utils`
- `analyze-project` - Deep project analysis and insights

### `/project:tm/setup`
- `install-taskmaster` - Comprehensive installation guide
- `quick-install-taskmaster` - One-line global installation

## Usage Patterns

### Natural Language
Most commands accept natural language arguments:
```
/project:tm/add-task create user authentication system
/project:tm/update mark all API tasks as high priority
/project:tm/list show blocked tasks
```

### ID-Based Commands
Commands requiring IDs intelligently parse from $ARGUMENTS:
```
/project:tm/show 45
/project:tm/expand 23
/project:tm/set-status/to-done 67
```

### Smart Defaults
Commands provide intelligent defaults and suggestions based on context.


