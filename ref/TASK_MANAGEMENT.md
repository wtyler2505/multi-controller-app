# Task Management Guide

## Task Management Protocol (MANDATORY)

### Pre-Work Verification

**ALWAYS execute this sequence before ANY implementation work:**

1. **Check Current Sprint Status**
   ```bash
   mcp__taskmaster-ai__next_task     # Identify next available task
   mcp__taskmaster-ai__get_task      # Get detailed requirements
   mcp__taskmaster-ai__complexity_report  # Review complexity analysis if exists
   ```

2. **Validate Dependencies**
   - Check if dependent tasks are complete: `mcp__taskmaster-ai__validate_dependencies`
   - Review blocking issues or deferred items
   - Confirm no conflicting work in progress

3. **Task Lifecycle Management**
   - **Starting**: `mcp__taskmaster-ai__set_task_status --id=<id> --status=in-progress`
   - **During**: Log ALL decisions/changes: `mcp__taskmaster-ai__update_subtask --id=<id> --prompt="<details>"`
   - **Completing**: 
     - Run all tests and linting
     - Verify performance budgets
     - `mcp__taskmaster-ai__set_task_status --id=<id> --status=review`
     - After review: `mcp__taskmaster-ai__set_task_status --id=<id> --status=done`

4. **Progress Documentation**
   - Log implementation approach before starting
   - Document any deviations from original plan
   - Record performance measurements
   - Note any technical debt incurred
   - Update with lessons learned

### Task ID Reference Format
- Always reference tasks in commits: `feat(component): description (task X.Y)`
- Include task ID in code comments for complex implementations
- Cross-reference related tasks in documentation

## Overview

The Multi-Controller App uses Task Master AI for comprehensive task management, enabling structured development workflow with AI assistance.

## Task Structure

### Task ID Format
- **Main tasks**: 1, 2, 3, etc.
- **Subtasks**: 1.1, 1.2, 2.1, etc.
- **Sub-subtasks**: 1.1.1, 1.1.2, etc.

### Task Status Values
- `pending` - Ready to work on
- `in-progress` - Currently being worked on
- `review` - Implementation complete, needs review
- `done` - Completed and verified
- `deferred` - Postponed
- `cancelled` - No longer needed
- `blocked` - Waiting on external factors

### Task Fields
```json
{
  "id": "1.2",
  "title": "Implement serial transport layer",
  "description": "Create async serial communication with reconnection",
  "status": "pending",
  "priority": "high",
  "dependencies": ["1.1"],
  "details": "Implementation requirements...",
  "testStrategy": "Unit tests for protocol, integration tests for hardware",
  "subtasks": []
}
```

## Essential Commands

### Daily Workflow
```bash
# View all tasks
task-master list
mcp__taskmaster-ai__get_tasks

# Get next available task
task-master next
mcp__taskmaster-ai__next_task

# View specific task details
task-master show 1.2
mcp__taskmaster-ai__get_task --id=1.2

# Update task status
task-master set-status --id=1.2 --status=in-progress
mcp__taskmaster-ai__set_task_status --id=1.2 --status=done
```

### Task Creation & Modification
```bash
# Add new task with AI assistance
task-master add-task --prompt="Create WebSocket transport" --research
mcp__taskmaster-ai__add_task

# Expand task into subtasks
task-master expand --id=2 --research
mcp__taskmaster-ai__expand_task --id=2

# Update task details
task-master update-task --id=1.2 --prompt="Add timeout handling"
mcp__taskmaster-ai__update_task

# Add implementation notes to subtask
task-master update-subtask --id=1.2.1 --prompt="Used tokio for async"
mcp__taskmaster-ai__update_subtask
```

### Analysis & Planning
```bash
# Analyze task complexity
task-master analyze-complexity --research
mcp__taskmaster-ai__analyze_project_complexity

# View complexity report
task-master complexity-report
mcp__taskmaster-ai__complexity_report

# Expand all eligible tasks
task-master expand --all --research
mcp__taskmaster-ai__expand_all
```

### Dependencies
```bash
# Add task dependency
task-master add-dependency --id=3 --depends-on=2
mcp__taskmaster-ai__add_dependency

# Validate dependencies
task-master validate-dependencies
mcp__taskmaster-ai__validate_dependencies

# Fix dependency issues
task-master fix-dependencies
mcp__taskmaster-ai__fix_dependencies
```

## Development Workflow Integration

### 1. Session Start
```bash
# Get current task
task-master next

# Review task details
task-master show <id>

# Mark as in-progress
task-master set-status --id=<id> --status=in-progress
```

