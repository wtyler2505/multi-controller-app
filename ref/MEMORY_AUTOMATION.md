# Memory Automation Reference

## Overview
This document defines how Cipher memory operations should function as automatic, native capabilities within Claude Code. Memory should work like actual memory - storing and retrieving without conscious effort.

## Automatic Storage Patterns

### Pattern Detection Rules

#### Code Patterns (Auto-Store)
```javascript
// Trigger: Any reusable code block
if (codeBlock.lines >= 3 && isPotentiallyReusable(codeBlock)) {
  cipher_extract_and_operate_memory({
    interaction: codeBlock,
    knowledgeInfo: {
      domain: detectDomain(codeBlock),
      codePattern: codeBlock.toString()
    },
    memoryMetadata: {
      projectId: currentProject,
      timestamp: Date.now(),
      confidence: calculateConfidence()
    }
  })
}
```

#### Function Patterns
- Function > 5 lines with clear purpose → Store
- Algorithm implementation → Store
- Configuration that works → Store
- Utility function → Store
- Complex regex → Store

#### Error Fix Patterns
```javascript
// Trigger: Error resolution
onErrorResolved: (error, solution) => {
  cipher_store_reasoning_memory({
    trace: {
      id: generateId(),
      steps: [
        {type: "observation", content: error.message},
        {type: "debugging", content: debugSteps},
        {type: "solution", content: solution}
      ],
      metadata: {
        errorType: error.constructor.name,
        fileContext: currentFile,
        fixDuration: timeToFix
      }
    },
    evaluation: {
      qualityScore: solution.worked ? 1.0 : 0.5,
      shouldStore: true
    }
  })
}
```

### Search Trigger Rules

#### Automatic Search Contexts
| Context | Trigger | Search Query |
|---------|---------|--------------|
| New implementation | Starting new function/feature | "similar [feature_type] implementation" |
| Error encountered | Exception/error message | "[error_message] fix solution" |
| Pattern needed | "How do I..." question | "[task_description] pattern" |
| Refactoring | Improving existing code | "[code_type] best practice" |
| Configuration | Setting up services | "[service] configuration working" |

#### Search Implementation
```javascript
// Before ANY implementation
beforeImplementation: async (task) => {
  const memories = await cipher_memory_search({
    query: `${task.type} ${task.description}`,
    top_k: 5,
    similarity_threshold: 0.6
  })
  
  if (memories.results.length > 0) {
    applyMemoryContext(memories)
  }
}
```

## Knowledge Graph Structure

### Entity Types
```javascript
const EntityTypes = {
  PATTERN: "Pattern",           // Reusable code patterns
  SOLUTION: "Solution",         // Error fixes and solutions
  DECISION: "Decision",         // Architecture/tech decisions
  CONFIGURATION: "Configuration", // Working configs
  LEARNING: "Learning",         // New knowledge acquired
  CONVENTION: "Convention",     // Project conventions
  OPTIMIZATION: "Optimization", // Performance improvements
  ALGORITHM: "Algorithm"        // Algorithm implementations
}
```

### Relationship Types
```javascript
const RelationTypes = {
  FIXES: "fixes",               // Solution fixes Error
  IMPLEMENTS: "implements",     // Pattern implements Feature
  REPLACES: "replaces",        // NewPattern replaces OldPattern
  DEPENDS_ON: "depends_on",    // Component depends_on Library
  SIMILAR_TO: "similar_to",    // Pattern similar_to Pattern
  DERIVED_FROM: "derived_from", // Solution derived_from Pattern
  CONFLICTS_WITH: "conflicts_with" // Config conflicts_with Config
}
```

### Automatic Entity Creation
```javascript
// Every significant action creates entities
onSignificantAction: (action) => {
  create_entities([{
    name: generateEntityName(action),
    entityType: detectEntityType(action),
    observations: extractObservations(action)
  }])
  
  // Create relationships
  if (action.relatedEntities) {
    create_relations(generateRelations(action))
  }
}
```

## Memory Operation Triggers

### Task-Based Triggers
```javascript
// Task start
onTaskStart: (taskId) => {
  // Search for similar tasks
  cipher_memory_search(`task similar to ${taskId}`)
  
  // Mark task memory start
  npm_run("memory:task-start", taskId)
}

// Task complete
onTaskComplete: (taskId, implementation) => {
  // Store implementation
  cipher_extract_and_operate_memory(implementation)
  
  // Mark task memory complete
  npm_run("memory:task-complete", taskId)
}
```

### Error-Based Triggers
```javascript
// Error encountered
onError: (error) => {
  // Search for previous solutions
  const solutions = cipher_memory_search(error.message)
  
  if (solutions.results.length > 0) {
    applySolution(solutions.results[0])
  }
}

// Error fixed
onErrorFixed: (error, solution) => {
  // Store solution immediately
  cipher_store_reasoning_memory({
    error: error,
    solution: solution,
    context: getCurrentContext()
  })
}
```

