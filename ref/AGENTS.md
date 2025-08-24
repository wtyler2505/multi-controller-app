# Claude Code Agents Reference

## Overview

The project includes 16 specialized Claude Code agents designed to handle specific domains and tasks. These agents can be invoked via the Task tool to provide expert assistance in their respective areas.

## Agent Directory

All agents are located in `.claude/agents/` and follow a consistent structure with domain expertise, practical examples, and best practices.

## Available Agents

### 1. Agent Expert

**File**: `agent-expert.md`  
**Purpose**: Creates and designs specialized Claude Code agents  
**Expertise**:

- Agent architecture and design patterns
- Prompt engineering for specialized domains
- Domain expertise modeling
- Agent interaction patterns
- Best practices for agent development

**Use Cases**:

- Creating new specialized agents
- Improving existing agent prompts
- Designing multi-agent workflows
- Agent capability assessment

### 2. Docs Scribe

**File**: `docs-scribe.md`  
**Purpose**: Maintains project documentation in sync with codebase  
**Expertise**:

- CLAUDE.md maintenance
- Decision log updates
- PRD synchronization
- Architecture documentation
- README generation

**Use Cases**:

- Updating documentation after code changes
- Creating comprehensive project guides
- Maintaining decision history
- Generating API documentation

### 3. Driver Engineer

**File**: `driver-engineer.md`  
**Purpose**: Creates and maintains device driver plugins  
**Expertise**:

- Arduino/ESP32/RioRand driver development
- Serial/TCP/UDP protocol implementation
- Driver manifest creation
- Hardware abstraction layers
- Plugin architecture

**Use Cases**:

- Implementing new device drivers
- Debugging hardware communication
- Creating driver manifests
- Protocol optimization

### 4. MCP Toolsmith

**File**: `mcp-toolsmith.md`  
**Purpose**: Manages MCP server configurations and integrations  
**Expertise**:

- MCP server installation and configuration
- Tool verification and testing
- Config file management
- Per-server CLAUDE.md imports
- MCP debugging

**Use Cases**:

- Adding new MCP servers
- Fixing MCP configuration issues
- Optimizing MCP workflows
- Troubleshooting connections

### 5. Memory Steward

**File**: `memory-steward.md`  
**Purpose**: Curates long-lived facts and conventions  
**Expertise**:

- Knowledge graph management
- Fact curation and organization
- Convention tracking
- PII/secret avoidance
- Memory optimization

**Use Cases**:

- Storing project conventions
- Managing long-term knowledge
- Cleaning sensitive data
- Organizing project facts

### 6. Packaging Release

**File**: `packaging-release.md`  
**Purpose**: Handles application packaging and release  
**Expertise**:

- Single-file/AOT packaging
- Code signing
- Artifact verification
- Release notes generation
- Distribution preparation

**Use Cases**:

- Creating release builds
- Signing executables
- Generating changelogs
- Packaging for distribution

### 7. Performance Profiler

**File**: `performance-profiler.md`  
**Purpose**: Maintains performance budgets and optimization  
**Expertise**:

- Performance budget enforcement
- AOT build settings
- Memory profiling
- CPU usage analysis
- Startup time optimization

**Use Cases**:

- Profiling application performance
- Optimizing resource usage
- Meeting performance budgets
- AOT configuration

### 8. Research Librarian

**File**: `research-librarian.md`  
**Purpose**: Evidence gathering and competitive analysis  
**Expertise**:

- Technology research
- Community insights gathering
- Documentation triangulation
- Competitive scanning
- Best practices discovery

**Use Cases**:

- Researching technology choices
- Gathering community feedback
- Comparing solutions
- Finding best practices

### 9. Safety Guardian

**File**: `safety-guardian.md`  
**Purpose**: Ensures safety invariants and control paths  
**Expertise**:

- Safety invariant definition
- Control path testing
- Emergency stop mechanisms
- Rate limiting implementation
- Hardware protection

**Use Cases**:

- Implementing safety features
- Testing emergency controls
- Validating safety invariants
- Control path verification

### 10. Security Hygiene

**File**: `security-hygiene.md`  
**Purpose**: Manages secrets and security best practices  
**Expertise**:

- Secrets scrubbing
- Token handling
- Tool allowlist management
- Security audit
- Vulnerability scanning

**Use Cases**:

- Removing secrets from code
- Managing API keys
- Security auditing
- Access control setup

### 11. Task Checker

**File**: `task-checker.md`  
**Purpose**: Verifies task implementation quality  
**Expertise**:

- Implementation verification
- Requirement validation
- Test execution
- Best practices checking
- Quality assurance

**Use Cases**:

- Reviewing completed tasks
- Validating implementations
- Running verification tests
- Quality checks

### 12. Task Executor

**File**: `task-executor.md`  
**Purpose**: Implements and completes specific tasks  
**Expertise**:

- Task implementation
- Code generation
- Problem solving
- Integration work
- Feature development

**Use Cases**:

- Implementing identified tasks
- Completing subtasks
- Feature development
- Bug fixing

### 13. Task Orchestrator

**File**: `task-orchestrator.md`  
**Purpose**: Coordinates complex task execution  
**Expertise**:

