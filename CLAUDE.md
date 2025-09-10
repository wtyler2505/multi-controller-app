# Multi-Controller App - Development Excellence Guide

## Complete Project Understanding

- **Project**: Multi-Controller App (Rust + egui)
- **Standard**: Personal craftsmanship, not commercial speed
- **Philosophy**: Correctness over efficiency, excellence over expedience
- **Branch**: development | **Primary**: main
- **Correctness Requirements**: Zero defects, complete testing, robust error handling

## Core Imports

@.claude/core/excellence.md
@.claude/core/memory-mastery.md
@.taskmaster/CLAUDE.md

## 🤖 Automatic Agent Usage (No Manual Invocation Required)

### Task-Related Triggers (ALWAYS USE)

- **Starting any work session** → `task-orchestrator` (analyzes task queue, deploys executors)
- **Implementing specific task** → `task-executor` (handles actual implementation)
- **Task marked as 'review'** → `task-checker` (verifies against requirements)
- **Multiple tasks available** → `task-orchestrator` (parallelizes work)
- **Task completion** → `task-orchestrator` (reassesses dependency graph)

### Code Problem Triggers (AUTOMATIC)

- **Performance issues/monitoring** → `rust-performance-monitor`
- **Async/await errors** → `rust-async-specialist`
- **Serial/hardware timeout** → `serial-hardware-specialist`
- **Memory leaks/cleanup** → `transport-lifecycle-guardian`
- **Build/compilation fails** → `cargo-build-engineer`
- **Security/credentials** → `rust-security-coordinator`
- **Safety/emergency stops** → `rust-safety-coordinator`
- **egui rendering issues** → `egui-performance-optimizer`
- **Ring buffer/telemetry** → `ring-buffer-architect`
- **Mock/test setup** → `mock-test-orchestrator`
- **Test failures** → `test-runner`

### ⚠️ CRITICAL: Agent Execution vs Documentation

**DOCUMENTATION IS NOT EXECUTION**

#### The Fundamental Rule

When deploying agents, they must ACTUALLY WRITE CODE, not just describe what they would do.

#### Correct Agent Deployment

```javascript
// ❌ WRONG - Just planning/documentation
Task('Describe handshake implementation', 'handshake-protocol-engineer');
// Returns: "I would implement a JSON schema with..."

// ✅ RIGHT - Actual execution with file creation
Task(
  'CREATE src/protocols/handshake/schema.rs with the complete HandshakeMessage implementation RIGHT NOW',
  'handshake-protocol-engineer'
);
// Returns: "Created schema.rs with 250 lines of working code"
```

#### Verification Checkpoints

- **Files Modified**: Agent must report actual files created/edited
- **Code Lines**: Concrete line count of implementation
- **Test Results**: If applicable, test execution results
- **Compilation Status**: For compiled languages, build results

#### Parallel Execution Requirements

Multiple agents CAN and SHOULD run simultaneously:

- Each agent owns their epic/track completely
- Agents make real commits to their assigned files
- Integration points are pre-defined in epic structure

#### 🎯 CRITICAL: The Parallel Execution Pattern That Actually Works

**THE ONLY WAY TO ACHIEVE TRUE PARALLEL EXECUTION:**

```xml
<!-- ✅ CORRECT - TRUE PARALLEL EXECUTION -->
<function_calls>
  <invoke name="Task">
    <parameter name="subagent_type">handshake-protocol-engineer</parameter>
    <parameter name="prompt">CREATE src/protocols/handshake/schema.rs NOW...</parameter>
  </invoke>
  <invoke name="Task">
    <parameter name="subagent_type">ui-controls-architect</parameter>
    <parameter name="prompt">CREATE src/ui/controls/manual_controls.rs NOW...</parameter>
  </invoke>
  <invoke name="Task">
    <parameter name="subagent_type">telemetry-collector</parameter>
    <parameter name="prompt">CREATE src/telemetry/parser.rs NOW...</parameter>
  </invoke>
</function_calls>
<!-- Result: ALL THREE agents execute SIMULTANEOUSLY -->

<!-- ❌ WRONG - SEQUENTIAL EXECUTION (DEFAULT TRAP) -->
<function_calls>
  <invoke name="Task">...</invoke>
</function_calls>
<!-- Then later... -->
<function_calls>
  <invoke name="Task">...</invoke>
</function_calls>
<!-- Result: Only ONE agent at a time, sequential execution -->
```

**MANDATORY RULE**: When deploying multiple agents, they MUST ALL be in the SAME function_calls block!

