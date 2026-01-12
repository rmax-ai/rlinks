# `docs/schema.md`

## Purpose

This document defines the **Redirect Object Schema** for the rlinks system.

This schema is:

* Versioned
* Forward-compatible
* Enforced by the CLI
* Relied upon by the Cloudflare Worker

No redirect may exist outside this schema.

---

## Schema versioning

Every redirect object contains:

```json
"v": 1
```

Schema rules:

* Versions are monotonic (`1 → 2 → 3`)
* Older versions must remain readable
* New fields must be optional for old versions
* Breaking changes require explicit migration tooling

---

## Storage model

Redirects are stored in Cloudflare KV as:

```
Key:   route:{code}
Value: JSON object (schema below)
```

Example:

```
route:parity
```

---

## Core object

### `Route` (v1)

```json
{
  "v": 1,
  "code": "parity",
  "target": "https://paritybench.rmax.tech",
  "status": "active",
  "created_at": "2026-01-12T12:41:00Z",
  "updated_at": "2026-01-12T12:41:00Z",
  "created_by": "max@rmax.ai",
  "meta": {
    "notes": "canonical parity benchmark",
    "tags": ["research", "benchmark"]
  },
  "rules": {
    "https_only": true,
    "no_loop": true,
    "expires_at": null
  },
  "stats": {
    "hits": 12442,
    "last_hit": "2026-01-12T13:21:00Z"
  }
}
```

---

## Field definitions

### `v`

Schema version.

Type: integer
Required.

---

### `code`

The short path segment.

Example:

```
parity
```

Type: string
Constraints:

* Lowercase
* `[a-z0-9-]+`
* No reserved words (`api`, `admin`, `www`)
* Max length: 64

Immutable after creation.

---

### `target`

Destination URL.

Type: string (URL)

Constraints:

* Must be absolute
* Must be HTTPS if `https_only = true`
* Must not resolve to `RMAX_DOMAIN` if `no_loop = true`

---

### `status`

Routing state.

Type: enum

Values:

* `active`
* `disabled`

Disabled redirects return `404`.

---

### `created_at`

ISO 8601 timestamp when the redirect was created.

Immutable.

---

### `updated_at`

ISO 8601 timestamp when the redirect was last modified.

Updated on every write.

---

### `created_by`

Human or system identifier that created the redirect.

Immutable.

---

## `meta`

Free-form metadata.

```json
"meta": {
  "notes": "...",
  "tags": ["..."]
}
```

Not used in routing.
Used for:

* documentation
* searching
* analytics
* future UIs

---

## `rules`

Routing policy.

```json
"rules": {
  "https_only": true,
  "no_loop": true,
  "expires_at": null
}
```

### `https_only`

If true, target must use HTTPS.

Default: true

---

### `no_loop`

If true, target must not point to `RMAX_DOMAIN`.

Prevents infinite redirects.

Default: true

---

### `expires_at`

Optional expiration time.

Type: ISO 8601 or null

Behavior:

* If current time > expires_at → return `410 Gone`

---

## `stats`

Derived data, not source-of-truth.

```json
"stats": {
  "hits": 12442,
  "last_hit": "2026-01-12T13:21:00Z"
}
```

May be:

* Updated lazily
* Recomputed
* Rebuilt from logs

Must never affect routing decisions.

---

## Hit log schema

Stored separately in KV.

Key:

```
hit:{code}:{YYYYMMDD}:{uuid}
```

Value:

```json
{
  "ts": "2026-01-12T13:01:22Z",
  "code": "parity",
  "country": "NL",
  "ua_hash": "e4c1...",
  "referrer": "https://twitter.com/...",
  "ip_prefix": "203.0.113.0/24"
}
```

Hit logs are:

* Append-only
* Never mutated
* Used for analytics and audits

---

## Backwards compatibility

The Worker must:

* Accept any `v` ≤ current
* Ignore unknown fields
* Only depend on:

  * `code`
  * `target`
  * `status`
  * `rules`

This guarantees old links keep working forever.

---

## Invariants

All redirect objects must satisfy:

1. No redirect loops
2. HTTPS target if required
3. Non-expired
4. Active status
5. Valid schema

If any invariant fails, the redirect must not be served.

---

> “A schema is a promise to the future.”

---

See also:
- [SPEC](./SPEC.md)
- [DEVELOPMENT](./DEVELOPMENT.md)
- [GUIDELINES](./GUIDELINES.md)

