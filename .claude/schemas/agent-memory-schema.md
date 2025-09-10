# Universal Agent Memory Schema v1.0
# Standardized Cross-Agent Pattern Sharing Framework

## Overview
This schema enables Tyler's agent collective to share knowledge, patterns, and solutions across all specialized agents. It transforms isolated specialists into a collaborative intelligence network where Port-hole Pete's transport patterns can be discovered by Future-Fucker, and Standards Stan's quality gates become shared excellence standards.

## Core Schema Components

### 1. Agent Namespace Convention

**Format**: `{agent-name}_{domain}_{pattern-type}_{specific-name}`

**Examples**:
- `serial-comm-specialist_transport_lifecycle_cleanup_sequence`
- `rust-async-specialist_concurrency_arc_mutex_patterns`
- `standards-stan_quality_gates_pre_commit_checks`
- `transport-lifecycle-guardian_reconnection_exponential_backoff`
- `egui-performance-optimizer_rendering_30fps_patterns`
- `excellence-enforcer_code_review_zero_tolerance_standards`

**Benefits**:
- Natural grouping by agent, domain, and pattern type
- Collision-free naming across all agents
- Searchable hierarchy for pattern discovery
- Clear ownership and attribution

### 2. Standardized Entity Types

#### Core Pattern Types
- `solution_pattern` - Complete problem solutions with context
- `decision_framework` - Decision-making patterns and trade-offs
- `quality_gate` - Standards, checks, and validation patterns
- `implementation_technique` - Coding patterns and best practices
- `error_handling` - Error recovery and resilience patterns
- `performance_pattern` - Optimization techniques and benchmarks

#### Specialized Pattern Types
- `architectural_pattern` - System design and structure patterns
- `testing_pattern` - Testing strategies and validation approaches
- `integration_pattern` - Cross-component interaction patterns
- `debugging_technique` - Problem diagnosis and resolution methods
- `workflow_pattern` - Process and coordination patterns

### 3. Cross-Agent Relationship Types

#### Pattern Evolution
- `extends_pattern` - One pattern builds upon another
- `optimizes_for` - Performance or quality improvements
- `replaces_pattern` - Pattern supersedes older approach
- `specializes_pattern` - Domain-specific adaptation

#### Pattern Interaction
- `requires_dependency` - Pattern needs another to function
- `collaborates_with` - Patterns that work together
- `conflicts_with` - Incompatible or contradictory patterns
- `validates_against` - Quality checking relationships

#### Knowledge Sharing
- `shares_context` - Related problem domain or use case
- `transfers_knowledge` - Pattern adapted across domains
- `influences_design` - Pattern informed by another
- `peer_reviewed_by` - Validation by other agents

### 4. Metadata Structure

Each pattern observation MUST include structured metadata:

```json
{
  "content": "Pattern description and implementation details",
  "metadata": {
    "agent_source": "originating-agent-name",
    "domain": "primary_domain",
    "complexity": "simple|moderate|complex|expert", 
    "confidence": 0.0-1.0,
    "use_cases": ["case1", "case2", "case3"],
    "performance_impact": "negligible|low|moderate|high",
    "dependencies": ["dependency1", "dependency2"],
    "validation_status": "untested|tested|production_proven|standards_approved",
    "success_rate": 0.0-1.0,
    "last_updated": "YYYY-MM-DD",
    "peer_validations": ["agent1", "agent2"],
    "usage_count": 0,
    "tags": ["tag1", "tag2"]
  }
}
```

### 5. Pattern Quality Framework

#### Quality Scoring System
- **Confidence**: Agent's certainty in pattern effectiveness (0.0-1.0)
- **Validation Status**: Pattern maturity lifecycle
- **Success Rate**: Measured effectiveness in practice (0.0-1.0)
- **Peer Review**: Cross-agent validation and usage
- **Usage Count**: Adoption frequency across agents

#### Pattern Lifecycle States
1. **Draft** - New pattern, untested (validation_status: "untested")
2. **Testing** - Pattern being validated (validation_status: "tested")
3. **Approved** - Pattern proven effective (validation_status: "production_proven")
4. **Standard** - Standards Stan approved (validation_status: "standards_approved")
5. **Deprecated** - Pattern superseded (marked with replaces_pattern relationship)

