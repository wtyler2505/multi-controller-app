---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management"]
description: Command for blocked operations
allowed-tools: Bash
argument-hint: [block-type] | --issue | --dependency | --external

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["issue-resolution", "dependency-management", "blocker-analysis"]
    complexity-factors: ["blocker-identification", "resolution-planning", "dependency-tracking"]
    specialized-tools: ["project-management", "issue-tracking", "dependency-analysis"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "task-orchestrator"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "cipher-memory", "taskmaster-ai"]
    specialized-functions: ["blocker-analysis", "issue-resolution"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "issue-resolution + dependency-management + blocker-analysis"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "blocker-patterns + resolution-knowledge"
    
    knowledge-preparation:
      - domain: "issue-resolution"
      - pattern-search: "blocker-strategies + resolution-patterns + management-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["blocker-analysis", "resolution-planning", "dependency-tracking"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "blocker-strategies + resolution-approaches + management-decisions"
      - pattern-recognition: "issue-resolution-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["blocker-results", "resolution-insights", "management-techniques"]
      - knowledge-extraction: "issue-methodologies + resolution-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["blocker-relationships", "resolution-dependencies", "management-connections"]
      - cross-reference: "related-resolution-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "blocker-knowledge + resolution-patterns"
      - continuous-learning: "issue-resolution-optimization"

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
    - blocker-analysis
    - resolution-planning
    - dependency-tracking
    - issue-monitoring
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-blocked"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "blocker-results + resolution-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["blocker-patterns", "resolution-techniques", "management-strategies"]
  learn-from: ["pm-status", "pm-next", "issue-tracking"]
  contribute-to: "issue-resolution-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-project-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-blocker-analysis
    - continuous-memory-updates
    - real-time-resolution-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - blocker-pattern-extraction
---

Run `bash .claude/scripts/pm/blocked.sh` using a sub-agent and show me the complete output.

- DO NOT truncate.
- DO NOT collapse.
- DO NOT abbreviate.
- Show ALL lines in full.
- DO NOT print any other comments.



