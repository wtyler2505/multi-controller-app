---
model: claude-sonnet-4-20250514
category: project-management
priority: high
tags: ["project-management", "github", "integration"]
description: Command for epic-sync operations
allowed-tools: Bash, Read, Write, LS, Task, mcp__taskmaster-ai__get_tasks, mcp__desktop-commander__read_multiple_files
argument-hint: <epic_name> | --dry-run | --force-update

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["epic-synchronization", "github-integration", "data-migration"]
    complexity-factors: ["multi-platform-sync", "data-transformation", "reference-updating"]
    specialized-tools: ["epic-management", "github-operations", "data-synchronization"]
  preferred-agents:
    primary: "task-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-executor"]
  tool-requirements:
    mcp-servers: ["taskmaster-ai", "desktop-commander", "cipher-memory"]
    specialized-functions: ["epic-synchronization", "github-integration"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "epic-synchronization + github-integration + data-migration"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "synchronization-patterns + integration-knowledge + migration-strategies"
    
    knowledge-preparation:
      - domain: "epic-synchronization"
      - pattern-search: "synchronization-strategies + integration-patterns + migration-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["sync-preparation", "data-migration", "reference-updating"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "synchronization-strategies + integration-approaches + migration-decisions"
      - pattern-recognition: "epic-synchronization-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["synchronization-results", "integration-insights", "migration-techniques"]
      - knowledge-extraction: "synchronization-methodologies + integration-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["synchronization-relationships", "integration-dependencies", "migration-connections"]
      - cross-reference: "related-synchronization-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "synchronization-knowledge + integration-patterns"
      - continuous-learning: "epic-synchronization-optimization"

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
    - sync-preparation
    - data-migration
    - reference-updating
    - github-operations
    - validation-checks
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "pm-epic-sync"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "synchronization-results + integration-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["synchronization-patterns", "integration-techniques", "migration-strategies"]
  learn-from: ["epic-create", "github-operations", "data-synchronization"]
  contribute-to: "epic-synchronization-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-sync-prerequisites
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-synchronization-operations
    - continuous-memory-updates
    - real-time-integration-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - synchronization-pattern-extraction
---

$/d'
    3. Create sub-issue using:
       - If gh-sub-issue available: 
         gh sub-issue create --parent $epic_number --title "$task_name" \
           --body-file /tmp/task-body.md --label "task,epic:$ARGUMENTS"
       - Otherwise: 
         gh issue create --title "$task_name" --body-file /tmp/task-body.md \
           --label "task,epic:$ARGUMENTS"
    4. Record: task_file:issue_number
    
    IMPORTANT: Always include --label parameter with "task,epic:$ARGUMENTS"
    
    Return mapping of files to issue numbers.
```

Consolidate results from parallel agents:
```bash
# Collect all mappings from agents
cat /tmp/batch-*/mapping.txt >> /tmp/task-mapping.txt

# IMPORTANT: After consolidation, follow step 3 to:
# 1. Build old->new ID mapping
# 2. Update all task references (depends_on, conflicts_with)
# 3. Rename files with proper frontmatter updates
```

### 3. Rename Task Files and Update References

First, build a mapping of old numbers to new issue IDs:
```bash
# Create mapping from old task numbers (001, 002, etc.) to new issue IDs
> /tmp/id-mapping.txt
while IFS=: read -r task_file task_number; do
  # Extract old number from filename (e.g., 001 from 001.md)
  old_num=$(basename "$task_file" .md)
  echo "$old_num:$task_number" >> /tmp/id-mapping.txt
done < /tmp/task-mapping.txt
```

Then rename files and update all references:
```bash
# Process each task file
while IFS=: read -r task_file task_number; do
  new_name="$(dirname "$task_file")/${task_number}.md"
  
  # Read the file content
  content=$(cat "$task_file")
  
  # Update depends_on and conflicts_with references
  while IFS=: read -r old_num new_num; do
    # Update arrays like [001, 002] to use new issue numbers
    content=$(echo "$content" | sed "s/\b$old_num\b/$new_num/g")
  done < /tmp/id-mapping.txt
  
  # Write updated content to new file
  echo "$content" > "$new_name"
  
  # Remove old file if different from new
  [ "$task_file" != "$new_name" ] && rm "$task_file"
  
  # Update github field in frontmatter
  # Add the GitHub URL to the frontmatter
  repo=$(gh repo view --json nameWithOwner -q .nameWithOwner)
  github_url="https://github.com/$repo/issues/$task_number"
  
  # Update frontmatter with GitHub URL and current timestamp
  current_date=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
  
  # Use sed to update the github and updated fields
  sed -i.bak "/^github:/c\github: $github_url" "$new_name"
  sed -i.bak "/^updated:/c\updated: $current_date" "$new_name"
  rm "${new_name}.bak"
done < /tmp/task-mapping.txt
```

### 4. Update Epic with Task List (Fallback Only)

If NOT using gh-sub-issue, add task list to epic:

```bash
if [ "$use_subissues" = false ]; then
  # Get current epic body
  gh issue view {epic_number} --json body -q .body > /tmp/epic-body.md
  
  # Append task list
  cat >> /tmp/epic-body.md << 'EOF'
  
  ## Tasks
  - [ ] #{task1_number} {task1_name}
  - [ ] #{task2_number} {task2_name}
  - [ ] #{task3_number} {task3_name}
  EOF
  
  # Update epic issue
  gh issue edit {epic_number} --body-file /tmp/epic-body.md
fi
```

With gh-sub-issue, this is automatic!

### 5. Update Epic File

Update the epic file with GitHub URL, timestamp, and real task IDs:

#### 5a. Update Frontmatter
```bash
# Get repo info
repo=$(gh repo view --json nameWithOwner -q .nameWithOwner)
epic_url="https://github.com/$repo/issues/$epic_number"
current_date=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Update epic frontmatter
sed -i.bak "/^github:/c\github: $epic_url" .claude/epics/$ARGUMENTS/epic.md
sed -i.bak "/^updated:/c\updated: $current_date" .claude/epics/$ARGUMENTS/epic.md
rm .claude/epics/$ARGUMENTS/epic.md.bak
```

#### 5b. Update Tasks Created Section
```bash
# Create a temporary file with the updated Tasks Created section
cat > /tmp/tasks-section.md << 'EOF'
## Tasks Created
EOF

# Add each task with its real issue number
for task_file in .claude/epics/$ARGUMENTS/[0-9]*.md; do
  [ -f "$task_file" ] || continue
  
  # Get issue number (filename without .md)
  issue_num=$(basename "$task_file" .md)
  
  # Get task name from frontmatter
  task_name=$(grep '^name:' "$task_file" | sed 's/^name: *//')
  
  # Get parallel status
  parallel=$(grep '^parallel:' "$task_file" | sed 's/^parallel: *//')
  
  # Add to tasks section
  echo "- [ ] #${issue_num} - ${task_name} (parallel: ${parallel})" >> /tmp/tasks-section.md
done

# Add summary statistics
total_count=$(ls .claude/epics/$ARGUMENTS/[0-9]*.md 2>/dev/null | wc -l)
parallel_count=$(grep -l '^parallel: true' .claude/epics/$ARGUMENTS/[0-9]*.md 2>/dev/null | wc -l)
sequential_count=$((total_count - parallel_count))

cat >> /tmp/tasks-section.md << EOF

Total tasks: ${total_count}
Parallel tasks: ${parallel_count}
Sequential tasks: ${sequential_count}
EOF

# Replace the Tasks Created section in epic.md
# First, create a backup
cp .claude/epics/$ARGUMENTS/epic.md .claude/epics/$ARGUMENTS/epic.md.backup

# Use awk to replace the section
awk '
  /^## Tasks Created/ { 
    skip=1
    while ((getline line < "/tmp/tasks-section.md") > 0) print line
    close("/tmp/tasks-section.md")
  }
  /^## / && !/^## Tasks Created/ { skip=0 }
  !skip && !/^## Tasks Created/ { print }
