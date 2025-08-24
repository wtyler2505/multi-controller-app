# Clear‑Thought MCP — CLAUDE.md
_Last updated: 2025-08-23 19:46:19 _

**Purpose.** Structured reasoning tools: mental models, debugging approaches, decision frameworks, collaborative reasoning, and scientific‑method scaffolds.

**Install (Claude Code).**
```
claude mcp add clear-thought -- npx -y @chirag127/clear-thought-mcp-server
```
(Alternative: install via Smithery, or clone/build per README.)

**Tool families.**
- Mental Models (first principles, Pareto, Occam, etc.).
- Design Patterns & Programming Paradigms (OOP/FP/reactive/concurrent).
- Debugging Approaches (binary search, divide‑and‑conquer, program slicing).
- Sequential Thinking & Decision Frameworks (criteria weighting, risk handling).
- Metacognition (confidence calibration, bias detection).
- Visual/structured argumentation.

**Usage patterns.**
- Produce short artifacts: *Goal → Constraints → Options → Evidence → Decision → Next 3 checks*.
- Prefer “cheapest disambiguator first” tests when uncertain.
- Keep intermediate notes compact to control token cost.

**Example invocations.**
- “Generate a failure tree for intermittent serial disconnects and the 3 cheapest probes.”
- “Compare Rust vs C# Native AOT for our target budgets using the decision framework template.”

# Import into root `CLAUDE.md`

Add these lines under **## Imports** in your root file:

@./.desktop-commander/CLAUDE.md
@./.filescope/CLAUDE.md
@./.clear-thought/CLAUDE.md
@./.context7/CLAUDE.md
@./.perplexity-ask/CLAUDE.md
@./.memory/CLAUDE.md
@./.time-server/CLAUDE.md
