# Product Requirements Document – Windows Multi‑Controller App

## Overview

This PRD defines the goals, scope, requirements, and milestones for building a lightweight Windows application that discovers, connects to and controls heterogeneous hardware devices (Arduino/ESP32/ESP8266/RioRand/Raspberry Pi). The app must be efficient (<2 % idle CPU and <150 MB base RAM), start in under 2 seconds, provide a single‑window UI for manual control, scripting and telemetry, and support plugin drivers for future devices.

## Goals

- **Universal controller**: Provide a portable `.exe` that can detect and connect to supported devices over serial, TCP/UDP and SSH and expose control endpoints.
- **Low resource usage**: Stay within strict performance budgets for startup time, CPU and memory usage. Avoid heavy frameworks (no Electron); prefer Native AOT when possible to minimise size and memory【864974145225502†L124-L142】.
- **Extensibility**: Use a plugin driver API (`IDeviceDriver`, `IDeviceSession`) so that new hardware can be integrated without modifying the core UI.
- **Safety**: Include a global “Stop All Outputs” button and implement rate‑limiting, clamping and hot‑plug resilience to prevent hardware damage.
- **User workflows**: Support manual controls, script execution (default runtime chosen later), telemetry graphs, profiles (save/load setups) and rolling logs.

## Non‑Goals

- **Cross‑platform GUIs**: The application targets Windows only; Linux/macOS support is out of scope for v1.
- **Enterprise fleet management**: Large‑scale provisioning and remote OTA updates are deferred for later phases.
- **Embedded firmware development**: The project does not include writing firmware for the target devices beyond stub drivers.

## Requirements

### Functional

1. **Discovery & Connection**
   - Enumerate available COM ports, TCP/UDP devices (via mDNS or manual entry) and SSH‑enabled Raspberry Pis.
   - Detect hot‑plug events and attempt automatic reconnection with exponential backoff.

2. **Control Modes**
   - **Manual**: Provide sliders, buttons and toggles to send control signals to devices.
   - **Scripted**: Support a default scripting language (JavaScript/Lua/Python) with sandboxed APIs (`devices.list()`, `dev.call(endpoint,args)` etc.).
   - **Telemetry**: Subscribe to and display real‑time signals (temperature, voltage, current) at ~30 FPS with minimal latency.

3. **Profiles**
   - Allow users to save and load named configurations (device list, transport settings, control mappings).

4. **Logging & Debugging**
   - Maintain rolling logs for device I/O and internal events.
   - Provide one‑click export of logs for troubleshooting.

5. **Extensibility**
   - Define a driver interface (`IDeviceDriver` and `IDeviceSession`) and store each driver in `/drivers/<name>` with a manifest describing supported transports and endpoints.

### Non‑Functional

- **Performance budgets**: Start up in under 2 seconds; idle CPU ≤ 2 %; base RAM ≤ 150 MB (≤ 220 MB with charts); serial write–ack latency ≤ 50 ms; network latency ≤ 100 ms.
- **Packaging**: Produce a single portable `.exe` that runs without requiring administrator privileges. Use .NET Native AOT (preferred) or Rust + Win32; compare prototypes to choose the better option based on memory and startup metrics【864974145225502†L124-L142】【155744810364120†L12-L16】.
- **Security**: No default remote execution; use SSH keys instead of passwords; deny access to `.env` and secrets; follow a least‑privilege permissions policy.
- **Stability**: Handle device disconnects gracefully, release COM handles, and recover without restarting the application.

## Success Metrics

- Meeting or beating all non‑functional performance budgets.
- Successfully controlling at least one real device (Arduino serial echo) and one stub driver through the UI.
- Passing automated unit, loopback and soak tests with ≤5 % RAM drift over an 8‑hour run.
- Achieving 100 % of high‑priority tasks in the backlog within the scheduled milestones.

## Milestones & Backlog

The backlog is organised by milestones, epics and tasks. Task IDs are for reference within TaskMaster.

### Milestone 1 – Project Setup & Prototyping

