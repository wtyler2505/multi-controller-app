---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality", "testing", "analysis"]
description: Test Quality Analyzer
allowed-tools: Read, Write, Edit, Bash
argument-hint: [analysis-type] | --coverage-quality | --test-effectiveness | --maintainability | --performance-analysis

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["test-quality-analysis", "coverage-assessment", "maintainability-evaluation"]
    complexity-factors: ["quality-metrics", "effectiveness-analysis", "performance-assessment"]
    specialized-tools: ["quality-analysis", "coverage-evaluation", "maintainability-assessment"]
  preferred-agents:
    primary: "mock-test-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["FileScopeMCP", "desktop-commander", "cipher-memory"]
    specialized-functions: ["quality-analysis", "coverage-assessment"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "test-quality-analysis + coverage-assessment + maintainability-evaluation"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "quality-patterns + analysis-knowledge"
    
    knowledge-preparation:
      - domain: "test-quality-analysis"
      - pattern-search: "quality-strategies + analysis-patterns + assessment-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["quality-analysis", "coverage-assessment", "maintainability-evaluation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "quality-strategies + analysis-approaches + assessment-decisions"
      - pattern-recognition: "test-quality-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["quality-analysis-results", "coverage-insights", "maintainability-techniques"]
      - knowledge-extraction: "quality-methodologies + analysis-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["quality-relationships", "analysis-dependencies", "assessment-connections"]
      - cross-reference: "related-quality-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "quality-knowledge + analysis-patterns"
      - continuous-learning: "test-quality-optimization"

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
    - quality-analysis
    - coverage-assessment
    - maintainability-evaluation
    - performance-analysis
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "test-quality-analyzer"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "quality-analysis-results + coverage-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["quality-patterns", "analysis-techniques", "assessment-methodologies"]
  learn-from: ["test-coverage", "generate-tests", "setup-comprehensive-testing"]
  contribute-to: "testing-quality-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-quality-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-quality-analysis
    - continuous-memory-updates
    - real-time-assessment-tracking
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - quality-pattern-extraction
---

# Test Quality Analyzer

Analyze test suite quality with comprehensive metrics and actionable improvement insights: **$ARGUMENTS**

## Current Quality Context

- Test coverage: !`find . -name "coverage" -type d | head -1 && echo "Coverage data available" || echo "No coverage data"`
- Test files: !`find . -name "*.test.*" -o -name "*.spec.*" | wc -l` test files
- Test complexity: Analysis of test suite maintainability and effectiveness patterns
- Performance metrics: Current test execution times and resource utilization

## Task

Execute comprehensive test quality analysis with improvement recommendations and optimization strategies:

**Analysis Type**: Use $ARGUMENTS to focus on coverage quality, test effectiveness, maintainability analysis, or performance analysis

**Test Quality Analysis Framework**:
1. **Coverage Quality Assessment** - Analyze coverage depth, evaluate coverage quality, assess edge case handling, identify coverage gaps
2. **Test Effectiveness Evaluation** - Measure defect detection capability, analyze test reliability, assess assertion quality, evaluate test value
3. **Maintainability Analysis** - Evaluate test code quality, analyze test organization, assess refactoring needs, optimize test structure
4. **Performance Assessment** - Analyze execution performance, identify bottlenecks, optimize test speed, reduce resource consumption
5. **Anti-Pattern Detection** - Identify testing anti-patterns, detect flaky tests, analyze test smells, recommend corrections
6. **Quality Metrics Tracking** - Implement quality scoring, track improvement trends, configure quality gates, optimize quality processes

**Advanced Features**: AI-powered quality assessment, predictive quality modeling, automated improvement suggestions, quality trend analysis, benchmark comparison.

**Quality Insights**: Test ROI analysis, quality correlation analysis, maintenance cost assessment, effectiveness benchmarking.

**Output**: Comprehensive quality analysis with detailed metrics, improvement recommendations, optimization strategies, and quality tracking framework.

