---
allowed-tools: Read, Write, LS, Bash, mcp__taskmaster-ai__get_tasks
---

# Bridge: Bidirectional Sync Task Master ↔ GitHub

Synchronize Task Master tasks with GitHub Issues for team visibility.

## Usage
```
/bridge:sync-all
```

## Steps

1. **Export Task Master to GitHub**
   - Get all tasks from Task Master
   - For each task without GitHub Issue ID:
     ```bash
     gh issue create \
       --title "[Task {id}] {title}" \
       --body "{description}\n\nTask Master ID: {id}" \
       --label "task:multi-controller"
     ```
   - Store Issue number back in Task Master metadata

2. **Create Sub-Issues for Subtasks**
   - For tasks with subtasks:
     ```bash
     gh sub-issue create {parent-issue} \
       --title "[Subtask {id}] {title}" \
       --body "{details}"
     ```

3. **Update Existing Issues**
   - For tasks with Issue IDs:
     ```bash
     gh issue edit {issue-id} \
       --body "{updated-description}\n\nStatus: {status}"
     ```

4. **Import GitHub Comments**
   - For each issue with new comments:
     ```bash
     gh issue view {issue-id} --comments
     ```
   - Add as Task Master updates if relevant

5. **Sync Status**
   - Map Task Master status to GitHub:
     - pending → open
     - in-progress → open + "in-progress" label
     - review → open + "review" label
     - done → closed
     - blocked → open + "blocked" label

6. **Generate Report**
   ```
   Synced Tasks:
   - Task 4 → Issue #1245
   - Task 5 → Issue #1246
   
   Updated Statuses:
   - Task 3: done → Issue #1244: closed
   
   New Comments:
   - Issue #1243: 2 new comments imported
   ```

## Metadata Storage

Store GitHub Issue IDs in `.taskmaster/github-map.json`:
```json
{
  "tasks": {
    "4": 1245,
    "4.1": 1247,
    "5": 1246
  },
  "lastSync": "2025-08-25T06:00:00Z"
}
```

## Error Handling
- Rate limit detection and backoff
- Partial sync on failure
- Conflict resolution for concurrent edits