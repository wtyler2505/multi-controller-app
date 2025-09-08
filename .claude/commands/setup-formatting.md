---
model: claude-sonnet-4-20250514
category: development-setup
priority: high
tags: ["development-setup"]
description: Setup code formatting and style enforcement
allowed-tools: Read, Write, Edit, Bash
argument-hint: [formatter-type] | --rust | --typescript | --python | --prettier

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["code-formatting", "style-enforcement", "development-tools"]
    complexity-factors: ["multi-language-formatting", "configuration-management", "integration-setup"]
    specialized-tools: ["formatting-tools", "linting-integration", "pre-commit-hooks"]
  preferred-agents:
    primary: "cargo-build-engineer"
    secondary: "general-purpose"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "context7", "cipher-memory"]
    specialized-functions: ["formatting-configuration", "style-enforcement"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "code-formatting + style-enforcement + development-tools"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "formatting-patterns + style-knowledge"
    
    knowledge-preparation:
      - domain: "code-formatting"
      - pattern-search: "formatting-strategies + style-patterns + tool-configuration"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["formatting-analysis", "configuration-setup", "style-validation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "formatting-strategies + tool-choices + configuration-decisions"
      - pattern-recognition: "formatting-setup-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["formatting-results", "style-insights", "configuration-techniques"]
      - knowledge-extraction: "formatting-methodologies + style-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["formatting-relationships", "tool-dependencies", "style-connections"]
      - cross-reference: "related-formatting-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "formatting-knowledge + style-patterns"
      - continuous-learning: "formatting-setup-optimization"

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
    - formatting-analysis
    - configuration-setup
    - style-validation
    - tool-installation
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "setup-formatting"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "formatting-setup-results + style-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["formatting-patterns", "style-techniques", "configuration-strategies"]
  learn-from: ["setup-linting", "setup-development-environment", "code-review"]
  contribute-to: "code-quality-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-project-type
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-formatter-setup
    - continuous-memory-updates
    - real-time-configuration-validation
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - formatting-pattern-extraction
---

# Setup Formatting

Setup code formatting and style enforcement: **$ARGUMENTS**

## Instructions

Configure comprehensive code formatting solutions for consistent code style:

1. **Project Type Detection**
   - Analyze codebase to determine primary language
   - Check existing configuration files  
   - Identify team preferences and standards
   - Assess integration requirements with existing tools

2. **Language-Specific Formatter Setup**
   - **Rust**: Configure `rustfmt` with team standards
   - **TypeScript/JavaScript**: Set up Prettier with ESLint integration
   - **Python**: Install Black and isort with consistent configuration  
   - **Multi-language**: Configure consistent cross-language standards

3. **Configuration Management**
   - Create formatter configuration files
   - Set up format-on-save for development environments
   - Configure pre-commit hooks for consistent formatting
   - Document formatting standards and team preferences

4. **IDE Integration**
   - Install and configure formatter extensions
   - Set up consistent keyboard shortcuts
   - Configure format-on-save behavior
   - Ensure team IDE settings consistency

5. **Automation Setup**
   - Add formatting scripts to package.json/Cargo.toml
   - Configure CI/CD format checking
   - Set up pre-commit formatting validation
   - Create format fixing automation

6. **Team Standards Documentation**
   - Document formatting decisions and rationale
   - Create formatting guidelines for team members
   - Set up formatting troubleshooting guides
   - Establish formatting review processes