| ID    | Title                                                                                      | Owner | Status | Depends On   |
| ----- | ------------------------------------------------------------------------------------------ | ----- | ------ | ------------ |
| M1‑T1 | Verify environment (Node 18+, .NET 8 SDK, Rust toolchain)                                  | AI    | ToDo   | –            |
| M1‑T2 | Scaffold repository structure (`/app`, `/drivers`, `/transports`, `/docs`, `/tests`, etc.) | AI    | ToDo   | –            |
| M1‑T3 | Create `Claude.md`, `.mcp.json`, `.claude/settings.json`, and subagent definitions         | AI    | ToDo   | M1‑T2        |
| M1‑T4 | Build minimal C# Native AOT UI with serial port echo; measure RAM & startup                | AI    | ToDo   | M1‑T1        |
| M1‑T5 | Build minimal Rust UI with serial port echo; measure RAM & startup                         | AI    | ToDo   | M1‑T1        |
| M1‑T6 | Compare prototype results; decide language & UI framework                                  | AI    | ToDo   | M1‑T4, M1‑T5 |
| M1‑T7 | Record trade‑off decision in `docs/decisions/decision-log.md`                              | AI    | ToDo   | M1‑T6        |

### Milestone 2 – Core Infrastructure

| ID    | Title                                                                  | Owner | Status  | Depends On |
| ----- | ---------------------------------------------------------------------- | ----- | ------- | ---------- |
| M2‑T1 | Implement Device Abstraction Layer (`IDeviceDriver`, `IDeviceSession`) | AI    | Pending | M1‑T6      |
| M2‑T2 | Implement Serial and TCP transports with reconnect/backoff logic       | AI    | Pending | M2‑T1      |
| M2‑T3 | Create fake device driver for UI testing                               | AI    | Pending | M2‑T1      |
| M2‑T4 | Implement JSON/TOML profile loader and hot‑reload                      | AI    | Pending | M2‑T1      |

### Milestone 3 – UI & Scripting

| ID    | Title                                                                                                    | Owner | Status  | Depends On |
| ----- | -------------------------------------------------------------------------------------------------------- | ----- | ------- | ---------- |
| M3‑T1 | Design and implement single‑window UI (Devices sidebar, tabs for Manual/Scripts/Telemetry/Logs/Profiles) | AI    | Pending | M2‑T3      |
| M3‑T2 | Integrate fake device into UI; display connection state                                                  | AI    | Pending | M3‑T1      |
| M3‑T3 | Choose and embed default scripting runtime (JS/Lua/Python)                                               | AI    | Pending | M3‑T1      |
| M3‑T4 | Implement scripting API (`devices.list()`, `dev.call()`, etc.)                                           | AI    | Pending | M3‑T3      |
| M3‑T5 | Implement telemetry ring buffers and charts (30 FPS)                                                     | AI    | Pending | M3‑T1      |

### Milestone 4 – Drivers & Testing

| ID    | Title                                                                | Owner | Status  | Depends On |
| ----- | -------------------------------------------------------------------- | ----- | ------- | ---------- |
| M4‑T1 | Implement Arduino serial echo driver                                 | AI    | Pending | M2‑T2      |
| M4‑T2 | Implement ESP32 TCP echo driver                                      | AI    | Pending | M2‑T2      |
| M4‑T3 | Implement RioRand PWM stub driver                                    | AI    | Pending | M2‑T2      |
| M4‑T4 | Write unit tests, loopback tests and soak tests for transports & DAL | AI    | Pending | M2‑T2      |

### Milestone 5 – Documentation & Release

| ID    | Title                                                                             | Owner | Status  | Depends On   |
| ----- | --------------------------------------------------------------------------------- | ----- | ------- | ------------ |
| M5‑T1 | Complete architecture diagrams and sequence diagrams in `docs/architecture/`      | AI    | Pending | M2‑T1        |
| M5‑T2 | Write user documentation (quickstart, device setup, troubleshooting)              | AI    | Pending | M3‑T5, M4‑T4 |
| M5‑T3 | Finalise MCP docs (`docs/mcp/`), ops guide (`docs/ops/claude-code.md`) and README | AI    | Pending | M3‑T3        |
| M5‑T4 | Conduct final acceptance tests; verify budgets and safety                         | AI    | Pending | All          |

## Risks & Mitigations (Summary)

Key risks include picking the wrong language/framework (test both early), libraries that are not AOT‑compatible【89652827176451†L120-L128】, transport instability, PWM safety and MCP tool misconfiguration. Mitigation plans are detailed in the project plan and tracked via backlog tasks.
