---
model: claude-sonnet-4-20250514
category: context-management
priority: high
tags: ["context-management"]
description: Prime Context
allowed-tools: Bash, Read, LS
argument-hint: [context-level] | --essential | --full | --priority

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["context-loading", "project-analysis", "state-management"]
    complexity-factors: ["context-validation", "file-integrity", "priority-loading"]
    specialized-tools: ["context-management", "file-validation", "project-analysis"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "documentation-specialist"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "FileScopeMCP", "cipher-memory"]
    specialized-functions: ["context-loading", "project-analysis"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "context-loading + project-analysis + state-management"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "context-patterns + loading-knowledge"
    
    knowledge-preparation:
      - domain: "context-management"
      - pattern-search: "context-strategies + loading-patterns + management-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["context-validation", "file-loading", "project-analysis"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "context-strategies + loading-approaches + management-decisions"
      - pattern-recognition: "context-loading-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["context-results", "loading-insights", "management-techniques"]
      - knowledge-extraction: "context-methodologies + loading-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["context-relationships", "loading-dependencies", "management-connections"]
      - cross-reference: "related-context-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "context-knowledge + loading-patterns"
      - continuous-learning: "context-management-optimization"

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
    - context-validation
    - file-loading
    - project-analysis
    - integrity-checks
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "context-prime"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "context-loading-results + analysis-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["context-patterns", "loading-techniques", "management-strategies"]
  learn-from: ["context-create", "context-update", "project-analysis"]
  contribute-to: "context-management-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-context-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-context-loading
    - continuous-memory-updates
    - real-time-validation-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - context-pattern-extraction
---

# Prime Context

This command loads essential context for a new agent session by reading the project context documentation and understanding the codebase structure.

## Preflight Checklist

Before proceeding, complete these validation steps.
Do not bother the user with preflight checks progress ("I'm not going to ..."). Just do them and move on.

### 1. Context Availability Check
- Run: `ls -la .claude/context/ 2>/dev/null`
- If directory doesn't exist or is empty:
  - Tell user: "âŒ No context found. Please run /context:create first to establish project context."
  - Exit gracefully
- Count available context files: `ls -1 .claude/context/*.md 2>/dev/null | wc -l`
- Report: "ðŸ“ Found {count} context files to load"

### 2. File Integrity Check
- For each context file found:
  - Verify file is readable: `test -r ".claude/context/{file}" && echo "readable"`
  - Check file has content: `test -s ".claude/context/{file}" && echo "has content"`
  - Check for valid frontmatter (should start with `---`)
- Report any issues:
  - Empty files: "âš ï¸ {filename} is empty (skipping)"
  - Unreadable files: "âš ï¸ Cannot read {filename} (permission issue)"
  - Missing frontmatter: "âš ï¸ {filename} missing frontmatter (may be corrupted)"

### 3. Project State Check
- Run: `git status --short 2>/dev/null` to see current state
- Run: `git branch --show-current 2>/dev/null` to get current branch
- Note if not in git repository (context may be less complete)

## Instructions

### 1. Context Loading Sequence

Load context files in priority order for optimal understanding:

**Priority 1 - Essential Context (load first):**
1. `project-overview.md` - High-level understanding of the project
2. `project-brief.md` - Core purpose and goals
3. `tech-context.md` - Technical stack and dependencies

**Priority 2 - Current State (load second):**
4. `progress.md` - Current status and recent work
5. `project-structure.md` - Directory and file organization

**Priority 3 - Deep Context (load third):**
6. `system-patterns.md` - Architecture and design patterns
7. `product-context.md` - User needs and requirements
8. `project-style-guide.md` - Coding conventions
9. `project-vision.md` - Long-term direction

### 2. Validation During Loading

For each file loaded:
- Check frontmatter exists and parse:
  - `created` date should be valid
  - `last_updated` should be â‰¥ created date
  - `version` should be present
- If frontmatter is invalid, note but continue loading content
- Track which files loaded successfully vs failed

### 3. Supplementary Information

After loading context files:
- Run: `git ls-files --others --exclude-standard | head -20` to see untracked files
- Read `README.md` if it exists for additional project information
- Check for `.env.example` or similar for environment setup needs

### 4. Error Recovery

**If critical files are missing:**
- `project-overview.md` missing: Try to understand from README.md
- `tech-context.md` missing: Analyze package.json/requirements.txt directly
- `progress.md` missing: Check recent git commits for status

**If context is incomplete:**
- Inform user which files are missing
- Suggest running `/context:update` to refresh context
- Continue with partial context but note limitations

### 5. Loading Summary

Provide comprehensive summary after priming:

```
ðŸ§  Context Primed Successfully

ðŸ“– Loaded Context Files:
  âœ… Essential: {count}/3 files
  âœ… Current State: {count}/2 files
  âœ… Deep Context: {count}/4 files

ðŸ” Project Understanding:
  - Name: {project_name}
  - Type: {project_type}
  - Language: {primary_language}
  - Status: {current_status from progress.md}
  - Branch: {git_branch}

ðŸ“Š Key Metrics:
  - Last Updated: {most_recent_update}
  - Context Version: {version}
  - Files Loaded: {success_count}/{total_count}

âš ï¸ Warnings:
  {list any missing files or issues}

ðŸŽ¯ Ready State:
  âœ… Project context loaded
  âœ… Current status understood
  âœ… Ready for development work

ðŸ’¡ Project Summary:
  {2-3 sentence summary of what the project is and current state}
```

### 6. Partial Context Handling

If some files fail to load:
- Continue with available context
- Clearly note what's missing
- Suggest remediation:
  - "Missing technical context - run /context:create to rebuild"
  - "Progress file corrupted - run /context:update to refresh"

### 7. Performance Optimization

For large contexts:
- Load files in parallel when possible
- Show progress indicator: "Loading context files... {current}/{total}"
- Skip extremely large files (>10000 lines) with warning
- Cache parsed frontmatter for faster subsequent loads

## Important Notes

- **Always validate** files before attempting to read
- **Load in priority order** to get essential context first
- **Handle missing files gracefully** - don't fail completely
- **Provide clear summary** of what was loaded and project state
- **Note any issues** that might affect development work



