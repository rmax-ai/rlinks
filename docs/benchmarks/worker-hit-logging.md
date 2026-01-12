# Worker hit-logging benchmark report

Summary:

- Short run (REQUESTS=100, repeats=3) results show **append** has slightly better latencies than **kv** in our PoC:
  - Concurrency=1: append p50=17.1ms p95=64.8ms; kv p50=24.1ms p95=64.5ms
  - Concurrency=100: append p50=250.8ms p95=268.0ms; kv p50=257.9ms p95=272.0ms
- Error rates were 0% across these runs.

Recommendation:

- Prefer the **append-only HITS** design: it shows equal-or-better latency in this PoC and is simpler and cheaper than per-hit KV writes. Implement a background aggregation job (BH2) to compute `stats` from HITS and keep KV writes batched.

Caveats & Next Steps:

- These runs are on local hardware with a deterministic KV latency shim; run full-scale experiments with `REQUESTS=5000` and on representative infrastructure (CI or dedicated machine) before finalizing.
- Add cost accounting for KV writes (per-write cost) and a production-grade server to model true Cloudflare Worker behavior.
