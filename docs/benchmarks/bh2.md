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

## Results
- Implemented a small CLI (`rlinks-bh2 aggregate --in <file> --out <file> --code <code>`) that reads bench-harness JSON and writes aggregated `stats` JSON. CLI has unit and integration tests.
- Smoke run (REQUESTS=1000) produced `benchmarks/results/bh2-agg.json` with count 1000 for `ok-code`.
- Representative experiments (REQUESTS=5000; concurrencies 1,10,50,100; repeats=3) were executed. Aggregated CSV: `benchmarks/results/summary-agg.csv`.
  - Observed aggregated p95 latencies for `bh2` series: 65.6ms (c=1), 31.3ms (c=10), 83.6ms (c=50), 168.3ms (c=100).
  - Error rates at low concurrency (1,10) were 0% for `bh2`; overall error counts are saved in `summary-agg.csv`.

## Next steps
1. Add stronger end-to-end tests that simulate partial failures and idempotency (reprocessing same HITS). ✅
2. Investigate a scheduled/background service to run aggregator continuously or as a cron job.
3. Add cost modeling for KV writes vs batched writes and include in the docs.

---

(Agent: start here — create branch, add prototype scaffold, and update `.agent/PLAN.md`).
