---
model: claude-sonnet-4-20250514
category: bridge-integration
priority: high
tags: ["bridge-integration", "github", "taskmaster"]
description: Bridge: Task Master to CCPM Epic
allowed-tools: Read, Write, LS, Bash, mcp__taskmaster-ai__get_task
argument-hint: [task-id] | --parallel | --sync

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["task-synchronization", "epic-conversion", "bridge-operations"]
    complexity-factors: ["data-mapping", "format-conversion", "workflow-bridging"]
    specialized-tools: ["taskmaster-integration", "epic-generation", "sync-operations"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["task-bridging", "epic-conversion"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "task-synchronization + epic-conversion + bridge-operations"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "bridge-patterns + conversion-knowledge"
    
    knowledge-preparation:
      - domain: "task-bridging"
      - pattern-search: "bridge-strategies + conversion-patterns + sync-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["conversion-analysis", "epic-creation", "sync-execution"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "bridge-strategies + conversion-approaches + sync-decisions"
      - pattern-recognition: "task-bridging-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["bridge-results", "conversion-insights", "sync-techniques"]
      - knowledge-extraction: "bridging-methodologies + conversion-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["bridge-relationships", "conversion-dependencies", "sync-connections"]
      - cross-reference: "related-bridging-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "bridge-knowledge + conversion-patterns"
      - continuous-learning: "task-bridging-optimization"

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
    - conversion-analysis
    - epic-creation
    - sync-execution
    - format-mapping
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "bridge-tm-to-epic"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "bridge-results + conversion-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["bridge-patterns", "conversion-techniques", "sync-strategies"]
  learn-from: ["sync-all", "parallel-start", "epic-decompose"]
  contribute-to: "task-bridging-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-task-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-conversion-operations
    - continuous-memory-updates
    - real-time-sync-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - bridge-pattern-extraction
---

# Bridge: Task Master to CCPM Epic

Convert a Task Master task into a CCPM epic format for parallel execution.

## Usage
```
/bridge:tm-to-epic [task-id]
```

## Steps

1. **Read Task from Task Master**
   - Use mcp__taskmaster-ai__get_task to fetch task details
   - Extract title, description, subtasks, dependencies

2. **Create Epic File**
   - Create `.claude/epics/[task-name]/epic.md`
   - Format:
     ```markdown
     # Epic: [Task Title]
     
     ## Overview
     [Task description and details]
     
     ## Acceptance Criteria
     - [From task test strategy]
     
     ## Technical Approach
     [From task details]
     
     ## Tasks
     [Subtasks converted to CCPM format]
     
     ## Dependencies
     [Task dependencies]
     
     ## Parallelization
     parallel: true (if subtasks can run concurrently)
     ```

3. **Create Task Files**
   - For each subtask, create `001.md`, `002.md`, etc.
   - Include parallel execution flags
   - Map to specialized agents

4. **Link Back to Task Master**
   - Store Task Master ID in epic metadata
   - Enable bidirectional sync

5. **Report Success**
   - Show created files
   - Provide next command to execute

## Error Handling
- If task doesn't exist, show available tasks
- If epic already exists, ask to overwrite
- Preserve all Task Master metadata


