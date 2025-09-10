# CLAUDE.md Optimization Reference

## Overview
This document details the optimization journey of CLAUDE.md from 44.9k to 7.4k characters, achieving an 83% reduction while preserving critical functionality.

## Optimization Timeline

### Phase 1: Initial State (44,870 characters)
**Problem**: File exceeded 40k recommended limit, causing:
- Slow Claude response times
- Excessive context consumption
- Performance degradation

**Content**: Verbose imports of all MCP server documentation with installation instructions

### Phase 2: Aggressive Optimization (5,652 characters)
**Approach**: Removed all redundancy and verbose sections
**Result**: 87.4% reduction
**Issue**: Lost critical context that caused user frustration

### Phase 3: Balanced Enhancement (7,422 characters)
**Approach**: Strategic restoration of essential elements
**Result**: Optimal balance between performance and completeness

## Critical Elements Restored

### 1. User Preferences (Top Priority)
```markdown
## USER PREFERENCES (READ FIRST)
- **Communication**: Direct, no fluff, no over-explanation
- **Style**: Be concise - "stop talking so i can run /compact"
- **Verification**: Always show proof with grep, never just claim
```

### 2. Agent Registration Warning
**Critical Issue Resolved**: cipher-orchestrator agent not recognized
```markdown
⚠️ **New agents in `.claude/agents/` require Claude Code restart to register**
```

### 3. Cipher Integration Details
**Session-Critical Knowledge**:
- Ollama embeddings with nomic-embed-text model
- 512MB memory limit
- Executable path: `dist/src/app/index.cjs`

### 4. Concrete Verification Protocol
```markdown
1. **Grep verify**: `grep -n "specific_change" file.ext`
2. **Diff check**: `git diff file.ext`
3. **Test execution**: `npm test` or run command
4. **NEVER say "I've implemented" without showing grep proof**
```

## Performance Metrics

| Version | Size (chars) | Size (KB) | Lines | Performance Impact |
|---------|-------------|-----------|-------|-------------------|
| Original | 44,870 | 44.9 | 1,174 | Severe degradation |
| Minimal | 5,652 | 5.7 | 154 | Optimal but incomplete |
| Enhanced | 7,422 | 7.4 | 184 | Optimal and complete |

## Removed Elements

### Successfully Eliminated
- Duplicate MCP installation instructions (8k chars)
- Verbose agent matrix tables (4k chars)
- Redundant verification protocols (3k chars)
- Outdated technology evaluation matrix (2k chars)
- Repetitive enforcement sections (5k chars)

### Preserved in Streamlined Form
- Agent domains (list instead of table)
- MCP servers (reference list instead of full imports)
- Verification steps (concrete commands instead of prose)

## Key Learnings

### What Must Stay
1. **User communication preferences** - Prevents frustration
2. **Agent restart requirements** - Critical for new agents
3. **Concrete command examples** - No ambiguity
4. **Cipher/Ollama configuration** - Session-specific setup
5. **File:line references** - Critical for navigation

### What Can Go
1. Installation instructions (available in docs)
2. Verbose explanations (user prefers concise)
3. Duplicate rules (consolidate once)
4. Historical decisions (keep in decision log)
5. Example code blocks (unless essential)

## Integration with Other Systems

### Cipher Memory Framework
- Acts as MCP aggregator hub
- Manages 8 servers, 106 tools total
- Uses Ollama for local embeddings

### Task Master
- Tracks optimization as completed task
- Progress: 17.39% → 17.93% with subtasks

### Ruler (Pending Task 24)
- Prepared section for migration
- Will extract reusable patterns
- Single source of truth for all AI agents

## Maintenance Guidelines

### When to Update CLAUDE.md
- New critical patterns discovered
- User preferences change
- Major architecture decisions
- New tool integrations

### Size Budget
- Target: 8-10k characters
- Hard limit: 40k characters
- Sweet spot: 7-8k for optimal performance

### Review Triggers
- Every 10 completed tasks
- Major milestone completions
- User frustration incidents
- Performance degradation

## Related Documentation
- `/ref/MCP_SERVERS.md` - Detailed MCP configuration
- `/ref/AGENTS.md` - Complete agent reference
- `/docs/decisions/decision-log.md` - Optimization decisions
- `/.cipher/cipher.yml` - Cipher configuration