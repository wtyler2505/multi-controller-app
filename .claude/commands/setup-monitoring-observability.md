---
model: claude-sonnet-4-20250514
category: development-setup
priority: high
tags: ["development-setup", "setup", "performance", "monitoring", "observability", "alerting", "metrics"]
description: Setup comprehensive monitoring and observability infrastructure with universal memory integration and intelligent pattern recognition

# Enhanced Context-Aware Agent Integration with Universal Memory
agent-selection:
  type: "context-aware"
  domain-hints: ["monitoring-setup", "observability", "performance-monitoring", "infrastructure", "alerting"]
  complexity-level: "complex"
  
  # Enhanced selection criteria for monitoring setup with memory integration
  selection-criteria:
    keyword-match: 0.90       # Strong monitoring/observability patterns
    argument-analysis: 0.80   # Monitoring scope context important
    project-context: 0.95     # Infrastructure type critical for setup
    error-context: 0.7        # May include monitoring failures
  
  # Specialized monitoring agents with memory capabilities
  preferred-agents: ["general-purpose", "rust-performance-monitor", "logging-integrator"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.85

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-monitoring-setup-workflow"
  
  base-tools:
    - "mcp__desktop-commander__start_process"  # Setup monitoring tools
    - "mcp__FileScopeMCP__find_important_files"  # Analyze system components
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
  
  conditional-tools:
    infrastructure-monitoring:
      - "mcp__desktop-commander__start_process"  # Prometheus, Grafana setup
      - "mcp__context7__get-library-docs"  # Monitoring tool documentation
      - "mcp__cipher-memory__open_nodes"  # Load monitoring patterns
    
    application-monitoring:
      - "mcp__FileScopeMCP__recalculate_importance"  # Critical service files
      - "mcp__cipher-memory__create_entities"  # Store monitoring insights
      - "mcp__desktop-commander__start_process"  # APM tool configuration
    
    alerting-setup:
      - "mcp__cipher-memory__add_observations"  # Store alerting strategies
      - "mcp__desktop-commander__start_process"  # Alert manager setup
      - "mcp__cipher-memory__create_relations"  # Connect alerting patterns
    
    observability-integration:
      - "mcp__desktop-commander__start_process"  # Tracing system setup
      - "mcp__cipher-memory__create_entities"  # Store observability patterns
      - "mcp__perplexity-ask__perplexity_ask"  # Research monitoring practices

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    monitoring-patterns-search:
      - query-pattern: "monitoring-setup + observability + ${infrastructure_type}-monitoring"
      - tools: ["mcp__cipher-memory__search_nodes"]
    infrastructure-analysis:
      - query-pattern: "monitoring-infrastructure + alerting + ${technology_stack}"
      - tools: ["mcp__cipher-memory__open_nodes"]
    best-practices-load:
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "monitoring-observability-related"
  execution-memory:
    setup-progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "monitoring-tool-configuration"
    pattern-discovery:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "effective-monitoring-identified"
    optimization-learning:
      - tool: "mcp__cipher-memory__create_entities"
      - trigger: "monitoring-baseline-established"
  post-execution-memory:
    monitoring-infrastructure-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-monitoring-setup-session-pattern"
    observability-pattern-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["tool-to-effectiveness", "metric-to-insight", "alert-to-action"]
    knowledge-enhancement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "monitoring-insights + alerting-strategies + observability-optimization"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - monitoring-setup-scope
      - infrastructure-analysis
      - memory-pattern-analysis
    
    execution:
      - tool-installation-progress
      - configuration-setup
      - metric-definition
      - alerting-configuration
      - dashboard-creation
    
    post-execution:
      - monitoring-validation
      - alerting-testing
      - memory-operations
      - optimization-recommendations
  
  structured-metadata:
    command-id: "setup-monitoring-observability"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    monitoring-scope: "${setup_arguments}"
    tools-configured: "${monitoring_tools_count}"
    metrics-defined: "${metrics_count}"
    alerts-configured: "${alerts_count}"
    dashboards-created: "${dashboard_count}"
    execution-time: "${duration_ms}"
    setup-quality-score: "${monitoring_effectiveness}"

# Enhanced workflow configuration
tool-chain: "universal-monitoring-setup-workflow"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
monitoring-pattern-recognition: true

allowed-tools: Read, Write, Edit, Bash, mcp__desktop-commander__*, mcp__FileScopeMCP__*, mcp__context7__*, mcp__cipher-memory__*, mcp__perplexity-ask__*, mcp__taskmaster-ai__*

argument-hint: [scope] | --metrics | --logging | --tracing | --alerting | --dashboards | --comprehensive

pre-execution:
  validate-tools: true
  load-context: true
  analyze-infrastructure: true
  search-monitoring-patterns: true
  log-session-start: true

post-execution:
  store-results: true
  update-learning: true
  generate-report: true
  persist-monitoring-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Setup Monitoring and Observability (Universal Integration)

Setup comprehensive monitoring and observability infrastructure with universal memory integration and intelligent pattern recognition: $ARGUMENTS

**ENHANCED WORKFLOW**: This command utilizes specialized monitoring agents (rust-performance-monitor, logging-integrator) with complete Cipher Memory integration for monitoring pattern recognition, observability optimization, and infrastructure setup persistence.

## Enhanced Pre-Execution Memory Analysis
Before monitoring setup, the system will:
1. **Search monitoring patterns**: Query Cipher Memory for effective monitoring configurations and observability strategies
2. **Load infrastructure knowledge**: Retrieve monitoring infrastructure best practices and tool configurations
3. **Analyze alerting strategies**: Understanding alerting patterns and incident response methodologies
4. **Connect observability knowledge**: Access comprehensive observability implementation and optimization patterns

## Instructions

1. **Observability Strategy Planning**
   - Analyze application architecture and monitoring requirements
   - Define key performance indicators (KPIs) and service level objectives (SLOs)
   - Plan monitoring stack architecture and data flow
   - Assess compliance and retention requirements
   - Define alerting strategies and escalation procedures

2. **Metrics Collection and Monitoring**
   - Set up application metrics collection (Prometheus, DataDog, New Relic)
   - Configure infrastructure monitoring for servers, containers, and cloud resources
   - Set up business metrics and user experience monitoring
   - Configure custom metrics for application-specific monitoring
   - Set up metrics aggregation and time-series storage

3. **Logging Infrastructure**
   - Set up centralized logging system (ELK Stack, Fluentd, Splunk)
   - Configure structured logging with consistent formats
   - Set up log aggregation and forwarding from all services
   - Configure log retention policies and archival strategies
   - Set up log parsing, enrichment, and indexing

4. **Distributed Tracing**
   - Set up distributed tracing system (Jaeger, Zipkin, AWS X-Ray)
   - Configure trace instrumentation in application code
   - Set up trace sampling and collection strategies
   - Configure trace correlation across service boundaries
   - Set up trace analysis and performance optimization

5. **Application Performance Monitoring (APM)**
   - Configure APM tools for application performance insights
   - Set up error tracking and exception monitoring
   - Configure database query monitoring and optimization
   - Set up real user monitoring (RUM) and synthetic monitoring
   - Configure performance profiling and bottleneck identification

6. **Infrastructure and System Monitoring**
   - Set up server and container monitoring (CPU, memory, disk, network)
   - Configure cloud service monitoring and cost tracking
   - Set up database monitoring and performance analysis
   - Configure network monitoring and security scanning
   - Set up capacity planning and resource optimization

7. **Alerting and Notification System**
   - Configure intelligent alerting with proper thresholds
   - Set up alert routing and escalation procedures
   - Configure notification channels (email, Slack, PagerDuty)
   - Set up alert correlation and noise reduction
   - Configure on-call scheduling and incident management

8. **Dashboards and Visualization**
   - Create comprehensive monitoring dashboards (Grafana, Kibana)
   - Set up real-time system health dashboards
   - Configure business metrics and KPI visualization
   - Create role-specific dashboards for different teams
   - Set up mobile-friendly monitoring interfaces

9. **Security Monitoring and Compliance**
   - Set up security event monitoring and SIEM integration
   - Configure compliance monitoring and audit trails
   - Set up vulnerability scanning and security alerting
   - Configure access monitoring and user behavior analytics
   - Set up data privacy and protection monitoring

10. **Incident Response and Automation**
    - Set up automated incident detection and response
    - Configure runbook automation and self-healing systems
    - Set up incident management and communication workflows
    - Configure post-incident analysis and improvement processes
    - Create monitoring maintenance and optimization procedures
    - Train team on monitoring tools and incident response procedures

## Universal Memory Integration Outcomes

### Monitoring Setup Knowledge Storage
This command will automatically:
- **Store comprehensive monitoring setup sessions** in Cipher Memory for observability pattern recognition
- **Create relationships** between monitoring tools, metrics, and operational insights
- **Document monitoring methodologies** and alerting best practices
- **Build knowledge graph** of monitoring-effectiveness mappings and observability optimization strategies

### Cross-Command Learning Enhancement
Monitoring setup patterns will improve:
- Future performance commands through established monitoring baseline patterns
- Debugging commands via documented monitoring and alerting insights
- Health check commands through monitoring infrastructure integration
- Setup commands via proven monitoring automation workflows

### Advanced Monitoring Intelligence
- **Tool Selection Optimization**: Automatic identification of optimal monitoring tools based on infrastructure characteristics
- **Alert Strategy**: Intelligent alerting configuration based on successful incident response patterns
- **Dashboard Design**: Smart dashboard creation using proven visualization and metric correlation patterns
- **Observability Architecture**: Automated observability infrastructure recommendations based on project complexity

### Intelligent Setup Enhancement Features
- **Infrastructure-Specific Configuration**: Tailored monitoring setup approaches based on technology stack and architecture
- **Context-Aware Alerting**: Smart alerting rule configuration considering system behavior and incident patterns
- **Progressive Monitoring Learning**: Each setup session improves future monitoring through pattern accumulation
- **Cross-Project Monitoring Knowledge**: Shared monitoring insights across different infrastructure types and scales

### Centralized Monitoring Setup Logging
All monitoring setup operations logged to `.claude/logs/command-execution.jsonl` including:
- Complete setup methodology and tool configuration tracking
- Monitoring infrastructure deployment results and validation metrics
- Memory operations for monitoring pattern capture and learning
- Alerting configuration effectiveness and observability optimization

**Next Commands**: Enhanced monitoring patterns will automatically improve commands like `project-health-check`, `performance-analysis`, `debug-error`, and `optimize-bundle-size`.


