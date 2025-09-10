---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality", "testing"]
description: Write Tests
allowed-tools: Read, Write, Edit, Bash
argument-hint: [target-file] | [test-type] | --unit | --integration | --e2e | --component

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["test-writing", "testing-frameworks", "test-optimization"]
    complexity-factors: ["test-complexity", "framework-integration", "coverage-requirements"]
    specialized-tools: ["test-generation", "framework-setup", "test-optimization"]
  preferred-agents:
    primary: "mock-test-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["FileScopeMCP", "desktop-commander", "cipher-memory"]
    specialized-functions: ["test-writing", "framework-integration"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "test-writing + testing-frameworks + test-optimization"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "testing-patterns + framework-knowledge"
    
    knowledge-preparation:
      - domain: "test-writing"
      - pattern-search: "testing-strategies + framework-patterns + optimization-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["test-analysis", "framework-setup", "test-generation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "testing-strategies + framework-approaches + optimization-decisions"
      - pattern-recognition: "test-writing-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["testing-results", "framework-insights", "optimization-techniques"]
      - knowledge-extraction: "testing-methodologies + framework-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["testing-relationships", "framework-dependencies", "optimization-connections"]
      - cross-reference: "related-testing-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "testing-knowledge + framework-patterns"
      - continuous-learning: "test-writing-optimization"

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
    - test-analysis
    - framework-setup
    - test-generation
    - optimization-application
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "write-tests"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "testing-results + framework-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["testing-patterns", "framework-techniques", "optimization-methodologies"]
  learn-from: ["generate-tests", "test-coverage", "setup-comprehensive-testing"]
  contribute-to: "testing-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-testing-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-test-analysis
    - continuous-memory-updates
    - real-time-optimization-application
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - testing-pattern-extraction
---

# Write Tests

Write comprehensive unit and integration tests with framework-specific best practices and intelligent optimization: **$ARGUMENTS**

## Current Testing Context

- Test framework: !`find . -name "jest.config.*" -o -name "*.test.*" | head -1 && echo "Jest/Vitest detected" || echo "Detect framework"`
- Target file: Analysis of $ARGUMENTS for test requirements and complexity
- Project patterns: !`find . -name "*.test.*" -o -name "*.spec.*" | head -3` existing test patterns
- Coverage setup: !`grep -l "coverage" package.json jest.config.* 2>/dev/null | head -1 || echo "Setup needed"`

## Task

Execute comprehensive test writing with framework-specific optimizations and best practices:

**Test Focus**: Use $ARGUMENTS to specify target file, unit tests, integration tests, e2e tests, or component tests

**Test Writing Framework**:
1. **Code Analysis** - Analyze target code structure, identify testable functions, assess dependency complexity, evaluate edge cases
2. **Test Strategy Design** - Plan test organization, design test hierarchies, identify mock requirements, optimize test isolation
3. **Framework Integration** - Setup framework-specific patterns, configure test utilities, implement proper assertions, optimize test performance
4. **Mock Implementation** - Design dependency mocks, implement test doubles, create factory functions, setup async handling
5. **Test Case Generation** - Write unit tests, integration tests, edge cases, error scenarios, performance tests, snapshot tests
6. **Quality Assurance** - Ensure test maintainability, optimize execution speed, validate coverage, implement proper cleanup

**Advanced Features**: Property-based testing, contract testing, visual regression testing, accessibility testing, performance benchmarking.

**Framework Support**: Jest/Vitest, React Testing Library, Vue Test Utils, Angular TestBed, Cypress, Playwright integration.

**Output**: Comprehensive test suite with unit tests, integration tests, proper mocking, test utilities, and coverage optimization.

