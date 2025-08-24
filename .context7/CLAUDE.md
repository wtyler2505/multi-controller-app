# Context7 MCP — CLAUDE.md
_Last updated: 2025-08-23 19:46:19 _

**Purpose.** Pull **version‑accurate, up‑to‑date** official docs and code examples into context.

**Install (Claude Code).**
Remote (HTTP):
```
claude mcp add --transport http context7 https://mcp.context7.com/mcp --header "CONTEXT7_API_KEY: <YOUR_API_KEY>"
```
SSE:
```
claude mcp add --transport sse context7 https://mcp.context7.com/sse --header "CONTEXT7_API_KEY: <YOUR_API_KEY>"
```
Local:
```
claude mcp add context7 -- npx -y @upstash/context7-mcp --api-key <YOUR_API_KEY>
```

**Patterns.**
- In prompts, add: `use context7` to force doc retrieval.
- If you know the library id, use the slash form (e.g., `/supabase/supabase`) to skip matching.
- Store the cited link next to the code that uses it; keep inserted snippets small (3–8 lines).

**Guardrails.**
- Prefer primary sources (vendor docs, standards).
- Record version in decision log when locking an API.

**Examples.**
- “use context7 — fetch `System.IO.Ports.SerialPort` docs for .NET 8 and show minimal example for event‑driven reads.”
- “use context7 — latest `tokio-serial` usage with async streams on Rust 1.81.”

# Import into root `CLAUDE.md`

Add these lines under **## Imports** in your root file:

@./.desktop-commander/CLAUDE.md
@./.filescope/CLAUDE.md
@./.clear-thought/CLAUDE.md
@./.context7/CLAUDE.md
@./.perplexity-ask/CLAUDE.md
@./.memory/CLAUDE.md
@./.time-server/CLAUDE.md
