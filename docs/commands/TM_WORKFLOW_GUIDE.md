# Task Master (`/tm:`) Workflow Guide

This guide provides a walkthrough of a standard development workflow using the `/tm:` command suite.

## Step 1: Project Initialization

If you are starting a new project, initialize Task Master first.

1.  **Initialize the project:**
    ```
    /tm:init:init-project
    ```
2.  **Create a Product Requirements Document (PRD)** in `.taskmaster/docs/prd.txt`.
3.  **Parse the PRD to create tasks:**
    ```
    /tm:parse-prd:parse-prd
    ```

## Step 2: Task Analysis and Decomposition

Before starting work, it's good practice to break down large tasks.

1.  **Analyze task complexity:**
    ```
    /tm:analyze-complexity:analyze-complexity
    ```
2.  **Review the complexity report:**
    ```
    /tm:complexity-report:complexity-report
    ```
3.  **Expand complex tasks into subtasks:**
    - For a single task: `/tm:expand:expand-task --id=<task-id>`
    - For all eligible tasks: `/tm:expand:expand-all-tasks`

## Step 3: The Daily Development Loop

This is the core loop you'll follow each day.

1.  **Find your next task:**
    ```
    /tm:next:next-task
    ```
2.  **Review the task details:**
    The `next` command usually shows the details, but you can always pull them up manually.
    ```
    /tm:show:show-task --id=<task-id>
    ```
3.  **Set the task status to `in-progress`:**
    ```
    /tm:set-status:to-in-progress --id=<task-id>
    ```
4.  **Implement the code.** Use commands like `/implement`, `/refactor`, and `/test` as you work.

5.  **Add subtasks for minor to-dos** that come up during development:
    ```
    /tm:add-subtask:add-subtask --parent-id=<task-id> --title="Refactor the helper function"
    ```

6.  **When finished, mark the task as done:**
    ```
    /tm:set-status:to-done --id=<task-id>
    ```

## Step 4: Managing Tasks and Dependencies

As the project evolves, you'll need to manage the task list.

-   **Add a new task manually:**
    ```
    /tm:add-task:add-task --title="Create a new login page" --priority=high
    ```
-   **Update an existing task:**
    ```
    /tm:update:update-single-task --id=<task-id> --description="Updated requirements..."
    ```
-   **Add a dependency:** If Task `2.1` must be completed before Task `2.2`.
    ```
    /tm:add-dependency:add-dependency --id=2.2 --depends-on=2.1
    ```
-   **Validate dependencies:** Check for circular or broken dependencies.
    ```
    /tm:validate-dependencies:validate-dependencies
    ```

## Step 5: Reporting and Status

Keep track of the project's overall progress.

-   **List all tasks in progress:**
    ```
    /tm:list:list-tasks --status in-progress
    ```
-   **Get a full project status overview:**
    ```
    /tm:status:project-status
    ```
-   **Generate a `README.md` with the current task list:**
    ```
    /tm:sync-readme:sync-readme
    ```