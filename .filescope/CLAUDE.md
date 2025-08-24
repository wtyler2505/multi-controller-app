# FileScope MCP — CLAUDE.md
_Last updated: 2025-08-23 19:46:19 _

**Purpose.** Analyze the codebase to rank important files, map dependencies, and generate diagrams & summaries to guide safe edits.

**Install (Claude Code, Windows).**
```
git clone https://github.com/admica/FileScopeMCP
cd FileScopeMCP
# Windows build
build.bat
# Then add to Claude Code (adjust path):
claude mcp add FileScopeMCP -- node <ABS_PATH_TO>/mcp-server.js --base-dir="<YOUR_PROJECT_ROOT>"
```
(See repo README for WSL/mac variants.)

**Core tools (server‑reported).**
- File Tree Mgmt: `list_saved_trees`, `create_file_tree`, `select_file_tree`, `delete_file_tree`
- Analysis: `list_files`, `get_file_importance`, `find_important_files`, `read_file_content`, `recalculate_importance`
- Summaries: `get_file_summary`, `set_file_summary`
- Watching: `toggle_file_watching`, `get_file_watching_status`, `update_file_watching_config`
- Diagrams: `generate_diagram` → outputs Mermaid/HTML; styles: directory/dependency/hybrid

**Guardrails & patterns.**
- Use FileScope **read‑only** to build a mental model before edits.
- Query importance, dependents, and dependencies before changing shared modules.
- After refactors, re‑scan and regenerate diagrams; update the decision log.

**Starter flow.**
```
create_file_tree(filename:"project.json", baseDirectory:"<repo>")
find_important_files(limit:7, minImportance:5)
generate_diagram(style:"hybrid", maxDepth:2, outputFormat:"html", outputPath:"diagrams/important-files")
```
# Import into root `CLAUDE.md`

Add these lines under **## Imports** in your root file:

@./.desktop-commander/CLAUDE.md
@./.filescope/CLAUDE.md
@./.clear-thought/CLAUDE.md
@./.context7/CLAUDE.md
@./.perplexity-ask/CLAUDE.md
@./.memory/CLAUDE.md
@./.time-server/CLAUDE.md
