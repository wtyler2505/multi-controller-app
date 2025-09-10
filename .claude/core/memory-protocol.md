# Cipher Memory Protocol - Complete Guide

## The 6 Cipher Memory Tools (USE THE RIGHT ONE!)

### 1. cipher_memory_search ✅ (THE WORKHORSE)
**Purpose**: Fast vector similarity search  
**Response**: 2-5 seconds  
**Use When**: Starting ANY implementation, finding patterns  
```javascript
cipher_memory_search("serial reconnection logic")
// Returns: Similar implementations with confidence scores
```

### 2. cipher_search_reasoning_patterns ✅ (PATTERN FINDER)
**Purpose**: Find problem-solving approaches  
**Response**: 2-5 seconds  
**Use When**: Tackling complex problems, need strategies  
```javascript
cipher_search_reasoning_patterns("debugging timeout issues", {
  domain: "transport", 
  complexity: "high"
})
```

### 3. cipher_store_reasoning_memory ✅ (THE ARCHIVIST)  
**Purpose**: Store complete reasoning traces  
**Response**: 3-8 seconds  
**Use When**: After solving complex problems  
```javascript
cipher_store_reasoning_memory(trace, evaluation)
// Store problem-solving journey for future reference
```

### 4. cipher_extract_reasoning_steps ⚠️ (THE PARSER)
**Purpose**: Convert conversation to structured reasoning  
**Response**: 5-10 seconds  
**Use When**: Analyzing past discussions (<2000 chars)  
```javascript
cipher_extract_reasoning_steps(userInput)  
// Returns: {steps: [{type: "thought", content: "..."}]}
```

### 5. cipher_evaluate_reasoning ✅ (THE CRITIC)
**Purpose**: Quality check before storage  
**Response**: 3-7 seconds  
**Use When**: Before storing important reasoning  
```javascript
cipher_evaluate_reasoning(trace)
// Returns: {qualityScore: 0.8, issues: [], suggestions: []}
```

### 6. cipher_extract_and_operate_memory ⚠️⚠️ (THE PROBLEM)
**Purpose**: Extract + decide + execute (too complex!)  
**Response**: 10-30 seconds (OFTEN TIMES OUT)  
**Use When**: ONLY for tiny snippets <500 chars  
```javascript
// DON'T DO THIS:
cipher_extract_and_operate_memory(entireConversation) // ❌ TIMEOUT!

// DO THIS INSTEAD:
cipher_memory_search("pattern") // ✅ Fast & reliable
```

## Decision Tree: Which Tool to Use?

```
NEED TO SEARCH?
├── For code patterns → cipher_memory_search() [2-5s]
└── For approaches → cipher_search_reasoning_patterns() [2-5s]

NEED TO STORE?  
├── Small pattern (<500 chars) → Maybe try extract_and_operate [30s]
├── Reasoning trace → cipher_store_reasoning_memory() [3-8s]
└── Quick save → create_entities() [instant fallback]

NEED TO EXTRACT?
├── From conversation → cipher_extract_reasoning_steps() [5-10s]
└── Quality check → cipher_evaluate_reasoning() [3-7s]
```

## Input Size Guidelines (CRITICAL!)

| Input Size | Tool Choice | Why |
|------------|-------------|-----|
| <500 chars | extract_and_operate (risky) | Might work, often doesn't |
| 500-2000 chars | extract_reasoning_steps | Designed for this range |
| >2000 chars | **CHUNK IT** - Break into 500-1000 char pieces | Will timeout otherwise |
| Any size | create_entities (fallback) | Always works instantly |

## Chunking Strategy (MANDATORY FOR LARGE INPUTS)

```javascript
// For large conversations or code
const CHUNK_SIZE = 1000;  // Safe size that won't timeout
const chunks = [];

// Break input into digestible pieces
for (let i = 0; i < input.length; i += CHUNK_SIZE) {
  chunks.push(input.slice(i, i + CHUNK_SIZE));
}

// Process each chunk separately
for (const chunk of chunks) {
  // Now safe to use even complex tools
  cipher_extract_reasoning_steps(chunk);  // Won't timeout
  // Or store each chunk's insights
  create_entities([{name: `Pattern chunk ${i}`, ...}]);
}
```

**REMEMBER**: These tools DO WORK - just feed them appropriately sized inputs!

## Common Workflows

### Starting a Task
```javascript
// 1. Search for similar work (ALWAYS DO THIS FIRST)
cipher_memory_search("task description")
cipher_search_reasoning_patterns("problem type")

// 2. Review results before implementing
```

### After Fixing a Bug
```javascript
// 1. Extract the approach (if conversation <2000 chars)
const trace = cipher_extract_reasoning_steps(conversation)

// 2. Evaluate quality
const eval = cipher_evaluate_reasoning(trace)

// 3. Store if good (qualityScore > 0.6)
if (eval.qualityScore > 0.6) {
  cipher_store_reasoning_memory(trace, eval)
}
```

### Storing a Code Pattern
```javascript
// For small patterns - direct storage is FASTER
create_entities([{
  name: "Serial Reconnection Pattern",
  entityType: "pattern",
  observations: ["50ms timeout", "exponential backoff", "3 retries"]
}])
// Don't use extract_and_operate for this!
```

## Performance Expectations

| Tool | Expected Time | Status |
|------|---------------|--------|
| cipher_memory_search | 2-5s | ✅ Reliable |
| cipher_search_reasoning_patterns | 2-5s | ✅ Reliable |
| cipher_evaluate_reasoning | 3-7s | ✅ Reliable |
| cipher_store_reasoning_memory | 3-8s | ✅ Reliable |
| cipher_extract_reasoning_steps | 5-10s | ⚠️ Watch size |
| cipher_extract_and_operate_memory | 10-30s | ❌ Often fails |

## Timeout Recovery Protocol

```javascript
try {
  // Optimistic: try complex tool
  cipher_extract_and_operate_memory(input)  
} catch (timeout) {
  // Plan B: simpler extraction
  try {
    cipher_extract_reasoning_steps(input.slice(0, 1500))
  } catch (timeout2) {
    // Plan C: direct storage
    create_entities([{name: "Pattern", ...}])  // Always works
  }
}
```

## Real Examples from Multi-Controller App

### Example 1: Arduino Driver Pattern (GOOD)
```javascript
// Searched first
cipher_memory_search("arduino serial driver")  // 3s ✅

// Stored pattern after implementation  
create_entities([{
  name: "Arduino Driver Probe",
  entityType: "pattern",
  observations: ["Send '?', expect 'Multi-Controller:Arduino'"]
}])  // Instant ✅
```

### Example 2: CLAUDE.md Optimization (BAD → GOOD)
```javascript
// BAD: Tried to extract entire conversation
cipher_extract_and_operate_memory(fullConversation)  // TIMEOUT ❌

// GOOD: Used direct storage
create_entities([{
  name: "CLAUDE.md Modular Pattern",
  entityType: "pattern",
  observations: ["Use @imports", "Keep under 2KB", "Context-aware"]
}])  // Instant ✅
```

## Golden Rules

1. **ALWAYS search before implementing** - cipher_memory_search()
2. **NEVER send >500 chars to extract_and_operate**
3. **PREFER simple tools** - They're faster and more reliable
4. **USE create_entities as fallback** - It never fails
5. **TRACK input size** - Prevent timeouts before they happen

## Metrics to Track

- Searches performed: X
- Patterns stored: Y  
- Timeouts encountered: Z
- Fallbacks used: W

Report: "Memory ops: X searches, Y stored, Z timeouts"