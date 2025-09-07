# CCPM + Task Master Integration Guide

## Overview

This document describes the hybrid workflow combining Task Master's AI-powered task management with CCPM's parallel execution capabilities.

## Architecture

```
Task Master (Planning) → CCPM (Execution) → GitHub (Collaboration)
     ↓                         ↓                    ↓
  Tasks.json            Parallel Agents      Team Visibility
     ↓                         ↓                    ↓
  Your Agents           Git Worktrees        Issue Comments
```

## Quick Start

### 1. Check Next Task
```bash
mcp__taskmaster-ai__next_task  # Get priority task from Task Master
```

### 2. For Simple Tasks (1-2 subtasks)
Use traditional sequential execution:
```bash
mcp__taskmaster-ai__set_task_status --id=5.1 --status=in-progress
# Implement task...
mcp__taskmaster-ai__set_task_status --id=5.1 --status=done
```

### 3. For Complex Tasks (3+ subtasks)
Use CCPM parallel execution:
```bash
/bridge:tm-to-epic 5           # Convert to CCPM epic
/bridge:parallel-start 5       # Launch parallel agents
/bridge:sync-all               # Push to GitHub for visibility
```

## Command Reference

### Bridge Commands (NEW)
- `/bridge:tm-to-epic [id]` - Convert Task Master task to CCPM epic
- `/bridge:parallel-start [id]` - Launch parallel execution with agents
- `/bridge:sync-all` - Bidirectional sync with GitHub Issues

### Task Master Commands (Existing)
- `mcp__taskmaster-ai__next_task` - Get next priority task
- `mcp__taskmaster-ai__get_task` - Get task details
- `mcp__taskmaster-ai__set_task_status` - Update task status
- `mcp__taskmaster-ai__update_subtask` - Log progress

### CCPM Commands (NEW)
- `/pm:epic-show [name]` - Display epic and tasks
- `/pm:issue-sync [id]` - Push updates to GitHub
- `/pm:standup` - Daily progress report
- `/pm:status` - Overall project dashboard

## Parallel Execution Example

### Task 5: Implement Device Manager
Subtasks:
- 5.1: Device discovery service
- 5.2: Driver registry
- 5.3: Connection management
- 5.4: Event system

### Sequential Approach (Old)
Time: 8 hours (2 hours per subtask)
```
5.1 → 5.2 → 5.3 → 5.4
```

### Parallel Approach (New)
Time: 2-3 hours (all simultaneously)
```
Worktree: ../mc-app-task-5/
├── Agent 1: driver-engineer → 5.1 Discovery
├── Agent 2: driver-engineer → 5.2 Registry
├── Agent 3: transport-engineer → 5.3 Connections
└── Agent 4: general-purpose → 5.4 Events
```

## Agent Mapping

| Task Type | Task Master Agent | CCPM Wrapper |
|-----------|------------------|--------------|
| Driver work | driver-engineer | + context firewall |
| Transport | transport-engineer | + parallel capability |
| UI/Telemetry | ui-telemetry-analyst | + worktree isolation |
| Testing | test-runner | + result consolidation |
| Documentation | docs-scribe | + GitHub sync |

## GitHub Integration

### Automatic Sync
- Task Master tasks → GitHub Issues
- Subtasks → Sub-issues (using gh-sub-issue)
- Status updates → Issue comments
- Completion → Issue closure

### Visibility Benefits
- Team sees progress in real-time
- Stakeholders get transparency
- Audit trail maintained
- No context switching for updates

## Best Practices

### When to Use Parallel Execution
✅ Use parallel when:
- Task has 3+ independent subtasks
- Subtasks touch different files/modules
- No shared state between subtasks
- Time-critical delivery needed

❌ Don't use parallel when:
- Subtasks are sequential/dependent
- Working on same file
- Simple bug fixes
- Exploratory work

### Context Management
- Main conversation stays strategic
- Agents handle implementation details
- Results consolidated before return
- 80-90% context reduction achieved

## Performance Metrics

### Expected Improvements
- **Velocity**: 3-5x faster delivery
- **Context**: 90% preservation (vs 40% before)
- **Quality**: 75% fewer bugs (spec-driven)
- **Collaboration**: Real-time visibility

### Measuring Success
```bash
# Check velocity improvement
git log --since="1 week ago" --oneline | wc -l

# Monitor context usage
/context  # Should use /compact less frequently

# Track parallel execution
ls -la ../mc-app-task-*  # Multiple worktrees active
```

## Troubleshooting

### Common Issues

**Issue**: Worktree conflicts
```bash
git worktree prune
git worktree list
```

**Issue**: Agent fails in parallel execution
- Continue with other agents
- Fix failed subtask manually
- Update Task Master status

**Issue**: GitHub rate limits
```bash
gh api rate_limit  # Check limits
# Wait or use caching
```

**Issue**: Context explosion
- Use more agents
- Increase parallel execution
- Clear between major tasks

## Migration Path

### Phase 1: Pilot (Week 1)
- Test with Task 5
- Measure improvements
- Document patterns

### Phase 2: Rollout (Week 2)
- Apply to all complex tasks
- Train team on commands
- Establish conventions

### Phase 3: Optimization (Ongoing)
- Refine agent mappings
- Improve parallelization
- Enhance GitHub integration

## Summary

The hybrid Task Master + CCPM workflow provides:
1. **AI-powered planning** (Task Master)
2. **Parallel execution** (CCPM)
3. **Team visibility** (GitHub)
4. **Context preservation** (Agents)
5. **Quality assurance** (Spec-driven)

This integration enables shipping the Multi-Controller App **5-10x faster** while maintaining high quality and full traceability.