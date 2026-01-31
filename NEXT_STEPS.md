# Next Steps

**Last Updated:** 2026-01-31

This file tracks the immediate next actions for the rlinks project. The orchestrator loop reads this file at the start of each iteration to determine what work to do next.

## High-Leverage Tasks (Pick One Per Iteration)

1. [ ] **Update CLI to use rlinks-kv** - Wire up the CLI to the real KV store
   - Add `rlinks-kv` dependency to `rlinks-cli`
   - Implement `KvStore` integration in CLI commands
   - Add flags for KV credentials (or env var support)

2. [ ] **Implement CLI analytics** - Add hit log querying capabilities
   - Design query interface
   - Implement pagination
   - Add export formats (JSON, CSV)

3. [ ] **Worker Implementation** - Begin implementation of `rlinks-worker`
   - Scaffolding with `wrangler`
   - Implement `rlinks-core` integration
   - Connect to KV

## Recently Completed

- Created `rlinks-kv` crate with Cloudflare KV integration.
- Documented Worker deployment, rollback, and monitoring procedures in `docs/operations.md`.
- Consolidated `SPEC.md`.

## Blockers

(List any tasks waiting on external dependencies or decisions)

## Notes

- Keep this file focused on immediate next steps (1-5 items)
- Move completed items to git commit messages
- Update this file as part of every iteration
