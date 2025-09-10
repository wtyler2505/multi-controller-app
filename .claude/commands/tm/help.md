---
model: claude-sonnet-4-20250514
category: project-management
priority: critical
tags: ["project-management", "help-system", "command-guidance", "workflow-assistance"]
description: Get help and guidance for available commands and workflows with comprehensive assistance

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["help-system", "command-guidance", "workflow-assistance"]
  complexity-level: "simple"
  selection-criteria:
    keyword-match: 0.85
    argument-analysis: 0.80
    project-context: 0.75
  preferred-agents: ["general-purpose"]
  fallback-agents: ["task-orchestrator"]
  confidence-threshold: 0.70

# Universal Cipher Memory Integration (MANDATORY)
cipher-memory-integration:
  enabled: true
  priority: "medium"
  pre-execution-memory:
    context-search:
      - query-pattern: "help system + command guidance + workflow assistance"
      - search-depth: "standard"
      - max-results: 8
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - help-history: "mcp__cipher-memory__search_nodes"
      - guidance-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    help-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - help-summary: "mcp__cipher-memory__create_entities"
      - guidance-patterns: "mcp__cipher-memory__create_entities"
      - assistance-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - command-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - help-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - help-insights: "mcp__cipher-memory__create_entities"

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
    help-tracking: true
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
  learning-domains: ["help-system", "command-guidance", "workflow-assistance"]
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
    load-help-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-help-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

tool-chain: "help-system-command-guidance"
auto-deploy: true
parallel-execution: false
allowed-tools: ["Bash", "Read", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph"]
---

Show help for Task Master commands.

Arguments: $ARGUMENTS

Display help for Task Master commands. If arguments provided, show specific command help.

## Task Master Command Help

### Quick Navigation

Type `/project:tm/` and use tab completion to explore all commands.

### Command Categories

#### ðŸš€ Setup & Installation
- `/project:tm/setup/install` - Comprehensive installation guide
- `/project:tm/setup/quick-install` - One-line global install

#### ðŸ“‹ Project Setup
- `/project:tm/init` - Initialize new project
- `/project:tm/init/quick` - Quick setup with auto-confirm
- `/project:tm/models` - View AI configuration
- `/project:tm/models/setup` - Configure AI providers

#### ðŸŽ¯ Task Generation
- `/project:tm/parse-prd` - Generate tasks from PRD
- `/project:tm/parse-prd/with-research` - Enhanced parsing
- `/project:tm/generate` - Create task files

#### ðŸ“ Task Management
- `/project:tm/list` - List tasks (natural language filters)
- `/project:tm/show <id>` - Display task details
- `/project:tm/add-task` - Create new task
- `/project:tm/update` - Update tasks naturally
- `/project:tm/next` - Get next task recommendation

#### ðŸ”„ Status Management
- `/project:tm/set-status/to-pending <id>`
- `/project:tm/set-status/to-in-progress <id>`
- `/project:tm/set-status/to-done <id>`
- `/project:tm/set-status/to-review <id>`
- `/project:tm/set-status/to-deferred <id>`
- `/project:tm/set-status/to-cancelled <id>`

#### ðŸ” Analysis & Breakdown
- `/project:tm/analyze-complexity` - Analyze task complexity
- `/project:tm/expand <id>` - Break down complex task
- `/project:tm/expand/all` - Expand all eligible tasks

#### ðŸ”— Dependencies
- `/project:tm/add-dependency` - Add task dependency
- `/project:tm/remove-dependency` - Remove dependency
- `/project:tm/validate-dependencies` - Check for issues

#### ðŸ¤– Workflows
- `/project:tm/workflows/smart-flow` - Intelligent workflows
- `/project:tm/workflows/pipeline` - Command chaining
- `/project:tm/workflows/auto-implement` - Auto-implementation

#### ðŸ“Š Utilities
- `/project:tm/utils/analyze` - Project analysis
- `/project:tm/status` - Project dashboard
- `/project:tm/learn` - Interactive learning

### Natural Language Examples

```
/project:tm/list pending high priority
/project:tm/update mark all API tasks as done
/project:tm/add-task create login system with OAuth
/project:tm/show current
```

### Getting Started

1. Install: `/project:tm/setup/quick-install`
2. Initialize: `/project:tm/init/quick`
3. Learn: `/project:tm/learn start`
4. Work: `/project:tm/workflows/smart-flow`

For detailed command info: `/project:tm/help <command-name>`


