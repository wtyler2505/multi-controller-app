---
model: claude-sonnet-4-20250514
category: bridge-integration
priority: high
tags: ["bridge-integration", "testing", "reference-validation", "data-migration"]
description: Test Reference Update - Comprehensive reference validation and data migration testing with integrity verification

# Phase 1B Enhanced Context-Aware Agent Integration
agent-selection:
  type: "context-aware"
  domain-expertise: ["reference-testing", "data-migration", "integrity-validation"]
  complexity-level: "complex"
  selection-criteria:
    keyword-match: 0.90
    argument-analysis: 0.95
    project-context: 0.85
  preferred-agents: ["general-purpose"]
  fallback-agents: ["task-orchestrator"]
  confidence-threshold: 0.80

# Universal Cipher Memory Integration (MANDATORY)
cipher-memory-integration:
  enabled: true
  priority: "high"
  pre-execution-memory:
    context-search:
      - query-pattern: "reference testing + data migration + integrity validation"
      - search-depth: "comprehensive"
      - max-results: 12
      - tools: ["mcp__cipher-memory__search_nodes"]
    context-loading:
      - related-patterns: "mcp__cipher-memory__open_nodes"
      - testing-history: "mcp__cipher-memory__search_nodes"
      - validation-patterns: "mcp__cipher-memory__search_nodes"
    graph-analysis:
      - full-context: "mcp__cipher-memory__read_graph"
      - pattern-identification: "internal"
  execution-memory:
    progress-tracking: "mcp__cipher-memory__add_observations"
    decision-logging: "mcp__cipher-memory__create_entities"
    testing-capture: "mcp__cipher-memory__add_observations"
  post-execution-memory:
    result-storage:
      - testing-summary: "mcp__cipher-memory__create_entities"
      - validation-patterns: "mcp__cipher-memory__create_entities"
      - migration-metrics: "mcp__cipher-memory__add_observations"
    relationship-creation:
      - command-relationships: "mcp__cipher-memory__create_relations"
      - project-relationships: "mcp__cipher-memory__create_relations"
      - testing-relationships: "mcp__cipher-memory__create_relations"
    knowledge-enrichment:
      - existing-patterns: "mcp__cipher-memory__add_observations"
      - testing-insights: "mcp__cipher-memory__create_entities"

# Universal Centralized Logging Integration (MANDATORY)
centralized-logging:
  enabled: true
  log-file: ".claude/execution-log.jsonl"
  log-components:
    execution-metadata: true
    agent-selection: true
    tool-chain: true
    memory-operations: true
    performance-metrics: true
    success-indicators: true
    testing-tracking: true
  logging-phases:
    pre-execution: true
    during-execution: true
    post-execution: true
    error-handling: true
  processing:
    real-time-write: true
    batch-processing: false
    error-recovery: true
    compression: false

# Cross-Command Learning Integration (MANDATORY)
cross-command-learning:
  enabled: true
  learning-domains: ["reference-testing", "data-migration", "integrity-validation"]
  pattern-sharing:
    success-patterns: "mcp__cipher-memory__create_entities"
    failure-patterns: "mcp__cipher-memory__create_entities"
    optimization-opportunities: "mcp__cipher-memory__add_observations"
  knowledge-synthesis:
    cross-domain-insights: "mcp__cipher-memory__create_relations"
    usage-pattern-analysis: "internal"
    performance-optimization: "internal"

# Workflow Integration (MANDATORY)
workflow-integration:
  enabled: true
  pre-execution:
    memory-context-loading: true
    cipher-search-patterns: true
    load-testing-history: true
    analyze-related-nodes: true
    validate-tools: true
    load-context: true
    detect-project-state: true
    initialize-execution-log: true
  post-execution:
    store-testing-results: true
    create-pattern-relationships: true
    enrich-existing-knowledge: true
    update-success-patterns: true
    update-selection-accuracy: true
    optimize-tool-chains: true
    finalize-execution-log: true
    generate-execution-summary: true

tool-chain: "reference-testing-data-migration"
auto-deploy: true
parallel-execution: false
allowed-tools: ["Bash", "Read", "Write", "mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes", "mcp__cipher-memory__create_entities", "mcp__cipher-memory__create_relations", "mcp__cipher-memory__add_observations", "mcp__cipher-memory__read_graph"]
---

# Task Three
This is task 003, depends on 001 and 002.
EOF
```

### 2. Create Mappings

Simulate the issue creation mappings:
```bash
# Simulate task -> issue number mapping
cat > /tmp/task-mapping.txt << 'EOF'
001.md:42
002.md:43
003.md:44
EOF

# Create old -> new ID mapping
> /tmp/id-mapping.txt
while IFS=: read -r task_file task_number; do
  old_num=$(basename "$task_file" .md)
  echo "$old_num:$task_number" >> /tmp/id-mapping.txt
done < /tmp/task-mapping.txt

echo "ID Mapping:"
cat /tmp/id-mapping.txt
```

### 3. Update References

Process each file and update references:
```bash
while IFS=: read -r task_file task_number; do
  echo "Processing: $task_file -> $task_number.md"
  
  # Read the file content
  content=$(cat "$task_file")
  
  # Update references
  while IFS=: read -r old_num new_num; do
    content=$(echo "$content" | sed "s/\b$old_num\b/$new_num/g")
  done < /tmp/id-mapping.txt
  
  # Write to new file
  new_name="${task_number}.md"
  echo "$content" > "$new_name"
  
  echo "Updated content preview:"
  grep -E "depends_on:|conflicts_with:" "$new_name"
  echo "---"
done < /tmp/task-mapping.txt
```

### 4. Verify Results

Check that references were updated correctly:
```bash
echo "=== Final Results ==="
for file in 42.md 43.md 44.md; do
  echo "File: $file"
  grep -E "name:|depends_on:|conflicts_with:" "$file"
  echo ""
done
```

Expected output:
- 42.md should have conflicts_with: [43, 44]
- 43.md should have depends_on: [42] and conflicts_with: [44]
- 44.md should have depends_on: [42, 43]

### 5. Cleanup

```bash
cd -
rm -rf /tmp/test-refs
rm -f /tmp/task-mapping.txt /tmp/id-mapping.txt
echo "âœ… Test complete and cleaned up"
```


