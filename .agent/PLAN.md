# Plan: Completing `docs/SPEC.md`

This document tracks the tasks required to bring `docs/SPEC.md` to a "Final" state for implementation.

## Status: In Progress

**Recent progress:** PoC implemented: `crates/rlinks-core`, `crates/rlinks-cli`, and `crates/rlinks-worker` exist and unit tests pass across the workspace. T2 has been decided: **append-only HITS + batch stats** (see `docs/DECISIONS/worker-hit-logging.md`). Next: add benchmark and background stats tasks.

| ID | Task | Status | Owner |
|---|---|---|---|
| T1 | Consolidate Reserved Codes | [ ] | - |
| T2 | Refine Worker Hit Logic | [Decided] | - |
| T3 | Define Error Responses | [ ] | - |
| T4 | Formalize Schema Migration Path | [ ] | - |
| T5 | CLI Command Specification | [ ] | - |
| T6 | Integration Points | [ ] | - |
| T7 | Final Consistency Pass | [ ] | - |

## Task Details

- **T1: Consolidate Reserved Codes**
  - Identify all reserved words (api, admin, www, etc.)
  - Document them in `docs/SPEC.md` Section 3 (Namespaces) or a new Section 12.
  - Cross-reference with `docs/schema.md`.

- **T2: Refine Worker Hit Logic**
  - `SPEC.md` currently suggests Worker increments `stats` on the `Route` object.
  - **Conflict:** KV writes are slow and potentially expensive in Workers for every hit.
  - Update Section 5 (Edge request flow) accordingly.

- **T3: Define Error Responses**
  - Section 11 (Failure handling) lists status codes but no bodies.
  - Define if they return HTML or JSON (for API use).
  - Document standard error templates.

- **T4: Formalize Schema Migration Path**
  - Section 8 mentions CLI computes new object.
  - Add details on how CLI handles objects with `v < CURRENT_VERSION`.
  - Reference `docs/schema.md` versioning rules.

- **T5: CLI Command Specification**
  - Flesh out Section 7 with exact flags and output formats (JSON/Text).
  - Define `rmax links search` or list filters.

- **T6: Integration Points**
  - Define how `HITS` are consumed (Analytics).
  - Document the `__admin__` hit log behavior.

- **T7: Final Consistency Pass**
  - Ensure `SPEC.md` aligns perfectly with `schema.md`, `DEVELOPMENT.md`, and `operations.md`.
  - Remove "Three actionable next steps" once implementation begins.

## Benchmarks & follow-ups

- **BH1: Worker hit-logging benchmark** [Completed]
  - Create a benchmarking harness that compares append-only HITS (append to log) vs per-hit KV writes under synthetic loads. (Added `benchmarks/harness`, `run_all.sh`, analysis and docs.)
  - Commands: a script that issues N concurrent requests, measures p50/p95 latency, and captures error rates/cost estimates.  (Next: run full-scale benchmark, add cost accounting, and finalize decision in `docs/benchmarks/worker-hit-logging.md`)
  - **Follow-up:** Full run executed (REQUESTS=5000, repeats=3) locally; artifacts saved to `benchmarks/results/` (`summary-agg.csv`, `p95_vs_concurrency.png`, representative JSONs). Next action: BH2 (background stats job) implementation and detailed cost accounting.

- **BH2: Background stats recompute job** (new)
  - Implement a separate service to consume HITS and produce `stats` snapshots. Include acceptance tests for idempotency and correctness.
  - Consider reconciliation paths (rebuild from HITS) and a retention policy for HITS.