#### Parallel Execution Verification Checklist

After deploying multiple agents in parallel:

1. **Count the results**: Should see N result blocks for N agents deployed
2. **Check implementation depth**: Each agent should report 100+ lines of code minimum
3. **Verify file creation**: Each agent should create/modify multiple files
4. **No documentation responses**: Agents must return "Created X with Y lines" not "I would create..."
5. **Git verification**: `git status` must show actual file changes from ALL agents

#### Common Parallel Execution Failures to Avoid

- ❌ **Sequential Trap**: Multiple Task calls in separate blocks = sequential
- ❌ **Documentation Response**: Agent returns plans instead of code = not executing
- ❌ **Single Agent Default**: Forgetting to add multiple invokes in one block
- ❌ **Missing Explicit Instructions**: Not using "CREATE <filepath> NOW" = agent confusion
- ❌ **No Verification**: Not checking git status after deployment = blind execution

#### Agent Output Quality Standards

Each deployed agent must meet these minimum criteria:
- **Code Volume**: 100+ lines of actual implementation (not comments)
- **File Creation**: Minimum 3-4 files per agent domain
- **Compilation**: All code must compile without errors
- **Type Safety**: Proper type definitions (no `any` in TS, proper traits in Rust)
- **Test Coverage**: At least one test file per major component
- **Documentation**: Inline doc comments for public APIs

#### Quality Verification Commands
```bash
# After parallel deployment, run:
git diff --stat | grep -E "^\s+\d+ files? changed"  # Verify file count
cargo check  # For Rust projects
npm run typecheck  # For TypeScript projects
```

#### Todo Tracking for Parallel Execution

When deploying multiple agents:
```javascript
// BEFORE deployment
TodoWrite([
  {content: "Deploy agent 1 for Task X", status: "pending"},
  {content: "Deploy agent 2 for Task Y", status: "pending"},
  {content: "Deploy agent 3 for Task Z", status: "pending"},
  {content: "Verify all agents created files", status: "pending"}
])

// DURING deployment - update to "in_progress"
// AFTER deployment - update to "completed" with results
```

This provides visibility into parallel execution progress and ensures nothing is missed.

### MANDATORY Agent Workflow

1. **ALWAYS start with** → `task-orchestrator` to understand work
2. **For implementation** → `task-executor` with EXPLICIT code-writing instructions
3. **For verification** → Check `git status` and `git diff` after each agent
4. **For issues** → Specialized agent based on problem type
5. **NEVER accept** → Documentation or plans as "implementation"

## 🚀 CCPM Epic-Based Parallel Execution

### Epic Conversion Process (Enables Parallelism)

1. **Convert Task to Epic Format**

   ```bash
   /bridge:tm-to-epic <task-id>
   # Creates: .claude/epics/<epic-name>/ with subtask files
   ```

2. **Deploy Multiple Agents Simultaneously**
   ```bash
   /bridge:parallel-start <task1>,<task2>,<task3>,<task4>
   # Launches 4 agent teams in parallel, each working on their epic
   ```

### Parallel Agent Deployment Pattern

```javascript
// Deploy 4 specialized teams simultaneously
const parallelDeployment = {
  'Team 1': {
    agent: 'handshake-protocol-engineer',
    epic: '.claude/epics/handshake-protocol/',
    files: ['001.md', '002.md', '003.md'],
    ownership: 'src/protocols/handshake/**',
  },
  'Team 2': {
    agent: 'cargo-build-engineer',
    epic: '.claude/epics/monorepo-cicd/',
    files: ['001.md', '002.md', '003.md'],
    ownership: '.github/workflows/**',
  },
  // Teams 3 & 4...
};

// Each team executes independently and concurrently
parallelDeployment.forEach((team) => {
  Task(
    `IMPLEMENT ${team.epic}/001.md by creating actual code files`,
    team.agent
  );
});
```

### Integration Coordination Points

- **Week 1**: All teams sync on monorepo structure
- **Week 2**: CI/CD pipeline integration
- **Week 3**: API contract finalization
- **Daily**: Async updates via Task Master

## 🚫 NO SQUIRREL CHASING RULE (CRITICAL)

**CLAUDE MUST HELP USER STAY FOCUSED - NO DIVERSIONS**

- When user gives a task direction, COMPLETE IT before considering other work
- If user gets distracted by other issues/ideas, REMIND them to finish current work first
- NEVER start new work streams until the current task is 100% complete
- Use phrases like: "Let's finish the command integration first, then handle the issues"
- This rule OVERRIDES all other considerations - focus trumps everything

