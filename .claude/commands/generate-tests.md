---
model: claude-sonnet-4-20250514
category: testing-quality
priority: critical
tags: ["testing-quality", "testing", "test-generation", "mock-testing", "test-automation"]
description: Automatically generate comprehensive test suites with universal memory integration and intelligent pattern recognition

# Enhanced Context-Aware Agent Integration with Universal Memory
agent-selection:
  type: "context-aware"
  domain-hints: ["test-generation", "mock-testing", "test-automation", "quality-assurance", "rust-testing"]
  complexity-level: "complex"
  
  # Enhanced selection criteria for test generation with memory integration
  selection-criteria:
    keyword-match: 0.95       # Strong testing/generation patterns
    argument-analysis: 0.85   # File/component context critical
    project-context: 0.90     # Project type affects testing approach
    error-context: 0.6        # May include test failures
  
  # Specialized testing agents with memory capabilities
  preferred-agents: ["mock-test-orchestrator", "cargo-build-engineer", "rust-performance-monitor"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.90

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-test-generation-workflow"
  
  base-tools:
    - "mcp__desktop-commander__start_process"  # Run tests and build tools
    - "mcp__FileScopeMCP__find_important_files"  # Locate test-related files
    - "mcp__context7__get-library-docs"  # Testing framework docs
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
  
  conditional-tools:
    rust-testing:
      - "mcp__desktop-commander__start_process"  # cargo test, tarpaulin
      - "mcp__context7__get-library-docs"  # Rust testing best practices
      - "mcp__cipher-memory__open_nodes"  # Load Rust test patterns
      - "mcp__taskmaster-ai__add_task"  # Create testing tasks
    
    javascript-testing:
      - "mcp__desktop-commander__start_process"  # npm test, jest
      - "mcp__context7__get-library-docs"  # Jest/Vitest documentation
      - "mcp__cipher-memory__create_entities"  # Store JS test patterns
    
    mock-testing:
      - "mcp__cipher-memory__add_observations"  # Store mocking strategies
      - "mcp__desktop-commander__start_process"  # Mock framework setup
      - "mcp__cipher-memory__create_relations"  # Connect mock patterns
    
    performance-testing:
      - "mcp__desktop-commander__start_process"  # Performance test tools
      - "mcp__FileScopeMCP__recalculate_importance"  # Performance-critical files
      - "mcp__cipher-memory__create_entities"  # Store performance test patterns

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    testing-patterns-search:
      - query-pattern: "test-generation + ${language}-testing + mock-strategies"
      - tools: ["mcp__cipher-memory__search_nodes"]
    framework-methodology:
      - query-pattern: "testing-frameworks + test-patterns + ${project_type}-testing"
      - tools: ["mcp__cipher-memory__open_nodes"]
    best-practices-synthesis:
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "testing-related"
  execution-memory:
    generation-progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "test-suite-component-completion"
    pattern-discovery:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "effective-test-strategy-identified"
    coverage-learning:
      - tool: "mcp__cipher-memory__create_entities"
      - trigger: "comprehensive-test-coverage-achieved"
  post-execution-memory:
    test-methodology-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-test-generation-session-pattern"
    strategy-effectiveness-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["test-type-to-coverage", "mock-strategy-to-effectiveness", "framework-to-best-practice"]
    knowledge-enhancement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "testing-insights + generation-strategies + coverage-optimization"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - test-generation-scope-analysis
      - testing-pattern-search
      - memory-pattern-analysis
    
    execution:
      - code-analysis-results
      - test-suite-generation
      - mock-implementation
      - coverage-analysis
      - test-execution-validation
    
    post-execution:
      - generation-summary
      - coverage-metrics
      - memory-operations
      - testing-recommendations
  
  structured-metadata:
    command-id: "generate-tests"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    target-component: "${target_arguments}"
    testing-framework: "${detected_framework}"
    tests-generated: "${test_count}"
    coverage-achieved: "${coverage_percentage}"
    mock-strategies: "${mock_implementations}"
    test-types-created: "${test_categories}"
    execution-time: "${duration_ms}"
    generation-quality-score: "${test_generation_effectiveness}"

# Enhanced workflow configuration
tool-chain: "universal-testing-generation"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
test-pattern-recognition: true

allowed-tools: Read, Write, Edit, Bash, mcp__desktop-commander__*, mcp__FileScopeMCP__*, mcp__context7__*, mcp__cipher-memory__*, mcp__taskmaster-ai__*

argument-hint: [file-path] | [component-name] | --unit | --integration | --mock | --performance

pre-execution:
  validate-tools: true
  load-context: true
  prepare-environment: true
  search-testing-patterns: true
  log-session-start: true

post-execution:
  store-results: true
  update-tasks: false
  generate-report: true
  persist-testing-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Generate Tests (Universal Integration)

Generate comprehensive test suite with intelligent pattern recognition, automated mock generation, and persistent learning: $ARGUMENTS

**ENHANCED WORKFLOW**: This command utilizes specialized testing agents (mock-test-orchestrator, cargo-build-engineer, rust-performance-monitor) with complete Cipher Memory integration for test pattern recognition, mocking strategies, and test generation methodology persistence.

## Enhanced Pre-Execution Memory Analysis
Before test generation, the system will:
1. **Search testing patterns**: Query Cipher Memory for effective test generation strategies and patterns
2. **Load framework knowledge**: Retrieve testing framework best practices and methodologies  
3. **Analyze mock strategies**: Understanding mocking approaches and test isolation patterns
4. **Connect coverage knowledge**: Access comprehensive test coverage optimization strategies

## Current Testing Setup

- Test framework: @package.json or @jest.config.js or @vitest.config.js (detect framework)
- Existing tests: !`find . -name "*.test.*" -o -name "*.spec.*" | head -5`
- Test coverage: !`npm run test:coverage 2>/dev/null || echo "No coverage script"`
- Target file: @$ARGUMENTS (if file path provided)

## Task

I'll analyze the target code and create complete test coverage including:

1. Unit tests for individual functions and methods
2. Integration tests for component interactions  
3. Edge case and error handling tests
4. Mock implementations for external dependencies
5. Test utilities and helpers as needed
6. Performance and snapshot tests where appropriate

## Process

I'll follow these steps:

1. Analyze the target file/component structure
2. Identify all testable functions, methods, and behaviors
3. Examine existing test patterns in the project
4. Create test files following project naming conventions
5. Implement comprehensive test cases with proper setup/teardown
6. Add necessary mocks and test utilities
7. Verify test coverage and add missing test cases

## Test Types

### Unit Tests
- Individual function testing with various inputs
- Component rendering and prop handling
- State management and lifecycle methods
- Utility function edge cases and error conditions

### Integration Tests
- Component interaction testing
- API integration with mocked responses
- Service layer integration
- End-to-end user workflows

### Framework-Specific Tests
- **React**: Component testing with React Testing Library
- **Vue**: Component testing with Vue Test Utils
- **Angular**: Component and service testing with TestBed
- **Node.js**: API endpoint and middleware testing

## Testing Best Practices

### Test Structure
- Use descriptive test names that explain the behavior
- Follow AAA pattern (Arrange, Act, Assert)
- Group related tests with describe blocks
- Use proper setup and teardown for test isolation

### Mock Strategy
- Mock external dependencies and API calls
- Use factories for test data generation
- Implement proper cleanup for async operations
- Mock timers and dates for deterministic tests

### Coverage Goals
- Aim for 80%+ code coverage
- Focus on critical business logic paths
- Test both happy path and error scenarios
- Include boundary value testing

I'll adapt to your project's testing framework (Jest, Vitest, Cypress, etc.) and follow established patterns.

## Universal Memory Integration Outcomes

### Testing Knowledge Storage
This command will automatically:
- **Store comprehensive test generation sessions** in Cipher Memory for test pattern recognition
- **Create relationships** between test types, mocking strategies, and coverage effectiveness
- **Document testing methodologies** and framework best practices
- **Build knowledge graph** of test-generation-to-coverage mappings and mock implementation strategies

### Cross-Command Learning Enhancement
Test generation patterns will improve:
- Future development commands through testing-aware architecture recommendations
- Debugging commands via documented test failure mode analysis
- Code review commands through established testing coverage patterns
- Setup commands via testing framework integration knowledge

### Advanced Testing Intelligence
- **Mock Strategy Recognition**: Automatic identification of optimal mocking approaches based on component types
- **Coverage Optimization**: Intelligent test case generation focusing on critical business logic paths
- **Framework Selection**: Smart testing framework recommendations based on project characteristics
- **Test Pattern Generation**: Automated creation of comprehensive test suites using established patterns

### Intelligent Test Enhancement Features
- **Language-Specific Testing**: Tailored test generation approaches based on project language and framework
- **Context-Aware Mock Creation**: Smart mock implementations considering component dependencies and interactions
- **Progressive Test Learning**: Each test generation session improves future testing through pattern accumulation
- **Cross-Project Testing Knowledge**: Shared testing insights across different codebases and project types

### Centralized Testing Logging
All test generation operations logged to `.claude/logs/command-execution.jsonl` including:
- Complete test generation methodology and tool usage tracking
- Mock strategy selection and implementation results
- Coverage analysis results and test case effectiveness metrics
- Memory operations for testing pattern capture and learning

**Next Commands**: Enhanced testing patterns will automatically improve commands like `code-review`, `debug-error`, `setup-comprehensive-testing`, and `test-coverage`.

