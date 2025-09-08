---
model: claude-sonnet-4-20250514
category: utilities-tools
priority: critical
tags: ["utilities-tools"]
description: Display all available development tools and their descriptions from your system

# Enhanced Context-Aware Agent Integration
enhanced-integration:
  enabled: true
  agent-selection-criteria:
    domain-expertise: ["general-purpose", "development-tools", "system-analysis"]
    complexity-factors: ["tool-discovery", "system-introspection", "documentation-generation"]
    specialized-tools: ["system-analysis", "tool-enumeration", "documentation-creation"]
  preferred-agents:
    primary: "general-purpose"
    fallback: ["mcp-toolsmith"]
  tool-requirements:
    mcp-servers: ["all-available"]
    specialized-functions: ["tool-discovery", "system-introspection"]

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "critical"
  
  # Pre-execution Memory Operations
  pre-execution-memory:
    context-search:
      - query-pattern: "development-tools + system-analysis + tool-discovery"
      - tools: ["mcp__cipher-memory__search_nodes", "mcp__cipher-memory__open_nodes"]
      - context-retrieval: "tool-enumeration-patterns + system-analysis-techniques"
    
    knowledge-preparation:
      - domain: "system-introspection"
      - pattern-search: "tool-discovery + documentation-generation"
      - tools: ["mcp__cipher-memory__read_graph"]
  
  # Execution Memory Operations
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - capture-points: ["tool-discovery", "analysis-insights", "documentation-creation"]
      - entity-updates: "real-time-progress"
    
    decision-logging:
      - tool: "mcp__cipher-memory__create_entities"
      - log-decisions: "tool-selection + analysis-approach + documentation-strategy"
      - pattern-recognition: "development-workflow-patterns"
  
  # Post-execution Memory Operations
  post-execution-memory:
    result-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - store-patterns: ["tool-analysis-results", "system-insights", "documentation-outputs"]
      - knowledge-extraction: "development-tool-patterns + system-analysis-techniques"
    
    relationship-creation:
      - tools: ["mcp__cipher-memory__create_relations"]
      - link-concepts: ["tool-relationships", "system-dependencies", "workflow-connections"]
      - cross-reference: "related-development-processes"
    
    knowledge-refinement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - enrich-existing: "tool-discovery-knowledge + system-analysis-patterns"
      - continuous-learning: "development-workflow-optimization"

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
    - tool-discovery-results
    - analysis-insights
    - documentation-generation
    - error-handling
    - completion-status
  
  # Structured Log Format
  log-structure:
    timestamp: "ISO-8601"
    command: "all-tools"
    execution-id: "UUID"
    agent-assignments: "selected-agents-with-reasoning"
    memory-operations: "cipher-memory-transactions"
    performance-metrics: "execution-time + memory-usage + success-rate"
    outcome-summary: "tool-analysis-results + insights-generated"

# Cross-Command Learning Integration
cross-command-learning:
  enabled: true
  share-insights: ["tool-discovery-patterns", "system-analysis-techniques"]
  learn-from: ["system-behavior-simulator", "debug-error", "project-health-check"]
  contribute-to: "development-workflow-knowledge-base"

# Workflow Integration
workflow-integration:
  pre-execution:
    - validate-system-access
    - prepare-memory-context
    - select-optimal-agents
  
  execution:
    - parallel-tool-discovery
    - continuous-memory-updates
    - real-time-insight-capture
  
  post-execution:
    - comprehensive-result-storage
    - cross-reference-generation
    - workflow-pattern-extraction
---

# Display All Available Development Tools

Display all available development tools

*Command originally created by IndyDevDan (YouTube: https://www.youtube.com/@indydevdan) / DislerH (GitHub: https://github.com/disler)*

## Instructions

Display all available tools from your system prompt in the following format:

1. **List each tool** with its TypeScript function signature
2. **Include the purpose** of each tool as a suffix
3. **Use double line breaks** between tools for readability
4. **Format as bullet points** for clear organization

The output should help developers understand:
- What tools are available in the current Claude Code session
- The exact function signatures for reference
- The primary purpose of each tool

Example format:
```typescript
â€¢ functionName(parameters: Type): ReturnType - Purpose of the tool

â€¢ anotherFunction(params: ParamType): ResultType - What this tool does
```

This command is useful for:
- Quick reference of available capabilities
- Understanding tool signatures
- Planning which tools to use for specific tasks


