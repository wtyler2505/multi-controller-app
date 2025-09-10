---
model: claude-sonnet-4-20250514
category: utilities-tools
priority: high
tags: ["utilities-tools", "setup"]
description: Command for initref operations

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["project-analysis", "documentation-creation", "reference-building"]
    complexity-factors: ["project-introspection", "documentation-synthesis", "knowledge-extraction"]
    specialized-tools: ["project-analysis", "documentation-generation", "reference-creation"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "content-creator"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["FileScopeMCP", "desktop-commander", "cipher-memory"]
    specialized-functions: ["project-analysis", "documentation-creation"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "project-analysis + documentation-creation + reference-building"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "project-patterns + documentation-knowledge"
    
    knowledge-preparation:
      - domain: "project-analysis"
      - pattern-search: "project-structures + documentation-patterns + reference-strategies"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["project-analysis", "documentation-creation", "reference-building"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "analysis-strategies + documentation-approaches + reference-methodologies"
      - pattern-recognition: "project-reference-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["reference-results", "project-insights", "documentation-techniques"]
      - knowledge-extraction: "project-methodologies + reference-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["project-relationships", "documentation-dependencies", "reference-connections"]
      - cross-reference: "related-project-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "project-knowledge + documentation-patterns"
      - continuous-learning: "reference-building-optimization"

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
    - project-analysis
    - documentation-creation
    - reference-building
    - synthesis-generation
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "initref"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "reference-building-results + project-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["project-patterns", "documentation-techniques", "reference-building-methods"]
  learn-from: ["project_reflection", "directory-deep-dive", "create-architecture-documentation"]
  contribute-to: "project-analysis-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-project-structure
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-project-analysis
    - continuous-memory-updates
    - real-time-reference-building
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - project-pattern-extraction
---

Build a reference for the implementation details of this project. Use provided summarize tool to get summary of the files. Avoid reading the content of many files yourself, as we might hit usage limits. Do read the content of important files though. Use the returned summaries to create reference files in /ref directory. Use markdown format for writing the documentation files.

Update CLAUDE.md file with the pointers to important documentation files.



