# Multi-Controller App - Project Overview

## Project Summary

The **Windows Multi-Controller App** is a lightweight Windows application designed to discover, connect to, and control heterogeneous hardware devices including Arduino, ESP32, ESP8266, RioRand, and Raspberry Pi. The application provides a unified interface for device control via Serial, TCP/UDP, and SSH protocols.

## Key Characteristics

- **Platform**: Windows 11 (Windows-only, single platform focus)
- **Performance Target**: < 2s startup, ≤ 2% idle CPU, ≤ 150 MB base RAM
- **Deployment**: Single portable .exe file (no admin required)
- **Architecture**: Plugin-based driver system with extensible interfaces
- **Safety**: Global stop mechanism, rate limiting, hot-plug recovery

## Project Goals

1. **Universal Controller**: Portable executable for multi-device control
2. **Low Resource Usage**: Strict performance budgets, Native AOT preferred
3. **Extensibility**: Plugin driver API for new hardware integration
4. **Safety**: Hardware protection via rate limiting and emergency stop
5. **User Workflows**: Manual controls, scripting, telemetry, profiles, logging

## Technology Stack (Under Evaluation)

### Option 1: C# with .NET 8 Native AOT

- **UI Framework**: WinUI 3
- **Serial**: System.IO.Ports
- **Network**: System.Net.Sockets
- **SSH**: SSH.NET library
- **Scripting**: ClearScript or Jint for JavaScript

### Option 2: Rust

- **UI Framework**: egui or native Win32
- **Serial**: serialport-rs
- **Network**: tokio for async TCP/UDP
- **SSH**: thrussh library
- **Scripting**: rlua for Lua or rustpython

## Current Project Status

- **Phase**: Project Setup & Prototyping (Milestone 1)
- **Tasks Completed**: 0/10 main tasks, 0/50 subtasks
- **Next Steps**: Environment verification and prototype development
- **Decision Pending**: C# vs Rust stack selection based on benchmarks

## Development Workflow

The project uses **Task Master AI** for task management with:

- Structured task breakdown in `.taskmaster/tasks/tasks.json`
- AI-assisted task expansion and complexity analysis
- MCP server integration for Claude Code workflow
- Comprehensive documentation in CLAUDE.md files

## Key Performance Requirements

| Metric          | Budget      | Measurement                 |
| --------------- | ----------- | --------------------------- |
| Startup Time    | < 2 seconds | Cold start to UI ready      |
| Idle CPU        | ≤ 2%        | Windows Performance Monitor |
| Base RAM        | ≤ 150 MB    | Without charts/telemetry    |
| RAM with Charts | ≤ 220 MB    | With active telemetry       |
| Serial Latency  | ≤ 50 ms     | Write-to-ACK round trip     |
| Network Latency | ≤ 100 ms    | TCP/UDP round trip          |
| Soak Test Drift | ≤ 5%        | RAM growth over 8 hours     |

## Repository Structure

```
multi-controller-app/
├── /app/           # Core application and UI code
├── /drivers/       # Device driver plugins
├── /transports/    # Serial/TCP/UDP/SSH implementations
├── /scripts/       # Sample automation scripts
├── /tests/         # Unit, loopback, and soak tests
├── /profiles/      # Configuration profiles
├── /docs/          # Architecture and decisions
├── /ref/           # Project reference documentation
└── /.taskmaster/   # Task management system
```

## Related Documentation

- [Architecture Details](ARCHITECTURE.md)
- [MCP Servers Guide](MCP_SERVERS.md)
- [Task Management](TASK_MANAGEMENT.md)
- [Development Setup](DEVELOPMENT_SETUP.md)
