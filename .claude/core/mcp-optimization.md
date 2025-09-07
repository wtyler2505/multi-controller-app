# MCP Server Optimization Guide

## Architecture Overview (Post-Aggregator Migration)
**Date**: 2025-01-06  
**Configuration**: 10 independent MCP servers (direct connections)  
**Philosophy**: Stability through independence, excellence through proper tool selection

## Server-Specific Optimization

### 1. cipher-memory (Core Memory System)
**Purpose**: Primary knowledge storage and retrieval  
**Optimal Usage**:
```javascript
// ALWAYS start with search to avoid duplicates
mcp__cipher-memory__search_nodes({query: "transport patterns"})
// Store only after verification
mcp__cipher-memory__create_entities([{
  name: "Pattern Name",
  entityType: "solution",
  observations: ["complete", "context", "here"]
}])
```
**Avoid**: Using for temporary data or session-specific information

### 2. taskmaster-ai (Task Orchestration)
**Purpose**: Task management and complexity analysis  
**Optimal Usage**:
```javascript
mcp__taskmaster-ai__next_task()  // Start of session
mcp__taskmaster-ai__get_task({id: "9.3"})  // Specific task
mcp__taskmaster-ai__update_subtask({  // Progress tracking
  id: "9.3.1",
  prompt: "Implementation complete with ring buffer"
})
```
**Avoid**: Manual task.json edits, using for non-task notes

### 3. desktop-commander (File Operations)
**Purpose**: File I/O and terminal operations  
**Optimal Usage**:
```javascript
// Use for file operations instead of Bash
mcp__desktop-commander__read_file({path: "src/main.rs"})
mcp__desktop-commander__edit_block({  // Surgical edits
  file_path: "src/main.rs",
  old_string: "exact match",
  new_string: "replacement"
})
```
**Avoid**: Using Bash for cat, grep, or file operations

### 4. FileScopeMCP (Code Intelligence)
**Purpose**: Code analysis and dependency mapping  
**Optimal Usage**:
```javascript
mcp__FileScopeMCP__find_important_files({minImportance: 7})
mcp__FileScopeMCP__generate_diagram({style: "dependency"})
```
**Avoid**: Using for simple file searches (use Glob instead)

### 5. clear-thought (Deep Reasoning)
**Purpose**: 38 structured reasoning operations  
**Optimal Usage**:
```javascript
// For complex problems requiring deep analysis
mcp__clear-thought__sequentialthinking({
  prompt: "Design reconnection strategy",
  parameters: {pattern: "tree", depth: 5}
})
// For debugging critical issues
mcp__clear-thought__debuggingapproach({
  prompt: "Serial timeout after 30s",
  parameters: {approach: "hypothesis_testing"}
})
```
**Avoid**: Using for simple decisions or quick answers

### 6. context7 (Documentation)
**Purpose**: Official library documentation  
**Optimal Usage**:
```javascript
mcp__context7__resolve-library-id({libraryName: "tokio"})
mcp__context7__get-library-docs({
  context7CompatibleLibraryID: "/tokio-rs/tokio",
  topic: "async runtime"
})
```
**Avoid**: Using for general web searches

### 7. perplexity-ask (Web Research)
**Purpose**: Real-time web information  
**Optimal Usage**:
```javascript
mcp__perplexity-ask__perplexity_ask({
  messages: [{
    role: "user",
    content: "Latest Rust async best practices 2025"
  }]
})
```
**Avoid**: Using for static documentation (use context7)

### 8. memory (Supplementary Storage)
**Purpose**: Additional memory beyond cipher  
**Optimal Usage**: For session-specific or temporary knowledge graphs
**Avoid**: Duplicating cipher-memory storage

### 9. time-server (Scheduling)
**Purpose**: Time operations and scheduling  
**Optimal Usage**: Timezone conversions, scheduling tasks
**Avoid**: Using for simple timestamps (use Date())

### 10. byterover-mcp (Project Tools)
**Purpose**: ByteRover-specific utilities  
**Optimal Usage**: Project-specific operations and templates
**Avoid**: Using for general-purpose operations

## Optimization Patterns

### Pattern 1: Memory-First Development
```javascript
// ALWAYS start with memory search
1. mcp__cipher-memory__search_nodes({query: "relevant pattern"})
2. Analyze existing patterns
3. Implement with context
4. Store new learnings
```

