# MCP Servers Reference

This project utilizes multiple MCP (Model Context Protocol) servers to enhance development capabilities. Each server provides specialized functionality accessible through Claude Code.

## Active MCP Servers

### 1. Task Master AI

**Purpose**: Task management and workflow orchestration
**Config Location**: `.mcp.json`
**Key Features**:

- Parse PRD documents into structured tasks
- AI-powered task expansion and complexity analysis
- Dependency management and validation
- Progress tracking and status updates

**Essential Commands**:

```bash
# Via MCP tools
mcp__taskmaster-ai__get_tasks         # List all tasks
mcp__taskmaster-ai__next_task         # Get next available task
mcp__taskmaster-ai__set_task_status   # Update task status
mcp__taskmaster-ai__expand_task       # Break task into subtasks
```

### 2. Desktop Commander

**Purpose**: Safe file I/O and terminal operations
**Features**:

- File reading/writing with safety checks
- Terminal command execution
- Process management
- Search capabilities

**Key Tools**:

- `read_file`: Read files with line limits
- `write_file`: Write with chunking support
- `start_process`: Launch terminal processes
- `search_code`: Code search with ripgrep

### 3. FileScope

**Purpose**: Codebase analysis and dependency mapping
**Features**:

- File importance ranking
- Dependency graph generation
- Mermaid diagram creation
- File summaries

**Key Tools**:

- `create_file_tree`: Initialize project analysis
- `find_important_files`: Identify critical files
- `generate_diagram`: Create visual representations

### 4. Clear Thought

**Purpose**: Structured reasoning and decision-making
**Features**:

- Mental models (First Principles, Pareto, Occam's Razor)
- Debugging approaches
- Decision frameworks
- Scientific method scaffolding

**Use Cases**:

- Technology stack decisions
- Architecture planning
- Complex problem solving

### 5. Context7

**Purpose**: Official documentation retrieval
**Features**:

- Version-accurate library docs
- Code examples
- API references

**Usage Pattern**:

```
# In prompts
"use context7 - fetch System.IO.Ports docs for .NET 8"
```

### 6. Perplexity Ask

**Purpose**: Real-time web research
**Features**:

- Community insights
- Technology comparisons
- Best practices research

### 7. Memory

**Purpose**: Long-term fact storage
**Features**:

- Project conventions storage
- Decision history
- Configuration persistence

### 8. Time Server

**Purpose**: Timezone and scheduling utilities
**Features**:

- Current time retrieval
- Timezone conversions
- Timestamp generation

## MCP Configuration

### Main Configuration (`.mcp.json`)

```json
{
  "mcpServers": {
    "task-master-ai": {
      "command": "npx",
      "args": ["-y", "--package=task-master-ai", "task-master-ai"],
      "env": {
        "ANTHROPIC_API_KEY": "...",
        "PERPLEXITY_API_KEY": "..."
      }
    }
    // Additional servers...
  }
}
```

### Claude Code Settings (`.claude/settings.json`)

```json
{
  "allowedTools": [
    "Edit",
    "Bash(task-master *)",
    "mcp__taskmaster-ai__*",
    "mcp__desktop-commander__*",
    "mcp__FileScopeMCP__*"
  ]
}
```

## MCP Server Workflows

### Task Management Workflow

1. Parse PRD: `mcp__taskmaster-ai__parse_prd`
2. Analyze complexity: `mcp__taskmaster-ai__analyze_project_complexity`
3. Expand tasks: `mcp__taskmaster-ai__expand_all`
4. Get next task: `mcp__taskmaster-ai__next_task`
5. Update progress: `mcp__taskmaster-ai__set_task_status`

### Code Analysis Workflow

1. Create file tree: `mcp__FileScopeMCP__create_file_tree`
2. Find important files: `mcp__FileScopeMCP__find_important_files`
3. Generate diagram: `mcp__FileScopeMCP__generate_diagram`
4. Set summaries: `mcp__FileScopeMCP__set_file_summary`

### Research Workflow

1. Official docs: Use Context7 for library references
2. Community insights: Use Perplexity Ask for best practices
3. Store decisions: Use Memory server for persistence
4. Document in decision log

## Integration with CLAUDE.md

The root `CLAUDE.md` file imports all MCP server configurations:

```markdown
## Imports

@./.taskmaster/CLAUDE.md
@./.desktop-commander/CLAUDE.md
@./.filescope/CLAUDE.md
@./.clear-thought/CLAUDE.md
@./.context7/CLAUDE.md
@./.perplexity-ask/CLAUDE.md
@./.memory/CLAUDE.md
@./.time-server/CLAUDE.md
```

Priority hierarchy when domains overlap:
TaskMaster ▶ Context7 ▶ FileScope ▶ Desktop-Commander ▶ Perplexity-Ask ▶ Memory ▶ Time-Server

## Best Practices

1. **Use MCP tools over manual approaches**: When a tool exists, use it
2. **Batch operations**: Call multiple tools in parallel when possible
3. **Research before implementation**: Use Context7 and Perplexity Ask
4. **Track progress**: Always update task status via Task Master
5. **Document decisions**: Store in Memory server and decision log
6. **Safety first**: Use Desktop Commander for file operations

## Troubleshooting

### MCP Connection Issues

- Check `.mcp.json` configuration
- Verify API keys in environment
- Use `--mcp-debug` flag with Claude Code
- Check Node.js installation

### Task Master Issues

- Never manually edit `tasks.json`
- Use `mcp__taskmaster-ai__fix_dependencies` for dependency issues
- Run `mcp__taskmaster-ai__generate` to regenerate task files

### FileScope Issues

- Recreate file tree if stale
- Use `recalculate_importance` after major changes
- Check file watching status if updates aren't detected
