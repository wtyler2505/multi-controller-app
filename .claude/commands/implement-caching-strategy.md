---
model: claude-sonnet-4-20250514
category: performance-optimization
priority: high
tags: ["performance-optimization"]
description: Implement Caching Strategy
allowed-tools: Read, Bash, Grep, Glob
argument-hint: [cache-type] | --browser | --application | --database

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["caching-strategies", "performance-optimization", "system-architecture"]
    complexity-factors: ["multi-layer-caching", "cache-invalidation", "performance-tuning"]
    specialized-tools: ["performance-analysis", "caching-implementation", "optimization-techniques"]
  preferred-agents:
    primary: "performance-optimizer"
    secondary: "general-purpose"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["context7", "perplexity-ask", "cipher-memory"]
    specialized-functions: ["performance-optimization", "caching-strategies"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "caching-strategies + performance-optimization + system-architecture"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "caching-patterns + performance-knowledge"
    
    knowledge-preparation:
      - domain: "performance-optimization"
      - pattern-search: "caching-strategies + optimization-techniques + performance-patterns"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["caching-analysis", "strategy-implementation", "performance-optimization"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "caching-approaches + optimization-strategies + architecture-decisions"
      - pattern-recognition: "caching-implementation-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["caching-implementations", "performance-insights", "optimization-techniques"]
      - knowledge-extraction: "caching-methodologies + performance-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["caching-relationships", "performance-dependencies", "optimization-connections"]
      - cross-reference: "related-performance-strategies"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "performance-knowledge + caching-patterns"
      - continuous-learning: "caching-strategy-optimization"

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
    - caching-analysis
    - strategy-implementation
    - performance-optimization
    - validation-testing
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "implement-caching-strategy"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "caching-implementation-results + performance-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["caching-patterns", "performance-techniques", "optimization-strategies"]
  learn-from: ["optimize-bundle-size", "setup-monitoring-observability", "system-behavior-simulator"]
  contribute-to: "performance-optimization-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-performance-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-caching-analysis
    - continuous-memory-updates
    - real-time-performance-tracking
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - optimization-pattern-extraction
---

# Implement Caching Strategy

Design and implement caching solutions: **$ARGUMENTS**

## Instructions

1. **Caching Strategy Analysis**
   - Analyze application architecture and identify caching opportunities
   - Assess current performance bottlenecks and data access patterns
   - Define caching requirements (TTL, invalidation, consistency)
   - Plan multi-layer caching architecture (browser, CDN, application, database)
   - Evaluate caching technologies and storage solutions

2. **Browser and Client-Side Caching**
   - Configure HTTP caching headers and cache policies for static assets
   - Implement service worker caching strategies for progressive web apps
   - Set up browser storage caching (localStorage, sessionStorage, IndexedDB)
   - Configure CDN caching rules and edge optimization
   - Implement cache-first, network-first, and stale-while-revalidate strategies

3. **Application-Level Caching**
   - Implement in-memory caching for frequently accessed data
   - Set up distributed caching with Redis or Memcached
   - Design cache key naming conventions and namespacing
   - Implement cache warming strategies for critical data
   - Configure cache expiration and TTL policies

4. **Database Query Caching**
   - Implement query result caching for expensive database operations
   - Set up prepared statement caching and connection pooling
   - Design cache invalidation strategies for data consistency
   - Implement materialized views for complex aggregations
   - Configure database-level caching features and optimizations

5. **API Response Caching**
   - Implement API endpoint response caching with appropriate headers
   - Set up middleware for automatic response caching
   - Configure GraphQL query caching and field-level optimization
   - Implement conditional requests with ETag and Last-Modified headers
   - Design cache invalidation for API data updates

6. **Cache Invalidation Strategies**
   - Design intelligent cache invalidation based on data dependencies
   - Implement event-driven cache invalidation systems
   - Set up cache tagging and bulk invalidation mechanisms
   - Configure time-based and trigger-based invalidation policies
   - Implement cache versioning and rollback strategies

7. **Frontend Caching Strategies**
   - Implement client-side data caching with libraries like React Query
   - Set up component-level caching and memoization
   - Configure asset bundling and chunk caching strategies
   - Implement progressive image loading and caching
   - Set up offline-first caching for PWAs

8. **Cache Monitoring and Analytics**
   - Set up cache performance monitoring and metrics collection
   - Track cache hit rates, miss rates, and efficiency metrics
   - Monitor cache memory usage and storage optimization
   - Implement cache performance alerting and notifications
   - Analyze cache usage patterns and optimization opportunities

9. **Cache Warming and Preloading**
   - Implement automated cache warming for critical data
   - Set up scheduled cache refresh and preloading strategies
   - Design on-demand cache generation for popular content
   - Configure cache warming triggers based on usage patterns
   - Implement predictive caching based on user behavior

10. **Testing and Validation**
    - Set up cache performance testing and benchmarking
    - Implement cache consistency validation and testing
    - Configure cache invalidation testing scenarios
    - Test cache behavior under high load and failure conditions
    - Validate cache security and data isolation requirements

Focus on implementing caching strategies that provide the most significant performance improvements while maintaining data consistency and system reliability. Always measure cache effectiveness and adjust strategies based on real-world usage patterns.