### 2. During Development
```bash
# Log implementation decisions
task-master update-subtask --id=<id> --prompt="Chose approach X because..."

# Track blockers
task-master update-task --id=<id> --prompt="Blocked by missing dependency Y"
```

### 3. Task Completion
```bash
# Run tests
npm test

# Mark for review
task-master set-status --id=<id> --status=review

# After review, mark done
task-master set-status --id=<id> --status=done

# Get next task
task-master next
```

## Task-Driven Git Workflow

The project features deep integration between Task Master and git operations, automating the entire workflow from task selection to pull request creation.

### Automated Task-to-Git Pipeline

#### 1. Starting a Task
```bash
# Get next available task
task-master next                    # Shows task 11: Fix memory leak

# Create and switch to task branch automatically
npm run task:start 11               # Creates feature/task-11-fix-memory-leak
# OR
npm run task:branch 11              # Same as above
```

#### 2. During Development
```bash
# Make your changes
code src/transport/serial.ts

# Stage changes
git add .

# Use smart commit (recommended)
npm run task:commit                 # Interactive commit with task context
# Automatically:
# - Analyzes changed files to suggest commit type
# - Fetches task 11 details from Task Master
# - Adds task reference to commit message
# - Enforces conventional commit format

# Example generated commit:
# fix(transport): resolve memory leak in serial buffer (task 11)
```

#### 3. Creating Pull Request
```bash
# Create PR with task context
npm run task:pr                     # Creates PR for current branch
# OR specify task
npm run task:pr 11

# Automatically includes:
# - Task title in PR title
# - Task details in PR description
# - Implementation checklist
# - Performance impact assessment
# - Task reference for tracking
```

### Manual Git Integration (Fallback)

When automation is unavailable, maintain task references manually:

#### Commit Messages
```bash
git commit -m "feat(transport): implement serial reconnection (task 2.1)"
```

#### Pull Requests
```bash
gh pr create --title "Task 2.1: Serial reconnection logic" \
  --body "Implements automatic reconnection for serial transport as specified in task 2.1"
```

#### Branch Naming
```bash
git checkout -b feature/task-2.1-serial-reconnection
```

### Synchronization Monitoring

Track your local repository status against GitHub:

```bash
# Basic status check
npm run sync:status

# Detailed view with files
npm run sync:status-detailed

# Live monitoring
npm run sync:watch

# Auto-fix safe issues
npm run sync:auto
```

The sync dashboard shows:
- Local vs remote branch status
- Uncommitted changes
- Unpushed commits
- Stash status
- Actionable recommendations

## Configuration

### Task Files Location
- **Main database**: `.taskmaster/tasks/tasks.json`
- **Individual tasks**: `.taskmaster/tasks/task_XXX.txt`
- **Configuration**: `.taskmaster/config.json`
- **PRD**: `.taskmaster/docs/prd.txt`

### Model Configuration
```bash
# Interactive setup
task-master models --setup

# Set specific models
task-master models --set-main claude-3-5-sonnet-20241022
task-master models --set-research perplexity-llama-3.1-sonar-large-128k-online
```

## Best Practices

### 1. Task Granularity
- Main tasks: Major features or components
- Subtasks: Implementation steps
- Sub-subtasks: Specific code changes

### 2. Status Management
- Update status immediately when starting/completing
- Use `review` status before marking `done`
- Add notes when deferring or cancelling

### 3. Documentation
- Log decisions in task details
- Document blockers and solutions
- Reference external resources

### 4. Parallel Development
Use git worktrees for parallel task work:
```bash
git worktree add ../task-2.1 feature/task-2.1
git worktree add ../task-3.1 feature/task-3.1
```

## Troubleshooting

### Task File Sync Issues
```bash
# Regenerate task files
task-master generate
mcp__taskmaster-ai__generate
```

### Dependency Conflicts
```bash
# Check for circular dependencies
task-master validate-dependencies

# Auto-fix issues
task-master fix-dependencies
```

### Model Failures
```bash
# Check API keys
cat .env | grep API_KEY

# Test with different model
task-master models --set-fallback gpt-4o-mini
```

## Advanced Features

### Batch Operations
```bash
# Update multiple tasks from ID onwards
task-master update --from=5 --prompt="Change to use new API"

# Clear all subtasks
task-master clear-subtasks --all

# Move task to different position
task-master move --from=5 --to=3
```

### Research Mode
Add `--research` flag for AI-enhanced operations:
```bash
task-master add-task --prompt="WebSocket implementation" --research
task-master expand --id=2 --research
task-master analyze-complexity --research
```

### Custom Workflows
Create Claude Code slash commands in `.claude/commands/`:
- `/tm/next` - Get and show next task
- `/tm/complete` - Complete current task
- `/tm/status` - Show project status