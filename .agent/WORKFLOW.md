# Agent-Native Development Workflow

This document defines how autonomous agents (like GitHub Copilot) should operate within the **rlinks** repository to ensure safety, consistency, and progress.

## 1. Interaction Cycle

Agents must follow the **OODA Loop** (Observe, Orient, Decide, Act):

1.  **Observe**: Read workspace state, `AGENTS.md`, and relevant `docs/*.md`.
2.  **Orient**: Check `.agent/PLAN.md` for current progress and blockers.
3.  **Decide**: Formulate a surgical plan. Propose it to the user if complex.
4.  **Act**: Use appropriate tools (edits, terminal, tests).

## 2. Managing Progress

- Use `manage_todo_list` for immediate task tracking.
- Sync long-term state to `.agent/PLAN.md`.
- **Concurrency**: Before starting a task in `PLAN.md`, an agent MUST claim it by changing its status to `[>]` (In Progress) and adding its session identifier (e.g., `- [>] **T1**: (Claimed by Agent-XYZ)`).
- Never start a new task without marking the previous one as completed or blocked.

## 3. Implementation Guardrails

- **Schema-First**: Always check `docs/schema.md` before changing code.
- **Evidence-Based**: Every edit must be followed by `cargo test` (or relevant validation) and `lsp_diagnostics`.
- **Surgical Edits**: Prefer `replace_string_in_file` over overwriting entire files.
- **Git Safety**: Never commit unless explicitly asked. Follow the message convention in `AGENTS.md`.

## 4. Documentation Protocol

- When changing behavior, update `docs/SPEC.md` first.
- Keep `AGENTS.md` updated if project-wide rules change.
- New documents should be referenced in `docs/INDEX.md`.

## 5. Failure Handling

- If a tool fails, analyze the error. Do not blindly retry.
- If a test fails, treat it as a regression. Revert or fix before proceeding.
- If documentation is ambiguous, ask the user or add a TODO to `.agent/PLAN.md`.

## 6. Security & Privacy

- Never print secrets to the terminal.
- Use `.env.example` for environment configuration.
- Audit any changes to `docs/security.md` with high priority.

## 7. Concurrency & Multi-Agent Coordination

When multiple agents are active:
- **Shared Files**: When editing global files like `docs/SPEC.md` or `PLAN.md`, use surgical edits (`replace_string_in_file`) to minimize merge conflicts.
- **Task Locking**: Only one agent may work on a specific task ID at a time.
- **Atomic Commits**: If asked to commit, include only the changes related to the claimed task.
- **Conflict Resolution**: If an agent detects that `PLAN.md` has changed since its last read, it must re-read the file before attempting to write.

```markdown
# Status Key
- [ ] Not Started
- [>] In Progress (Claimed)
- [x] Completed
- [!] Blocked / Needs Human Review
```
