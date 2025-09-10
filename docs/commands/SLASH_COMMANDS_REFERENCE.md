# Full Slash Command Reference

This document provides a comprehensive, categorized list of all available slash commands.

## Table of Contents

- [Task Master (`/tm:`)](#task-master-tm)
- [Project Management (`/pm:`)](#project-management-pm)
- [Code & Project Lifecycle](#code--project-lifecycle)
- [Testing & Quality Assurance](#testing--quality-assurance)
- [Documentation & TODOS](#documentation--todos)
- [Git & Version Control](#git--version-control)
- [System Integration (`/bridge:`)](#system-integration-bridge)
- [Session & Context](#session--context)
- [Claude Code Application](#claude-code-application)
- [Advanced & Specialized Tools](#advanced--specialized-tools)

---

## Task Master (`/tm:`)

The `/tm:` command group provides a powerful, integrated project management suite directly within the CLI. It uses AI to enhance task creation, analysis, and workflow automation.

### Project Setup & Configuration

- **/tm:init:init-project [prd-file]**
  - **Action:** Interactively initializes a new Task Master project in the current directory.
  - **How it Works:** Runs `task-master init`. It suggests a project name, checks for a git repository, and verifies AI provider configurations. If a PRD file is provided, it automatically calls `/tm:parse-prd` after initialization.

- **/tm:init:init-project-quick**
  - **Action:** Initializes a new project non-interactively, accepting all default settings.
  - **How it Works:** Runs `task-master init -y`. Ideal for quick setup in automated scripts.

- **/tm:models:view-models**
  - **Action:** Displays the current AI model configuration for main, research, and fallback roles.
  - **How it Works:** Runs `task-master models` and shows which models are configured and whether their API keys are present.

- **/tm:models:setup-models**
  - **Action:** Starts an interactive session to configure AI providers and API keys.
  - **How it Works:** Runs `task-master models --setup`. It guides the user through selecting models and providing necessary API keys.

- **/tm:setup:install-taskmaster**
  - **Action:** Checks if `task-master-ai` is installed globally via npm and installs it if not.
  - **How it Works:** Checks for the `task-master` command and Node.js version, then runs `npm install -g task-master-ai` if needed. Also provides troubleshooting for permission errors.

- **/tm:setup:quick-install-taskmaster**
  - **Action:** A streamlined, one-line command to install Task Master globally if it's not already present.
  - **How it Works:** Runs `task-master --version 2>/dev/null || npm install -g task-master-ai`.

### Task & Subtask Management

- **/tm:add-task [description]**
  - **Action:** Adds a new task using natural language processing to determine properties.
  - **How it Works:** Parses the description for keywords related to priority (e.g., "urgent"), dependencies (e.g., "after task 23"), and complexity. It presents a preview for confirmation before creating the task.

- **/tm:list [filters]**
  - **Action:** Lists tasks with intelligent, natural language filtering.
  - **How it Works:** Parses arguments for status, priority, ID ranges, or special keywords like "blocked" or "tree". It then constructs and runs the appropriate `task-master list` command.

- **/tm:list:list-tasks-by-status [status]**
  - **Action:** Lists all tasks that match a specific status.
  - **How it Works:** Runs `task-master list --status=[status]` and provides enhanced display with context-specific insights (e.g., for `pending` tasks, it recommends a start order).

- **/tm:list:list-tasks-with-subtasks**
  - **Action:** Displays all tasks in a hierarchical tree view, showing nested subtasks.
  - **How it Works:** Runs `task-master list --with-subtasks` and formats the output to show parent/child relationships and completion percentages for parent tasks.

- **/tm:show [task-id|keyword]**
  - **Action:** Shows highly detailed, contextual information for a specific task or group of tasks.
  - **How it Works:** Accepts a task ID or keywords like "current", "next", or "critical". It displays core details, relationships, time intelligence, risk assessment, and actionable suggestions.

- **/tm:update:update-task [updates]**
  - **Action:** Updates one or more tasks using natural language for bulk operations.
  - **How it Works:** Intelligently detects fields to update based on keywords (e.g., "complete tasks 12, 15"). It provides a preview of changes and their impact before executing.

- **/tm:update:update-single-task [id] [update]**
  - **Action:** Applies a specific, AI-assisted update to a single task.
  - **How it Works:** Runs `task-master update-task --id=[id] --prompt="[update]"`. The AI reads the task's current state and intelligently integrates the new information, updating multiple fields (description, test strategy, complexity) for consistency.

- **/tm:update:update-tasks-from-id [start-id] [update]**
  - **Action:** Applies a contextual update to a task and all its subsequent related tasks.
  - **How it Works:** Runs `task-master update --from=[start-id] --prompt="[update]"`. It identifies a logical group of tasks to update, previews the changes, and applies them consistently across the board.

- **/tm:remove-task [task-id]**
  - **Action:** Permanently removes a task after a thorough impact analysis.
  - **How it Works:** Before deleting, it shows which dependent tasks will be affected and how much work will be lost. It requires confirmation unless a `-y` flag is used.

- **/tm:add-subtask [parent-id] [title]**
  - **Action:** Adds one or more subtasks to a parent task.
  - **How it Works:** Can create a new subtask or convert an existing standalone task into a subtask. If the title contains commas or "and", it automatically creates multiple subtasks.

- **/tm:add-subtask:convert-task-to-subtask [parent-id] [task-id]**
  - **Action:** Converts an existing standalone task into a subtask of another.
  - **How it Works:** Runs `task-master add-subtask --parent=[parent-id] --task-id=[task-id]`. It changes the task's ID, updates all dependency references, and adjusts its priority to match the new parent.

- **/tm:remove-subtask [subtask-id]**
  - **Action:** Removes a subtask, with an option to convert it back to a standalone task.
  - **How it Works:** Can either delete the subtask or run `task-master remove-subtask --id=[id] --convert` to promote it to a standalone task, preserving its history and data.

- **/tm:clear-subtasks [task-id]**
  - **Action:** Removes all subtasks from a specific parent task after confirmation.
  - **How it Works:** Runs `task-master clear-subtasks --id=[id]`. It warns if any of the subtasks are in-progress before deleting.

- **/tm:clear-subtasks:clear-all-subtasks**
  - **Action:** A highly destructive command to remove all subtasks from all tasks across the entire project.
  - **How it Works:** Runs `task-master clear-subtasks --all`. It performs a project-wide analysis, shows a detailed impact report, and requires an explicit confirmation phrase (`CLEAR ALL SUBTASKS`) to proceed.

### Status & Workflow

- **/tm:next:next-task [quick|easy|important]**
  - **Action:** Intelligently determines and prepares the best task to work on next.
  - **How it Works:** Analyzes the project state, recent activity, and dependencies. If a task is already in progress, it suggests resuming it. Otherwise, it recommends the highest-priority unblocked task and prepares the environment for it.

- **/tm:set-status:to-pending [task-id]**
- **/tm:set-status:to-in-progress [task-id]**
- **/tm:set-status:to-review [task-id]**
- **/tm:set-status:to-done [task-id]**
- **/tm:set-status:to-deferred [task-id]**
- **/tm:set-status:to-cancelled [task-id]**
  - **Action:** Sets the status of a task. These are more than simple state changes.
  - **How it Works:** Each command includes pre-checks and post-actions. For example, `to-in-progress` can set up a git branch and display relevant files, while `to-done` will identify and display newly unblocked tasks.

- **/tm:status:project-status [sprint|blocked|team]**
  - **Action:** Provides a comprehensive, dashboard-like overview of the project's health and progress.
  - **How it Works:** Generates an executive summary with progress metrics, blockers, and time analysis. It can provide predictive insights like completion projections and risk assessments.

### Analysis, Dependencies & Expansion

- **/tm:analyze-complexity [--research]**
  - **Action:** Uses AI to analyze all pending tasks and recommend which ones are too complex and should be broken down.
  - **How it Works:** Runs `task-master analyze-complexity`. It assigns a complexity score (1-10) to each task and generates a report with expansion recommendations and risk assessments.

- **/tm:complexity-report**
  - **Action:** Displays the detailed report generated by the `analyze-complexity` command.
  - **How it Works:** Reads and formats the report from `.taskmaster/reports/complexity-analysis.md`, providing an executive summary, risk matrix, and actionable recommendations.

- **/tm:expand:expand-task [task-id]**
  - **Action:** Intelligently breaks down a single complex task into 3-7 smaller, manageable subtasks.
  - **How it Works:** Runs `task-master expand --id=[id]`. The AI generates subtasks based on the parent task's type (e.g., a bug fix gets "Reproduce, Diagnose, Fix, Verify" subtasks).

- **/tm:expand:expand-all-tasks [--force]**
  - **Action:** Intelligently expands all pending tasks that have a high complexity score.
  - **How it Works:** Runs `task-master expand --all`. It identifies all tasks that would benefit from being broken down and expands them in a single batch operation.

- **/tm:add-dependency [task-id] [depends-on-id]**
  - **Action:** Creates a dependency where one task must be completed before another can start.
  - **How it Works:** Runs `task-master add-dependency`. It parses natural language (e.g., "make 5 depend on 3"), checks for circular dependencies, and shows the impact on the project timeline.

- **/tm:remove-dependency [task-id] [depends-on-id]**
  - **Action:** Removes a dependency relationship between two tasks.
  - **How it Works:** Runs `task-master remove-dependency`. It shows which tasks will become unblocked and warns if the removal could break a critical path or logical sequence.

- **/tm:validate-dependencies**
  - **Action:** Performs a comprehensive check for dependency issues across the entire project.
  - **How it Works:** Runs `task-master validate-dependencies`. It checks for circular dependencies, references to deleted tasks, and other logical issues, then presents a report with suggested fixes.

- **/tm:fix-dependencies**
  - **Action:** Automatically fixes common dependency problems found by the validation command.
  - **How it Works:** Runs `task-master fix-dependencies`. It can automatically remove references to deleted tasks, break simple circular dependencies, and clean up duplicates. It will prompt for manual review on more complex issues.

### Automation & Workflows

- **/tm:workflows:auto-implement-tasks [task-id]**
  - **Action:** An advanced workflow that uses AI to perform a task, including code generation, testing, and documentation updates.
  - **How it Works:** Follows a strategy based on the task type (Feature, Bug Fix, Refactor). It uses a test-driven approach, follows project conventions, and includes quality assurance checks like linting and formatting.

- **/tm:workflows:command-pipeline [spec]**
  - **Action:** Executes a sequence of `/tm:` commands based on a specification.
  - **How it Works:** Parses a pipeline string (e.g., `init → expand-all → sprint-plan`) and executes the commands in order. Supports conditional logic, variables, and iteration.

- **/tm:workflows:smart-workflow**
  - **Action:** Analyzes the project state and your recent command history to execute the most logical next workflow.
  - **How it Works:** If you just completed a task, it might suggest finding the next one. If it's the morning, it might suggest a daily standup workflow. It learns your patterns over time.

### Utilities & Help

- **/tm:parse-prd [file-path] [--research]**
  - **Action:** Analyzes a Product Requirements Document (PRD) and generates a complete task breakdown.
  - **How it Works:** Runs `task-master parse-prd`. The AI extracts requirements, identifies components, and creates a list of tasks with estimated complexities and dependencies. The `--research` flag uses a secondary AI for more in-depth technical suggestions.

- **/tm:generate:generate-tasks**
  - **Action:** Creates individual, AI-readable markdown files for each task in `.taskmaster/tasks/`.
  - **How it Works:** Runs `task-master generate`. These files are useful for providing context to other AI agents, for documentation, or for offline archival.

- **/tm:sync-readme [filters]**
  - **Action:** Creates or updates the project's `README.md` with a beautifully formatted summary of the current task status.
  - **How it Works:** Runs `task-master sync-readme`. It can include progress bars, status badges, and task lists grouped by priority or status.

- **/tm:help [command]**
  - **Action:** Displays help information for Task Master commands.

- **/tm:learn [topic]**
  - **Action:** Provides an interactive way to learn about Task Master's capabilities.

- **/tm:tm-main**
  - **Action:** Displays a command reference for the Task Master suite.

- **/tm:utils:analyze-project [focus]**
  - **Action:** Performs an advanced, multi-dimensional analysis of the project.
  - **How it Works:** Can focus on specific areas like "velocity", "risk", or "dependencies". It generates reports with metrics, visualizations, and actionable recommendations.

---

## Project Management (`/pm:`)

This is a comprehensive project management system designed for orchestrating development work, often with AI agents. It is based on a local file structure of "Epics" and "Issues" stored in `.claude/epics/` and `.claude/prds/`, which are then synchronized with GitHub Issues. Many of these commands are wrappers around shell scripts.

### PRD & Epic Lifecycle

- **/pm:prd-new [feature-name]**
  - **Action:** Starts a brainstorming session to create a new Product Requirements Document (PRD).
  - **How it Works:** Guides the user through creating a structured PRD with sections like Executive Summary, User Stories, and Success Criteria. Saves the output to `.claude/prds/[feature-name].md`.

- **/pm:prd-list**
  - **Action:** Lists all existing PRDs.
  - **How it Works:** Executes the `.claude/scripts/pm/prd-list.sh` script.

- **/pm:prd-edit [feature-name]**
  - **Action:** Interactively edit an existing PRD file.
  - **How it Works:** Reads the specified PRD and prompts the user for which sections to edit, then saves the changes.

- **/pm:prd-parse [feature-name]**
  - **Action:** Converts a PRD into a technical implementation Epic.
  - **How it Works:** Reads the specified PRD and creates a corresponding epic file at `.claude/epics/[feature-name]/epic.md` with a detailed technical approach, architecture decisions, and a high-level task breakdown.

- **/pm:epic-decompose [epic-name]**
  - **Action:** Breaks an epic down into concrete, actionable task files.
  - **How it Works:** Reads the `epic.md` and creates sequentially numbered task files (e.g., `001.md`, `002.md`) within the epic's directory. It can create tasks in parallel using sub-agents for efficiency.

- **/pm:epic-sync [epic-name]**
  - **Action:** Pushes a local epic and its tasks to GitHub, creating a main issue for the epic and sub-issues for each task.
  - **How it Works:** Uses the `gh` CLI to create issues. It then renames the local task files from their sequential number (`001.md`) to the corresponding GitHub issue number (`123.md`) and updates all internal dependency references.

- **/pm:epic-oneshot [feature-name]**
  - **Action:** A convenience command that runs `/pm:epic-decompose` and `/pm:epic-sync` in sequence.
  - **How it Works:** Takes a feature from decomposition to GitHub issues in a single step.

### Epic & Issue Management

- **/pm:epic-list**
  - **Action:** Lists all current epics.
  - **How it Works:** Executes the `.claude/scripts/pm/epic-list.sh` script.

- **/pm:epic-show [epic-name]**
  - **Action:** Displays the details of a specific epic.
  - **How it Works:** Executes the `.claude/scripts/pm/epic-show.sh` script.

- **/pm:epic-status [epic-name]**
  - **Action:** Shows the current status and progress of an epic.
  - **How it Works:** Executes the `.claude/scripts/pm/epic-status.sh` script.

- **/pm:epic-edit [epic-name]**
  - **Action:** Interactively edit the details of an `epic.md` file.

- **/pm:epic-refresh [epic-name]**
  - **Action:** Updates an epic's progress percentage based on the status of its task files.
  - **How it Works:** Scans all task files within the epic, calculates the percentage of closed tasks, and updates the `progress` field in the epic's frontmatter.

- **/pm:epic-start [epic-name]**
  - **Action:** Begins parallel agent-based work on an epic's tasks within a dedicated git branch.
  - **How it Works:** Creates or checks out a branch named `epic/[epic-name]`. It identifies all "ready" issues (those with no unmet dependencies) and launches parallel AI agents to work on them. It tracks progress in an `execution-status.md` file.

- **/pm:epic-close [epic-name]**
  - **Action:** Marks an epic as complete after all its tasks are done.
  - **How it Works:** Verifies all tasks are closed, updates the epic's status to `completed`, and closes the corresponding GitHub issue.

- **/pm:epic-merge [epic-name]**
  - **Action:** Merges the completed epic branch back into the main branch.
  - **How it Works:** Performs pre-merge validation, runs tests, merges the `epic/[epic-name]` branch, and then cleans up by removing the worktree/branch and archiving the local epic files.

- **/pm:issue-analyze [issue-number]**
  - **Action:** Analyzes a specific issue to identify parallel work streams for efficient execution by agents.
  - **How it Works:** Breaks down the work required for an issue into independent "streams" (e.g., Database Layer, API Layer, UI Layer) and saves this plan to an `[issue-number]-analysis.md` file.

- **/pm:issue-start [issue-number]**
  - **Action:** Launches parallel AI agents to begin work on an issue based on its analysis file.
  - **How it Works:** Reads the analysis file and starts agents for each defined work stream, assigning them specific files to modify within the epic's worktree.

- **/pm:issue-show [issue-number]**
  - **Action:** Displays detailed information about a specific issue, including its GitHub status and local context.

- **/pm:issue-status [issue-number]**
  - **Action:** Provides a quick status check for an issue, including its state (open/closed) and local sync status.

- **/pm:issue-edit [issue-number]**
  - **Action:** Interactively edit an issue's details both locally and on GitHub.

- **/pm:issue-close [issue-number]**
  - **Action:** Marks a local task as complete and closes the corresponding issue on GitHub.

- **/pm:issue-reopen [issue-number]**
  - **Action:** Reopens a closed issue both locally and on GitHub.

### General & Utility Commands

- **/pm:init**
  - **Action:** Initializes the project management system.
  - **How it Works:** Executes the `.claude/scripts/pm/init.sh` script.

- **/pm:status**
  - **Action:** Displays the overall project status.
  - **How it Works:** Executes the `.claude/scripts/pm/status.sh` script.

- **/pm:next**
  - **Action:** Shows the next recommended task or epic to work on.
  - **How it Works:** Executes the `.claude/scripts/pm/next.sh` script.

- **/pm:search [query]**
  - **Action:** Searches for epics or issues.
  - **How it Works:** Executes the `.claude/scripts/pm/search.sh` script with the given query.

- **/pm:sync [epic-name]**
  - **Action:** Performs a full bidirectional sync between local files and GitHub issues.
  - **How it Works:** Pulls changes from GitHub, then pushes local changes. It can handle merge conflicts by prompting the user.

- **/pm:import**
  - **Action:** Imports existing GitHub issues that are not yet tracked in the local system.
  - **How it Works:** Fetches all issues from the repository, identifies untracked ones, and creates the corresponding local epic/task file structure for them.

- **/pm:standup**
  - **Action:** Generates a standup report of recent activity.
  - **How it Works:** Executes the `.claude/scripts/pm/standup.sh` script.

- **/pm:in-progress**
  - **Action:** Lists all tasks currently in progress.
  - **How it Works:** Executes the `.claude/scripts/pm/in-progress.sh` script.

- **/pm:blocked**
  - **Action:** Lists all tasks that are currently blocked by dependencies.
  - **How it Works:** Executes the `.claude/scripts/pm/blocked.sh` script.

- **/pm:validate**
  - **Action:** Validates the integrity of the local project management files.
  - **How it Works:** Executes the `.claude/scripts/pm/validate.sh` script.

- **/pm:clean**
  - **Action:** Archives old, completed epics and cleans up stale files.
  - **How it Works:** Identifies epics that have been completed for more than 30 days and moves them to an `.archived` directory.

- **/pm:help**
  - **Action:** Displays help information for the `/pm` commands.
  - **How it Works:** Executes the `.claude/scripts/pm/help.sh` script.

---

## Code & Project Lifecycle

Commands for creating, modifying, and understanding code, as well as setting up the project environment.

### Scaffolding & Implementation

*These are core functionalities of the AI assistant.*

- **/implement**: Smart Implementation Engine to write code based on a prompt.
- **/refactor**: Intelligent Refactoring Engine for improving code structure.
- **/scaffold**: Intelligent Scaffolding for new components or features.

### Setup & Configuration

- **/init**: Initializes a new `CLAUDE.md` file with codebase documentation. *(This is a core command, not a project-specific file).*

- **/setup-development-environment**
  - **Action:** A comprehensive command to set up a complete, standardized development environment.
  - **How it Works:** This command outlines a 10-step process that includes installing runtimes (Node.js, Python), package managers, git, and project-specific tooling. It also covers setting up code quality tools (linters, formatters), a local development server, IDE configurations, and environment variables.

- **/setup-monorepo [nx|lerna|turborepo|...]**
  - **Action:** Configures the project as a monorepo using a specified tool.
  - **How it Works:** If a tool is specified, it configures the project accordingly (e.g., `nx.json` for Nx, `lerna.json` for Lerna). If no tool is specified, it analyzes the project and recommends the best fit. The process includes setting up the directory structure (`packages/`, `libs/`), build system integration, and CI/CD pipelines.

- **/setup-formatting**
  - **Action:** Sets up code formatting tools like Prettier.
  - **How it Works:** Installs the necessary packages (e.g., `prettier` for JS/TS, `black` for Python), creates a default configuration file (e.g., `.prettierrc`), and provides guidance on setting up IDE integration and pre-commit hooks.

- **/setup-linting**
  - **Action:** Sets up code linting tools like ESLint.
  - **How it Works:** Analyzes the project to select the right tools (e.g., ESLint for JS/TS, Flake8 for Python), installs dependencies, creates a base configuration file, and provides instructions for IDE and CI/CD integration.

### Analysis & Understanding

- **/directory-deep-dive [directory-path]**
  - **Action:** Analyzes a specific directory to understand its architecture, purpose, and patterns.
  - **How it Works:** The AI investigates the code within the target directory, looking for design patterns, dependencies, and naming conventions. It then creates or updates a `CLAUDE.md` file inside that directory to document its findings, ensuring context is loaded when working in that area.

- **/initref**
  - **Action:** Builds a reference for the implementation details of the project.
  - **How it Works:** Uses the AI's `summarize` tool to get summaries of project files, which it then uses to create detailed markdown documentation in a `/ref` directory. It also updates the main `CLAUDE.md` to point to this new documentation.

- **/project-health-check [--30-days|--sprint]**
  - **Action:** Generates a comprehensive report on the project's overall health.
  - **How it Works:** Analyzes git history, code quality metrics (test coverage, complexity), delivery performance (cycle time, bug ratio), and dependency health (outdated packages, security vulnerabilities). It outputs a markdown report with an overall health score and actionable recommendations.

- **/project_reflection**
  - **Action:** An advanced command where the AI analyzes the project to generate optimized instructions for *itself* and other AI assistants.
  - **How it Works:** It performs a deep analysis of the project's architecture, code patterns, and development practices. It then generates a set of project-specific guidelines (e.g., "preferred implementation patterns," "common pitfalls to avoid") and recommends updates to the `CLAUDE.md` file to improve future AI performance on the project.

---

## Testing & Quality Assurance

Commands for generating, running, and analyzing tests, as well as setting up comprehensive testing infrastructure.

### Test Generation & Planning

- **/generate-tests [file-path]**
  - **Action:** A comprehensive command to generate a full test suite for a given file or component.
  - **How it Works:** The AI analyzes the target code, identifies testable functions and behaviors, and creates new test files following existing project conventions. It aims to generate unit tests, integration tests, and edge case handling, including necessary mocks.

- **/write-tests [target-file] [--unit|--integration]**
  - **Action:** A more focused version of `generate-tests` for writing specific types of tests.
  - **How it Works:** The AI analyzes the target file and writes tests according to the specified type (unit, integration, etc.), implementing framework-specific best practices for mocking and assertions.

- **/generate-test-cases [target-file] [--edge-cases]**
  - **Action:** Intelligently generates a wide range of test cases based on code analysis.
  - **How it Works:** The AI parses function signatures, analyzes control flow and branching paths, and identifies parameter domains to generate positive tests, negative tests, boundary value tests, and error condition tests.

- **/testing_plan_integration [target-code]**
  - **Action:** Creates a comprehensive integration testing plan for a piece of code.
  - **How it Works:** The AI analyzes the code's testability, suggests refactoring for better dependency injection if needed, designs integration scenarios, and plans a mocking strategy for external dependencies.

### Test Execution & Orchestration

- **/testing:prime**
  - **Action:** Prepares the testing environment by detecting the framework and validating dependencies.
  - **How it Works:** The AI scans for test framework configurations (Jest, Pytest, etc.), verifies that the necessary dependencies are installed, and creates a `.claude/testing-config.md` file to store the discovered settings for future test runs.

- **/testing:run [test-target]**
  - **Action:** Executes tests using the configured test-runner agent.
  - **How it Works:** Can run all tests, a specific test file, or tests matching a pattern. It uses the configuration from `/testing:prime` to execute the correct command and reports the results, providing detailed analysis on failures.

- **/test-automation-orchestrator [--parallel|--sequential]**
  - **Action:** A high-level command to design and implement an intelligent test orchestration strategy.
  - **How it Works:** The AI analyzes all test suites and designs an optimal execution strategy (e.g., parallel execution, intelligent batching) to improve performance and resource allocation within the CI/CD pipeline.

### Test Infrastructure Setup

- **/setup-comprehensive-testing [--unit|--integration|--e2e]**
  - **Action:** A strategic command to set up a complete, multi-layered testing infrastructure.
  - **How it Works:** The AI analyzes the project and guides the setup of a full testing pyramid, including unit, integration, and E2E testing frameworks, CI/CD integration, and quality gates.

- **/e2e-setup [--cypress|--playwright]**
  - **Action:** Sets up a comprehensive end-to-end testing suite.
  - **How it Works:** The AI helps select and configure an E2E framework like Cypress or Playwright, sets up the test environment, designs a Page Object Model (POM) pattern, and integrates the suite into the CI/CD pipeline.

- **/setup-visual-testing**
  - **Action:** Sets up a visual regression testing workflow.
  - **How it Works:** The AI guides the selection and configuration of tools like Percy or Chromatic. The process includes creating baseline screenshots, integrating with the CI/CD pipeline to detect visual changes, and establishing a review/approval workflow.

### Test Analysis & Maintenance

- **/test-coverage [--line|--branch]**
  - **Action:** Analyzes test coverage and provides a report with recommendations for improvement.
  - **How it Works:** The AI configures and runs coverage tools (like Jest --coverage, NYC, Coverage.py), analyzes the output to identify critical uncovered code, and provides a plan to address coverage gaps.

- **/test-quality-analyzer**
  - **Action:** Performs a deep analysis of the entire test suite's quality.
  - **How it Works:** Goes beyond simple coverage to evaluate test effectiveness, maintainability, and performance. It detects anti-patterns (e.g., flaky tests) and provides recommendations for improving the overall quality and reliability of the test suite.

- **/test-changelog-automation**
  - **Action:** Automates the testing and validation of the project's changelog.
  - **How it Works:** The AI sets up a workflow, typically in CI/CD, to validate that changelog entries are correctly formatted, consistent, and adhere to project standards.

---

## Documentation & TODOS

Commands for creating, maintaining, and managing project documentation and tracking TODO items.

### Documentation Generation & Maintenance

- **/create-architecture-documentation [--c4-model|--adr]**
  - **Action:** A comprehensive command to generate architecture documentation.
  - **How it Works:** The AI analyzes the project's structure, dependencies, and code to produce documentation. It can be guided to use specific frameworks like the C4 model or to generate Architecture Decision Records (ADRs). The process includes creating system context diagrams, documenting data flow, and defining security architecture.

- **/create-onboarding-guide [--developer|--designer]**
  - **Action:** Creates a detailed onboarding guide for new team members.
  - **How it Works:** The AI analyzes the project to create a guide that includes development environment setup, project overview, codebase structure, and key workflows. It can be tailored for different roles like developers or designers.

- **/docs-maintenance [--audit|--update|--validate]**
  - **Action:** A proactive command to implement a comprehensive documentation maintenance system.
  - **How it Works:** The AI sets up a framework for maintaining documentation quality. This includes content audits for freshness, validation of internal and external links, style and consistency checking, and automated synchronization with code changes.

- **/interactive-documentation [--docusaurus|--gitbook]**
  - **Action:** Creates an interactive documentation platform with live examples.
  - **How it Works:** The AI guides the setup of a documentation platform like Docusaurus or GitBook. The process includes integrating live code playgrounds, interactive API testing, and component galleries to create an engaging user experience.

- **/troubleshooting-guide [--application|--database]**
  - **Action:** Generates a systematic troubleshooting guide for the project.
  - **How it Works:** The AI analyzes the system to identify common issues and failure points. It then creates a guide with diagnostic procedures, common error codes, and solutions for different components of the system like the application, database, or network.

- **/update-docs**
  - **Action:** Systematically updates existing project documentation to reflect the latest implementation status.
  - **How it Works:** The AI reviews implementation plans, testing results, and recent code changes. It then updates the relevant documentation files (e.g., in a `specs/` directory) by marking tasks as complete (✅), adding notes about the implementation, and ensuring the documentation accurately reflects the current state of the project.

- **/docs**: A core command to manage documentation. The specifics of this command are built-in.

### TODO Management

*These are core functionalities of the AI assistant.*

- **/todos**: Lists current TODO items found within the codebase.
- **/create-todos**: Creates smart TODOs from code or descriptions.
- **/find-todos**: Finds all development tasks or TODOs within the project.
- **/todos-to-issues**: A command to convert TODO items into GitHub Issues.

---

## Git & Version Control

Commands for interacting with Git.

- `/commit`: Smart Git Commit assistant.
- `/clean-branches`: Clean up old or merged Git branches.
- `/pr-comments`: Get comments from a GitHub pull request.
- `/security-review`: Complete a security review of pending changes on the current branch.

---

## System Integration (`/bridge:`)

Commands for syncing data between systems.

- `/bridge:sync-all`: Bidirectional sync between Task Master and GitHub.
- `/bridge:tm-to-epic`: Sync tasks from Task Master to a CCPM Epic.
- `/bridge:parallel-start`: Parallel Start from Task Master.

---

## Session & Context

Commands for managing the conversation and context.

### Session Lifecycle

- `/session-start`: Start a new coding session.
- `/session-end`: End a coding session.
- `/resume`: Resume a previous conversation.

### Context Management

Commands for creating, loading, and maintaining a set of documents in `.claude/context/` that provide the AI with a deep understanding of the project.

#### /context:create

Analyzes the entire project to generate a comprehensive set of initial context documents. This is a powerful but potentially slow command that should be run once at the beginning of a project.

- **How it Works:**
  1. **Preflight Checks:** Verifies permissions and checks if context files already exist, prompting the user before overwriting.
  2. **Project Analysis:** Detects project type (Node.js, Python, etc.), git status, and directory structure.
  3. **File Generation:** Creates a series of markdown files in `.claude/context/`, including:
      - `progress.md`: Current git status, branch, and recent commits.
      - `project-structure.md`: Directory layout and file organization.
      - `tech-context.md`: Dependencies and technologies used.
      - `system-patterns.md`: Observed architectural patterns.
      - `product-context.md`: Core features and user personas.
      - ...and several others.
  4. **Validation:** Ensures each file is created successfully and contains content.
- **Dependencies:** `git`, `date`.
- **Usage:** Run this once on a new project to give the AI a foundational understanding.

#### /context:prime

Loads the essential context from the `.claude/context/` directory into the current session. This is much faster than `/context:create`.

- **How it Works:**
  1. **Preflight Checks:** Verifies that the context directory and its files exist and are readable.
  2. **Priority Loading:** Reads the context files in a specific order, starting with the most critical (`project-overview.md`, `tech-context.md`) and proceeding to deeper context.
  3. **Validation:** Checks for valid frontmatter in each file.
  4. **Summary:** Provides a detailed summary of the loaded context, including project type, status, and any warnings about missing or corrupted files.
- **Dependencies:** Existing context files generated by `/context:create` or `/context:update`.
- **Usage:** Run this at the beginning of every new coding session to quickly get the AI up to speed.

#### /context:update

Intelligently updates the existing context files to reflect the latest project changes. This is more efficient than regenerating everything from scratch.

- **How it Works:**
  1. **Change Detection:** Uses `git diff` and `git log` to identify recent commits, file modifications, and dependency changes.
  2. **Smart Updates:** Selectively modifies context files based on the changes detected.
      - `progress.md` is always updated with the latest git status.
      - `tech-context.md` is updated only if `package.json` (or equivalent) has changed.
      - `project-structure.md` is updated only if new directories or files are added.
      - Other files are updated less frequently.
  3. **Timestamping:** Updates the `last_updated` field in the frontmatter of any modified file.
  4. **Summary:** Reports which files were updated, which were skipped (because they were up-to-date), and any errors.
- **Dependencies:** `git`, `date`, existing context files.
- **Usage:** Run this at the end of a coding session or after a significant commit to keep the AI's knowledge base current.
- `/clear` (or `/reset`, `/new`): Clear conversation history.
- `/compact`: Clear history but keep a summary in context.
- `/context`: Visualize current context usage.

---

## Claude Code Application

Commands for managing the Claude Code application itself.

- `/help`: Show help and available commands.
- `/status`: Show Claude Code status (version, model, connectivity).
- `/statusline`: Set up the status line UI.
- `/config` (or `/theme`): Open the configuration panel.
- `/model`: Set the AI model.
- `/output-style`: Set the output style.
- `/output-style:new`: Create a custom output style.
- `/permissions` (or `/allowed-tools`): Manage tool permissions.
- `/login`: Sign in to your Anthropic account.
- `/logout`: Sign out.
- `/upgrade`: Upgrade to a higher plan.
- `/release-notes`: View release notes.
- `/feedback` (or `/bug`): Submit feedback or a bug report.
- `/doctor`: Diagnose and verify the Claude Code installation.
- `/migrate-installer`: Migrate from global to local npm installation.
- `/install-github-app`: Set up Claude GitHub Actions.
- `/ide`: Manage IDE integrations.
- `/mcp`: Manage MCP servers.
- `/hooks`: Manage hook configurations.
- `/bashes`: List and manage background tasks.
- `/memory`: Edit Claude memory files.
- `/export`: Export the current conversation.
- `/exit` (or `/quit`): Exit the application.
- `/vim`: Toggle Vim editing mode.

---

## Advanced & Specialized Tools

High-level commands for complex analysis, simulation, and automation.

### Code & Architecture

- `/code-review`: Comprehensive code quality review.
- `/security-scan`: Security analysis of the codebase.
- `/debug-error`: Systematically debug and fix errors.
- `/optimize-bundle-size`: Reduce and optimize application bundle size.
- `/implement-caching-strategy`: Design and implement caching solutions.
- `/design-database-schema`: Design an optimized database schema.
- `/add-performance-monitoring`: Set up application performance monitoring.

### Simulation & Modeling

- `/architecture-scenario-explorer`: Explore architecture scenarios.
- `/decision-tree-explorer`: Explore complex decision branches.
- `/system-behavior-simulator`: Simulate system behavior.
- `/system-dynamics-modeler`: Model complex system dynamics.
- `/simulation-calibrator`: Calibrate simulation accuracy.

### Automation & Orchestration

- `/workflow-orchestrator`: Orchestrate complex automation workflows.
- `/rule2hook`: Convert project rules to Claude Code hooks.
- `/ultra-think`: Engage deep analysis and problem-solving mode.

### Miscellaneous

- `/add-dir`: Add a new working directory.
- `/all-tools`: Display all available development tools.
- `/cipher-memory`: Cipher Memory Commands.
- `/cipher-memory:system_prompt`: Get the system prompt for the Cipher agent.
- `/code-permutation-tester`: Test different permutations of code.
- `/contributing`: Complete Contribution Strategy - Context Aware.
- `/cost`: Show session cost and duration.
- `/create_command`: Command Creator Assistant.
- `/cleanproject`: Clean the project.
- `/privacy-settings`: View and update privacy settings.
