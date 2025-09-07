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

### MANDATORY Agent Workflow
1. **ALWAYS start with** → `task-orchestrator` to understand work
2. **For implementation** → `task-executor` automatically deployed
3. **For verification** → `task-checker` when status is 'review'
4. **For issues** → Specialized agent based on problem type
5. **NEVER skip agents** → They work like automatic breathing

## Comprehensive Development Practices
1. **Before ANY work** → Thoroughly understand the problem space through exhaustive search
2. **During implementation** → Document every decision with complete context and reasoning
3. **After completion** → Store comprehensive reasoning traces and implementation patterns
4. **Throughout** → Use ALL appropriate tools properly, regardless of time taken

## Context-Aware Loading
- **Debugging Windows?** → @.claude/debug/windows-fixes.md
- **Need deep reasoning?** → @.claude/reasoning/thoroughness.md
- **Clear-Thought 1.5 operations?** → @.claude/reasoning/clear-thought-operations.md
- **Working with agents?** → @.claude/agents/

## 🔧 MCP Server Architecture (Optimized 2025-01-06)

### Direct Server Configuration (No Aggregator)
9 independent MCP servers provide focused functionality:

| Server | Purpose | Primary Use Cases |
|--------|---------|-------------------|
| **cipher-memory** | Persistent memory & patterns | Knowledge storage, pattern search |
| **taskmaster-ai** | Task management | Task tracking, complexity analysis |
| **desktop-commander** | File & terminal ops | File edits, process management |
| **FileScopeMCP** | Code analysis | Dependency mapping, architecture viz |
| **clear-thought** | 38 reasoning operations | Deep analysis, problem solving |
| **context7** | Documentation lookup | API references, library docs |
| **perplexity-ask** | Web research | Real-time info, best practices |
| **memory** | Additional storage | Supplementary memory operations |
| **time-server** | Time utilities | Scheduling, timestamps |

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

| Metric | Target | Enforcement |
|--------|--------|-------------|
| Startup Time | < 2s | `ValidateStartupPerformance()` |
| Idle CPU | ≤ 2% | Runtime monitoring |
| Base Memory | ≤ 150 MB | Telemetry tracking |
| Serial Latency | ≤ 50ms | `LatencyMonitor` |
| Network Latency | ≤ 100ms | Transport stats |
| Chart FPS | ~30 FPS | 33ms update interval |
| Telemetry Buffer | 2,000+ samples | Ring buffer size |

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
*Excellence through thoroughness: Every tool used properly, every decision documented completely*
- We are in a Windows environment on a Windows OS. ALWAYS use the correct syntax for Windows.