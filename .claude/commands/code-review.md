---
model: claude-sonnet-4-20250514
category: testing-quality
priority: critical
tags: ["testing-quality", "analysis", "code-review", "security-analysis", "performance-analysis"]
description: Comprehensive code quality review with universal memory integration and intelligent pattern recognition

# Enhanced Context-Aware Agent Integration with Universal Memory
agent-selection:
  type: "context-aware"
  domain-hints: ["code-review", "quality-analysis", "security-review", "performance-analysis", "architecture-review"]
  complexity-level: "complex"
  
  # Enhanced selection criteria for code review with memory integration
  selection-criteria:
    keyword-match: 0.90       # Strong code review/analysis patterns
    argument-analysis: 0.8    # File/commit context important
    project-context: 0.95     # Project type critical for review approach
    error-context: 0.7        # May include code quality issues
  
  # Specialized code review agents with memory capabilities
  preferred-agents: ["general-purpose", "rust-security-coordinator", "rust-performance-monitor"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.85

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-code-review-workflow"
  
  base-tools:
    - "Read"  # Core file reading capability
    - "Bash"  # Git and analysis commands
    - "Grep"  # Pattern searching in code
    - "Glob"  # File pattern matching
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
  
  conditional-tools:
    rust-code-review:
      - "mcp__desktop-commander__start_process"  # cargo clippy, audit
      - "mcp__context7__get-library-docs"  # Rust best practices
      - "mcp__FileScopeMCP__find_important_files"  # Critical file analysis
      - "mcp__cipher-memory__open_nodes"  # Load Rust review patterns
    
    security-analysis:
      - "mcp__desktop-commander__search_code"  # Security pattern search
      - "mcp__cipher-memory__create_entities"  # Store security findings
      - "mcp__perplexity-ask__perplexity_ask"  # Research security issues
    
    performance-analysis:
      - "mcp__FileScopeMCP__recalculate_importance"  # Performance-critical files
      - "mcp__cipher-memory__add_observations"  # Store performance insights
      - "mcp__desktop-commander__start_process"  # Performance profiling
    
    architecture-review:
      - "mcp__FileScopeMCP__generate_diagram"  # Architecture visualization
      - "mcp__cipher-memory__create_relations"  # Connect architectural patterns
      - "mcp__taskmaster-ai__add_task"  # Create refactoring tasks

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    code-quality-patterns:
      - query-pattern: "code-review + ${language}-best-practices + quality-analysis"
      - tools: ["mcp__cipher-memory__search_nodes"]
    security-vulnerability-database:
      - query-pattern: "security-vulnerabilities + ${language}-security + code-analysis"
      - tools: ["mcp__cipher-memory__open_nodes"]
    architectural-patterns:
      - query-pattern: "architecture-review + design-patterns + ${project_type}"
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "code-quality-related"
  execution-memory:
    review-progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "code-review-section-completion"
    pattern-identification:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "quality-issue-identified"
    best-practice-learning:
      - tool: "mcp__cipher-memory__create_entities"
      - trigger: "improvement-recommendation-generated"
  post-execution-memory:
    review-methodology-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-code-review-session-pattern"
    quality-pattern-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["issue-type-to-solution", "language-to-best-practice", "architecture-to-quality"]
    knowledge-enhancement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "code-quality-insights + security-patterns + performance-optimizations"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - code-review-scope-analysis
      - quality-pattern-search
      - memory-pattern-analysis
    
    execution:
      - repository-analysis-results
      - code-quality-assessment
      - security-review-findings
      - performance-analysis-results
      - architecture-evaluation
      - testing-coverage-analysis
    
    post-execution:
      - review-summary-generation
      - recommendation-prioritization
      - memory-operations
      - improvement-roadmap
  
  structured-metadata:
    command-id: "code-review"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    review-scope: "${review_arguments}"
    languages-analyzed: "${detected_languages}"
    quality-issues-found: "${issue_count}"
    security-findings: "${security_issue_count}"
    performance-concerns: "${performance_issue_count}"
    recommendations-generated: "${recommendation_count}"
    execution-time: "${duration_ms}"
    review-quality-score: "${review_effectiveness}"

# Enhanced workflow configuration
tool-chain: "universal-code-review-workflow"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
quality-pattern-recognition: true

allowed-tools: Read, Bash, Grep, Glob, mcp__desktop-commander__*, mcp__FileScopeMCP__*, mcp__context7__*, mcp__cipher-memory__*, mcp__perplexity-ask__*, mcp__taskmaster-ai__*

argument-hint: [file-path] | [commit-hash] | --full | --security | --performance | --architecture

pre-execution:
  validate-tools: true
  load-context: true
  analyze-arguments: true
  detect-project-state: true
  search-quality-patterns: true
  log-session-start: true

post-execution:
  store-results: true
  update-learning: true
  generate-report: true
  persist-review-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Code Quality Review (Universal Integration)

Perform comprehensive code quality review with intelligent pattern recognition, security vulnerability analysis, and persistent learning: $ARGUMENTS

**ENHANCED WORKFLOW**: This command utilizes specialized code review agents (rust-security-coordinator, rust-performance-monitor) with complete Cipher Memory integration for quality pattern recognition, security analysis, and code review methodology persistence.

## Enhanced Pre-Execution Memory Analysis
Before code review, the system will:
1. **Search quality patterns**: Query Cipher Memory for similar code review findings and best practices
2. **Load security knowledge**: Retrieve known vulnerability patterns and security best practices
3. **Analyze architecture patterns**: Understanding design patterns, anti-patterns, and architectural decisions
4. **Connect review methodology**: Access comprehensive code review approaches and tools

## Current State

- Git status: !`git status --porcelain`
- Recent changes: !`git diff --stat HEAD~5`
- Repository info: !`git log --oneline -5`
- Build status: !`npm run build --dry-run 2>/dev/null || echo "No build script"`

## Task

Follow these steps to conduct a thorough code review:

1. **Repository Analysis**
   - Examine the repository structure and identify the primary language/framework
   - Check for configuration files (package.json, requirements.txt, Cargo.toml, etc.)
   - Review README and documentation for context

2. **Code Quality Assessment**
   - Scan for code smells, anti-patterns, and potential bugs
   - Check for consistent coding style and naming conventions
   - Identify unused imports, variables, or dead code
   - Review error handling and logging practices

3. **Security Review**
   - Look for common security vulnerabilities (SQL injection, XSS, etc.)
   - Check for hardcoded secrets, API keys, or passwords
   - Review authentication and authorization logic
   - Examine input validation and sanitization

4. **Performance Analysis**
   - Identify potential performance bottlenecks
   - Check for inefficient algorithms or database queries
   - Review memory usage patterns and potential leaks
   - Analyze bundle size and optimization opportunities

5. **Architecture & Design**
   - Evaluate code organization and separation of concerns
   - Check for proper abstraction and modularity
   - Review dependency management and coupling
   - Assess scalability and maintainability

6. **Testing Coverage**
   - Check existing test coverage and quality
   - Identify areas lacking proper testing
   - Review test structure and organization
   - Suggest additional test scenarios

7. **Documentation Review**
   - Evaluate code comments and inline documentation
   - Check API documentation completeness
   - Review README and setup instructions
   - Identify areas needing better documentation

8. **Recommendations**
   - Prioritize issues by severity (critical, high, medium, low)
   - Provide specific, actionable recommendations
   - Suggest tools and practices for improvement
   - Create a summary report with next steps

Remember to be constructive and provide specific examples with file paths and line numbers where applicable.

## Universal Memory Integration Outcomes

### Code Quality Knowledge Storage
This command will automatically:
- **Store comprehensive review findings** in Cipher Memory for quality pattern recognition
- **Create relationships** between code issues, solutions, and best practices
- **Document security vulnerabilities** and prevention strategies
- **Build knowledge graph** of architecture patterns and quality metrics

### Cross-Command Learning Enhancement
Code review patterns will improve:
- Future development commands through quality-aware recommendations
- Debugging commands via documented code issue patterns
- Testing commands through identified coverage gaps and quality concerns
- Setup commands via architectural best practice integration

### Advanced Code Review Intelligence
- **Pattern Recognition**: Automatic identification of similar code quality issues from past reviews
- **Security Vulnerability Detection**: Intelligent scanning based on historical security findings
- **Performance Optimization Suggestions**: Smart recommendations based on performance analysis patterns
- **Architecture Quality Assessment**: Automated evaluation using established design pattern knowledge

### Intelligent Review Enhancement Features
- **Language-Specific Analysis**: Tailored review approaches based on project language and framework
- **Context-Aware Recommendations**: Smart suggestions considering project type and architectural patterns
- **Progressive Learning**: Each review improves future code analysis through pattern accumulation
- **Cross-Project Knowledge**: Shared insights across different codebases and project types

### Centralized Code Review Logging
All review operations logged to `.claude/logs/command-execution.jsonl` including:
- Complete review methodology and tool usage tracking
- Quality issue discovery and prioritization results
- Security analysis findings and recommendation generation
- Memory operations for code quality pattern capture and learning

**Next Commands**: Enhanced code review patterns will automatically improve commands like `debug-error`, `generate-tests`, `setup-linting`, `create-architecture-documentation`, and `setup-comprehensive-testing`.

