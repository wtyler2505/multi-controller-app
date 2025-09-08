---
model: claude-sonnet-4-20250514
category: taskmaster-core
priority: medium
tags: ["taskmaster-core"]
description: Command for project-status operations
---

Enhanced status command with comprehensive project insights.

Arguments: $ARGUMENTS

## Intelligent Status Overview

### 1. **Executive Summary**
Quick dashboard view:
- ðŸƒ Active work (in-progress tasks)
- ðŸ“Š Progress metrics (% complete, velocity)
- ðŸš§ Blockers and risks
- â±ï¸ Time analysis (estimated vs actual)
- ðŸŽ¯ Sprint/milestone progress

### 2. **Contextual Analysis**

Based on $ARGUMENTS, focus on:
- "sprint" â†’ Current sprint progress and burndown
- "blocked" â†’ Dependency chains and resolution paths
- "team" â†’ Task distribution and workload
- "timeline" â†’ Schedule adherence and projections
- "risk" â†’ High complexity or overdue items

### 3. **Smart Insights**

**Workflow Health:**
- Idle tasks (in-progress > 24h without updates)
- Bottlenecks (multiple tasks waiting on same dependency)
- Quick wins (low complexity, high impact)

**Predictive Analytics:**
- Completion projections based on velocity
- Risk of missing deadlines
- Recommended task order for optimal flow

### 4. **Visual Intelligence**

Dynamic visualization based on data:
```
Sprint Progress: â–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–‘â–‘ 80% (16/20 tasks)
Velocity Trend: â†—ï¸ +15% this week
Blocked Tasks:  ðŸ”´ 3 critical path items

Priority Distribution:
High:   â–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆâ–ˆ 8 tasks (2 blocked)
Medium: â–ˆâ–ˆâ–ˆâ–ˆâ–‘â–‘â–‘â–‘ 4 tasks
Low:    â–ˆâ–ˆâ–‘â–‘â–‘â–‘â–‘â–‘ 2 tasks
```

### 5. **Actionable Recommendations**

Based on analysis:
1. **Immediate actions** (unblock critical path)
2. **Today's focus** (optimal task sequence)
3. **Process improvements** (recurring patterns)
4. **Resource needs** (skills, time, dependencies)

### 6. **Historical Context**

Compare to previous periods:
- Velocity changes
- Pattern recognition
- Improvement areas
- Success patterns to repeat


