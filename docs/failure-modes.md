# `docs/failure-modes.md`

## Purpose

This document enumerates all known ways the rmax.to redirect fabric can fail, how each failure manifests, and how the system is designed to behave.

Redirect systems are global routing infrastructure.
Failures must be predictable.

---

## Failure taxonomy

Failures are grouped into five classes:

1. Data corruption
2. Control-plane errors
3. Runtime failures
4. Abuse and attack
5. Operational mistakes

---

## 1. Data corruption

### 1.1 Invalid redirect object in KV

**Cause**

* Manual KV edit
* Bug in CLI
* Partial write

**Symptom**

* Worker fails to parse JSON
* 500 errors for affected link

**Mitigation**

* Worker validates schema before use
* Invalid objects are treated as 404
* CLI enforces schema on all writes

**Recovery**

* Fetch KV record
* Re-write via CLI
* Audit hit logs for impact

---

### 1.2 Redirect loop

**Cause**

* Target points back to rmax.to
* Recursive chaining

**Symptom**

* Browser infinite redirects
* Cloudflare error

**Mitigation**

* CLI blocks loops
* Worker enforces `no_loop`

**Recovery**

* Disable offending redirect
* Rotate target

---

## 2. Control-plane failures

### 2.1 Lost Cloudflare API token

**Cause**

* Token revoked
* Token leaked

**Symptom**

* CLI cannot write
* New redirects impossible

**Mitigation**

* Token scoped only to KV
* Rotatable via Terraform

**Recovery**

* Rotate token
* Update CLI environment

---

### 2.2 Two writers

**Cause**

* Someone writes to KV outside rlinks

**Symptom**

* Drift
* Lost data
* Broken links

**Mitigation**

* Policy: rlinks is only writer
* KV credentials not shared

**Recovery**

* Compare KV vs CLI
* Reconcile manually
* Rotate secrets

---

## 3. Runtime failures

### 3.1 Cloudflare KV outage

**Cause**

* Cloudflare regional failure

**Symptom**

* 503 or slow redirects

**Mitigation**

* Cloudflare edge replication
* Graceful error responses

**Recovery**

* Wait for Cloudflare
* No data loss

---

### 3.2 Worker deployment bug

**Cause**

* Broken Worker release

**Symptom**

* All links fail
* Edge returns 500

**Mitigation**

* Canary deploy
* Version pinning

**Recovery**

* Roll back Worker
* KV untouched

---

## 4. Abuse & attacks

### 4.1 Traffic flood

**Cause**

* Botnet
* Scraper
* Hotlink

**Symptom**

* KV read amplification
* Log explosion

**Mitigation**

* Cloudflare rate limiting
* Per-IP throttling
* Optional caching

**Recovery**

* Block source
* Drop logs if needed

---

### 4.2 Phishing use

**Cause**

* Someone creates misleading redirect

**Symptom**

* Trust erosion
* External complaints

**Mitigation**

* Single-writer CLI
* Auditable metadata
* Manual review

**Recovery**

* Disable redirect
* Preserve logs

---

## 5. Operational mistakes

### 5.1 Accidental delete

**Cause**

* `rlinks delete`

**Symptom**

* Link returns 404

**Mitigation**

* Prefer `disable`
* Soft deletes

**Recovery**

* Recreate from logs or memory

---

### 5.2 Wrong target

**Cause**

* Human error

**Symptom**

* Link points to wrong site

**Mitigation**

* Validation
* `updated_at` tracking

**Recovery**

* Set correct target
* No data loss

---

## Golden rule

If something goes wrong:

1. Inspect the KV object
2. Inspect hit logs
3. Fix via CLI
4. Never panic

The system is designed to be **observable and reversible**.

---

> “Hope is not a strategy. Logs are.”

---

See also:
- [operations](./operations.md)
- [security](./security.md)
- [SPEC](./SPEC.md)

