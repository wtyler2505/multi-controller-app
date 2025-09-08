---
model: claude-sonnet-4-20250514
category: utilities-tools
priority: critical
tags: ["utilities-tools"]
description: Store and retrieve code patterns, solutions, and knowledge using Cipher memory

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["memory-management", "knowledge-storage", "pattern-recognition"]
    complexity-factors: ["memory-operations", "knowledge-extraction", "pattern-analysis"]
    specialized-tools: ["cipher-memory", "knowledge-management", "pattern-storage"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "memory-specialist"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["cipher-memory", "memory"]
    specialized-functions: ["memory-operations", "knowledge-storage"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "memory-management + knowledge-storage + pattern-recognition"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "memory-patterns + knowledge-management-techniques"
    
    knowledge-preparation:
      - domain: "memory-operations"
      - pattern-search: "memory-patterns + storage-strategies + retrieval-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["memory-operations", "pattern-extraction", "knowledge-storage"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "memory-strategies + storage-approaches + retrieval-methods"
      - pattern-recognition: "memory-operation-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["memory-operation-results", "knowledge-patterns", "storage-insights"]
      - knowledge-extraction: "memory-management-techniques + pattern-recognition-methods"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["memory-relationships", "knowledge-connections", "pattern-associations"]
      - cross-reference: "related-memory-operations"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "memory-knowledge + storage-patterns"
      - continuous-learning: "memory-operation-optimization"

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
    - pattern-recognition
    - knowledge-storage
    - retrieval-operations
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "cipher-memory"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "memory-operation-results + knowledge-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["memory-patterns", "knowledge-storage-techniques", "pattern-recognition-methods"]
  learn-from: ["all-commands"]
  contribute-to: "universal-memory-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-memory-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-memory-operations
    - continuous-memory-updates
    - real-time-insight-capture
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - memory-pattern-extraction
---

# Cipher Memory Commands
Custom commands for interacting with Cipher's memory system during development.

## /memory-start-task
Load all relevant memories when starting a new task.

### Steps:
1. Get the current task from Task Master: `mcp__taskmaster-ai__get_task --id=$ARGUMENTS`
2. Search knowledge graph for task-related entities: `mcp__cipher-aggregator__search_nodes --query="task $ARGUMENTS"`
3. Search embeddings for similar implementations: `mcp__cipher-aggregator__cipher_memory_search --query="$TASK_DESCRIPTION"`
4. Load component-specific memories: `mcp__cipher-aggregator__open_nodes --names=["$COMPONENT_NAMES"]`
5. Display summary of loaded context
6. Store task start event in memory

### Usage:
```
/memory-start-task 4.2
```

## /memory-store-solution
Store a successful implementation or solution pattern.

### Steps:
1. Extract the solution from recent changes: `git diff HEAD~1`
2. Identify the pattern type and components affected
3. Create knowledge graph entity:
   ```javascript
   mcp__cipher-aggregator__create_entities([{
     name: "$COMPONENT.Solution",
     entityType: "Implementation.Pattern",
     observations: ["$SOLUTION_DETAILS"]
   }])
   ```
4. Store in knowledge memory with embeddings:
   ```javascript
   mcp__cipher-aggregator__cipher_extract_and_operate_memory({
     interaction: ["$PROBLEM", "$SOLUTION"],
     knowledgeInfo: {domain: "$DOMAIN", codePattern: "$PATTERN"},
     memoryMetadata: {importance: "IMPORTANT", taskId: "$TASK_ID"}
   })
   ```
5. Create relationships to related components
6. Confirm storage and display memory ID

### Usage:
```
/memory-store-solution "Implemented ring buffer for telemetry data"
```

## /memory-debug
Store and search debugging/troubleshooting patterns.

### Steps:
1. Capture error context and symptoms
2. Search for similar issues: `mcp__cipher-aggregator__cipher_memory_search --query="error $ERROR_MESSAGE"`
3. If no matches, prompt for solution steps
4. Once resolved, store the troubleshooting process:
   ```javascript
   mcp__cipher-aggregator__cipher_store_reasoning_memory({
     trace: {
       steps: [
         {type: "observation", content: "$ERROR_DESCRIPTION"},
         {type: "thought", content: "$HYPOTHESIS"},
         {type: "action", content: "$DEBUGGING_STEPS"},
         {type: "conclusion", content: "$SOLUTION"}
       ]
     }
   })
   ```
5. Link to affected components in knowledge graph
6. Tag with error category for future reference

### Usage:
```
/memory-debug "Serial timeout after 50ms"
```

## /memory-review
Extract and store learnings from completed work.

### Steps:
1. Get completed task details: `mcp__taskmaster-ai__get_task --id=$ARGUMENTS --status=done`
2. Analyze git commits for patterns: `git log --oneline -n 10`
3. Extract key learnings and patterns
4. Store architectural decisions:
   ```javascript
   mcp__cipher-aggregator__create_entities([{
     name: "Decision.$TOPIC",
     entityType: "Architecture.Decision",
     observations: ["Context: $CONTEXT", "Decision: $DECISION", "Rationale: $RATIONALE"]
   }])
   ```
5. Store implementation patterns in knowledge memory
6. Update task entity with completion insights
7. Prune temporary memories from task
8. Generate summary report

### Usage:
```
/memory-review 4.2
```

## /memory-search
Intelligent context-aware memory search.

### Steps:
1. Determine current context (file, task, component)
2. Build search query based on context:
   - If in file: Include component and interfaces
   - If debugging: Include error patterns
   - If implementing: Include similar patterns
3. Execute parallel searches:
   - Knowledge graph: `mcp__cipher-aggregator__search_nodes`
   - Embeddings: `mcp__cipher-aggregator__cipher_memory_search`
   - Reasoning: `mcp__cipher-aggregator__cipher_search_reasoning_patterns`
4. Rank results by relevance and recency
5. Display formatted results with context

### Usage:
```
/memory-search "serial communication patterns"
/memory-search  # Auto-detects context
```

## /memory-stats
Display memory system statistics and health.

### Steps:
1. Get usage stats: `mcp__cipher-aggregator__get_usage_stats`
2. Count entities by type in knowledge graph
3. Check embedding service status
4. Calculate memory growth rate
5. Identify most/least accessed memories
6. Show pruning recommendations
7. Display in formatted table

### Usage:
```
/memory-stats
```

## /memory-prune
Clean up old or unnecessary memories.

### Steps:
1. Identify memories marked as TEMPORARY older than 7 days
2. Find duplicate memories with high similarity (>0.95)
3. Locate orphaned relationships
4. Show preview of memories to prune
5. Confirm with user
6. Execute pruning:
   ```javascript
   mcp__cipher-aggregator__delete_entities({
     entityNames: ["$OLD_ENTITIES"]
   })
   ```
7. Optimize embeddings index
8. Display space recovered

### Usage:
```
/memory-prune --dry-run  # Preview only
/memory-prune --confirm  # Execute pruning
```

## /memory-backup
Create backup of current memory state.

### Steps:
1. Export knowledge graph: `mcp__cipher-aggregator__read_graph`
2. Export memory search index
3. Copy SQLite database to backup location
4. Create timestamped archive in `.cipher/backups/`
5. Verify backup integrity
6. Rotate old backups (keep last 5)
7. Display backup location and size

### Usage:
```
/memory-backup
```

## Quick Tips
- Use `/memory-start-task` at the beginning of each work session
- Store solutions immediately after successful tests
- Run `/memory-review` after completing major features
- Use `/memory-stats` weekly to monitor health
- Backup before major refactoring


