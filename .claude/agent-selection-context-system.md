# Agent Selection Context System

## Overview
Intelligent system for matching commands to optimal agents and tools based on context analysis, domain expertise requirements, and project characteristics.

## Context Analysis Framework

### 1. Command Context Extraction
```yaml
context-extractors:
  # Explicit context from command
  command-category: string          # From YAML frontmatter
  command-tags: array[string]       # From YAML frontmatter
  command-description: string       # From YAML frontmatter
  allowed-tools: array[string]      # From YAML frontmatter
  
  # Implicit context from content
  domain-keywords:                  # Extracted from command content
    - rust: ["rust", "cargo", "async", "tokio", "serde"]
    - performance: ["performance", "profiling", "benchmark", "optimization"]
    - testing: ["test", "mock", "assert", "coverage", "validation"]
    - hardware: ["serial", "arduino", "device", "transport", "protocol"]
    - ui: ["egui", "gui", "rendering", "widget", "layout"]
    - architecture: ["design", "pattern", "structure", "system"]
    - documentation: ["docs", "readme", "guide", "documentation"]
    
  # Project context detection
  project-type:                     # Auto-detected from codebase
    detection-patterns:
      rust: ["Cargo.toml", "src/main.rs", "*.rs"]
      javascript: ["package.json", "*.js", "*.ts"]
      python: ["requirements.txt", "*.py", "__init__.py"]
      
  current-branch: string           # Git branch context
  recent-commits: array[string]    # Recent commit messages
  active-files: array[string]      # Recently modified files
```

### 2. Agent Capability Mapping
```yaml
agent-capabilities:
  task-orchestrator:
    domains: ["task-management", "project-coordination", "parallel-execution"]
    expertise: ["complex-workflows", "dependency-management", "resource-coordination"]
    tools: ["mcp__taskmaster-ai__*"]
    confidence-scores:
      task-management: 0.95
      project-coordination: 0.90
      parallel-execution: 0.85
      
  task-executor:
    domains: ["implementation", "execution", "specific-tasks"]
    expertise: ["focused-implementation", "single-task-completion"]
    tools: ["mcp__taskmaster-ai__*", "mcp__desktop-commander__*"]
    confidence-scores:
      implementation: 0.90
      task-completion: 0.85
      
  serial-comm-specialist:
    domains: ["hardware", "serial-communication", "device-integration"]
    expertise: ["arduino", "microcontrollers", "protocols", "transport-layers"]
    tools: ["mcp__desktop-commander__*", "mcp__FileScopeMCP__*"]
    confidence-scores:
      hardware: 0.95
      serial-communication: 0.95
      device-integration: 0.90
      rust-async: 0.80
      
  rust-performance-monitor:
    domains: ["performance", "monitoring", "profiling", "optimization"]
    expertise: ["rust-performance", "system-metrics", "bottleneck-analysis"]
    tools: ["mcp__desktop-commander__*", "mcp__FileScopeMCP__*"]
    confidence-scores:
      performance: 0.95
      rust-optimization: 0.90
      monitoring: 0.85
      
  egui-performance-optimizer:
    domains: ["ui", "egui", "rendering", "gui-performance"]
    expertise: ["immediate-mode-gui", "rendering-optimization", "widget-performance"]
    tools: ["mcp__desktop-commander__*", "mcp__FileScopeMCP__*"]
    confidence-scores:
      ui-performance: 0.95
      egui: 0.95
      rendering: 0.90
      
  mock-test-orchestrator:
    domains: ["testing", "mocking", "validation", "simulation"]
    expertise: ["test-automation", "mock-frameworks", "hardware-simulation"]
    tools: ["mcp__desktop-commander__*", "mcp__taskmaster-ai__*"]
    confidence-scores:
      testing: 0.95
      mocking: 0.90
      simulation: 0.85
      
  general-purpose:
    domains: ["architecture", "design", "documentation", "research"]
    expertise: ["problem-solving", "analysis", "documentation", "research"]
    tools: ["mcp__context7__*", "mcp__perplexity-ask__*", "mcp__clear-thought__*"]
    confidence-scores:
      general: 1.0
      research: 0.85
      documentation: 0.80
```

## Selection Algorithm

### 3. Context-Agent Matching Algorithm
```typescript
interface AgentSelectionContext {
  commandCategory: string;
  domainHints: string[];
  complexityLevel: 'low' | 'medium' | 'high' | 'expert';
  projectType: string;
  requiresParallel: boolean;
  toolRequirements: string[];
}

interface AgentScore {
  agentId: string;
  confidenceScore: number;
  domainMatch: number;
  toolCompatibility: number;
  contextRelevance: number;
  totalScore: number;
}

function selectOptimalAgent(context: AgentSelectionContext): AgentSelection {
  const candidates = getAvailableAgents();
  const scores: AgentScore[] = [];
  
  for (const agent of candidates) {
    const score = calculateAgentScore(agent, context);
    scores.push(score);
  }
  
  // Sort by total score descending
  scores.sort((a, b) => b.totalScore - a.totalScore);
  
  return {
    primary: scores[0].agentId,
    fallback: scores.slice(1, 3).map(s => s.agentId),
    reasoning: generateSelectionReasoning(scores[0], context)
  };
}

function calculateAgentScore(agent: Agent, context: AgentSelectionContext): AgentScore {
  // Domain expertise match (40% weight)
  const domainMatch = calculateDomainMatch(agent.domains, context.domainHints);
  
  // Tool compatibility (30% weight)
  const toolCompatibility = calculateToolCompatibility(agent.tools, context.toolRequirements);
  
  // Context relevance (20% weight)  
  const contextRelevance = calculateContextRelevance(agent, context);
  
  // Agent availability (10% weight)
  const availability = checkAgentAvailability(agent.id);
  
  const totalScore = 
    (domainMatch * 0.4) + 
    (toolCompatibility * 0.3) + 
    (contextRelevance * 0.2) + 
    (availability * 0.1);
    
  return {
    agentId: agent.id,
    confidenceScore: agent.confidenceScores[context.primaryDomain] || 0.5,
    domainMatch,
    toolCompatibility,
    contextRelevance,
    totalScore
  };
}
```

