# Plan: Completing `docs/SPEC.md`

This document tracks the tasks required to bring `docs/SPEC.md` to a "Final" state for implementation.

## Status: In Progress

| ID | Task | Status | Owner |
|---|---|---|---|
| T1 | Consolidate Reserved Codes | [ ] | - |
| T2 | Refine Worker Hit Logic | [ ] | - |
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