### Pattern 2: Task-Driven Workflow
```javascript
// Structure work around tasks
1. mcp__taskmaster-ai__next_task()
2. mcp__clear-thought__sequentialthinking() // Plan approach
3. Implementation with appropriate tools
4. mcp__taskmaster-ai__set_task_status({status: "done"})
```

### Pattern 3: Progressive Tool Escalation
```javascript
// Start simple, escalate as needed
Simple: Read → Grep → Glob
Medium: FileScopeMCP → desktop-commander
Complex: clear-thought → cipher-memory → perplexity-ask
```

### Pattern 4: Parallel Research
```javascript
// Use multiple research tools simultaneously
await Promise.all([
  mcp__context7__get-library-docs(),
  mcp__perplexity-ask__perplexity_ask(),
  mcp__cipher-memory__search_nodes()
])
```

## Performance Considerations

### Memory Management
- Each MCP server consumes ~20-50MB RAM
- Total footprint: ~300-500MB (exceeds 150MB budget)
- **Mitigation**: Selective server activation based on task

### Connection Management
- Initial connection: ~2-5s per server
- Reconnection on timeout: Automatic
- **Best Practice**: Keep servers warm with periodic usage

### Timeout Configuration
```json
{
  "cipher-memory": 60000,      // Complex operations
  "taskmaster-ai": 60000,       // AI operations
  "desktop-commander": 45000,   // File operations
  "clear-thought": 45000,       // Reasoning
  "context7": 60000,           // Documentation fetch
  "perplexity-ask": 60000,     // Web research
  "memory": 60000,             // Graph operations
  "time-server": 15000,        // Quick operations
  "byterover-mcp": 45000,      // Varies by operation
  "FileScopeMCP": 60000        // Code analysis
}
```

## Troubleshooting Guide

### Issue: MCP Server Not Responding
```bash
# Check connection
/mcp  # Shows connection status

# Force reconnect
Restart Claude Code

# Verify server installation
node C:\\path\\to\\server\\index.js --version
```

### Issue: Tool Not Found
```bash
# Verify server provides tool
/mcp  # List available tools

# Check tool name format
mcp__servername__toolname  # Correct format
```

### Issue: Timeout Errors
```javascript
// Increase timeout in .mcp.json
"timeout": 120000  // 2 minutes for slow operations
```

### Issue: Memory Server Conflicts
```javascript
// Use distinct purposes
cipher-memory: Long-term patterns and knowledge
memory: Session-specific graphs
filesystem: Temporary data
```

## Best Practices

### 1. Tool Selection Hierarchy
Always prefer MCP tools over built-in tools when available:
- `mcp__desktop-commander__read_file` > `Read`
- `mcp__FileScopeMCP__*` > `Grep` for code analysis
- `mcp__taskmaster-ai__*` > manual task tracking

### 2. Batch Operations
Combine related operations to reduce overhead:
```javascript
// Good: Batch related operations
const results = await Promise.all([
  mcp__cipher-memory__search_nodes({query: "pattern1"}),
  mcp__cipher-memory__search_nodes({query: "pattern2"})
])

// Avoid: Sequential when parallel is possible
```

### 3. Context Preservation
Store important discoveries immediately:
```javascript
// After solving a problem
mcp__cipher-memory__create_entities([{
  name: "Solution: " + problem,
  entityType: "solution",
  observations: [/* complete context */]
}])
```

### 4. Server Health Monitoring
Regular health checks:
```bash
/mcp  # Start of each session
# After any connection issues
# When switching between heavy operations
```

## Migration Notes (From Aggregator)

### What Changed
- **Before**: Single aggregator managing all servers
- **After**: Direct connections to each server
- **Impact**: More stable, slightly higher memory usage

### Tool Name Changes
- `cipher_*` → `mcp__cipher-memory__*`
- `taskmaster_*` → `mcp__taskmaster-ai__*`
- Direct tool access without aggregation layer

### Configuration Differences
- Removed: `MCP_SERVER_MODE: "aggregator"`
- Added: Individual server configurations
- Each server has independent timeout and environment

---
*Excellence through proper tool selection and optimal usage patterns*