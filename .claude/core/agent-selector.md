# Agent Selector - Context-Aware Intelligence Engine

## Overview
The Agent Selector is the core intelligence engine for Phase 1B context-aware command enhancement. It analyzes command context, project state, and execution environment to dynamically select optimal agents and tool chains.

## Architecture

### Core Selection Algorithm
```typescript
interface SelectionContext {
  commandName: string;
  arguments?: string;
  projectType: 'rust' | 'typescript' | 'mixed';
  currentFiles?: string[];
  errorContext?: string;
  complexity: 'simple' | 'medium' | 'complex';
  previousExecutions?: ExecutionHistory[];
  userPreferences?: AgentPreference[];
}

interface AgentAssignment {
  primaryAgent: string;
  confidence: number; // 0-1 scale
  fallbackAgents: string[];
  toolChain: string;
  requiredTools: string[];
  reasoning: string;
  selectionCriteria: SelectionCriteria;
}

class AgentSelector {
  private domainClassifier: DomainClassifier;
  private complexityAnalyzer: ComplexityAnalyzer;
  private toolChainOptimizer: ToolChainOptimizer;
  private learningSystem: LearningSystem;

  async selectAgent(context: SelectionContext): Promise<AgentAssignment> {
    // Multi-phase analysis
    const domainAnalysis = await this.analyzeDomain(context);
    const complexityAnalysis = await this.analyzeComplexity(context);
    const projectAnalysis = await this.analyzeProject(context);
    const historyAnalysis = await this.analyzeHistory(context);

    // Candidate scoring
    const candidates = await this.scoreAgents(
      domainAnalysis, 
      complexityAnalysis, 
      projectAnalysis, 
      historyAnalysis
    );

    // Tool chain optimization
    const optimalToolChain = await this.selectToolChain(candidates[0], context);

    return {
      primaryAgent: candidates[0].agent,
      confidence: candidates[0].score,
      fallbackAgents: candidates.slice(1, 3).map(c => c.agent),
      toolChain: optimalToolChain.name,
      requiredTools: optimalToolChain.tools,
      reasoning: this.generateReasoning(candidates[0], domainAnalysis, complexityAnalysis),
      selectionCriteria: {
        domainMatch: domainAnalysis.strength,
        complexityFit: complexityAnalysis.level,
        projectContext: projectAnalysis.relevance,
        historicalSuccess: historyAnalysis.successRate
      }
    };
  }

  private async analyzeDomain(context: SelectionContext): Promise<DomainAnalysis> {
    return this.domainClassifier.classify(context.commandName, context.arguments);
  }
}
```

