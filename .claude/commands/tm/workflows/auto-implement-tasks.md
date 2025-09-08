---
model: claude-sonnet-4-20250514
category: task-management
priority: medium
tags: ["task-management", "tasks"]
description: Command for auto-implement-tasks operations

# Agent and Tool Integration
assigned-agent: task-orchestrator
required-tools:
  - "mcp__taskmaster-ai__get_tasks"
  - "mcp__taskmaster-ai__next_task"
  - "mcp__taskmaster-ai__set_task_status"
  - "mcp__desktop-commander__start_process"
tool-chain: task-coordination
auto-deploy: true
parallel-execution: true
max-agent-count: 3

# Workflow Configuration
pre-execution:
  validate-tools: true
  load-context: true
  prepare-environment: true
post-execution:
  store-results: true
  update-tasks: true
  generate-report: true
---

Enhanced auto-implementation with intelligent code generation and testing.

Arguments: $ARGUMENTS

## Intelligent Auto-Implementation

Advanced implementation with context awareness and quality checks.

### 1. **Pre-Implementation Analysis**

Before starting:
- Analyze task complexity and requirements
- Check codebase patterns and conventions
- Identify similar completed tasks
- Assess test coverage needs
- Detect potential risks

### 2. **Smart Implementation Strategy**

Based on task type and context:

**Feature Tasks**
1. Research existing patterns
2. Design component architecture
3. Implement with tests
4. Integrate with system
5. Update documentation

**Bug Fix Tasks**
1. Reproduce issue
2. Identify root cause
3. Implement minimal fix
4. Add regression tests
5. Verify side effects

**Refactoring Tasks**
1. Analyze current structure
2. Plan incremental changes
3. Maintain test coverage
4. Refactor step-by-step
5. Verify behavior unchanged

### 3. **Code Intelligence**

**Pattern Recognition**
- Learn from existing code
- Follow team conventions
- Use preferred libraries
- Match style guidelines

**Test-Driven Approach**
- Write tests first when possible
- Ensure comprehensive coverage
- Include edge cases
- Performance considerations

### 4. **Progressive Implementation**

Step-by-step with validation:
```
Step 1/5: Setting up component structure âœ“
Step 2/5: Implementing core logic âœ“
Step 3/5: Adding error handling âš¡ (in progress)
Step 4/5: Writing tests â³
Step 5/5: Integration testing â³

Current: Adding try-catch blocks and validation...
```

### 5. **Quality Assurance**

Automated checks:
- Linting and formatting
- Test execution
- Type checking
- Dependency validation
- Performance analysis

### 6. **Smart Recovery**

If issues arise:
- Diagnostic analysis
- Suggestion generation
- Fallback strategies
- Manual intervention points
- Learning from failures

### 7. **Post-Implementation**

After completion:
- Generate PR description
- Update documentation
- Log lessons learned
- Suggest follow-up tasks
- Update task relationships

Result: High-quality, production-ready implementations.