### 4. Selection Heuristics
```yaml
selection-heuristics:
  # Direct category mapping
  category-rules:
    "task-management": 
      primary: "task-orchestrator"
      confidence: 0.9
      
    "performance-optimization":
      conditions:
        - if: "project-type:rust"
          then: "rust-performance-monitor"
        - if: "domain:ui OR domain:egui"
          then: "egui-performance-optimizer"  
        - else: "general-purpose"
      
    "testing-quality":
      conditions:
        - if: "contains:mock OR contains:simulation"
          then: "mock-test-orchestrator"
        - if: "project-type:rust AND contains:build"
          then: "cargo-build-engineer"
        - else: "general-purpose"
  
  # Domain-specific rules  
  domain-rules:
    hardware_communication:
      keywords: ["serial", "arduino", "device", "transport", "protocol"]
      agent: "serial-comm-specialist"
      confidence: 0.95
      
    async_performance:
      keywords: ["async", "tokio", "performance", "concurrency"]
      conditions:
        - if: "project-type:rust"
          then: "rust-async-specialist"
        - else: "general-purpose"
    
    ui_performance:
      keywords: ["egui", "gui", "rendering", "widget", "ui"]
      agent: "egui-performance-optimizer"
      confidence: 0.90
      
  # Complexity-based rules
  complexity-rules:
    expert_level:
      criteria: ["complexity:expert", "requires:research", "parallel:true"]
      approach: "multi-agent"
      primary: "task-orchestrator"
      secondary: ["general-purpose", "domain-specialist"]
      
    high_complexity:
      criteria: ["complexity:high", "domains:multiple"]  
      primary: "task-orchestrator"
      tools: "research-heavy"
      
    low_complexity:
      criteria: ["complexity:low", "single-domain"]
      primary: "most-specific-domain-agent"
      fallback: "general-purpose"
```

## Context Detection Implementation

### 5. Project Context Detection
```yaml
context-detection:
  file-system-analysis:
    rust-project:
      required: ["Cargo.toml"]
      optional: ["src/", "tests/", ".cargo/"]
      confidence: 0.95
      
    node-project:
      required: ["package.json"]
      optional: ["node_modules/", "src/", "dist/"]
      confidence: 0.90
      
    multi-controller-app:
      markers: 
        - "src/transport/"
        - "src/drivers/"
        - "egui"
        - "serialport"
      context:
        type: "rust-hardware-gui"
        domains: ["hardware", "serial", "ui", "performance"]
        specialized-agents: ["serial-comm-specialist", "egui-performance-optimizer"]
        
  git-context:
    branch-analysis:
      patterns:
        - "feature/*": { context: "implementation", agent-preference: "task-executor" }
        - "performance/*": { context: "optimization", agent-preference: "rust-performance-monitor" }
        - "test/*": { context: "testing", agent-preference: "mock-test-orchestrator" }
        - "docs/*": { context: "documentation", agent-preference: "general-purpose" }
        
    commit-history:
      keywords-analysis:
        - performance: ["optimize", "perf", "speed", "benchmark"]
        - testing: ["test", "mock", "coverage", "validation"]  
        - hardware: ["serial", "device", "transport", "protocol"]
        - ui: ["ui", "gui", "egui", "widget", "render"]
```

### 6. Dynamic Context Updates
```yaml
dynamic-context:
  session-learning:
    track-agent-performance:
      success-rate: number        # Track successful completions
      user-satisfaction: number   # Implicit feedback
      execution-time: number      # Performance metrics
      
    adapt-selections:
      learning-rate: 0.1
      confidence-threshold: 0.7
      fallback-trigger: 0.5
      
  real-time-context:
    active-files: array[string]   # Currently open/modified files
    recent-errors: array[string]  # Recent error messages
    current-task: string          # Active Task Master task
    time-context: string          # Time of day, deadline pressure
    
  context-caching:
    cache-duration: "1 hour"
    invalidation-triggers:
      - file-system-changes
      - git-branch-changes  
      - new-dependencies
      - agent-availability-changes
```

## Integration Points

### 7. Command Parser Integration
```yaml
parser-integration:
  enhanced-frontmatter-processing:
    - extract-explicit-agent-preferences
    - detect-domain-hints-from-content
    - analyze-tool-requirements
    - determine-complexity-indicators
    
  context-enrichment:
    - project-type-detection
    - git-context-analysis
    - file-system-scanning
    - recent-activity-analysis
    
  agent-selection-pipeline:
    1. extract-command-context
    2. analyze-project-environment
    3. match-agent-capabilities
    4. calculate-selection-scores
    5. validate-agent-availability
    6. generate-fallback-chain
    7. prepare-tool-configuration
```

### 8. Performance & Monitoring
```yaml
performance-monitoring:
  selection-metrics:
    - selection-time: target 200ms
    - accuracy-rate: target >85%
    - user-override-rate: target <15%
    - agent-availability-rate: target >95%
    
  optimization:
    - context-caching
    - agent-preloading
    - selection-memoization
    - parallel-capability-checking
    
  feedback-loops:
    - success-rate-tracking
    - execution-time-monitoring
    - user-satisfaction-signals
    - agent-performance-analytics
```

This context system provides intelligent, adaptive agent selection that improves over time while maintaining predictable behavior and robust fallback mechanisms.