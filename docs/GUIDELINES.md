# `GUIDELINES.md`

## 1. What rlinks is

**rlinks** is the **authoritative control plane** for the `rmax.to` redirect fabric.

It is not a URL shortener.
It is not a marketing tool.
It is a **routing layer for identity and infrastructure**.

Every change to a redirect is a production change.

---

## 2. Source of truth

Cloudflare KV is the **runtime truth**.
rlinks is the **only writer** to that truth.

Terraform:

* Owns infrastructure
* Never owns redirect records

No other system may mutate redirect objects.

---

## 3. Redirects are typed objects

Redirects are not strings.
They are versioned, structured records.

Every redirect must:

* Have a schema version
* Be validated before write
* Be auditable after write

Raw KV writes are forbidden.

---

## 4. Write discipline

All writes must go through:

```
rlinks-cli → rlinks-core → rlinks-kv → Cloudflare KV
```

Direct KV mutation is a **production incident**.

---

## 5. Safety invariants

Every redirect must satisfy:

* No loops back to rmax.to
* No shadowing of reserved words (`admin`, `api`, `www`)
* HTTPS targets only (unless explicitly overridden)
* Immutable creation metadata
* Forward-only schema versions

Violations must be blocked by the CLI.

---

## 6. Schema evolution

* Schema versions are explicit (`v: 1`, `v: 2`, …)
* Older records must continue to load
* Migrations must be explicit and reversible
* No breaking changes in-place

---

## 7. Observability is mandatory

Every redirect hit must be logged.
Every mutation must be auditable.

“No logs” means “it didn’t happen”.

---

## 8. Rollbackability

Every destructive action must be reversible:

* Disable, don’t delete
* Version before overwrite
* Keep historical hit logs forever

---

## 9. Performance constraints

* Redirect lookup must stay O(1)
* Worker path must remain sub-10ms
* KV read must be single-key

No joins, no scans, no heavy logic on the hot path.

---

## 10. Security

The redirect fabric is an attack surface.

* Admin tokens are secrets
* Links must not be forgeable
* CLI must never leak credentials
* Worker must not expose admin endpoints

---

## 11. Philosophy

rlinks exists to ensure:

> **Links do not die.**

All design choices must serve:

* Stability
* Auditability
* Sovereignty

---

## Three operating rules

1. If a link breaks, treat it like prod is down.
2. If you cannot explain a redirect change, it must be reverted.
3. If you cannot observe it, you do not control it.

---

See also:
- [DEVELOPMENT](./DEVELOPMENT.md)
- [security](./security.md)
- [SPEC](./SPEC.md)

