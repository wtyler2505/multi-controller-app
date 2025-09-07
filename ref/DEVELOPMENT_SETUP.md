# Development Setup Reference

**Last Updated**: 2025-08-25  
**Setup Time**: ~30 minutes  
**Platform**: Windows 11  
**Technology Stack**: Rust + egui (decided 2025-08-25)

## Prerequisites

### Required SDKs and Tools

#### Rust Toolchain (PRIMARY - REQUIRED)

- **Version**: Stable channel 1.75+
- **Target**: x86_64-pc-windows-msvc
- **Install**: 
  ```powershell
  # Option 1: Download from https://rustup.rs/
  # Option 2: Use winget
  winget install Rustlang.Rustup
  ```
- **Configure**:
  ```bash
  rustup default stable
  rustup target add x86_64-pc-windows-msvc
  rustup component add rust-analyzer clippy rustfmt
  ```
- **Verify**: 
  ```bash
  rustc --version   # Should show 1.75+
  cargo --version   # Should show 1.75+
  ```

#### .NET 8 SDK (OPTIONAL - for C# prototype only)

- **Version**: 8.0 or later
- **Features**: Native AOT support
- **Download**: https://dotnet.microsoft.com/download/dotnet/8.0
- **Verify**: `dotnet --version`
- **Note**: Only needed for benchmarking against C# prototype

#### Node.js

- **Version**: 18.0 or later
- **Purpose**: MCP servers, build tools
- **Download**: https://nodejs.org/
- **Verify**: `node --version`

### Development Tools

#### Claude Code

- **Purpose**: Primary development environment
- **MCP Support**: Built-in MCP client
- **Install**: Via npm or direct download

#### Git

- **Version**: 2.0 or later
- **Purpose**: Version control
- **Verify**: `git --version`

#### Windows SDK

- **Version**: Windows 11 SDK
- **Purpose**: Native Windows APIs
- **Install**: Via Visual Studio Installer

## Verification-First Development Requirements

### Core Principles

Before starting any development work:

1. **Verification Before Claims**: Never claim implementation without proof
2. **Task Management**: Always check TaskMaster for next task before starting
3. **File Management**: Never create files unless explicitly requested
4. **Performance Validation**: Always validate against performance budgets
5. **Code References**: Always use file:line format for references

### Performance Validation Tools

Required for monitoring performance budgets:

- **Windows Performance Monitor**: For CPU and memory profiling
- **.NET Diagnostics**: `dotnet-counters`, `dotnet-trace`
- **Benchmark.NET**: For micro-benchmarks
- **Application Insights**: For telemetry (optional)

### Self-Verification Commands

Essential commands for verification:

```bash
# Verify changes were made
grep -n "search_term" file.md

# Count lines to verify additions
wc -l file.md

# Check task status
mcp__taskmaster-ai__next_task
mcp__taskmaster-ai__get_task --id=X.Y

# Validate performance
dotnet build -c Release /p:PublishAot=true
# Measure startup time and memory
```

## Environment Configuration

### API Keys Required

Create `.env` file in project root:

```env
# Required for Task Master AI operations
ANTHROPIC_API_KEY=your_key_here
PERPLEXITY_API_KEY=your_key_here

# Optional for alternative models
OPENAI_API_KEY=your_key_here
GOOGLE_API_KEY=your_key_here
MISTRAL_API_KEY=your_key_here
```

### MCP Server Setup

Configure `.mcp.json`:

```json
{
  "mcpServers": {
    "task-master-ai": {
      "command": "npx",
      "args": ["-y", "--package=task-master-ai", "task-master-ai"],
      "env": {
        "ANTHROPIC_API_KEY": "${ANTHROPIC_API_KEY}",
        "PERPLEXITY_API_KEY": "${PERPLEXITY_API_KEY}"
      }
    }
  }
}
```

### Claude Code Configuration

`.claude/settings.json`:

```json
{
  "allowedTools": [
    "Edit",
    "Read",
    "Write",
    "Bash(task-master *)",
    "Bash(npm *)",
    "Bash(dotnet *)",
    "Bash(cargo *)",
    "mcp__*"
  ]
}
```

## Project Initialization

### Step 1: Clone or Create Repository

```bash
# Create new project
mkdir multi-controller-app
cd multi-controller-app
git init

# Or clone existing
git clone <repository-url>
cd multi-controller-app
```

### Step 2: Initialize Task Master

```bash
# Initialize Task Master
npx task-master-ai init

# Parse PRD if available
npx task-master-ai parse-prd .taskmaster/docs/prd.txt

# Configure AI models
npx task-master-ai models --setup
```

### Step 3: Setup MCP Servers

```bash
# Install MCP servers via Claude Code
claude mcp add task-master-ai -- npx -y task-master-ai
claude mcp add desktop-commander -- npx -y @wonderwhy-er/desktop-commander
claude mcp add clear-thought -- npx -y @chirag127/clear-thought-mcp-server
```

### Step 4: Install Git Hooks

```bash
# Install git automation hooks (REQUIRED)
npm run git:install-hooks        # Windows
npm run git:install-hooks-bash   # Linux/Mac

# This installs:
# - pre-commit: Secrets scanning, performance validation
# - commit-msg: Conventional commit enforcement
# - pre-push: Final validation before push

# Verify installation
ls .git/hooks/
# Should show: pre-commit, commit-msg, pre-push (without .sample)
```

### Step 5: Create Project Structure

