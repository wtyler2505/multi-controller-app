---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality", "testing"]
description: Test Automation Orchestrator
allowed-tools: Read, Write, Edit, Bash
argument-hint: [orchestration-type] | --parallel | --sequential | --conditional | --pipeline-optimization

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["test-orchestration", "automation-optimization", "resource-management"]
    complexity-factors: ["parallel-execution", "resource-allocation", "performance-optimization"]
    specialized-tools: ["test-orchestration", "automation-management", "resource-optimization"]
  preferred-agents:
    primary: "mock-test-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["FileScopeMCP", "desktop-commander", "cipher-memory"]
    specialized-functions: ["test-orchestration", "automation-optimization"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "test-orchestration + automation-optimization + resource-management"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "orchestration-patterns + automation-knowledge"
    
    knowledge-preparation:
      - domain: "test-automation"
      - pattern-search: "orchestration-strategies + automation-patterns + resource-optimization"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["orchestration-setup", "automation-configuration", "performance-optimization"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "orchestration-strategies + automation-approaches + resource-decisions"
      - pattern-recognition: "test-orchestration-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["orchestration-results", "automation-insights", "resource-techniques"]
      - knowledge-extraction: "orchestration-methodologies + automation-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["orchestration-relationships", "automation-dependencies", "resource-connections"]
      - cross-reference: "related-testing-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "automation-knowledge + orchestration-patterns"
      - continuous-learning: "test-orchestration-optimization"

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
    - orchestration-setup
    - automation-configuration
    - resource-optimization
    - performance-monitoring
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "test-automation-orchestrator"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "orchestration-results + automation-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["orchestration-patterns", "automation-techniques", "resource-optimization-methods"]
  learn-from: ["generate-tests", "setup-comprehensive-testing", "test-quality-analyzer"]
  contribute-to: "testing-automation-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-orchestration-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-orchestration-setup
    - continuous-memory-updates
    - real-time-performance-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - orchestration-pattern-extraction
---

# Test Automation Orchestrator

Orchestrate intelligent test automation with execution optimization and resource management: **$ARGUMENTS**

## Current Orchestration Context

- Test suites: !`find . -name "*.test.*" -o -name "*.spec.*" | wc -l` test files across project
- Test frameworks: !`find . -name "jest.config.*" -o -name "cypress.config.*" -o -name "playwright.config.*" | wc -l` configured frameworks
- CI system: !`find . -name ".github" -o -name ".gitlab-ci.yml" | head -1 || echo "No CI detected"`
- Resource usage: Analysis of current test execution patterns and performance

## Task

Implement intelligent test orchestration with execution optimization and resource management:

**Orchestration Type**: Use $ARGUMENTS to focus on parallel execution, sequential execution, conditional testing, or pipeline optimization

**Test Orchestration Framework**:
1. **Test Discovery & Classification** - Analyze test suites, classify test types, assess execution requirements, optimize categorization
2. **Execution Strategy Design** - Design parallel execution strategies, implement intelligent batching, optimize resource allocation, configure conditional execution
3. **Dependency Management** - Analyze test dependencies, implement execution ordering, configure prerequisite validation, optimize dependency resolution
4. **Resource Optimization** - Configure parallel execution, implement resource pooling, optimize memory usage, design scalable execution
5. **Pipeline Integration** - Design CI/CD integration, implement stage orchestration, configure failure handling, optimize feedback loops
6. **Monitoring & Analytics** - Implement execution monitoring, configure performance tracking, design failure analysis, optimize reporting

**Advanced Features**: AI-driven test selection, predictive execution optimization, dynamic resource allocation, intelligent failure recovery, cost optimization.

**Quality Assurance**: Execution reliability, performance consistency, resource efficiency, maintainability optimization.

**Output**: Complete test orchestration system with optimized execution, intelligent resource management, comprehensive monitoring, and performance analytics.

