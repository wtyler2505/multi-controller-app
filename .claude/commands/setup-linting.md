---
model: claude-sonnet-4-20250514
category: development-setup
priority: critical
tags: ["development-setup", "setup", "linting", "code-quality", "automation"]
description: Setup comprehensive code linting and quality tools with universal memory integration and intelligent pattern recognition

# Enhanced Context-Aware Agent Integration with Universal Memory
agent-selection:
  type: "context-aware"
  domain-hints: ["linting-setup", "code-quality", "development-environment", "automation", "build-tools"]
  complexity-level: "standard"
  
  # Enhanced selection criteria for linting setup with memory integration
  selection-criteria:
    keyword-match: 0.85       # Strong linting/quality patterns
    argument-analysis: 0.75   # Tool-specific context important
    project-context: 0.90     # Language/framework critical for setup
    error-context: 0.5        # May include configuration issues
  
  # Specialized setup agents with memory capabilities
  preferred-agents: ["general-purpose", "cargo-build-engineer"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.80

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-linting-setup-workflow"
  
  base-tools:
    - "mcp__desktop-commander__start_process"  # Install and configure tools
    - "mcp__FileScopeMCP__find_important_files"  # Locate config files
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
  
  conditional-tools:
    rust-linting:
      - "mcp__desktop-commander__start_process"  # cargo clippy, rustfmt
      - "mcp__context7__get-library-docs"  # Rust linting best practices
      - "mcp__cipher-memory__open_nodes"  # Load Rust quality patterns
    
    javascript-linting:
      - "mcp__desktop-commander__start_process"  # eslint, prettier setup
      - "mcp__context7__get-library-docs"  # ESLint configuration docs
      - "mcp__cipher-memory__create_entities"  # Store JS linting patterns
    
    quality-automation:
      - "mcp__cipher-memory__add_observations"  # Store automation strategies
      - "mcp__desktop-commander__start_process"  # CI/CD integration setup
      - "mcp__cipher-memory__create_relations"  # Connect quality patterns

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    linting-patterns-search:
      - query-pattern: "linting-setup + ${language}-quality + code-standards"
      - tools: ["mcp__cipher-memory__search_nodes"]
    framework-configuration:
      - query-pattern: "linting-configuration + quality-tools + ${project_type}"
      - tools: ["mcp__cipher-memory__open_nodes"]
    automation-strategies:
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "quality-automation-related"
  execution-memory:
    setup-progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "linting-tool-installation"
    configuration-discovery:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "effective-config-identified"
    automation-learning:
      - tool: "mcp__cipher-memory__create_entities"
      - trigger: "automation-workflow-established"
  post-execution-memory:
    setup-methodology-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-linting-setup-session-pattern"
    quality-pattern-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["tool-to-effectiveness", "config-to-quality", "automation-to-workflow"]
    knowledge-enhancement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "linting-insights + quality-automation + configuration-optimization"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - linting-setup-scope-analysis
      - quality-pattern-search
      - memory-pattern-analysis
    
    execution:
      - project-analysis-results
      - tool-installation-progress
      - configuration-setup
      - automation-integration
      - validation-testing
    
    post-execution:
      - setup-summary
      - quality-metrics
      - memory-operations
      - automation-recommendations
  
  structured-metadata:
    command-id: "setup-linting"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    setup-scope: "${setup_arguments}"
    languages-configured: "${detected_languages}"
    tools-installed: "${linting_tools_count}"
    configurations-created: "${config_files_count}"
    automation-level: "${automation_integration}"
    execution-time: "${duration_ms}"
    setup-quality-score: "${setup_effectiveness}"

# Enhanced workflow configuration
tool-chain: "universal-linting-setup-workflow"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
quality-pattern-recognition: true

allowed-tools: Read, Write, Edit, Bash, mcp__desktop-commander__*, mcp__FileScopeMCP__*, mcp__context7__*, mcp__cipher-memory__*, mcp__taskmaster-ai__*

argument-hint: [language] | [framework] | --eslint | --prettier | --clippy | --ci-integration

pre-execution:
  validate-tools: true
  load-context: true
  analyze-project-structure: true
  search-linting-patterns: true
  log-session-start: true

post-execution:
  store-results: true
  update-learning: true
  generate-report: true
  persist-quality-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Setup Comprehensive Linting (Universal Integration)

Setup comprehensive code linting and quality tools with universal memory integration and intelligent pattern recognition: $ARGUMENTS

**ENHANCED WORKFLOW**: This command utilizes specialized setup agents (cargo-build-engineer for Rust) with complete Cipher Memory integration for linting pattern recognition, configuration optimization, and quality automation persistence.

## Enhanced Pre-Execution Memory Analysis
Before linting setup, the system will:
1. **Search quality patterns**: Query Cipher Memory for effective linting configurations and quality standards
2. **Load framework knowledge**: Retrieve language-specific linting best practices and tool configurations
3. **Analyze automation strategies**: Understanding CI/CD integration patterns and workflow automation
4. **Connect quality knowledge**: Access comprehensive code quality improvement methodologies

## Instructions

Follow this systematic approach to setup linting: **$ARGUMENTS**

1. **Project Analysis**
   - Identify programming languages and frameworks
   - Check existing linting configuration
   - Review current code style and patterns
   - Assess team preferences and requirements

2. **Tool Selection by Language**

   **JavaScript/TypeScript:**
   ```bash
   npm install -D eslint @typescript-eslint/parser @typescript-eslint/eslint-plugin
   npm install -D prettier eslint-config-prettier eslint-plugin-prettier
   ```

   **Python:**
   ```bash
   pip install flake8 black isort mypy pylint
   ```

   **Java:**
   ```bash
   # Add to pom.xml or build.gradle
   # Checkstyle, SpotBugs, PMD
   ```

3. **Configuration Setup**

   **ESLint (.eslintrc.json):**
   ```json
   {
     "extends": [
       "eslint:recommended",
       "@typescript-eslint/recommended",
       "prettier"
     ],
     "parser": "@typescript-eslint/parser",
     "plugins": ["@typescript-eslint"],
     "rules": {
       "no-console": "warn",
       "no-unused-vars": "error",
       "@typescript-eslint/no-explicit-any": "warn"
     }
   }
   ```

4. **IDE Integration**
   - Configure VS Code settings
   - Setup auto-fix on save
   - Install relevant extensions

5. **CI/CD Integration**
   ```yaml
   - name: Lint code
     run: npm run lint
   ```

6. **Package.json Scripts**
   ```json
   {
     "scripts": {
       "lint": "eslint src --ext .ts,.tsx",
       "lint:fix": "eslint src --ext .ts,.tsx --fix",
       "format": "prettier --write src"
     }
   }
   ```

Remember to customize rules based on team preferences and gradually enforce stricter standards.

## Universal Memory Integration Outcomes

### Quality Setup Knowledge Storage
This command will automatically:
- **Store comprehensive linting setup sessions** in Cipher Memory for quality pattern recognition
- **Create relationships** between linting tools, configurations, and code quality improvements
- **Document quality automation strategies** and CI/CD integration patterns
- **Build knowledge graph** of tool-effectiveness mappings and configuration optimization strategies

### Cross-Command Learning Enhancement
Linting setup patterns will improve:
- Future development commands through quality-aware configuration recommendations
- Code review commands via established linting rule patterns
- Testing commands through quality standard integration
- Setup commands via proven automation workflow patterns

### Advanced Quality Intelligence
- **Configuration Optimization**: Automatic identification of optimal linting configurations based on project characteristics
- **Tool Selection**: Intelligent linting tool recommendations based on language, framework, and team preferences
- **Automation Integration**: Smart CI/CD integration strategies based on successful automation patterns
- **Quality Evolution**: Progressive improvement of code quality standards through pattern learning

### Intelligent Setup Enhancement Features
- **Language-Specific Configuration**: Tailored linting setup approaches based on project language and framework
- **Context-Aware Rule Selection**: Smart linting rule recommendations considering project complexity and team experience
- **Progressive Quality Implementation**: Each setup session improves future quality automation through pattern accumulation
- **Cross-Project Quality Knowledge**: Shared quality insights across different codebases and project types

### Centralized Quality Setup Logging
All linting setup operations logged to `.claude/logs/command-execution.jsonl` including:
- Complete setup methodology and tool usage tracking
- Configuration optimization results and automation integration
- Memory operations for quality pattern capture and learning
- Quality improvement metrics and automation effectiveness

**Next Commands**: Enhanced quality patterns will automatically improve commands like `code-review`, `setup-comprehensive-testing`, `project-health-check`, and `optimize-bundle-size`.


