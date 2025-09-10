---
model: claude-sonnet-4-20250514
category: task-management
priority: high
tags: ["task-management", "tasks"]
description: Command for add-dependency operations
allowed-tools: mcp__taskmaster-ai__add_dependency, mcp__taskmaster-ai__get_tasks
argument-hint: [task-dependency] | --id=X --depends-on=Y | "X depends on Y"

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["dependency-management", "task-relationships", "workflow-analysis"]
    complexity-factors: ["dependency-validation", "circular-detection", "workflow-impact"]
    specialized-tools: ["dependency-analysis", "task-management", "workflow-tools"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "task-executor"
    fallback: ["general-purpose"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "cipher-memory", "desktop-commander"]
    specialized-functions: ["dependency-management", "task-analysis"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "dependency-management + task-relationships + workflow-analysis"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "dependency-patterns + workflow-knowledge"
    
    knowledge-preparation:
      - domain: "dependency-management"
      - pattern-search: "dependency-strategies + workflow-patterns + relationship-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["dependency-analysis", "relationship-creation", "workflow-validation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "dependency-strategies + relationship-approaches + workflow-decisions"
      - pattern-recognition: "dependency-management-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["dependency-results", "relationship-insights", "workflow-techniques"]
      - knowledge-extraction: "dependency-methodologies + workflow-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["dependency-relationships", "workflow-dependencies", "task-connections"]
      - cross-reference: "related-dependency-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "dependency-knowledge + workflow-patterns"
      - continuous-learning: "dependency-management-optimization"

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
    - dependency-analysis
    - relationship-creation
    - workflow-validation
    - circular-detection
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "tm-add-dependency"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "dependency-results + workflow-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["dependency-patterns", "workflow-techniques", "relationship-strategies"]
  learn-from: ["validate-dependencies", "remove-dependency", "task-analysis"]
  contribute-to: "dependency-management-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-taskmaster-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-dependency-analysis
    - continuous-memory-updates
    - real-time-validation-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - dependency-pattern-extraction
---

Add a dependency between tasks.

Arguments: $ARGUMENTS

Parse the task IDs to establish dependency relationship.

## Adding Dependencies

Creates a dependency where one task must be completed before another can start.

## Argument Parsing

Parse natural language or IDs:
- "make 5 depend on 3" â†’ task 5 depends on task 3
- "5 needs 3" â†’ task 5 depends on task 3
- "5 3" â†’ task 5 depends on task 3
- "5 after 3" â†’ task 5 depends on task 3

## Execution

```bash
task-master add-dependency --id=<task-id> --depends-on=<dependency-id>
```

## Validation

Before adding:
1. **Verify both tasks exist**
2. **Check for circular dependencies**
3. **Ensure dependency makes logical sense**
4. **Warn if creating complex chains**

## Smart Features

- Detect if dependency already exists
- Suggest related dependencies
- Show impact on task flow
- Update task priorities if needed

## Post-Addition

After adding dependency:
1. Show updated dependency graph
2. Identify any newly blocked tasks
3. Suggest task order changes
4. Update project timeline

## Example Flows

```
/project:tm/add-dependency 5 needs 3
â†’ Task #5 now depends on Task #3
â†’ Task #5 is now blocked until #3 completes
â†’ Suggested: Also consider if #5 needs #4
```


