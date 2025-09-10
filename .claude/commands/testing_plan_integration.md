---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality", "testing"]
description: Testing Plan Integration
allowed-tools: Read, Write, Edit, Bash
argument-hint: [target-code] | [test-type] | --rust | --inline | --refactoring-suggestions

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["testing-planning", "integration-testing", "refactoring-guidance"]
    complexity-factors: ["testability-analysis", "integration-strategy", "code-refactoring"]
    specialized-tools: ["testing-planning", "integration-analysis", "refactoring-assessment"]
  preferred-agents:
    primary: "mock-test-orchestrator"
    secondary: "rust-async-specialist"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["FileScopeMCP", "desktop-commander", "cipher-memory"]
    specialized-functions: ["testing-planning", "integration-testing"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "testing-planning + integration-testing + refactoring-guidance"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "testing-patterns + integration-knowledge"
    
    knowledge-preparation:
      - domain: "testing-integration"
      - pattern-search: "planning-strategies + integration-patterns + refactoring-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["planning-analysis", "integration-setup", "refactoring-guidance"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "planning-strategies + integration-approaches + refactoring-decisions"
      - pattern-recognition: "testing-integration-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["integration-plans", "testing-insights", "refactoring-techniques"]
      - knowledge-extraction: "integration-methodologies + planning-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["integration-relationships", "planning-dependencies", "refactoring-connections"]
      - cross-reference: "related-testing-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "testing-knowledge + integration-patterns"
      - continuous-learning: "testing-integration-optimization"

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
    - planning-analysis
    - integration-setup
    - refactoring-guidance
    - testability-assessment
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "testing_plan_integration"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "integration-planning-results + testing-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["integration-patterns", "planning-techniques", "refactoring-methodologies"]
  learn-from: ["generate-tests", "setup-comprehensive-testing", "test-quality-analyzer"]
  contribute-to: "testing-integration-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-integration-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-planning-analysis
    - continuous-memory-updates
    - real-time-integration-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - integration-pattern-extraction
---

# Testing Plan Integration

Create integration testing plan with inline test strategy and refactoring suggestions: **$ARGUMENTS**

## Current Testing Context

- Project type: !`[ -f Cargo.toml ] && echo "Rust project" || [ -f package.json ] && echo "Node.js project" || echo "Multi-language project"`
- Test framework: !`find . -name "*.test.*" -o -name "*.spec.*" | head -3` existing tests
- Target code: Analysis of $ARGUMENTS for testability assessment
- Integration complexity: Assessment of component interactions and dependencies

## Task

Execute comprehensive integration testing plan with testability analysis:

**Planning Focus**: Use $ARGUMENTS to specify target code, test type requirements, Rust inline testing, or refactoring suggestions

**Integration Testing Framework**:
1. **Code Testability Analysis** - Analyze target code structure, identify testing challenges, assess coupling levels, evaluate dependency injection
2. **Test Strategy Design** - Design integration test approach, plan inline vs separate test files, identify test boundaries, optimize test isolation
3. **Refactoring Assessment** - Identify testability improvements, suggest dependency injection, recommend interface abstractions, optimize component boundaries
4. **Test Case Planning** - Design integration scenarios, identify critical paths, plan data flow testing, assess error handling coverage
5. **Mock Strategy** - Plan external dependency mocking, design test doubles, identify integration boundaries, optimize test performance
6. **Execution Planning** - Design test execution order, plan test data management, optimize test environment setup, ensure test isolation

**Advanced Features**: Rust-style inline testing, property-based integration tests, contract testing, service virtualization, chaos engineering integration.

**Quality Assurance**: Test maintainability, execution performance, coverage optimization, feedback loop efficiency.

**Output**: Comprehensive integration test plan with test case specifications, refactoring recommendations, implementation strategy, and quality metrics.

