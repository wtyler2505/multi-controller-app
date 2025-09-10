---
model: claude-sonnet-4-20250514
category: bridge-integration
priority: high
tags: ["bridge-integration", "parallel-execution", "task-orchestration", "multi-agent-coordination"]
description: Bridge: Parallel Start from Task Master with comprehensive orchestration and multi-agent coordination

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["parallel-execution", "task-orchestration", "multi-agent-coordination"]
  complexity-level: "complex"
  selection-criteria:
    keyword-match: 0.95
    argument-analysis: 0.95
    project-context: 0.90
  preferred-agents: ["task-orchestrator"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.85

# Universal Cipher Memory Integration (MANDATORY)
cipher-memory-integration:
  enabled: true
  priority: "high"
  pre-execution-memory:
    context-search:
      - query-pattern: "parallel execution + task orchestration + multi-agent coordination"
      - search-depth: "comprehensive"
      - max-results: 15
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - orchestration-history: "mcp__cipher-memory__search_nodes"
      - coordination-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    orchestration-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - orchestration-summary: "mcp__cipher-memory__create_entities"
      - coordination-patterns: "mcp__cipher-memory__create_entities"
      - parallel-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - command-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - orchestration-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - coordination-insights: "mcp__cipher-memory__create_entities"

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
    orchestration-tracking: true
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
  learning-domains: ["parallel-execution", "task-orchestration", "multi-agent-coordination"]
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
    load-orchestration-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-orchestration-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

# Multi-Agent Deployment Configuration
agent-deployment-map:
  driver-work: "serial-comm-specialist"
  transport-work: "serial-comm-specialist"
  ui-work: "egui-performance-optimizer"
  performance-work: "rust-performance-monitor"
  testing-work: "mock-test-orchestrator"
  documentation-work: "general-purpose"

tool-chain: "parallel-orchestration-multi-agent-coordination"
auto-deploy: true
parallel-execution: true
max-agent-count: 6
allowed-tools: ["Read", "Write", "LS", "Bash", "Task", "mcp__taskmaster-ai__get_tasks", "mcp__taskmaster-ai__get_task", "mcp__taskmaster-ai__set_task_status", "mcp__desktop-commander__start_process", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph"]
---

-|
| Driver implementation | driver-engineer |
| Serial/TCP/UDP | transport-engineer |
| UI components | ui-telemetry-analyst |
| Performance testing | performance-profiler |
| Security validation | security-hygiene |
| Documentation | docs-scribe |

## Error Handling
- If worktree exists, use existing
- If agents fail, report and continue others
- Maintain Task Master sync throughout


