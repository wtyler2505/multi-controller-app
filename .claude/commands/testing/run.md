---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality"]
description: Execute comprehensive test suites
allowed-tools: Bash, Read, Write, LS, Task
argument-hint: [test-type] | --unit | --integration | --e2e | --all | --watch

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["test-execution", "quality-assurance", "test-automation"]
    complexity-factors: ["test-orchestration", "result-analysis", "performance-validation"]
    specialized-tools: ["test-runners", "result-analysis", "quality-validation"]
  preferred-agents:
    primary: "mock-test-orchestrator"
    secondary: "cargo-build-engineer"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "taskmaster-ai", "cipher-memory"]
    specialized-functions: ["test-execution", "quality-assurance"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "test-execution + quality-assurance + test-automation"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "testing-patterns + execution-knowledge"
    
    knowledge-preparation:
      - domain: "test-execution"
      - pattern-search: "testing-strategies + execution-patterns + quality-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["test-analysis", "execution-monitoring", "result-validation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "testing-strategies + execution-approaches + validation-decisions"
      - pattern-recognition: "test-execution-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["test-results", "execution-insights", "quality-techniques"]
      - knowledge-extraction: "testing-methodologies + execution-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["test-relationships", "execution-dependencies", "quality-connections"]
      - cross-reference: "related-testing-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "testing-knowledge + execution-patterns"
      - continuous-learning: "test-execution-optimization"

# Centralized Logging Integration
logging-integration:
  enabled: true
  log-file: ".claude/command-execution.jsonl"
  
  # Comprehensive Execution Logging
  log-level: "comprehensive"
  
  capture-points:
    - command-initiation
    - agent-selection-process
    - memory-operations
    - test-analysis
    - execution-monitoring
    - result-validation
    - performance-assessment
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "testing-run"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "test-execution-results + quality-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["testing-patterns", "execution-techniques", "quality-strategies"]
  learn-from: ["testing-prime", "generate-tests", "test-coverage"]
  contribute-to: "test-execution-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-test-environment
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-test-execution
    - continuous-memory-updates
    - real-time-result-analysis
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - testing-pattern-extraction
---

# Run Tests

Execute tests with the configured test-runner agent.

## Usage
```
/testing:run [test_target]
```

Where `test_target` can be:
- Empty (run all tests)
- Test file path
- Test pattern
- Test suite name

## Quick Check

```bash
# Check if testing is configured
test -f .claude/testing-config.md || echo "âŒ Testing not configured. Run /testing:prime first"
```

If test target provided, verify it exists:
```bash
# For file targets
test -f "$ARGUMENTS" || echo "âš ï¸ Test file not found: $ARGUMENTS"
```

## Instructions

### 1. Determine Test Command

Based on testing-config.md and target:
- No arguments â†’ Run full test suite from config
- File path â†’ Run specific test file
- Pattern â†’ Run tests matching pattern

### 2. Execute Tests

Use the test-runner agent from `.claude/agents/test-runner.md`:

```markdown
Execute tests for: $ARGUMENTS (or "all" if empty)

Requirements:
- Run with verbose output for debugging
- No mocks - use real services
- Capture full output including stack traces
- If test fails, check test structure before assuming code issue
```

### 3. Monitor Execution

- Show test progress
- Capture stdout and stderr
- Note execution time

### 4. Report Results

**Success:**
```
âœ… All tests passed ({count} tests in {time}s)
```

**Failure:**
```
âŒ Test failures: {failed_count} of {total_count}

{test_name} - {file}:{line}
  Error: {error_message}
  Likely: {test issue | code issue}
  Fix: {suggestion}

Run with more detail: /testing:run {specific_test}
```

**Mixed:**
```
Tests complete: {passed} passed, {failed} failed, {skipped} skipped

Failed:
- {test_1}: {brief_reason}
- {test_2}: {brief_reason}
```

### 5. Cleanup

```bash
# Kill any hanging test processes
pkill -f "jest|mocha|pytest" 2>/dev/null || true
```

## Error Handling

- Test command fails â†’ "âŒ Test execution failed: {error}. Check test framework is installed."
- Timeout â†’ Kill process and report: "âŒ Tests timed out after {time}s"
- No tests found â†’ "âŒ No tests found matching: $ARGUMENTS"

## Important Notes

- Always use test-runner agent for analysis
- No mocking - real services only
- Check test structure if failures occur
- Keep output focused on failures


