# `docs/roadmap.md`

## Purpose

This roadmap defines the **planned evolution** of rlinks and the rmax.to redirect fabric.

Features are added only when they do not violate:

* Stability
* Auditability
* Sovereignty
* Determinism

---

## Phase 0 — Foundation (MVP)

**Goal:** A globally reliable redirect fabric.

Deliverables:

* Cloudflare Worker redirector
* Cloudflare KV namespaces (ROUTES, HITS)
* rlinks CLI
* Redirect schema v1
* Hit logging
* Validation & safety invariants

Success criteria:

* Links resolve globally in <10ms
* Redirects are created and updated only via CLI
* Every hit is logged
* No loops or hijacks possible

---

## Phase 1 — Observability

**Goal:** Understand traffic without breaking simplicity.

Features:

* Daily hit aggregates
* Per-link stats
* 404 tracking
* CLI `stats` command

No dashboards.
Data remains in KV.

---

## Phase 2 — Safety & control

**Goal:** Make mistakes reversible.

Features:

* Versioned redirect history
* Soft delete with restore
* Change diffs
* Audit trails

No destructive mutations.

---

## Phase 3 — Routing intelligence

**Goal:** Controlled flexibility.

Features:

* Time-based redirects (`expires_at`)
* Canary routing (percent-based)
* Temporary overrides
* Scheduled promotions

Still deterministic and auditable.

---

## Phase 4 — Trust & security

**Goal:** Make links cryptographically verifiable.

Features:

* Signed redirect objects
* Tamper detection
* CLI-based verification

This makes rmax.to suitable for private or sensitive links.

---

## Phase 5 — Distribution

**Goal:** Let others use the fabric safely.

Features:

* Role-based CLI tokens
* Scoped permissions
* Optional public read API

Only after security and auditability are mature.

---

## What will never be added

rlinks will never include:

* Marketing dashboards
* Ad tracking
* Cookie injection
* User profiling
* Multi-tenant SaaS features

Those destroy link sovereignty.

---

## Design guardrails

Every new feature must:

1. Preserve backward compatibility
2. Be schema-driven
3. Be observable
4. Be reversible

If not, it does not ship.

---

> “A good roadmap is not a list of features. It is a list of things you refuse to become.”

---

See also:
- [SPEC](./SPEC.md)
- [operations](./operations.md)
- [architecture](./architecture.md)
- [security](./security.md)

