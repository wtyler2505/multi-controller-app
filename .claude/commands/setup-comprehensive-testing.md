---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality"]
description: Setup comprehensive testing infrastructure
allowed-tools: Read, Write, Edit, Bash
argument-hint: [test-type] | --unit | --integration | --e2e | --performance | --full-suite

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["testing-validation", "test-automation", "quality-assurance"]
    complexity-factors: ["multi-layer-testing", "test-framework-integration", "performance-validation"]
    specialized-tools: ["test-automation", "mock-testing", "performance-monitoring"]
  preferred-agents:
    primary: "mock-test-orchestrator"
    secondary: "cargo-build-engineer"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "context7", "cipher-memory"]
    specialized-functions: ["test-automation", "quality-assurance"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "testing-validation + test-automation + quality-assurance"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "testing-patterns + automation-knowledge"
    
    knowledge-preparation:
      - domain: "testing-quality"
      - pattern-search: "testing-strategies + automation-patterns + quality-assurance"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["testing-analysis", "framework-setup", "automation-configuration"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "testing-strategies + framework-choices + automation-decisions"
      - pattern-recognition: "testing-setup-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["testing-results", "automation-insights", "quality-techniques"]
      - knowledge-extraction: "testing-methodologies + automation-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["testing-relationships", "framework-dependencies", "quality-connections"]
      - cross-reference: "related-testing-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "testing-knowledge + automation-patterns"
      - continuous-learning: "testing-setup-optimization"

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
    - testing-analysis
    - framework-setup
    - automation-configuration
    - quality-validation
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "setup-comprehensive-testing"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "testing-setup-results + quality-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["testing-patterns", "automation-techniques", "quality-strategies"]
  learn-from: ["generate-tests", "test-coverage", "test-automation-orchestrator"]
  contribute-to: "testing-quality-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-project-structure
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-framework-setup
    - continuous-memory-updates
    - real-time-quality-validation
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - testing-pattern-extraction

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-testing-workflow"
  
  base-tools:
    - "mcp__desktop-commander__start_process"
    - "mcp__FileScopeMCP__find_important_files"
    - "mcp__context7__get-library-docs"
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
  
  conditional-tools:
    rust-project:
      - "mcp__desktop-commander__start_process"  # For cargo test, tarpaulin
      - "mcp__FileScopeMCP__find_important_files"  # Find test files
      - "mcp__context7__get-library-docs"  # Rust testing docs
      - "mcp__taskmaster-ai__add_task"  # Create testing tasks
      - "mcp__cipher-memory__create_entities"  # Store Rust test patterns
    
    javascript-project:
      - "mcp__desktop-commander__start_process"  # For npm test
      - "mcp__FileScopeMCP__find_important_files"  # Find test files
      - "mcp__context7__get-library-docs"  # Jest/Vitest docs
      - "mcp__taskmaster-ai__add_task"  # Create testing tasks
      - "mcp__cipher-memory__add_observations"  # Enhance JS testing patterns
    
    complex-setup:
      - "mcp__desktop-commander__start_process"
      - "mcp__FileScopeMCP__recalculate_importance"
      - "mcp__context7__get-library-docs"
      - "mcp__cipher-memory__open_nodes"  # Load testing infrastructure patterns
      - "mcp__cipher-memory__create_relations"  # Connect testing strategies
    
    performance-testing:
      - "mcp__desktop-commander__start_process"  # Performance test execution
      - "mcp__cipher-memory__add_observations"  # Store performance insights

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    testing-patterns-search:
      - query-pattern: "comprehensive-testing + test-infrastructure + ${language}-testing"
      - tools: ["mcp__cipher-memory__search_nodes"]
    framework-analysis:
      - query-pattern: "testing-frameworks + ${project_type}-testing + quality-assurance"
      - tools: ["mcp__cipher-memory__open_nodes"]
    best-practices-load:
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "testing-related"
  execution-memory:
    setup-progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "testing-framework-configured"
    pattern-discovery:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "successful-test-setup"
    optimization-learning:
      - tool: "mcp__cipher-memory__create_entities"
      - trigger: "performance-benchmark-established"
  post-execution-memory:
    infrastructure-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-testing-infrastructure-pattern"
    methodology-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["test-type-to-framework", "coverage-to-quality", "automation-to-efficiency"]
    knowledge-synthesis:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "testing-best-practices + automation-strategies + quality-metrics"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - testing-context-analysis
      - framework-selection-rationale
      - memory-pattern-analysis
    
    execution:
      - framework-installation
      - test-configuration
      - coverage-setup
      - automation-implementation
      - performance-benchmarking
    
    post-execution:
      - testing-infrastructure-validation
      - coverage-metrics
      - automation-status
      - memory-operations
      - quality-assessment
  
  structured-metadata:
    command-id: "setup-comprehensive-testing"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    testing-scope: "${testing_arguments}"
    frameworks-configured: "${test_frameworks}"
    coverage-achieved: "${coverage_percentage}"
    automation-level: "${automation_status}"
    execution-time: "${duration_ms}"
    quality-metrics: "${testing_quality_score}"

# Enhanced workflow configuration
tool-chain: "universal-testing-infrastructure"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
performance-monitoring: true

allowed-tools: Read, Write, Edit, Bash, mcp__desktop-commander__*, mcp__FileScopeMCP__*, mcp__context7__*, mcp__taskmaster-ai__*, mcp__cipher-memory__*

argument-hint: [scope] | --unit | --integration | --e2e | --visual | --performance | --full-stack

pre-execution:
  validate-tools: true
  load-context: true
  analyze-arguments: true
  detect-project-state: true
  prepare-environment: true
  search-testing-patterns: true
  log-session-start: true

post-execution:
  store-results: true
  update-learning: true
  generate-report: true
  log-performance: true
  persist-testing-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Setup Comprehensive Testing (Universal Integration)

Setup complete testing infrastructure with intelligent pattern recognition, automated framework selection, and persistent learning: **$ARGUMENTS**

**ENHANCED WORKFLOW**: This command utilizes specialized testing agents (mock-test-orchestrator, cargo-build-engineer, rust-performance-monitor) with complete Cipher Memory integration for testing pattern recognition and quality assurance learning persistence.

## Current Testing Infrastructure

- Project type: !`[ -f package.json ] && echo "Node.js" || [ -f requirements.txt ] && echo "Python" || [ -f pom.xml ] && echo "Java" || echo "Multi-language"`
- Existing tests: !`find . -name "*.test.*" -o -name "*.spec.*" | wc -l` test files
- CI system: !`find . -name ".github" -o -name ".gitlab-ci.yml" -o -name "Jenkinsfile" | head -1 || echo "No CI detected"`
- Framework: !`grep -l "jest\\|vitest\\|pytest\\|junit" package.json requirements.txt pom.xml 2>/dev/null | head -1 || echo "Detect framework"`

## Task

Implement comprehensive testing infrastructure with multi-layer testing strategy:

**Setup Scope**: Use $ARGUMENTS to focus on unit, integration, e2e, visual, performance testing, or full-stack implementation

**Comprehensive Testing Framework**:
1. **Testing Strategy Design** - Analyze project requirements, define testing pyramid, plan coverage goals, optimize testing investment
2. **Unit Testing Setup** - Configure primary framework (Jest, Vitest, pytest), setup test runners, implement test utilities, optimize execution
3. **Integration Testing** - Setup integration test framework, configure test databases, implement API testing, optimize test isolation
4. **E2E Testing Configuration** - Setup browser testing (Cypress, Playwright), configure test environments, implement page objects
5. **Visual & Performance Testing** - Setup visual regression testing, configure performance benchmarks, implement accessibility testing
6. **CI/CD Integration** - Configure automated test execution, setup parallel testing, implement quality gates, optimize pipeline performance

**Advanced Features**: Contract testing, chaos engineering, load testing, security testing, cross-browser testing, mobile testing.

**Infrastructure Quality**: Test reliability, execution performance, maintainability, scalability, cost optimization.

**Output**: Complete testing infrastructure with configured frameworks, CI integration, quality metrics, and maintenance workflows.

## Enhanced Pre-Execution Memory Analysis
Before infrastructure setup, the system will:
1. **Search testing patterns**: Query Cipher Memory for comprehensive testing configurations
2. **Load framework knowledge**: Retrieve successful testing infrastructure patterns
3. **Analyze quality metrics**: Understanding testing effectiveness patterns
4. **Connect to Task Master**: Integration for testing task automation

## Universal Memory Integration Outcomes

### Testing Knowledge Storage
This command will automatically:
- **Store successful testing configurations** in Cipher Memory for framework optimization
- **Create relationships** between testing types, frameworks, and quality outcomes
- **Document performance benchmarks** and testing best practices
- **Build knowledge graph** of testing infrastructure patterns

### Cross-Command Learning Enhancement
Testing infrastructure patterns will improve:
- Future setup commands through shared testing knowledge
- Quality assurance optimization via pattern recognition
- Framework selection intelligence through success analytics
- Error prevention through documented testing pitfalls

### Advanced Testing Intelligence
- **Framework Auto-Selection**: Based on project type and historical success patterns
- **Coverage Optimization**: Intelligent test coverage recommendations from memory
- **Performance Benchmarking**: Automated baseline establishment and tracking
- **Quality Gate Intelligence**: Smart quality thresholds based on project patterns

### Centralized Testing Logging
All testing operations logged to `.claude/logs/command-execution.jsonl` including:
- Framework installation and configuration results
- Coverage metrics and quality assessments
- Performance benchmark establishment
- Memory operations for testing knowledge capture
- Automation status and optimization recommendations

**Next Commands**: Enhanced testing patterns will automatically improve commands like `generate-tests`, `test-coverage`, and `setup-monitoring-observability`.

