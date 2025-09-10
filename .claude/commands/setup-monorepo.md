---
model: claude-sonnet-4-20250514
category: development-setup
priority: high
tags: ["development-setup", "setup"]
description: Setup Monorepo

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["monorepo-architecture", "build-systems", "workspace-management"]
    complexity-factors: ["multi-package-coordination", "build-orchestration", "dependency-management"]
    specialized-tools: ["monorepo-configuration", "build-optimization", "workspace-setup"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "cargo-build-engineer"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "context7", "cipher-memory"]
    specialized-functions: ["monorepo-setup", "build-configuration"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "monorepo-architecture + build-systems + workspace-management"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "monorepo-patterns + build-system-knowledge"
    
    knowledge-preparation:
      - domain: "monorepo-setup"
      - pattern-search: "monorepo-strategies + build-patterns + workspace-configurations"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["monorepo-analysis", "setup-configuration", "build-optimization"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "monorepo-strategies + build-approaches + workspace-decisions"
      - pattern-recognition: "monorepo-setup-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["monorepo-configurations", "build-insights", "workspace-techniques"]
      - knowledge-extraction: "monorepo-methodologies + build-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["monorepo-relationships", "build-dependencies", "workspace-connections"]
      - cross-reference: "related-setup-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "monorepo-knowledge + build-patterns"
      - continuous-learning: "monorepo-setup-optimization"

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
    - monorepo-analysis
    - setup-configuration
    - build-optimization
    - workspace-validation
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "setup-monorepo"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "monorepo-setup-results + build-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["monorepo-patterns", "build-techniques", "workspace-management-methods"]
  learn-from: ["setup-development-environment", "setup-comprehensive-testing", "setup-linting"]
  contribute-to: "development-setup-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-project-structure
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-monorepo-setup
    - continuous-memory-updates
    - real-time-build-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - setup-pattern-extraction
---

# Setup Monorepo

Configure monorepo project structure

## Instructions

1. **Monorepo Tool Analysis**
   - Parse monorepo tool from arguments: `$ARGUMENTS` (nx, lerna, rush, yarn-workspaces, pnpm-workspaces, turborepo)
   - If no tool specified, analyze project structure and recommend best tool based on:
     - Project size and complexity
     - Existing package manager
     - Team preferences and CI/CD requirements
   - Validate tool compatibility with existing codebase

2. **Workspace Structure Setup**
   - Create standard monorepo directory structure:
     - `packages/` or `apps/` for applications
     - `libs/` or `shared/` for shared libraries
     - `tools/` for build tools and scripts
     - `docs/` for documentation
   - Configure workspace root package.json with workspace definitions
   - Set up proper .gitignore for monorepo patterns

3. **Tool-Specific Configuration**
   - **Nx**: Initialize Nx workspace, configure nx.json, add essential plugins
   - **Lerna**: Set up lerna.json, configure version management and publishing
   - **Rush**: Initialize rush.json, configure build orchestration and policies
   - **Yarn Workspaces**: Configure workspaces in package.json, set up workspace protocols
   - **pnpm Workspaces**: Set up pnpm-workspace.yaml, configure filtering and dependencies
   - **Turborepo**: Initialize turbo.json, configure pipeline and caching

4. **Package Management Configuration**
   - Configure package manager settings for workspace support
   - Set up dependency hoisting and deduplication rules
   - Configure workspace-specific package.json templates
   - Set up cross-package dependency management
   - Configure private package registry if needed

5. **Build System Integration**
   - Configure build orchestration and task running
   - Set up dependency graph analysis and affected package detection
   - Configure parallel builds and task caching
   - Set up incremental builds for changed packages
   - Configure build artifacts and output management

6. **Development Workflow**
   - Set up workspace-wide development scripts
   - Configure hot reloading and watch mode for development
   - Set up workspace-wide linting and formatting
   - Configure debugging across multiple packages
   - Set up workspace-wide testing and coverage

7. **Version Management**
   - Configure versioning strategy (independent vs. fixed versions)
   - Set up changelog generation for workspace packages
   - Configure release workflow and package publishing
   - Set up semantic versioning and conventional commits
   - Configure workspace-wide dependency updates

8. **CI/CD Pipeline Integration**
   - Configure CI to detect affected packages and run targeted tests
   - Set up build matrix for different package combinations
   - Configure deployment pipeline for multiple packages
   - Set up workspace-wide quality gates
   - Configure artifact publishing and registry management

9. **Documentation and Standards**
   - Create workspace-wide development guidelines
   - Document package creation and management procedures
   - Set up workspace-wide code standards and conventions
   - Create architectural decision records for monorepo patterns
   - Document deployment and release procedures

10. **Validation and Testing**
    - Verify workspace configuration is correct
    - Test package creation and cross-package dependencies
    - Validate build pipeline and task execution
    - Test development workflow and hot reloading
    - Verify CI/CD integration and affected package detection
    - Create example packages to demonstrate workspace functionality


