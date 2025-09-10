# Enhanced Command YAML Schema for Agent/MCP Integration

## Overview
This document defines the enhanced YAML frontmatter schema for integrating specialized agents and MCP server tools with Claude Code slash commands.

## Enhanced YAML Fields

### Required Fields (Existing)
```yaml
model: claude-sonnet-4-20250514
category: string
priority: high|medium|low
tags: array[string]
description: string
```

### New Integration Fields

#### Agent Configuration
```yaml
# Direct agent assignment (Tier 1)
assigned-agent: string              # Primary agent for this command
fallback-agents: array[string]      # Backup agents if primary unavailable

# Context-aware selection (Tier 2)
agent-selection-criteria:
  domain-hints: array[string]       # ["rust", "performance", "testing"]
  complexity-level: string          # "low", "medium", "high", "expert"
  reasoning-type: string            # "sequential", "collaborative", "creative"
  project-context: string           # Auto-detected or specified

# Smart selection (Tier 3)
smart-agent-selection: boolean      # Enable intelligent agent selection
```

#### MCP Tool Configuration
```yaml
# Direct tool specification
required-tools: array[string]       # Mandatory MCP tools
suggested-tools: array[string]      # Optional/fallback tools
excluded-tools: array[string]       # Tools to avoid

# Tool chains (predefined sequences)
tool-chain: string                  # Reference to template
auto-tool-sequence: array[string]   # Custom sequence

# Tool selection criteria
tool-selection:
  performance: boolean              # Prioritize performance tools
  research: boolean                # Include research tools
  collaboration: boolean           # Enable collaborative tools
  security: boolean                # Include security validation
```

#### Workflow Configuration
```yaml
# Execution behavior
auto-deploy: boolean                # Automatically deploy agents
parallel-execution: boolean         # Allow parallel agent deployment
max-agent-count: number            # Limit concurrent agents

# Integration settings
pre-execution:
  validate-tools: boolean          # Check tool availability
  load-context: boolean           # Load relevant context
  prepare-environment: boolean     # Setup development environment

post-execution:
  store-results: boolean           # Save to cipher-memory
  update-tasks: boolean            # Update Task Master
  generate-report: boolean         # Create execution report
```

## Agent Registry

### Available Specialized Agents
```yaml
agents:
  # Task Management
  task-orchestrator:
    description: "Coordinates complex multi-step tasks and parallel execution"
    domains: ["project-management", "task-coordination"]
    tools: ["mcp__taskmaster-ai__*"]
    
  task-executor:
    description: "Implements and executes specific identified tasks"
    domains: ["implementation", "task-completion"]
    tools: ["mcp__taskmaster-ai__*", "mcp__desktop-commander__*"]
    
  task-checker:
    description: "Verifies task completion and quality assurance"
    domains: ["validation", "quality-control"]
    tools: ["mcp__taskmaster-ai__*", "mcp__desktop-commander__*"]

  # Development Specialists
  serial-comm-specialist:
    description: "Serial communication and hardware integration"
    domains: ["hardware", "serial", "arduino", "communication"]
    tools: ["mcp__desktop-commander__*", "mcp__FileScopeMCP__*"]
    
  rust-async-specialist:
    description: "Rust async/await patterns and concurrency"
    domains: ["rust", "async", "concurrency", "performance"]
    tools: ["mcp__desktop-commander__*", "mcp__context7__*"]
    
  egui-performance-optimizer:
    description: "egui GUI performance optimization"
    domains: ["ui", "egui", "performance", "rendering"]
    tools: ["mcp__desktop-commander__*", "mcp__FileScopeMCP__*"]

  # Testing & Quality
  mock-test-orchestrator:
    description: "Test suites, mocking, and hardware simulation"
    domains: ["testing", "mocking", "simulation", "validation"]
    tools: ["mcp__desktop-commander__*", "mcp__taskmaster-ai__*"]
    
  cargo-build-engineer:
    description: "Rust build systems and compilation management"
    domains: ["rust", "build", "cargo", "compilation"]
    tools: ["mcp__desktop-commander__*", "mcp__context7__*"]

  # Performance & Monitoring
  rust-performance-monitor:
    description: "Performance monitoring and optimization"
    domains: ["performance", "monitoring", "profiling", "optimization"]
    tools: ["mcp__desktop-commander__*", "mcp__FileScopeMCP__*"]
    
  telemetry-collector:
    description: "Real-time data collection and buffering"
    domains: ["telemetry", "data-collection", "monitoring"]
    tools: ["mcp__desktop-commander__*", "mcp__taskmaster-ai__*"]

  # Architecture & Design
  general-purpose:
    description: "General development and problem-solving"
    domains: ["general", "architecture", "design", "documentation"]
    tools: ["mcp__context7__*", "mcp__perplexity-ask__*", "mcp__clear-thought__*"]
```

