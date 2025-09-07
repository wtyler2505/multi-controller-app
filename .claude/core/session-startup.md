# 🚀 Claude Code Session Startup Checklist

## Pre-Flight Checks (Every Session)

### 1. MCP Server Health Check
```bash
/mcp  # Should show all 10 servers connected
```

Expected Output:
- ✅ cipher-memory
- ✅ taskmaster-ai  
- ✅ desktop-commander
- ✅ FileScopeMCP
- ✅ clear-thought
- ✅ context7
- ✅ perplexity-ask
- ✅ memory
- ✅ time-server
- ✅ byterover-mcp

### 2. Slash Command Verification
```bash
/help  # Should list 43+ commands
```

Key Commands to Verify:
- `/ultra-think` - Deep analysis
- `/project-reflection` - Project analysis
- `/cipher-memory` - Memory operations
- `/tm-*` - TaskMaster commands

### 3. Load Current Context
```javascript
// Get current task
await mcp__taskmaster-ai__next_task()

// Load relevant patterns
await mcp__cipher-memory__search_nodes({
  query: "current work area"
})

// Check project status
await mcp__taskmaster-ai__get_tasks({
  status: "in-progress"
})
```

## 🔧 If Servers Don't Connect

### Quick Fix Sequence
1. **Restart Claude Code** (usually fixes 90% of issues)
2. **Check `.mcp.json`** for syntax errors
3. **Verify server installations**:
```bash
node C:\Users\wtyle\node_modules\task-master-ai\mcp-server\server.js --version
```

### Individual Server Tests
```javascript
// Test each server with simple command
await mcp__cipher-memory__search_nodes({query: "test"})
await mcp__taskmaster-ai__get_tasks()
await mcp__desktop-commander__list_directory({path: "."})
await mcp__clear-thought__sessioninfo()
```

## 📊 Performance Baseline

### Memory Check
```powershell
# Check Claude Code memory usage
Get-Process "Cursor" | Select-Object Name, @{n='RAM(MB)';e={$_.WorkingSet/1MB}}
```

Target: < 500MB total for all MCP servers

### Response Time Test
```javascript
// Time a simple operation
console.time("mcp-test")
await mcp__cipher-memory__search_nodes({query: "test"})
console.timeEnd("mcp-test")
```

Expected: < 2 seconds

## 🎯 Session Optimization

### High-Performance Setup
```javascript
// Pre-warm critical servers
await Promise.all([
  mcp__cipher-memory__read_graph(),
  mcp__taskmaster-ai__get_tasks(),
  mcp__FileScopeMCP__list_files()
])
```

### Task-Specific Loading
```javascript
// For debugging work
await mcp__clear-thought__debuggingapproach({
  prompt: "Load debugging context"
})

// For implementation
await mcp__FileScopeMCP__find_important_files({
  minImportance: 7
})

// For research
await mcp__perplexity-ask__perplexity_ask({
  messages: [{role: "user", content: "Latest best practices"}]
})
```

## 🚨 Emergency Procedures

### If Everything Fails
1. **Backup current work**
```bash
git add . && git commit -m "WIP: MCP issues"
```

2. **Restore aggregator** (last resort)
```bash
cp .mcp.json.backup-* .mcp.json
```

3. **Minimal mode** (essential servers only)
```json
{
  "mcpServers": {
    "taskmaster-ai": {...},
    "desktop-commander": {...}
  }
}
```

## 📈 Session Metrics to Track

### At Start
- [ ] All 10 servers connected?
- [ ] Slash commands recognized?
- [ ] Current task loaded?
- [ ] Memory baseline recorded?

### During Work
- [ ] Tool response times normal?
- [ ] Memory usage stable?
- [ ] No timeout errors?
- [ ] Patterns being stored?

### At End
- [ ] Patterns stored in Cipher?
- [ ] Tasks updated in TaskMaster?
- [ ] Work committed to git?
- [ ] Session metrics logged?

## 🎪 Daily Maintenance

### Morning Routine
1. Check for MCP server updates
2. Review yesterday's patterns
3. Plan today's tasks
4. Warm up servers

### Evening Routine
1. Store learned patterns
2. Update task progress
3. Document blockers
4. Plan tomorrow

## 🔑 Key Performance Indicators

| Metric | Target | Action if Exceeded |
|--------|--------|-------------------|
| MCP Connection Time | < 5s | Restart Claude Code |
| Tool Response Time | < 2s | Check specific server |
| Total RAM Usage | < 500MB | Disable unused servers |
| Timeout Errors | 0 | Increase timeout in .mcp.json |

## 📝 Session Log Template

```markdown
## Session: [DATE]

### Startup
- MCP Servers: 10/10 connected
- Slash Commands: Working
- Current Task: #X.Y
- Memory Baseline: XXX MB

### Work Completed
- [List accomplishments]

### Patterns Stored
- [List new patterns]

### Issues Encountered
- [List any problems]

### Next Session
- [Priority items]
```

---
*Run through this checklist at the start of each session for optimal performance*