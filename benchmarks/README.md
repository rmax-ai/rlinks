# Benchmarks: Worker hit-logging

This folder contains a small harness and orchestration for measuring Worker hit-logging designs.

How to run (local):

- Make sure you have Rust toolchain installed.
- Run: `./run_all.sh` (may take some time; adjust `REQUESTS` env var to reduce run-time).

Output:
- `benchmarks/results/`: Contains raw JSON results and `summary-agg.csv` and optionally `p95_vs_concurrency.png`.

Interpretation:
- Each JSON contains p50/p95/p99 latencies (ms), counts, errors, and rps.
- Use `summary-agg.csv` for aggregated per-mode/concurrency numbers.
