# Perplexity‑Ask MCP — CLAUDE.md
_Last updated: 2025-08-23 19:46:19 _

**Purpose.** Real‑time, web‑wide research via Perplexity’s Sonar API inside MCP clients.

**Install (Claude Code).**
Docker:
```
claude mcp add perplexity-ask -- docker run -i --rm -e PERPLEXITY_API_KEY mcp/perplexity-ask
```
NPX:
```
claude mcp add perplexity-ask -- npx -y server-perplexity-ask
```
Environment:
```
setx PERPLEXITY_API_KEY "<YOUR_KEY>"   # PowerShell
export PERPLEXITY_API_KEY="<YOUR_KEY>" # Git Bash
```

**Tool.**
- `perplexity_ask(messages:[...])` — conversational search; supports parameter tuning in server config.

**Patterns.**
- Triangulate claims with Context7 before coding changes.
- Include 2–3 independent sources; summarize findings → decision → trade‑offs.
- Watch cost: summarize, then deep‑dive only where needed.

**Example prompt.**
- “Use Perplexity to compare Windows serial libraries for high‑rate telemetry and cite official docs + 2 community threads.”

# Import into root `CLAUDE.md`

Add these lines under **## Imports** in your root file:

@./.desktop-commander/CLAUDE.md
@./.filescope/CLAUDE.md
@./.clear-thought/CLAUDE.md
@./.context7/CLAUDE.md
@./.perplexity-ask/CLAUDE.md
@./.memory/CLAUDE.md
@./.time-server/CLAUDE.md
