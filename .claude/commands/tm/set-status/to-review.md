---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management", "analysis"]
description: Command for to-review operations
---

Set a task's status to review.

Arguments: $ARGUMENTS (task ID)

## Marking Task for Review

This status indicates work is complete but needs verification before final approval.

## When to Use Review Status

- Code complete but needs peer review
- Implementation done but needs testing
- Documentation written but needs proofreading
- Design complete but needs stakeholder approval

## Execution

```bash
task-master set-status --id=$ARGUMENTS --status=review
```

## Review Preparation

When setting to review:
1. **Generate Review Checklist**
   - Link to PR/MR if applicable
   - Highlight key changes
   - Note areas needing attention
   - Include test results

2. **Documentation**
   - Update task with review notes
   - Link relevant artifacts
   - Specify reviewers if known

3. **Smart Actions**
   - Create review reminders
   - Track review duration
   - Suggest reviewers based on expertise
   - Prepare rollback plan if needed


