---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Command for epic-start-worktree operations
allowed-tools: Bash, Read, Write, LS, Task, mcp__taskmaster-ai__get_tasks, mcp__desktop-commander__list_directory
argument-hint: <epic_name> | --max-agents=<number> | --strategy=<execution_strategy>

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-execution", "worktree-management", "parallel-coordination"]
    complexity-factors: ["multi-agent-orchestration", "dependency-management", "resource-coordination"]
    specialized-tools: ["epic-management", "worktree-operations", "agent-coordination"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["epic-execution", "parallel-coordination"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-execution + worktree-management + parallel-coordination"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "execution-patterns + worktree-knowledge + coordination-strategies"
    
    knowledge-preparation:
      - domain: "epic-execution"
      - pattern-search: "execution-strategies + worktree-patterns + coordination-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["worktree-setup", "agent-coordination", "parallel-execution"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "execution-strategies + coordination-approaches + worktree-decisions"
      - pattern-recognition: "epic-execution-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["execution-results", "coordination-insights", "worktree-techniques"]
      - knowledge-extraction: "execution-methodologies + coordination-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["execution-relationships", "coordination-dependencies", "worktree-connections"]
      - cross-reference: "related-execution-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "execution-knowledge + coordination-patterns"
      - continuous-learning: "epic-execution-optimization"

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
    - worktree-setup
    - agent-coordination
    - parallel-execution
    - resource-management
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-start-worktree"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "execution-results + coordination-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["execution-patterns", "coordination-techniques", "worktree-strategies"]
  learn-from: ["epic-start", "epic-status", "parallel-execution"]
  contribute-to: "epic-execution-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-epic-state
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-coordination-setup
    - continuous-memory-updates
    - real-time-execution-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - execution-pattern-extraction
---

# Execution Status

## Active Agents
- Agent-1: Issue #1234 Stream A (Database) - Started {time}
- Agent-2: Issue #1234 Stream B (API) - Started {time}
- Agent-3: Issue #1235 Stream A (UI) - Started {time}

## Queued Issues
- Issue #1236 - Waiting for #1234
- Issue #1237 - Waiting for #1235

## Completed
- {None yet}
```

### 6. Monitor and Coordinate

Set up monitoring:
```bash
echo "
Agents launched successfully!

Monitor progress:
  /pm:epic-status $ARGUMENTS

View worktree changes:
  cd ../epic-$ARGUMENTS && git status

Stop all agents:
  /pm:epic-stop $ARGUMENTS

Merge when complete:
  /pm:epic-merge $ARGUMENTS
"
```

### 7. Handle Dependencies

As agents complete streams:
- Check if any blocked issues are now ready
- Launch new agents for newly-ready work
- Update execution-status.md

## Output Format

```
ðŸš€ Epic Execution Started: $ARGUMENTS

Worktree: ../epic-$ARGUMENTS
Branch: epic/$ARGUMENTS

Launching {total} agents across {issue_count} issues:

Issue #1234: Database Schema
  â”œâ”€ Stream A: Schema creation (Agent-1) âœ“ Started
  â””â”€ Stream B: Migrations (Agent-2) âœ“ Started

Issue #1235: API Endpoints
  â”œâ”€ Stream A: User endpoints (Agent-3) âœ“ Started
  â”œâ”€ Stream B: Post endpoints (Agent-4) âœ“ Started
  â””â”€ Stream C: Tests (Agent-5) â¸ Waiting for A & B

Blocked Issues (2):
  - #1236: UI Components (depends on #1234)
  - #1237: Integration (depends on #1235, #1236)

Monitor with: /pm:epic-status $ARGUMENTS
```

## Error Handling

If agent launch fails:
```
âŒ Failed to start Agent-{id}
  Issue: #{issue}
  Stream: {stream}
  Error: {reason}

Continue with other agents? (yes/no)
```

If worktree creation fails:
```
âŒ Cannot create worktree
  {git error message}

Try: git worktree prune
Or: Check existing worktrees with: git worktree list
```

## Important Notes

- Follow `/rules/worktree-operations.md` for git operations
- Follow `/rules/agent-coordination.md` for parallel work
- Agents work in the SAME worktree (not separate ones)
- Maximum parallel agents should be reasonable (e.g., 5-10)
- Monitor system resources if launching many agents



