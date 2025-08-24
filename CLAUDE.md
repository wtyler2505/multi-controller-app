# This file provides operational guidance to **Claude Code** when working in this repository.

## Project Reference Documentation

Comprehensive reference documentation is available in the `/ref` directory:

- **[Project Overview](./ref/PROJECT_OVERVIEW.md)** - Project summary, goals, and current status
- **[Architecture Reference](./ref/ARCHITECTURE.md)** - System architecture, components, and interfaces
- **[MCP Servers Guide](./ref/MCP_SERVERS.md)** - MCP server configurations and workflows
- **[Task Management](./ref/TASK_MANAGEMENT.md)** - Task structure, commands, and development workflow
- **[Development Setup](./ref/DEVELOPMENT_SETUP.md)** - Prerequisites, environment setup, and build instructions
- **[Agents Reference](./ref/AGENTS.md)** - Comprehensive guide to all 16 specialized Claude Code agents
- **[Memory MCP Guide](./ref/MEMORY.md)** - Advanced strategies for knowledge graph utilization

## Imports

**Import TaskMaster's workflow rules as‑is (authoritative).**\
@./.taskmaster/CLAUDE.md

**Import Desktop‑Commander's file/terminal rules.**\
@./.desktop-commander/CLAUDE.md

**Import FileScope code‑graphing rules.**\
@./.filescope/CLAUDE.md

**Import Clear‑Thought structured‑reasoning rules.**\
@./.clear-thought/CLAUDE.md

**Import Context7 official‑docs rules.**\
@./.context7/CLAUDE.md

**Import Perplexity‑Ask research rules.**\
@./.perplexity-ask/CLAUDE.md

**Import Memory server rules.**\
@./.memory/CLAUDE.md

**Import Time‑Server rules.**\
@./.time-server/CLAUDE.md

> Imports are authoritative for their domain. When domains overlap, prefer: TaskMaster ▶ Context7 ▶ FileScope ▶ Desktop‑Commander ▶ Perplexity‑Ask ▶ Memory ▶ Time‑Server.

## MCP Operating Rules (global)

- Prefer MCP tools over freeform guesses. When a tool exists, **use it** to fetch evidence before coding or deciding.
- Show a tiny plan before risky ops (schema changes, file deletes, credential handling) and run a dry‑run when possible.
- Keep edits minimal and localized. Do not reformat huge files unless explicitly asked.
- Never write secrets into logs or commit history. Scrub tokens/keys from copied snippets.
- For docs inserted into code, include **the smallest relevant snippet** next to the callsite; link the full source in the comment.
- Slash commands: use `/mcp` to inspect connected servers and prompts; use server prompts when available.

## Project Overview

Windows **Multi‑Controller App** — A lightweight Windows application for discovering, connecting, and controlling heterogeneous hardware devices (Arduino/ESP32/ESP8266/RioRand/Raspberry Pi) over Serial, TCP/UDP, and SSH.

## Performance Requirements (budgets)

- Startup time: < 2 s
- Idle CPU: ≤ 2%
- Base RAM: ≤ 150 MB (≤ 220 MB with charts)
- Serial write‑ack latency: ≤ 50 ms; network ≤ 100 ms

## Architecture Overview

- UI Layer (WPF/WinUI or Rust) with tabs: Devices, Manual Controls, Scripts, Telemetry, Logs, Profiles
- Device Manager orchestrates discovery/connection and lifecycles
- Driver Registry for plug‑in drivers implementing `IDeviceDriver`/`IDeviceSession`
- Transport Layer: async Serial/TCP/UDP/SSH with reconnect/backoff
- Scripting Engine: sandboxed runtime (JS/Lua/Python) for automation
- Telemetry: ring‑buffered streams with decimation
- Profile Manager: JSON/TOML with hot‑reload

### Driver Interface (canonical)

```csharp
interface IDeviceDriver {
    string Name { get; }
    string[] SupportedTransports { get; }
    Task<bool> ProbeAsync(ITransport transport);
    Task<IDeviceSession> OpenAsync(ITransport transport);
}

interface IDeviceSession {
    Task<object> InvokeAsync(string endpoint, object[] args);
    Task<IDisposable> SubscribeAsync(string stream, Action<byte[]> handler);
    Task CloseAsync();
}
```

## Safety Requirements

- **Global Stop**: Must immediately neutralize all control outputs.
- **Rate Limiting**: Enforce bounds and ramp rates on PWM/actuation.
- **Hot‑plug Recovery**: Gracefully handle disconnects without app restart.

## Technology Decision Status

Evaluating **C# Native AOT** vs **Rust**. Criteria: idle memory (<150 MB), startup (<2 s), single‑file binary, AOT‑safe libs.

## Testing Requirements

- Unit tests for core logic
- Loopback tests for transports
- Soak tests (8‑hour runs, ≤5% RAM drift)

## Repository Layout (source of truth)