- Task dependency analysis
- Parallel execution planning
- Resource allocation
- Workflow orchestration
- Progress monitoring

**Use Cases**:

- Managing complex features
- Coordinating parallel work
- Dependency resolution
- Workflow optimization

### 14. Test Runner

**File**: `test-runner.md`  
**Purpose**: Executes comprehensive test suites  
**Expertise**:

- Unit test execution
- Loopback testing
- Hardware-in-loop testing
- Soak test management
- Test report generation

**Use Cases**:

- Running test suites
- Generating test reports
- Flaky test triage
- Performance testing

### 15. Transport Engineer

**File**: `transport-engineer.md`  
**Purpose**: Implements communication transports  
**Expertise**:

- Serial/TCP/UDP/SSH protocols
- Reconnection strategies
- Backoff algorithms
- Latency optimization
- Connection pooling

**Use Cases**:

- Implementing transports
- Optimizing communication
- Debugging connections
- Protocol development

### 16. UI Telemetry Analyst

**File**: `ui-telemetry-analyst.md`  
**Purpose**: Optimizes telemetry and visualization  
**Expertise**:

- Telemetry decimation
- Buffer optimization
- Chart rendering
- Data visualization
- Performance monitoring

**Use Cases**:

- Implementing telemetry
- Optimizing charts
- Data decimation
- UI performance

## Agent Invocation

### Using the Task Tool

```javascript
// Basic invocation
Task({
  subagent_type: 'driver-engineer',
  description: 'Implement Arduino driver',
  prompt: 'Create a driver for Arduino Uno with serial echo functionality',
});

// With detailed context
Task({
  subagent_type: 'performance-profiler',
  description: 'Profile memory usage',
  prompt:
    'Analyze current memory usage and suggest optimizations to stay under 150MB budget',
});
```

### Agent Selection Guidelines

1. **Choose by Domain**: Select agents based on their specialized domain
2. **Consider Dependencies**: Some agents work well in sequence
3. **Parallel Execution**: Launch multiple agents for independent tasks
4. **Result Integration**: Agents return detailed reports for integration

## Common Agent Workflows

### Feature Implementation Workflow

1. **task-orchestrator**: Analyze and plan the feature
2. **task-executor**: Implement individual components
3. **test-runner**: Execute tests
4. **task-checker**: Verify implementation
5. **docs-scribe**: Update documentation

### Performance Optimization Workflow

1. **performance-profiler**: Profile current performance
2. **ui-telemetry-analyst**: Analyze telemetry overhead
3. **transport-engineer**: Optimize communication
4. **packaging-release**: Create optimized build

### Security Review Workflow

1. **security-hygiene**: Scan for secrets
2. **safety-guardian**: Verify safety invariants
3. **test-runner**: Execute security tests
4. **docs-scribe**: Document security measures

## Agent Best Practices

### 1. Clear Task Definition

- Provide specific, actionable tasks
- Include relevant context and constraints
- Specify expected outputs

### 2. Sequential vs Parallel

- Use sequential for dependent tasks
- Launch parallel agents for independent work
- Coordinate results with task-orchestrator

### 3. Result Integration

- Agents provide detailed reports
- Extract actionable items from reports
- Update task status based on results

### 4. Error Handling

- Agents report blockers and issues
- Provide fallback strategies
- Document unresolved problems

## Agent Capabilities Matrix

| Agent                | Code Gen | Analysis | Testing | Docs | Config |
| -------------------- | -------- | -------- | ------- | ---- | ------ |
| agent-expert         | ✓        | ✓        | -       | ✓    | -      |
| docs-scribe          | -        | ✓        | -       | ✓    | -      |
| driver-engineer      | ✓        | ✓        | ✓       | ✓    | ✓      |
| mcp-toolsmith        | ✓        | ✓        | ✓       | -    | ✓      |
| memory-steward       | -        | ✓        | -       | ✓    | -      |
| packaging-release    | ✓        | ✓        | ✓       | ✓    | ✓      |
| performance-profiler | ✓        | ✓        | ✓       | ✓    | ✓      |
| research-librarian   | -        | ✓        | -       | ✓    | -      |
| safety-guardian      | ✓        | ✓        | ✓       | ✓    | -      |
| security-hygiene     | -        | ✓        | ✓       | ✓    | ✓      |
| task-checker         | -        | ✓        | ✓       | -    | -      |
| task-executor        | ✓        | ✓        | ✓       | -    | -      |
| task-orchestrator    | -        | ✓        | -       | -    | -      |
| test-runner          | ✓        | ✓        | ✓       | ✓    | -      |
| transport-engineer   | ✓        | ✓        | ✓       | ✓    | ✓      |
| ui-telemetry-analyst | ✓        | ✓        | ✓       | ✓    | ✓      |

## Integration with Task Master

Agents work seamlessly with Task Master:

- **task-orchestrator** analyzes task dependencies
- **task-executor** implements identified tasks
- **task-checker** verifies completed work
- Results update task status automatically

## Custom Agent Development

To create new agents:

1. Use **agent-expert** to design the agent
2. Follow the template structure in existing agents
3. Define clear expertise boundaries
4. Include practical examples
5. Test with various prompts
6. Document in this reference