## Dual Memory System

### System 1 Memory (Fast/Intuitive)
- **What**: Code patterns, snippets, configurations
- **When**: Immediate recall needed
- **Storage**: Vector embeddings for similarity search
- **Retrieval**: < 100ms target

### System 2 Memory (Slow/Analytical)
- **What**: Reasoning chains, decision processes
- **When**: Complex problem solving
- **Storage**: Full reasoning traces
- **Retrieval**: < 1s acceptable

## Memory Metrics

### Automatic Tracking
```javascript
const MemoryMetrics = {
  searches: 0,
  stores: 0,
  hits: 0,
  misses: 0,
  entityCount: 0,
  relationCount: 0,
  
  report: function() {
    return `Memory Activity:
    - Searches: ${this.searches} (${this.hits} hits)
    - Stored: ${this.stores} items
    - Graph: ${this.entityCount} entities, ${this.relationCount} relations
    - Hit rate: ${(this.hits/this.searches*100).toFixed(1)}%`
  }
}
```

### Reporting Schedule
- Every 10 tool uses → Quick metrics
- Every 20 interactions → Full report
- On request → Detailed analysis

## Storage Decision Tree

```
Is it code?
├─ Yes → Is it 3+ lines?
│  ├─ Yes → Is it reusable?
│  │  ├─ Yes → STORE as Pattern
│  │  └─ No → Skip
│  └─ No → Skip
└─ No → Is it an error fix?
   ├─ Yes → STORE as Solution
   └─ No → Is it a decision?
      ├─ Yes → STORE as Decision
      └─ No → Is it new knowledge?
         ├─ Yes → STORE as Learning
         └─ No → Skip
```

## Automatic Memory Chains

### Implementation Memory Chain
```javascript
async function implementationChain(task) {
  // 1. Search for similar
  const similar = await cipher_memory_search(task.description)
  
  // 2. Apply context
  if (similar.results.length > 0) {
    applyContext(similar.results)
  }
  
  // 3. Implement
  const implementation = await implement(task)
  
  // 4. Store result
  await cipher_extract_and_operate_memory(implementation)
  
  // 5. Update graph
  await create_entities([{
    name: task.name,
    entityType: "Implementation",
    observations: [implementation.summary]
  }])
}
```

### Debug Memory Chain
```javascript
async function debugChain(error) {
  // 1. Search for solutions
  const solutions = await cipher_memory_search(error.message)
  
  // 2. Apply if found
  if (solutions.results.length > 0) {
    const applied = await applySolution(solutions.results[0])
    if (applied.success) return applied
  }
  
  // 3. Debug manually
  const solution = await debugManually(error)
  
  // 4. Store solution
  await cipher_store_reasoning_memory({
    error: error,
    solution: solution
  })
  
  // 5. Create entities
  await create_entities([
    {name: error.type, entityType: "Error", observations: [error.message]},
    {name: solution.name, entityType: "Solution", observations: [solution.code]}
  ])
  
  // 6. Create relationship
  await create_relations([{
    from: solution.name,
    to: error.type,
    relationType: "fixes"
  }])
}
```

## Performance Optimization

### Batch Operations
```javascript
// Batch multiple memories
const memoryBatch = []

onCodeBlock: (block) => {
  memoryBatch.push(block)
  
  if (memoryBatch.length >= 5) {
    cipher_extract_and_operate_memory(memoryBatch)
    memoryBatch = []
  }
}
```

### Cache Strategy
- Recent searches cached for 5 minutes
- Frequent patterns cached permanently
- Error solutions cached per session

## Validation Criteria

### Storage Success
- 100% of error fixes stored
- 90% of patterns (3+ lines) stored
- 100% of decisions stored
- Zero manual store commands needed

### Retrieval Success
- 80% hit rate on similar problems
- <500ms average search time
- 90% relevance score

## Best Practices

### DO:
✅ Store immediately upon discovery
✅ Search before every implementation
✅ Update graph continuously
✅ Track all metrics
✅ Chain operations automatically

### DON'T:
❌ Wait for user command to store
❌ Ask permission to search
❌ Store trivial patterns (<3 lines)
❌ Ignore failed searches
❌ Break chains on failure

## Common Patterns Library

### Stored Automatically:
1. **Error handlers** - try/catch patterns
2. **Validation functions** - input validators
3. **API calls** - fetch patterns
4. **State management** - update patterns
5. **Component structures** - UI patterns
6. **Configuration objects** - working configs
7. **Regular expressions** - complex patterns
8. **Algorithm implementations** - sorting, searching
9. **Optimization techniques** - performance improvements
10. **Security patterns** - sanitization, validation