---
model: claude-sonnet-4-20250514
category: task-management
priority: high
tags: ["task-management"]
description: Add new tasks with intelligent parsing
allowed-tools: mcp__taskmaster-ai__add_task, mcp__taskmaster-ai__get_tasks
argument-hint: [task-description] | --research | --priority=high | --with-subtasks

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["task-management", "project-planning", "requirement-analysis"]
    complexity-factors: ["task-decomposition", "priority-assessment", "dependency-analysis"]
    specialized-tools: ["task-creation", "project-management", "planning-tools"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "task-executor"
    fallback: ["general-purpose"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "cipher-memory", "perplexity-ask"]
    specialized-functions: ["task-management", "intelligent-parsing"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "task-management + project-planning + requirement-analysis"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "task-patterns + planning-knowledge"
    
    knowledge-preparation:
      - domain: "task-management"
      - pattern-search: "task-strategies + planning-patterns + management-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["task-analysis", "requirement-parsing", "task-creation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "task-strategies + parsing-approaches + management-decisions"
      - pattern-recognition: "task-creation-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["task-results", "planning-insights", "management-techniques"]
      - knowledge-extraction: "task-methodologies + planning-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["task-relationships", "planning-dependencies", "management-connections"]
      - cross-reference: "related-task-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "task-knowledge + planning-patterns"
      - continuous-learning: "task-management-optimization"

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
    - task-analysis
    - requirement-parsing
    - task-creation
    - priority-assessment
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "tm-add-task"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "task-creation-results + planning-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["task-patterns", "planning-techniques", "management-strategies"]
  learn-from: ["expand-task", "update-task", "analyze-complexity"]
  contribute-to: "task-management-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-taskmaster-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-requirement-analysis
    - continuous-memory-updates
    - real-time-task-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - task-pattern-extraction
---

Add new tasks with intelligent parsing and context awareness.

Arguments: $ARGUMENTS

## Smart Task Addition

Parse natural language to create well-structured tasks.

### 1. **Input Understanding**

I'll intelligently parse your request:
- Natural language â†’ Structured task
- Detect priority from keywords (urgent, ASAP, important)
- Infer dependencies from context
- Suggest complexity based on description
- Determine task type (feature, bug, refactor, test, docs)

### 2. **Smart Parsing Examples**

**"Add urgent task to fix login bug"**
â†’ Title: Fix login bug
â†’ Priority: high
â†’ Type: bug
â†’ Suggested complexity: medium

**"Create task for API documentation after task 23 is done"**
â†’ Title: API documentation
â†’ Dependencies: [23]
â†’ Type: documentation
â†’ Priority: medium

**"Need to refactor auth module - depends on 12 and 15, high complexity"**
â†’ Title: Refactor auth module
â†’ Dependencies: [12, 15]
â†’ Complexity: high
â†’ Type: refactor

### 3. **Context Enhancement**

Based on current project state:
- Suggest related existing tasks
- Warn about potential conflicts
- Recommend dependencies
- Propose subtasks if complex

### 4. **Interactive Refinement**

```yaml
Task Preview:
â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
Title: [Extracted title]
Priority: [Inferred priority]
Dependencies: [Detected dependencies]
Complexity: [Estimated complexity]

Suggestions:
- Similar task #34 exists, consider as dependency?
- This seems complex, break into subtasks?
- Tasks #45-47 work on same module
```

### 5. **Validation & Creation**

Before creating:
- Validate dependencies exist
- Check for duplicates
- Ensure logical ordering
- Verify task completeness

### 6. **Smart Defaults**

Intelligent defaults based on:
- Task type patterns
- Team conventions
- Historical data
- Current sprint/phase

Result: High-quality tasks from minimal input.


