# `docs/security.md`

## Purpose

This document defines how **rlinks** protects the integrity of the **rmax.to redirect fabric**.

Redirect systems are not neutral.
They are powerful **traffic-routing primitives** and therefore **high-value attack surfaces**.

---

## Threat model

An attacker may try to:

1. Hijack a redirect to send users to a malicious site
2. Poison analytics or logs
3. Perform phishing using trusted rmax.to links
4. Delete or corrupt redirect data
5. Perform denial-of-service via traffic or KV abuse

Security design must assume:

* Public internet
* Hostile inputs
* Zero trust outside the CLI

---

## Trust boundaries

```
Internet  →  Worker  →  Cloudflare KV  ←  rlinks CLI
```

Only **rlinks CLI** is trusted to write.

Everything else is untrusted.

---

## Secrets

The following secrets exist:

| Secret               | Purpose              |
| -------------------- | -------------------- |
| Cloudflare API Token | Allows KV read/write |
| Worker secret        | Allows hit logging   |

Rules:

* Tokens must be scoped to minimal permissions
* Tokens must never be embedded in code
* Tokens must be rotated periodically
* Tokens must not be shared

---

## Write protection

Only rlinks CLI may write to redirect records.

Enforcement:

* KV credentials are held only by CLI
* Worker cannot write to `ROUTES`
* Terraform cannot write to records
* Cloudflare dashboard access is restricted

Direct KV mutation is a **security incident**.

---

## Redirect integrity

All redirect objects must satisfy:

* No redirect loops
* No reserved word hijacking
* HTTPS-only targets by default
* Explicit status (`active` / `disabled`)
* Valid schema

The CLI enforces these invariants.

The Worker verifies them before serving.

---

## Phishing prevention

rmax.to links are trusted by recipients.

Mitigations:

* Single-writer model
* Human identity stored in `created_by`
* Metadata for review
* Soft-delete instead of erase

Suspicious redirects can be traced and disabled without destroying evidence.

---

## Abuse & traffic attacks

Mitigations:

* Cloudflare rate limiting
* IP-based throttling
* Country filtering if needed
* Optional caching

KV read amplification must be controlled.

---

## Auditability

All events are logged:

* Redirect hits
* Redirect mutations
* Admin operations

Logs are append-only.

If a redirect was abused, there must be evidence.

---

## Recovery

If compromise is suspected:

1. Rotate Cloudflare API tokens
2. Disable affected redirects
3. Preserve hit logs
4. Reconcile KV with CLI state

No automatic deletion.

---

## Security philosophy

The redirect fabric must be:

* Hard to corrupt
* Easy to inspect
* Impossible to silently subvert

Trust is not assumed.
It is enforced by design.

---

> “Security is not a feature. It is a property.”

---

See also:
- [operations](./operations.md)
- [schema](./schema.md)
- [failure-modes](./failure-modes.md)

