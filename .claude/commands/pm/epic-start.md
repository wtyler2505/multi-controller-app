---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Command for epic-start operations
allowed-tools: Bash, Read, Write, LS, Task, mcp__taskmaster-ai__get_tasks, mcp__desktop-commander__read_file
argument-hint: <epic_name> | --branch-only | --max-agents=<number>

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-initiation", "branch-management", "execution-coordination"]
    complexity-factors: ["git-operations", "multi-agent-setup", "dependency-coordination"]
    specialized-tools: ["epic-management", "git-operations", "execution-coordination"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["epic-initiation", "execution-coordination"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-initiation + branch-management + execution-coordination"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "initiation-patterns + branch-knowledge + coordination-strategies"
    
    knowledge-preparation:
      - domain: "epic-initiation"
      - pattern-search: "initiation-strategies + branch-patterns + coordination-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["branch-setup", "execution-initiation", "coordination-setup"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "initiation-strategies + coordination-approaches + branch-decisions"
      - pattern-recognition: "epic-initiation-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["initiation-results", "coordination-insights", "branch-techniques"]
      - knowledge-extraction: "initiation-methodologies + coordination-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["initiation-relationships", "coordination-dependencies", "branch-connections"]
      - cross-reference: "related-initiation-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "initiation-knowledge + coordination-patterns"
      - continuous-learning: "epic-initiation-optimization"

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
    - branch-setup
    - execution-initiation
    - coordination-setup
    - git-operations
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-start"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "initiation-results + coordination-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["initiation-patterns", "coordination-techniques", "branch-strategies"]
  learn-from: ["epic-start-worktree", "epic-status", "execution-coordination"]
  contribute-to: "epic-initiation-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-epic-readiness
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-initiation-setup
    - continuous-memory-updates
    - real-time-coordination-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - initiation-pattern-extraction
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

View branch changes:
  git status

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

If uncommitted changes are found:
```
âŒ You have uncommitted changes. Please commit or stash them before starting an epic.

To commit changes:
  git add .
  git commit -m "Your commit message"

To stash changes:
  git stash push -m "Work in progress"
  # (Later restore with: git stash pop)
```

If branch creation fails:
```
âŒ Cannot create branch
  {git error message}

Try: git branch -d epic/$ARGUMENTS
Or: Check existing branches with: git branch -a
```

## Important Notes

- Follow `/rules/branch-operations.md` for git operations
- Follow `/rules/agent-coordination.md` for parallel work
- Agents work in the SAME branch (not separate branches)
- Maximum parallel agents should be reasonable (e.g., 5-10)
- Monitor system resources if launching many agents



