# Task Master (`/tm:`) Workflow Guide (Printable)

## 1. Project Initialization
1. `/tm:init:init-project`: Initialize Task Master.
2. Create `prd.txt` in `.taskmaster/docs/`.
3. `/tm:parse-prd:parse-prd`: Parse the PRD to create tasks.

## 2. Task Analysis and Decomposition
1. `/tm:analyze-complexity:analyze-complexity`: Analyze complexity of tasks.
2. `/tm:complexity-report:complexity-report`: Review the report.
3. `/tm:expand:expand-all-tasks`: Break down all complex tasks into subtasks.

## 3. Daily Development Loop
1. `/tm:next:next-task`: Get your next task.
2. `/tm:show:show-task --id=<id>`: Review task details.
3. `/tm:set-status:to-in-progress --id=<id>`: Mark task as started.
4. Implement code (using `/implement`, `/test`, etc.).
5. `/tm:add-subtask:add-subtask --parent-id=<id> ...`: Add minor to-dos as they arise.
6. `/tm:set-status:to-done --id=<id>`: Mark task as finished.

## 4. Managing Tasks and Dependencies
- `/tm:add-task:add-task --title=...`: Add a new task.
- `/tm:update:update-single-task --id=<id> ...`: Update an existing task.
- `/tm:add-dependency:add-dependency --id=2.2 --depends-on=2.1`: Add a dependency.
- `/tm:validate-dependencies:validate-dependencies`: Check for dependency issues.

## 5. Reporting and Status
- `/tm:list:list-tasks --status in-progress`: List all in-progress tasks.
- `/tm:status:project-status`: Get a project status overview.
- `/tm:sync-readme:sync-readme`: Export tasks to a README file.