### Domain Classification System
```typescript
class DomainClassifier {
  private static DOMAIN_PATTERNS = {
    'rust-development': {
      keywords: ['rust', 'cargo', 'clippy', 'fmt', 'build', 'crate'],
      agents: ['rust-async-specialist', 'cargo-build-engineer', 'rust-performance-monitor'],
      strength: 0.9,
      toolChains: ['rust-development', 'performance-analysis']
    },
    
    'serial-hardware': {
      keywords: ['serial', 'arduino', 'hardware', 'device', 'transport', 'connection'],
      agents: ['serial-hardware-specialist', 'serial-comm-specialist'],
      strength: 0.95,
      toolChains: ['hardware-integration', 'transport-development']
    },
    
    'testing-validation': {
      keywords: ['test', 'mock', 'coverage', 'spec', 'validate', 'verify'],
      agents: ['mock-test-orchestrator', 'cargo-build-engineer'],
      strength: 0.85,
      toolChains: ['testing-validation', 'rust-development']
    },
    
    'performance-optimization': {
      keywords: ['performance', 'benchmark', 'profile', 'monitor', 'optimize', 'cpu', 'memory'],
      agents: ['rust-performance-monitor', 'egui-performance-optimizer'],
      strength: 0.9,
      toolChains: ['performance-analysis', 'monitoring-systems']
    },
    
    'ui-interface': {
      keywords: ['ui', 'egui', 'gui', 'interface', 'chart', 'visualization', 'rendering'],
      agents: ['egui-performance-optimizer', 'ui-controls-architect', 'visualization-engineer'],
      strength: 0.88,
      toolChains: ['ui-development', 'visualization-systems']
    },
    
    'security-safety': {
      keywords: ['security', 'auth', 'credential', 'encrypt', 'safety', 'emergency', 'watchdog'],
      agents: ['rust-security-coordinator', 'rust-safety-coordinator'],
      strength: 0.92,
      toolChains: ['security-implementation', 'safety-systems']
    },
    
    'task-management': {
      keywords: ['task', 'todo', 'project', 'workflow', 'epic', 'status'],
      agents: ['task-orchestrator', 'task-executor', 'task-checker'],
      strength: 0.85,
      toolChains: ['task-coordination', 'project-management']
    },
    
    'architecture-design': {
      keywords: ['architecture', 'design', 'system', 'model', 'structure', 'pattern'],
      agents: ['general-purpose'], // Enhanced with Clear-Thought tools
      strength: 0.75,
      toolChains: ['research-heavy', 'architecture-analysis']
    }
  };

  classify(commandName: string, arguments?: string): DomainAnalysis {
    const text = `${commandName} ${arguments || ''}`.toLowerCase();
    const matches: DomainMatch[] = [];

    for (const [domain, pattern] of Object.entries(this.DOMAIN_PATTERNS)) {
      const keywordMatches = pattern.keywords.filter(keyword => 
        text.includes(keyword.toLowerCase())
      );
      
      if (keywordMatches.length > 0) {
        const score = (keywordMatches.length / pattern.keywords.length) * pattern.strength;
        matches.push({
          domain,
          score,
          matchedKeywords: keywordMatches,
          agents: pattern.agents,
          toolChains: pattern.toolChains
        });
      }
    }

    // Sort by score and return top matches
    matches.sort((a, b) => b.score - a.score);
    
    return {
      primaryDomain: matches[0] || null,
      alternativeDomains: matches.slice(1, 3),
      confidence: matches[0]?.score || 0,
      strength: this.calculateOverallStrength(matches)
    };
  }
}
```

### Complexity Analysis Engine
```typescript
class ComplexityAnalyzer {
  private static COMPLEXITY_INDICATORS = {
    simple: {
      patterns: ['show', 'list', 'get', 'view', 'display'],
      characteristics: ['read-only', 'single-operation', 'no-dependencies'],
      maxTools: 2,
      estimatedTime: 30 // seconds
    },
    
    medium: {
      patterns: ['setup', 'configure', 'update', 'generate', 'create'],
      characteristics: ['multi-step', 'file-modification', 'moderate-dependencies'],
      maxTools: 5,
      estimatedTime: 120 // seconds
    },
    
    complex: {
      patterns: ['implement', 'orchestrate', 'analyze', 'optimize', 'debug'],
      characteristics: ['multi-system', 'heavy-analysis', 'high-dependencies'],
      maxTools: 10,
      estimatedTime: 300 // seconds
    }
  };

  analyzeComplexity(context: SelectionContext): ComplexityAnalysis {
    const commandName = context.commandName.toLowerCase();
    const hasArguments = !!context.arguments;
    const argumentComplexity = this.analyzeArguments(context.arguments);
    
    // Pattern-based analysis
    let patternScore = 0;
    let detectedLevel = 'simple';
    
    for (const [level, indicators] of Object.entries(this.COMPLEXITY_INDICATORS)) {
      const patternMatches = indicators.patterns.filter(pattern => 
        commandName.includes(pattern)
      );
      
      if (patternMatches.length > 0) {
        const score = (patternMatches.length / indicators.patterns.length) * 
                     (level === 'complex' ? 3 : level === 'medium' ? 2 : 1);
        
        if (score > patternScore) {
          patternScore = score;
          detectedLevel = level;
        }
      }
    }
    
    // Adjust for context
    if (hasArguments && argumentComplexity > 0.5) {
      detectedLevel = this.elevateComplexity(detectedLevel);
    }
    
    if (context.errorContext) {
      detectedLevel = 'complex'; // Error scenarios are inherently complex
    }
    
    return {
      level: detectedLevel as 'simple' | 'medium' | 'complex',
      confidence: patternScore,
      factors: {
        patternMatch: patternScore,
        argumentComplexity,
        hasErrorContext: !!context.errorContext,
        estimatedDuration: this.COMPLEXITY_INDICATORS[detectedLevel].estimatedTime
      },
      recommendations: {
        maxTools: this.COMPLEXITY_INDICATORS[detectedLevel].maxTools,
        parallelExecution: detectedLevel === 'complex',
        requiresFallback: detectedLevel !== 'simple'
      }
    };
  }
}
```

