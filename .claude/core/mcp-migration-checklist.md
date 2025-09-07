# MCP Migration Checklist - From Aggregator to Direct Servers

## 📅 Migration Date: 2025-01-06
**Previous**: cipher-aggregator (single point, unstable)  
**Current**: 10 direct MCP server connections (stable, independent)

## ✅ Pre-Migration Checklist

### 1. Backup Current Configuration
- [x] Backup `.mcp.json` with timestamp
- [x] Document current tool names
- [x] Note any custom configurations
- [x] Save aggregator configuration (`.cipher/cipher.yml`)

### 2. Identify Required Servers
- [x] List all tools currently in use
- [x] Map tools to their source servers
- [x] Verify server installation paths
- [x] Check API key requirements

## 🔄 Migration Steps

### Step 1: Stop Aggregator
```bash
# Close Claude Code
# Remove aggregator from .mcp.json
```

### Step 2: Configure Individual Servers
```json
{
  "mcpServers": {
    "cipher-memory": { /* direct config */ },
    "taskmaster-ai": { /* direct config */ },
    "desktop-commander": { /* direct config */ },
    "FileScopeMCP": { /* direct config */ },
    "clear-thought": { /* direct config */ },
    // ... additional servers
  }
}
```

### Step 3: Update Tool Names
| Old Format | New Format |
|------------|------------|
| `cipher_memory_search()` | `mcp__cipher-memory__search_nodes()` |
| `cipher_store_entities()` | `mcp__cipher-memory__create_entities()` |
| `taskmaster_get_task()` | `mcp__taskmaster-ai__get_task()` |
| `clear_thought()` | `mcp__clear-thought__sequentialthinking()` |

### Step 4: Restart and Verify
- [x] Restart Claude Code
- [x] Run `/mcp` to check connections
- [x] Test each server with a simple command
- [x] Verify slash commands are recognized

## 📝 Post-Migration Tasks

### Documentation Updates
- [x] Update `CLAUDE.md` with new architecture
- [x] Update `.claude/core/memory-mastery.md`
- [x] Create `.claude/core/mcp-optimization.md`
- [x] Update all referenced import files
- [x] Document troubleshooting procedures

### Testing Verification
- [x] Test cipher-memory operations
- [x] Test taskmaster-ai functions
- [x] Test desktop-commander file ops
- [x] Test clear-thought reasoning
- [x] Verify slash commands work

### Performance Validation
- [ ] Monitor memory usage (target: <500MB total)
- [ ] Check connection stability over time
- [ ] Verify timeout configurations
- [ ] Test parallel operations

## 🚨 Common Issues & Solutions

### Issue 1: "Tool not found"
**Cause**: Tool name format changed  
**Solution**: Use `mcp__servername__toolname` format

### Issue 2: Slash commands missing
**Cause**: Claude Code needs to rescan  
**Solution**: Restart Claude Code

### Issue 3: Server timeout
**Cause**: Default timeout too short  
**Solution**: Increase timeout in `.mcp.json`

### Issue 4: Connection failures
**Cause**: Server not installed or wrong path  
**Solution**: Verify installation with `node path/to/server --version`

### Issue 5: Memory conflicts
**Cause**: Multiple memory servers  
**Solution**: Use cipher-memory for patterns, memory for graphs

## 🔧 Rollback Plan

If migration fails:
1. Restore backup: `cp .mcp.json.backup-[timestamp] .mcp.json`
2. Restart Claude Code
3. Verify aggregator reconnects
4. Document failure reason for debugging

## 📊 Success Metrics

### Immediate (Day 1)
- ✅ All servers connect successfully
- ✅ Core tools functional
- ✅ Slash commands recognized
- ✅ No connection timeouts

### Short-term (Week 1)
- [ ] Zero aggregator-related failures
- [ ] Improved connection stability
- [ ] Faster tool response times
- [ ] Reduced memory overhead

### Long-term (Month 1)
- [ ] No regression to aggregator needed
- [ ] All team members migrated
- [ ] Documentation fully updated
- [ ] Performance metrics improved

## 🎯 Key Benefits Achieved

1. **Stability**: No single point of failure
2. **Debugging**: Easier to identify which server fails
3. **Performance**: Parallel operations possible
4. **Flexibility**: Can disable/enable individual servers
5. **Maintenance**: Update servers independently

## 📚 References

- **MCP Specification**: https://modelcontextprotocol.org
- **Cipher Documentation**: `.cipher/MEMORY_PROTOCOL.md`
- **TaskMaster Guide**: `.taskmaster/CLAUDE.md`
- **Optimization Guide**: `.claude/core/mcp-optimization.md`

## 🔄 Future Improvements

1. **Selective Loading**: Load servers based on task type
2. **Connection Pool**: Manage server lifecycle dynamically
3. **Performance Monitoring**: Track per-server metrics
4. **Automated Health Checks**: Periodic connection validation
5. **Smart Fallbacks**: Use alternative servers on failure

---
*Migration completed successfully - all systems operational*