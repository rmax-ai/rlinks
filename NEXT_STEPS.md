# Next Steps

**Last Updated:** 2026-01-28

This file tracks the immediate next actions for the rlinks project. The orchestrator loop reads this file at the start of each iteration to determine what work to do next.

## High-Leverage Tasks (Pick One Per Iteration)

1. [x] **Complete SPEC.md consolidation** - Address tasks T1-T7 in `.agent/PLAN.md` (T1-T7 completed)
   - Define reserved codes explicitly
   - Document error response formats
   - Formalize schema migration paths
   - Specify CLI command details

2. [ ] **Add integration tests for CLI** - Ensure CLI commands work end-to-end
   - Test against dev KV namespace
   - Cover create/update/delete/get operations
   - Verify validation rules

3. [ ] **Document Worker deployment** - Complete operational runbook
   - Add deployment checklist to `docs/operations.md`
   - Document rollback procedures
   - Add monitoring setup guide

4. [ ] **Implement CLI analytics** - Add hit log querying capabilities
   - Design query interface
   - Implement pagination
   - Add export formats (JSON, CSV)

## Recently Completed

(Track completed work here)

## Blocked

(List any tasks waiting on external dependencies or decisions)

## Notes

- Keep this file focused on immediate next steps (1-5 items)
- Move completed items to git commit messages
- Update this file as part of every iteration
