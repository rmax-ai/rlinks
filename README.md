# rlinks

**rlinks** is the authoritative control plane for **rmax.to**, a global, low-latency URL redirect fabric built on Cloudflare Workers and KV.

It provides durable, auditable, and sovereign redirects for canonical links (projects, research, writing, identity), with a strict separation between infrastructure, control, and runtime.

This is not a URL shortener.
It is infrastructure.

---

## What rlinks is

* A **typed, versioned redirect system**
* A **single-writer control plane** for Cloudflare KV
* A **CLI-first** operational tool
* A **global edge redirect runtime** (<10 ms)

rlinks exists to ensure that links under `rmax.to` never break, never drift silently, and can evolve safely over time.

---

## What rlinks is NOT

* Not a marketing link shortener (Bitly, TinyURL, etc.)
* Not a multi-tenant SaaS
* Not UI-driven
* Not Terraform-managed at the record level

If you are looking for campaigns, QR codes, or dashboards, this is the wrong tool.

---

## High-level architecture

```
            ┌─────────────┐
            │ rlinks CLI  │   (control plane)
            └──────┬──────┘
                   │
                   ▼
        Cloudflare KV (ROUTES, HITS)
                   │
                   ▼
      Cloudflare Worker (edge runtime)
                   │
                   ▼
                Internet
```

* **Terraform** manages infrastructure only (Worker, KV, DNS, secrets)
* **rlinks CLI** is the only writer of redirect records
* **Cloudflare Worker** performs read-only routing + hit logging

---

## Repository structure

```
rlinks/
  crates/
    rlinks-core/    # domain models, validation, schema
    rlinks-kv/      # Cloudflare KV integration
    rlinks-cli/     # CLI interface
  docs/
    schema.md
    architecture.md
    failure-modes.md
    security.md
    operations.md
    roadmap.md
  GUIDELINES.md
  DEVELOPMENT.md
  README.md
  .env.example
```

Each crate has a single responsibility. Cross-layer coupling is forbidden.

---

## Core concepts

### Redirects are objects, not strings

Each redirect is a structured, versioned record:

* Schema versioned
* Validated before write
* Auditable after write
* Evolvable without breaking links

This allows rlinks to grow into analytics, experiments, and policy without re-architecture.

---

### Source of truth

* **Cloudflare KV** is the runtime truth
* **rlinks CLI** is the only authority allowed to mutate it
* **Terraform** must never manage individual redirect records

This prevents drift and accidental deletion.

---

## Redirect lifecycle

1. Redirect is created via CLI
2. Record is validated and written to KV
3. Edge Worker resolves requests using a single KV lookup
4. Each hit is logged append-only
5. Redirects are disabled or rotated, never silently overwritten

---

## Example usage

Create or update a redirect:

```
rlinks set parity https://paritybench.rmax.tech
```

Disable a redirect (soft delete):

```
rlinks disable parity
```

Inspect a redirect:

```
rlinks get parity
```

List all redirects:

```
rlinks list
```

---

## Environment configuration

rlinks reads configuration from environment variables:

```
CLOUDFLARE_ACCOUNT_ID
CLOUDFLARE_API_TOKEN
KV_NAMESPACE_ID
RMAX_DOMAIN=rmax.to
```

Never commit real credentials.
Use `.env.example` as reference.

---

## Safety guarantees

rlinks enforces the following invariants:

* No redirect loops back to `rmax.to`
* Reserved paths cannot be claimed (`api`, `admin`, `www`)
* HTTPS targets only (unless explicitly overridden)
* Immutable creation metadata
* Forward-only schema versions

If a redirect violates an invariant, it is rejected.

---

## Observability

* Every redirect hit is logged (append-only)
* Every mutation is auditable
* No silent failures

If it is not logged, it did not happen.

---

## Development & contribution

Before contributing, read:

* `GUIDELINES.md` — system constitution
* `DEVELOPMENT.md` — engineering rules
* `docs/schema.md` — redirect object contract

Breaking a link is a production incident.

---

## Roadmap (high-level)

* MVP redirect + hit logging
* Aggregated analytics
* Canary routing
* Temporal redirects
* Signed / private links
* Geo-aware routing

See `docs/roadmap.md` for details.

---

## License

Internal infrastructure project.
License to be defined if externalized.

---

## Philosophy

> Links are part of identity.
> Identity infrastructure must be boring, reliable, and sovereign.

rlinks is designed to disappear into the background and never surprise you.

---

> “Good infrastructure is invisible until it fails — and unforgettable when it does.”
