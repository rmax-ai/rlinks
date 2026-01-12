# Benchmarks methodology — Worker hit-logging (BH1)

This document describes the experimental design, metrics, aggregation rules, and interpretation guidance for worker hit-logging benchmarks.

## Goal
Compare design alternatives for recording hits at the edge:
- `append` — append-only HITS (cheap, write-ahead log)
- `kv` — per-hit KV writes (stronger immediate stats but costly and slow at scale)

## Experimental design
- Independent variables:
  - `mode` (append, kv)
  - `concurrency` (1, 10, 50, 100, 500)
  - `REQUESTS` (default 5000) and `--kv-latency-ms` (simulate remote KV latency)
- Repeats: run each `(mode, concurrency)` 3 times to observe variance.
- Fixed factors: same request URL (`/ok-code`), same harness client, same server binary.

## Metrics
- Latency percentiles: **p50**, **p95**, **p99** (ms)
- Throughput: **rps** (requests/sec)
- Errors: count of failed requests
- Observability artifacts: raw per-run JSON (contains the above), aggregated CSV

## Aggregation rules
- For each `(mode, concurrency)` take the arithmetic mean of p50/p95/p99/rps across repeats and sum errors.
- Record both per-repeat rows and aggregated rows in `summary-agg.csv` for transparency.

## Statistical notes
- Latency distributions are skewed; report percentiles rather than means for tail behavior.
- Use repeats to check for outliers; when variance is high, run more repeats or instrument the server.

## Acceptance & abort rules
- If **errors > 1%** at low concurrency (1 or 10), abort the full run and collect server logs (`server-*.log`) and system metrics to investigate.
- If server experiences run-time failures (crashes, OOMs), preserve logs and abort.

## Reproducibility
- Capture environment metadata in `benchmarks/results/env.json` (uname, CPU count, rustc/cargo versions, notes on REQUESTS and any deviations).
- Commit summarized artifacts and representative raw JSONs. If many raw files are produced, archive them in `benchmarks/results/archives/` and include a README for reproducibility.
- For CI: run with smaller `REQUESTS` and annotate metadata as `CI-smoke`.

## Plotting & visual checks
- Plot `p95` (or p50/p95) vs concurrency on a log-x axis to visualize scaling and saturation.
- Look for divergence between `append` and `kv` at different concurrency levels and for signs of saturation (rapid growth in p95 or rising error rates).

## Interpreting results
- Prefer `append` when latency is equal-or-better and error rates are lower — it is simpler and cheaper operationally.
- Investigate high-concurrency anomalies: check CPU, concurrency limits, system I/O, and simulated KV latency effects.

## Cost considerations
- For any decision that relies on `kv`, include a per-write cost model and expected writes/sec at production traffic to estimate monthly costs.

## Notes & future work
- BH2: design and bench a background aggregation job that processes HITS and produces batched `stats` objects; validate correctness, latency tolerance, and cost.
- Consider adding experiments with variable request payloads and skewed code distributions.

---

For specific run scripts and how to execute them, see `benchmarks/run_all.sh` and `benchmarks/README.md`.
