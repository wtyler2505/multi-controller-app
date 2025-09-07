---
allowed-tools: Read, Write, LS, Bash, Task, mcp__taskmaster-ai__get_task, mcp__taskmaster-ai__set_task_status
---

# Bridge: Parallel Start from Task Master

Launch parallel execution of a Task Master task using CCPM agents and worktrees.

## Usage
```
/bridge:parallel-start [task-id]
```

## Steps

1. **Validate Task**
   - Use mcp__taskmaster-ai__get_task to fetch task
   - Ensure task has subtasks
   - Check dependencies are met

2. **Create Worktree**
   ```bash
   git worktree add ../mc-app-task-[id] feature/task-[id]
   ```

3. **Set Task Status**
   - Update Task Master: status = "in-progress"
   - Log start time and execution mode

4. **Analyze Parallelization**
   - Identify which subtasks can run concurrently
   - Map subtasks to specialized agents:
     - Hardware tasks → driver-engineer
     - Transport tasks → transport-engineer
     - UI tasks → ui-telemetry-analyst
     - Testing tasks → test-runner

5. **Launch Parallel Agents**
   ```
   Task: parallel-worker
   Prompt: Execute subtasks [list] in worktree [path] using agents [map]
   ```

6. **Monitor Progress**
   - Track agent completion
   - Update Task Master subtask statuses
   - Consolidate results

7. **Report Status**
   - Show active agents
   - Display worktree path
   - Provide monitoring commands

## Agent Mapping

| Subtask Type | Agent |
|-------------|-------|
| Driver implementation | driver-engineer |
| Serial/TCP/UDP | transport-engineer |
| UI components | ui-telemetry-analyst |
| Performance testing | performance-profiler |
| Security validation | security-hygiene |
| Documentation | docs-scribe |

## Error Handling
- If worktree exists, use existing
- If agents fail, report and continue others
- Maintain Task Master sync throughout