```
/app/           # UI and core application code
/drivers/       # Device driver plugins with manifest.json
/transports/    # Serial/TCP/UDP/SSH implementations
/scripts/       # Sample control and telemetry scripts
/tests/         # All test code
/profiles/      # Example JSON/TOML configuration profiles
/docs/          # Architecture, decisions, risks documentation
```

## Operations & Logs

- MCP server log cache (local dev): `C:\Users\wtyle\AppData\Local\claude-cli-nodejs\Cache\C--Users-wtyle-multi-controller-app`
- Do **not** use the `task-master-ai` CLI without explicit approval. Parse PRDs via **TaskMaster MCP** (authoritative) and keep PRD files in `./.taskmaster/`.

---

# `.desktop-commander/CLAUDE.md`

**Purpose**: Safe, Windows‑friendly file I/O and terminal orchestration from inside Claude Code.

## Use When

- Reading/writing project files, scaffolding repos, running local commands, inspecting logs.

## Guardrails

- Default to **non‑destructive** actions. For delete/move/overwrite, first emit a short plan and request confirmation.
- Keep edits minimal; preserve line endings and encoding. For bulk edits, prefer patch‑style diffs.
- Never touch outside the repo unless the path is explicitly provided by the user.

## Patterns

- File ops: read → small diff → write. Include a unified diff in the message.
- Terminal ops: run checks (`git status`, tests) before builds; surface stderr/stdout separately.
- On Windows, when shells differ, emit both **PowerShell** and **Git Bash** variants.

## Examples

- List repo tree (non‑recursively) and show sizes.
- Create missing `/docs/decisions/decision-log.md` with header and today’s entry.
- Build `HelloSerialWpfAOT` with exact commands and capture timings.

---

# `.filescope/CLAUDE.md`

**Purpose**: Codebase graphing, imports mapping, and hotspot discovery.

## Use When

- You need a fast mental model of the repo, dependency flow, or to locate high‑impact files before edits.

## Guardrails

- Read‑only by default. Don’t move or rewrite files via FileScope; combine with Desktop‑Commander for changes.

## Patterns

- Start with a **repo map** and an **importance ranking** to target edits.
- Before module changes, fetch **import graphs** to avoid cyclical deps.
- After heavy edits, re‑scan to validate structure and update the decision log.

---

# `.clear-thought/CLAUDE.md`

**Purpose**: Structured reasoning for high‑impact decisions (failure trees, test plans, risk analysis).

## Use When

- Choosing stack, designing driver interfaces, defining safety invariants, or debugging elusive defects.

## Guardrails

- Summarize assumptions and alternatives in bullets. Cap intermediate notes to keep token use tight.

## Patterns

- Produce: Goal → Constraints → Options → Evidence → Decision → Next 3 checks.
- Emit a **cheapest‑disambiguator first** test if uncertainty remains.

---

# `.context7/CLAUDE.md`

**Purpose**: Pull **official documentation** and version‑accurate references.

## Use When

- Calling platform APIs, library functions, or CLIs where accuracy matters; prior to writing unfamiliar code.

## Guardrails

- Prefer primary sources (vendor docs, standards). Insert only the **relevant snippet** inline with a link.

## Patterns

- For each API used: cite version, copy the 3–8 lines that justify usage, and record the link in comments and `/docs/decisions/`.

---

# `.perplexity-ask/CLAUDE.md`

**Purpose**: Broad, community‑sourced research and competitive scans.

## Use When

- Exploring alternatives, gathering tips/tricks, or surveying community workflows.

## Guardrails

- Triangulate results with **Context7** (official docs) before implementation. Prefer multiple independent sources. Keep cost in check—summarize.

## Patterns

- Produce a short **Findings** list with source URLs, then a **Decision** with trade‑offs.

---

# `.memory/CLAUDE.md`

**Purpose**: Persist stable facts and project conventions for future sessions.

## Use When

- Storing long‑lived preferences, repo paths, chosen stack decisions, or performance budgets.

## Guardrails

- Avoid storing secrets/PII. Save only information likely to remain valid for months.

## Patterns

- When creating a new convention (naming, directory, template), store a single‑sentence memory plus the canonical path.

---

# `.time-server/CLAUDE.md`

**Purpose**: Confirm timezones and schedule‑related checks.

## Use When

- Coordinating schedules, timestamps in logs, or generating time‑boxed performance runs.

## Guardrails

- Treat time as advisory; don’t hinge safety logic solely on wall‑clock checks.

---

## Slash Command Cheatsheet (server‑agnostic)

- `/mcp` — list configured servers, tools, and prompts; check auth/state.
- `/mcp__<server>__<prompt> [args]` — execute a server prompt when available.

## Decision Log Hooks

For any material change (APIs, transports, safety rules), append a 3‑line entry to `docs/decisions/decision-log.md`: **Context → Change → Impact**.

- DO NOT TAKE SHORTCUTS FOR ANY REASON WHATSOEVER AND TAKING EASIER OR SIMPLER ROUTES JUST BECAUSE YOU ARE HAVING A LITTLE TROUBLE WITH THE ROUTE YOU ARE ON!
- My GitHub Username is wtyler2505 | https://github.com/wtyler2505