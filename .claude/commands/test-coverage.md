---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality", "testing", "coverage-analysis", "quality-metrics", "test-optimization"]
description: Comprehensive test coverage analysis and optimization with universal memory integration and intelligent pattern recognition

# Enhanced Context-Aware Agent Integration with Universal Memory
agent-selection:
  type: "context-aware"
  domain-hints: ["coverage-analysis", "test-optimization", "quality-metrics", "testing-assessment", "gap-analysis"]
  complexity-level: "standard"
  
  # Enhanced selection criteria for coverage analysis with memory integration
  selection-criteria:
    keyword-match: 0.90       # Strong coverage/testing patterns
    argument-analysis: 0.85   # Coverage type context important
    project-context: 0.90     # Language/framework affects coverage tools
    error-context: 0.6        # May include testing gaps
  
  # Specialized testing agents with memory capabilities
  preferred-agents: ["mock-test-orchestrator", "cargo-build-engineer", "general-purpose"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.85

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-coverage-analysis-workflow"
  
  base-tools:
    - "mcp__desktop-commander__start_process"  # Run coverage tools
    - "mcp__FileScopeMCP__find_important_files"  # Analyze test files
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
  
  conditional-tools:
    rust-coverage:
      - "mcp__desktop-commander__start_process"  # cargo tarpaulin
      - "mcp__context7__get-library-docs"  # Rust coverage best practices
      - "mcp__cipher-memory__open_nodes"  # Load Rust coverage patterns
    
    javascript-coverage:
      - "mcp__desktop-commander__start_process"  # jest --coverage, nyc
      - "mcp__context7__get-library-docs"  # Jest/NYC documentation
      - "mcp__cipher-memory__create_entities"  # Store JS coverage patterns
    
    coverage-analysis:
      - "mcp__cipher-memory__add_observations"  # Store coverage insights
      - "mcp__desktop-commander__start_process"  # Coverage report generation
      - "mcp__cipher-memory__create_relations"  # Connect coverage patterns
    
    gap-analysis:
      - "mcp__FileScopeMCP__recalculate_importance"  # Critical uncovered files
      - "mcp__cipher-memory__create_entities"  # Store gap analysis patterns
      - "mcp__taskmaster-ai__add_task"  # Create coverage improvement tasks

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    coverage-patterns-search:
      - query-pattern: "test-coverage + coverage-analysis + ${language}-coverage"
      - tools: ["mcp__cipher-memory__search_nodes"]
    quality-benchmarks:
      - query-pattern: "coverage-thresholds + quality-metrics + ${project_type}"
      - tools: ["mcp__cipher-memory__open_nodes"]
    testing-strategies:
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "coverage-testing-related"
  execution-memory:
    analysis-progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "coverage-metric-calculation"
    gap-identification:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "coverage-gap-identified"
    improvement-learning:
      - tool: "mcp__cipher-memory__create_entities"
      - trigger: "coverage-improvement-strategy"
  post-execution-memory:
    coverage-analysis-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-coverage-analysis-session-pattern"
    quality-pattern-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["coverage-to-quality", "gap-to-risk", "improvement-to-effectiveness"]
    knowledge-enhancement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "coverage-insights + quality-optimization + testing-effectiveness"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - coverage-analysis-scope
      - quality-benchmark-search
      - memory-pattern-analysis
    
    execution:
      - coverage-tool-execution
      - metric-calculation
      - gap-analysis
      - threshold-evaluation
      - report-generation
    
    post-execution:
      - coverage-summary
      - improvement-recommendations
      - memory-operations
      - quality-assessment
  
  structured-metadata:
    command-id: "test-coverage"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    coverage-type: "${coverage_arguments}"
    line-coverage: "${line_coverage_percentage}"
    branch-coverage: "${branch_coverage_percentage}"
    function-coverage: "${function_coverage_percentage}"
    uncovered-lines: "${uncovered_lines_count}"
    critical-gaps: "${critical_gap_count}"
    execution-time: "${duration_ms}"
    analysis-quality-score: "${coverage_analysis_effectiveness}"

# Enhanced workflow configuration
tool-chain: "universal-coverage-analysis-workflow"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
coverage-pattern-recognition: true

allowed-tools: Read, Write, Edit, Bash, mcp__desktop-commander__*, mcp__FileScopeMCP__*, mcp__context7__*, mcp__cipher-memory__*, mcp__taskmaster-ai__*

argument-hint: [coverage-type] | --line | --branch | --function | --statement | --report | --comprehensive

pre-execution:
  validate-tools: true
  load-context: true
  analyze-test-framework: true
  search-coverage-patterns: true
  log-session-start: true

post-execution:
  store-results: true
  update-learning: true
  generate-report: true
  persist-coverage-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Test Coverage (Universal Integration)

Analyze and improve test coverage with universal memory integration and intelligent pattern recognition: **$ARGUMENTS**

**ENHANCED WORKFLOW**: This command utilizes specialized testing agents (mock-test-orchestrator, cargo-build-engineer) with complete Cipher Memory integration for coverage pattern recognition, quality assessment, and testing optimization persistence.

## Enhanced Pre-Execution Memory Analysis
Before coverage analysis, the system will:
1. **Search coverage patterns**: Query Cipher Memory for effective coverage analysis strategies and quality thresholds
2. **Load quality benchmarks**: Retrieve coverage benchmarks and testing effectiveness patterns
3. **Analyze gap strategies**: Understanding coverage gap analysis and improvement methodologies
4. **Connect testing knowledge**: Access comprehensive testing optimization and quality assurance patterns

## Current Coverage Context

- Test framework: !`find . -name "jest.config.*" -o -name ".nycrc*" -o -name "coverage.xml" | head -1 || echo "Detect framework"`
- Coverage tools: !`npm ls nyc jest @jest/core 2>/dev/null | grep -E "nyc|jest" | head -2 || echo "No JS coverage tools"`
- Existing coverage: !`find . -name "coverage" -type d | head -1 && echo "Coverage data exists" || echo "No coverage data"`
- Test files: !`find . -name "*.test.*" -o -name "*.spec.*" | wc -l` test files

## Task

Execute comprehensive coverage analysis with improvement recommendations and reporting:

**Coverage Type**: Use $ARGUMENTS to focus on line coverage, branch coverage, function coverage, statement coverage, or comprehensive reporting

**Coverage Analysis Framework**:
1. **Coverage Tool Setup** - Configure appropriate tools (Jest, NYC, Istanbul, Coverage.py, JaCoCo), setup collection settings, optimize performance, enable reporting
2. **Coverage Measurement** - Generate line coverage, branch coverage, function coverage, statement coverage reports, identify uncovered code paths
3. **Gap Analysis** - Identify critical uncovered paths, analyze coverage quality, assess business logic coverage, evaluate edge case handling
4. **Threshold Management** - Configure coverage thresholds, implement quality gates, setup trend monitoring, enforce minimum standards
5. **Reporting & Visualization** - Generate detailed reports, create coverage dashboards, implement trend analysis, setup automated notifications
6. **Improvement Planning** - Prioritize coverage gaps, recommend test additions, identify refactoring opportunities, plan coverage enhancement

**Advanced Features**: Differential coverage analysis, coverage trend monitoring, integration with code review, automated coverage alerts, performance impact assessment.

**Quality Insights**: Coverage quality assessment, test effectiveness analysis, maintainability correlation, risk area identification.

**Output**: Comprehensive coverage analysis with detailed reports, gap identification, improvement recommendations, and quality metrics tracking.

## Universal Memory Integration Outcomes

### Coverage Analysis Knowledge Storage
This command will automatically:
- **Store comprehensive coverage analysis sessions** in Cipher Memory for coverage pattern recognition
- **Create relationships** between coverage metrics, testing strategies, and quality outcomes
- **Document coverage methodologies** and gap analysis best practices
- **Build knowledge graph** of coverage-quality mappings and testing optimization strategies

### Cross-Command Learning Enhancement
Coverage analysis patterns will improve:
- Future testing commands through established coverage baseline patterns
- Test generation commands via documented coverage gap analysis
- Quality assurance commands through coverage quality integration
- Setup commands via proven testing effectiveness methodologies

### Advanced Coverage Intelligence
- **Gap Prioritization**: Automatic identification of critical coverage gaps based on code importance and risk analysis
- **Quality Correlation**: Intelligent correlation between coverage metrics and code quality indicators
- **Threshold Optimization**: Smart coverage threshold recommendations based on project characteristics and quality goals
- **Improvement Strategies**: Automated generation of coverage improvement plans based on gap analysis patterns

### Intelligent Analysis Enhancement Features
- **Language-Specific Analysis**: Tailored coverage analysis approaches based on programming language and testing framework
- **Context-Aware Reporting**: Smart coverage reporting considering project complexity and business criticality
- **Progressive Coverage Learning**: Each analysis session improves future coverage assessments through pattern accumulation
- **Cross-Project Coverage Knowledge**: Shared coverage insights across different codebases and testing strategies

### Centralized Coverage Analysis Logging
All coverage analysis operations logged to `.claude/logs/command-execution.jsonl` including:
- Complete analysis methodology and tool execution tracking
- Coverage metric calculation results and gap identification
- Memory operations for coverage pattern capture and learning
- Quality assessment results and improvement recommendation effectiveness

**Next Commands**: Enhanced coverage patterns will automatically improve commands like `generate-tests`, `setup-comprehensive-testing`, `code-review`, and `project-health-check`.

