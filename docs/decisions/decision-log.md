# Decision Log

## 2025-09-02: Task 21 Complete - Cipher MCP Aggregation Fully Operational
**Context**: Task 21 "Install and Configure Cipher Memory Framework" completed successfully
**Achievement**: All 8 MCP servers now aggregated through Cipher v0.2.2 with 106 tools available
**Components Working**:
- Task Master AI (36 tools)
- Desktop Commander (21 tools)  
- FileScopeMCP (18 tools)
- Clear Thought (9 tools)
- Context7 (2 tools)
- Perplexity Ask (1 tool)
- Memory (9 tools)
- Time Server (2 tools)
- Cipher Internal (8 tools)
**New Agent**: Created cipher-orchestrator agent for all Cipher-related operations
**Impact**: Single MCP connection point, persistent memory, knowledge graphs, 18% project completion

## 2025-09-02: Fixed Cipher Memory Operations - maxIterations Undefined Error
**Context**: Cipher v0.2.2 connected successfully but memory operations failing with "Cannot read properties of undefined (reading 'maxIterations')" error when using mcp__cipher-aggregator__ask_cipher
**Root Cause**: cipher.yml missing required llm configuration section with maxIterations property, even in MCP mode
**Solution**: Added complete llm configuration block with maxIterations: 50, fixed systemPrompt structure, corrected embedding.type (was provider), added agent configuration section
**Impact**: Memory operations now functional, enables persistent memory across Claude Code sessions

## 2025-08-26: Cipher Memory Framework as MCP Aggregator Hub
**Context**: Needed unified access to all 8 MCP servers with persistent memory capabilities
**Decision**: Configured Cipher v0.2.2 as MCP server aggregator, NOT standalone client
**Implementation**: 
- Single entry point in .mcp.json pointing to Cipher
- Cipher aggregates all other MCP servers (Task Master, Desktop Commander, etc.)
- Uses Claude Code's API instead of requiring separate API keys
- Local in-memory vector store for development (512MB)
**Resolved Issues**:
- Task Master AI path: `/mcp-server/server.js` not `/dist/index.js`
- Time Server: Using custom path at `RoverMissionControl/kite-mcp-server-time`
- API Keys: Not needed - Cipher uses Claude Code's existing API access
**Impact**: All MCP tools accessible through single aggregator with added memory persistence

## 2025-09-03: Cipher Memory Fixed & Vibe-tree Integrated
- **Context**: Cipher memory not persisting, embeddings timing out (30s), SQLite DB not created
- **Change**: Created SQLite schema, fixed env vars (USE_MEMORY_ONLY=false), verified Ollama pipeline  
- **Impact**: Memory operations <5s, embeddings stored persistently, Vibe-tree enables 5 parallel sessions

This log records architectural and technical decisions made during the development of the Windows Multi‑Controller App. Each entry follows an ADR‑style format: Title, Date, Status, Context, Decision and Consequences. Subsequent entries will be appended chronologically. See [architecture.md](../architecture/architecture.md) and [PRD.md](../../PRD.md) for more context.

## 2025‑08‑23 – Evaluate Programming Language & UI Framework

- **Status:** Proposed
- **Context:** The project requires a lightweight GUI on Windows with a single portable executable, fast start‑up and low memory usage. Two candidate technology stacks have been identified: C# with WPF/WinUI 3 compiled using .NET 8 Native AOT, and Rust with a minimal Win32/Rust UI library.
- **Decision:** Build and measure minimal prototypes in both stacks. The C# prototype will use WPF with the `<PublishAot>true</PublishAot>` and `<PublishTrimmed>true</PublishTrimmed>` properties to enable Native AOT. The Rust prototype will use a lightweight GUI (e.g., egui) compiled in release mode. Measurements will include application size, start‑up time, idle CPU and memory usage. The final selection will prioritise meeting the performance budgets and developer productivity.
- **Consequences:** Additional initial effort is required to implement two prototypes and collect metrics, but this de‑risks the choice of stack. Should the Rust prototype significantly outperform the C# version, the team may opt for Rust despite the steeper learning curve. The decision will be revisited after the prototypes are analysed.

## 2025‑08‑24 – Implement Verification-First Development and Enhanced Operational Guidance