### 🐿️ Smart Squirrel Capture System

**WHEN SQUIRRELS APPEAR - CAPTURE, DON'T CHASE**

- **Immediate Response**: "Great idea! Let me capture this for after we finish [current task]"
- **Auto-Capture**: Document ALL squirrels in `.claude/squirrel-log.jsonl` with full context
- **Smart Context**: Record what was interrupted, why squirrel appeared, effort estimate
- **Categorization**: urgent_bug, feature_request, optimization, technical_debt, research, external_dependency
- **Review Planning**: Schedule captured squirrels based on priority and current task completion

#### Capture Format (Auto-Generated):

```json
{
  "timestamp": "ISO-8601",
  "squirrel_type": "category",
  "description": "what user wanted to do",
  "current_context": "what was interrupted",
  "priority_hint": "urgent/high/medium/low",
  "estimated_effort": "time estimate",
  "dependencies": ["what must finish first"],
  "capture_reason": "why this came up"
}
```

#### Sample Squirrel Capture:

- **User**: "Oh we should fix the GitHub issues too"
- **Claude**: "Excellent point! Let me capture that: _[Auto-logs: GitHub issues #2-#11 resolution, medium priority, 3-4 hours, depends on command integration completion]_ - Now back to batch 7 of command integration!"

## 📊 Critical Chain Project Management (CCPM)

### Core CCPM Principles for Agent Orchestration

#### 1. Critical Chain Identification

```yaml
Critical Path (Task 28): Handshake Protocol → Transport Layer → Device Drivers
Buffer Tasks: UI (Task 8), Documentation (Task 12), Testing (Task 10)
```

- Prioritize agents on critical chain tasks
- Use buffer tasks for parallel work that won't block progress

#### 2. Buffer Management Strategy

- **Project Buffer**: 20% time allocation at end of critical chain
- **Feeding Buffers**: 10% time where non-critical meets critical
- **Resource Buffers**: Extra agent capacity for critical tasks

#### 3. Resource Allocation Pattern

```javascript
const resourceAllocation = {
  criticalChain: ['agent1', 'agent2'], // 2 agents on critical path
  feedingPaths: ['agent3', 'agent4'], // 2 agents on parallel tracks
  bufferAgents: ['agent5'], // 1 floating agent for bottlenecks
};
```

#### 4. Aggressive Duration Estimates

- Cut traditional estimates by 50% (CCPM principle)
- Rely on buffers to absorb variation
- Focus on flow, not individual task completion

#### 5. Multi-Tasking Prohibition

- Each agent works on ONE epic at a time
- No context switching until epic complete
- Protect agent focus for maximum throughput

## ✅ Execution Verification Protocol

### Mandatory Verification After Each Agent Deployment

#### 1. Code Creation Verification

```bash
# After agent deployment, verify actual files created
git status  # Must show new/modified files
git diff --stat  # Must show line count changes
```

#### 2. Implementation Metrics

Each agent must report:

- **Files Created**: List of new files with paths
- **Files Modified**: List of edited files with line changes
- **Code Lines**: Total lines added/modified
- **Tests Written**: Number of test cases created
- **Build Status**: Compilation/linting results

#### 3. Anti-Pattern Detection

Watch for these signs of fake execution:

- ❌ No `git status` changes after "implementation"
- ❌ Agent returns only markdown or documentation
- ❌ Description of what "would be done" vs "was done"
- ❌ No concrete file paths in response
- ❌ Missing line numbers or code snippets

#### 4. Remediation for Fake Execution

If agent returns documentation instead of code:

1. STOP immediately
2. Re-deploy with explicit instruction: "WRITE THE ACTUAL CODE IN <filepath> NOW"
3. Verify with `git diff <filepath>`
4. Only proceed after confirmation of real changes

## Comprehensive Development Practices

1. **Before ANY work** → Thoroughly understand the problem space through exhaustive search
2. **During implementation** → Verify agents are writing code with `git status` checks
3. **After completion** → Store comprehensive reasoning traces and implementation patterns
4. **Throughout** → Use ALL appropriate tools properly, regardless of time taken

## Context-Aware Loading

- **Debugging Windows?** → @.claude/debug/windows-fixes.md
- **Need deep reasoning?** → @.claude/reasoning/thoroughness.md
- **Clear-Thought 1.5 operations?** → @.claude/reasoning/clear-thought-operations.md
- **Working with agents?** → @.claude/agents/
- **Epic conversion?** → @.claude/epics/
- **Bridge commands?** → @.claude/commands/bridge/

