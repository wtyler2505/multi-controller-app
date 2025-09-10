---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality", "testing"]
description: Test Changelog Automation
allowed-tools: Read, Write, Edit, Bash
argument-hint: [automation-type] | --changelog | --workflow-demo | --ci-integration | --validation

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["changelog-automation", "workflow-integration", "validation-testing"]
    complexity-factors: ["ci-cd-integration", "validation-rules", "automated-documentation"]
    specialized-tools: ["changelog-automation", "workflow-integration", "validation-systems"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "workflow-orchestrator"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "FileScopeMCP", "cipher-memory"]
    specialized-functions: ["changelog-automation", "workflow-integration"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "changelog-automation + workflow-integration + validation-testing"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "automation-patterns + workflow-knowledge"
    
    knowledge-preparation:
      - domain: "changelog-automation"
      - pattern-search: "automation-strategies + validation-patterns + workflow-integration"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["automation-setup", "validation-configuration", "workflow-integration"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "automation-strategies + validation-approaches + workflow-decisions"
      - pattern-recognition: "changelog-automation-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["automation-results", "validation-insights", "workflow-techniques"]
      - knowledge-extraction: "automation-methodologies + validation-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["automation-relationships", "validation-dependencies", "workflow-connections"]
      - cross-reference: "related-automation-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "automation-knowledge + validation-patterns"
      - continuous-learning: "changelog-automation-optimization"

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
    - automation-setup
    - validation-configuration
    - workflow-integration
    - ci-cd-setup
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "test-changelog-automation"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "automation-results + validation-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["automation-patterns", "validation-techniques", "workflow-integration-methods"]
  learn-from: ["workflow-orchestrator", "setup-comprehensive-testing", "test-quality-analyzer"]
  contribute-to: "automation-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-automation-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-automation-setup
    - continuous-memory-updates
    - real-time-workflow-integration
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - automation-pattern-extraction
---

# Test Changelog Automation

Automate changelog testing workflow with comprehensive CI integration: **$ARGUMENTS**

## Current Automation Context

- Changelog files: !`find . -name "CHANGELOG*" -o -name "changelog*" | head -1 || echo "No changelog detected"`
- CI system: !`find . -name ".github" -o -name ".gitlab-ci.yml" -o -name "Jenkinsfile" | head -1 || echo "No CI detected"`
- Version control: !`git status >/dev/null 2>&1 && echo "Git repository" || echo "No git repository"`
- Release process: Analysis of existing release automation and versioning

## Task

Implement comprehensive changelog automation with testing and validation workflows:

**Automation Type**: Use $ARGUMENTS to focus on changelog automation, workflow demonstration, CI integration, or validation testing

**Changelog Automation Framework**:
1. **Automation Setup** - Configure changelog generation, setup version control integration, implement automated updates, design validation rules
2. **Workflow Integration** - Design CI/CD integration, configure automated triggers, implement validation checks, optimize execution performance
3. **Testing Strategy** - Create changelog validation tests, implement format verification, design content validation, setup regression testing
4. **Quality Assurance** - Configure automated formatting, implement consistency checks, setup content validation, optimize maintenance workflows
5. **Validation Framework** - Design automated validation rules, implement compliance checking, configure error reporting, optimize feedback loops
6. **CI Integration** - Setup automated execution, configure deployment triggers, implement notification systems, optimize pipeline performance

**Advanced Features**: Automated release note generation, semantic versioning integration, automated documentation updates, compliance validation.

**Quality Metrics**: Changelog accuracy, automation reliability, validation effectiveness, maintenance efficiency.

**Output**: Complete changelog automation with testing workflows, CI integration, validation rules, and maintenance procedures.

