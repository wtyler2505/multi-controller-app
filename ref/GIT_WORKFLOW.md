# Git Workflow & GitHub Integration

**Last Updated**: 2025-08-25  
**Automation Level**: Full (9 components)  
**Integration**: Task Master + CCPM + GitHub Issues  
**Safety**: Enforced via hooks

## Repository Information

- **Repository URL**: https://github.com/wtyler2505/multi-controller-app
- **Owner**: wtyler2505
- **Email**: wtyler2505@outlook.com
- **License**: MIT

## Branch Strategy

### Main Branches
- **`main`**: Production-ready code, protected branch
- **`development`**: Active development, integration branch

### Feature Branches
- Format: `feature/<task-id>-<brief-description>`
- Example: `feature/task-2.1-serial-transport`
- Created from: `development`
- Merged to: `development` via PR

### Hotfix Branches
- Format: `hotfix/<issue-number>-<brief-description>`
- Created from: `main`
- Merged to: Both `main` and `development`

## Git Configuration

### Required Setup
```bash
git config user.name "wtyler2505"
git config user.email "wtyler2505@outlook.com"
```

### Recommended Aliases
```bash
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.st status
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
```

## 9-Component Git Automation System

The project features a comprehensive git automation system with three phases:

### Phase 1: Safety Net (Components 1-3)

#### 1. Secrets Scanner (`scripts/git-automation/secrets-scanner.js`)
- **Purpose**: Pre-commit hook to detect exposed credentials
- **Usage**: Automatic on `git commit`
- **Manual**: `npm run validate:secrets`
- **Patterns Detected**:
  - API keys (OpenAI, Anthropic, AWS, etc.)
  - Passwords and tokens
  - Private keys and certificates
  - Connection strings

#### 2. Performance Gate (`scripts/git-automation/performance-gate.ps1`)
- **Purpose**: Enforce performance budgets before commit
- **Usage**: Automatic on `git commit`
- **Manual**: `npm run validate:perf`
- **Validates**:
  - Startup time < 2s
  - Memory usage ≤ 150MB
  - CPU idle ≤ 2%

#### 3. Git Hooks Setup (`scripts/git-automation/setup-hooks.ps1`)
- **Purpose**: Install and maintain git hooks
- **Usage**: `npm run git:install-hooks`
- **Installs**:
  - pre-commit: Secrets + performance
  - commit-msg: Conventional commits
  - pre-push: Final validation

### Phase 2: Task Integration (Components 4-6)

#### 4. Task Branch Creator (`scripts/git-automation/task-branch.js`)
- **Purpose**: Create branches from Task Master tasks
- **Usage**: `npm run task:start <task-id>`
- **Features**:
  - Fetches task details from Task Master
  - Creates branch: `feature/task-X.Y-description`
  - Sets task status to in-progress
  - Switches to new branch

#### 5. Smart Commit (`scripts/git-automation/smart-commit.js`)
- **Purpose**: Generate intelligent commit messages
- **Usage**: `npm run task:commit`
- **Features**:
  - Analyzes staged changes
  - Suggests commit type and scope
  - Includes task ID reference
  - Interactive confirmation

#### 6. PR Creator (`scripts/git-automation/pr-create.js`)
- **Purpose**: Create PRs with task context
- **Usage**: `npm run task:pr [task-id]`
- **Features**:
  - Pulls task details from Task Master
  - Generates comprehensive PR body
  - Includes test checklist
  - Links to task documentation

### Phase 3: Synchronization (Components 7-9)

#### 7. Sync Status Dashboard (`scripts/git-automation/sync-status.ps1`)
- **Purpose**: Real-time repository status
- **Usage**: `npm run sync:status`
- **Shows**:
  - Local vs remote differences
  - Uncommitted changes
  - Branch status
  - Stash entries

#### 8. Auto-Fixer (`scripts/git-automation/sync-auto-fix.ps1`)
- **Purpose**: Resolve common sync issues
- **Usage**: `npm run sync:auto`
- **Fixes**:
  - Diverged branches (safe merge)
  - Uncommitted changes (auto-stash)
  - Missing upstream (set tracking)
  - Outdated branches (fetch + pull)

#### 9. Watch Mode (`scripts/git-automation/sync-watch.ps1`)
- **Purpose**: Live monitoring of repository
- **Usage**: `npm run sync:watch`
- **Features**:
  - Real-time file change detection
  - Automatic status updates
  - Performance metrics
  - Alert on issues