## Tool Chain Templates

### Predefined Tool Chains
```yaml
tool-chains:
  # Development workflows
  rust-development:
    description: "Complete Rust development workflow"
    tools:
      - "mcp__taskmaster-ai__get_tasks"
      - "mcp__desktop-commander__start_process"  
      - "mcp__FileScopeMCP__find_important_files"
      - "mcp__cipher-memory__search_nodes"
    sequence: ["context", "implementation", "validation", "documentation"]

  performance-analysis:
    description: "Performance monitoring and optimization"
    tools:
      - "mcp__desktop-commander__start_process"
      - "mcp__FileScopeMCP__find_important_files"
      - "mcp__clear-thought__sequentialthinking"
      - "mcp__cipher-memory__create_entities"
    
  research-heavy:
    description: "Research and documentation workflow"
    tools:
      - "mcp__perplexity-ask__perplexity_ask"
      - "mcp__context7__get-library-docs"
      - "mcp__clear-thought__collaborativereasoning"
      - "mcp__memory__create_entities"

  testing-validation:
    description: "Testing and quality assurance"
    tools:
      - "mcp__desktop-commander__start_process"
      - "mcp__taskmaster-ai__get_task"
      - "mcp__FileScopeMCP__recalculate_importance"
      - "mcp__cipher-memory__store_knowledge"

  # Task management workflows  
  task-coordination:
    description: "Task Master integration workflow"
    tools:
      - "mcp__taskmaster-ai__get_tasks"
      - "mcp__taskmaster-ai__next_task"
      - "mcp__taskmaster-ai__set_task_status"
      - "mcp__cipher-memory__create_entities"
```

## Selection Heuristics

### Agent Selection Rules
```yaml
selection-rules:
  # Category-based selection
  "category:task-management":
    primary: "task-orchestrator"
    fallback: ["task-executor", "general-purpose"]
    
  "category:performance-optimization":
    primary: "rust-performance-monitor"
    fallback: ["general-purpose"]
    
  "category:testing-quality":
    primary: "mock-test-orchestrator" 
    fallback: ["cargo-build-engineer", "general-purpose"]
    
  # Context-aware selection
  "rust + async + performance":
    primary: "rust-async-specialist"
    tools: "rust-development"
    
  "hardware + serial + communication":
    primary: "serial-comm-specialist"
    tools: "rust-development"
    
  "ui + egui + rendering":
    primary: "egui-performance-optimizer"
    tools: "performance-analysis"

  # Complexity-based selection
  "complexity:high + research:true":
    primary: "general-purpose"
    tools: "research-heavy"
    reasoning: "collaborative"
    
  "complexity:expert + critical:true":
    primary: "task-orchestrator"
    tools: "task-coordination"
    parallel: true
```

## Usage Examples

### Tier 1 - Direct Assignment
```yaml
---
model: claude-sonnet-4-20250514
category: task-management
priority: high
assigned-agent: task-orchestrator
required-tools: ["mcp__taskmaster-ai__get_tasks", "mcp__taskmaster-ai__next_task"]
auto-deploy: true
tool-chain: task-coordination
---
```

### Tier 2 - Context-Aware Selection
```yaml
---
model: claude-sonnet-4-20250514
category: performance-optimization
priority: medium
agent-selection-criteria:
  domain-hints: ["rust", "performance", "profiling"]
  complexity-level: "high"
  project-context: "multi-controller-app"
tool-chain: performance-analysis
smart-agent-selection: true
---
```

### Tier 3 - Basic Enhancement
```yaml
---
model: claude-sonnet-4-20250514
category: documentation
priority: medium
fallback-agents: ["general-purpose"]
suggested-tools: ["mcp__context7__*", "mcp__perplexity-ask__*"]
auto-deploy: false
---
```

## Implementation Notes

### Validation Rules
- `assigned-agent` must exist in agent registry
- `required-tools` must be available from configured MCP servers
- `tool-chain` must reference valid template
- `fallback-agents` must provide at least one always-available option

### Performance Considerations  
- Agent deployment adds 1-3s overhead
- Tool chains should be optimized for common workflows
- Parallel execution requires careful resource management
- Fallback systems ensure reliability

### Migration Strategy
- Phase 1: Add schema without breaking existing commands
- Phase 2: Implement Tier 1 direct assignments
- Phase 3: Add context-aware selection
- Phase 4: Universal smart selection deployment