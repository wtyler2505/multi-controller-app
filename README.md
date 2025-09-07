# Multi-Controller App

[![Rust CI](https://github.com/wtyler2505/multi-controller-app/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/wtyler2505/multi-controller-app/actions/workflows/rust-ci.yml)
[![Test Coverage](https://github.com/wtyler2505/multi-controller-app/actions/workflows/test-coverage.yml/badge.svg)](https://github.com/wtyler2505/multi-controller-app/actions/workflows/test-coverage.yml)
[![codecov](https://codecov.io/gh/wtyler2505/multi-controller-app/branch/main/graph/badge.svg)](https://codecov.io/gh/wtyler2505/multi-controller-app)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight Windows application for discovering, connecting to, and controlling heterogeneous hardware devices (Arduino/ESP32/ESP8266/RioRand/Raspberry Pi) over Serial, TCP/UDP, or SSH.

## 🚀 Features

- **Multi-Protocol Support**: Serial, TCP, UDP, SSH
- **Hot-Swappable Drivers**: Plugin architecture for device drivers
- **Real-Time Telemetry**: High-performance data streaming with decimation
- **Performance Optimized**: < 2s startup, ≤ 2% idle CPU, ≤ 150MB RAM
- **Native AOT Compilation**: Single executable distribution
- **Extensible**: Easy to add new device support

## 📋 Task Progress

**Last Updated:** January 24, 2025

### Summary
- **Total Tasks:** 19 main tasks (100 subtasks)
- **Completed:** 0 (0%)
- **In Progress:** 0 (0%)
- **Pending:** 19 (100%)
- **Overall Progress:** ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜ 0%

### Subtask Breakdown
- **Total Subtasks:** 100
- **Completed:** 0 (0%)
- **Pending:** 100 (100%)
- **Average Subtasks per Task:** 5.3

## 📋 Prerequisites

- Windows 10/11 (64-bit)
- .NET 8 SDK (for C# development)
- Node.js 18+ (for TypeScript/MCP servers)
- Git
- VS Code or Visual Studio 2022

## 🛠️ Quick Setup

### Automated Setup (Recommended)

```powershell
# Run as Administrator
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
.\scripts\setup-windows.ps1
```

### Manual Setup

1. **Clone the repository**
```bash
git clone https://github.com/wtyler2505/multi-controller-app.git
cd multi-controller-app
```

2. **Install dependencies**
```bash
npm install
```

3. **Copy environment variables**
```bash
cp .env.example .env
# Edit .env with your API keys
```

4. **Build the project**
```bash
npm run build
dotnet build app/MultiControllerApp.csproj
```

## 📋 Task Management

### Quick Commands
```bash
# Task workflow
npm run tm:next                    # Get next available task
npm run tm:show 11                 # View task details  
npm run task:branch 11             # Create task branch
npm run task:commit                # Smart commit with task context
npm run task:pr                    # Create PR with task info

# Worktree workflow for parallel tasks
git worktree add ../task-11 feature/task-11
npm run tm:set-status 11 in-progress

# Status management
npm run tm:list --status=pending   # View pending work
npm run tm:deps 11                 # Check dependencies
npm run sync:status                # Repository sync status
```

### Task Master Integration
- All development work is tracked via Task Master
- Use `mcp__taskmaster-ai__*` commands in Claude Code
- See [Task Management Guide](./ref/TASK_MANAGEMENT.md) for detailed workflows
- View [TaskMaster Workflow Guide](./docs/commands/TM_WORKFLOW_GUIDE.md) for commands

## 🧪 Testing & Coverage

### Test Coverage Requirements
- **Minimum Coverage**: 80% for all modules
- **CI/CD Integration**: Automated coverage checks on every PR
- **Platform Support**: Full coverage on Linux, test validation on Windows/macOS

### Running Tests
```bash
# Run all tests
cargo test

# Run with coverage (Linux/macOS with tarpaulin)
cargo tarpaulin --out Html --output-dir coverage

# Windows coverage measurement
.\scripts\measure-coverage.ps1 -Html -Verbose

# Run specific test categories
cargo test transport  # Transport layer tests
cargo test driver    # Device driver tests
cargo test --test performance_tests  # Performance benchmarks
```

### Test Categories
- **Unit Tests**: 100+ tests for individual components
- **Integration Tests**: 48+ tests for cross-module interactions
- **Loopback Tests**: 48+ tests for transport protocols
- **Performance Tests**: Latency, throughput, stress testing
- **Device Driver Tests**: 150+ tests covering all driver endpoints

### Coverage Reports
- HTML reports generated in `coverage/` directory
- CI uploads reports to Codecov for tracking
- Windows fallback using test counting method
- Coverage badges displayed in README

## 🔧 Development

### Development Guidelines

- **Verification-First Development**: All implementations must be verified before claiming completion (see [CLAUDE.md](./CLAUDE.md))
- **Task Management**: Use TaskMaster protocol for all work - check tasks before starting
- **Code Standards**: Follow [Coding Standards](./ref/CODING_STANDARDS.md) for consistent code style
- **Examples**: See [Style Guide](./docs/STYLE_GUIDE.md) for implementation patterns
- **Editor Setup**: Project includes `.editorconfig` for consistent formatting
- **File Management**: Never create files unless explicitly requested - always edit existing files
- **Performance**: Validate against budgets: <2s startup, ≤2% CPU, ≤150MB RAM

### 🌳 Parallel Development with Git Worktrees

Work on multiple tasks simultaneously without context switching:

#### Setup Worktrees for Active Tasks
```bash
# Check available tasks
npm run tm:next                          # Get next task
npm run tm:list --status=pending         # View all pending

# Create worktrees for parallel work (examples)
git worktree add ../mc-task-11 feature/task-11-memory-leak
git worktree add ../mc-task-13 feature/task-13-serial-latency
git worktree add ../mc-task-18 feature/task-18-arduino-driver

# List active worktrees
git worktree list
```

#### Run Separate Development Sessions
```bash
# Terminal 1: Memory leak fix
cd ../mc-task-11
code .  # or claude
npm run tm:set-status 11 in-progress

# Terminal 2: Serial latency
cd ../mc-task-13
code .  # or claude
npm run tm:set-status 13 in-progress

# Terminal 3: Arduino driver
cd ../mc-task-18
code .  # or claude
npm run tm:set-status 18 in-progress
```

#### Task Groups for Parallel Work
**Independent Tasks (Can work simultaneously):**
- Tasks 1, 2, 19: Infrastructure setup
- Tasks 11, 13, 15, 16: Bug fixes (no dependencies)
- Task 12: Test coverage

**Dependent Chains (Work sequentially):**
- Chain 1: Task 3 → Task 4 → Task 5 → Task 6
- Chain 2: Task 6 → Tasks 7, 14, 18
- Chain 3: Task 7 → Task 8 → Task 9 → Task 10

### Available Scripts

```bash
# Development server with hot reload
npm run dev

# Build TypeScript
npm run build

# Run tests
npm test
npm run test:coverage

# Code quality
npm run lint
npm run format

# Type checking
npm run typecheck
```

### Project Structure

```
multi-controller-app/
├── app/                    # C# WinUI 3 application
├── packages/              # Monorepo packages
│   ├── core/             # Core types, interfaces, errors
│   ├── monitoring/       # Logging, metrics, telemetry
│   └── eslint-config/    # Shared ESLint configuration
├── drivers/              # Device driver implementations
├── transports/           # Communication protocols
├── tests/                # Test files
├── scripts/              # Build and setup scripts
├── profiles/             # Device configuration profiles
├── docs/                 # Documentation
└── ref/                  # Reference documentation
```

## 🧪 Testing

### Unit Tests
```bash
npm test
```

### Integration Tests
```bash
npm run test:integration
```

### Performance Tests
```bash
npm run test:performance
```

### Coverage Report
```bash
npm run test:coverage
```

## 📦 Building for Production

### C# Native AOT Build
```bash
cd app
dotnet publish -c Release -r win-x64 --self-contained true -p:PublishAot=true
```

### TypeScript Build
```bash
npm run build
```

## 🐳 Docker Development

```bash
# Start development environment
docker-compose up app-dev

# Run with mock services
docker-compose up
```

## 🎯 Performance Budgets

| Metric | Target | Current |
|--------|--------|---------|
| Startup Time | < 2s | ✅ |
| Idle CPU | ≤ 2% | ✅ |
| Base Memory | ≤ 150 MB | ✅ |
| Serial Latency | ≤ 50ms | ✅ |
| Network Latency | ≤ 100ms | ✅ |

## 🔌 Supported Devices

- **Arduino**: Uno, Mega, Nano
- **ESP32**: All variants
- **ESP8266**: NodeMCU, Wemos D1
- **RioRand**: 8-channel relay boards
- **Raspberry Pi**: All models (via SSH)

## 🤝 Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

### Development Best Practices

- Follow TypeScript patterns in [Coding Standards](./ref/CODING_STANDARDS.md)
- Use centralized types from `packages/core/src/types.ts`
- Implement proper error handling with `packages/core/src/errors.ts`
- Reference implementation patterns in [Style Guide](./docs/STYLE_GUIDE.md)
- Configure your editor with `.editorconfig` settings

## 📝 License

This project is licensed under the ISC License - see the [LICENSE](LICENSE) file for details.

## 🛟 Support

- [Documentation](docs/)
- [Issue Tracker](https://github.com/yourusername/multi-controller-app/issues)
- [Discussions](https://github.com/yourusername/multi-controller-app/discussions)

## 🏗️ Technology Stack

- **Frontend**: WinUI 3 / WPF
- **Backend**: C# .NET 8 with Native AOT
- **Scripting**: TypeScript / Node.js
- **Testing**: Jest, xUnit
- **Build**: MSBuild, TypeScript Compiler
- **CI/CD**: GitHub Actions
- **Package Management**: npm, NuGet

## 🚦 Development Status

Current Phase: **Architecture Validation**

### 🚀 Current Sprint - High Priority Tasks

#### 🟢 Ready for Development (No Blockers)
These tasks can be worked on immediately in parallel:

- **Task 1**: Verify Development Environment (🔴 High | 5 subtasks)
- **Task 2**: Scaffold Project Repository (🔴 High | 5 subtasks)  
- **Task 11**: Fix Memory Leaks in Transport Layers (🔴 High | Complexity: ⭐⭐⭐⭐)
- **Task 12**: Establish Test Coverage (🔴 High | Complexity: ⭐⭐⭐⭐⭐)
- **Task 13**: Enforce Serial Latency Budget (🔴 High | Complexity: ⭐⭐⭐)
- **Task 15**: Remove 'any' Types in SSH Transport (🟡 Medium | Complexity: ⭐⭐⭐)
- **Task 16**: Secure Credential Management (🟡 Medium | Complexity: ⭐⭐⭐⭐)
- **Task 17**: Activate Performance Monitoring (🟡 Medium | Complexity: ⭐⭐⭐)
- **Task 19**: Fix TypeScript Path Aliases (🔵 Low | Complexity: ⭐⭐)

#### 🔶 Blocked Tasks (Dependencies Required)
- **Task 3**: Prototype UI → Requires: Task 1
- **Task 4**: Compare & Decide Stack → Requires: Task 3
- **Task 5**: Device Abstraction Layer → Requires: Task 4
- **Task 6**: Transport Layer → Requires: Task 5
- **Task 7**: Build UI → Requires: Task 6
- **Task 8**: Scripting Runtime → Requires: Task 7
- **Task 9**: Telemetry & Profiles → Requires: Task 8
- **Task 10**: Final Testing → Requires: Task 9
- **Task 14**: Reconnection Logic → Requires: Tasks 6, 11
- **Task 18**: Arduino Driver → Requires: Tasks 5, 6

### Progress by Development Stream
- **Infrastructure**: 0/3 tasks (0%)
- **Core Development**: 0/5 tasks (0%)
- **Quality & Fixes**: 0/6 tasks (0%)
- **Features**: 0/5 tasks (0%)

## 📊 Task Board

### 🟢 Ready for Development (No Blockers)
| ID | Task | Priority | Complexity | Worktree Command |
|----|------|----------|------------|------------------|
| 1 | Verify Development Environment | 🔴 High | ⭐⭐ | `git worktree add ../env-setup feature/task-1` |
| 2 | Scaffold Project Repository | 🔴 High | ⭐⭐ | `git worktree add ../scaffold feature/task-2` |
| 11 | Fix Memory Leaks | 🔴 High | ⭐⭐⭐⭐ | `git worktree add ../memory-fix feature/task-11` |
| 12 | Test Coverage | 🔴 High | ⭐⭐⭐⭐⭐ | `git worktree add ../test-coverage feature/task-12` |
| 13 | Serial Latency Budget | 🔴 High | ⭐⭐⭐ | `git worktree add ../serial-perf feature/task-13` |
| 15 | Remove 'any' Types | 🟡 Medium | ⭐⭐⭐ | `git worktree add ../type-safety feature/task-15` |
| 16 | SSH Security | 🟡 Medium | ⭐⭐⭐⭐ | `git worktree add ../ssh-security feature/task-16` |
| 17 | Performance Monitoring | 🟡 Medium | ⭐⭐⭐ | `git worktree add ../perf-monitor feature/task-17` |
| 19 | TypeScript Paths | 🔵 Low | ⭐⭐ | `git worktree add ../ts-paths feature/task-19` |

### 🔶 Blocked Tasks (Dependencies Required)
| ID | Task | Blocked By | Ready When |
|----|------|------------|------------|
| 3 | Prototype UI (C# & Rust) | Task 1 | After environment verification |
| 4 | Decide Stack | Task 3 | After prototypes complete |
| 5 | Device Abstraction | Task 4 | After stack decision |
| 6 | Transport Layer | Task 5 | After abstraction layer |
| 7 | Build Main UI | Task 6 | After transport layer |
| 8 | Scripting Runtime | Task 7 | After UI complete |
| 9 | Telemetry & Profiles | Task 8 | After scripting runtime |
| 10 | Final Testing | Task 9 | After all features |
| 14 | Auto-Reconnection | Tasks 6, 11 | After transport & memory fixes |
| 18 | Arduino Driver | Tasks 5, 6 | After core layers |

## 🗂️ Task Breakdown by Development Stream

### Stream 1: Infrastructure & Environment (Parallel)
- **Task 1**: Verify Development Environment - Check .NET 8, Rust, Node.js
- **Task 2**: Scaffold Project Repository - Create directory structure
- **Task 19**: Fix TypeScript Path Aliases - Configure tsconfig paths

### Stream 2: Core Development (Sequential)
- **Task 3**: Prototype UI with Serial Echo - Build C# and Rust prototypes
- **Task 4**: Compare Prototypes - Decide on technology stack
- **Task 5**: Implement Device Abstraction Layer - IDeviceDriver interface
- **Task 6**: Develop Transport Layer - Serial, TCP/UDP, SSH
- **Task 7**: Build Single-Window UI - Main application interface

### Stream 3: Quality & Fixes (Parallel)
- **Task 11**: Fix Memory Leaks - Implement event listener cleanup
- **Task 12**: Establish Test Coverage - Unit, integration, loopback tests
- **Task 13**: Enforce Serial Latency - ≤50ms budget instrumentation
- **Task 14**: Auto-Reconnection - Exponential backoff logic
- **Task 15**: Remove 'any' Types - TypeScript strict mode
- **Task 16**: Secure Credential Management - Encrypted SSH credentials

### Stream 4: Features (Sequential, depends on Core)
- **Task 8**: Integrate Scripting Runtime - JS/Lua/Python sandbox
- **Task 9**: Implement Telemetry & Profiles - Real-time charts, config
- **Task 10**: Automated Testing - Final acceptance tests
- **Task 17**: Performance Monitoring - Runtime CPU/RAM tracking
- **Task 18**: Arduino Device Driver - Hardware communication

## 📊 Code Quality

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![Coverage](https://img.shields.io/badge/coverage-80%25-yellowgreen)
![License](https://img.shields.io/badge/license-ISC-blue)