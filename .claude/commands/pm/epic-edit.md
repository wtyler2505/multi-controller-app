---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Command for epic-edit operations
allowed-tools: Read, Write, LS, mcp__taskmaster-ai__get_task, mcp__desktop-commander__read_file
argument-hint: <epic_name> | --interactive | --field=<field_name>

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-management", "content-editing", "metadata-handling"]
    complexity-factors: ["content-modification", "version-control", "metadata-preservation"]
    specialized-tools: ["epic-management", "content-editing", "version-control"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["epic-editing", "content-management"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-management + content-editing + metadata-handling"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "editing-patterns + epic-knowledge"
    
    knowledge-preparation:
      - domain: "epic-editing"
      - pattern-search: "editing-strategies + content-patterns + metadata-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["epic-analysis", "content-modification", "metadata-updates"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "editing-strategies + content-approaches + metadata-decisions"
      - pattern-recognition: "epic-editing-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["editing-results", "content-insights", "metadata-techniques"]
      - knowledge-extraction: "editing-methodologies + content-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["editing-relationships", "content-dependencies", "metadata-connections"]
      - cross-reference: "related-editing-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "editing-knowledge + content-patterns"
      - continuous-learning: "epic-editing-optimization"

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
    - epic-analysis
    - content-modification
    - metadata-updates
    - version-control
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-edit"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "editing-results + content-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["editing-patterns", "content-techniques", "metadata-strategies"]
  learn-from: ["epic-show", "epic-create", "content-management"]
  contribute-to: "epic-editing-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-epic-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-editing-analysis
    - continuous-memory-updates
    - real-time-content-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - editing-pattern-extraction
---

# Epic Edit

Edit epic details after creation.

## Usage
```
/pm:epic-edit <epic_name>
```

## Instructions

### 1. Read Current Epic

Read `.claude/epics/$ARGUMENTS/epic.md`:
- Parse frontmatter
- Read content sections

### 2. Interactive Edit

Ask user what to edit:
- Name/Title
- Description/Overview
- Architecture decisions
- Technical approach
- Dependencies
- Success criteria

### 3. Update Epic File

Get current datetime: `date -u +"%Y-%m-%dT%H:%M:%SZ"`

Update epic.md:
- Preserve all frontmatter except `updated`
- Apply user's edits to content
- Update `updated` field with current datetime

### 4. Option to Update GitHub

If epic has GitHub URL in frontmatter:
Ask: "Update GitHub issue? (yes/no)"

If yes:
```bash
gh issue edit {issue_number} --body-file .claude/epics/$ARGUMENTS/epic.md
```

### 5. Output

```
âœ… Updated epic: $ARGUMENTS
  Changes made to: {sections_edited}
  
{If GitHub updated}: GitHub issue updated âœ…

View epic: /pm:epic-show $ARGUMENTS
```

## Important Notes

Preserve frontmatter history (created, github URL, etc.).
Don't change task files when editing epic.
Follow `/rules/frontmatter-operations.md`.


