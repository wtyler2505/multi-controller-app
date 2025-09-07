# Clear-Thought 1.5 Operations Guide - Complete Reference

## Excellence Through Comprehensive Reasoning

Clear-Thought 1.5 provides 38 modular operations for thorough problem-solving. Each operation is self-contained, tested, and optimized for excellence.

## Core Thinking Operations (7)

### 1. sequential_thinking
**Purpose**: Chain-of-thought reasoning with multiple patterns
**Patterns**: chain, tree, beam, mcts, graph, auto
**Excellence Usage**: Use for ALL complex problems requiring step-by-step analysis
```javascript
clear_thought({
  operation: "sequential_thinking",
  prompt: "Design the Arduino driver reconnection logic",
  parameters: {
    pattern: "tree",  // Explore multiple solution branches
    depth: 5,        // Thorough exploration
    temperature: 0.2  // Focus on correctness
  }
})
```

### 2. mental_model
**Purpose**: Apply proven thinking frameworks
**Models**: first_principles, inversion, second_order, probabilistic
**Excellence Usage**: Break down complex systems to fundamental truths
```javascript
clear_thought({
  operation: "mental_model",
  prompt: "Analyze our transport layer architecture",
  parameters: {
    model: "first_principles",
    depth: "comprehensive"
  }
})
```

### 3. debugging_approach
**Purpose**: Systematic debugging with multiple strategies
**Approaches**: binary_search, divide_conquer, hypothesis_testing
**Excellence Usage**: Debug thoroughly, not quickly
```javascript
clear_thought({
  operation: "debugging_approach",
  prompt: "Serial timeout after 30 seconds",
  parameters: {
    approach: "hypothesis_testing",
    thoroughness: "exhaustive"
  }
})
```

### 4. creative_thinking
**Purpose**: Generate innovative solutions
**Excellence Usage**: Explore ALL possibilities before choosing
```javascript
clear_thought({
  operation: "creative_thinking",
  prompt: "Alternative approaches to device detection",
  parameters: {
    techniques: ["brainstorming", "scamper", "lateral"],
    minimum_ideas: 10
  }
})
```

### 5. visual_reasoning
**Purpose**: Work with diagrams and visual structures
**Excellence Usage**: Create complete architectural diagrams
```javascript
clear_thought({
  operation: "visual_reasoning",
  prompt: "Transport layer component relationships",
  parameters: {
    diagram_type: "architecture",
    detail_level: "comprehensive"
  }
})
```

### 6. metacognitive_monitoring
**Purpose**: Monitor and assess reasoning quality
**Excellence Usage**: Check assumptions and biases in EVERY decision
```javascript
clear_thought({
  operation: "metacognitive_monitoring",
  prompt: "Review our decision to use Rust over C#",
  parameters: {
    check_biases: true,
    validate_assumptions: true
  }
})
```

### 7. scientific_method
**Purpose**: Rigorous hypothesis testing
**Excellence Usage**: Prove correctness through experimentation
```javascript
clear_thought({
  operation: "scientific_method",
  prompt: "Verify 50ms latency enforcement works",
  parameters: {
    hypothesis: "Latency enforcement prevents overload",
    experiments: ["load_test", "edge_cases", "failure_modes"]
  }
})
```

## Collaborative & Decision Operations (4)

### 8. collaborative_reasoning
**Purpose**: Multi-perspective analysis
**Excellence Usage**: Consider ALL stakeholder views
```javascript
clear_thought({
  operation: "collaborative_reasoning",
  prompt: "Should we implement TCP or UDP transport?",
  parameters: {
    personas: ["NetworkExpert", "SecurityArchitect", "PerformanceEngineer"],
    consensus_required: false
  }
})
```

### 9. decision_framework
**Purpose**: Structured decision analysis
**Excellence Usage**: Make decisions with COMPLETE information
```javascript
clear_thought({
  operation: "decision_framework",
  prompt: "Choose between exponential vs linear backoff",
  parameters: {
    criteria: ["reliability", "performance", "complexity"],
    weights: [0.5, 0.3, 0.2]
  }
})
```

### 10. socratic_method
**Purpose**: Question-driven exploration
**Excellence Usage**: Challenge EVERY assumption
```javascript
clear_thought({
  operation: "socratic_method",
  prompt: "Why do we need 3 retry attempts?",
  parameters: {
    depth: "exhaustive",
    challenge_level: "rigorous"
  }
})
```

### 11. structured_argumentation
**Purpose**: Formal argument construction
**Excellence Usage**: Build bulletproof reasoning
```javascript
clear_thought({
  operation: "structured_argumentation",
  prompt: "Justify our modular architecture",
  parameters: {
    argument_type: "toulmin",
    include_rebuttals: true
  }
})
```

