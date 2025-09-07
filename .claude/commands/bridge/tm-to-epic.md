---
allowed-tools: Read, Write, LS, Bash, mcp__taskmaster-ai__get_task
---

# Bridge: Task Master to CCPM Epic

Convert a Task Master task into a CCPM epic format for parallel execution.

## Usage
```
/bridge:tm-to-epic [task-id]
```

## Steps

1. **Read Task from Task Master**
   - Use mcp__taskmaster-ai__get_task to fetch task details
   - Extract title, description, subtasks, dependencies

2. **Create Epic File**
   - Create `.claude/epics/[task-name]/epic.md`
   - Format:
     ```markdown
     # Epic: [Task Title]
     
     ## Overview
     [Task description and details]
     
     ## Acceptance Criteria
     - [From task test strategy]
     
     ## Technical Approach
     [From task details]
     
     ## Tasks
     [Subtasks converted to CCPM format]
     
     ## Dependencies
     [Task dependencies]
     
     ## Parallelization
     parallel: true (if subtasks can run concurrently)
     ```

3. **Create Task Files**
   - For each subtask, create `001.md`, `002.md`, etc.
   - Include parallel execution flags
   - Map to specialized agents

4. **Link Back to Task Master**
   - Store Task Master ID in epic metadata
   - Enable bidirectional sync

5. **Report Success**
   - Show created files
   - Provide next command to execute

## Error Handling
- If task doesn't exist, show available tasks
- If epic already exists, ask to overwrite
- Preserve all Task Master metadata