### NPM Scripts Reference

```bash
# Safety Commands
npm run validate:all        # Run all validations
npm run validate:secrets    # Check for exposed secrets
npm run validate:perf       # Check performance budgets
npm run git:install-hooks   # Install git hooks

# Task Integration
npm run task:start <id>     # Create task branch
npm run task:commit         # Smart commit with context
npm run task:pr [id]        # Create PR for task

# Synchronization
npm run sync:status         # Show sync dashboard
npm run sync:auto          # Auto-fix sync issues
npm run sync:watch         # Live monitoring mode

# Combined Workflows
npm run task:complete <id>  # Commit + PR + merge
```

## Commit Conventions

### Commit Message Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- **feat**: New feature
- **fix**: Bug fix
- **docs**: Documentation changes
- **style**: Code style changes (formatting, etc.)
- **refactor**: Code refactoring
- **perf**: Performance improvements
- **test**: Test additions or corrections
- **build**: Build system changes
- **ci**: CI/CD configuration changes
- **chore**: Routine tasks, maintenance

### Examples
```bash
feat(driver): implement Arduino Uno serial driver

fix(transport): resolve TCP reconnection timeout issue

docs(readme): update installation instructions

perf(telemetry): optimize ring buffer for high-frequency data
```

### Task References
Always reference Task Master task IDs when applicable:
```bash
git commit -m "feat(auth): implement JWT authentication (task 1.2)"
```

## Pull Request Process

### Creating a PR
```bash
# Using GitHub CLI
gh pr create --title "Task 1.2: Implement JWT authentication" \
  --body "Implements JWT auth system as specified in task 1.2" \
  --base development
```

### PR Template
```markdown
## Summary
Brief description of changes

## Task Reference
- Task ID: [e.g., 1.2]
- Task Title: [e.g., Implement JWT authentication]

## Changes Made
- List of specific changes
- Implementation details

## Testing
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed

## Performance Impact
- Startup time: No change / +Xms
- Memory usage: No change / +XMB
- CPU usage: No change / +X%

## Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] No exposed secrets or API keys
```

## Security Guidelines

### Never Commit
- API keys or tokens
- Passwords or credentials
- Private keys or certificates
- Personal information
- Internal URLs or endpoints

### Use Instead
- Environment variables: `${API_KEY}`
- .env files (in .gitignore)
- Secrets management service
- Placeholder values in examples

### If Secrets Are Accidentally Committed
1. Immediately rotate the exposed credentials
2. Create new orphan branch:
   ```bash
   git checkout --orphan clean-branch
   git add .
   git commit -m "Initial commit without secrets"
   git branch -D main
   git branch -m main
   git push -f origin main
   ```

## Git Automation System

The project includes a comprehensive git automation system that provides safety checks, task integration, and smart workflows.

### Quick Start with Automation

```bash
# Install git hooks (one-time setup)
npm run git:install-hooks        # Windows
npm run git:install-hooks-bash   # Linux/Mac

# Daily workflow with automation
npm run task:start 11            # Create branch for task 11
npm run task:commit              # Smart commit with task context
npm run task:pr                  # Create PR with validation
npm run sync:status              # Check repository sync status
```

### Available Automation Commands

#### Task Integration
- `npm run task:branch <id>` - Create/switch to task branch
- `npm run task:commit` - Interactive smart commit with task context
- `npm run task:pr [id]` - Create PR with task details and checklist
- `npm run task:start <id>` - Alias for task:branch

#### Validation & Safety
- `npm run validate:all` - Run all validation checks
- `npm run validate:secrets` - Check for exposed API keys/credentials
- `npm run validate:perf` - Verify performance budgets
- `npm run validate:ps1` - Validate PowerShell scripts for compatibility

#### Synchronization
- `npm run sync:status` - Display sync dashboard
- `npm run sync:status-detailed` - Show with file-level details
- `npm run sync:watch` - Live monitoring mode
- `npm run sync:auto` - Auto-fix safe synchronization issues

#### Git Operations
- `npm run git:install-hooks` - Install pre-commit, commit-msg, pre-push hooks
- `npm run git:check-secrets` - Manual secrets scan
- `npm run git:check-perf` - Manual performance validation

### Automation Features