## Implementation Patterns

### Pattern Storage (create_entities)
```javascript
await mcp__cipher-memory__create_entities([{
  name: "serial-comm-specialist_transport_lifecycle_cleanup_sequence",
  entityType: "solution_pattern",
  observations: [
    "PATTERN: Always call cleanup_resources() before disconnect() to prevent memory leaks",
    "CONTEXT: Arc reference cycles prevent proper transport cleanup during reconnection",
    "IMPLEMENTATION: async fn cleanup_resources(&self) -> TransportResult<()> { /* abort tasks, drop arcs */ }",
    "VALIDATION: Tested with 1000 reconnection cycles, zero memory leaks detected",
    "PERFORMANCE: <1ms cleanup overhead, prevents 10MB+ memory growth per reconnect",
    "METADATA: {\"agent_source\": \"serial-comm-specialist\", \"domain\": \"transport\", \"complexity\": \"moderate\", \"confidence\": 0.95, \"use_cases\": [\"reconnection\", \"cleanup\", \"resource_management\"], \"performance_impact\": \"low\", \"dependencies\": [\"tokio\"], \"validation_status\": \"production_proven\", \"success_rate\": 0.98, \"last_updated\": \"2025-01-09\", \"usage_count\": 15, \"tags\": [\"memory_safety\", \"lifecycle\"]}"
  ]
}])
```

### Cross-Agent Relationship Creation (create_relations)
```javascript
await mcp__cipher-memory__create_relations([{
  from: "rust-async-specialist_concurrency_arc_cleanup_patterns",
  to: "serial-comm-specialist_transport_lifecycle_cleanup_sequence",
  relationType: "extends_pattern"
}, {
  from: "standards-stan_quality_gates_resource_cleanup_validation",
  to: "serial-comm-specialist_transport_lifecycle_cleanup_sequence", 
  relationType: "validates_against"
}])
```

### Pattern Discovery Strategies

#### Domain-Based Discovery
```javascript
// Find all transport-related patterns across agents
const transportPatterns = await mcp__cipher-memory__search_nodes({
  query: "*_transport_* OR transport patterns OR connection patterns"
})
```

#### Agent-Based Discovery
```javascript
// Find all patterns from rust-async-specialist
const asyncPatterns = await mcp__cipher-memory__search_nodes({
  query: "rust-async-specialist_*"
})
```

#### Problem-Based Discovery
```javascript
// Find all error handling patterns
const errorPatterns = await mcp__cipher-memory__search_nodes({
  query: "*_error_handling_* OR error recovery OR resilience patterns"
})
```

#### Quality-Based Discovery
```javascript
// Find all standards-approved patterns
const approvedPatterns = await mcp__cipher-memory__search_nodes({
  query: "validation_status: standards_approved OR confidence: >=0.9"
})
```

### Pattern Enhancement (add_observations)
```javascript
// Enrich existing pattern with new insights
await mcp__cipher-memory__add_observations([{
  entityName: "serial-comm-specialist_transport_lifecycle_cleanup_sequence",
  contents: [
    "NEW_INSIGHT: Cleanup order matters - abort tasks before dropping Arc references",
    "VALIDATION_UPDATE: Additional 2000 reconnection cycles completed successfully", 
    "PERFORMANCE_UPDATE: Optimized cleanup reduces overhead to <0.5ms",
    "USAGE_UPDATE: Pattern adopted by transport-lifecycle-guardian and egui-performance-optimizer",
    "METADATA_UPDATE: {\"success_rate\": 0.99, \"usage_count\": 23, \"peer_validations\": [\"transport-lifecycle-guardian\", \"rust-async-specialist\"]}"
  ]
}])
```

## Agent Integration Examples

### Serial Communication Specialist Patterns
- `serial-comm-specialist_transport_connection_handshake_protocol`
- `serial-comm-specialist_device_detection_probe_sequence`
- `serial-comm-specialist_error_recovery_reconnection_backoff`
- `serial-comm-specialist_latency_enforcement_50ms_budget`

