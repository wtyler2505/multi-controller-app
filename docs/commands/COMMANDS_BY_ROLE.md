# Slash Commands by Role

This guide organizes commands based on common roles within a software development team, such as Developer and Project Manager.

## For the Core Developer

Commands focused on the day-to-day cycle of coding, testing, and committing.

### Daily Workflow
- `/tm:next:next-task`: Get your next assigned task.
- `/tm:show:show-task`: Review the details and requirements of the task.
- `/implement`: Write new code with AI assistance.
- `/refactor`: Improve and clean up existing code.
- `/format`: Ensure your code adheres to project style guidelines.
- `/fix-imports`: Quickly resolve broken import statements.
- `/test`: Run tests against your changes.
- `/commit`: Draft a well-formed commit message.
- `/tm:set-status:to-done`: Mark your task as complete.

### Problem Solving & Understanding
- `/explain-like-senior`: Understand a complex piece of code.
- `/debug-error`: Systematically find and fix bugs.
- `/understand`: Get a high-level overview of the project.
- `/directory-deep-dive`: Analyze a specific part of the codebase.

### Test Generation
- `/generate-tests`: Create a full test suite for a file.
- `/write-tests`: Add specific unit or integration tests.
- `/test-coverage`: Check for gaps in test coverage.

### Documentation
- `/update-docs`: Update documentation to reflect your code changes.
- `/todos`: Check for any outstanding TODOs in the code.
- `/create-todos`: Add new TODOs for future work.

## For the Project Manager / Team Lead

Commands focused on planning, tracking, and managing the project and its tasks.

### Task & Project Management (`/tm:`, `/pm:`)
- `/tm:init:init-project`: Start a new project in Task Master.
- `/tm:parse-prd:parse-prd`: Import tasks from a Product Requirements Document.
- `/tm:list:list-tasks`: View all tasks, with filters for status, priority, etc.
- `/tm:status:project-status`: Get a high-level overview of project progress.
- `/pm:standup`: Generate a daily standup report.
- `/tm:add-task:add-task`: Add new tasks to the project.
- `/tm:update:update-task`: Modify existing tasks.
- `/tm:expand:expand-task`: Break down large tasks into smaller, manageable subtasks.
- `/tm:add-dependency:add-dependency`: Define relationships and blockers between tasks.

### Analysis & Reporting
- `/tm:analyze-complexity:analyze-complexity`: Assess the effort required for upcoming tasks.
- `/tm:complexity-report:complexity-report`: View the detailed complexity analysis.
- `/project-health-check`: Get a report on key project health metrics.
- `/predict-issues`: Identify potential risks and future problem areas in the code.

### Epic & Issue Management (`/pm:`)
- `/pm:epic-list`: View all high-level epics.
- `/pm:epic-decompose`: Break down an epic into user stories or tasks.
- `/pm:epic-sync`: Sync epic status with other systems.
- `/pm:issue-show`: Review details of a specific issue.

### Documentation & Onboarding
- `/create-onboarding-guide`: Create a guide to help new developers get started.
- `/create-architecture-documentation`: Generate and maintain architecture documents.
- `/tm:sync-readme:sync-readme`: Export the current task list to a README file.

## For the DevOps / SRE Role

Commands focused on infrastructure, CI/CD, and environment setup.

- `/setup-development-environment`: Standardize the development environment for the team.
- `/setup-comprehensive-testing`: Configure the full testing infrastructure.
- `/e2e-setup`: Set up the end-to-end testing suite.
- `/setup-monitoring-observability`: Configure monitoring and observability tools.
- `/add-performance-monitoring`: Add detailed performance metric collection.
- `/implement-caching-strategy`: Design and set up caching layers.
- `/optimize-bundle-size`: Analyze and reduce application bundle size.
- `/install-github-app`: Integrate Claude with GitHub Actions.