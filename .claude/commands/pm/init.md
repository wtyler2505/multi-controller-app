---
model: claude-sonnet-4-20250514
category: project-management
priority: medium
tags: ["project-management", "setup", "initialization", "workflow"]
description: Project Management Init Command with Universal Integration

# Enhanced Context-Aware Agent Integration with Universal Memory
agent-selection:
  type: "context-aware"
  domain-hints: ["project-management", "initialization", "script-automation", "workflow-setup"]
  complexity-level: "medium"  # Upgraded from simple due to memory integration
  
  # Enhanced selection criteria for PM init with memory integration
  selection-criteria:
    keyword-match: 0.85       # PM/init patterns
    argument-analysis: 0.5    # Script execution with memory context
    project-context: 0.9      # Project state highly relevant for PM
    error-context: 0.6        # Memory helps with error resolution
  
  # Enhanced agent preferences with memory capabilities
  preferred-agents: ["general-purpose", "task-orchestrator"]
  fallback-agents: ["general-purpose"]
  confidence-threshold: 0.75

# Enhanced Tool Selection with Universal Memory Integration
tool-selection:
  type: "intelligent-workflow"
  
  base-tools:
    - "mcp__desktop-commander__start_process"
    - "mcp__cipher-memory__search_nodes"  # Universal memory integration
    - "mcp__taskmaster-ai__initialize_project"  # PM-specific initialization
  
  conditional-tools:
    pm-setup-context:
      - "mcp__taskmaster-ai__get_tasks"  # Check existing tasks
      - "mcp__cipher-memory__open_nodes"  # Load PM patterns
      - "mcp__desktop-commander__list_directory"  # Project structure analysis
    
    error-context:
      - "mcp__desktop-commander__start_process"
      - "mcp__cipher-memory__search_nodes"  # Check for similar issues
      - "mcp__cipher-memory__add_observations"  # Document errors
    
    learning-context:
      - "mcp__cipher-memory__create_entities"  # Store PM setup patterns
      - "mcp__cipher-memory__create_relations"  # Connect PM workflows

# Universal Cipher Memory Integration (MANDATORY FOR ALL COMMANDS)
cipher-memory-integration:
  enabled: true
  priority: "high"
  pre-execution-memory:
    context-search:
      - query-pattern: "pm-init + project-management-setup + initialization-workflow"
      - tools: ["mcp__cipher-memory__search_nodes"]
    pattern-retrieval:
      - query-pattern: "project-initialization + pm-scripts + setup-automation"
      - tools: ["mcp__cipher-memory__open_nodes"]
    workflow-analysis:
      - tools: ["mcp__cipher-memory__read_graph"]
      - filter: "project-management-related"
  execution-memory:
    progress-tracking:
      - tool: "mcp__cipher-memory__add_observations"
      - trigger: "script-execution-milestone"
    pattern-recognition:
      - tool: "mcp__cipher-memory__create_relations"
      - trigger: "successful-pm-setup"
  post-execution-memory:
    workflow-storage:
      - tools: ["mcp__cipher-memory__create_entities"]
      - content: "complete-pm-initialization-pattern"
    relationship-mapping:
      - tools: ["mcp__cipher-memory__create_relations"]
      - relationships: ["pm-tool-to-project-type", "script-to-outcome", "setup-to-workflow"]
    knowledge-enhancement:
      - tools: ["mcp__cipher-memory__add_observations"]
      - content: "pm-best-practices + automation-opportunities + process-improvements"

# Centralized Logging Integration (MANDATORY FOR ALL COMMANDS)
logging-integration:
  enabled: true
  log-file: ".claude/logs/command-execution.jsonl"
  log-level: "comprehensive"
  
  log-phases:
    pre-execution:
      - command-metadata
      - pm-context-analysis
      - script-validation
      - memory-search-results
    
    execution:
      - script-invocation
      - taskmaster-integration
      - progress-tracking
      - error-handling
    
    post-execution:
      - completion-status
      - pm-setup-validation
      - memory-operations
      - workflow-optimization-suggestions
  
  structured-metadata:
    command-id: "pm-init"
    session-id: "${session_timestamp}"
    user-context: "${user_request}"
    project-context: "${project_type}"
    agent-assigned: "${selected_agent}"
    tools-used: "${tool_list}"
    memory-operations: "${cipher_memory_ops}"
    taskmaster-integration: "${tm_operations}"
    script-execution: "${script_results}"
    execution-time: "${duration_ms}"
    success-metrics: "${pm_setup_quality}"

# Enhanced workflow configuration
tool-chain: "universal-project-management"
auto-deploy: true
parallel-execution: false
memory-persistence: true
cross-command-learning: true
taskmaster-integration: true

allowed-tools: Bash, mcp__desktop-commander__*, mcp__cipher-memory__*, mcp__taskmaster-ai__*

pre-execution:
  validate-tools: true
  load-context: true      # Enhanced: Load PM context from memory
  analyze-arguments: true  # Enhanced: Analyze for PM-specific needs
  search-pm-patterns: true
  log-session-start: true

post-execution:
  store-results: true     # Enhanced: Store PM setup patterns
  update-learning: true
  generate-report: false
  persist-pm-knowledge: true
  log-session-complete: true
  update-knowledge-graph: true
---

# Project Management Init Command (Universal Integration)

**ENHANCED WORKFLOW**: Execute PM initialization script with intelligent pattern recognition, Task Master integration, and persistent learning.

## Pre-Execution Memory Analysis
Before script execution, the system will:
1. **Search PM patterns**: Query Cipher Memory for similar project initialization workflows
2. **Load setup context**: Retrieve successful PM configuration patterns
3. **Analyze project state**: Understanding current PM tool landscape
4. **Connect to Task Master**: Integration with existing project task structure

## Intelligent Script Execution

Run `bash .claude/scripts/pm/init.sh` using context-aware agent selection with complete output:

- DO NOT truncate.
- DO NOT collapse.
- DO NOT abbreviate.
- Show ALL lines in full.
- DO NOT print any other comments.

## Enhanced Execution Features

### Task Master Integration
- **Project initialization** with Task Master if not already configured
- **Workflow synchronization** between PM scripts and task management
- **Dependency mapping** between PM tools and project tasks

### Universal Memory Integration Outcomes

#### Pattern Storage
This command will automatically:
- **Store successful PM setup workflows** in Cipher Memory
- **Create relationships** between PM tools, scripts, and project outcomes
- **Document process improvements** and automation opportunities
- **Build knowledge graph** of project management best practices

#### Cross-Command Learning
PM initialization patterns will enhance:
- Future project setup commands through shared PM knowledge
- Task-specific optimizations via workflow pattern recognition
- Script automation intelligence through execution analytics
- Error prevention through documented PM setup pitfalls

#### Centralized Logging
All PM operations logged to `.claude/logs/command-execution.jsonl` including:
- Script execution results and performance metrics
- Task Master integration status and synchronization
- Memory operations for PM knowledge capture
- Workflow optimization recommendations

**Next Commands**: Enhanced PM patterns will automatically improve commands like `pm/status`, `pm/sync`, and other project management workflows.



