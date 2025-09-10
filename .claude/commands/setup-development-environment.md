---
model: claude-sonnet-4-20250514
category: development-setup
priority: high
tags: ["development-setup"]
description: Complete development environment setup
allowed-tools: Read, Write, Edit, Bash
argument-hint: [tool-stack] | --rust | --nodejs | --python | --full-stack

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["development-setup", "environment-configuration", "tool-installation"]
    complexity-factors: ["multi-language-setup", "dependency-management", "configuration-automation"]
    specialized-tools: ["environment-setup", "package-management", "configuration-tools"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "cargo-build-engineer"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "context7", "cipher-memory"]
    specialized-functions: ["environment-configuration", "tool-installation"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "development-setup + environment-configuration + tool-installation"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "setup-patterns + environment-knowledge"
    
    knowledge-preparation:
      - domain: "development-environment"
      - pattern-search: "setup-strategies + configuration-patterns + tool-installation"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["environment-analysis", "setup-execution", "configuration-validation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "setup-strategies + tool-choices + configuration-decisions"
      - pattern-recognition: "environment-setup-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["setup-results", "environment-insights", "configuration-techniques"]
      - knowledge-extraction: "environment-setup-methodologies + configuration-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["setup-relationships", "tool-dependencies", "configuration-connections"]
      - cross-reference: "related-environment-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "environment-knowledge + setup-patterns"
      - continuous-learning: "environment-setup-optimization"

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
    - environment-analysis
    - setup-execution
    - configuration-validation
    - tool-installation
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "setup-development-environment"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "environment-setup-results + configuration-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["setup-patterns", "environment-techniques", "configuration-strategies"]
  learn-from: ["setup-comprehensive-testing", "setup-linting", "setup-monitoring-observability"]
  contribute-to: "development-environment-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-system-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-tool-installation
    - continuous-memory-updates
    - real-time-configuration-validation
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - environment-pattern-extraction
  
  # Pre-execution memory operations (ALWAYS EXECUTED)
  pre-execution-memory:
    context-search:
      - query-pattern: "setup-development-environment + rust-development + environment-configuration"
      - search-depth: "comprehensive"
      - max-results: 10
      - tools: ["mcp__cipher-memory__search_nodes"]
    
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - setup-history: "mcp__cipher-memory__search_nodes"
      - error-patterns: "mcp__cipher-memory__search_nodes"
    
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: internal
  
  # During execution memory tracking (CONTINUOUS)
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    setup-decisions: "mcp__cipher-memory__create_entities"
    environment-issues: "mcp__cipher-memory__add_observations"
    performance-metrics: internal
  
  # Post-execution storage (ALWAYS EXECUTED)
  post-execution-memory:
    result-storage:
      - setup-summary: "mcp__cipher-memory__create_entities"
      - configuration-patterns: "mcp__cipher-memory__create_entities"
      - performance-data: "mcp__cipher-memory__add_observations"
    
    relationship-creation:
      - setup-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - tool-relationships: "mcp__cipher-memory__create_relations"
    
    knowledge-enrichment:
      - existing-setups: "mcp__cipher-memory__add_observations"
      - new-insights: "mcp__cipher-memory__create_entities"

# Universal Centralized Logging (MANDATORY FOR ALL COMMANDS)
centralized-logging:
  enabled: true
  log-file: ".claude/execution-log.jsonl"
  
  # What gets logged (COMPREHENSIVE)
  log-components:
    execution-metadata: true      # Command, timestamp, context
    agent-selection: true         # Selection process and results
    tool-chain: true             # Tools selected and used
    memory-operations: true       # All Cipher Memory interactions
    performance-metrics: true    # Timing, resource usage
    success-indicators: true     # Success/failure, error details
    learning-data: true          # Pattern accuracy, optimization data
  
  # Logging phases (EVERY EXECUTION)
  logging-phases:
    pre-execution: true          # Log command start and context
    during-execution: true       # Log setup progress and decisions
    post-execution: true         # Log setup results and learning
    error-handling: true         # Log any setup errors or failures

# Enhanced tool selection with memory integration
tool-selection:
  type: "context-driven"
  
  # MANDATORY memory tools for ALL commands
  mandatory-tools:
    - "mcp__cipher-memory__search_nodes"
    - "mcp__cipher-memory__open_nodes"
    - "mcp__cipher-memory__create_entities"
    - "mcp__cipher-memory__create_relations"
    - "mcp__cipher-memory__add_observations"
    - "mcp__cipher-memory__read_graph"
  
  # Base tools for development environment setup
  base-tools:
    - "mcp__desktop-commander__start_process"
    - "mcp__FileScopeMCP__find_important_files"
    - "mcp__context7__get-library-docs"
  
  conditional-tools:
    rust-project:
      - "mcp__desktop-commander__start_process"
      - "mcp__context7__get-library-docs"
      - "mcp__perplexity-ask__perplexity_ask"
    
    typescript-project:
      - "mcp__desktop-commander__start_process"
      - "mcp__context7__get-library-docs"
      - "mcp__perplexity-ask__perplexity_ask"
    
    mixed-project:
      - "mcp__desktop-commander__start_process"
      - "mcp__FileScopeMCP__recalculate_importance"
      - "mcp__context7__get-library-docs"

# Enhanced workflow configuration
tool-chain: memory-integrated-rust-development
auto-deploy: true
parallel-execution: false

# Universal pre-execution (EVERY COMMAND)
pre-execution:
  # Memory operations (MANDATORY)
  memory-context-loading: true
  cipher-search-patterns: true
  load-execution-history: true
  analyze-related-nodes: true
  
  # Standard operations
  validate-tools: true
  load-context: true
  analyze-arguments: true
  detect-project-state: true
  prepare-environment: true
  
  # Logging
  initialize-execution-log: true

# Universal post-execution (EVERY COMMAND)
post-execution:
  # Memory operations (MANDATORY)
  store-execution-results: true
  create-pattern-relationships: true
  enrich-existing-knowledge: true
  update-success-patterns: true
  
  # Learning operations
  update-selection-accuracy: true
  optimize-tool-chains: true
  analyze-performance-metrics: true
  
  # Standard operations
  generate-report: true
  
  # Logging
  finalize-execution-log: true
  generate-execution-summary: true
---

# Setup Development Environment

Setup complete development environment

## Instructions

1. **Environment Analysis and Requirements**
   - Analyze current project structure and technology stack
   - Identify required development tools and dependencies
   - Check existing development environment configuration
   - Determine team size and collaboration requirements
   - Assess platform requirements (Windows, macOS, Linux)

2. **Core Development Tools Installation**
   - Verify and install required runtime environments (Node.js, Python, Java, etc.)
   - Set up package managers with proper versions (npm, yarn, pnpm, pip, maven, etc.)
   - Install and configure version control tools (Git, Git LFS)
   - Set up code editors with workspace-specific settings (VSCode, IntelliJ)
   - Configure terminal and shell environment

3. **Project-Specific Tooling**
   - Install project dependencies and dev dependencies
   - Set up build tools and task runners
   - Configure bundlers and module systems
   - Install testing frameworks and runners
   - Set up debugging tools and extensions
   - Configure profiling and performance monitoring tools

4. **Code Quality and Standards**
   - Install and configure linting tools (ESLint, Pylint, etc.)
   - Set up code formatting tools (Prettier, Black, etc.)
   - Configure pre-commit hooks with Husky or similar
   - Set up code spell checking and grammar tools
   - Configure import sorting and organization tools
   - Set up code complexity and quality metrics

5. **Development Server and Database**
   - Set up local development server with hot reloading
   - Configure database server and management tools
   - Set up containerized development environment (Docker)
   - Configure API mocking and testing tools
   - Set up local SSL certificates for HTTPS development
   - Configure environment variable management

6. **IDE and Editor Configuration**
   - Configure workspace settings and extensions
   - Set up language-specific plugins and syntax highlighting
   - Configure IntelliSense and auto-completion
   - Set up debugging configurations and breakpoints
   - Configure integrated terminal and task running
   - Set up code snippets and templates

7. **Environment Variables and Secrets**
   - Create .env template files for different environments
   - Set up local environment variable management
   - Configure secrets management for development
   - Set up API keys and service credentials
   - Configure environment-specific configuration files
   - Document required environment variables

8. **Documentation and Knowledge Base**
   - Create comprehensive setup documentation
   - Document common development workflows
   - Set up project wiki or knowledge base
   - Create troubleshooting guides for common issues
   - Document coding standards and best practices
   - Set up onboarding checklist for new team members

9. **Collaboration and Communication Tools**
   - Configure team communication channels
   - Set up code review workflows and tools
   - Configure issue tracking and project management
   - Set up shared development resources and services
   - Configure team calendars and meeting tools
   - Set up shared documentation and file storage

10. **Validation and Testing**
    - Verify all tools and dependencies are properly installed
    - Test development server startup and hot reloading
    - Validate database connections and data access
    - Test build processes and deployment workflows
    - Verify code quality tools are working correctly
    - Test collaboration workflows and team access
    - Create development environment health check script


