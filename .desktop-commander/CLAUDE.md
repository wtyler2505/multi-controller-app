# Desktop‑Commander MCP — CLAUDE.md
_Last updated: 2025-08-23 19:46:19 _

**Purpose.** Give Claude safe terminal control, file search, diff‑style edits, process management, and basic data analysis on your machine.

**Install (Claude Code, Windows).**
PowerShell (recommended, auto‑updates):
```
npx @wonderwhy-er/desktop-commander@latest setup
# To remove later:
npx @wonderwhy-er/desktop-commander@latest remove
```
Manual add to Claude Code:
```
claude mcp add desktop-commander -- npx -y @wonderwhy-er/desktop-commander
```

**Capabilities (high‑level).**
- Terminal: run long‑lived commands with streaming output; manage processes; timeouts.
- Filesystem: read/write, create/move, list, search, metadata; negative‑offset reads (tail‑like).
- Edits: surgical text replacements; diff‑oriented change plans.
- Settings: update server config at runtime.
- Telemetry: can be disabled via the “Disable telemetry” tool/command.

**Guardrails.**
- Default to non‑destructive actions; ask before delete/move/overwrite.
- For edits: show a unified diff and minimize unrelated formatting changes.
- Stay inside the repo unless the user explicitly provides a path.
- Never print secrets or tokens into logs or commit history.

**Usage patterns.**
- File change: read → propose unified diff → apply minimal patch.
- Terminal: show intent/command; capture stderr/stdout; stop on non‑zero exit (unless told otherwise).
- Windows shell variants: when relevant, provide both PowerShell and Git Bash commands.

**Quick prompts.**
- “List the repo tree and the five largest files.”
- “Tail the last 200 lines of `logs/app.log` and follow for 30s.”
- “Create `docs/decisions/decision-log.md` with today’s entry.”

# Import into root `CLAUDE.md`

Add these lines under **## Imports** in your root file:

@./.desktop-commander/CLAUDE.md
@./.filescope/CLAUDE.md
@./.clear-thought/CLAUDE.md
@./.context7/CLAUDE.md
@./.perplexity-ask/CLAUDE.md
@./.memory/CLAUDE.md
@./.time-server/CLAUDE.md
