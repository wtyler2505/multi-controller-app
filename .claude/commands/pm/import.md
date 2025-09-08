---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management"]
description: Command for import operations
allowed-tools: Bash, Read, Write, LS, mcp__taskmaster-ai__get_tasks, mcp__desktop-commander__read_multiple_files
argument-hint: [--epic <epic_name>] [--label <label>] | --dry-run

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["data-import", "github-integration", "structure-migration"]
    complexity-factors: ["multi-source-import", "data-transformation", "structure-creation"]
    specialized-tools: ["data-migration", "github-operations", "structure-generation"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["data-import", "github-integration"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "data-import + github-integration + structure-migration"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "import-patterns + integration-knowledge + migration-strategies"
    
    knowledge-preparation:
      - domain: "data-import"
      - pattern-search: "import-strategies + integration-patterns + migration-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["data-analysis", "structure-creation", "import-validation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "import-strategies + integration-approaches + migration-decisions"
      - pattern-recognition: "data-import-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["import-results", "integration-insights", "migration-techniques"]
      - knowledge-extraction: "import-methodologies + integration-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["import-relationships", "integration-dependencies", "migration-connections"]
      - cross-reference: "related-import-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "import-knowledge + integration-patterns"
      - continuous-learning: "data-import-optimization"

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
    - data-analysis
    - structure-creation
    - import-validation
    - github-operations
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-import"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "import-results + integration-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["import-patterns", "integration-techniques", "migration-strategies"]
  learn-from: ["epic-sync", "github-operations", "data-migration"]
  contribute-to: "data-import-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-import-prerequisites
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-import-operations
    - continuous-memory-updates
    - real-time-integration-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - import-pattern-extraction
---

# Import

Import existing GitHub issues into the PM system.

## Usage
```
/pm:import [--epic <epic_name>] [--label <label>]
```

Options:
- `--epic` - Import into specific epic
- `--label` - Import only issues with specific label
- No args - Import all untracked issues

## Instructions

### 1. Fetch GitHub Issues

```bash
# Get issues based on filters
if [[ "$ARGUMENTS" == *"--label"* ]]; then
  gh issue list --label "{label}" --limit 1000 --json number,title,body,state,labels,createdAt,updatedAt
else
  gh issue list --limit 1000 --json number,title,body,state,labels,createdAt,updatedAt
fi
```

### 2. Identify Untracked Issues

For each GitHub issue:
- Search local files for matching github URL
- If not found, it's untracked and needs import

### 3. Categorize Issues

Based on labels:
- Issues with "epic" label â†’ Create epic structure
- Issues with "task" label â†’ Create task in appropriate epic
- Issues with "epic:{name}" label â†’ Assign to that epic
- No PM labels â†’ Ask user or create in "imported" epic

### 4. Create Local Structure

For each issue to import:

**If Epic:**
```bash
mkdir -p .claude/epics/{epic_name}
# Create epic.md with GitHub content and frontmatter
```

**If Task:**
```bash
# Find next available number (001.md, 002.md, etc.)
# Create task file with GitHub content
```

Set frontmatter:
```yaml
name: {issue_title}
status: {open|closed based on GitHub}
created: {GitHub createdAt}
updated: {GitHub updatedAt}
github: https://github.com/{org}/{repo}/issues/{number}
imported: true
```

### 5. Output

```
ðŸ“¥ Import Complete

Imported:
  Epics: {count}
  Tasks: {count}
  
Created structure:
  {epic_1}/
    - {count} tasks
  {epic_2}/
    - {count} tasks
    
Skipped (already tracked): {count}

Next steps:
  Run /pm:status to see imported work
  Run /pm:sync to ensure full synchronization
```

## Important Notes

Preserve all GitHub metadata in frontmatter.
Mark imported files with `imported: true` flag.
Don't overwrite existing local files.


