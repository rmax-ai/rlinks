# Worker hit-logging benchmark report

Summary:

- Full run (REQUESTS=5000, repeats=3) results show **append** generally has equal-or-better latency than **kv** at low-to-moderate concurrency; at very high concurrency (500) both modes observe elevated latencies and non-negligible error rates.
  - Concurrency=1: append p50=17.50ms p95=57.36ms; kv p50=23.39ms p95=62.41ms
  - Concurrency=100: append p50=35.71ms p95=122.29ms; kv p50=33.76ms p95=146.97ms
  - Concurrency=500: append p50=200.60ms p95=519.58ms (694 errors / 5000); kv p50=138.83ms p95=750.99ms (273 errors / 5000)
- Error rates at low concurrency (1 and 10) were 0%; errors rise substantially at concurrency=500, indicating server saturation or resource limits.

Recommendation:

- **Prefer the append-only HITS design.** It shows equal-or-better latency in representative runs and is simpler and cheaper than per-hit KV writes. Implement a background aggregation job (BH2) to compute `stats` from HITS and keep KV writes batched.

Caveats & Next Steps:

- These full runs were executed locally; we recorded environment metadata in `benchmarks/results/env.json`. Before finalizing, run on representative infrastructure or CI and add cost accounting for KV writes (per-write cost) and a production-grade server model for true Cloudflare behavior.
- Save representative raw results (e.g., `append-100-1.json`, `kv-100-1.json`) and `summary-agg.csv` in `benchmarks/results/` for audit and further analysis.