```powershell
# PowerShell script for Windows
New-Item -ItemType Directory -Path @(
    "app",
    "drivers",
    "transports",
    "scripts",
    "tests",
    "profiles",
    "docs",
    "ref"
) -Force
```

## Development Workflows

### Rust Setup (PRIMARY WORKFLOW)

#### Project Creation

```bash
# Create WinUI 3 project
dotnet new winui3 -n MultiControllerApp -o app

# Add Native AOT configuration
dotnet add package Microsoft.DotNet.ILCompiler
```

#### Project File Configuration

```xml
<PropertyGroup>
    <OutputType>WinExe</OutputType>
    <TargetFramework>net8.0-windows10.0.19041.0</TargetFramework>
    <PublishAot>true</PublishAot>
    <InvariantGlobalization>false</InvariantGlobalization>
    <SelfContained>true</SelfContained>
</PropertyGroup>
```

#### Build Commands

```bash
# Debug build
dotnet build

# Release with AOT
dotnet publish -c Release -r win-x64
```

#### Project Creation

```bash
# Create Rust project
cargo new --bin multi-controller-app
cd multi-controller-app

# Add dependencies
cargo add tokio --features full
cargo add serialport
cargo add eframe  # or egui
```

#### Cargo.toml Configuration

```toml
[package]
name = "multi-controller-app"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
serialport = "4"
eframe = "0.24"

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
strip = true
```

#### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run application
cargo run --release

# Check without building
cargo check

# Format code
cargo fmt

# Lint with clippy
cargo clippy -- -D warnings
```

### C# Native AOT Setup (LEGACY - for comparison only)

## Testing Setup

### Unit Testing

#### C# Testing

```bash
# Add test project
dotnet new xunit -n MultiControllerApp.Tests
dotnet add reference ../app/MultiControllerApp.csproj

# Run tests
dotnet test
```

#### Rust Testing

```bash
# Run tests
cargo test

# Run with coverage
cargo tarpaulin
```

### Performance Testing

#### Memory Profiling

```powershell
# Windows Performance Monitor
perfmon /res

# .NET diagnostics
dotnet-counters monitor -n MultiControllerApp
```

#### Startup Time Measurement

```powershell
# PowerShell timing
Measure-Command { .\MultiControllerApp.exe }
```

## Debugging Configuration

### VS Code / Cursor

`.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": ".NET Core Launch",
      "type": "coreclr",
      "request": "launch",
      "preLaunchTask": "build",
      "program": "${workspaceFolder}/app/bin/Debug/net8.0-windows/MultiControllerApp.dll"
    },
    {
      "name": "Rust Debug",
      "type": "lldb",
      "request": "launch",
      "cargo": {
        "args": ["build", "--bin=multi-controller-app"],
        "filter": {
          "name": "multi-controller-app",
          "kind": "bin"
        }
      }
    }
  ]
}
```

## Common Issues and Solutions

### Issue: Native AOT Compatibility

**Problem**: Library not compatible with AOT
**Solution**: Use AOT-compatible alternatives or source generators

### Issue: High Memory Usage

**Problem**: Exceeding 150MB budget
**Solution**:

- Enable trimming in publish settings
- Use object pooling
- Reduce buffer sizes

### Issue: Slow Startup

**Problem**: Startup exceeds 2 seconds
**Solution**:

- Defer resource loading
- Use lazy initialization
- Profile with dotnet-trace

### Issue: MCP Connection Failed

**Problem**: MCP servers not connecting
**Solution**:

- Check API keys in environment
- Verify Node.js installation
- Run with `--mcp-debug` flag

### Issue: PowerShell Script Failures

**Problem**: PowerShell scripts fail with "Unexpected token" or encoding errors
**Solution**:

1. **Check for Unicode/Emojis**:
   ```bash
   # Validate PowerShell script
   npm run validate:ps1 scripts/myscript.ps1
   ```

2. **Common character replacements**:
   - ✅ → [OK]
   - ❌ → [ERROR]
   - ⚠️ → [WARNING]
   - ℹ️ → [INFO]
   - ╔══╗ → +==+

3. **Fix automatically**:
   ```powershell
   # Replace emojis in file
   (Get-Content script.ps1) -replace '✅','[OK]' -replace '❌','[ERROR]' | Set-Content script-fixed.ps1
   ```

4. **Prevention**:
   - Always use ASCII characters in PowerShell scripts
   - Test scripts with `powershell -File script.ps1` before committing
   - Use the validation tool before any PowerShell script commit

## Build Automation

### GitHub Actions Workflow

`.github/workflows/build.yml`:

```yaml
name: Build and Test

on: [push, pull_request]

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup .NET
        uses: actions/setup-dotnet@v3
        with:
          dotnet-version: '8.0.x'

      - name: Setup Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Build C# Prototype
        run: dotnet publish -c Release -r win-x64

      - name: Build Rust Prototype
        run: cargo build --release

      - name: Run Tests
        run: |
          dotnet test
          cargo test
```

## Performance Monitoring

### Key Metrics to Track

- **Startup Time**: < 2 seconds
- **Idle CPU**: ≤ 2%
- **Base RAM**: ≤ 150 MB
- **Serial Latency**: ≤ 50 ms
- **Network Latency**: ≤ 100 ms

### Monitoring Tools

```bash
# Process monitoring
Get-Process MultiControllerApp | Select-Object CPU, WS

# Network monitoring
netstat -an | findstr :8080

# Serial port monitoring
mode COM3
```
