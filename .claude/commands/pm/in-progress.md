---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management"]
description: Command for in-progress operations
allowed-tools: Bash, mcp__taskmaster-ai__get_tasks, mcp__desktop-commander__read_file
argument-hint: [filter-options] | --status | --detailed

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["progress-monitoring", "status-tracking", "workflow-analysis"]
    complexity-factors: ["status-aggregation", "progress-visualization", "workflow-insights"]
    specialized-tools: ["progress-tracking", "status-monitoring", "workflow-analysis"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["progress-monitoring", "status-tracking"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "progress-monitoring + status-tracking + workflow-analysis"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "monitoring-patterns + tracking-knowledge + workflow-strategies"
    
    knowledge-preparation:
      - domain: "progress-monitoring"
      - pattern-search: "monitoring-strategies + tracking-patterns + workflow-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["status-collection", "progress-analysis", "workflow-insights"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "monitoring-strategies + tracking-approaches + workflow-decisions"
      - pattern-recognition: "progress-monitoring-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["monitoring-results", "tracking-insights", "workflow-techniques"]
      - knowledge-extraction: "monitoring-methodologies + tracking-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["monitoring-relationships", "tracking-dependencies", "workflow-connections"]
      - cross-reference: "related-monitoring-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "monitoring-knowledge + tracking-patterns"
      - continuous-learning: "progress-monitoring-optimization"

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
    - workflow-insights
    - reporting-generation
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-in-progress"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "monitoring-results + tracking-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["monitoring-patterns", "tracking-techniques", "workflow-strategies"]
  learn-from: ["epic-status", "pm-status", "progress-tracking"]
  contribute-to: "progress-monitoring-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-monitoring-access
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

Run `bash .claude/scripts/pm/in-progress.sh` using a sub-agent and show me the complete output.

- DO NOT truncate.
- DO NOT collapse.
- DO NOT abbreviate.
- Show ALL lines in full.
- DO NOT print any other comments.



