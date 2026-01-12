# AGENTS.md — rlinks (agent guidance)

Purpose

- This file is the canonical, agent-oriented guide for contributors and automated assistants working on the rlinks repository. It collects the most important project rules, invariants, working workflows, and references so agents can operate quickly, safely, and with minimal back-and-forth.

Quick rules (start here)

1. Read before you act: always read: README.md, docs/GUIDELINES.md, docs/DEVELOPMENT.md, docs/SPEC.md, docs/schema.md, docs/operations.md, docs/security.md, docs/failure-modes.md and docs/architecture.md for context specific to your task.
2. Single-writer rule: only the CLI (rlinks) may mutate redirect records in Cloudflare KV. Direct KV edits or Terraform record management are forbidden and are security/operational incidents.
3. Schema-first: any change to redirect behavior starts with the schema (rlinks-core). Update types, validation, docs, tests, and migrations in that order.
4. Tests & linting: every change must include tests (unit / integration / property) appropriate to the change. Run cargo test, cargo clippy, cargo fmt and make sure CI would pass.
5. Non-destructive by default: prefer disable/soft-delete and reversible changes; do not perform destructive write or delete operations on production data without explicit human approval.
6. Never commit or push on behalf of the user unless they explicitly ask you to. When asked to commit follow the Git Safety Protocol (see the "Commit & PR guidance" section).
7. Evidence-based: show verification after any action (build/test output, lsp_diagnostics, ls or git status). Don’t assume—demonstrate.
8. Security: never print or commit secrets. Use .env.example and only run integration tests against dedicated dev KV namespaces.
9. Performance guardrails: keep the edge path O(1), single KV lookup, and sub-10ms. Don’t add heavy work on the hot path.
10. When running shell commands use a timeout wrapper for potentially long-running commands and the workdir parameter if available.

Agent roles & how to behave

- Oracle (answering): Provide concise, precise, evidence-backed technical answers. If uncertain, indicate what you know and what remains to check.

- Operator (commands): Use the repository tooling (cargo, git, gh) via safe, non-destructive commands. Always prefer discovery commands first (git status, git diff, cargo test). Wrap long-running commands with a timeout and use the provided workdir parameter.

- Surgeon (edits): Read files before editing. Make surgical edits; preserve formatting; run lsp_diagnostics and tests after edits. If a change needs more than 3 steps, create a todo list (todowrite) and manage progress.

Where to start (file checklist)

- Docs index: docs/INDEX.md — table of contents and quick cross-references.
- Core documents: `README.md`, `docs/GUIDELINES.md`, `docs/DEVELOPMENT.md`, `docs/SPEC.md`.
- Agent state: `.agent/PLAN.md` — current task list and progress.
- Workflow: `.agent/WORKFLOW.md` — rules for agent operation.
- Bootstrap: `.agent/prompts/bootstrap.prompt.md` — developer intent.

Key invariants (must be preserved)

- Redirects are typed, versioned objects (not strings).
- Cloudflare KV is runtime truth; rlinks CLI is the only writer.
- No redirect loops to rmax.to.
- Reserved paths (api, admin, www) cannot be claimed.
- HTTPS targets only (unless explicitly allowed via rules).
- Immutable creation metadata; forward-only schema versions.
- Observability: every hit and every mutation must be logged.
- Rollbacks: destructive changes must be reversible (disable, do not delete when possible).

Change checklists (concise)

- Small bug fix
  - Reproduce locally (add failing unit test).
  - Fix code in appropriate crate (rlinks-core, rlinks-kv, rlinks-cli).
  - Run cargo test, cargo clippy, cargo fmt.
  - Provide test output and lsp_diagnostics.

- New feature / API change
  - Schema-first: add or change field(s) in rlinks-core and docs/schema.md.
  - Add validation and tests that cover invariants.
  - Add migration tooling if required (schema forward-only rules apply).
  - Update CLI and Worker compatibility and add integration or e2e tests.

- Schema change
  - Bump or add schema version explicit field (v: N).
  - Make new fields optional for older records unless a migration exists.
  - Provide a migration tool and tests.
  - Verify Worker & CLI understand the new schema prior to release.

- Production fix / emergency
  - Prefer `rlinks set <code> <url>` or `rlinks disable <code>` through the CLI.
  - Do not use Wrangler, Terraform, or the Cloudflare dashboard to mutate route records.
  - After any production change, collect evidence: KV object, hit logs, audit hits, and 'rlinks get <code>'.

Testing & CI

- Mandatory: cargo test --workspace
- Linting: cargo clippy --workspace --all-targets -- -D warnings
- Formatting: cargo fmt --all
- Integration tests that hit Cloudflare are allowed only against dedicated dev KV namespaces and credentials. Never point tests at production KV.

Git, commits & PRs

- Never create commits unless explicitly requested by the user.
- When asked to commit:
  1. Run git status, git diff, git log -1 to verify the working tree and style.
  2. Stage only relevant files.
  3. Write a concise 1–2 line commit message focused on the why (type: fix/feat/docs/test): e.g. "fix(validation): block reserved words in redirect code".
  4. Run tests and linters. If pre-commit hooks change files, create a new commit (do not amend unless you created HEAD and it hasn't been pushed).
  5. Do not force-push to protected branches.
- PR checklist: tests pass, clippy passes, schema docs updated (if applicable), migration tooling included (if applicable), CHANGELOG/roadmap note (if feature), no secrets.
- Use gh CLI for PR operations and include a short summary + bullet points in the PR body.

Commands & tooling (examples)

- Run tests: timeout 5m cargo test --workspace
- Run clippy: timeout 2m cargo clippy --workspace --all-targets -- -D warnings
- Format: timeout 30s cargo fmt --all
- Run a crate tests: timeout 2m cargo test -p rlinks-core
- Inspect: git status; git diff --staged; rg / grep using project tooling

Safety & destructive operations

- Never run or suggest destructive shell commands like rm -rf, git reset --hard, or force pushes without explicit, repeated confirmation from a human owner.
- If you need to modify or delete TODO.md or other one-line lists, follow the project/Repo TODO policy and create a timestamped backup before editing.

Security & secrets

- Never print, log, or commit tokens or secrets.
- Use .env.example as the template. Integration runs requiring real secrets must be done in a dedicated dev environment with rotated credentials.
- If a secret is exposed, follow docs/security.md incident steps: rotate tokens via Terraform, disable affected redirects, preserve logs.

Incident & recovery steps (brief)

- If a redirect breaks: treat like a production outage. Follow docs/failure-modes.md.
- Immediately: inspect KV object, check schema & status, check hit logs, reproduce via CLI, disable if needed.
- For suspected compromise: rotate Cloudflare API token, preserve logs, reconcile KV.

Progressive disclosure (where to learn more)

- README.md — project overview and high-level architecture
- docs/GUIDELINES.md — system constitution and invariants
- docs/DEVELOPMENT.md — development and testing discipline
- docs/SPEC.md & docs/schema.md — authoritative behavior and schema
- docs/operations.md — deployment & runbook items
- docs/security.md — threat model and incident response
- docs/failure-modes.md — failure taxonomy and recovery
- docs/roadmap.md — planned evolution and guardrails

Owner & contact hints

- Primary identity fields are present in created_by; contact the person in created_by (for example max@rmax.ai) for policy/approval questions.

If in doubt

- Ask a concise clarifying question and do not take risky actions.
- Prefer small, well-tested, and reversible changes.

This file is the starting point; follow the linked docs for detailed procedures and the code for exact implementation details.