### Agent Scoring Algorithm
```typescript
class AgentScorer {
  private static AGENT_CAPABILITIES = {
    'task-orchestrator': {
      domains: ['task-management', 'workflow-coordination', 'project-orchestration'],
      complexity: ['medium', 'complex'],
      specializations: ['parallel-execution', 'dependency-management', 'multi-agent-coordination'],
      tools: ['taskmaster-ai', 'desktop-commander', 'cipher-memory'],
      baseScore: 0.85
    },
    
    'task-executor': {
      domains: ['task-management', 'implementation'],
      complexity: ['simple', 'medium'],
      specializations: ['focused-implementation', 'task-completion', 'progress-tracking'],
      tools: ['taskmaster-ai', 'FileScopeMCP', 'cipher-memory'],
      baseScore: 0.80
    },
    
    'rust-performance-monitor': {
      domains: ['performance-optimization', 'rust-development', 'monitoring'],
      complexity: ['medium', 'complex'],
      specializations: ['performance-analysis', 'cpu-monitoring', 'memory-profiling'],
      tools: ['desktop-commander', 'FileScopeMCP', 'context7'],
      baseScore: 0.90
    },
    
    'serial-hardware-specialist': {
      domains: ['serial-hardware', 'device-communication', 'transport-layer'],
      complexity: ['medium', 'complex'],
      specializations: ['arduino-integration', 'serial-protocols', 'hardware-debugging'],
      tools: ['desktop-commander', 'context7', 'cipher-memory'],
      baseScore: 0.95
    },
    
    'mock-test-orchestrator': {
      domains: ['testing-validation', 'test-automation', 'quality-assurance'],
      complexity: ['medium', 'complex'],
      specializations: ['mock-frameworks', 'test-generation', 'coverage-analysis'],
      tools: ['desktop-commander', 'FileScopeMCP', 'context7'],
      baseScore: 0.88
    },
    
    'general-purpose': {
      domains: ['*'], // Universal fallback
      complexity: ['simple', 'medium', 'complex'],
      specializations: ['research', 'analysis', 'clear-thought-integration'],
      tools: ['clear-thought', 'cipher-memory', 'context7', 'perplexity-ask'],
      baseScore: 0.70 // Lower base score, higher adaptability
    }
  };

  scoreAgent(agentId: string, domainAnalysis: DomainAnalysis, complexityAnalysis: ComplexityAnalysis, projectContext: ProjectContext): AgentScore {
    const capabilities = this.AGENT_CAPABILITIES[agentId];
    if (!capabilities) return { agent: agentId, score: 0, reasoning: 'Unknown agent' };
    
    let score = capabilities.baseScore;
    const reasons: string[] = [];
    
    // Domain matching
    if (domainAnalysis.primaryDomain) {
      const domainMatch = capabilities.domains.includes(domainAnalysis.primaryDomain.domain) || 
                         capabilities.domains.includes('*');
      if (domainMatch) {
        score += 0.15;
        reasons.push(`Strong domain match: ${domainAnalysis.primaryDomain.domain}`);
      }
    }
    
    // Complexity alignment
    const complexityMatch = capabilities.complexity.includes(complexityAnalysis.level);
    if (complexityMatch) {
      score += 0.10;
      reasons.push(`Complexity alignment: ${complexityAnalysis.level}`);
    } else {
      score -= 0.05;
      reasons.push(`Complexity mismatch: requires ${complexityAnalysis.level}`);
    }
    
    // Specialization bonus
    const projectSpecializations = this.detectRequiredSpecializations(projectContext);
    const matchingSpecializations = capabilities.specializations.filter(spec => 
      projectSpecializations.includes(spec)
    );
    
    if (matchingSpecializations.length > 0) {
      score += matchingSpecializations.length * 0.05;
      reasons.push(`Specialization match: ${matchingSpecializations.join(', ')}`);
    }
    
    // Tool availability bonus
    const availableTools = projectContext.availableTools || [];
    const agentTools = capabilities.tools;
    const toolAvailability = agentTools.filter(tool => 
      availableTools.some(available => available.includes(tool))
    ).length / agentTools.length;
    
    score += toolAvailability * 0.10;
    if (toolAvailability > 0.8) {
      reasons.push(`High tool availability: ${Math.round(toolAvailability * 100)}%`);
    }
    
    return {
      agent: agentId,
      score: Math.min(score, 1.0), // Cap at 1.0
      reasoning: reasons.join('; '),
      details: {
        baseScore: capabilities.baseScore,
        domainBonus: domainMatch ? 0.15 : 0,
        complexityBonus: complexityMatch ? 0.10 : -0.05,
        specializationBonus: matchingSpecializations.length * 0.05,
        toolBonus: toolAvailability * 0.10
      }
    };
  }
}
```