## 🔧 MCP Server Architecture (Optimized 2025-01-06)

### Direct Server Configuration (No Aggregator)

9 independent MCP servers provide focused functionality:

| Server                | Purpose                      | Primary Use Cases                    |
| --------------------- | ---------------------------- | ------------------------------------ |
| **cipher-memory**     | Persistent memory & patterns | Knowledge storage, pattern search    |
| **taskmaster-ai**     | Task management              | Task tracking, complexity analysis   |
| **desktop-commander** | File & terminal ops          | File edits, process management       |
| **FileScopeMCP**      | Code analysis                | Dependency mapping, architecture viz |
| **clear-thought**     | 38 reasoning operations      | Deep analysis, problem solving       |
| **context7**          | Documentation lookup         | API references, library docs         |
| **perplexity-ask**    | Web research                 | Real-time info, best practices       |
| **memory**            | Additional storage           | Supplementary memory operations      |
| **time-server**       | Time utilities               | Scheduling, timestamps               |

### Optimized Tool Selection Patterns

```bash
# Memory Operations (Hierarchical approach)
mcp__cipher-memory__* → mcp__memory__* → filesystem

# Task Management (Structured workflow)
mcp__taskmaster-ai__* → TodoWrite → manual tracking

# Code Analysis (Efficiency cascade)
mcp__FileScopeMCP__* → Grep → Read

# Research (Authority hierarchy)
mcp__perplexity-ask__* → mcp__context7__* → WebSearch

# Reasoning (Complexity-based)
mcp__clear-thought__* (complex) → sequential thinking (simple)
```

## Complete Workflows

```bash
# Thorough task understanding
mcp__taskmaster-ai__get_task --id=X     # Full context
mcp__taskmaster-ai__get_tasks --status=pending  # Overview
mcp__taskmaster-ai__set_task_status     # After verification

# Comprehensive memory operations
mcp__cipher-memory__search("pattern")   # Direct memory search
mcp__cipher-memory__store_entities()    # Store patterns
mcp__memory__create_entities()          # Additional storage

# Critical Discovery Storage Pattern
When discovering important patterns or solutions:
```javascript
// IMMEDIATELY store breakthrough discoveries
mcp__cipher-memory__ask_cipher({
  message: "CRITICAL DISCOVERY - Store this pattern: [detailed description]"
})

// Include: problem context, solution pattern, verification steps, common failures
```

**Trigger conditions for storage**:
- Solving a frustrating problem (like parallel execution)
- Discovering undocumented behavior
- Finding optimal configuration patterns
- Debugging complex issues

# Clear-Thought 1.5 reasoning (38 operations)
mcp__clear-thought__sequentialthinking  # Step-by-step analysis
mcp__clear-thought__mentalmodel         # First principles
mcp__clear-thought__collaborativereasoning # Multiple perspectives

# File operations (Windows-optimized)
mcp__desktop-commander__read_file       # Fast file reading
mcp__desktop-commander__edit_block      # Surgical edits
mcp__FileScopeMCP__find_important_files # Code analysis

# Proper verification
grep -n "pattern" file.rs               # Verify exact changes
cargo test -- --nocapture               # Full test output
cargo clippy -- -W clippy::all         # Complete linting
```

## Current Session Metrics

- Memory searches performed: X (target: thorough coverage)
- Patterns properly stored: Y (with complete context)
- Reasoning traces archived: Z (fully evaluated first)
- Tasks completed correctly: W (zero defects)

## Verification Checkpoints

- Task understanding: `mcp__taskmaster-ai__get_task` with full details
- Memory coverage: `cipher_memory_search()` until no new patterns found
- Implementation correctness: Full test suite passes
- Documentation completeness: Every decision recorded

## 🦀 Rust-Specific Patterns (CRITICAL)

### Must-Follow Transport Patterns

```rust
// ALWAYS follow this lifecycle pattern
transport.cleanup_resources().await?;  // FIRST - prevent memory leaks
transport.disconnect().await?;         // THEN disconnect

// ALWAYS use Arc for transport sharing
let transport: Arc<dyn Transport> = Arc::new(serial_transport);

// ALWAYS use TransportResult<T> for fallible operations
async fn send_data(transport: &mut dyn Transport) -> TransportResult<()> {
    transport.send(&data).await
}
```