- **Status:** Implemented
- **Context:** The project's CLAUDE.md file lacked critical operational sections causing inefficient development, false implementation claims, and missing standards. Specifically missing: Task Management Protocol, File Management Rules, Performance Validation Protocol, Technology Decision Matrix details, Code Reference Standards, and Self-Verification requirements.
- **Decision:** Implemented comprehensive enhancements to CLAUDE.md adding 294 lines across 6 major sections: (1) Verification-First Development principles at the top of the file, (2) Task Management Protocol with mandatory pre-work verification, (3) File Management Rules with absolute prohibitions on unnecessary file creation, (4) Performance Validation Protocol with measurement methods and regression response, (5) Expanded Technology Decision Matrix with implementation guidelines, (6) Code References standards requiring file:line format, and (7) Anti-Bullshit Measures including self-verification protocols and penalties for false claims.
- **Consequences:** All future development must follow verification-first principles - never claiming implementation without proof. File creation is now strictly prohibited unless explicitly requested. Task management via TaskMaster is mandatory before starting work. Performance budgets must be validated during implementation. Code references must use file:line format. These changes prevent false claims and ensure honest, verifiable development practices.

## 2025-01-24 – Git/GitHub Automation System Implementation

- **Status:** Implemented
- **Context:** The Multi-Controller App project needed intelligent automation for git workflows to ensure code quality, maintain performance budgets, and integrate with Task Master for task-driven development. Manual git operations were error-prone and didn't enforce project standards consistently.
- **Decision:** Implemented a comprehensive three-phase git automation system: Phase 1 (Safety) - secrets scanning, performance validation, git hooks; Phase 2 (Task Integration) - task-branch mapping, smart commits, PR automation; Phase 3 (Synchronization) - sync status dashboard with auto-fix capabilities. Used Node.js for cross-platform scripts, PowerShell/Bash dual implementation for OS-specific features, and configuration-driven patterns in `.gitmeta/config/`.
- **Consequences:** All commits now undergo automatic validation for secrets and performance. Task-driven development is enforced through branch naming and commit messages. PRs include task context and validation checklists. The system adds ~3-5 seconds overhead per commit but prevents critical issues from reaching the repository. Requires Node.js and Git Bash on Windows. Successfully implemented all 9 automation components with npm scripts for easy access.

## 2025-08-25 – Technology Stack Final Decision: Rust Selected

- **Status:** Decided
- **Context:** Completed comprehensive performance benchmarking of C# Native AOT vs Rust prototypes for the Multi-Controller App. Both implementations included full WinUI 3 and egui UIs respectively, with serial port echo functionality. Testing measured startup time, memory usage, CPU usage, and binary size against our performance budgets (startup <2s, RAM ≤150MB, CPU ≤2%).
- **Decision:** **Selected Rust with egui framework** based on superior performance across all metrics. Benchmark results: Rust achieved 231ms startup (vs C# 699ms), 68MB memory usage (vs C# 78MB), 0% idle CPU (tied), and 4MB single-file distribution (vs C# 62MB across 60+ files). Both prototypes met all performance budgets, but Rust's 3x faster startup and 14x smaller distribution size are compelling advantages for a device control application.
- **Consequences:** The project will proceed with Rust implementation, accepting the trade-offs of longer initial development time and steeper learning curve. Benefits include superior performance (critical for real-time device control), simpler distribution (single 4MB executable), better resource efficiency (important for multi-device scenarios), and stronger guarantees for hardware control safety. The C# prototype will be maintained as a reference implementation. Team will need Rust training, but the performance advantages justify this investment. Review scheduled for Q1 2026 after initial production deployment.

## 2025-08-25 – Task Master Template Loading Issue Resolution

- **Status:** Fixed
- **Context:** Task Master MCP server was failing to load prompt templates with errors "Prompt template 'expand-task' not found" and "Prompt template 'analyze-complexity' not found". Investigation revealed the project was using a fixed path to task-master-ai installation in .mcp.json that didn't exist, and missing critical prompt templates in .taskmaster/templates/ directory.
- **Decision:** Fixed the issue through three changes: (1) Updated .mcp.json to use "pnpm dlx task-master-ai" instead of fixed node path for dynamic package resolution, (2) Created missing prompt templates: expand-task.md, analyze-complexity.md, add-task.md, and update-task.md in .taskmaster/templates/, (3) Templates follow Task Master's expected format with proper handlebars variables and comprehensive instructions for AI-powered task management.
- **Consequences:** Task Master MCP tools (analyze_project_complexity, expand_task, add_task, update_task) can now load their prompt templates successfully. The pnpm dlx approach ensures latest version is always used without local installation dependencies. Template standardization improves consistency of AI-generated task breakdowns and complexity analysis. Project now fully supports Task Master's advanced features for parallel execution and automated task management.
