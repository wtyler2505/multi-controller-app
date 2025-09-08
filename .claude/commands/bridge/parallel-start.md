---
model: claude-sonnet-4-20250514
category: bridge-integration
priority: medium
tags: ["bridge-integration"]
description: Bridge: Parallel Start from Task Master

# Agent and Tool Integration
assigned-agent: task-orchestrator
required-tools:
  - "mcp__taskmaster-ai__get_tasks"
  - "mcp__taskmaster-ai__get_task"
  - "mcp__taskmaster-ai__set_task_status"
  - "mcp__desktop-commander__start_process"
tool-chain: task-coordination
auto-deploy: true
parallel-execution: true
max-agent-count: 6

# Multi-Agent Deployment Configuration
agent-deployment-map:
  driver-work: "serial-comm-specialist"
  transport-work: "serial-comm-specialist"
  ui-work: "egui-performance-optimizer"
  performance-work: "rust-performance-monitor"
  testing-work: "mock-test-orchestrator"
  documentation-work: "general-purpose"

# Workflow Configuration
pre-execution:
  validate-tools: true
  load-context: true
  prepare-environment: true
post-execution:
  store-results: true
  update-tasks: true
  generate-report: true

allowed-tools: Read, Write, LS, Bash, Task, mcp__taskmaster-ai__get_task, mcp__taskmaster-ai__set_task_status
---

-|
| Driver implementation | driver-engineer |
| Serial/TCP/UDP | transport-engineer |
| UI components | ui-telemetry-analyst |
| Performance testing | performance-profiler |
| Security validation | security-hygiene |
| Documentation | docs-scribe |

## Error Handling
- If worktree exists, use existing
- If agents fail, report and continue others
- Maintain Task Master sync throughout


