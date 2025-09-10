---
model: claude-sonnet-4-20250514
category: task-management
priority: high
tags: ["task-management", "tasks"]
description: Display all tasks with their current status and details

# Agent and Tool Integration
assigned-agent: task-orchestrator
required-tools:
  - "mcp__taskmaster-ai__get_tasks"
  - "mcp__taskmaster-ai__complexity_report"
  - "mcp__cipher-memory__search_nodes"
tool-chain: task-coordination
auto-deploy: true

# Workflow Configuration
pre-execution:
  validate-tools: true
  load-context: true
post-execution:
  store-results: false
  update-tasks: false
---

List tasks with intelligent argument parsing.

Parse arguments to determine filters and display options:
- Status: pending, in-progress, done, review, deferred, cancelled
- Priority: high, medium, low (or priority:high)
- Special: subtasks, tree, dependencies, blocked
- IDs: Direct numbers (e.g., "1,3,5" or "1-5")
- Complex: "pending high" = pending AND high priority

Arguments: $ARGUMENTS

Let me parse your request intelligently:

1. **Detect Filter Intent**
   - If arguments contain status keywords â†’ filter by status
   - If arguments contain priority â†’ filter by priority
   - If arguments contain "subtasks" â†’ include subtasks
   - If arguments contain "tree" â†’ hierarchical view
   - If arguments contain numbers â†’ show specific tasks
   - If arguments contain "blocked" â†’ show blocked tasks only

2. **Smart Combinations**
   Examples of what I understand:
   - "pending high" â†’ pending tasks with high priority
   - "done today" â†’ tasks completed today
   - "blocked" â†’ tasks with unmet dependencies
   - "1-5" â†’ tasks 1 through 5
   - "subtasks tree" â†’ hierarchical view with subtasks

3. **Execute Appropriate Query**
   Based on parsed intent, run the most specific task-master command

4. **Enhanced Display**
   - Group by relevant criteria
   - Show most important information first
   - Use visual indicators for quick scanning
   - Include relevant metrics

5. **Intelligent Suggestions**
   Based on what you're viewing, suggest next actions:
   - Many pending? â†’ Suggest priority order
   - Many blocked? â†’ Show dependency resolution
   - Looking at specific tasks? â†’ Show related tasks