' .claude/epics/$ARGUMENTS/epic.md.backup > .claude/epics/$ARGUMENTS/epic.md

# Clean up
rm .claude/epics/$ARGUMENTS/epic.md.backup
rm /tmp/tasks-section.md
```

### 6. Create Mapping File

Create `.claude/epics/$ARGUMENTS/github-mapping.md`:
```bash
# Create mapping file
cat > .claude/epics/$ARGUMENTS/github-mapping.md << EOF
# GitHub Issue Mapping

Epic: #${epic_number} - https://github.com/${repo}/issues/${epic_number}

Tasks:
EOF

# Add each task mapping
for task_file in .claude/epics/$ARGUMENTS/[0-9]*.md; do
  [ -f "$task_file" ] || continue
  
  issue_num=$(basename "$task_file" .md)
  task_name=$(grep '^name:' "$task_file" | sed 's/^name: *//')
  
  echo "- #${issue_num}: ${task_name} - https://github.com/${repo}/issues/${issue_num}" >> .claude/epics/$ARGUMENTS/github-mapping.md
done

# Add sync timestamp
echo "" >> .claude/epics/$ARGUMENTS/github-mapping.md
echo "Synced: $(date -u +"%Y-%m-%dT%H:%M:%SZ")" >> .claude/epics/$ARGUMENTS/github-mapping.md
```

### 7. Create Worktree

Follow `/rules/worktree-operations.md` to create development worktree:

```bash
# Ensure main is current
git checkout main
git pull origin main

# Create worktree for epic
git worktree add ../epic-$ARGUMENTS -b epic/$ARGUMENTS

echo "âœ… Created worktree: ../epic-$ARGUMENTS"
```

### 8. Output

```
âœ… Synced to GitHub
  - Epic: #{epic_number} - {epic_title}
  - Tasks: {count} sub-issues created
  - Labels applied: epic, task, epic:{name}
  - Files renamed: 001.md â†’ {issue_id}.md
  - References updated: depends_on/conflicts_with now use issue IDs
  - Worktree: ../epic-$ARGUMENTS

Next steps:
  - Start parallel execution: /pm:epic-start $ARGUMENTS
  - Or work on single issue: /pm:issue-start {issue_number}
  - View epic: https://github.com/{owner}/{repo}/issues/{epic_number}
```

## Error Handling

Follow `/rules/github-operations.md` for GitHub CLI errors.

If any issue creation fails:
- Report what succeeded
- Note what failed
- Don't attempt rollback (partial sync is fine)

## Important Notes

- Trust GitHub CLI authentication
- Don't pre-check for duplicates
- Update frontmatter only after successful creation
- Keep operations simple and atomic


