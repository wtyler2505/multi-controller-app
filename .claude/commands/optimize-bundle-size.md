---
model: claude-sonnet-4-20250514
category: performance-optimization
priority: high
tags: ["performance-optimization", "performance", "bundle-analysis", "build-optimization", "asset-optimization"]
description: Comprehensive bundle size optimization with universal memory integration and intelligent performance pattern recognition

# Enhanced Context-Aware Agent Integration with Universal Memory
agent-selection:
  type: "context-aware"
  domain-hints: ["bundle-optimization", "performance-analysis", "build-tools", "asset-optimization", "web-performance"]
  complexity-level: "complex"
  
  # Enhanced selection criteria for bundle optimization with memory integration
  selection-criteria:
    keyword-match: 0.90       # Strong optimization/performance patterns
    argument-analysis: 0.85   # Build tool context critical
    project-context: 0.95     # Frontend framework affects optimization approach
    error-context: 0.6        # May include build performance issues
  
  # Specialized optimization agents with memory capabilities
  preferred-agents: ["general-purpose", "rust-performance-monitor", "egui-performance-optimizer"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.85

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-bundle-optimization-workflow"
  
  base-tools:
    - "mcp__desktop-commander__start_process"  # Bundle analysis and build tools
    - "mcp__FileScopeMCP__find_important_files"  # Analyze bundle composition
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
  
  conditional-tools:
    bundle-analysis:
      - "mcp__desktop-commander__start_process"  # webpack-bundle-analyzer, etc.
      - "mcp__context7__get-library-docs"  # Build tool documentation
      - "mcp__cipher-memory__open_nodes"  # Load optimization patterns
    
    performance-monitoring:
      - "mcp__FileScopeMCP__recalculate_importance"  # Performance-critical files
      - "mcp__cipher-memory__create_entities"  # Store performance insights
      - "mcp__desktop-commander__start_process"  # Performance profiling
    
    build-optimization:
      - "mcp__cipher-memory__add_observations"  # Store optimization strategies
      - "mcp__desktop-commander__start_process"  # Build configuration optimization
      - "mcp__cipher-memory__create_relations"  # Connect optimization patterns
    
    asset-optimization:
      - "mcp__desktop-commander__start_process"  # Image/asset optimization
      - "mcp__cipher-memory__create_entities"  # Store asset optimization patterns
      - "mcp__perplexity-ask__perplexity_ask"  # Research optimization techniques

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  pre-execution-memory:
    optimization-patterns-search:
      - query-pattern: "bundle-optimization + ${build_tool}-performance + asset-optimization"
      - tools: ["mcp__cipher-memory__search_nodes"]
    build-tool-strategies:
      - query-pattern: "build-optimization + performance-tuning + ${framework}-optimization"
      - tools: ["mcp__cipher-memory__open_nodes"]
    performance-benchmarks:
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "performance-optimization-related"
  execution-memory:
    optimization-progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "optimization-technique-application"
    performance-discovery:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "effective-optimization-identified"
    bundle-learning:
      - tool: "mcp__cipher-memory__create_entities"
      - trigger: "significant-size-reduction-achieved"
  post-execution-memory:
    optimization-methodology-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-bundle-optimization-session-pattern"
    performance-pattern-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["technique-to-reduction", "tool-to-effectiveness", "optimization-to-performance"]
    knowledge-enhancement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "optimization-insights + performance-strategies + build-tool-mastery"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - bundle-analysis-scope
      - optimization-pattern-search
      - memory-pattern-analysis
    
    execution:
      - bundle-analysis-results
      - optimization-technique-application
      - build-configuration-changes
      - asset-optimization-processing
      - performance-measurement
    
    post-execution:
      - optimization-summary
      - performance-improvements
      - memory-operations
      - monitoring-recommendations
  
  structured-metadata:
    command-id: "optimize-bundle-size"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    build-tool: "${build_tool_used}"
    initial-bundle-size: "${bundle_size_before}"
    optimized-bundle-size: "${bundle_size_after}"
    size-reduction: "${size_reduction_percentage}"
    techniques-applied: "${optimization_techniques}"
    performance-gain: "${loading_time_improvement}"
    execution-time: "${duration_ms}"
    optimization-quality-score: "${optimization_effectiveness}"

# Enhanced workflow configuration
tool-chain: "universal-bundle-optimization-workflow"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
performance-pattern-recognition: true

allowed-tools: Read, Bash, Grep, Glob, mcp__desktop-commander__*, mcp__FileScopeMCP__*, mcp__context7__*, mcp__cipher-memory__*, mcp__perplexity-ask__*, mcp__taskmaster-ai__*

argument-hint: [build-tool] | --webpack | --vite | --rollup | --comprehensive | --analysis-only

pre-execution:
  validate-tools: true
  load-context: true
  analyze-build-system: true
  search-optimization-patterns: true
  log-session-start: true

post-execution:
  store-results: true
  update-learning: true
  generate-report: true
  persist-optimization-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Optimize Bundle Size (Universal Integration)

Reduce and optimize bundle sizes with universal memory integration and intelligent performance pattern recognition: **$ARGUMENTS**

**ENHANCED WORKFLOW**: This command utilizes specialized optimization agents (rust-performance-monitor, egui-performance-optimizer) with complete Cipher Memory integration for optimization pattern recognition, performance analysis, and build optimization persistence.

## Enhanced Pre-Execution Memory Analysis
Before bundle optimization, the system will:
1. **Search optimization patterns**: Query Cipher Memory for effective bundle optimization strategies and techniques
2. **Load performance benchmarks**: Retrieve performance optimization best practices and build tool configurations
3. **Analyze build strategies**: Understanding build optimization patterns and asset optimization methodologies
4. **Connect performance knowledge**: Access comprehensive performance improvement and monitoring patterns

## Instructions

1. **Bundle Analysis and Assessment**
   - Analyze current bundle size and composition using webpack-bundle-analyzer or similar tools
   - Identify large dependencies and unused code across all bundles
   - Assess current build configuration and optimization settings
   - Create baseline measurements for optimization tracking
   - Document current performance metrics and loading times

2. **Build Tool Configuration**
   - Configure build tool optimization settings for production builds
   - Enable code splitting and chunk optimization features
   - Configure tree shaking and dead code elimination
   - Set up bundle analyzers and visualization tools
   - Optimize build performance and output sizes

3. **Code Splitting and Lazy Loading**
   - Implement route-based code splitting for single-page applications
   - Set up dynamic imports for components and modules
   - Configure lazy loading for non-critical resources
   - Optimize chunk sizes and loading strategies
   - Implement progressive loading patterns

4. **Tree Shaking and Dead Code Elimination**
   - Configure build tools for optimal tree shaking
   - Mark packages as side-effect free where appropriate
   - Optimize import statements for better tree shaking
   - Use ES6 modules and avoid CommonJS where possible
   - Implement babel plugins for automatic import optimization

5. **Dependency Optimization**
   - Analyze and audit package dependencies for size impact
   - Replace large libraries with smaller alternatives
   - Use specific imports instead of importing entire libraries
   - Implement dependency deduplication strategies
   - Configure external dependencies and CDN usage

6. **Asset Optimization**
   - Optimize images through compression and format conversion
   - Implement responsive image loading strategies
   - Configure asset minification and compression
   - Set up efficient file loaders and processors
   - Optimize font loading and subsetting

7. **Module Federation and Micro-frontends**
   - Implement module federation for large applications
   - Configure shared dependencies and runtime optimization
   - Set up micro-frontend architecture for code sharing
   - Optimize remote module loading and caching
   - Implement federation performance monitoring

8. **Performance Monitoring and Measurement**
   - Set up bundle size monitoring and tracking
   - Configure automated bundle analysis in CI/CD
   - Monitor bundle size changes over time
   - Set up performance budgets and alerts
   - Track loading performance metrics

9. **Progressive Loading Strategies**
   - Implement resource hints (preload, prefetch, dns-prefetch)
   - Configure service workers for caching strategies
   - Set up intersection observer for lazy loading
   - Optimize critical resource loading priorities
   - Implement adaptive loading based on connection speed

10. **Validation and Continuous Monitoring**
    - Set up automated bundle size validation in CI/CD
    - Configure bundle size thresholds and alerts
    - Implement bundle size regression testing
    - Monitor real-world loading performance
    - Set up automated optimization recommendations

Focus on optimizations that provide the most significant bundle size reductions while maintaining application functionality. Always measure the impact of changes on both bundle size and runtime performance.

## Universal Memory Integration Outcomes

### Bundle Optimization Knowledge Storage
This command will automatically:
- **Store comprehensive optimization sessions** in Cipher Memory for bundle optimization pattern recognition
- **Create relationships** between optimization techniques, build tools, and performance improvements
- **Document optimization methodologies** and build configuration best practices
- **Build knowledge graph** of optimization-effectiveness mappings and performance improvement strategies

### Cross-Command Learning Enhancement
Bundle optimization patterns will improve:
- Future performance commands through established optimization baseline patterns
- Build configuration commands via documented build optimization strategies
- Asset management commands through optimization technique integration
- Monitoring commands via proven performance measurement methodologies

### Advanced Optimization Intelligence
- **Technique Selection**: Automatic identification of optimal optimization techniques based on project characteristics
- **Performance Prediction**: Intelligent estimation of optimization impact before implementation
- **Build Tool Optimization**: Smart build configuration recommendations based on successful optimization patterns
- **Asset Strategy**: Automated asset optimization strategies based on content type and usage patterns

### Intelligent Optimization Enhancement Features
- **Project-Specific Optimization**: Tailored optimization approaches based on framework, build tool, and project complexity
- **Context-Aware Techniques**: Smart optimization technique selection considering application requirements and constraints
- **Progressive Optimization Learning**: Each optimization session improves future optimization through pattern accumulation
- **Cross-Project Optimization Knowledge**: Shared optimization insights across different projects and build systems

### Centralized Optimization Logging
All bundle optimization operations logged to `.claude/logs/command-execution.jsonl` including:
- Complete optimization methodology and technique application tracking
- Bundle size reduction results and performance improvement metrics
- Memory operations for optimization pattern capture and learning
- Build configuration optimization effectiveness and monitoring recommendations

**Next Commands**: Enhanced optimization patterns will automatically improve commands like `performance-analysis`, `setup-monitoring-observability`, `project-health-check`, and `code-review`.