## Integration Points

### Command Enhancement Pattern
```yaml
# Template for Tier 2 commands
---
model: claude-sonnet-4-20250514
category: [existing-category]
priority: [existing-priority]
tags: [existing-tags]
description: [existing-description]

# Phase 1B Context-Aware Enhancement
agent-selection:
  type: "context-aware"
  domain-hints: ["rust", "testing"]  # Detected domains
  complexity-level: "medium"         # Analyzed complexity
  
  # Selection scoring criteria
  selection-criteria:
    keyword-match: 0.8     # Domain keyword matching strength
    argument-analysis: 0.7  # Argument complexity assessment
    project-context: 0.9    # Project type and files relevance
    error-context: 0.0      # Error context availability (if applicable)
  
  # Agent preferences and fallbacks
  preferred-agents: ["rust-performance-monitor", "mock-test-orchestrator"]
  fallback-agents: ["general-purpose", "cargo-build-engineer"]
  confidence-threshold: 0.75  # Minimum confidence for primary agent

# Dynamic tool selection
tool-selection:
  type: "context-driven"
  
  # Always available tools
  base-tools:
    - "mcp__desktop-commander__start_process"
    - "mcp__FileScopeMCP__find_important_files"
  
  # Conditional tool chains
  conditional-tools:
    high-confidence: 
      rust-context: ["mcp__context7__get-library-docs", "mcp__desktop-commander__start_process"]
      testing-context: ["mcp__desktop-commander__start_process", "mcp__FileScopeMCP__recalculate_importance"]
    
    medium-confidence:
      general-context: ["mcp__cipher-memory__search_nodes", "mcp__desktop-commander__start_process"]
      
    low-confidence:
      fallback-tools: ["mcp__desktop-commander__start_process"]

# Enhanced workflow configuration  
pre-execution:
  validate-tools: true
  load-context: true
  analyze-arguments: true
  detect-project-state: true
  
post-execution:
  store-results: true
  update-learning: true    # Feed success/failure back to selection engine
  log-performance: true    # Track execution metrics
---
```

