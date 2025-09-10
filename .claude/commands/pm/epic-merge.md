---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github"]
description: Command for epic-merge operations
allowed-tools: Bash, Read, Write, mcp__taskmaster-ai__get_tasks, mcp__desktop-commander__read_file
argument-hint: <epic_name> | --strategy=<merge_strategy> | --no-cleanup

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-merging", "git-workflows", "conflict-resolution"]
    complexity-factors: ["merge-strategy", "conflict-detection", "branch-management"]
    specialized-tools: ["git-operations", "merge-tools", "conflict-resolution"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["epic-merging", "git-operations"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-merging + git-workflows + conflict-resolution"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "merging-patterns + git-knowledge + conflict-strategies"
    
    knowledge-preparation:
      - domain: "epic-merging"
      - pattern-search: "merging-strategies + git-patterns + conflict-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["merge-preparation", "conflict-detection", "resolution-strategies"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "merging-strategies + conflict-approaches + resolution-decisions"
      - pattern-recognition: "epic-merging-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["merging-results", "conflict-insights", "resolution-techniques"]
      - knowledge-extraction: "merging-methodologies + git-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["merging-relationships", "git-dependencies", "conflict-connections"]
      - cross-reference: "related-merging-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "merging-knowledge + git-patterns"
      - continuous-learning: "epic-merging-optimization"

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
    - merge-preparation
    - conflict-detection
    - resolution-strategies
    - git-operations
    - cleanup-procedures
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-merge"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "merging-results + conflict-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["merging-patterns", "conflict-techniques", "git-strategies"]
  learn-from: ["epic-start", "epic-status", "git-operations"]
  contribute-to: "epic-merging-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-epic-state
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-validation-checks
    - continuous-memory-updates
    - real-time-conflict-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - merging-pattern-extraction
---

# Epic Merge

Merge completed epic from worktree back to main branch.

## Usage
```
/pm:epic-merge <epic_name>
```

## Quick Check

1. **Verify worktree exists:**
   ```bash
   git worktree list | grep "epic-$ARGUMENTS" || echo "âŒ No worktree for epic: $ARGUMENTS"
   ```

2. **Check for active agents:**
   Read `.claude/epics/$ARGUMENTS/execution-status.md`
   If active agents exist: "âš ï¸ Active agents detected. Stop them first with: /pm:epic-stop $ARGUMENTS"

## Instructions

### 1. Pre-Merge Validation

Navigate to worktree and check status:
```bash
cd ../epic-$ARGUMENTS

# Check for uncommitted changes
if [[ $(git status --porcelain) ]]; then
  echo "âš ï¸ Uncommitted changes in worktree:"
  git status --short
  echo "Commit or stash changes before merging"
  exit 1
fi

# Check branch status
git fetch origin
git status -sb
```

### 2. Run Tests (Optional but Recommended)

```bash
# Look for test commands
if [ -f package.json ]; then
  npm test || echo "âš ï¸ Tests failed. Continue anyway? (yes/no)"
elif [ -f Makefile ]; then
  make test || echo "âš ï¸ Tests failed. Continue anyway? (yes/no)"
fi
```

### 3. Update Epic Documentation

Get current datetime: `date -u +"%Y-%m-%dT%H:%M:%SZ"`

Update `.claude/epics/$ARGUMENTS/epic.md`:
- Set status to "completed"
- Update completion date
- Add final summary

### 4. Attempt Merge

```bash
# Return to main repository
cd {main-repo-path}

# Ensure main is up to date
git checkout main
git pull origin main

# Attempt merge
echo "Merging epic/$ARGUMENTS to main..."
git merge epic/$ARGUMENTS --no-ff -m "Merge epic: $ARGUMENTS

Completed features:
$(cd .claude/epics/$ARGUMENTS && ls *.md | grep -E '^[0-9]+' | while read f; do
  echo "- $(grep '^name:' $f | cut -d: -f2)"
done)

Closes epic #$(grep 'github:' .claude/epics/$ARGUMENTS/epic.md | grep -oE '#[0-9]+')"
```

### 5. Handle Merge Conflicts

If merge fails with conflicts:
```bash
# Check conflict status
git status

echo "
âŒ Merge conflicts detected!

Conflicts in:
$(git diff --name-only --diff-filter=U)

Options:
1. Resolve manually:
   - Edit conflicted files
   - git add {files}
   - git commit
   
2. Abort merge:
   git merge --abort
   
3. Get help:
   /pm:epic-resolve $ARGUMENTS

Worktree preserved at: ../epic-$ARGUMENTS
"
exit 1
```

### 6. Post-Merge Cleanup

If merge succeeds:
```bash
# Push to remote
git push origin main

# Clean up worktree
git worktree remove ../epic-$ARGUMENTS
echo "âœ… Worktree removed: ../epic-$ARGUMENTS"

# Delete branch
git branch -d epic/$ARGUMENTS
git push origin --delete epic/$ARGUMENTS 2>/dev/null || true

# Archive epic locally
mkdir -p .claude/epics/archived/
mv .claude/epics/$ARGUMENTS .claude/epics/archived/
echo "âœ… Epic archived: .claude/epics/archived/$ARGUMENTS"
```

### 7. Update GitHub Issues

Close related issues:
```bash
# Get issue numbers from epic
epic_issue=$(grep 'github:' .claude/epics/archived/$ARGUMENTS/epic.md | grep -oE '[0-9]+$')

# Close epic issue
gh issue close $epic_issue -c "Epic completed and merged to main"

# Close task issues
for task_file in .claude/epics/archived/$ARGUMENTS/[0-9]*.md; do
  issue_num=$(grep 'github:' $task_file | grep -oE '[0-9]+$')
  if [ ! -z "$issue_num" ]; then
    gh issue close $issue_num -c "Completed in epic merge"
  fi
done
```

### 8. Final Output

```
âœ… Epic Merged Successfully: $ARGUMENTS

Summary:
  Branch: epic/$ARGUMENTS â†’ main
  Commits merged: {count}
  Files changed: {count}
  Issues closed: {count}
  
Cleanup completed:
  âœ“ Worktree removed
  âœ“ Branch deleted
  âœ“ Epic archived
  âœ“ GitHub issues closed
  
Next steps:
  - Deploy changes if needed
  - Start new epic: /pm:prd-new {feature}
  - View completed work: git log --oneline -20
```

## Conflict Resolution Help

If conflicts need resolution:
```
The epic branch has conflicts with main.

This typically happens when:
- Main has changed since epic started
- Multiple epics modified same files
- Dependencies were updated

To resolve:
1. Open conflicted files
2. Look for <<<<<<< markers
3. Choose correct version or combine
4. Remove conflict markers
5. git add {resolved files}
6. git commit
7. git push

Or abort and try later:
  git merge --abort
```

## Important Notes

- Always check for uncommitted changes first
- Run tests before merging when possible
- Use --no-ff to preserve epic history
- Archive epic data instead of deleting
- Close GitHub issues to maintain sync


