---
model: claude-sonnet-4-20250514
category: task-management
priority: high
tags: ["task-management", "tasks"]
description: Command for add-subtask operations
allowed-tools: mcp__taskmaster-ai__add_subtask, mcp__taskmaster-ai__get_task
argument-hint: [subtask-spec] | --parent=X --title="Y" | "X: Y"

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["task-decomposition", "subtask-creation", "hierarchy-management"]
    complexity-factors: ["task-breakdown", "parent-child-relationships", "workflow-structuring"]
    specialized-tools: ["task-management", "hierarchy-tools", "decomposition-analysis"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "task-executor"
    fallback: ["general-purpose"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "cipher-memory", "desktop-commander"]
    specialized-functions: ["subtask-creation", "task-decomposition"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "task-decomposition + subtask-creation + hierarchy-management"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "decomposition-patterns + hierarchy-knowledge"
    
    knowledge-preparation:
      - domain: "task-decomposition"
      - pattern-search: "decomposition-strategies + hierarchy-patterns + subtask-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["task-analysis", "subtask-creation", "hierarchy-structuring"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "decomposition-strategies + subtask-approaches + hierarchy-decisions"
      - pattern-recognition: "task-decomposition-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["decomposition-results", "subtask-insights", "hierarchy-techniques"]
      - knowledge-extraction: "decomposition-methodologies + subtask-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["decomposition-relationships", "subtask-dependencies", "hierarchy-connections"]
      - cross-reference: "related-decomposition-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "decomposition-knowledge + subtask-patterns"
      - continuous-learning: "task-decomposition-optimization"

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
    - subtask-creation
    - hierarchy-structuring
    - parent-child-linking
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "tm-add-subtask"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "decomposition-results + subtask-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["decomposition-patterns", "subtask-techniques", "hierarchy-strategies"]
  learn-from: ["expand-task", "add-task", "task-analysis"]
  contribute-to: "task-decomposition-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-taskmaster-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-decomposition-analysis
    - continuous-memory-updates
    - real-time-hierarchy-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - decomposition-pattern-extraction
---

Add a subtask to a parent task.

Arguments: $ARGUMENTS

Parse arguments to create a new subtask or convert existing task.

## Adding Subtasks

Creates subtasks to break down complex parent tasks into manageable pieces.

## Argument Parsing

Flexible natural language:
- "add subtask to 5: implement login form"
- "break down 5 with: setup, implement, test"
- "subtask for 5: handle edge cases"
- "5: validate user input" â†’ adds subtask to task 5

## Execution Modes

### 1. Create New Subtask
```bash
task-master add-subtask --parent=<id> --title="<title>" --description="<desc>"
```

### 2. Convert Existing Task
```bash
task-master add-subtask --parent=<id> --task-id=<existing-id>
```

## Smart Features

1. **Automatic Subtask Generation**
   - If title contains "and" or commas, create multiple
   - Suggest common subtask patterns
   - Inherit parent's context

2. **Intelligent Defaults**
   - Priority based on parent
   - Appropriate time estimates
   - Logical dependencies between subtasks

3. **Validation**
   - Check parent task complexity
   - Warn if too many subtasks
   - Ensure subtask makes sense

## Creation Process

1. Parse parent task context
2. Generate subtask with ID like "5.1"
3. Set appropriate defaults
4. Link to parent task
5. Update parent's time estimate

## Example Flows

```
/project:tm/add-subtask to 5: implement user authentication
â†’ Created subtask #5.1: "implement user authentication"
â†’ Parent task #5 now has 1 subtask
â†’ Suggested next subtasks: tests, documentation

/project:tm/add-subtask 5: setup, implement, test
â†’ Created 3 subtasks:
  #5.1: setup
  #5.2: implement  
  #5.3: test
```

## Post-Creation

- Show updated task hierarchy
- Suggest logical next subtasks
- Update complexity estimates
- Recommend subtask order


