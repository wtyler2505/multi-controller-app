---
model: claude-sonnet-4-20250514
category: project-management
priority: critical
tags: ["project-management"]
description: Get comprehensive help and command guidance
allowed-tools: Bash, Read, Write
argument-hint: [help-type] | --commands | --workflows | --overview | --examples

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["help-systems", "documentation", "command-guidance"]
    complexity-factors: ["help-delivery", "user-assistance", "command-explanation"]
    specialized-tools: ["help-generation", "documentation-access", "guidance-systems"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "documentation-specialist"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "cipher-memory", "context7"]
    specialized-functions: ["help-delivery", "command-guidance"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "help-systems + documentation + command-guidance"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "help-patterns + guidance-knowledge"
    
    knowledge-preparation:
      - domain: "help-systems"
      - pattern-search: "help-strategies + guidance-patterns + documentation-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["help-analysis", "guidance-delivery", "user-assistance"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "help-strategies + delivery-approaches + guidance-decisions"
      - pattern-recognition: "help-delivery-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["help-results", "guidance-insights", "assistance-techniques"]
      - knowledge-extraction: "help-methodologies + guidance-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["help-relationships", "guidance-dependencies", "assistance-connections"]
      - cross-reference: "related-help-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "help-knowledge + guidance-patterns"
      - continuous-learning: "help-system-optimization"

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
    - help-analysis
    - guidance-delivery
    - user-assistance
    - documentation-access
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-help"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "help-delivery-results + guidance-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["help-patterns", "guidance-techniques", "assistance-strategies"]
  learn-from: ["pm-status", "pm-init", "command-documentation"]
  contribute-to: "help-system-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-help-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-help-retrieval
    - continuous-memory-updates
    - real-time-guidance-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - help-pattern-extraction
---

Run `bash .claude/scripts/pm/help.sh` using a sub-agent and show me the complete output.

- DO NOT truncate.
- DO NOT collapse.
- DO NOT abbreviate.
- Show ALL lines in full.
- DO NOT print any other comments.



