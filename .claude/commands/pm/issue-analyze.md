---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github", "analysis"]
description: Command for issue-analyze operations
allowed-tools: Bash, Read, Write, LS, mcp__taskmaster-ai__get_task, mcp__desktop-commander__read_file
argument-hint: <issue_number> | --streams=<number> | --detailed

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["issue-analysis", "parallel-decomposition", "workflow-optimization"]
    complexity-factors: ["work-stream-analysis", "dependency-mapping", "parallelization-strategy"]
    specialized-tools: ["issue-analysis", "workflow-optimization", "parallel-planning"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["issue-analysis", "parallel-decomposition"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "issue-analysis + parallel-decomposition + workflow-optimization"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "analysis-patterns + decomposition-knowledge + optimization-strategies"
    
    knowledge-preparation:
      - domain: "issue-analysis"
      - pattern-search: "analysis-strategies + decomposition-patterns + optimization-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["issue-decomposition", "stream-analysis", "optimization-planning"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "analysis-strategies + decomposition-approaches + optimization-decisions"
      - pattern-recognition: "issue-analysis-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["analysis-results", "decomposition-insights", "optimization-techniques"]
      - knowledge-extraction: "analysis-methodologies + decomposition-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["analysis-relationships", "decomposition-dependencies", "optimization-connections"]
      - cross-reference: "related-analysis-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "analysis-knowledge + decomposition-patterns"
      - continuous-learning: "issue-analysis-optimization"

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
    - issue-decomposition
    - stream-analysis
    - optimization-planning
    - parallelization-strategy
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-issue-analyze"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "analysis-results + decomposition-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["analysis-patterns", "decomposition-techniques", "optimization-strategies"]
  learn-from: ["issue-start", "parallel-execution", "workflow-optimization"]
  contribute-to: "issue-analysis-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-issue-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-analysis-operations
    - continuous-memory-updates
    - real-time-decomposition-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - analysis-pattern-extraction
---

# Parallel Work Analysis: Issue #$ARGUMENTS

## Overview
{Brief description of what needs to be done}

## Parallel Streams

### Stream A: {Stream Name}
**Scope**: {What this stream handles}
**Files**:
- {file_pattern_1}
- {file_pattern_2}
**Agent Type**: {backend|frontend|fullstack|database}-specialist
**Can Start**: immediately
**Estimated Hours**: {hours}
**Dependencies**: none

### Stream B: {Stream Name}
**Scope**: {What this stream handles}
**Files**:
- {file_pattern_1}
- {file_pattern_2}
**Agent Type**: {agent_type}
**Can Start**: immediately
**Estimated Hours**: {hours}
**Dependencies**: none

### Stream C: {Stream Name}
**Scope**: {What this stream handles}
**Files**:
- {file_pattern_1}
**Agent Type**: {agent_type}
**Can Start**: after Stream A completes
**Estimated Hours**: {hours}
**Dependencies**: Stream A

## Coordination Points

### Shared Files
{List any files multiple streams need to modify}:
- `src/types/index.ts` - Streams A & B (coordinate type updates)
- `package.json` - Stream B (add dependencies)

### Sequential Requirements
{List what must happen in order}:
1. Database schema before API endpoints
2. API types before UI components
3. Core logic before tests

## Conflict Risk Assessment
- **Low Risk**: Streams work on different directories
- **Medium Risk**: Some shared type files, manageable with coordination
- **High Risk**: Multiple streams modifying same core files

## Parallelization Strategy

**Recommended Approach**: {sequential|parallel|hybrid}

{If parallel}: Launch Streams A, B simultaneously. Start C when A completes.
{If sequential}: Complete Stream A, then B, then C.
{If hybrid}: Start A & B together, C depends on A, D depends on B & C.

## Expected Timeline

With parallel execution:
- Wall time: {max_stream_hours} hours
- Total work: {sum_all_hours} hours
- Efficiency gain: {percentage}%

Without parallel execution:
- Wall time: {sum_all_hours} hours

## Notes
{Any special considerations, warnings, or recommendations}
```

### 4. Validate Analysis

Ensure:
- All major work is covered by streams
- File patterns don't unnecessarily overlap
- Dependencies are logical
- Agent types match the work type
- Time estimates are reasonable

### 5. Output

```
âœ… Analysis complete for issue #$ARGUMENTS

Identified {count} parallel work streams:
  Stream A: {name} ({hours}h)
  Stream B: {name} ({hours}h)
  Stream C: {name} ({hours}h)
  
Parallelization potential: {factor}x speedup
  Sequential time: {total}h
  Parallel time: {reduced}h

Files at risk of conflict:
  {list shared files if any}

Next: Start work with /pm:issue-start $ARGUMENTS
```

## Important Notes

- Analysis is local only - not synced to GitHub
- Focus on practical parallelization, not theoretical maximum
- Consider agent expertise when assigning streams
- Account for coordination overhead in estimates
- Prefer clear separation over maximum parallelization


