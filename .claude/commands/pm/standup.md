---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "standup-reporting", "progress-aggregation", "team-communication"]
description: Standup Operations - Automated standup report generation with progress aggregation and team communication

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["standup-reporting", "progress-aggregation", "report-generation"]
  complexity-level: "medium"
  selection-criteria:
    keyword-match: 0.85
    argument-analysis: 0.90
    project-context: 0.80
  preferred-agents: ["general-purpose"]
  fallback-agents: ["task-orchestrator"]
  confidence-threshold: 0.75

# Universal Cipher Memory Integration (MANDATORY)
cipher-memory-integration:
  enabled: true
  priority: "high"
  pre-execution-memory:
    context-search:
      - query-pattern: "standup reporting + progress aggregation + report generation"
      - search-depth: "comprehensive"
      - max-results: 10
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - standup-history: "mcp__cipher-memory__search_nodes"
      - reporting-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    standup-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - standup-summary: "mcp__cipher-memory__create_entities"
      - reporting-patterns: "mcp__cipher-memory__create_entities"
      - aggregation-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - command-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - standup-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - standup-insights: "mcp__cipher-memory__create_entities"

# Universal Centralized Logging Integration (MANDATORY)
centralized-logging:
  enabled: true
  log-file: ".claude/execution-log.jsonl"
  log-components:
    execution-metadata: true
    agent-selection: true
    tool-chain: true
    memory-operations: true
    performance-metrics: true
    success-indicators: true
    standup-tracking: true
  logging-phases:
    pre-execution: true
    during-execution: true
    post-execution: true
    error-handling: true
  processing:
    real-time-write: true
    batch-processing: false
    error-recovery: true
    compression: false

# Cross-Command Learning Integration (MANDATORY)
cross-command-learning:
  enabled: true
  learning-domains: ["standup-reporting", "progress-aggregation", "report-generation"]
  pattern-sharing:
    success-patterns: "mcp__cipher-memory__create_entities"
    failure-patterns: "mcp__cipher-memory__create_entities"
    optimization-opportunities: "mcp__cipher-memory__add_observations"
  knowledge-synthesis:
    cross-domain-insights: "mcp__cipher-memory__create_relations"
    usage-pattern-analysis: "internal"
    performance-optimization: "internal"

# Workflow Integration (MANDATORY)
workflow-integration:
  enabled: true
  pre-execution:
    memory-context-loading: true
    cipher-search-patterns: true
    load-standup-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-standup-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

tool-chain: "standup-reporting-progress-aggregation"
auto-deploy: true
parallel-execution: false
allowed-tools: ["Bash", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph"]
---

Run `bash .claude/scripts/pm/standup.sh` using a sub-agent and show me the complete output.

- DO NOT truncate.
- DO NOT collapse.
- DO NOT abbreviate.
- Show ALL lines in full.
- DO NOT print any other comments.



