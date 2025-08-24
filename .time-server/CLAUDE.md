# Time MCP — CLAUDE.md
_Last updated: 2025-08-23 19:46:19 _

**Purpose.** Provide reliable time and timezone utilities to agents (current time, UNIX timestamps, conversions).

**Install (Claude Code).**
Example using a common time MCP server:
```
claude mcp add time-server -- npx -y @theobrigitte/mcp-time
# or:
claude mcp add time-server -- npx -y @sidharthrajaram/time-mcp
```
(Any time MCP with `get_datetime` and `get_current_unix_timestamp`‑style tools is fine.)

**Typical tools.**
- `get_datetime(timezone?: string)` → ISO string in TZ or UTC
- `get_current_unix_timestamp()` → current epoch seconds

**Usage patterns.**
- Confirm timezone before scheduling, log correlation, or time‑boxed tests.
- Treat wall‑clock as advisory; avoid encoding safety logic purely on time.

# Import into root `CLAUDE.md`

Add these lines under **## Imports** in your root file:

@./.desktop-commander/CLAUDE.md
@./.filescope/CLAUDE.md
@./.clear-thought/CLAUDE.md
@./.context7/CLAUDE.md
@./.perplexity-ask/CLAUDE.md
@./.memory/CLAUDE.md
@./.time-server/CLAUDE.md
