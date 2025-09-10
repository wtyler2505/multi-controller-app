---
model: claude-sonnet-4-20250514
category: architecture-design
priority: high
tags: ["architecture-design", "decision-analysis"]
description: Decision Tree Explorer
allowed-tools: Read, Write, Edit, WebSearch
argument-hint: [decision-context] | --strategic | --investment | --operational | --crisis-response

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["decision-analysis", "strategic-planning", "risk-assessment"]
    complexity-factors: ["decision-complexity", "stakeholder-involvement", "analysis-depth"]
    specialized-tools: ["decision-frameworks", "analysis-tools", "reasoning-systems"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "decision-analyst"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["clear-thought", "cipher-memory", "perplexity-ask"]
    specialized-functions: ["decision-analysis", "strategic-reasoning"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "decision-analysis + strategic-planning + risk-assessment"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "decision-patterns + strategic-knowledge"
    
    knowledge-preparation:
      - domain: "decision-analysis"
      - pattern-search: "decision-strategies + analysis-patterns + risk-frameworks"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["decision-analysis", "option-evaluation", "risk-assessment"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "decision-strategies + analysis-approaches + evaluation-decisions"
      - pattern-recognition: "decision-analysis-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["decision-results", "analysis-insights", "strategic-techniques"]
      - knowledge-extraction: "decision-methodologies + analysis-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["decision-relationships", "analysis-dependencies", "strategic-connections"]
      - cross-reference: "related-decision-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "decision-knowledge + analysis-patterns"
      - continuous-learning: "decision-analysis-optimization"

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
    - decision-analysis
    - option-evaluation
    - risk-assessment
    - strategic-reasoning
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "decision-tree-explorer"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "decision-results + analysis-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["decision-patterns", "analysis-techniques", "strategic-methodologies"]
  learn-from: ["ultra-think", "architecture-scenario-explorer", "system-dynamics-modeler"]
  contribute-to: "decision-analysis-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-decision-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-decision-analysis
    - continuous-memory-updates
    - real-time-strategic-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - decision-pattern-extraction
---

# Decision Tree Explorer

Explore complex decision scenarios with comprehensive probability analysis, risk assessment, and intelligent optimization: **$ARGUMENTS**

## Current Decision Context

- Decision scope: Based on $ARGUMENTS (strategic, investment, operational, crisis response)
- Available options: Current alternatives under consideration
- Success criteria: Key metrics for decision evaluation
- Resource constraints: Limitations affecting available choices

## Task

Create comprehensive decision tree analysis for optimal choice selection:

**Decision Context**: Use $ARGUMENTS to analyze strategic decisions, investments, operations, or crisis responses

**Decision Framework**:
1. **Option Generation** - Comprehensive alternative identification including hybrid and innovative approaches
2. **Probability Assessment** - Systematic likelihood estimation using base rates, expert judgment, and market data
3. **Expected Value Analysis** - Multi-dimensional value calculation including financial, strategic, and risk factors
4. **Sensitivity Analysis** - Critical assumption testing and break-even analysis
5. **Risk Assessment** - Comprehensive risk identification, impact analysis, and mitigation strategies
6. **Optimization Engine** - Multi-criteria decision analysis with stakeholder preference integration

**Advanced Analytics**: Monte Carlo simulations, real options valuation, decision path optimization, and robustness testing.

**Implementation Integration**: Connect analysis to specific actions, success metrics, and contingency planning.

**Output**: Complete decision tree with probability-weighted outcomes, expected value calculations, risk assessments, and strategic recommendations with implementation guidance.