### Async Safety Rules

- **NEVER** hold Mutex/RwLock guards across `.await` points
- **ALWAYS** use `tokio::sync` primitives, not `std::sync` in async code
- **ALWAYS** handle task cleanup with `JoinHandle` abort on disconnect

### Mock Testing Pattern (Use for ALL driver tests)

```rust
let fixture = DriverTestFixture::with_device(MockDeviceType::ArduinoUno);
fixture.connect().await.unwrap();
let transport: Arc<dyn Transport> = Arc::new(fixture.transport.lock().await.clone());
```

### Latency Enforcement Pattern

```rust
use crate::transport::monitor::LatencyMonitor;

let monitor = LatencyMonitor::new(Duration::from_millis(50));
monitor.enforce_latency().await;  // ALWAYS use for serial operations
```

## 📋 Task-Specific Guidance

### Task 9.x (Telemetry & Profiles)

- Use `egui_plot` version 0.29 (must match egui version)
- Maintain 30 FPS update rate with 33ms intervals
- Implement data decimation to limit chart points to 300
- Use ring buffers with 2,000+ sample capacity
- Profile hot-reload via `notify` crate and TOML format

### Task 10.x (Testing & Acceptance)

- Unit tests: Use `MockTransport` for all hardware simulation
- Loopback tests: Verify with `DriverTestFixture` pattern
- Soak tests: **MUST** run for 8+ hours minimum
- Hardware tests: Guard with `#[cfg(feature = "hardware-tests")]`
- Coverage target: 80% minimum, verify with `cargo tarpaulin`

### Task 15.x (TypeScript Type Safety)

- Replace **ALL** `any` types with explicit interfaces
- Define proper types for `stream` and `connectConfig`
- Enable TypeScript strict mode with `noImplicitAny`
- Verify with `npm run typecheck` before commits

### Task 16.x (SSH Security)

- **NEVER** store passwords in plaintext
- Use OS credential vaults:
  - Windows: Credential Manager
  - macOS: Keychain
  - Linux: Secret Service
- Implement AES-256 encryption for credential storage
- Clear decrypted secrets from memory after use

### Task 17.x (Performance Monitoring)

- Activate `ValidateStartupPerformance()` in App constructor
- Poll CPU/RAM on background thread every 1s
- Log violations to telemetry system
- Thresholds: <2s startup, ≤2% CPU, ≤150MB RAM

### Task 19.x (TypeScript Path Aliases)

- Install `tsconfig-paths` package
- Configure `baseUrl` and `paths` in tsconfig.json
- Update Node.js entrypoints with `ts-node` and `tsconfig-paths/register`

## 🔨 Build and Compilation Management

### Git Workflow Patterns

#### Handling Pre-Commit Hook Failures
When ESLint or other hooks block commits:
```bash
# If warnings are blocking (not errors), use --no-verify
git commit --no-verify -m "commit message"

# For branch protection requiring PRs:
git checkout -b feature/new-branch-name
git push -u origin feature/new-branch-name
gh pr create --base main --title "..." --body "..."
```