## Analysis Operations (7)

### 12. systems_thinking
**Purpose**: Model interconnected components
**Excellence Usage**: Understand COMPLETE system behavior
```javascript
clear_thought({
  operation: "systems_thinking",
  prompt: "Model device driver interactions",
  parameters: {
    include_feedback_loops: true,
    identify_emergent_properties: true
  }
})
```

### 13. research
**Purpose**: Generate research placeholders
**Excellence Usage**: Identify ALL knowledge gaps
```javascript
clear_thought({
  operation: "research",
  prompt: "Serial communication best practices",
  parameters: {
    depth: "comprehensive",
    include_citations: true
  }
})
```

### 14. analogical_reasoning
**Purpose**: Draw parallels between domains
**Excellence Usage**: Learn from ALL similar problems
```javascript
clear_thought({
  operation: "analogical_reasoning",
  prompt: "Apply network patterns to device management",
  parameters: {
    source_domain: "network_protocols",
    target_domain: "device_drivers"
  }
})
```

### 15. causal_analysis
**Purpose**: Investigate cause-effect relationships
**Excellence Usage**: Find ROOT causes, not symptoms
```javascript
clear_thought({
  operation: "causal_analysis",
  prompt: "Why do connections drop after 30 seconds?",
  parameters: {
    method: "fishbone",
    depth: "root_cause"
  }
})
```

### 16. statistical_reasoning
**Purpose**: Statistical analysis
**Modes**: summary, bayes, hypothesis_test, monte_carlo
**Excellence Usage**: Prove with data, not assumptions
```javascript
clear_thought({
  operation: "statistical_reasoning",
  prompt: "Analyze reconnection success rates",
  parameters: {
    mode: "hypothesis_test",
    confidence_level: 0.99
  }
})
```

### 17. simulation
**Purpose**: Model system behavior
**Excellence Usage**: Test ALL scenarios before implementation
```javascript
clear_thought({
  operation: "simulation",
  prompt: "Simulate 1000 reconnection attempts",
  parameters: {
    iterations: 1000,
    include_edge_cases: true
  }
})
```

### 18. optimization
**Purpose**: Find optimal solutions
**Excellence Usage**: Optimize for correctness, then performance
```javascript
clear_thought({
  operation: "optimization",
  prompt: "Optimize retry timing parameters",
  parameters: {
    objective: "reliability",
    constraints: ["latency < 100ms", "cpu < 2%"]
  }
})
```

## Advanced Reasoning Patterns (5)

### 19. tree_of_thought
**Purpose**: Explore solution tree
**Excellence Usage**: Consider ALL branches
```javascript
clear_thought({
  operation: "tree_of_thought",
  prompt: "Implementation paths for device detection"
})
```

### 20. beam_search
**Purpose**: Parallel exploration of top solutions
**Excellence Usage**: Keep multiple options open
```javascript
clear_thought({
  operation: "beam_search",
  prompt: "Best approaches for error handling",
  parameters: { beam_width: 5 }
})
```

### 21. mcts (Monte Carlo Tree Search)
**Purpose**: Probabilistic solution exploration
**Excellence Usage**: Find optimal strategies
```javascript
clear_thought({
  operation: "mcts",
  prompt: "Optimal testing strategy",
  parameters: { simulations: 100 }
})
```

### 22. graph_of_thought
**Purpose**: Non-linear reasoning paths
**Excellence Usage**: Handle complex interdependencies
```javascript
clear_thought({
  operation: "graph_of_thought",
  prompt: "Component dependency resolution"
})
```

### 23. orchestration_suggest
**Purpose**: Recommend operation sequences
**Excellence Usage**: Plan complete reasoning workflows
```javascript
clear_thought({
  operation: "orchestration_suggest",
  prompt: "Debug serial timeout issue"
})
```

## UI & Visualization (2)

### 24. visual_dashboard
**Purpose**: Create dashboard skeletons
**Excellence Usage**: Design comprehensive monitoring
```javascript
clear_thought({
  operation: "visual_dashboard",
  prompt: "Device status monitoring dashboard"
})
```

### 25. custom_framework
**Purpose**: Define custom reasoning frameworks
**Excellence Usage**: Create project-specific approaches
```javascript
clear_thought({
  operation: "custom_framework",
  prompt: "Excellence-focused development framework",
  parameters: {
    steps: ["understand", "design", "implement", "verify", "document"]
  }
})
```

## Notebook Operations (4)

### 26. notebook_create
**Purpose**: Create interactive notebooks
**Excellence Usage**: Document and test thoroughly
```javascript
clear_thought({
  operation: "notebook_create",
  prompt: "Transport layer testing notebook",
  parameters: { name: "transport-tests" }
})
```

