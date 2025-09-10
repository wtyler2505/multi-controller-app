---
model: claude-sonnet-4-20250514
category: context-management
priority: high
tags: ["context-management"]
description: Update Context
allowed-tools: Bash, Read, Write, LS
argument-hint: [update-scope] | --incremental | --full | --targeted

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["context-updating", "change-detection", "incremental-sync"]
    complexity-factors: ["change-analysis", "context-maintenance", "update-optimization"]
    specialized-tools: ["context-management", "change-detection", "sync-operations"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "documentation-specialist"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "FileScopeMCP", "cipher-memory"]
    specialized-functions: ["context-updating", "change-detection"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "context-updating + change-detection + incremental-sync"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "update-patterns + sync-knowledge"
    
    knowledge-preparation:
      - domain: "context-updating"
      - pattern-search: "update-strategies + sync-patterns + maintenance-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["change-analysis", "context-updating", "sync-execution"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "update-strategies + sync-approaches + maintenance-decisions"
      - pattern-recognition: "context-updating-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["update-results", "sync-insights", "maintenance-techniques"]
      - knowledge-extraction: "updating-methodologies + sync-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["update-relationships", "sync-dependencies", "maintenance-connections"]
      - cross-reference: "related-updating-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "update-knowledge + sync-patterns"
      - continuous-learning: "context-updating-optimization"

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
    - change-analysis
    - context-updating
    - sync-execution
    - validation-checks
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "context-update"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "update-results + sync-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["update-patterns", "sync-techniques", "maintenance-strategies"]
  learn-from: ["context-create", "context-prime", "project-analysis"]
  contribute-to: "context-updating-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-context-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-update-operations
    - continuous-memory-updates
    - real-time-sync-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - update-pattern-extraction
---

```
4. **Make targeted updates** - don't rewrite entire file
5. **Add update notes** at the bottom if significant:
   ```markdown
   ## Update History
   - {date}: {summary of what changed}
   ```

### 3. Update Validation

After updating each file:
- Verify file still has valid frontmatter
- Check file size is reasonable (not corrupted)
- Ensure markdown formatting is preserved
- Confirm updates accurately reflect changes

### 4. Skip Optimization

**Skip files that don't need updates:**
- If no relevant changes detected, skip the file
- Report skipped files in summary
- Don't update timestamp if content unchanged
- This preserves accurate "last modified" information

### 5. Error Handling

**Common Issues:**
- **File locked:** "âŒ Cannot update {file} - may be open in editor"
- **Permission denied:** "âŒ Cannot write to {file} - check permissions"
- **Corrupted file:** "âš ï¸ {file} appears corrupted - skipping update"
- **Disk space:** "âŒ Insufficient disk space for updates"

If update fails:
- Report which files were successfully updated
- Note which files failed and why
- Preserve original files (don't leave corrupted state)

### 6. Update Summary

Provide detailed summary of updates:

```
ðŸ”„ Context Update Complete

ðŸ“Š Update Statistics:
  - Files Scanned: {total_count}
  - Files Updated: {updated_count}
  - Files Skipped: {skipped_count} (no changes needed)
  - Errors: {error_count}

ðŸ“ Updated Files:
  âœ… progress.md - Updated recent commits, current status
  âœ… tech-context.md - Added 3 new dependencies
  âœ… project-structure.md - Noted new /utils directory

â­ï¸ Skipped Files (no changes):
  - project-brief.md (last updated: 5 days ago)
  - project-vision.md (last updated: 2 weeks ago)
  - system-patterns.md (last updated: 3 days ago)

âš ï¸ Issues:
  {any warnings or errors}

â° Last Update: {timestamp}
ðŸ”„ Next: Run this command regularly to keep context current
ðŸ’¡ Tip: Major changes? Consider running /context:create for full refresh
```

### 7. Incremental Update Tracking

**Track what was updated:**
- Note which sections of each file were modified
- Keep changes focused and surgical
- Don't regenerate unchanged content
- Preserve formatting and structure

### 8. Performance Optimization

For large projects:
- Process files in parallel when possible
- Show progress: "Updating context files... {current}/{total}"
- Skip very large files with warning
- Use git diff to quickly identify changed areas

## Context Gathering Commands

Use these commands to detect changes:
- Context directory: `.claude/context/`
- Current git status: `git status --short`
- Recent commits: `git log --oneline -10`
- Changed files: `git diff --name-only HEAD~5..HEAD 2>/dev/null`
- Branch info: `git branch --show-current`
- Uncommitted changes: `git diff --stat`
- New untracked files: `git ls-files --others --exclude-standard | head -10`
- Dependency changes: Check package.json, requirements.txt, etc.

## Important Notes

- **Only update files with actual changes** - preserve accurate timestamps
- **Always use real datetime** from system clock for `last_updated`
- **Make surgical updates** - don't regenerate entire files
- **Validate each update** - ensure files remain valid
- **Provide detailed summary** - show what changed and what didn't
- **Handle errors gracefully** - don't corrupt existing context

$ARGUMENTS



