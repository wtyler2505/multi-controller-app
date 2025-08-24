# Memory MCP — CLAUDE.md
_Last updated: 2025-08-23 19:46:19 _

**Purpose.** Long‑term, searchable memory for assistants (store / retrieve / semantic search). Prefer local‑first where possible.

**Popular options (pick one):**
- **OpenMemory MCP (mem0)** — local app integrating mem0; follow official install guide and app settings.
  - Docs: https://mem0.ai/blog/introducing-openmemory-mcp
- **Memory MCP Server (HPKV)** — long‑term memory for IDE assistants.
  - Repo: https://github.com/hpkv-io/memory-mcp-server
- **Memory Cache Server (tosin2013)** — token‑saving cache between turns.
  - Repo: https://github.com/tosin2013/mcp-memory-cache-server
- **MCP Memory (Puliczek)** — vector/RAG memory (Cloudflare Workers + D1/Vectorize).
  - Repo: https://github.com/Puliczek/mcp-memory

**Install (Claude Code) — generic patterns.**
Clone/build and register:
```
git clone <chosen-repo-url>
cd <repo>
npm i && npm run build  # if provided
# Then add to Claude Code:
claude mcp add memory -- node <ABS_PATH_TO>/dist/index.js
```
Smithery (when available):
```
npx -y @smithery/cli install <npm-scope-or-repo> --client claude
```

**Guardrails.**
- Never store secrets/PII. Persist only long‑lived, project‑relevant facts.
- Keep entries compact; prefer normalized fields (topic/source/date) to reduce retrieval noise.
- Periodically export to repo‑local `./docs/memory/` for auditability.

**Usage patterns.**
- On creating a new convention, store a one‑sentence memory with canonical path.
- When retrieving: search by topic + time window; include top‑k + confidence.
- Add a short provenance note (where did this memory come from?).

**Example prompts.**
- “Store a memory: our serial safety invariant and the location of the red‑button handler.”
- “Retrieve memories related to ‘RioRand’ since last month and summarize implications.”

# Import into root `CLAUDE.md`

Add these lines under **## Imports** in your root file:

@./.desktop-commander/CLAUDE.md
@./.filescope/CLAUDE.md
@./.clear-thought/CLAUDE.md
@./.context7/CLAUDE.md
@./.perplexity-ask/CLAUDE.md
@./.memory/CLAUDE.md
@./.time-server/CLAUDE.md
