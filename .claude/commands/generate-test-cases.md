---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality", "testing"]
description: Generate Test Cases
allowed-tools: Read, Write, Edit, Bash
argument-hint: [target] | [scope] | --unit | --integration | --edge-cases | --automatic

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["test-case-generation", "code-analysis", "testing-strategies"]
    complexity-factors: ["automatic-test-generation", "edge-case-discovery", "coverage-optimization"]
    specialized-tools: ["test-generation", "code-analysis", "coverage-analysis"]
  preferred-agents:
    primary: "mock-test-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["FileScopeMCP", "desktop-commander", "cipher-memory"]
    specialized-functions: ["test-generation", "code-analysis"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "test-case-generation + code-analysis + testing-strategies"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "testing-patterns + code-analysis-knowledge"
    
    knowledge-preparation:
      - domain: "test-generation"
      - pattern-search: "testing-strategies + code-patterns + coverage-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["code-analysis", "test-generation", "coverage-optimization"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "generation-strategies + testing-approaches + coverage-methods"
      - pattern-recognition: "test-generation-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["test-generation-results", "coverage-insights", "testing-techniques"]
      - knowledge-extraction: "testing-methodologies + generation-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["testing-relationships", "code-dependencies", "coverage-connections"]
      - cross-reference: "related-testing-strategies"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "testing-knowledge + generation-patterns"
      - continuous-learning: "test-generation-optimization"

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
    - code-analysis
    - test-generation
    - coverage-optimization
    - edge-case-discovery
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "generate-test-cases"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "test-generation-results + coverage-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["testing-patterns", "generation-techniques", "coverage-optimization-methods"]
  learn-from: ["generate-tests", "test-coverage", "setup-comprehensive-testing"]
  contribute-to: "testing-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-code-analysis-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-test-generation
    - continuous-memory-updates
    - real-time-coverage-tracking
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - testing-pattern-extraction
---

# Generate Test Cases

Generate comprehensive test cases with automatic analysis and intelligent coverage: **$ARGUMENTS**

## Current Test Generation Context

- Target code: Analysis of $ARGUMENTS for test case generation requirements
- Test framework: !`find . -name "jest.config.*" -o -name "*.test.*" | head -1 && echo "Jest/Vitest detected" || echo "Detect framework"`
- Code complexity: !`find . -name "*.js" -o -name "*.ts" -o -name "*.py" | xargs wc -l 2>/dev/null | tail -1 | awk '{print $1}' || echo "0"` lines of code
- Existing patterns: !`find . -name "*.test.*" -o -name "*.spec.*" | head -3` test file patterns

## Task

Execute intelligent test case generation with comprehensive coverage and optimization:

**Generation Scope**: Use $ARGUMENTS to specify target file, unit tests, integration tests, edge cases, or automatic comprehensive generation

**Test Case Generation Framework**:
1. **Code Structure Analysis** - Parse function signatures, analyze control flow, identify branching paths, assess complexity metrics
2. **Test Pattern Recognition** - Analyze existing test patterns, identify testing conventions, extract reusable patterns, optimize consistency
3. **Input Space Analysis** - Identify parameter domains, analyze boundary conditions, discover edge cases, evaluate error conditions
4. **Test Case Design** - Generate positive test cases, negative test cases, boundary value tests, equivalence class tests
5. **Mock Strategy Planning** - Identify external dependencies, design mock implementations, create test data factories, optimize test isolation
6. **Coverage Optimization** - Ensure path coverage, optimize test efficiency, eliminate redundancy, maximize testing value

**Advanced Features**: Automatic edge case discovery, intelligent input generation, test data synthesis, coverage gap analysis, performance test generation.

**Quality Assurance**: Test maintainability, execution performance, assertion quality, debugging effectiveness.

**Output**: Comprehensive test case suite with optimized coverage, intelligent mocking, proper assertions, and maintenance guidelines.

