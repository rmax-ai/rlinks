# Worker hit-logging benchmark report

Summary:

- p50/p95 numbers and recommendation will be added after running `benchmarks/run_all.sh`.

Verdict:

- (placeholder) Preliminary expectation: `append` should have lower p50/p95 and lower cost compared to per-hit `kv` writes for high QPS.

Next steps:
- Run full benchmark on representative hardware (or CI runner with higher capacity)
- Consider background aggregation service (BH2) if append design is chosen