## Validation and Testing

### Testing Strategy
1. **Unit Tests**: Test individual components (domain classification, complexity analysis)
2. **Integration Tests**: Test end-to-end selection pipeline
3. **A/B Testing**: Compare context-aware selection vs manual/random selection
4. **Performance Testing**: Measure selection speed and accuracy
5. **User Acceptance Testing**: Validate real-world usage scenarios

### Success Metrics
- **Selection Accuracy**: >85% optimal agent selection on first attempt
- **Selection Speed**: <5s for complex scenarios, <2s for simple scenarios  
- **Fallback Success**: >95% successful execution with fallback agents
- **User Satisfaction**: Seamless experience with minimal manual intervention

### Learning and Optimization
```typescript
class LearningSystem {
  async recordExecution(selection: AgentAssignment, result: ExecutionResult) {
    // Record success/failure for learning
    await this.storage.recordOutcome({
      selectionCriteria: selection.selectionCriteria,
      primaryAgent: selection.primaryAgent,
      success: result.success,
      executionTime: result.duration,
      userSatisfaction: result.userFeedback?.satisfaction,
      timestamp: Date.now()
    });
    
    // Update selection weights
    await this.updateSelectionWeights(selection, result);
  }
  
  async optimizeSelection(context: SelectionContext): Promise<SelectionOptimization> {
    const historicalData = await this.getHistoricalData(context);
    const patterns = this.analyzePatterns(historicalData);
    
    return {
      adjustedWeights: this.calculateOptimalWeights(patterns),
      confidenceAdjustment: this.calculateConfidenceAdjustment(patterns),
      recommendedTools: this.optimizeToolSelection(patterns)
    };
  }
}
```

## Implementation Phases

### Phase 1B.1: Core Engine (Week 1)
- [ ] Implement `AgentSelector` class with basic selection logic
- [ ] Create `DomainClassifier` with keyword-based classification
- [ ] Build `ComplexityAnalyzer` with pattern recognition
- [ ] Develop `AgentScorer` with multi-factor scoring

### Phase 1B.2: Integration (Week 2)  
- [ ] Create command enhancement templates
- [ ] Integrate with existing YAML frontmatter system
- [ ] Implement tool chain optimization
- [ ] Add fallback and error handling

### Phase 1B.3: Learning System (Week 3)
- [ ] Build execution tracking and learning system
- [ ] Implement performance monitoring and optimization
- [ ] Create validation and testing framework
- [ ] Add user feedback collection mechanisms

## Usage Examples

### Example 1: Rust Performance Command
```bash
Input: /project:optimize-performance "CPU usage monitoring"

Context Analysis:
- Domain: performance-optimization (score: 0.9)
- Complexity: medium (pattern: optimize)
- Project: rust-egui

Agent Selection:
- Primary: rust-performance-monitor (score: 0.92)
- Fallback: [general-purpose, egui-performance-optimizer]
- Tools: [desktop-commander, FileScopeMCP, context7]
- Reasoning: "Strong performance domain match, Rust project context, medium complexity fits agent capabilities"
```

### Example 2: Testing Setup Command
```bash
Input: /project:setup-comprehensive-testing "mock Arduino drivers"

Context Analysis:
- Domain: testing-validation (score: 0.85)
- Complexity: complex (pattern: setup + comprehensive)
- Project: rust-hardware

Agent Selection:
- Primary: mock-test-orchestrator (score: 0.88)
- Fallback: [cargo-build-engineer, general-purpose]
- Tools: [desktop-commander, FileScopeMCP, context7, cipher-memory]
- Reasoning: "Testing domain match, complex setup requirements, hardware context suggests mock frameworks"
```

This context-aware selection system transforms static slash commands into intelligent, adaptive automation tools that understand the user's intent and project context to select optimal execution strategies.