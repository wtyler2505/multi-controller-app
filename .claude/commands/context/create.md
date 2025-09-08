---
model: claude-sonnet-4-20250514
category: context-management
priority: high
tags: ["context-management"]
description: Create comprehensive initial project context
allowed-tools: Bash, Read, Write, LS
argument-hint: [context-type] | --full | --minimal | --custom

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["context-creation", "project-analysis", "documentation-generation"]
    complexity-factors: ["project-understanding", "context-synthesis", "documentation-structuring"]
    specialized-tools: ["project-analysis", "context-generation", "documentation-tools"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "documentation-specialist"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "FileScopeMCP", "cipher-memory"]
    specialized-functions: ["context-creation", "project-analysis"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "context-creation + project-analysis + documentation-generation"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "context-patterns + analysis-knowledge"
    
    knowledge-preparation:
      - domain: "context-management"
      - pattern-search: "context-strategies + analysis-patterns + documentation-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["context-analysis", "documentation-creation", "project-mapping"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "context-strategies + documentation-approaches + analysis-decisions"
      - pattern-recognition: "context-creation-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["context-results", "analysis-insights", "documentation-techniques"]
      - knowledge-extraction: "context-methodologies + analysis-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["context-relationships", "analysis-dependencies", "documentation-connections"]
      - cross-reference: "related-context-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "context-knowledge + analysis-patterns"
      - continuous-learning: "context-creation-optimization"

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
    - context-analysis
    - documentation-creation
    - project-mapping
    - structure-analysis
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "context-create"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "context-creation-results + analysis-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["context-patterns", "analysis-techniques", "documentation-strategies"]
  learn-from: ["context-prime", "context-update", "project-analysis"]
  contribute-to: "context-management-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-project-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-context-analysis
    - continuous-memory-updates
    - real-time-documentation-generation
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - context-pattern-extraction
---

```

Generate the following initial context files:
  - `progress.md` - Document current project status, completed work, and immediate next steps
    - Include: Current branch, recent commits, outstanding changes
  - `project-structure.md` - Map out the directory structure and file organization
    - Include: Key directories, file naming patterns, module organization
  - `tech-context.md` - Catalog current dependencies, technologies, and development tools
    - Include: Language version, framework versions, dev dependencies
  - `system-patterns.md` - Identify existing architectural patterns and design decisions
    - Include: Design patterns observed, architectural style, data flow
  - `product-context.md` - Define product requirements, target users, and core functionality
    - Include: User personas, core features, use cases
  - `project-brief.md` - Establish project scope, goals, and key objectives
    - Include: What it does, why it exists, success criteria
  - `project-overview.md` - Provide a high-level summary of features and capabilities
    - Include: Feature list, current state, integration points
  - `project-vision.md` - Articulate long-term vision and strategic direction
    - Include: Future goals, potential expansions, strategic priorities
  - `project-style-guide.md` - Document coding standards, conventions, and style preferences
    - Include: Naming conventions, file structure patterns, comment style
### 4. Quality Validation

After creating each file:
- Verify file was created successfully
- Check file is not empty (minimum 10 lines of content)
- Ensure frontmatter is present and valid
- Validate markdown formatting is correct

### 5. Error Handling

**Common Issues:**
- **No write permissions:** "âŒ Cannot write to .claude/context/. Check permissions."
- **Disk space:** "âŒ Insufficient disk space for context files."
- **File creation failed:** "âŒ Failed to create {filename}. Error: {error}"

If any file fails to create:
- Report which files were successfully created
- Provide option to continue with partial context
- Never leave corrupted or incomplete files

### 6. Post-Creation Summary

Provide comprehensive summary:
```
ðŸ“‹ Context Creation Complete

ðŸ“ Created context in: .claude/context/
âœ… Files created: {count}/9

ðŸ“Š Context Summary:
  - Project Type: {detected_type}
  - Language: {primary_language}
  - Git Status: {clean/changes}
  - Dependencies: {count} packages

ðŸ“ File Details:
  âœ… progress.md ({lines} lines) - Current status and recent work
  âœ… project-structure.md ({lines} lines) - Directory organization
  [... list all files with line counts and brief description ...]

â° Created: {timestamp}
ðŸ”„ Next: Use /context:prime to load context in new sessions
ðŸ’¡ Tip: Run /context:update regularly to keep context current
```

## Context Gathering Commands

Use these commands to gather project information:
- Target directory: `.claude/context/` (create if needed)
- Current git status: `git status --short`
- Recent commits: `git log --oneline -10`
- Project README: Read `README.md` if exists
- Package files: Check for `package.json`, `requirements.txt`, `Cargo.toml`, `go.mod`, etc.
- Documentation scan: `find . -type f -name '*.md' -path '*/docs/*' 2>/dev/null | head -10`
- Test detection: `find . -type d \( -name 'test' -o -name 'tests' -o -name '__tests__' -o -name 'spec' \) 2>/dev/null | head -5`

## Important Notes

- **Always use real datetime** from system clock, never placeholders
- **Ask for confirmation** before overwriting existing context
- **Validate each file** is created successfully
- **Provide detailed summary** of what was created
- **Handle errors gracefully** with specific guidance

$ARGUMENTS



