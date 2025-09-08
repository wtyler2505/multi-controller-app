---
model: claude-sonnet-4-20250514
category: task-management
priority: high
tags: ["task-management", "analysis"]
description: Command for analyze-complexity operations
allowed-tools: mcp__taskmaster-ai__analyze_project_complexity, mcp__taskmaster-ai__complexity_report
argument-hint: [analysis-scope] | --research | --threshold=N | --range=X-Y

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["complexity-analysis", "task-evaluation", "risk-assessment"]
    complexity-factors: ["analysis-depth", "scoring-algorithms", "recommendation-generation"]
    specialized-tools: ["complexity-analysis", "risk-assessment", "task-evaluation"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "perplexity-ask", "cipher-memory"]
    specialized-functions: ["complexity-analysis", "task-evaluation"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "complexity-analysis + task-evaluation + risk-assessment"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "analysis-patterns + evaluation-knowledge"
    
    knowledge-preparation:
      - domain: "complexity-analysis"
      - pattern-search: "analysis-strategies + evaluation-patterns + assessment-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["complexity-evaluation", "risk-assessment", "recommendation-generation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "analysis-strategies + evaluation-approaches + assessment-decisions"
      - pattern-recognition: "complexity-analysis-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["analysis-results", "evaluation-insights", "assessment-techniques"]
      - knowledge-extraction: "analysis-methodologies + evaluation-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["analysis-relationships", "evaluation-dependencies", "assessment-connections"]
      - cross-reference: "related-analysis-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "analysis-knowledge + evaluation-patterns"
      - continuous-learning: "complexity-analysis-optimization"

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
    - complexity-evaluation
    - risk-assessment
    - recommendation-generation
    - scoring-analysis
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "tm-analyze-complexity"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "analysis-results + evaluation-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["analysis-patterns", "evaluation-techniques", "assessment-strategies"]
  learn-from: ["expand-task", "complexity-report", "task-evaluation"]
  contribute-to: "complexity-analysis-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-taskmaster-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-complexity-analysis
    - continuous-memory-updates
    - real-time-evaluation-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - analysis-pattern-extraction
---

Analyze task complexity and generate expansion recommendations.

Arguments: $ARGUMENTS

Perform deep analysis of task complexity across the project.

## Complexity Analysis

Uses AI to analyze tasks and recommend which ones need breakdown.

## Execution Options

```bash
task-master analyze-complexity [--research] [--threshold=5]
```

## Analysis Parameters

- `--research` â†’ Use research AI for deeper analysis
- `--threshold=5` â†’ Only flag tasks above complexity 5
- Default: Analyze all pending tasks

## Analysis Process

### 1. **Task Evaluation**
For each task, AI evaluates:
- Technical complexity
- Time requirements
- Dependency complexity
- Risk factors
- Knowledge requirements

### 2. **Complexity Scoring**
Assigns score 1-10 based on:
- Implementation difficulty
- Integration challenges
- Testing requirements
- Unknown factors
- Technical debt risk

### 3. **Recommendations**
For complex tasks:
- Suggest expansion approach
- Recommend subtask breakdown
- Identify risk areas
- Propose mitigation strategies

## Smart Analysis Features

1. **Pattern Recognition**
   - Similar task comparisons
   - Historical complexity accuracy
   - Team velocity consideration
   - Technology stack factors

2. **Contextual Factors**
   - Team expertise
   - Available resources
   - Timeline constraints
   - Business criticality

3. **Risk Assessment**
   - Technical risks
   - Timeline risks
   - Dependency risks
   - Knowledge gaps

## Output Format

```
Task Complexity Analysis Report
â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”â”

High Complexity Tasks (>7):
ðŸ“ #5 "Implement real-time sync" - Score: 9/10
   Factors: WebSocket complexity, state management, conflict resolution
   Recommendation: Expand into 5-7 subtasks
   Risks: Performance, data consistency

ðŸ“ #12 "Migrate database schema" - Score: 8/10
   Factors: Data migration, zero downtime, rollback strategy
   Recommendation: Expand into 4-5 subtasks
   Risks: Data loss, downtime

Medium Complexity Tasks (5-7):
ðŸ“ #23 "Add export functionality" - Score: 6/10
   Consider expansion if timeline tight

Low Complexity Tasks (<5):
âœ… 15 tasks - No expansion needed

Summary:
- Expand immediately: 2 tasks
- Consider expanding: 5 tasks
- Keep as-is: 15 tasks
```

## Actionable Output

For each high-complexity task:
1. Complexity score with reasoning
2. Specific expansion suggestions
3. Risk mitigation approaches
4. Recommended subtask structure

## Integration

Results are:
- Saved to `.taskmaster/reports/complexity-analysis.md`
- Used by expand command
- Inform sprint planning
- Guide resource allocation

## Next Steps

After analysis:
```
/project:tm/expand 5    # Expand specific task
/project:tm/expand/all  # Expand all recommended
/project:tm/complexity-report  # View detailed report
```


