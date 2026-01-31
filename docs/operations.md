# `docs/operations.md`

## Purpose

This document defines how to operate the **rmax.to redirect fabric** safely and reliably using **rlinks**.

Links are global infrastructure.
Operations must be boring, predictable, and reversible.

---

## Environments

At minimum:

| Environment | Purpose              |
| ----------- | -------------------- |
| `dev`       | Local testing        |
| `prod`      | Live rmax.to traffic |

Each environment must have:

* Its own KV namespaces
* Its own Worker deployment
* Its own API tokens

Never share KV between environments.

---

## Deployment flow

### Worker

1. **Pre-flight Check**:
   - Run `cargo test --workspace` to ensure core logic and validation are sound.
   - Verify `wrangler.toml` points to the correct entry point and account ID.
   - Check that `rlinks-core` version matches `package.json` or `wrangler.toml` compatibility date if applicable.

2. **Staging / Preview**:
   - Run `wrangler dev` locally to sanity check the build.
   - Deploy to a staging environment (if configured) or use `wrangler deploy --dry-run` to verify config.
   - Verify a known "canary" route (e.g., `rmax.to/health` or a test code) resolves correctly.

3. **Production Deployment**:
   - Execute: `wrangler deploy`
   - **Verify**: Immediately run `rlinks get <code>` on a known active route and `curl -I https://rmax.to/<code>` to confirm availability.
   - **Observe**: Watch Cloudflare Worker "Real-time Logs" for 1-2 minutes to catch instant crash loops.

Worker deployments must never modify KV data implicitly.

### Rollback Procedures

If a deployment introduces a regression (e.g., 500 errors, broken redirects):

1. **Immediate Revert**:
   - Use Wrangler's rollback command: `wrangler rollback <version-id>` (or redeploy the previous known-good git commit).
   - This reverts the *code* but leaves *data* (KV) untouched.

2. **Schema Conflicts**:
   - If the bad deployment involved a schema change, ensure the old worker code can tolerate the new data shape (forward-only compatibility).
   - If data corruption occurred (rare, as workers are read-mostly), refer to "Backup & Recovery" to restore the `ROUTES` namespace.

3. **Verification**:
   - After rollback, verify critical paths:
     - `curl -I https://rmax.to/<code>` (Redirects work)
     - `rlinks get <code>` (CLI can still read KV)

---

## Monitoring & Observability

Reliability relies on visibility. Use the Cloudflare Dashboard and internal logs.

### Key Metrics (Cloudflare Dashboard)

Monitor these signals in **Workers & Pages > rlinks > Metrics**:

1. **Requests**: Baseline traffic volume. Sudden drops suggest DNS/routing issues.
2. **Error Rate (5xx)**: Should be < 0.1%. Spikes indicate logic errors or KV unavailability.
   - *Alert*: Configure Cloudflare Notification for "Worker Errors > 1%".
3. **CPU Time**: Should be < 10ms average. Spikes indicate inefficient logic or serialization issues.
4. **KV Read/Write Operations**: Ensure usage aligns with traffic. High writes suggest a bug in the stats/logging logic.

### Logs

1. **Hit Logs** (KV `HITS` namespace):
   - Source of truth for traffic analytics.
   - Check `hit:__admin__:*` for recent mutation trails during incidents.

2. **Live Tail**:
   - Use `wrangler tail` during active debugging to see console logs and exceptions in real-time.

---

### rlinks CLI

1. Build
2. Tag
3. Distribute binary
4. Update local environments

CLI versions must be compatible with Worker schema.

---

## Token rotation

When rotating Cloudflare API tokens:

1. Create new token in Cloudflare
2. Update Terraform secrets
3. Update CLI environments
4. Revoke old token

Never rotate by deleting first.

---

## Backup & recovery

KV is the live state.

Backups:

* Periodically export `ROUTES` namespace
* Periodically export `HITS` namespace (optional)

If KV is lost:

* Restore latest export
* Worker continues immediately

---

## Emergency redirect

If a link must be changed immediately:

```
rlinks set <code> <new-url>
```

This propagates globally in seconds.

Do not use Cloudflare dashboard.

---

## Disable a compromised link

```
rlinks disable <code>
```

This stops routing but preserves:

* History
* Hit logs
* Audit trail

---

## Restore a disabled link

```
rlinks set <code> <url>
```

This reactivates with new `updated_at`.

---

## Investigating issues

If a redirect is wrong:

1. `rlinks get <code>`
2. Inspect `target`, `status`, `updated_at`
3. `rlinks stats <code>`
4. Inspect hit logs if needed

Never guess.

---

## KV cleanup

Hit logs may grow large.

Policy:

* Keep raw logs for N days
* Aggregate older data
* Optionally delete raw logs

Redirect objects must never be deleted automatically.

---

## Operational invariants

* Redirects must always be recoverable
* No destructive writes without audit
* No manual KV edits
* No Terraform record management

---

## When things go wrong

Follow:

1. `docs/failure-modes.md`
2. Logs
3. CLI

Do not improvise.

---

> “Operations is where architecture proves itself.”

---

See also:
- [failure-modes](./failure-modes.md)
- [security](./security.md)
- [SPEC](./SPEC.md)
- [roadmap](./roadmap.md)

