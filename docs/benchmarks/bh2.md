# BH2 — Background stats aggregation

## Goal
Implement BH2: a background aggregation job that consumes append-only HITS and produces batched `stats` snapshots, reducing per-hit KV write pressure while ensuring correctness (idempotent, eventually consistent) and acceptable latency.

## Acceptance criteria
- `docs/benchmarks/bh2.md` documents design, failure modes, and acceptance tests.
- A minimal prototype exists that batches HITS and writes aggregated stats (idempotent).
- Unit and integration tests prove correctness and idempotency.
- Bench artifacts show BH2 reduces per-hit writes and maintains acceptable latencies in representative scenarios.

## Design notes
- Consumer reads HITS with an offset/epoch watermark to ensure at-least-once processing and idempotency.
- Batches are applied periodically (configurable window) to KV as a single write or small number of writes.
- Ensure retries are safe and that partial failures do not double-count.

## Implementation plan
1. Prototype: create a small crate or module that reads HITS (or simulated HITS), aggregates counts per code, and writes a `stats` snapshot (simulate KV or use a test shim).
2. Tests: unit tests for aggregation correctness; integration tests for end-to-end behavior and idempotency.
3. Bench: add `benchmarks/run_bh2.sh` (or extend `run_all.sh`) to run representative loads and compare per-hit KV pressure and latency.
4. Cost model: add a short calc comparing per-write KV cost vs batched writes.

## Notes & failure handling
- If server/resource saturation occurs during bench, capture `server-*.log`, system metrics, and abort further runs.
- If prototype complexity is larger than expected, fallback to a consumer simulator that proves correctness and cost benefit.

---

(Agent: start here — create branch, add prototype scaffold, and update `.agent/PLAN.md`.)