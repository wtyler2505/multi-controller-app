---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality"]
description: Prime Testing Environment
allowed-tools: Bash, Read, Write, LS
argument-hint: [framework-type] | --auto-detect | --manual | --validate

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["test-environment-setup", "framework-detection", "test-configuration"]
    complexity-factors: ["framework-analysis", "environment-validation", "test-orchestration"]
    specialized-tools: ["test-framework-tools", "environment-setup", "configuration-management"]
  preferred-agents:
    primary: "mock-test-orchestrator"
    secondary: "cargo-build-engineer"
    fallback: ["general-purpose"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "taskmaster-ai", "cipher-memory"]
    specialized-functions: ["test-setup", "framework-detection"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "test-environment-setup + framework-detection + test-configuration"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "testing-patterns + setup-knowledge"
    
    knowledge-preparation:
      - domain: "test-environment-setup"
      - pattern-search: "setup-strategies + configuration-patterns + testing-techniques"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["framework-detection", "environment-setup", "configuration-validation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "setup-strategies + configuration-approaches + testing-decisions"
      - pattern-recognition: "test-setup-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["setup-results", "configuration-insights", "testing-techniques"]
      - knowledge-extraction: "setup-methodologies + configuration-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["setup-relationships", "configuration-dependencies", "testing-connections"]
      - cross-reference: "related-testing-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "setup-knowledge + configuration-patterns"
      - continuous-learning: "test-setup-optimization"

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
    - framework-detection
    - environment-setup
    - configuration-validation
    - dependency-checks
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "testing-prime"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "setup-results + configuration-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["setup-patterns", "configuration-techniques", "testing-strategies"]
  learn-from: ["testing-run", "generate-tests", "test-coverage"]
  contribute-to: "test-setup-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-project-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-framework-detection
    - continuous-memory-updates
    - real-time-setup-monitoring
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - setup-pattern-extraction
---

# Testing Configuration

## Framework
- Type: {framework_name}
- Version: {framework_version}
- Config File: {config_file_path}

## Test Structure
- Test Directory: {test_dir}
- Test Files: {count} files found
- Naming Pattern: {pattern}

## Commands
- Run All Tests: `{full_test_command}`
- Run Specific Test: `{specific_test_command}`
- Run with Debugging: `{debug_command}`

## Environment
- Required ENV vars: {list}
- Test Database: {if applicable}
- Test Servers: {if applicable}

## Test Runner Agent Configuration
- Use verbose output for debugging
- Run tests sequentially (no parallel)
- Capture full stack traces
- No mocking - use real implementations
- Wait for each test to complete
```

### 4. Configure Test-Runner Agent

Prepare agent context based on framework:

```markdown
# Test-Runner Agent Configuration

## Project Testing Setup
- Framework: {framework}
- Test Location: {directories}
- Total Tests: {count}
- Last Run: Never

## Execution Rules
1. Always use the test-runner agent from `.claude/agents/test-runner.md`
2. Run with maximum verbosity for debugging
3. No mock services - use real implementations
4. Execute tests sequentially - no parallel execution
5. Capture complete output including stack traces
6. If test fails, analyze test structure before assuming code issue
7. Report detailed failure analysis with context

## Test Command Templates
- Full Suite: `{full_command}`
- Single File: `{single_file_command}`
- Pattern Match: `{pattern_command}`
- Watch Mode: `{watch_command}` (if available)

## Common Issues to Check
- Environment variables properly set
- Test database/services running
- Dependencies installed
- Proper file permissions
- Clean test state between runs
```

### 5. Validation Steps

After configuration:
- Try running a simple test to validate setup
- Check if test command works: `{test_command} --version` or equivalent
- Verify test files are discoverable
- Ensure no permission issues

### 6. Output Summary

```
ðŸ§ª Testing Environment Primed

ðŸ” Detection Results:
  âœ… Framework: {framework_name} {version}
  âœ… Test Files: {count} files in {directories}
  âœ… Config: {config_file}
  âœ… Dependencies: All installed

ðŸ“‹ Test Structure:
  - Pattern: {test_file_pattern}
  - Directories: {test_directories}
  - Utilities: {test_helpers}

ðŸ¤– Agent Configuration:
  âœ… Test-runner agent configured
  âœ… Verbose output enabled
  âœ… Sequential execution set
  âœ… Real services (no mocks)

âš¡ Ready Commands:
  - Run all tests: /testing:run
  - Run specific: /testing:run {test_file}
  - Run pattern: /testing:run {pattern}

ðŸ’¡ Tips:
  - Always run tests with verbose output
  - Check test structure if tests fail
  - Use real services, not mocks
  - Let each test complete fully
```

### 7. Error Handling

**Common Issues:**

**No Framework Detected:**
- Message: "âš ï¸ No test framework found"
- Solution: "Please specify test command manually"
- Store user's response for future use

**Missing Dependencies:**
- Message: "âŒ Test framework not installed"
- Solution: "Install dependencies first: npm install / pip install -r requirements.txt"

**No Test Files:**
- Message: "âš ï¸ No test files found"
- Solution: "Create tests first or check test directory location"

**Permission Issues:**
- Message: "âŒ Cannot access test files"
- Solution: "Check file permissions"

### 8. Save Configuration

If successful, save configuration for future sessions:
- Store in `.claude/testing-config.md`
- Include all discovered settings
- Update on subsequent runs if changes detected

## Important Notes

- **Always detect** rather than assume test framework
- **Validate dependencies** before claiming ready
- **Configure for debugging** - verbose output is critical
- **No mocking** - use real services for accurate testing
- **Sequential execution** - avoid parallel test issues
- **Store configuration** for consistent future runs

$ARGUMENTS



