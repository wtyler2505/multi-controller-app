---
model: claude-sonnet-4-20250514
category: performance-optimization
priority: high
tags: ["performance-optimization", "performance"]
description: Add comprehensive performance monitoring, profiling, and metrics collection to your application
allowed-tools: Read, Bash, Grep, Glob
argument-hint: [monitoring-type] | --apm | --rum | --custom

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["performance-monitoring", "profiling-analysis", "metrics-collection"]
    complexity-factors: ["monitoring-scope", "performance-requirements", "integration-complexity"]
    specialized-tools: ["performance-analysis", "monitoring-setup", "metrics-optimization"]
  preferred-agents:
    primary: "rust-performance-monitor"
    secondary: "general-purpose"
    fallback: ["performance-optimizer"]
  tool-requirements:
    mcp-servers: ["desktop-commander", "FileScopeMCP", "cipher-memory", "context7"]
    specialized-functions: ["performance-monitoring", "profiling-analysis"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "performance-monitoring + profiling-analysis + metrics-collection"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "performance-patterns + monitoring-knowledge"
    
    knowledge-preparation:
      - domain: "performance-monitoring"
      - pattern-search: "monitoring-strategies + profiling-patterns + metrics-optimization"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["monitoring-setup", "profiling-configuration", "metrics-implementation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "monitoring-strategies + profiling-approaches + metrics-decisions"
      - pattern-recognition: "performance-monitoring-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["monitoring-results", "profiling-insights", "metrics-techniques"]
      - knowledge-extraction: "performance-methodologies + monitoring-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["monitoring-relationships", "profiling-dependencies", "metrics-connections"]
      - cross-reference: "related-performance-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "performance-knowledge + monitoring-patterns"
      - continuous-learning: "performance-monitoring-optimization"

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
    - monitoring-setup
    - profiling-configuration
    - metrics-implementation
    - performance-analysis
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "add-performance-monitoring"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "monitoring-results + profiling-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["monitoring-patterns", "profiling-techniques", "metrics-methodologies"]
  learn-from: ["rust-performance-monitor", "setup-monitoring-observability", "test-quality-analyzer"]
  contribute-to: "performance-monitoring-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-monitoring-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-monitoring-setup
    - continuous-memory-updates
    - real-time-performance-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - monitoring-pattern-extraction
---

# Add Performance Monitoring

Setup comprehensive application performance monitoring with intelligent profiling and advanced metrics collection: **$ARGUMENTS**

## Instructions

1. **Performance Monitoring Strategy**
   - Define key performance indicators (KPIs) and service level objectives (SLOs)
   - Identify critical user journeys and performance bottlenecks
   - Plan monitoring architecture and data collection strategy
   - Assess existing monitoring infrastructure and integration points
   - Define alerting thresholds and escalation procedures

2. **Application Performance Monitoring (APM)**
   - Set up comprehensive APM solution (New Relic, Datadog, AppDynamics)
   - Configure distributed tracing for request lifecycle visibility
   - Implement custom metrics and performance tracking
   - Set up transaction monitoring and error tracking
   - Configure performance profiling and diagnostics

3. **Real User Monitoring (RUM)**
   - Implement client-side performance tracking and web vitals monitoring
   - Set up user experience metrics collection (LCP, FID, CLS, TTFB)
   - Configure custom performance metrics for user interactions
   - Monitor page load performance and resource loading
   - Track user journey performance across different devices

4. **Server Performance Monitoring**
   - Monitor system metrics (CPU, memory, disk, network)
   - Set up process and application-level monitoring
   - Configure event loop lag and garbage collection monitoring
   - Implement custom server performance metrics
   - Monitor resource utilization and capacity planning

5. **Database Performance Monitoring**
   - Track database query performance and slow query identification
   - Monitor database connection pool utilization
   - Set up database performance metrics and alerting
   - Implement query execution plan analysis
   - Monitor database resource usage and optimization opportunities

6. **Error Tracking and Monitoring**
   - Implement comprehensive error tracking (Sentry, Bugsnag, Rollbar)
   - Configure error categorization and impact analysis
   - Set up error alerting and notification systems
   - Track error trends and resolution metrics
   - Implement error context and debugging information

7. **Custom Metrics and Dashboards**
   - Implement business metrics tracking (Prometheus, StatsD)
   - Create performance dashboards and visualizations
   - Configure custom alerting rules and thresholds
   - Set up performance trend analysis and reporting
   - Implement performance regression detection

8. **Alerting and Notification System**
   - Configure intelligent alerting based on performance thresholds
   - Set up multi-channel notifications (email, Slack, PagerDuty)
   - Implement alert escalation and on-call procedures
   - Configure alert fatigue prevention and noise reduction
   - Set up performance incident management workflows

9. **Performance Testing Integration**
   - Integrate monitoring with load testing and performance testing
   - Set up continuous performance testing and monitoring
   - Configure performance baseline tracking and comparison
   - Implement performance test result analysis and reporting
   - Monitor performance under different load scenarios

10. **Performance Optimization Recommendations**
    - Generate actionable performance insights and recommendations
    - Implement automated performance analysis and reporting
    - Set up performance optimization tracking and measurement
    - Configure performance improvement validation
    - Create performance optimization prioritization frameworks

Focus on monitoring strategies that provide actionable insights for performance optimization. Ensure monitoring overhead is minimal and doesn't impact application performance.

