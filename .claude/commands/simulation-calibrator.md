---
model: claude-sonnet-4-20250514
category: architecture-design
priority: high
tags: ["architecture-design"]
description: Simulation Calibrator
allowed-tools: Read, Write, Edit, WebSearch
argument-hint: [simulation-type] | --business | --technical | --behavioral | --strategic

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["simulation-modeling", "calibration-techniques", "validation-strategies"]
    complexity-factors: ["accuracy-optimization", "bias-detection", "continuous-improvement"]
    specialized-tools: ["simulation-calibration", "validation-analysis", "accuracy-optimization"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "system-behavior-simulator"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["clear-thought", "perplexity-ask", "cipher-memory"]
    specialized-functions: ["simulation-calibration", "validation-analysis"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "simulation-modeling + calibration-techniques + validation-strategies"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "simulation-patterns + calibration-knowledge"
    
    knowledge-preparation:
      - domain: "simulation-calibration"
      - pattern-search: "calibration-strategies + validation-techniques + accuracy-patterns"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["calibration-analysis", "validation-execution", "accuracy-optimization"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "calibration-strategies + validation-approaches + optimization-techniques"
      - pattern-recognition: "simulation-calibration-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["calibration-results", "validation-insights", "accuracy-techniques"]
      - knowledge-extraction: "calibration-methodologies + validation-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["calibration-relationships", "validation-dependencies", "accuracy-connections"]
      - cross-reference: "related-simulation-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "simulation-knowledge + calibration-patterns"
      - continuous-learning: "calibration-process-optimization"

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
    - calibration-analysis
    - validation-execution
    - accuracy-optimization
    - bias-detection
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "simulation-calibrator"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "calibration-results + validation-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["calibration-patterns", "validation-techniques", "accuracy-optimization-methods"]
  learn-from: ["system-behavior-simulator", "architecture-scenario-explorer", "system-dynamics-modeler"]
  contribute-to: "simulation-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-simulation-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-calibration-analysis
    - continuous-memory-updates
    - real-time-accuracy-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - calibration-pattern-extraction
---

# Simulation Calibrator

Calibrate simulation accuracy with comprehensive validation and continuous improvement: **$ARGUMENTS**

## Current Simulation State

- Simulation type: Based on $ARGUMENTS (business, technical, behavioral, strategic simulation)
- Accuracy requirements: Mission-critical (95%+), strategic (80-95%), or exploratory (50-70%)
- Validation data: Historical outcomes, real-world benchmarks, and expert assessments
- Performance metrics: Current accuracy levels and improvement opportunities

## Task

Implement systematic simulation calibration with comprehensive accuracy improvement:

**Simulation Type**: Use $ARGUMENTS to calibrate business simulations, technical models, behavioral predictions, or strategic scenarios

**Calibration Framework**:
1. **Baseline Assessment** - Historical validation, accuracy metrics, and error pattern analysis
2. **Bias Detection** - Systematic identification of cognitive, data, and model biases with mitigation strategies
3. **Validation Loops** - Multi-level validation with internal consistency, expert review, and empirical testing
4. **Real-Time Calibration** - Continuous monitoring, automated adjustments, and adaptive learning integration
5. **Quality Assurance** - Meta-calibration assessment and improvement sustainability
6. **Improvement Roadmap** - Systematic enhancement strategies with performance tracking

**Advanced Features**: Automated bias detection, machine learning calibration, cross-simulation learning, and predictive accuracy optimization.

**Quality Control**: Independent validation, benchmark comparison, and comprehensive documentation for institutional learning.

**Output**: Calibrated simulation with validated accuracy metrics, bias correction reports, continuous improvement systems, and enhanced decision support reliability.

