# Cipher Memory Protocol for Multi-Controller App
## Version 1.0 - Living Document

This protocol defines systematic rules for utilizing Cipher's memory capabilities throughout the development lifecycle.

## 1. Memory Storage Triggers

### 1.1 Automatic Triggers
These events automatically trigger memory storage operations:

| Event | Memory Type | Storage Action |
|-------|------------|----------------|
| Task Start | Knowledge Graph | Load related memories, search similar tasks |
| Test Pass | Knowledge Memory | Store solution pattern with embeddings |
| Error Resolved | Reflection Memory | Store troubleshooting reasoning trace |
| Performance Milestone | Knowledge Graph | Create performance entity with metrics |
| Architecture Decision | Knowledge Graph | Create decision entity with rationale |
| Code Pattern Found | Knowledge Memory | Store pattern with context |
| Safety Rule Violation | Knowledge Graph | Create CRITICAL safety entity |
| Task Complete | Reflection Memory | Extract and store learning patterns |

### 1.2 Manual Triggers
Explicitly store when:
- Discovering non-obvious solution
- Making important design choice
- Finding performance optimization
- Resolving complex bug
- Learning new pattern or technique

## 2. Storage Rules by Content Type

### 2.1 Knowledge Graph (Entities & Relations)
**Use for structural, relational information:**
- Component architecture and dependencies
- Team decisions and rationale  
- Project milestones and progress
- Critical paths and interfaces
- Safety requirements and constraints

**Entity Format:**
```javascript
{
  name: "ComponentName.AspectType",
  entityType: "Project.Category",
  observations: [
    "Key fact or decision",
    "Implementation detail",
    "Important constraint"
  ]
}
```

### 2.2 Knowledge Memory (Embeddings)
**Use for searchable, semantic content:**
- Code implementations and patterns
- Error messages and solutions
- Configuration examples
- Performance optimizations
- Test strategies

**Storage Format:**
```javascript
cipher_extract_and_operate_memory({
  interaction: ["problem description", "solution steps"],
  knowledgeInfo: {
    domain: "category",
    codePattern: "pattern description"
  },
  memoryMetadata: {
    taskId: "current_task",
    importance: "CRITICAL|IMPORTANT|CONTEXTUAL|TEMPORARY"
  }
})
```

### 2.3 Reflection Memory (Reasoning)
**Use for process and reasoning:**
- Problem-solving approaches
- Debugging strategies
- Decision-making processes
- Failed attempts and learnings
- Complex implementation reasoning

## 3. Memory Hierarchy & Retention

### 3.1 Importance Levels

| Level | Retention | Examples | Pruning |
|-------|-----------|----------|---------|
| CRITICAL | Permanent | Safety rules, architecture, interfaces | Never |
| IMPORTANT | 90+ days | Working patterns, solutions, optimizations | Annual review |
| CONTEXTUAL | 30 days | Current task details, recent debugging | Monthly |
| TEMPORARY | 7 days | Experiments, failed attempts, exploration | Weekly |

### 3.2 Tagging Strategy
Every memory must include:
- `projectId`: "multi-controller-app"
- `importance`: Level from hierarchy
- `taskId`: Related task number
- `component`: Affected component
- `timestamp`: Creation time
- `source`: Origin (code, test, debug, etc.)

## 4. Retrieval Patterns

### 4.1 Context-Aware Search Templates

**Starting a Task:**
```javascript
search_queries = [
  `task ${taskId} implementation`,
  `component ${componentName} patterns`,
  `interface ${interfaceName} examples`
]
```

**Debugging:**
```javascript
search_queries = [
  `error "${errorMessage}"`,
  `symptom ${behavior}`,
  `${componentName} troubleshooting`
]
```

**Implementation:**
```javascript
search_queries = [
  `pattern ${patternName}`,
  `${language} ${featureType} example`,
  `performance ${metricName} optimization`
]
```

**Code Review:**
```javascript
search_queries = [
  `decision ${topicArea}`,
  `best practice ${componentType}`,
  `safety ${requirementType}`
]
```

### 4.2 Proactive Loading Rules

| Context | Auto-Load Memories |
|---------|-------------------|
| File opened | Component memories, related interfaces |
| Test run | Previous failures, test strategies |
| Build started | Build optimizations, common issues |
| PR created | Decision history, review checklist |
| Error thrown | Similar errors, troubleshooting steps |

## 5. Task Master Integration

### 5.1 Task Lifecycle Memory Operations

```yaml
task_start:
  - cipher_memory_search("task ${id}")
  - Load dependencies memories
  - Retrieve similar completed tasks

subtask_progress:
  - Store implementation snippets
  - Update reasoning traces
  - Link discoveries to subtask

task_complete:
  - Extract successful patterns
  - Store lessons learned
  - Update knowledge graph
  - Prune temporary memories
```

### 5.2 Memory-Task Linking
- Every memory tagged with task ID
- Task completion triggers pattern extraction
- Failed tasks store troubleshooting wisdom
- Blocked tasks save context for resumption

## 6. Pruning Strategies

### 6.1 Automatic Pruning Schedule
- **Hourly**: Remove duplicate searches
- **Daily**: Consolidate similar memories
- **Weekly**: Prune TEMPORARY level
- **Monthly**: Review CONTEXTUAL level
- **Quarterly**: Optimize embeddings

### 6.2 Pruning Rules
Never prune if:
- Marked as CRITICAL
- Referenced by active task
- Part of safety requirements
- Core architectural decision
- Unique solution pattern

Always prune if:
- Duplicate with lower confidence
- Expired TEMPORARY memory
- Orphaned relation without entities
- Superseded by newer version
- Below similarity threshold (0.3)

## 7. Quality Assurance

### 7.1 Memory Health Metrics
Monitor weekly:
- Search success rate (target: >90%)
- Average search time (target: <500ms)
- Memory growth rate
- Pruning effectiveness
- Knowledge gap identification

### 7.2 Validation Checks
Before storing:
- Verify not duplicate (>0.95 similarity)
- Ensure proper tagging
- Validate importance level
- Check relationship consistency

## 8. Emergency Procedures

### 8.1 Memory Recovery
If memory system fails:
1. Check SQLite database integrity
2. Verify Ollama embeddings service
3. Restore from .cipher/backups/
4. Rebuild embeddings if needed
5. Re-index knowledge graph

### 8.2 Fallback Mode
When memory unavailable:
- Continue development with warnings
- Queue memories for later storage
- Use file-based search as backup
- Document critical decisions manually

## 9. Continuous Improvement

### 9.1 Feedback Loop
- Log helpful memories (increase importance)
- Track search misses (identify gaps)
- Monitor performance impact
- Refine storage triggers
- Optimize search patterns

### 9.2 Evolution Strategy
- Weekly: Review search effectiveness
- Monthly: Analyze memory usage patterns
- Quarterly: Major protocol updates
- Annually: Complete system audit

## Quick Reference Commands

```bash
# Store current context
mcp__cipher-aggregator__create_entities

# Search for pattern
mcp__cipher-aggregator__cipher_memory_search --query "pattern"

# Store reasoning
mcp__cipher-aggregator__cipher_store_reasoning_memory

# Check memory stats
mcp__cipher-aggregator__get_usage_stats

# Prune old memories
mcp__cipher-aggregator__delete_entities --importance "TEMPORARY"
```

---
*This protocol is a living document. Update based on experience and evolving needs.*