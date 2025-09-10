---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality", "setup"]
description: E2E Setup
allowed-tools: Read, Write, Edit, Bash
argument-hint: [framework] | --cypress | --playwright | --webdriver | --puppeteer | --mobile

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["e2e-testing", "test-automation", "framework-configuration"]
    complexity-factors: ["testing-framework-setup", "automation-configuration", "cross-browser-testing"]
    specialized-tools: ["test-framework-setup", "automation-configuration", "testing-optimization"]
  preferred-agents:
    primary: "mock-test-orchestrator"
    secondary: "general-purpose"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "context7", "cipher-memory"]
    specialized-functions: ["e2e-testing", "test-automation"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "e2e-testing + test-automation + framework-configuration"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "testing-patterns + automation-knowledge"
    
    knowledge-preparation:
      - domain: "e2e-testing"
      - pattern-search: "testing-frameworks + automation-strategies + e2e-patterns"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["framework-selection", "setup-configuration", "test-implementation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "framework-choices + configuration-strategies + testing-approaches"
      - pattern-recognition: "e2e-testing-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["e2e-setup-results", "testing-insights", "automation-techniques"]
      - knowledge-extraction: "testing-methodologies + automation-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["testing-relationships", "framework-dependencies", "automation-connections"]
      - cross-reference: "related-testing-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "testing-knowledge + automation-patterns"
      - continuous-learning: "e2e-testing-optimization"

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
    - framework-selection
    - setup-configuration
    - test-implementation
    - automation-setup
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "e2e-setup"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "e2e-setup-results + testing-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["testing-patterns", "automation-techniques", "framework-configuration-methods"]
  learn-from: ["generate-tests", "setup-comprehensive-testing", "setup-visual-testing"]
  contribute-to: "testing-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-testing-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-framework-setup
    - continuous-memory-updates
    - real-time-configuration-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - testing-pattern-extraction
---

# E2E Setup

Configure comprehensive end-to-end testing suite with framework optimization: **$ARGUMENTS**

## Current E2E Context

- Application type: !`find . -name "index.html" -o -name "app.js" -o -name "App.tsx" | head -1 && echo "Web app" || echo "Detect app type"`
- Framework: !`grep -l "react\\|vue\\|angular" package.json 2>/dev/null || echo "Detect framework"`
- Existing tests: !`find . -name "cypress" -o -name "playwright" -o -name "e2e" | head -1 || echo "No E2E setup"`
- CI system: !`find . -name ".github" -o -name ".gitlab-ci.yml" | head -1 || echo "No CI detected"`

## Task

Implement comprehensive end-to-end testing with framework selection and optimization:

**Framework Focus**: Use $ARGUMENTS to specify Cypress, Playwright, WebDriver, Puppeteer, mobile testing, or auto-detect best fit

**E2E Testing Framework**:
1. **Framework Selection & Setup** - Choose optimal E2E tool, install dependencies, configure basic settings, setup project structure
2. **Test Environment Configuration** - Setup test environments, configure base URLs, implement environment switching, optimize test isolation
3. **Page Object Patterns** - Design page object model, create reusable components, implement element selectors, optimize maintainability
4. **Test Data Management** - Setup test data strategies, implement fixtures, configure database seeding, design cleanup procedures
5. **Cross-Browser Testing** - Configure multi-browser execution, setup mobile testing, implement responsive testing, optimize compatibility
6. **CI/CD Integration** - Configure automated execution, setup parallel testing, implement reporting, optimize performance

**Advanced Features**: Visual regression testing, accessibility testing, performance monitoring, API testing integration, mobile device testing.

**Quality Assurance**: Test reliability optimization, flaky test prevention, execution speed optimization, debugging capabilities.

**Output**: Complete E2E testing setup with framework configuration, test suites, CI integration, and maintenance workflows.

