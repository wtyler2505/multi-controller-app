# Task Master ↔ GitHub Issues Sync Guide

## Overview

This guide documents the bidirectional synchronization between Task Master and GitHub Issues, enabling team collaboration and visibility for the Multi-Controller App project.

## Current Status

- **GitHub Issues Created**: #2-#11 (Tasks 27-36)
- **Sync Script**: `scripts/git-automation/taskmaster-github-sync.js`
- **Mapping File**: `.taskmaster/github-map.json`
- **Repository**: wtyler2505/multi-controller-app

## Architecture

### Components

1. **Task Master**: Local task management system with 36 tasks
2. **GitHub Issues**: Remote collaboration interface
3. **Sync Script**: Node.js bridge for bidirectional sync
4. **Mapping File**: Persistent ID mappings

### Data Flow

```
Task Master (tasks.json) <--> Sync Script <--> GitHub Issues API
                                  |
                            github-map.json
```

## Usage

### Running the Sync

```bash
# Navigate to scripts directory
cd scripts/git-automation

# Dry run (preview changes)
node taskmaster-github-sync.js --dry-run --verbose

# Actual sync
node taskmaster-github-sync.js --verbose

# Silent mode
node taskmaster-github-sync.js
```

### Command Options

- `--dry-run`: Preview changes without applying them
- `--verbose`: Show detailed output
- `--help`: Display help information

## Status Mapping

### Task Master → GitHub

| Task Master Status | GitHub State | GitHub Labels |
| ------------------ | ------------ | ------------- |
| pending            | open         | -             |
| in-progress        | open         | in-progress   |
| review             | open         | review        |
| done               | closed       | -             |
| blocked            | open         | blocked       |
| deferred           | open         | deferred      |
| cancelled          | closed       | -             |

### GitHub → Task Master

| GitHub State | Task Master Status |
| ------------ | ------------------ |
| open         | pending            |
| closed       | done               |

## GitHub Labels

### Priority Labels

- `priority:high` - Critical tasks
- `priority:medium` - Standard priority
- `priority:low` - Nice to have

### Status Labels

- `in-progress` - Currently being worked on
- `review` - Ready for review
- `blocked` - Waiting on external factors
- `deferred` - Postponed

### Type Labels

- `task:multi-controller` - All synced tasks
- `epic` - Tasks with subtasks
- `agent:serial-comm-specialist` - Assigned to specific agent

## Task-to-Issue Mappings

| Task ID | GitHub Issue | Title                                            |
| ------- | ------------ | ------------------------------------------------ |
| 27      | #2           | Implement Device Connection & Handshake Protocol |
| 28      | #3           | Develop Manual Control Widgets                   |
| 29      | #4           | Implement Telemetry Visualization Charts         |
| 30      | #5           | Create Telemetry Collection Buffer               |
| 31      | #6           | Implement Profile Management System              |
| 32      | #7           | Develop Command Processing Queue                 |
| 33      | #8           | Implement Comprehensive Logging System           |
| 34      | #9           | Add Scripting Engine Support                     |
| 35      | #10          | Optimize egui Performance for Real-time Updates  |
| 36      | #11          | Implement Complete Session Lifecycle             |

## CCPM Epic Integration

Each GitHub issue corresponds to a CCPM epic with specialized agent assignments:

```yaml
Task 27: serial-comm-specialist + handshake-protocol-engineer
Task 28: ui-controls-architect
Task 29: visualization-engineer
Task 30: telemetry-collector
Task 31: profile-manager
Task 32: command-processor
Task 33: logging-integrator
Task 34: scripting-architect
Task 35: performance-optimizer
Task 36: transport-lifecycle-guardian
```

## Sync Process

### Initial Setup (Completed)

1. Created GitHub labels for categorization
2. Generated github-map.json with mappings
3. Created GitHub Issues #2-#11 for tasks 27-36
4. Implemented sync script with bidirectional support

### Ongoing Synchronization

1. **Task Updates**: Run sync script after Task Master changes
2. **Issue Updates**: Sync pulls GitHub changes to Task Master
3. **Status Tracking**: Automatic state synchronization
4. **Label Management**: Priority and status reflected in labels

### Manual Sync Workflow

```bash
# 1. Check current status
task-master list

# 2. Make updates in Task Master
task-master set-status --id=27 --status=in-progress

# 3. Sync to GitHub
cd scripts/git-automation
node taskmaster-github-sync.js --verbose

# 4. Verify in GitHub
gh issue list --state all
```

## Automation Options

### GitHub Actions (Future)

```yaml
name: Sync Task Master
on:
  schedule:
    - cron: '0 */6 * * *' # Every 6 hours
  workflow_dispatch:

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: node scripts/git-automation/taskmaster-github-sync.js
```

### Local Cron (Windows Task Scheduler)

```powershell
# Create scheduled task
$action = New-ScheduledTaskAction -Execute "node" -Argument "C:\path\to\taskmaster-github-sync.js"
$trigger = New-ScheduledTaskTrigger -Daily -At 9am
Register-ScheduledTask -TaskName "TaskMasterSync" -Action $action -Trigger $trigger
```

## Troubleshooting

### Common Issues

1. **Authentication Failed**
   - Ensure `gh` CLI is authenticated: `gh auth status`
   - Login if needed: `gh auth login`

2. **Mapping File Corrupt**
   - Backup exists at `.taskmaster/github-map.json.backup`
   - Restore: `cp .taskmaster/github-map.json.backup .taskmaster/github-map.json`

3. **Rate Limiting**
   - GitHub API has rate limits
   - Use `--verbose` to see API calls
   - Wait or authenticate for higher limits

4. **Sync Conflicts**
   - Manual changes in both systems
   - Review with `--dry-run` first
   - Resolve conflicts manually

### Debug Commands

```bash
# Check GitHub authentication
gh auth status

# List all issues
gh issue list --state all --limit 50

# View specific issue
gh issue view 2

# Check Task Master status
task-master list --status pending

# View sync logs
cat .taskmaster/sync-logs/sync-2025-09-07.json
```

## Best Practices

1. **Always Dry Run First**: Use `--dry-run` before actual sync
2. **Regular Syncs**: Run at least daily to prevent conflicts
3. **Single Source of Truth**: Prefer Task Master for task management
4. **Team Communication**: Notify team of major sync operations
5. **Backup Before Sync**: Keep github-map.json backed up

## Future Enhancements

### Planned Features

1. **Bidirectional Comment Sync**: Sync comments between systems
2. **Assignee Management**: Map Task Master owners to GitHub assignees
3. **Milestone Integration**: Link tasks to GitHub milestones
4. **Pull Request Linking**: Auto-link PRs to issues
5. **Webhook Support**: Real-time sync via webhooks

### Integration Points

- **CI/CD Pipeline**: Auto-sync on commits
- **Slack Notifications**: Alert team of sync status
- **Dashboard**: Web interface for sync monitoring
- **Metrics**: Track sync performance and reliability

## Support

For issues or questions:

1. Check sync logs in `.taskmaster/sync-logs/`
2. Review this documentation
3. Contact project maintainer: wtyler2505@outlook.com

---

_Last Updated: 2025-09-07_
_Version: 1.0.0_
_Status: Production Ready_
