---
model: claude-sonnet-4-20250514
category: utilities-tools
priority: high
tags: ["utilities-tools"]
description: Directory Deep Dive

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["code-analysis", "architecture-understanding", "documentation-creation"]
    complexity-factors: ["directory-analysis", "pattern-recognition", "knowledge-extraction"]
    specialized-tools: ["code-exploration", "architecture-analysis", "documentation-generation"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "code-analyzer"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["FileScopeMCP", "desktop-commander", "cipher-memory"]
    specialized-functions: ["directory-analysis", "code-exploration"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "directory-analysis + code-exploration + architecture-understanding"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "code-analysis-patterns + architecture-knowledge"
    
    knowledge-preparation:
      - domain: "code-analysis"
      - pattern-search: "directory-patterns + architecture-analysis + code-organization"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["directory-exploration", "pattern-identification", "documentation-creation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "analysis-strategies + pattern-recognition + documentation-approaches"
      - pattern-recognition: "directory-analysis-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["directory-insights", "architecture-patterns", "code-organization-knowledge"]
      - knowledge-extraction: "code-analysis-methodologies + architectural-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["code-relationships", "architectural-dependencies", "organization-patterns"]
      - cross-reference: "related-code-analysis-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "code-analysis-knowledge + architectural-patterns"
      - continuous-learning: "code-exploration-optimization"

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
    - directory-exploration
    - pattern-identification
    - architecture-analysis
    - documentation-generation
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "directory-deep-dive"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "directory-analysis-results + architectural-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["code-analysis-patterns", "architectural-understanding", "documentation-techniques"]
  learn-from: ["create-architecture-documentation", "project_reflection", "initref"]
  contribute-to: "code-analysis-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-directory-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-directory-analysis
    - continuous-memory-updates
    - real-time-pattern-recognition
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - architectural-pattern-extraction
---

# Directory Deep Dive

Analyze directory structure and purpose

## Instructions

1. **Target Directory**
   - Focus on the specified directory `$ARGUMENTS` or the current working directory

2. **Investigate Architecture**
   - Analyze the implementation principles and architecture of the code in this directory and its subdirectories
   - Look for:
     - Design patterns being used
     - Dependencies and their purposes
     - Key abstractions and interfaces
     - Naming conventions and code organization

3. **Create or Update Documentation**
   - Create a CLAUDE.md file capturing this knowledge
   - If one already exists, update it with newly discovered information
   - Include:
     - Purpose and responsibility of this module
     - Key architectural decisions
     - Important implementation details
     - Common patterns used throughout the code
     - Any gotchas or non-obvious behaviors

4. **Ensure Proper Placement**
   - Place the CLAUDE.md file in the directory being analyzed
   - This ensures the context is loaded when working in that specific area

## Credit

This command is based on the work of Thomas Landgraf: https://thomaslandgraf.substack.com/p/claude-codes-memory-working-with


