# 🚀 MCP Quick Reference Card (2025-01-06)

## Tool Name Migration Map

### Cipher Memory Operations
| OLD Format | NEW Format |
|------------|------------|
| `cipher_memory_search()` | `mcp__cipher-memory__search_nodes()` |
| `cipher_store_entities()` | `mcp__cipher-memory__create_entities()` |
| `cipher_search_reasoning_patterns()` | `mcp__cipher-memory__search_nodes()` |
| `cipher_store_reasoning_memory()` | `mcp__cipher-memory__create_entities()` |
| `create_entities()` | `mcp__cipher-memory__create_entities()` |
| `create_relations()` | `mcp__cipher-memory__create_relations()` |
| `add_observations()` | `mcp__cipher-memory__add_observations()` |
| `read_graph()` | `mcp__cipher-memory__read_graph()` |

### TaskMaster Operations
| OLD Format | NEW Format |
|------------|------------|
| `get_task()` | `mcp__taskmaster-ai__get_task()` |
| `get_tasks()` | `mcp__taskmaster-ai__get_tasks()` |
| `next_task()` | `mcp__taskmaster-ai__next_task()` |
| `set_task_status()` | `mcp__taskmaster-ai__set_task_status()` |
| `update_task()` | `mcp__taskmaster-ai__update_task()` |
| `update_subtask()` | `mcp__taskmaster-ai__update_subtask()` |
| `expand_task()` | `mcp__taskmaster-ai__expand_task()` |

### Clear-Thought Operations
| OLD Format | NEW Format |
|------------|------------|
| `clear_thought("sequential")` | `mcp__clear-thought__sequentialthinking()` |
| `clear_thought("mental_model")` | `mcp__clear-thought__mentalmodel()` |
| `clear_thought("debugging")` | `mcp__clear-thought__debuggingapproach()` |
| `clear_thought("decision")` | `mcp__clear-thought__decisionframework()` |
| `clear_thought("collaborative")` | `mcp__clear-thought__collaborativereasoning()` |
| `clear_thought("ulysses")` | `mcp__clear-thought__ulyssesprotocol()` |

### File Operations
| OLD Format | NEW Format |
|------------|------------|
| `Read` | `mcp__desktop-commander__read_file()` |
| `Edit` | `mcp__desktop-commander__edit_block()` |
| `Write` | `mcp__desktop-commander__write_file()` |
| `Bash` | `mcp__desktop-commander__start_process()` |

## 📝 Common Usage Patterns

### Memory Search & Store
```javascript
// Search first
await mcp__cipher-memory__search_nodes({query: "transport patterns"})

// Store if new
await mcp__cipher-memory__create_entities([{
  name: "Pattern Name",
  entityType: "solution",
  observations: ["details"]
}])
```

### Task Workflow
```javascript
// Start session
await mcp__taskmaster-ai__next_task()

// Work on task
await mcp__taskmaster-ai__get_task({id: "9.3"})

// Update progress
await mcp__taskmaster-ai__update_subtask({
  id: "9.3.1",
  prompt: "Implemented with notes"
})

// Complete
await mcp__taskmaster-ai__set_task_status({
  id: "9.3.1",
  status: "done"
})
```

### Deep Reasoning
```javascript
// Complex problem
await mcp__clear-thought__sequentialthinking({
  prompt: "Design approach",
  parameters: {pattern: "tree"}
})

// Debugging
await mcp__clear-thought__debuggingapproach({
  prompt: "Error analysis"
})
```

### File Operations
```javascript
// Read
await mcp__desktop-commander__read_file({
  path: "src/main.rs"
})

// Edit
await mcp__desktop-commander__edit_block({
  file_path: "src/main.rs",
  old_string: "exact",
  new_string: "replacement"
})
```

## 🔥 Hot Keys

### Session Start
1. `/mcp` - Check all connections
2. `mcp__taskmaster-ai__next_task()` - Get current task
3. `mcp__cipher-memory__search_nodes({query: "relevant"})` - Load context

### During Work
- `mcp__FileScopeMCP__find_important_files()` - Find key files
- `mcp__perplexity-ask__perplexity_ask()` - Research
- `mcp__context7__get-library-docs()` - Get docs

### Problem Solving
- `mcp__clear-thought__sequentialthinking()` - Step-by-step
- `mcp__clear-thought__debuggingapproach()` - Debug
- `mcp__clear-thought__collaborativereasoning()` - Multiple views

### Completion
- `mcp__taskmaster-ai__set_task_status()` - Mark done
- `mcp__cipher-memory__create_entities()` - Store patterns
- `mcp__cipher-memory__create_relations()` - Link knowledge

## 🎯 Server Quick Reference

| Server | Primary Tools | Use For |
|--------|--------------|---------|
| **cipher-memory** | search_nodes, create_entities | Knowledge storage |
| **taskmaster-ai** | get_task, next_task | Task management |
| **desktop-commander** | read_file, edit_block | File operations |
| **FileScopeMCP** | find_important_files | Code analysis |
| **clear-thought** | 38 operations | Deep reasoning |
| **context7** | get-library-docs | Documentation |
| **perplexity-ask** | perplexity_ask | Web research |
| **memory** | create_entities | Extra storage |
| **time-server** | get_current_time | Scheduling |
| **byterover-mcp** | Various | ByteRover tools |

## ⚡ Performance Tips

### Parallel Operations
```javascript
await Promise.all([
  mcp__cipher-memory__search_nodes(),
  mcp__context7__resolve-library-id(),
  mcp__perplexity-ask__perplexity_ask()
])
```

### Tool Selection Order
1. Try MCP tool first
2. Fall back to built-in
3. Use manual as last resort

### Memory Management
- Each server: ~20-50MB
- Total: ~300-500MB
- Monitor with: `/mcp`

## 🚨 Troubleshooting

| Issue | Solution |
|-------|----------|
| Tool not found | Check format: `mcp__server__tool` |
| Timeout | Increase in `.mcp.json` |
| No connection | Restart Claude Code |
| Slash commands missing | Restart to rescan |

---
*Keep this card handy - Print or pin for quick reference*