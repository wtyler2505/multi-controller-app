# Git Automation System

## Overview

Intelligent git/GitHub automation system for the Multi-Controller App project, providing:
- **Safety checks**: Secrets scanning, performance validation
- **Task integration**: Branch creation from Task Master tasks
- **Smart commits**: Conventional commits with task references
- **PR automation**: Create PRs with task context and checklists
- **Sync monitoring**: Real-time repository synchronization status

## Quick Start

### 1. Install Git Hooks

```bash
# Windows
npm run git:install-hooks

# Linux/Mac
npm run git:install-hooks-bash
```

This installs three hooks:
- `pre-commit`: Runs secrets scanner and performance checks
- `commit-msg`: Enforces conventional commits and adds task references
- `pre-push`: Final validation before pushing to remote

### 2. Basic Workflow

```bash
# Start working on a task
npm run task:start 11          # Creates branch feature/task-11-fix-memory-leaks

# Make changes and commit
npm run task:commit             # Interactive smart commit with task context

# Create pull request
npm run task:pr                 # Creates PR with task details and checklist

# Check sync status
npm run sync:status             # Shows local vs remote synchronization
```

## Available Commands

### Task Workflows
- `npm run task:branch <id>` - Create/switch to task branch
- `npm run task:commit` - Smart commit with task context
- `npm run task:pr [id]` - Create PR for current branch or task

### Validation
- `npm run validate:all` - Run all validation checks
- `npm run validate:secrets` - Check for exposed secrets
- `npm run validate:perf` - Check performance budgets

### Synchronization
- `npm run sync:status` - Show sync dashboard
- `npm run sync:status-detailed` - Show with file details
- `npm run sync:watch` - Live monitoring mode
- `npm run sync:auto` - Auto-fix safe sync issues

### Git Hooks Management
- `npm run git:install-hooks` - Install all hooks (Windows)
- `npm run git:install-hooks-bash` - Install all hooks (Linux/Mac)

## Configuration

### Secrets Patterns
Edit `.gitmeta/config/secrets-patterns.json`:
```json
{
  "patterns": ["API_KEY", "SECRET", "PASSWORD"],
  "files": [".env", "*.key", "*.pem"],
  "regex_patterns": ["Bearer\\s+[A-Za-z0-9\\-_=]+"]
}
```

### Performance Budgets
Edit `.gitmeta/config/performance-budgets.json`:
```json
{
  "startup": { "max_ms": 2000, "warn_ms": 1500 },
  "memory": { "max_mb": 150, "warn_mb": 130 },
  "cpu_idle": { "max_percent": 2 }
}
```

## Features

### 1. Secret Protection
- Scans staged files for API keys, passwords, tokens
- Blocks commits containing sensitive data
- Configurable patterns and file types

### 2. Performance Gates
- Validates startup time < 2s
- Checks memory usage < 150MB
- Ensures idle CPU ≤ 2%
- Runs automatically on .NET file changes

### 3. Task-Branch Mapping
- Creates branches from Task Master tasks
- Format: `feature/task-{id}-{title-slug}`
- Updates task status to "in-progress"
- Links commits to tasks automatically

### 4. Smart Commits
- Interactive commit message generator
- Conventional commit format enforcement
- Automatic task reference addition
- Analyzes changes to suggest commit type

### 5. PR Automation
- Creates PRs with task context
- Includes implementation checklist
- Groups changes by type (features, fixes, tests)
- Updates task status to "review"

### 6. Sync Dashboard
- Real-time local vs remote status
- Shows ahead/behind commits
- Lists uncommitted changes
- Provides actionable recommendations
- Auto-fix mode for safe operations

## Commit Convention

All commits must follow conventional format:
```
type(scope): description (task X.Y)
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `style`: Formatting changes
- `refactor`: Code restructuring
- `perf`: Performance improvements
- `test`: Test additions/changes
- `build`: Build system changes
- `ci`: CI/CD changes
- `chore`: Maintenance tasks
- `revert`: Revert previous commit

### Examples
```bash
feat(serial): implement reconnection logic (task 11)
fix(transport): resolve memory leak in event handlers (task 11.1)
docs(readme): update installation instructions
perf(telemetry): optimize ring buffer performance (task 13)
```

## Bypassing Checks (Use Sparingly)

```bash
# Bypass pre-commit hooks
git commit --no-verify

# Bypass pre-push validation
git push --no-verify
```

## Troubleshooting

### Hooks Not Running
- Ensure hooks are installed: `npm run git:install-hooks`
- Check `.git/hooks/` directory for hook files
- Verify Node.js is installed: `node --version`

### Performance Check Fails
- Build project first: `dotnet build -c Release`
- Skip startup check: Use `--skip-startup` flag
- Adjust budgets in `.gitmeta/config/performance-budgets.json`

### Secrets Scanner False Positives
- Update patterns in `.gitmeta/config/secrets-patterns.json`
- Use `git commit --no-verify` for legitimate cases
- Add files to `.gitignore` if they shouldn't be committed

### PR Creation Fails
- Install GitHub CLI: `winget install GitHub.cli`
- Authenticate: `gh auth login`
- Push branch first: `git push -u origin <branch>`

## Architecture

```
scripts/git-automation/
├── hooks/                  # Git hook templates
│   ├── pre-commit         # Runs validation checks
│   ├── commit-msg         # Enforces commit format
│   └── pre-push           # Final validation
├── task-branch.js         # Task-to-branch mapping
├── smart-commit.js        # Intelligent commit messages
├── pr-create.js           # PR automation with task context
├── secrets-scanner.js     # Detect exposed secrets
├── performance-gate.ps1/sh # Performance validation
├── sync-status.ps1        # Sync dashboard
└── install-hooks.ps1/sh  # Hook installation scripts

.gitmeta/
├── config/
│   ├── secrets-patterns.json    # Secret detection patterns
│   └── performance-budgets.json # Performance thresholds
├── templates/             # Message templates
└── logs/                  # Automation logs
```

## Integration with Task Master

The system deeply integrates with Task Master for:
- Creating branches from tasks
- Adding task references to commits
- Including task details in PRs
- Updating task status through workflow
- Validating task dependencies

## Best Practices

1. **Always work on task branches**: Use `npm run task:start <id>`
2. **Commit frequently**: Small, focused commits with clear messages
3. **Run validation**: Use `npm run validate:all` before pushing
4. **Keep synced**: Check `npm run sync:status` regularly
5. **Use smart tools**: Leverage `task:commit` and `task:pr` commands
6. **Review checklists**: Complete PR validation before merging

## License

Part of Multi-Controller App project - MIT License