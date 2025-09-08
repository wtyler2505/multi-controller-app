---
model: claude-sonnet-4-20250514
category: taskmaster-core
priority: medium
tags: ["taskmaster-core"]
description: Command for learn operations
---

Learn about Task Master capabilities through interactive exploration.

Arguments: $ARGUMENTS

## Interactive Task Master Learning

Based on your input, I'll help you discover capabilities:

### 1. **What are you trying to do?**

If $ARGUMENTS contains:
- "start" / "begin" â†’ Show project initialization workflows
- "manage" / "organize" â†’ Show task management commands  
- "automate" / "auto" â†’ Show automation workflows
- "analyze" / "report" â†’ Show analysis tools
- "fix" / "problem" â†’ Show troubleshooting commands
- "fast" / "quick" â†’ Show efficiency shortcuts

### 2. **Intelligent Suggestions**

Based on your project state:

**No tasks yet?**
```
You'll want to start with:
1. /project:task-master:init <prd-file>
   â†’ Creates tasks from requirements
   
2. /project:task-master:parse-prd <file>
   â†’ Alternative task generation

Try: /project:task-master:init demo-prd.md
```

**Have tasks?**
Let me analyze what you might need...
- Many pending tasks? â†’ Learn sprint planning
- Complex tasks? â†’ Learn task expansion
- Daily work? â†’ Learn workflow automation

### 3. **Command Discovery**

**By Category:**
- ðŸ“‹ Task Management: list, show, add, update, complete
- ðŸ”„ Workflows: auto-implement, sprint-plan, daily-standup
- ðŸ› ï¸ Utilities: check-health, complexity-report, sync-memory
- ðŸ” Analysis: validate-deps, show dependencies

**By Scenario:**
- "I want to see what to work on" â†’ `/project:task-master:next`
- "I need to break this down" â†’ `/project:task-master:expand <id>`
- "Show me everything" â†’ `/project:task-master:status`
- "Just do it for me" â†’ `/project:workflows:auto-implement`

### 4. **Power User Patterns**

**Command Chaining:**
```
/project:task-master:next
/project:task-master:start <id>
/project:workflows:auto-implement
```

**Smart Filters:**
```
/project:task-master:list pending high
/project:task-master:list blocked
/project:task-master:list 1-5 tree
```

**Automation:**
```
/project:workflows:pipeline init â†’ expand-all â†’ sprint-plan
```

### 5. **Learning Path**

Based on your experience level:

**Beginner Path:**
1. init â†’ Create project
2. status â†’ Understand state
3. next â†’ Find work
4. complete â†’ Finish task

**Intermediate Path:**
1. expand â†’ Break down complex tasks
2. sprint-plan â†’ Organize work
3. complexity-report â†’ Understand difficulty
4. validate-deps â†’ Ensure consistency

**Advanced Path:**
1. pipeline â†’ Chain operations
2. smart-flow â†’ Context-aware automation
3. Custom commands â†’ Extend the system

### 6. **Try This Now**

Based on what you asked about, try:
[Specific command suggestion based on $ARGUMENTS]

Want to learn more about a specific command?
Type: /project:help <command-name>


