# Claude Code Slash Command Reference (Printable)

This document provides a comprehensive, categorized list of all available slash commands.

## Task Master (`/tm:`)

The `/tm:` command group is a powerful, integrated project management suite.

### Project & Task Initialization
- `/tm:init:init-project`: Initialize a new Task Master project.
- `/tm:init:init-project-quick`: Quick initialization with auto-confirmation.
- `/tm:parse-prd:parse-prd`: Parse a PRD document to generate tasks.
- `/tm:parse-prd:parse-prd-with-research`: Parse PRD with enhanced research mode.
- `/tm:add-task:add-task`: Add new tasks with intelligent parsing.

### Task & Subtask Management
- `/tm:list:list-tasks`: List tasks with intelligent argument parsing.
- `/tm:list:list-tasks-by-status`: List tasks filtered by a specific status.
- `/tm:list:list-tasks-with-subtasks`: List all tasks including their subtasks.
- `/tm:show:show-task`: Show detailed task information.
- `/tm:update:update-single-task`: Update a single specific task.
- `/tm:update:update-task`: Update tasks with intelligent field detection.
- `/tm:update:update-tasks-from-id`: Update multiple tasks starting from a specific ID.
- `/tm:remove-task:remove-task`: Remove a task permanently.
- `/tm:add-subtask:add-subtask`: Add a subtask to a parent task.
- `/tm:add-subtask:convert-task-to-subtask`: Convert an existing task into a subtask.
- `/tm:remove-subtask:remove-subtask`: Remove a subtask from its parent.
- `/tm:clear-subtasks:clear-subtasks`: Clear all subtasks from a specific task.
- `/tm:clear-subtasks:clear-all-subtasks`: Clear all subtasks from all tasks globally.

### Status & Workflow
- `/tm:next:next-task`: Intelligently determine and prepare the next action.
- `/tm:set-status:to-pending`: Set a task's status to pending.
- `/tm:set-status:to-in-progress`: Start working on a task.
- `/tm:set-status:to-review`: Set a task's status to review.
- `/tm:set-status:to-done`: Mark a task as completed.
- `/tm:set-status:to-deferred`: Defer a task for later.
- `/tm:set-status:to-cancelled`: Cancel a task permanently.
- `/tm:status:project-status`: Show enhanced status with comprehensive project insights.

### Complexity, Dependencies & Analysis
- `/tm:analyze-complexity:analyze-complexity`: Analyze task complexity and generate expansion recommendations.
- `/tm:complexity-report:complexity-report`: Display the task complexity analysis report.
- `/tm:expand:expand-task`: Break down a complex task into subtasks.
- `/tm:expand:expand-all-tasks`: Expand all pending tasks that need subtasks.
- `/tm:add-dependency:add-dependency`: Add a dependency between tasks.
- `/tm:remove-dependency:remove-dependency`: Remove a dependency between tasks.
- `/tm:validate-dependencies:validate-dependencies`: Validate all task dependencies for issues.
- `/tm:fix-dependencies:fix-dependencies`: Automatically fix dependency issues.

### Configuration & Automation
- `/tm:setup:install-taskmaster`: Check and install Task Master if needed.
- `/tm:setup:quick-install-taskmaster`: Quick install Task Master globally.
- `/tm:models:view-models`: View current AI model configuration.
- `/tm:models:setup-models`: Run interactive setup to configure AI models.
- `/tm:generate:generate-tasks`: Generate individual task files from tasks.json.
- `/tm:sync-readme:sync-readme`: Export tasks to README.md.
- `/tm:workflows:smart-workflow`: Execute an intelligent workflow based on project state.
- `/tm:workflows:command-pipeline`: Execute a pipeline of commands.
- `/tm:workflows:auto-implement-tasks`: Auto-implement tasks with code generation and testing.
- `/tm:utils:analyze-project`: Perform advanced project analysis.

### Help & Learning
- `/tm:help`: Show help for Task Master commands.
- `/tm:learn`: Learn about Task Master capabilities interactively.
- `/tm:tm-main`: Task Master Command Reference.

---

## Project Management (`/pm:`)

A collection of scripts for project management.