#### 1. Secrets Protection
- **Pre-commit scanning**: Blocks commits containing API keys, passwords, tokens
- **Configuration**: `.gitmeta/config/secrets-patterns.json`
- **Patterns detected**: API_KEY, SECRET, PASSWORD, Bearer tokens
- **Recovery**: If detected, rotation required before proceeding

#### 2. Performance Gates
- **Automatic validation**: Runs on .NET/C# file changes
- **Budgets enforced**:
  - Startup time < 2s
  - Memory usage < 150MB
  - Idle CPU ≤ 2%
- **Configuration**: `.gitmeta/config/performance-budgets.json`

#### 3. Task-Driven Development
- **Branch naming**: `feature/task-{id}-{title-slug}`
- **Commit integration**: Automatic task ID references
- **PR context**: Task details included in PR description
- **Status tracking**: Integrated with Task Master

#### 4. Smart Commit Generation
The `npm run task:commit` command provides:
- Analyzes staged files to suggest commit type
- Fetches current task context
- Enforces conventional commit format
- Adds task reference automatically
- Interactive prompts for details

#### 5. PowerShell Script Validation
Special validation for Windows PowerShell scripts:
- **ASCII-only enforcement**: No emojis or Unicode characters
- **Box-drawing replacement**: Converts to ASCII alternatives
- **Validation command**: `npm run validate:ps1 <script.ps1>`
- **Common fixes**:
  - ✅ → [OK]
  - ❌ → [ERROR]
  - ⚠️ → [WARNING]
  - ╔══╗ → +==+

### Git Hooks

Three hooks are automatically installed:

#### Pre-commit Hook
```bash
#!/bin/sh
# Runs automatically before each commit
node scripts/git-automation/secrets-scanner.js || exit 1
# Performance check for .NET changes
if git diff --cached --name-only | grep -E '\.(cs|csproj)$'; then
  npm run validate:perf || exit 1
fi
```

#### Commit-msg Hook
```bash
#!/bin/sh
# Enforces conventional commits and adds task references
node scripts/git-automation/validate-commit-msg.js "$1" || exit 1
```

#### Pre-push Hook
```bash
#!/bin/sh
# Final validation before pushing
npm run validate:all || exit 1
```

## Workflow Commands

### Automated Daily Development
```bash
# Start working on a task
npm run task:start 15           # Creates feature/task-15-implement-feature

# Make changes and commit
git add .
npm run task:commit              # Interactive smart commit

# Create pull request
npm run task:pr                  # Creates PR with task context

# Check synchronization
npm run sync:status              # View local vs remote status
```

### Manual Daily Development (Fallback)
```bash
# Start new feature
git checkout development
git pull origin development
git checkout -b feature/task-X.Y-description

# Work on feature
git add .
git commit -m "feat: implement feature (task X.Y)"

# Push and create PR
git push -u origin feature/task-X.Y-description
gh pr create

# After PR approval and merge
git checkout development
git pull origin development
git branch -d feature/task-X.Y-description
```

### Syncing with Upstream
```bash
# Update local development
git checkout development
git pull origin development

# Update local main
git checkout main
git pull origin main

# Rebase feature branch
git checkout feature/my-feature
git rebase development
```

### Handling Conflicts
```bash
# During rebase
git rebase development
# Fix conflicts in files
git add <resolved-files>
git rebase --continue

# Or abort if needed
git rebase --abort
```

## GitHub Actions Integration

### Automated Workflows
- **CI Pipeline**: Runs on all PRs
- **Build Validation**: Checks compilation
- **Test Suite**: Runs unit and integration tests
- **Code Quality**: Linting and formatting checks
- **Security Scanning**: Checks for vulnerabilities

### Manual Workflows
- **Release Build**: Creates production artifacts
- **Performance Tests**: Runs benchmark suite
- **Deploy**: Deploys to staging/production

## Collaboration Guidelines

### Code Reviews
- All code requires review before merge
- Address all feedback constructively
- Use "Suggest changes" for specific improvements
- Approve with confidence level:
  - ✅ "Approve" - Ready to merge
  - 💭 "Comment" - Questions or suggestions
  - ❌ "Request changes" - Must fix before merge

### Issue Management
- Link PRs to issues: "Fixes #123"
- Use labels appropriately
- Update issue status in Task Master
- Close issues via commit messages

