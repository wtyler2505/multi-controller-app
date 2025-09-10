# Claude Code Agents Reference

## Overview

The Multi-Controller App includes 16 specialized Claude Code agents to assist with various development tasks. Each agent has specific expertise and tool access optimized for their domain.

## MANDATORY: Verification Requirements for All Agents

### Anti-Bullshit Measures

All agents MUST follow the Verification-First Development principles from CLAUDE.md:

1. **Never claim implementation without proof** - Use grep/diff to verify changes
2. **Follow Task Management Protocol** - Check TaskMaster before starting work
3. **Enforce File Management Rules** - Never create files unless explicitly requested
4. **Validate Performance Budgets** - Always check against limits
5. **Use Code References Format** - Always use file:line format

### Agent Self-Verification Protocol

Before any agent claims task completion:
- Run verification commands to prove changes exist
- Count lines to confirm additions/modifications
- Test any code or commands provided
- Show evidence of implementation

Agents that violate these rules must:
1. Admit the failure explicitly
2. Show exact verification commands
3. Implement immediately or explain why not

## Core Development Agents

### 1. Task Orchestrator
**Purpose**: Coordinate and manage Task Master task execution
**Use When**: Starting work sessions, analyzing task queue, identifying parallel work
**Key Tools**: Task Master MCP tools, dependency analysis

### 2. Task Executor
**Purpose**: Implement and complete specific tasks
**Use When**: Actually implementing features, fixing bugs, writing code
**Key Tools**: Full development toolset

### 3. Task Checker
**Purpose**: Verify task implementation quality
**Use When**: Tasks marked as 'review' status
**Key Tools**: Testing, code analysis, quality checks

## Hardware & Transport Agents

### 4. Driver Engineer
**Purpose**: Create and maintain hardware device drivers
**Use When**: Implementing drivers for Arduino, ESP32, RioRand, Raspberry Pi
**Expertise**: IDeviceDriver interface, protocol implementation, hardware abstraction

### 5. Transport Engineer
**Purpose**: Implement transport layer protocols
**Use When**: Working on Serial, TCP, UDP, SSH communication
**Expertise**: Async I/O, reconnection logic, latency enforcement

### 6. Safety Guardian
**Purpose**: Implement safety-critical systems
**Use When**: Emergency stops, rate limiting, hardware safeguards
**Expertise**: Fail-safe patterns, invariant verification

## UI & Performance Agents

### 7. UI Telemetry Analyst
**Purpose**: Optimize telemetry visualization and real-time data
**Use When**: High-frequency data handling, chart performance, UI responsiveness
**Expertise**: Decimation algorithms, ring buffers, chart optimization

### 8. Performance Profiler
**Purpose**: Optimize application performance
**Use When**: Startup optimization, memory profiling, Native AOT configuration
**Expertise**: Performance measurement, profiling tools, optimization techniques

## Testing & Quality Agents

### 9. Test Runner
**Purpose**: Implement comprehensive testing strategies
**Use When**: Creating unit tests, integration tests, HIL testing
**Expertise**: Test patterns, soak testing, flaky test analysis

### 10. Security Hygiene
**Purpose**: Manage security and credentials
**Use When**: Credential scanning, security audits, gitignore configuration
**Expertise**: Security patterns, credential detection, vulnerability scanning

## Research & Documentation Agents

### 11. Research Librarian
**Purpose**: Conduct comprehensive research
**Use When**: Evaluating libraries, gathering community insights, competitive analysis
**Tools**: Perplexity Ask, Context7, evidence triangulation

### 12. Docs Scribe
**Purpose**: Maintain project documentation
**Use When**: Updating CLAUDE.md, decision logs, README files
**Expertise**: Documentation automation, cross-file consistency

## Infrastructure Agents

### 13. MCP Toolsmith
**Purpose**: Configure and troubleshoot MCP servers
**Use When**: Setting up MCP integrations, debugging connections
**Expertise**: MCP protocol, server configuration, tool verification

### 14. Memory Steward
**Purpose**: Manage long-term project memory
**Use When**: Storing conventions, architectural decisions, performance baselines
**Tools**: Memory MCP, knowledge graph curation

### 15. Packaging Release
**Purpose**: Prepare production releases
**Use When**: Creating distributions, AOT compilation, code signing
**Expertise**: Build automation, artifact verification, deployment

### 16. Agent Expert
**Purpose**: Create and improve other agents
**Use When**: Developing new agents, auditing existing agents
**Expertise**: Agent architecture, prompt engineering, quality assurance

## Agent Usage Patterns

### Automatic Agent Selection
Claude Code automatically selects appropriate agents based on task context:
```
User: "Implement serial reconnection"
→ Transport Engineer agent activated
```

### Manual Agent Invocation
Explicitly request specific agents:
```
User: "Use the safety-guardian agent to implement emergency stop"
```

### Multi-Agent Workflows
Complex tasks may involve multiple agents:
1. Task Orchestrator → identifies work
2. Driver Engineer → implements driver
3. Test Runner → creates tests
4. Task Checker → verifies implementation

## Agent Tool Access

### Universal Tools
All agents have access to:
- Read, Write, Edit files
- Bash commands
- Grep, Glob, LS
- TodoWrite

### Specialized Tool Access
Agents have additional tools based on their domain:
- **Transport Engineer**: Serial testing tools
- **UI Telemetry Analyst**: Chart profiling tools
- **Security Hygiene**: Credential scanners
- **Research Librarian**: Perplexity Ask, Context7

## Best Practices

### 1. Let Agents Specialize
Use agents for their intended purpose rather than general tasks

### 2. Chain Agents for Complex Work
Break complex tasks into agent-appropriate subtasks

### 3. Trust Agent Expertise
Agents have domain-specific knowledge and patterns

### 4. Provide Context
Give agents relevant task IDs, file paths, and requirements

## Agent Configuration

### Location
Agent definitions: `.claude/agents/`

### Structure
Each agent file includes:
- Name and purpose
- When to use
- Example scenarios
- Tool requirements
- Domain expertise

### Customization
Agents can be modified for project-specific needs:
1. Edit agent definition file
2. Adjust tool access
3. Add domain knowledge
4. Update examples

## Troubleshooting Agents

### Agent Not Activating
- Check task description for trigger keywords
- Manually specify agent in request
- Verify agent file exists

### Agent Errors
- Check tool permissions in settings.json
- Verify required MCP servers running
- Review agent output for issues

### Performance Issues
- Limit concurrent agent invocations
- Use appropriate agents for task size
- Monitor token usage