---
model: claude-sonnet-4-20250514
category: taskmaster-core
priority: high
tags: ["taskmaster-core", "interactive-learning", "command-discovery", "capability-exploration"]
description: Interactive Task Master learning system with comprehensive capability discovery and guided workflows

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["interactive-learning", "command-discovery", "capability-exploration"]
  complexity-level: "moderate"
  selection-criteria:
    keyword-match: 0.90
    argument-analysis: 0.85
    project-context: 0.80
  preferred-agents: ["general-purpose"]
  fallback-agents: ["task-orchestrator"]
  confidence-threshold: 0.75

# Universal Cipher Memory Integration (MANDATORY)
cipher-memory-integration:
  enabled: true
  priority: "medium"
  pre-execution-memory:
    context-search:
      - query-pattern: "interactive learning + command discovery + capability exploration"
      - search-depth: "standard"
      - max-results: 10
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - learning-history: "mcp__cipher-memory__search_nodes"
      - capability-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    learning-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - learning-summary: "mcp__cipher-memory__create_entities"
      - capability-patterns: "mcp__cipher-memory__create_entities"
      - interaction-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - command-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - learning-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - learning-insights: "mcp__cipher-memory__create_entities"

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
    learning-tracking: true
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
  learning-domains: ["interactive-learning", "command-discovery", "capability-exploration"]
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
    load-learning-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-learning-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

tool-chain: "interactive-learning-command-discovery"
auto-deploy: true
parallel-execution: false
allowed-tools: ["Bash", "Read", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph"]
---

Learn about Task Master capabilities through interactive exploration.

Arguments: $ARGUMENTS

## Interactive Task Master Learning

Based on your input, I'll help you discover capabilities:

### 1. **What are you trying to do?**

If $ARGUMENTS contains:
- "start" / "begin" â†’ Show project initialization workflows
- "manage" / "organize" â†’ Show task management commands  
- "automate" / "auto" â†’ Show automation workflows
- "analyze" / "report" â†’ Show analysis tools
- "fix" / "problem" â†’ Show troubleshooting commands
- "fast" / "quick" â†’ Show efficiency shortcuts

### 2. **Intelligent Suggestions**

Based on your project state:

**No tasks yet?**
```
You'll want to start with:
1. /project:task-master:init <prd-file>
   â†’ Creates tasks from requirements
   
2. /project:task-master:parse-prd <file>
   â†’ Alternative task generation

Try: /project:task-master:init demo-prd.md
```

**Have tasks?**
Let me analyze what you might need...
- Many pending tasks? â†’ Learn sprint planning
- Complex tasks? â†’ Learn task expansion
- Daily work? â†’ Learn workflow automation

### 3. **Command Discovery**

**By Category:**
- ðŸ“‹ Task Management: list, show, add, update, complete
- ðŸ”„ Workflows: auto-implement, sprint-plan, daily-standup
- ðŸ› ï¸ Utilities: check-health, complexity-report, sync-memory
- ðŸ” Analysis: validate-deps, show dependencies

**By Scenario:**
- "I want to see what to work on" â†’ `/project:task-master:next`
- "I need to break this down" â†’ `/project:task-master:expand <id>`
- "Show me everything" â†’ `/project:task-master:status`
- "Just do it for me" â†’ `/project:workflows:auto-implement`

### 4. **Power User Patterns**

**Command Chaining:**
```
/project:task-master:next
/project:task-master:start <id>
/project:workflows:auto-implement
```

**Smart Filters:**
```
/project:task-master:list pending high
/project:task-master:list blocked
/project:task-master:list 1-5 tree
```

**Automation:**
```
/project:workflows:pipeline init â†’ expand-all â†’ sprint-plan
```

### 5. **Learning Path**

Based on your experience level:

**Beginner Path:**
1. init â†’ Create project
2. status â†’ Understand state
3. next â†’ Find work
4. complete â†’ Finish task

**Intermediate Path:**
1. expand â†’ Break down complex tasks
2. sprint-plan â†’ Organize work
3. complexity-report â†’ Understand difficulty
4. validate-deps â†’ Ensure consistency

**Advanced Path:**
1. pipeline â†’ Chain operations
2. smart-flow â†’ Context-aware automation
3. Custom commands â†’ Extend the system

### 6. **Try This Now**

Based on what you asked about, try:
[Specific command suggestion based on $ARGUMENTS]

Want to learn more about a specific command?
Type: /project:help <command-name>