- `/pm:init`: Initialize the project.
- `/pm:status`: Show project status.
- `/pm:next`: Get the next task.
- `/pm:search`: Search for a task.
- `/pm:sync`: Sync project data.
- `/pm:import`: Import data.
- `/pm:validate`: Validate project data.
- `/pm:clean`: Clean project artifacts.
- `/pm:help`: Show help for PM scripts.
- `/pm:standup`: Generate a standup report.
- `/pm:in-progress`: List tasks in progress.
- `/pm:blocked`: List blocked tasks.

### Epics
- `/pm:epic-list`, `/pm:epic-show`, `/pm:epic-status`, `/pm:epic-start`, `/pm:epic-start-worktree`, `/pm:epic-decompose`, `/pm:epic-refresh`, `/pm:epic-sync`, `/pm:epic-edit`, `/pm:epic-merge`, `/pm:epic-close`, `/pm:epic-oneshot`

### Issues
- `/pm:issue-analyze`, `/pm:issue-show`, `/pm:issue-status`, `/pm:issue-start`, `/pm:issue-sync`, `/pm:issue-edit`, `/pm:issue-reopen`, `/pm:issue-close`

### PRDs
- `/pm:prd-list`, `/pm:prd-status`, `/pm:prd-new`, `/pm:prd-edit`, `/pm:prd-parse`

---

## Code & Project Lifecycle

- `/scaffold`, `/implement`, `/refactor`, `/make-it-pretty`, `/format`, `/fix-imports`, `/remove-comments`, `/init`, `/setup-development-environment`, `/setup-monorepo`, `/setup-formatting`, `/setup-linting`, `/understand`, `/explain-like-senior`, `/directory-deep-dive`, `/initref`, `/project-health-check`, `/predict-issues`, `/project_reflection`

---

## Testing & Quality Assurance

- `/generate-tests`, `/generate-test-cases`, `/write-tests`, `/testing_plan_integration`, `/test`, `/testing:run`, `/test-automation-orchestrator`, `/e2e-setup`, `/setup-visual-testing`, `/setup-comprehensive-testing`, `/test-coverage`, `/test-quality-analyzer`, `/test-changelog-automation`, `/pm:test-reference-update`, `/testing:prime`

---

## Documentation & TODOS

- `/docs`, `/update-docs`, `/create-architecture-documentation`, `/create-onboarding-guide`, `/interactive-documentation`, `/docs-maintenance`, `/troubleshooting-guide`, `/todos`, `/create-todos`, `/find-todos`, `/fix-todos`, `/todos-to-issues`

---

## Git & Version Control

- `/commit`, `/clean-branches`, `/pr-comments`, `/security-review`

---

## System Integration (`/bridge:`)

- `/bridge:sync-all`, `/bridge:tm-to-epic`, `/bridge:parallel-start`

---

## Session & Context

- `/session-start`, `/session-end`, `/resume`, `/context:create`, `/context:prime`, `/context:update`, `/clear`, `/compact`, `/context`

---

## Claude Code Application

- `/help`, `/status`, `/statusline`, `/config`, `/model`, `/output-style`, `/output-style:new`, `/permissions`, `/login`, `/logout`, `/upgrade`, `/release-notes`, `/feedback`, `/doctor`, `/migrate-installer`, `/install-github-app`, `/ide`, `/mcp`, `/hooks`, `/bashes`, `/memory`, `/export`, `/exit`, `/vim`

---

## Advanced & Specialized Tools

- `/code-review`, `/security-scan`, `/debug-error`, `/optimize-bundle-size`, `/implement-caching-strategy`, `/design-database-schema`, `/add-performance-monitoring`, `/architecture-scenario-explorer`, `/decision-tree-explorer`, `/system-behavior-simulator`, `/system-dynamics-modeler`, `/simulation-calibrator`, `/workflow-orchestrator`, `/rule2hook`, `/ultra-think`, `/add-dir`, `/all-tools`, `/cipher-memory`, `/cipher-memory:system_prompt`, `/code-permutation-tester`, `/contributing`, `/cost`, `/create_command`, `/cleanproject`, `/privacy-settings`