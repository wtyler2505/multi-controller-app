---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Command for epic-list operations
allowed-tools: Bash, mcp__desktop-commander__list_directory, mcp__taskmaster-ai__get_tasks
argument-hint: [filter-options] | --status=<status> | --all | --active

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-listing", "project-overview", "status-reporting"]
    complexity-factors: ["data-aggregation", "status-filtering", "progress-visualization"]
    specialized-tools: ["epic-management", "data-visualization", "status-reporting"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["epic-listing", "status-aggregation"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-listing + project-overview + status-reporting"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "listing-patterns + overview-knowledge"
    
    knowledge-preparation:
      - domain: "epic-listing"
      - pattern-search: "listing-strategies + overview-patterns + reporting-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["data-collection", "status-aggregation", "report-generation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "listing-strategies + aggregation-approaches + reporting-decisions"
      - pattern-recognition: "epic-listing-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["listing-results", "overview-insights", "reporting-techniques"]
      - knowledge-extraction: "listing-methodologies + overview-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["listing-relationships", "overview-dependencies", "reporting-connections"]
      - cross-reference: "related-listing-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "listing-knowledge + overview-patterns"
      - continuous-learning: "epic-listing-optimization"

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
    - data-collection
    - status-aggregation
    - report-generation
    - filtering-operations
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-list"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "listing-results + overview-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["listing-patterns", "overview-techniques", "reporting-strategies"]
  learn-from: ["epic-show", "epic-status", "project-overview"]
  contribute-to: "epic-listing-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-project-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-data-collection
    - continuous-memory-updates
    - real-time-aggregation-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - listing-pattern-extraction
---

Run `bash .claude/scripts/pm/epic-list.sh` using a sub-agent and show me the complete output.

- You MUST display the complete output.
- DO NOT truncate.
- DO NOT collapse.
- DO NOT abbreviate.
- Show ALL lines in full.
- DO NOT print any other comments.




