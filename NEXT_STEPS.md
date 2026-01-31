# Next Steps

**Last Updated:** 2026-01-31

This file tracks the immediate next actions for the rlinks project. The orchestrator loop reads this file at the start of each iteration to determine what work to do next.

## High-Leverage Tasks (Pick One Per Iteration)

1. [x] **Add integration tests for CLI** - Ensure CLI commands work end-to-end (Basic validation tests added)
   - [x] Create integration test framework
   - [x] Test validation rules (HTTP, reserved codes)
   - [ ] Test real CRUD operations (requires mocking or dev KV)

2. [x] **Complete SPEC.md consolidation** - Address tasks T1-T7 in `.agent/PLAN.md` (T1-T7 completed)
   - Define reserved codes explicitly
   - Document error response formats
   - Formalize schema migration paths
   - Specify CLI command details

3. [x] **Document Worker deployment** - Complete operational runbook
   - [x] Add deployment checklist to `docs/operations.md`
   - [x] Document rollback procedures
   - [x] Add monitoring setup guide

4. [ ] **Implement CLI analytics** - Add hit log querying capabilities
   - Design query interface
   - Implement pagination
   - Add export formats (JSON, CSV)

5. [ ] **Worker Implementation** - Begin implementation of `rlinks-worker`
   - Scaffolding with `wrangler`
   - Implement `rlinks-core` integration
   - Connect to KV

## Recently Completed

- Documented Worker deployment, rollback, and monitoring procedures in `docs/operations.md`.
- Consolidated `SPEC.md`.

## Blockers

(List any tasks waiting on external dependencies or decisions)

## Notes

- Keep this file focused on immediate next steps (1-5 items)
- Move completed items to git commit messages
- Update this file as part of every iteration