### 27. notebook_add_cell
**Purpose**: Add cells to notebooks
**Excellence Usage**: Build comprehensive test suites
```javascript
clear_thought({
  operation: "notebook_add_cell",
  prompt: "Add reconnection test",
  parameters: {
    notebookId: "transport-tests",
    cellType: "code"
  }
})
```

### 28. notebook_run_cell
**Purpose**: Execute notebook cells
**Excellence Usage**: Verify ALL test cases
```javascript
clear_thought({
  operation: "notebook_run_cell",
  parameters: {
    notebookId: "transport-tests",
    cellId: "reconnection-test"
  }
})
```

### 29. notebook_export
**Purpose**: Export notebook content
**Excellence Usage**: Preserve complete test results
```javascript
clear_thought({
  operation: "notebook_export",
  parameters: { notebookId: "transport-tests" }
})
```

## Session Management (3)

### 30. session_info
**Purpose**: Get session information
**Excellence Usage**: Track reasoning completeness
```javascript
clear_thought({
  operation: "session_info"
})
```

### 31. session_export
**Purpose**: Export session for persistence
**Excellence Usage**: Preserve ALL reasoning traces
```javascript
clear_thought({
  operation: "session_export",
  parameters: { include_metadata: true }
})
```

### 32. session_import
**Purpose**: Restore session state
**Excellence Usage**: Continue thorough analysis
```javascript
clear_thought({
  operation: "session_import",
  parameters: { sessionData: previousSession }
})
```

## Metagame Operations (2)

### 33. ulysses_protocol ⭐
**Purpose**: HIGH-STAKES debugging framework
**Excellence Usage**: For CRITICAL issues requiring systematic resolution
```javascript
clear_thought({
  operation: "ulysses_protocol",
  prompt: "Production system authentication failure",
  parameters: {
    stakes: "critical",
    budget: "unlimited",  // Excellence over efficiency
    phases: ["triage", "diagnosis", "resolution", "verification"],
    gates: ["safety_check", "rollback_ready", "monitoring_enabled"]
  }
})
```

### 34. ooda_loop
**Purpose**: Rapid iterative decision-making
**Excellence Usage**: Continuous improvement cycles
```javascript
clear_thought({
  operation: "ooda_loop",
  prompt: "Optimize reconnection strategy",
  parameters: {
    iterations: 10,
    focus: "observe_thoroughly"
  }
})
```

## Analysis Support Operations (3)

### 35. ethical_analysis
**Purpose**: Evaluate ethical implications
**Excellence Usage**: Consider ALL ethical dimensions
```javascript
clear_thought({
  operation: "ethical_analysis",
  prompt: "Data collection practices",
  parameters: {
    framework: "comprehensive",
    stakeholders: ["users", "developers", "society"]
  }
})
```

### 36. code_execution
**Purpose**: Execute code safely
**Excellence Usage**: Test thoroughly in sandboxed environment
```javascript
clear_thought({
  operation: "code_execution",
  prompt: "Test backoff algorithm",
  parameters: {
    language: "python",
    timeout: 10000
  }
})
```

### 37-38. Additional specialized operations
Reserved for future expansion

## Excellence Workflows

### For Complex Problems
1. `orchestration_suggest` - Get operation sequence
2. `sequential_thinking` - Deep exploration
3. `mental_model` - First principles analysis
4. `collaborative_reasoning` - Multiple perspectives
5. `decision_framework` - Structured decision
6. `metacognitive_monitoring` - Validate reasoning
7. `session_export` - Preserve complete analysis

### For Debugging
1. `ulysses_protocol` - High-stakes framework
2. `debugging_approach` - Systematic investigation
3. `causal_analysis` - Root cause analysis
4. `scientific_method` - Hypothesis testing
5. `statistical_reasoning` - Data validation

### For Design
1. `systems_thinking` - Model interactions
2. `visual_reasoning` - Create diagrams
3. `analogical_reasoning` - Learn from patterns
4. `optimization` - Find best approach
5. `structured_argumentation` - Justify decisions

## Integration with Excellence Philosophy

**Every operation should be used with:**
- Thoroughness over speed
- Complete analysis over quick answers
- Multiple perspectives over single viewpoint
- Verification over assumption
- Documentation over memory

## Timeout Considerations

All operations configured with 60000ms timeout for thoroughness:
- Simple operations: Complete in 5-10s
- Complex operations: May use full 60s
- Session operations: Near-instant
- Ulysses Protocol: May require multiple iterations

## Remember

We have 38 tools for reasoning. Use them ALL when appropriate. Excellence demands comprehensive analysis, not quick solutions.