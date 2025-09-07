# MCP Servers Configuration Guide

## Overview

The Multi-Controller App uses multiple MCP (Model Context Protocol) servers to provide AI-powered development assistance. Each server specializes in different aspects of the development workflow.

## Core MCP Servers

### 1. Task Master AI
**Purpose**: Task management and workflow orchestration
**Configuration**: `.mcp.json`
```json
"taskmaster-ai": {
  "type": "stdio",
  "command": "node",
  "args": ["C:\\Users\\wtyle\\node_modules\\task-master-ai\\mcp-server\\server.js"],
  "env": {
    "ANTHROPIC_API_KEY": "${ANTHROPIC_API_KEY}",
    "PERPLEXITY_API_KEY": "${PERPLEXITY_API_KEY}",
    "OPENAI_API_KEY": "${OPENAI_API_KEY}"
  }
}
```

### 2. Desktop Commander
**Purpose**: File operations and terminal management
**Features**:
- Safe file I/O operations
- Terminal command execution
- Process management
- File search and analysis

### 3. FileScope
**Purpose**: Codebase analysis and dependency mapping
**Features**:
- File importance ranking
- Dependency graph generation
- Code structure visualization
- Impact analysis

### 4. Clear Thought
**Purpose**: Structured reasoning and decision-making
**Tools**:
- Mental models
- Debugging approaches
- Decision frameworks
- Scientific method

### 5. Context7
**Purpose**: Official documentation retrieval
**Usage**: Fetch version-accurate API docs and examples

### 6. Perplexity Ask
**Purpose**: Web research and community insights
**Usage**: Real-time information gathering

### 7. Memory
**Purpose**: Long-term knowledge persistence
**Usage**: Store project conventions and decisions

### 8. Time Server
**Purpose**: Timezone and scheduling utilities
**Usage**: Time-based operations and logging

## Configuration Files

### `.mcp.json` (Project Root)
Main MCP server configuration for the project.

### `.claude/settings.json`
Claude Code specific settings and tool permissions.

### `.claude/settings.local.json`
Local overrides for development environment.

## Environment Variables Required

```bash
# Core AI Services
ANTHROPIC_API_KEY=your_key_here
PERPLEXITY_API_KEY=your_key_here

# Optional AI Services
OPENAI_API_KEY=your_key_here
GOOGLE_API_KEY=your_key_here
MISTRAL_API_KEY=your_key_here
XAI_API_KEY=your_key_here
OPENROUTER_API_KEY=your_key_here
```

## MCP Server Usage Patterns

### Task Management
```bash
# Via MCP
mcp__taskmaster-ai__get_tasks
mcp__taskmaster-ai__next_task
mcp__taskmaster-ai__set_task_status

# Via CLI
task-master list
task-master next
task-master set-status --id=1 --status=done
```

### File Operations
```bash
# Safe file operations
mcp__desktop-commander__read_file
mcp__desktop-commander__write_file
mcp__desktop-commander__create_directory
```

### Code Analysis
```bash
# Analyze codebase
mcp__FileScopeMCP__create_file_tree
mcp__FileScopeMCP__find_important_files
mcp__FileScopeMCP__generate_diagram
```

### Documentation Retrieval
```bash
# Get official docs
mcp__context7__resolve-library-id
mcp__context7__get-library-docs
```

### Research
```bash
# Web research
mcp__perplexity-ask__perplexity_ask
```

## Best Practices

### 1. Tool Selection Priority
When multiple tools can accomplish a task:
1. Use MCP tools over manual operations
2. Prefer specialized servers for their domains
3. Combine servers for comprehensive solutions

### 2. Error Handling
- Check server connectivity before operations
- Handle timeout gracefully
- Provide fallback options

### 3. Performance Considerations
- Batch operations when possible
- Cache frequently accessed data
- Use appropriate timeout values

### 4. Security
- Never hardcode API keys
- Use environment variable references
- Validate server responses
- Audit tool usage

## Troubleshooting

### Server Not Responding
1. Check `.mcp.json` configuration
2. Verify API keys in environment
3. Test with `claude mcp list`
4. Check server logs

### Authentication Failures
1. Verify API key validity
2. Check environment variable names
3. Ensure proper quoting in config

### Performance Issues
1. Monitor server resource usage
2. Check network connectivity
3. Review operation complexity
4. Consider caching strategies

## Server-Specific Commands

### Initialize Servers
```bash
# Install MCP servers
npm install -g task-master-ai
npm install -g @wonderwhy-er/desktop-commander
npm install -g @upstash/context7-mcp
```

### Test Connectivity
```bash
# List available servers
claude mcp list

# Test specific server
claude mcp test taskmaster-ai
```

### Debug Mode
```bash
# Start Claude Code with MCP debugging
claude --mcp-debug
```