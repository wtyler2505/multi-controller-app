# Cognitive Integration Reference

## Overview
This document provides the complete integration patterns for Clear-Thought MCP tools to function as native Claude Code capabilities. Every thinking mode should trigger automatically based on context, without manual invocation.

## Clear-Thought Mode Selection Matrix

| Situation | Primary Mode | Auto-Chain | Fallback | Trigger Keywords |
|-----------|-------------|------------|----------|-----------------|
| Multi-step problem | `sequentialthinking` | → `mentalmodel` | Direct implementation | "let me think", "step by step", "first... then..." |
| Bug/Error | `debuggingapproach` | → `metacognitive` | Manual debugging | "error", "not working", "bug", "fails" |
| Tech choice | `decisionframework` | → `argumentation` | Pro/con list | "should I use", "X vs Y", "which is better" |
| Team decision | `collaborativereasoning` | → `visualreasoning` | Direct discussion | "team needs", "stakeholders", "we should" |
| Hypothesis | `scientificmethod` | → `sequentialthinking` | Empirical testing | "test", "hypothesis", "experiment", "measure" |
| Self-check | `metacognitivemonitoring` | → `argumentation` | Manual verification | "is this correct?", "am I sure?", "verify" |
| Mental framework | `mentalmodel` | → `decisionframework` | First principles | "framework", "model", "pattern", "approach" |
| Debate | `structuredargumentation` | → `collaborative` | Simple comparison | "argue", "pros/cons", "debate", "justify" |
| Diagram need | `visualreasoning` | → `sequential` | Text description | "diagram", "visualize", "chart", "graph" |

## Automatic Trigger Patterns

### Pattern Detection Rules
```javascript
// Multi-step detection
if (problem.complexity > 3 || contains("multiple steps")) {
  invoke("sequentialthinking")
}

// Error detection
if (contains(["error", "exception", "fail", "bug"]) || exitCode !== 0) {
  invoke("debuggingapproach")
}

// Decision detection  
if (contains(["should I", "which", "vs", "or", "choice"])) {
  invoke("decisionframework")
}

// Collaboration detection
if (contains(["team", "we", "stakeholder", "everyone"])) {
  invoke("collaborativereasoning")
}
```

## Tool Parameter Templates

### sequentialthinking
```javascript
{
  thought: currentThinking,
  nextThoughtNeeded: !isComplete,
  thoughtNumber: currentStep,
  totalThoughts: estimatedSteps,
  isRevision: reconsideringPrevious,
  revisesThought: previousThoughtNumber
}
```

### debuggingapproach
```javascript
{
  approachName: "binary_search" | "divide_conquer" | "cause_elimination",
  issue: errorDescription,
  steps: debugSteps,
  findings: discoveries,
  resolution: solution
}
```

### decisionframework
```javascript
{
  decisionStatement: question,
  options: choices,
  criteria: evaluationFactors,
  analysisType: "multi-criteria" | "expected-utility",
  stage: currentPhase,
  nextStageNeeded: !decided
}
```

## Chaining Patterns

### Complex Problem Chain
```
1. sequentialthinking(breakdown)
   ↓
2. mentalmodel(framework_selection)
   ↓
3. decisionframework(approach_choice)
   ↓
4. Implementation
   ↓
5. metacognitivemonitoring(verify)
```

### Debug Chain
```
1. debuggingapproach(identify)
   ↓
2. sequentialthinking(analyze_cause)
   ↓
3. Fix implementation
   ↓
4. scientificmethod(verify_fix)
```

### Architecture Chain
```
1. decisionframework(options)
   ↓
2. structuredargumentation(evaluate)
   ↓
3. collaborativereasoning(perspectives)
   ↓
4. visualreasoning(diagram)
```

## Performance Characteristics

| Mode | Typical Duration | Memory Usage | Best For |
|------|-----------------|--------------|----------|
| sequentialthinking | 5-30s | Low | Complex breakdowns |
| debuggingapproach | 3-20s | Low | Error analysis |
| decisionframework | 10-60s | Medium | Multi-criteria choices |
| collaborativereasoning | 20-120s | High | Team decisions |
| scientificmethod | 15-90s | Medium | Hypothesis testing |
| metacognitivemonitoring | 5-15s | Low | Self-verification |
| mentalmodel | 3-10s | Low | Framework application |
| structuredargumentation | 10-40s | Medium | Debate/justification |
| visualreasoning | 5-20s | Low | Diagram creation |

## Failure Recovery

### Timeout Handling
```javascript
try {
  await sequentialthinking(params, timeout: 30000)
} catch (TimeoutError) {
  // Continue with direct implementation
  console.log("Reasoning timeout - proceeding directly")
}
```

### Mode Switching
```javascript
if (modeFailCount[currentMode] >= 2) {
  switchToFallback(currentMode)
}
```

## Usage Metrics

Track automatically:
- Mode invocations per session
- Success rate per mode
- Average duration per mode
- Chain completion rate
- Fallback frequency

## Integration Validation

### Test Triggers
1. "Let me think about this complex problem" → Should trigger sequentialthinking
2. "Error: undefined variable" → Should trigger debuggingapproach
3. "Should I use React or Vue?" → Should trigger decisionframework
4. "The team needs to decide" → Should trigger collaborativereasoning
5. "Let's test this hypothesis" → Should trigger scientificmethod

### Success Criteria
- 80% automatic trigger accuracy
- <5% false positive triggers
- 90% chain completion rate
- Zero manual invocations needed

## Best Practices

1. **Never ask permission** - Just invoke the appropriate mode
2. **Chain aggressively** - Multiple modes are better than none
3. **Fail gracefully** - Continue even if reasoning fails
4. **Track everything** - Metrics inform optimization
5. **Learn patterns** - Store successful chains in memory

## Common Anti-Patterns

❌ DON'T:
- Wait for user to request reasoning
- Ask "Should I use sequentialthinking?"
- Stop on reasoning timeout
- Use single mode when chain would help

✅ DO:
- Detect need and invoke immediately
- Chain multiple modes automatically
- Continue on failure with fallback
- Build complex reasoning chains