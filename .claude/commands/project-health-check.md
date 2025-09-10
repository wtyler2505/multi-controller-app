---
model: claude-sonnet-4-20250514
category: testing-quality
priority: high
tags: ["testing-quality", "project-analysis", "health-monitoring", "metrics", "reporting"]
description: Comprehensive project health analysis with universal memory integration and intelligent metrics recognition

# Enhanced Context-Aware Agent Integration with Universal Memory
agent-selection:
  type: "context-aware"
  domain-hints: ["project-analysis", "health-monitoring", "metrics", "quality-assessment", "performance-analysis"]
  complexity-level: "complex"
  
  # Enhanced selection criteria for project health analysis with memory integration
  selection-criteria:
    keyword-match: 0.90       # Strong analysis/monitoring patterns
    argument-analysis: 0.80   # Time period context important
    project-context: 0.95     # Project type critical for health metrics
    error-context: 0.6        # May include quality issues
  
  # Specialized analysis agents with memory capabilities
  preferred-agents: ["general-purpose", "rust-performance-monitor", "cargo-build-engineer"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.85

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-health-analysis-workflow"
  
  base-tools:
    - "mcp__desktop-commander__start_process"  # Run analysis commands
    - "mcp__FileScopeMCP__find_important_files"  # Analyze codebase structure
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
  
  conditional-tools:
    rust-health-analysis:
      - "mcp__desktop-commander__start_process"  # cargo audit, clippy metrics
      - "mcp__context7__get-library-docs"  # Rust health best practices
      - "mcp__cipher-memory__open_nodes"  # Load health analysis patterns
    
    performance-monitoring:
      - "mcp__FileScopeMCP__recalculate_importance"  # Performance-critical files
      - "mcp__cipher-memory__create_entities"  # Store performance insights
      - "mcp__desktop-commander__start_process"  # Performance profiling
    
    quality-metrics:
      - "mcp__cipher-memory__add_observations"  # Store quality insights
      - "mcp__desktop-commander__start_process"  # Quality analysis tools
      - "mcp__cipher-memory__create_relations"  # Connect quality patterns
    
    dependency-analysis:
      - "mcp__desktop-commander__start_process"  # Security audits
      - "mcp__cipher-memory__create_entities"  # Store dependency patterns
      - "mcp__perplexity-ask__perplexity_ask"  # Research security issues

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    health-patterns-search:
      - query-pattern: "project-health + ${project_type}-metrics + quality-analysis"
      - tools: ["mcp__cipher-memory__search_nodes"]
    historical-health-data:
      - query-pattern: "health-monitoring + project-metrics + performance-analysis"
      - tools: ["mcp__cipher-memory__open_nodes"]
    benchmark-standards:
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "health-metrics-related"
  execution-memory:
    analysis-progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "health-metric-calculation"
    pattern-identification:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "health-trend-identified"
    insight-learning:
      - tool: "mcp__cipher-memory__create_entities"
      - trigger: "actionable-insight-generated"
  post-execution-memory:
    health-analysis-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-health-analysis-session-pattern"
    metric-pattern-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["metric-to-trend", "issue-to-solution", "health-to-action"]
    knowledge-enhancement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "health-insights + monitoring-strategies + improvement-recommendations"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - health-analysis-scope
      - metric-pattern-search
      - memory-pattern-analysis
    
    execution:
      - codebase-analysis-results
      - quality-metric-calculation
      - dependency-health-assessment
      - performance-analysis
      - trend-analysis
    
    post-execution:
      - health-report-generation
      - recommendation-prioritization
      - memory-operations
      - action-plan-creation
  
  structured-metadata:
    command-id: "project-health-check"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    evaluation-period: "${analysis_timeframe}"
    metrics-analyzed: "${health_metrics_count}"
    issues-identified: "${issue_count}"
    recommendations-generated: "${recommendation_count}"
    health-score: "${overall_health_score}"
    execution-time: "${duration_ms}"
    analysis-quality-score: "${analysis_effectiveness}"

# Enhanced workflow configuration
tool-chain: "universal-health-analysis-workflow"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
health-pattern-recognition: true

allowed-tools: Read, Bash, Grep, Glob, mcp__desktop-commander__*, mcp__FileScopeMCP__*, mcp__context7__*, mcp__cipher-memory__*, mcp__perplexity-ask__*, mcp__taskmaster-ai__*

argument-hint: [evaluation-period] | --30-days | --sprint | --quarter | --comprehensive

pre-execution:
  validate-tools: true
  load-context: true
  analyze-timeframe: true
  search-health-patterns: true
  log-session-start: true

post-execution:
  store-results: true
  update-learning: true
  generate-report: true
  persist-health-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Project Health Check (Universal Integration)

Analyze comprehensive project health and metrics with universal memory integration and intelligent pattern recognition: **$ARGUMENTS**

**ENHANCED WORKFLOW**: This command utilizes specialized analysis agents (rust-performance-monitor, cargo-build-engineer) with complete Cipher Memory integration for health pattern recognition, metric analysis, and improvement recommendation persistence.

## Enhanced Pre-Execution Memory Analysis
Before health analysis, the system will:
1. **Search health patterns**: Query Cipher Memory for historical project health data and analysis methodologies
2. **Load benchmark standards**: Retrieve industry health benchmarks and quality thresholds
3. **Analyze trend patterns**: Understanding health trend analysis and prediction methodologies
4. **Connect improvement strategies**: Access comprehensive project improvement and optimization patterns

## Current Project State

- Git activity: !`git log --oneline --since="30 days ago" | wc -l`
- Contributors: !`git shortlog -sn --since="30 days ago" | head -5`
- Branch status: !`git branch -r | wc -l` remote branches
- Code changes: !`git diff --stat HEAD~30 2>/dev/null || echo "Not enough history"`
- Dependencies: @package.json or @requirements.txt or @Cargo.toml (if exists)

## Task

Generate a comprehensive project health report analyzing:

**Evaluation Period**: Use $ARGUMENTS or default to last 30 days

**Health Dimensions**:
1. **Code Quality Metrics**
   - Test coverage and trends
   - Code complexity analysis
   - Security vulnerabilities (run npm audit or equivalent)
   - Technical debt indicators

2. **Delivery Performance**
   - Sprint velocity trends (if task management tools available)
   - Cycle time analysis
   - Bug vs feature ratio
   - On-time delivery metrics

3. **Team Health Indicators**
   - PR review turnaround time
   - Commit frequency distribution
   - Work distribution balance
   - Knowledge concentration risk

4. **Dependency Health**
   - Outdated packages assessment
   - Security audit results
   - License compliance check
   - External service dependencies

**Health Report Format**:
- Overall health score (0-100) with color-coded status
- Executive summary with key findings
- Detailed metrics tables with current vs target values
- Trend analysis and risk assessment
- Actionable recommendations prioritized by impact

**Output**: Generate markdown report with charts, metrics tables, and specific action items for improving project health.

## Universal Memory Integration Outcomes

### Health Analysis Knowledge Storage
This command will automatically:
- **Store comprehensive health analysis sessions** in Cipher Memory for health pattern recognition
- **Create relationships** between health metrics, trends, and improvement strategies
- **Document health monitoring methodologies** and analysis best practices
- **Build knowledge graph** of health-improvement mappings and metric optimization strategies

### Cross-Command Learning Enhancement
Health analysis patterns will improve:
- Future monitoring commands through established health baseline patterns
- Performance optimization commands via documented performance trend analysis
- Quality assurance commands through health metric integration
- Setup commands via proven health improvement methodologies

### Advanced Health Intelligence
- **Trend Prediction**: Automatic identification of health trend patterns and future risk prediction
- **Benchmark Comparison**: Intelligent health score comparison against industry and historical benchmarks
- **Risk Assessment**: Smart identification of project health risks based on metric correlation patterns
- **Improvement Prioritization**: Automated prioritization of health improvements based on impact analysis

### Intelligent Analysis Enhancement Features
- **Project-Specific Metrics**: Tailored health analysis approaches based on project type and characteristics
- **Context-Aware Scoring**: Smart health scoring considering project complexity and development stage
- **Progressive Health Learning**: Each analysis session improves future health assessments through pattern accumulation
- **Cross-Project Health Knowledge**: Shared health insights across different projects and domains

### Centralized Health Analysis Logging
All health analysis operations logged to `.claude/logs/command-execution.jsonl` including:
- Complete analysis methodology and metric calculation tracking
- Health trend identification and risk assessment results
- Memory operations for health pattern capture and learning
- Improvement recommendation generation and prioritization effectiveness

**Next Commands**: Enhanced health patterns will automatically improve commands like `optimize-bundle-size`, `setup-monitoring-observability`, `performance-analysis`, and `code-review`.