#### Comprehensive Commit Messages
Use heredoc for detailed multi-line commits:
```bash
git commit -m "$(cat <<'EOF'
feat: brief description

## What Changed
- Detail 1
- Detail 2

## Implementation Details
...

🤖 Generated with [Claude Code](https://claude.ai/code)
Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

### Handling Cargo Build Locks

When encountering "Blocking waiting for file lock on build directory":

1. Check for existing cargo processes: `tasklist | findstr cargo`
2. Clean stale locks if needed: `del /f /q target\.cargo-lock 2>nul`
3. For long builds, use background execution: `cargo build --release` with longer timeout
4. Monitor build progress: Check `target\debug\build` or `target\release\build` directories

### Build Best Practices

- First builds compile all dependencies (5-10 minutes typical)
- Use `cargo build` before `cargo run` for better control
- For release builds: `cargo build --release` (optimized but slower)
- Check Cargo.toml for workspace members requiring separate builds

### Windows Process Management

- List processes: `tasklist | findstr <process_name>`
- Kill process: `taskkill /F /PID <pid>` or `taskkill /F /IM <process.exe>`
- Check port usage: `netstat -ano | findstr :<port>`
- Clean up locks: `del /f /q <lock_file>` (use `/f` for force, `/q` for quiet)
- Background processes: Always track PIDs and clean up on failure

## 🧪 Testing Protocol

### Before EVERY Commit

1. `cargo test` - Run all unit tests
2. `cargo test --features hardware-tests` - With real hardware only
3. `cargo clippy -- -W clippy::all` - Linting check
4. `cargo fmt --check` - Format verification
5. `cargo tarpaulin --out Html` - Coverage report (Linux/macOS)

### Test Categories & Patterns

- **Unit Tests**: Mock all I/O with `MockTransport`
- **Integration Tests**: Use `DriverTestFixture` for driver testing
- **Loopback Tests**: Verify protocol handling
- **Performance Tests**: Validate latency and throughput
- **Soak Tests**: 8+ hour stability verification

## 🔧 Error Recovery Patterns

### Common Windows Build Issues

1. **File Lock Errors**
   - Check: `tasklist | findstr cargo`
   - Clean: `del /f /q target\.cargo-lock`
   - Retry with fresh build: `cargo clean && cargo build`

2. **Node Module Conflicts**
   - Check: `dir node_modules\<package>`
   - Clean: `rmdir /s /q node_modules && pnpm install`
   - For stubborn directories: `powershell -Command "Remove-Item -Path node_modules -Recurse -Force"`

3. **Long-Running Builds**
   - Run in background with adequate timeout (600000ms for first builds)
   - Monitor with `dir target\debug\deps | findstr /c:".rlib"`
   - Check progress via file timestamps in target directory

### Build and Install Operation Timeouts

- **pnpm install**: 600000ms (10 minutes) for full install
- **cargo build (first)**: 600000ms minimum
- **cargo run**: 300000ms for subsequent runs
- **cargo test**: 120000ms for test suite
- Always verify completion before proceeding to dependent steps

## ⚠️ Common Pitfalls to Avoid

### Memory & Resource Management

- ❌ Missing `cleanup_resources()` before disconnect
- ❌ Holding Arc references preventing cleanup
- ❌ Not aborting spawned tasks on disconnect
- ❌ Memory leaks in reconnection cycles

### Async & Concurrency

- ❌ Holding Mutex guards across `.await` points
- ❌ Using `std::sync` instead of `tokio::sync` in async
- ❌ Not handling task panics properly
- ❌ Deadlocks from nested lock acquisition

### Error Handling

- ❌ Using `unwrap()` in production code
- ❌ Ignoring `TransportResult` errors
- ❌ Not implementing proper backoff for retries
- ❌ Swallowing errors without logging

### Performance

- ❌ Ignoring 50ms serial latency budget
- ❌ Not using `LatencyMonitor` for enforcement
- ❌ Excessive allocations in hot paths
- ❌ Missing data decimation for telemetry

### Process Management

- ❌ Not checking for existing processes before starting new ones
- ❌ Leaving failed background processes running
- ❌ Using inadequate timeouts for long operations
- ❌ Not cleaning up file locks after process failures

## 📊 Performance Requirements

| Metric           | Target         | Enforcement                    |
| ---------------- | -------------- | ------------------------------ |
| Startup Time     | < 2s           | `ValidateStartupPerformance()` |
| Idle CPU         | ≤ 2%           | Runtime monitoring             |
| Base Memory      | ≤ 150 MB       | Telemetry tracking             |
| Serial Latency   | ≤ 50ms         | `LatencyMonitor`               |
| Network Latency  | ≤ 100ms        | Transport stats                |
| Chart FPS        | ~30 FPS        | 33ms update interval           |
| Telemetry Buffer | 2,000+ samples | Ring buffer size               |

## 🏗️ Architecture Patterns

### Plugin Architecture

```rust
// Dynamic driver loading
let driver = load_driver_plugin("arduino_uno")?;
driver.probe_async(transport).await?;
```

### Device Session Lifecycle

```rust
let session = driver.create_session(transport).await?;
// Use session for operations
session.cleanup().await?;  // ALWAYS cleanup
```

### Transport Abstraction

```rust
#[async_trait]
pub trait Transport: Send + Sync {
    async fn connect(&mut self) -> TransportResult<()>;
    async fn send(&mut self, data: &[u8]) -> TransportResult<()>;
    async fn receive(&mut self, timeout: Duration) -> TransportResult<Vec<u8>>;
    async fn cleanup_resources(&mut self) -> TransportResult<()>;
}
```

---

_Excellence through thoroughness: Every tool used properly, every decision documented completely_
_Parallel execution through CCPM: Multiple agents writing real code simultaneously_

- We are in a Windows environment on a Windows OS. ALWAYS use the correct syntax for Windows.
