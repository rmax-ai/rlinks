# Benchmarks — Worker hit-logging

This folder documents the benchmark harness and how to run, analyze, and interpret worker hit-logging experiments (BH1).

## 🔑 Purpose
- Provide a reproducible procedure for comparing `append` (append-only HITS) vs `kv` (per-hit KV writes) hit-logging modes.
- Record and preserve representative artifacts for audit and decision-making: environment metadata, raw JSON runs, aggregated summaries, plots, and a short verdict.

## Quick start
1. Run all experiments locally (careful on shared CI):

   REQUESTS=5000 bash benchmarks/run_all.sh

   - Overrides: set `REQUESTS` to a lower value for constrained machines.
   - Server logs are saved to `benchmarks/results/server-*.log`.

2. Produce summaries & plots:

   python3 benchmarks/analysis.py benchmarks/results

   - Generates `summary-agg.csv` and, if matplotlib is available, `p95_vs_concurrency.png`.

## Artifacts (what to save)
- `benchmarks/results/env.json` — environment metadata (uname, CPU count, rustc/cargo versions, notes).
- `benchmarks/results/*.json` — raw run outputs (one per mode/concurrency/repeat).
- `benchmarks/results/summary-agg.csv` — aggregated p50/p95/p99, rps, errors.
- `benchmarks/results/p95_vs_concurrency.png` — p95 vs concurrency plot.
- `benchmarks/results/server-*.log` — server start/runtime logs.

> Commit policy: save summarized artifacts (aggregated CSV, representative JSONs, and plot). Avoid committing all raw outputs unless requested; use `benchmarks/results/archives/` for large collections.

## Acceptance criteria
- JSON result files and `summary-agg.csv` exist for the planned run.
- p50/p95/p99, rps, and total errors are reported in `summary-agg.csv`.
- Low-concurrency error rates (concurrency 1 or 10) must be < 1%.
- A short verdict is added to `docs/benchmarks/worker-hit-logging.md`.

## Troubleshooting & failure handling
- If server fails to start: collect `benchmarks/results/server-*.log` and abort.
- If >1% error at low concurrency (1 or 10): capture `dmesg`/syslog, server logs, and abort runs to diagnose resource saturation.

## Next steps & extensions
- BH2: implement background stats recompute job and validate correctness & cost.
- Add cost accounting (per-KV write cost) to decision reports.

---

See `docs/benchmarks/methodology.md` for experimental design, metrics, and interpretation guidance.