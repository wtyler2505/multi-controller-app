---
model: claude-sonnet-4-20250514
category: architecture-design
priority: high
tags: ["architecture-design"]
description: Design Database Schema
allowed-tools: Read, Write, Edit, Bash
argument-hint: [schema-type] | --relational | --nosql | --hybrid | --normalize

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["database-design", "data-modeling", "schema-optimization"]
    complexity-factors: ["normalization-strategies", "performance-optimization", "scalability-planning"]
    specialized-tools: ["database-modeling", "schema-generation", "performance-analysis"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "database-architect"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["clear-thought", "context7", "cipher-memory"]
    specialized-functions: ["database-design", "schema-modeling"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "database-design + schema-modeling + data-architecture"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "database-patterns + schema-design-knowledge"
    
    knowledge-preparation:
      - domain: "database-architecture"
      - pattern-search: "schema-patterns + database-design + normalization-strategies"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["schema-analysis", "design-decisions", "optimization-strategies"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "schema-approaches + optimization-strategies + design-trade-offs"
      - pattern-recognition: "database-design-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["schema-designs", "database-insights", "optimization-techniques"]
      - knowledge-extraction: "database-design-methodologies + schema-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["schema-relationships", "data-dependencies", "performance-connections"]
      - cross-reference: "related-database-designs"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "database-knowledge + schema-design-patterns"
      - continuous-learning: "database-design-optimization"

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
    - schema-analysis
    - design-decisions
    - optimization-planning
    - validation-testing
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "design-database-schema"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "schema-design-results + database-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["database-patterns", "schema-design-techniques", "optimization-strategies"]
  learn-from: ["architecture-scenario-explorer", "implement-caching-strategy", "system-behavior-simulator"]
  contribute-to: "database-architecture-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-schema-analysis
    - continuous-memory-updates
    - real-time-design-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - database-pattern-extraction
---

# Design Database Schema

Design optimized database schemas with comprehensive data modeling: **$ARGUMENTS**

## Current Project Context

- Application type: Based on $ARGUMENTS or codebase analysis
- Data requirements: @requirements/ or project documentation
- Existing schema: @prisma/schema.prisma or @migrations/ or database dumps
- Performance needs: Expected scale, query patterns, and data volume

## Task

Design comprehensive database schema with optimal structure and performance:

**Schema Type**: Use $ARGUMENTS to specify relational, NoSQL, hybrid approach, or normalization level

**Design Framework**:
1. **Requirements Analysis** - Business entities, relationships, data flow, and access patterns
2. **Entity Modeling** - Tables/collections, attributes, primary/foreign keys, constraints
3. **Relationship Design** - One-to-one, one-to-many, many-to-many associations
4. **Normalization Strategy** - Data consistency vs performance trade-offs
5. **Performance Optimization** - Indexing strategy, query optimization, partitioning
6. **Security Design** - Access control, data encryption, audit trails

**Advanced Patterns**: Implement temporal data, soft deletes, JSONB fields, full-text search, audit logging, and scalability patterns.

**Validation**: Ensure referential integrity, data consistency, query performance, and future extensibility.

**Output**: Complete schema design with DDL scripts, ER diagrams, performance analysis, and migration strategy.

