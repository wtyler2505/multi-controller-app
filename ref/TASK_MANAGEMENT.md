# Task Management Reference

## Overview

This project uses **Task Master AI** for comprehensive task management, providing AI-powered task generation, tracking, and workflow orchestration.

## Current Task Status

- **Total Tasks**: 10 main tasks
- **Total Subtasks**: 50 subtasks
- **Completion**: 0% (0/10 tasks, 0/50 subtasks)
- **Current Phase**: Milestone 1 - Project Setup & Prototyping

## Task Structure

### Task ID Format

- Main tasks: `1`, `2`, `3`, ...
- Subtasks: `1.1`, `1.2`, `2.1`, ...
- Sub-subtasks: `1.1.1`, `1.1.2`, ...

### Task Status Values

- `pending` - Ready to work on
- `in-progress` - Currently being worked on
- `done` - Completed and verified
- `review` - Awaiting review
- `blocked` - Waiting on dependencies
- `deferred` - Postponed
- `cancelled` - No longer needed

## Milestone Breakdown

### Milestone 1: Project Setup & Prototyping

**Tasks 1-4** | **Status**: Pending

1. **Verify Development Environment**
   - Check .NET 8 SDK, Rust toolchain, Node.js 18+
   - 5 subtasks | Complexity: 4

2. **Scaffold Project Repository**
   - Create directory structure and config files
   - 5 subtasks | Complexity: 5 | Depends on: Task 1

3. **Prototype UI with Serial Echo**
   - Build C# and Rust prototypes for comparison
   - 5 subtasks | Complexity: 7 | Depends on: Task 1

4. **Compare Prototypes and Decide Stack**
   - Analyze performance and select technology
   - 5 subtasks | Complexity: 6 | Depends on: Task 3

### Milestone 2: Core Infrastructure

**Tasks 5-6** | **Status**: Pending

5. **Implement Device Abstraction Layer**
   - IDeviceDriver and IDeviceSession interfaces
   - 5 subtasks | Complexity: 8 | Depends on: Task 4

6. **Develop Transport Layer**
   - Serial, TCP/UDP, SSH implementations
   - 5 subtasks | Complexity: 8 | Depends on: Task 5

### Milestone 3: UI & Scripting

**Tasks 7-8** | **Status**: Pending

7. **Build Single-Window UI**
   - Main window with tabs and controls
   - 5 subtasks | Complexity: 7 | Depends on: Task 6

8. **Integrate Scripting Runtime**
   - Embed JS/Lua/Python with sandboxed API
   - 5 subtasks | Complexity: 7 | Depends on: Task 7

### Milestone 4: Features & Testing

**Tasks 9-10** | **Status**: Pending

9. **Implement Telemetry, Profiles, Logging**
   - Real-time charts, config management, logs
   - 5 subtasks | Complexity: 6 | Depends on: Task 8

10. **Automated Testing and Acceptance**
    - Unit, loopback, soak, and acceptance tests
    - 5 subtasks | Complexity: 6 | Depends on: Task 9

## Task Management Commands

### CLI Commands

```bash
# View tasks
task-master list                    # Show all tasks
task-master next                    # Get next available task
task-master show <id>              # View task details

# Update tasks
task-master set-status --id=<id> --status=done
task-master update-task --id=<id> --prompt="changes"
task-master update-subtask --id=<id> --prompt="notes"

# Task operations
task-master expand --id=<id> --research
task-master add-dependency --id=<id> --depends-on=<id>
task-master move --from=<id> --to=<id>
```

### MCP Tools

```javascript
// Get tasks
mcp__taskmaster - ai__get_tasks;
mcp__taskmaster - ai__next_task;
mcp__taskmaster - ai__get_task;

// Update status
mcp__taskmaster - ai__set_task_status;

// Manage tasks
mcp__taskmaster - ai__expand_task;
mcp__taskmaster - ai__update_task;
mcp__taskmaster - ai__add_dependency;
```

## Task Files

### Primary Files

- `.taskmaster/tasks/tasks.json` - Main task database
- `.taskmaster/tasks/task_*.txt` - Individual task files
- `.taskmaster/reports/task-complexity-report.json` - Complexity analysis
- `.taskmaster/config.json` - AI model configuration

### Never Manually Edit

- `tasks.json` - Use commands only
- `config.json` - Use `task-master models`
- Task markdown files - Auto-generated

## Development Workflow

### Starting a Task

1. Get next task: `mcp__taskmaster-ai__next_task`
2. Review details: `mcp__taskmaster-ai__get_task`
3. Set in-progress: `mcp__taskmaster-ai__set_task_status`
4. Implement solution
5. Update with notes: `mcp__taskmaster-ai__update_subtask`
6. Mark complete: `mcp__taskmaster-ai__set_task_status`

### Task Expansion

1. Analyze complexity: `mcp__taskmaster-ai__analyze_project_complexity`
2. Expand eligible tasks: `mcp__taskmaster-ai__expand_task`
3. Review subtasks: `mcp__taskmaster-ai__get_task`
4. Adjust if needed: `mcp__taskmaster-ai__update_task`

### Dependency Management

1. Check dependencies: `mcp__taskmaster-ai__validate_dependencies`
2. Add new dependencies: `mcp__taskmaster-ai__add_dependency`
3. Fix issues: `mcp__taskmaster-ai__fix_dependencies`

## Best Practices

1. **Always track progress**: Update task status immediately
2. **Document as you go**: Use update_subtask for implementation notes
3. **Respect dependencies**: Don't start blocked tasks
4. **Use AI assistance**: Add `--research` for complex tasks
5. **Keep tasks atomic**: One clear outcome per task
6. **Regular status checks**: Run `next_task` frequently

## Task Prioritization

### High Priority

- Environment setup (Task 1)
- Repository scaffolding (Task 2)
- Prototype development (Task 3)
- Technology decision (Task 4)

### Medium Priority

- Scripting integration (Task 8)
- Telemetry and profiles (Task 9)

### Completion Criteria

- All subtasks marked done
- Tests passing
- Documentation updated
- Performance budgets met

## Integration with Claude Code

### Slash Commands

Located in `.claude/commands/tm/`:

- `/tm next` - Get next task
- `/tm show <id>` - Show task details
- `/tm done <id>` - Mark task complete
- `/tm status` - Project status overview

### Auto-Context

Task Master configuration is automatically loaded via:

- `CLAUDE.md` imports
- `.taskmaster/CLAUDE.md` guide
- MCP server connection
