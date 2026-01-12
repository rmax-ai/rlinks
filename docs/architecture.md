# `docs/architecture.md`

## Purpose

This document describes the architecture of **rlinks** and the **rmax.to redirect fabric**.

It defines:

* System boundaries
* Data flow
* Ownership of state
* Separation of concerns

This architecture is designed to keep links **fast, safe, and eternal**.

---

## High-level system

```
                      ┌──────────────┐
                      │ Terraform    │
                      │ (infra only) │
                      └──────┬───────┘
                             │
                             ▼
                 ┌────────────────────┐
                 │ Cloudflare Platform │
                 │ - Workers           │
                 │ - KV                │
                 │ - DNS               │
                 └──────┬─────────────┘
                             │
                             ▼
                    ┌────────────────┐
                    │ rlinks CLI      │
                    │ (control plane) │
                    └────────────────┘
                             │
                             ▼
                      Cloudflare KV
                  (redirects + hit logs)
                             │
                             ▼
                   Cloudflare Worker
                   (edge redirector)
                             │
                             ▼
                         Internet
```

---

## Planes of responsibility

The system is split into three planes.

### 1. Infrastructure plane (Terraform)

Owns:

* Workers
* KV namespaces
* DNS
* Secrets
* Routes

Does NOT own:

* Redirect records
* Analytics
* State

Terraform provides the substrate.

---

### 2. Control plane (rlinks)

Owns:

* Redirect object lifecycle
* Validation
* Versioning
* Auditing

rlinks is the **only writer** to KV.

All redirect mutations must flow through:

```
rlinks-cli → rlinks-core → rlinks-kv → Cloudflare KV
```

---

### 3. Runtime plane (Worker)

Owns:

* Request handling
* Redirect resolution
* Hit logging

Worker behavior must be:

* Deterministic
* Stateless
* Fast

It never mutates redirect definitions.

---

## Data model

Two KV namespaces:

| Namespace | Purpose                                           |
| --------- | ------------------------------------------------- |
| ROUTES    | Redirect objects (`route:{code}`)                 |
| HITS      | Append-only hit logs (`hit:{code}:{date}:{uuid}`) |

---

## Request flow

```
Browser
  ↓
https://rmax.to/parity
  ↓
Cloudflare Worker
  ↓
KV.get("route:parity")
  ↓
Validate status & rules
  ↓
KV.put("hit:parity:20260112:uuid")
  ↓
HTTP 301 → https://paritybench.rmax.tech
```

Single lookup, single log, constant time.

---

## Write flow

```
rmax CLI
  ↓
Validate + diff
  ↓
KV.get(route)
  ↓
KV.put(route)
  ↓
Audit log
```

No direct writes to KV are allowed.

---

## Failure isolation

| Component | Failure impact                         |
| --------- | -------------------------------------- |
| Worker    | All redirects broken, but state intact |
| KV        | Temporary unavailability               |
| CLI       | No new changes, but links still work   |
| Terraform | Infra drift only                       |

No component can destroy redirect data accidentally.

---

## Why this architecture works

* KV is used as a **distributed routing table**
* Worker is used as a **pure resolver**
* CLI is used as a **transactional writer**
* Terraform is used as a **substrate**

This prevents:

* Drift
* Split-brain
* Accidental deletion
* Coupling between infra and content

---

> “Architecture is the art of how to waste space.” — Frank Lloyd Wright

---

See also:
- [SPEC](./SPEC.md)
- [schema](./schema.md)
- [operations](./operations.md)
- [DEVELOPMENT](./DEVELOPMENT.md)