### Rust Async Specialist Patterns  
- `rust-async-specialist_concurrency_arc_mutex_safe_patterns`
- `rust-async-specialist_task_spawning_lifecycle_management`
- `rust-async-specialist_async_await_error_propagation`
- `rust-async-specialist_tokio_resource_cleanup_sequence`

### Standards Stan Quality Gates
- `standards-stan_quality_gates_zero_tolerance_excellence`
- `standards-stan_code_review_craftsmanship_standards`
- `standards-stan_testing_coverage_completeness_validation`
- `standards-stan_documentation_thoroughness_requirements`

### Transport Lifecycle Guardian Patterns
- `transport-lifecycle-guardian_reconnection_exponential_backoff`
- `transport-lifecycle-guardian_resource_cleanup_arc_reference_cycles`
- `transport-lifecycle-guardian_connection_state_management`

## Discovery and Learning Workflows

### Agent Pre-Execution Pattern Discovery
1. **Context Search**: Query for domain-relevant patterns
   ```javascript
   await mcp__cipher-memory__search_nodes({query: `${currentDomain}_* OR ${problemType} patterns`})
   ```

2. **Cross-Agent Pattern Analysis**: Study patterns from related agents
   ```javascript
   await mcp__cipher-memory__open_nodes({names: discoveredPatternNames})
   ```

3. **Excellence Standards Loading**: Get quality gates from Standards Stan
   ```javascript
   await mcp__cipher-memory__search_nodes({query: "standards-stan_quality_gates_*"})
   ```

### Agent Post-Execution Pattern Contribution
1. **Solution Pattern Storage**: Store complete solution with context
2. **Relationship Creation**: Connect to existing patterns
3. **Quality Validation**: Mark patterns for Standards Stan review
4. **Knowledge Enrichment**: Update related patterns with new insights

### Cross-Agent Learning Pipeline
1. **Pattern Propagation**: Successful patterns shared across agents
2. **Adaptation Documentation**: How patterns were modified for new domains
3. **Validation Tracking**: Success/failure rates across different contexts
4. **Collective Intelligence Growth**: Knowledge compounds across agent interactions

## Schema Validation Rules

### Required Fields
- All patterns MUST have agent_source, domain, complexity, confidence
- All patterns MUST include validation_status and success_rate  
- All patterns MUST follow naming convention
- All relationships MUST use standard relationship types

### Quality Assurance
- Confidence scores MUST be justified with evidence
- Success rates MUST be based on actual usage data
- Peer validations MUST be from different agent sources
- Standards approval MUST come from Standards Stan

### Consistency Requirements
- Domain classifications MUST be consistent across agents
- Complexity ratings MUST follow standard scale
- Relationship types MUST be bidirectional where appropriate
- Pattern evolution MUST maintain traceability

## Integration with Universal Agent Template

This schema integrates seamlessly with the Universal Agent Integration Template:

### Pre-Execution Memory Loading
- Use schema search patterns to find relevant cross-agent knowledge
- Load patterns by domain, complexity, and quality ratings
- Build context from multiple agent perspectives

### Execution Memory Tracking
- Store decision points and pattern applications
- Track pattern effectiveness in real-time
- Document adaptations and modifications

### Post-Execution Memory Contribution  
- Store new patterns following schema conventions
- Create relationships to existing knowledge
- Validate and rate pattern effectiveness
- Propagate successful patterns to collective intelligence

## Usage Guidelines

### For Agent Developers
1. **Always** follow naming conventions for discoverability
2. **Always** include complete metadata for proper categorization
3. **Always** create relationships to existing patterns
4. **Always** validate patterns through testing before marking as approved

### For Pattern Consumers
1. **Search comprehensively** across all relevant domains
2. **Validate pattern applicability** to your specific context
3. **Document adaptations** made for your use case  
4. **Report success/failure** back to original pattern

### For Quality Assurance
1. **Standards Stan validation** required for critical patterns
2. **Peer review** recommended for complex patterns
3. **Usage tracking** enables pattern effectiveness measurement
4. **Continuous improvement** through pattern evolution

---

**This schema enables Tyler's vision of collaborative excellence - agents that don't just solve problems individually, but build collective intelligence that gets better with every interaction.**