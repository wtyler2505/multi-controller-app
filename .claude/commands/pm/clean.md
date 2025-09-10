---
model: claude-sonnet-4-20250514
category: project-management
priority: medium
tags: ["project-management"]
description: Clean
allowed-tools: Bash, Read, Write, LS
argument-hint: [clean-scope] | --dry-run | --archive | --purge

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["project-cleanup", "archive-management", "maintenance-operations"]
    complexity-factors: ["cleanup-analysis", "data-preservation", "organization-optimization"]
    specialized-tools: ["file-management", "archive-operations", "cleanup-tools"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "task-orchestrator"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "cipher-memory", "FileScopeMCP"]
    specialized-functions: ["project-cleanup", "archive-management"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "project-cleanup + archive-management + maintenance-operations"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "cleanup-patterns + maintenance-knowledge"
    
    knowledge-preparation:
      - domain: "project-cleanup"
      - pattern-search: "cleanup-strategies + archive-patterns + maintenance-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["cleanup-analysis", "archive-operations", "maintenance-execution"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "cleanup-strategies + archive-approaches + maintenance-decisions"
      - pattern-recognition: "project-cleanup-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["cleanup-results", "maintenance-insights", "archive-techniques"]
      - knowledge-extraction: "cleanup-methodologies + maintenance-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["cleanup-relationships", "maintenance-dependencies", "archive-connections"]
      - cross-reference: "related-cleanup-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "cleanup-knowledge + maintenance-patterns"
      - continuous-learning: "project-cleanup-optimization"

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
    - cleanup-analysis
    - archive-operations
    - maintenance-execution
    - preservation-checks
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-clean"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "cleanup-results + maintenance-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["cleanup-patterns", "maintenance-techniques", "archive-strategies"]
  learn-from: ["pm-status", "epic-close", "project-organization"]
  contribute-to: "project-cleanup-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-project-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-cleanup-analysis
    - continuous-memory-updates
    - real-time-maintenance-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - cleanup-pattern-extraction
---

# Clean

Clean up completed work and archive old epics.

## Usage
```
/pm:clean [--dry-run]
```

Options:
- `--dry-run` - Show what would be cleaned without doing it

## Instructions

### 1. Identify Completed Epics

Find epics with:
- `status: completed` in frontmatter
- All tasks closed
- Last update > 30 days ago

### 2. Identify Stale Work

Find:
- Progress files for closed issues
- Update directories for completed work
- Orphaned task files (epic deleted)
- Empty directories

### 3. Show Cleanup Plan

```
ðŸ§¹ Cleanup Plan

Completed Epics to Archive:
  {epic_name} - Completed {days} days ago
  {epic_name} - Completed {days} days ago
  
Stale Progress to Remove:
  {count} progress files for closed issues
  
Empty Directories:
  {list_of_empty_dirs}
  
Space to Recover: ~{size}KB

{If --dry-run}: This is a dry run. No changes made.
{Otherwise}: Proceed with cleanup? (yes/no)
```

### 4. Execute Cleanup

If user confirms:

**Archive Epics:**
```bash
mkdir -p .claude/epics/.archived
mv .claude/epics/{completed_epic} .claude/epics/.archived/
```

**Remove Stale Files:**
- Delete progress files for closed issues > 30 days
- Remove empty update directories
- Clean up orphaned files

**Create Archive Log:**
Create `.claude/epics/.archived/archive-log.md`:
```markdown
# Archive Log

## {current_date}
- Archived: {epic_name} (completed {date})
- Removed: {count} stale progress files
- Cleaned: {count} empty directories
```

### 5. Output

```
âœ… Cleanup Complete

Archived:
  {count} completed epics
  
Removed:
  {count} stale files
  {count} empty directories
  
Space recovered: {size}KB

System is clean and organized.
```

## Important Notes

Always offer --dry-run to preview changes.
Never delete PRDs or incomplete work.
Keep archive log for history.


