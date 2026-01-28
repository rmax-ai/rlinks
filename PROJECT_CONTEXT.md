# Project Context: rlinks

**Purpose:** This document provides essential context for anyone (human or agent) working on the rlinks project.

## What is rlinks?

rlinks is a **URL shortener and redirect service** running on Cloudflare Workers, designed for personal/small-team use with strong operational and security guarantees.

**Core Value Proposition:**
- 🔒 Secure: Single-writer discipline, immutable audit trail
- ⚡ Fast: Edge-optimized, sub-10ms redirects
- 🛠️ Operator-friendly: CLI-first, GitOps-compatible
- 📊 Observable: Every hit logged, full audit capability

## Architecture Principles

### 1. Single-Writer Authority
**Invariant:** Only the CLI (`rlinks`) may write or modify redirect records in Cloudflare KV.

**Why:** Prevents data corruption, enables strong consistency guarantees, simplifies audit trails.

**Implication:** Never use Wrangler, Terraform, or the Cloudflare dashboard to modify redirect data directly.

### 2. Schema-First Design
**Invariant:** All redirect data is typed, validated, and versioned.

**Why:** Enables safe evolution, prevents malformed data at the edge.

**Implication:** Changes to redirect behavior start with schema updates in `rlinks-core`, followed by CLI and Worker compatibility.

### 3. Forward-Only Versioning
**Invariant:** Schema versions only increase; old versions must be readable by new code.

**Why:** Allows rolling updates without downtime.

**Implication:** New fields must be optional or have migrations; deletions require explicit version bumps and compatibility windows.

### 4. Observability by Default
**Invariant:** Every redirect hit is logged; every mutation is audited.

**Why:** Enables post-incident analysis, usage analytics, and security forensics.

**Implication:** Hit logs are append-only in KV; the Worker never skips logging (even on errors).

### 5. Edge Performance First
**Invariant:** Redirects resolve in O(1) time with a single KV lookup, targeting <10ms p99.

**Why:** Core user experience depends on speed; complexity belongs in CLI/batch jobs.

**Implication:** No heavy computation, no network calls (except KV), no analytics aggregation in the hot path.

## Key Components

### rlinks-core (Rust Library)
- Defines `RedirectRecord` schema and validation rules
- Shared by CLI and Worker
- Source of truth for data structure

### rlinks-cli (Rust Binary)
- Command-line tool for redirect management
- Only component authorized to write to KV
- Validates all inputs before mutation

### rlinks-worker (Rust + Cloudflare Workers)
- Edge redirect service
- Read-only KV access (except hit logs)
- Handles HTTP requests at `rmax.to/*`

## Operational Model

**Normal Operations:**
1. Operator runs `rlinks create <code> <url>`
2. CLI validates and writes to KV
3. Worker picks up changes immediately (no deploy)

**Emergency Scenarios:**
- Use `rlinks disable <code>` to stop a redirect
- Never delete records; disable them (preserves audit trail)
- See `docs/failure-modes.md` for incident response

## Development Workflow

1. **Read first:** Check `AGENTS.md`, `docs/GUIDELINES.md`, `docs/DEVELOPMENT.md`
2. **Schema changes:** Update `rlinks-core`, then tests, then docs
3. **Test everything:** `cargo test --workspace`, `cargo clippy`, `cargo fmt`
4. **Document decisions:** Add to `docs/DECISIONS/` if architectural
5. **Never commit secrets:** Use `.env.example` and rotate if exposed

## Security Model

**Threat Model:** Documented in `docs/security.md`

**Key Threats:**
1. Unauthorized redirect modification → Mitigated by single-writer + API token rotation
2. Redirect hijacking → Mitigated by validation rules (no loops, HTTPS-only)
3. Hit log manipulation → Mitigated by append-only + KV ACLs
4. Token exposure → Mitigated by secret rotation + audit logs

**Incident Response:** Follow `docs/security.md` procedures immediately.

## Current State (Phase 2)

- ✅ Core implementation complete (PoC validated)
- 🔄 Finalizing specification (`docs/SPEC.md`)
- ⏳ Building integration test suite
- ⏳ Preparing for alpha deployment

See `PROGRESS.md` and `PHASE_LEDGER.md` for detailed status.

## Working with Autonomous Agents

**If you're an AI agent:**
- Read `AGENTS.md` for operational rules
- Check `NEXT_STEPS.md` before starting work
- Follow `.agent/WORKFLOW.md` for process discipline
- Use the todo list to track your work
- Never commit unless explicitly asked
- Always run tests after changes

**If you're configuring an agent:**
- Point it to this file first for context
- Ensure it reads linked docs before acting
- Verify it follows single-writer and schema-first rules
- Monitor for policy violations (direct KV edits, secret exposure)

## Quick Reference Links

- **Architecture:** `docs/architecture.md`
- **Schema:** `docs/schema.md`
- **Spec:** `docs/SPEC.md`
- **Security:** `docs/security.md`
- **Operations:** `docs/operations.md`
- **Development:** `docs/DEVELOPMENT.md`
- **Agent Rules:** `AGENTS.md`
- **Current Plan:** `.agent/PLAN.md`

---

**Last Updated:** 2026-01-28
**Maintainer:** See `created_by` metadata in redirect records (e.g., max@rmax.ai)
