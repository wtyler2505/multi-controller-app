# Repository Guidelines

This guide helps contributors (humans and AI agents) work consistently and efficiently in this project.

## Project Structure & Module Organization
- Source: `src/` (primary app code), `assets/` (static files).
- Tests: `tests/` for unit/integration tests; mirrors `src/` structure.
- Tooling: `.taskmaster/` (Task Master tasks/config), `.claude/` (Claude Code settings), `.env` (API keys), `.mcp.json` (MCP servers).
- Scripts: `scripts/` for maintenance and automation.

## Build, Test, and Development Commands
- Install deps: `npm ci` or `pip install -r requirements.txt` (use whichever the repo provides).
- Run locally: `npm run dev` or `python -m src`.
- Build: `npm run build` or `make build` if available.
- Tests: `npm test` or `pytest -q`.
- Task Master (agent workflow): `task-master list`, `task-master next`, `task-master show <id>`, `task-master set-status --id=<id> --status=done`.

## Coding Style & Naming Conventions
- Indentation: 2 spaces for JS/TS/JSON/YAML; 4 spaces for Python.
- Filenames: `kebab-case` for scripts/assets, `PascalCase` for components/classes, `snake_case` for Python modules.
- Lint/format (if configured): `npm run lint && npm run format` or `ruff format && ruff check`.
- Keep functions small; prefer pure functions and clear interfaces.

## Testing Guidelines
- Co-locate tests under `tests/` mirroring `src/` paths (e.g., `src/foo/bar.ts` → `tests/foo/bar.test.ts`).
- Aim for meaningful coverage of core logic and error paths.
- Run the full test suite locally before opening a PR.

## Commit & Pull Request Guidelines
- Commits: use conventional style where possible, e.g., `feat: add device polling (task 2.1)`.
- PRs: include description, linked issues/tasks, screenshots or logs when UI/CLI changes, and testing notes.
- Keep PRs focused and small; note breaking changes clearly.

## Agent-Specific Instructions
- Use Task Master to drive work: `task-master next` → implement → `task-master set-status`.
- Log implementation notes during work: `task-master update-subtask --id=<id> --prompt="notes"`.
- Avoid editing `.taskmaster/tasks/tasks.json` or `.taskmaster/config.json` directly; use CLI commands.

## Security & Configuration
- Store API keys in `.env` (never commit). Ensure `.gitignore` excludes it.
- For MCP/Claude, verify models and keys via `task-master models` and `.mcp.json`.
