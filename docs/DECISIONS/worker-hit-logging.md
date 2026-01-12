# Worker hit-logging decision

Context
- The Worker must be extremely fast on the hot path (sub-10ms edge latency budget).
- We need to balance correctness, cost (per-KV write), and operational complexity.

Options considered

1) Per-hit KV increments
- Worker performs an atomic increment on a `stats` key for each hit.
- Pros: stats are immediately available and strongly accurate.
- Cons: High cost at scale (per-hit KV writes), increased latency, possible rate limits.

2) Append-only HITS + lazy/batch stats
- Worker appends a small JSON line to an append-only `HITS` store (or a log-like KV) and returns immediately. A background job computes derived `stats` periodically.
- Pros: Lowest per-request latency and cost; append-only semantics are simple and robust; allows eventual consistency and reconciliations.
- Cons: Stats are eventually consistent and require a background pipeline or occasional recompute.

Recommendation

- Choose: **Append-only HITS + batch stats** (Option 2)
- Rationale: Keeps the edge fast and inexpensive; allows flexible batching, sampling, and reconciliation strategies; aligns with performance guardrails.

Follow-ups
- Add a benchmark plan to validate append latency and cost with realistic traffic patterns (N synthetic requests, measure p50/p95 latency).  
- Implement Worker to append hits (local PoC implemented in `crates/rlinks-worker` that writes `hits.log`).
- Implement a background recompute job (separate service) to generate `stats` from HITS and expose them via CLI/API.

Notes
- Add safeguards: cap per-request work, circuit-breakers, and sampling modes if immediate stats are required for high-volume codes.
