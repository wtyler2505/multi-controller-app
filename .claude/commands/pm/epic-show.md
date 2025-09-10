---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Command for epic-show operations
allowed-tools: Bash, mcp__taskmaster-ai__get_task, mcp__desktop-commander__read_file
argument-hint: <epic_name> | --format=<format> | --detailed

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-display", "information-presentation", "data-visualization"]
    complexity-factors: ["data-aggregation", "formatting-requirements", "interactive-display"]
    specialized-tools: ["epic-management", "data-presentation", "information-visualization"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["epic-display", "information-presentation"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-display + information-presentation + data-visualization"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "display-patterns + presentation-knowledge + visualization-strategies"
    
    knowledge-preparation:
      - domain: "epic-display"
      - pattern-search: "display-strategies + presentation-patterns + visualization-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["data-retrieval", "formatting-operations", "display-generation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "display-strategies + presentation-approaches + visualization-decisions"
      - pattern-recognition: "epic-display-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["display-results", "presentation-insights", "visualization-techniques"]
      - knowledge-extraction: "display-methodologies + presentation-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["display-relationships", "presentation-dependencies", "visualization-connections"]
      - cross-reference: "related-display-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "display-knowledge + presentation-patterns"
      - continuous-learning: "epic-display-optimization"

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
    - data-retrieval
    - formatting-operations
    - display-generation
    - user-interaction
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-show"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "display-results + presentation-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["display-patterns", "presentation-techniques", "visualization-strategies"]
  learn-from: ["epic-status", "epic-list", "information-display"]
  contribute-to: "epic-display-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-epic-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-data-retrieval
    - continuous-memory-updates
    - real-time-formatting-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - display-pattern-extraction
---

Run `bash .claude/scripts/pm/epic-show.sh $ARGUMENTS` using a sub-agent and show me the complete output.

- DO NOT truncate.
- DO NOT collapse.
- DO NOT abbreviate.
- Show ALL lines in full.
- DO NOT print any other comments.



