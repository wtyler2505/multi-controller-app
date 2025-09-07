# Brutal Minimalism Hook Configuration

## Philosophy
**Only hooks that prevent catastrophic failures or preserve critical knowledge.**

## Current Hook Setup (As of 2025-01-05)

### ✅ KEPT: Essential Hooks Only

#### 1. Pre-Commit Test Hook
**Location**: `.git/hooks/pre-commit.ps1`
**Purpose**: Prevent broken commits
**What it does**:
- Runs `cargo test --quiet`
- Runs `cargo clippy --quiet`
- Blocks commit on failure
**Performance**: <30 seconds

#### 2. Cleanup Resources Check (Optional)
**Location**: `.claude/minimal-hooks/check-cleanup-resources.ps1`
**Purpose**: Warn about missing cleanup_resources() calls
**What it does**:
- Checks transport files for cleanup pattern
- Warns but doesn't block
**Performance**: <100ms

#### 3. Unwrap Check (Optional)
**Location**: `.claude/minimal-hooks/check-unwrap.ps1`
**Purpose**: Warn about unwrap() in production code
**What it does**:
- Scans for .unwrap() calls
- Warns but doesn't block
**Performance**: <100ms

### ❌ REMOVED: Non-Essential Hooks

- **Auto-format hooks** - Disrupts flow, use manual formatting
- **Verbose logging hooks** - Too noisy, clutters output
- **File watcher hooks** - Performance killer
- **Auto-documentation hooks** - Over-engineering for personal project
- **Complex validation hooks** - False positives, unnecessary complexity
- **Session tracking hooks** - Not needed for single developer
- **Telemetry hooks** - Privacy concerns, no value
- **Auto-PR hooks** - Manual control preferred
- **Style enforcement hooks** - Trust developer discipline

## Windows Compatibility Notes

### PowerShell Execution
```powershell
# If hooks don't run, check execution policy:
Get-ExecutionPolicy
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### Git Hook Setup
```bash
# Make git use PowerShell for hooks on Windows
git config core.hooksPath .git/hooks
```

### Path Issues
- Use forward slashes in git hooks
- Use backslashes in PowerShell scripts
- Always quote paths with spaces

## Testing Hooks

### Test Pre-Commit Hook
```powershell
# Direct test
powershell .\.git\hooks\pre-commit.ps1

# Test with intentional failure
echo "fn bad() { panic!() }" >> src/test.rs
git add . && git commit -m "test"
# Should fail
git restore src/test.rs
```

### Performance Measurement
```powershell
# Measure hook execution time
Measure-Command { .\.git\hooks\pre-commit.ps1 }
```

## Monitoring & Adjustment

### Hook Metrics to Track
1. **Execution time** - Should be <30s for pre-commit
2. **False positive rate** - Should be <1%
3. **Times hook prevented bad commit** - Track value

### When to Add a Hook
Only add a new hook if:
1. It prevents data loss or corruption
2. It catches critical bugs before production
3. It saves >10 minutes of debugging time per occurrence
4. It has <1% false positive rate

### When to Remove a Hook
Remove a hook if:
1. It triggers false positives >5% of the time
2. It adds >5 seconds to common operations
3. It hasn't caught a real issue in 30 days
4. It can be replaced with discipline

## Emergency Bypass

If hooks are blocking critical work:

```bash
# Bypass pre-commit hook (use sparingly)
git commit --no-verify -m "Emergency fix"

# Disable all hooks temporarily
mv .git/hooks .git/hooks.disabled
# Do work
mv .git/hooks.disabled .git/hooks
```

## Future Considerations

### Potentially Valuable Hooks (Not Yet Implemented)
1. **Large file prevention** - Block commits >10MB
2. **Secrets scanner** - Prevent API key commits
3. **Breaking change detector** - Warn on API changes

### Hooks to Never Implement
1. **Auto-PR creation** - Removes human judgment
2. **Auto-merge** - Too dangerous
3. **Force push prevention** - Sometimes necessary
4. **Commit message formatting** - Over-engineering

## Summary

**Current state**: 1 critical hook (pre-commit tests)
**Philosophy**: Brutal minimalism - maximum value, minimum complexity
**Result**: Fast, reliable, no false positives

Remember: Excellence comes from thoughtful development, not aggressive automation.