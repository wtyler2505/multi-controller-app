---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Command for epic-status operations
allowed-tools: Bash, mcp__taskmaster-ai__get_tasks, mcp__desktop-commander__read_file
argument-hint: <epic_name> | --format=<format> | --refresh

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-monitoring", "status-reporting", "progress-visualization"]
    complexity-factors: ["multi-status-aggregation", "real-time-monitoring", "report-generation"]
    specialized-tools: ["epic-management", "status-monitoring", "progress-reporting"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["status-monitoring", "progress-reporting"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-monitoring + status-reporting + progress-visualization"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "monitoring-patterns + reporting-knowledge + visualization-strategies"
    
    knowledge-preparation:
      - domain: "epic-monitoring"
      - pattern-search: "monitoring-strategies + reporting-patterns + visualization-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["status-collection", "progress-analysis", "report-generation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "monitoring-strategies + reporting-approaches + visualization-decisions"
      - pattern-recognition: "epic-monitoring-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["monitoring-results", "reporting-insights", "visualization-techniques"]
      - knowledge-extraction: "monitoring-methodologies + reporting-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["monitoring-relationships", "reporting-dependencies", "visualization-connections"]
      - cross-reference: "related-monitoring-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "monitoring-knowledge + reporting-patterns"
      - continuous-learning: "epic-monitoring-optimization"

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
    - status-collection
    - progress-analysis
    - report-generation
    - visualization-rendering
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-status"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "monitoring-results + reporting-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["monitoring-patterns", "reporting-techniques", "visualization-strategies"]
  learn-from: ["epic-refresh", "epic-show", "status-monitoring"]
  contribute-to: "epic-monitoring-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-epic-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-status-collection
    - continuous-memory-updates
    - real-time-monitoring-analysis
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - monitoring-pattern-extraction
---

Run `bash .claude/scripts/pm/epic-status.sh $ARGUMENTS` using the bash tool and show me the complete stdout printed to the console.

- DO NOT truncate.
- DO NOT collapse.
- DO NOT abbreviate.
- Show ALL lines in full.
- DO NOT print any other comments.



