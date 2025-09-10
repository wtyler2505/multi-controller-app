---
model: claude-sonnet-4-20250514
category: utilities-tools
priority: high
tags: ["utilities-tools", "deep-analysis"]
description: Deep Analysis and Problem Solving Mode

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["deep-analysis", "problem-solving", "strategic-reasoning"]
    complexity-factors: ["analysis-depth", "reasoning-complexity", "solution-scope"]
    specialized-tools: ["reasoning-frameworks", "analysis-tools", "cognitive-systems"]
  preferred-agents:
    primary: "general-purpose"
    secondary: "strategic-analyst"
    fallback: ["task-orchestrator"]
  tool-requirements:
    mcp-servers: ["clear-thought", "cipher-memory", "perplexity-ask"]
    specialized-functions: ["deep-analysis", "strategic-reasoning"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "deep-analysis + problem-solving + strategic-reasoning"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "analysis-patterns + reasoning-knowledge"
    
    knowledge-preparation:
      - domain: "deep-analysis"
      - pattern-search: "analysis-strategies + reasoning-patterns + problem-solving-frameworks"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["analysis-execution", "reasoning-process", "solution-development"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "analysis-strategies + reasoning-approaches + solution-decisions"
      - pattern-recognition: "deep-analysis-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["analysis-results", "reasoning-insights", "solution-techniques"]
      - knowledge-extraction: "analysis-methodologies + reasoning-patterns"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["analysis-relationships", "reasoning-dependencies", "solution-connections"]
      - cross-reference: "related-analysis-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "analysis-knowledge + reasoning-patterns"
      - continuous-learning: "deep-analysis-optimization"

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
    - analysis-execution
    - reasoning-process
    - solution-development
    - strategic-evaluation
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "ultra-think"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "analysis-results + reasoning-insights"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["analysis-patterns", "reasoning-techniques", "solution-methodologies"]
  learn-from: ["decision-tree-explorer", "architecture-scenario-explorer", "system-dynamics-modeler"]
  contribute-to: "deep-analysis-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-analysis-requirements
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-analysis-execution
    - continuous-memory-updates
    - real-time-reasoning-optimization
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - analysis-pattern-extraction
---

# Deep Analysis and Problem Solving Mode

Comprehensive deep analysis and problem solving mode with advanced reasoning frameworks and strategic optimization

## Instructions

1. **Initialize Ultra Think Mode**
   - Acknowledge the request for enhanced analytical thinking
   - Set context for deep, systematic reasoning
   - Prepare to explore the problem space comprehensively

2. **Parse the Problem or Question**
   - Extract the core challenge from: **$ARGUMENTS**
   - Identify all stakeholders and constraints
   - Recognize implicit requirements and hidden complexities
   - Question assumptions and surface unknowns

3. **Multi-Dimensional Analysis**
   Approach the problem from multiple angles:
   
   ### Technical Perspective
   - Analyze technical feasibility and constraints
   - Consider scalability, performance, and maintainability
   - Evaluate security implications
   - Assess technical debt and future-proofing
   
   ### Business Perspective
   - Understand business value and ROI
   - Consider time-to-market pressures
   - Evaluate competitive advantages
   - Assess risk vs. reward trade-offs
   
   ### User Perspective
   - Analyze user needs and pain points
   - Consider usability and accessibility
   - Evaluate user experience implications
   - Think about edge cases and user journeys
   
   ### System Perspective
   - Consider system-wide impacts
   - Analyze integration points
   - Evaluate dependencies and coupling
   - Think about emergent behaviors

4. **Generate Multiple Solutions**
   - Brainstorm at least 3-5 different approaches
   - For each approach, consider:
     - Pros and cons
     - Implementation complexity
     - Resource requirements
     - Potential risks
     - Long-term implications
   - Include both conventional and creative solutions
   - Consider hybrid approaches

5. **Deep Dive Analysis**
   For the most promising solutions:
   - Create detailed implementation plans
   - Identify potential pitfalls and mitigation strategies
   - Consider phased approaches and MVPs
   - Analyze second and third-order effects
   - Think through failure modes and recovery

6. **Cross-Domain Thinking**
   - Draw parallels from other industries or domains
   - Apply design patterns from different contexts
   - Consider biological or natural system analogies
   - Look for innovative combinations of existing solutions

7. **Challenge and Refine**
   - Play devil's advocate with each solution
   - Identify weaknesses and blind spots
   - Consider "what if" scenarios
   - Stress-test assumptions
   - Look for unintended consequences

8. **Synthesize Insights**
   - Combine insights from all perspectives
   - Identify key decision factors
   - Highlight critical trade-offs
   - Summarize innovative discoveries
   - Present a nuanced view of the problem space

9. **Provide Structured Recommendations**
   Present findings in a clear structure:
   ```
   ## Problem Analysis
   - Core challenge
   - Key constraints
   - Critical success factors
   
   ## Solution Options
   ### Option 1: [Name]
   - Description
   - Pros/Cons
   - Implementation approach
   - Risk assessment
   
   ### Option 2: [Name]
   [Similar structure]
   
   ## Recommendation
   - Recommended approach
   - Rationale
   - Implementation roadmap
   - Success metrics
   - Risk mitigation plan
   
   ## Alternative Perspectives
   - Contrarian view
   - Future considerations
   - Areas for further research
   ```

10. **Meta-Analysis**
    - Reflect on the thinking process itself
    - Identify areas of uncertainty
    - Acknowledge biases or limitations
    - Suggest additional expertise needed
    - Provide confidence levels for recommendations

## Usage Examples

```bash
# Architectural decision
/project:ultra-think Should we migrate to microservices or improve our monolith?

# Complex problem solving
/project:ultra-think How do we scale our system to handle 10x traffic while reducing costs?

# Strategic planning
/project:ultra-think What technology stack should we choose for our next-gen platform?

# Design challenge
/project:ultra-think How can we improve our API to be more developer-friendly while maintaining backward compatibility?
```

## Key Principles

- **First Principles Thinking**: Break down to fundamental truths
- **Systems Thinking**: Consider interconnections and feedback loops
- **Probabilistic Thinking**: Work with uncertainties and ranges
- **Inversion**: Consider what to avoid, not just what to do
- **Second-Order Thinking**: Consider consequences of consequences

## Output Expectations

- Comprehensive analysis (typically 2-4 pages of insights)
- Multiple viable solutions with trade-offs
- Clear reasoning chains
- Acknowledgment of uncertainties
- Actionable recommendations
- Novel insights or perspectives


