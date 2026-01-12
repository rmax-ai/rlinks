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

1. Build Worker
2. Deploy via Terraform or Wrangler
3. Validate on a canary route
4. Promote to production

Worker deployments must never modify KV data.

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

