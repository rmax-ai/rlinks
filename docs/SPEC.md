# rmax.to — Redirect Fabric v1

## 1. System boundaries

rmax.to is not a URL shortener.
It is a **canonical routing layer** for Max’s digital identity.

It must satisfy:

* Permanent link stability
* Global <10 ms redirects
* No vendor lock-in
* Deterministic behavior
* Forensic-grade logs

---

## 2. Architecture

```
            ┌─────────────┐
            │ rmax CLI    │
            │ (Control)   │
            └──────┬──────┘
                   │
                   ▼
            Cloudflare KV
         (source of runtime truth)
                   │
                   ▼
     Cloudflare Worker (Edge Runtime)
                   │
                   ▼
               Internet
```

Terraform owns only:

* Worker
* KV namespaces
* DNS
* Secrets

Mappings are owned by the CLI.

---

## 3. Namespaces

Two KV namespaces:

### `ROUTES`

Holds redirect objects.

Keys:

```
route:{code}
```

### `HITS`

Append-only hit logs.

Keys:

```
hit:{code}:{YYYYMMDD}:{uuid}
```

---

## 4. Redirect object schema

Stored at `route:{code}`

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
    "hits": 0,
    "last_hit": null
  }
}
```

---

## 5. Edge request flow

For request:

```
GET https://rmax.to/{code}
```

Worker executes:

1. Extract `{code}`
2. `obj = KV.get("route:" + code)`
3. If not found or `status != active` → 404
4. If `expires_at < now` → 410
5. If `no_loop` and target points to rmax.to → 500
6. Log hit:

   ```
   hit:{code}:{YYYYMMDD}:{uuid}
   ```
7. Increment `stats.hits` and `stats.last_hit`
8. Return `301 Location: obj.target`

---

## 6. Hit log object

Stored at `hit:{code}:{date}:{uuid}`

```json
{
  "ts": "2026-01-12T13:01:22Z",
  "code": "parity",
  "country": "NL",
  "ua_hash": "e4c1…",
  "referrer": "https://twitter.com/…",
  "ip_prefix": "203.0.113.0/24"
}
```

Used for:

* analytics
* abuse detection
* historical audits

---

## 7. Internal CLI

Binary: `rmax`

### Commands

```
rmax links list
rmax links get parity
rmax links set parity https://paritybench.rmax.tech
rmax links disable parity
rmax links delete parity
rmax links stats parity
```

---

## 8. CLI write pipeline

When running:

```
rmax links set parity https://x
```

The CLI:

1. Fetches existing `route:parity`
2. Validates:

   * URL
   * HTTPS
   * No loops
   * Reserved words
3. Computes new object
4. Sets `updated_at`
5. Writes back to KV
6. Emits an audit hit:

   ```
   hit:__admin__:YYYYMMDD:uuid
   ```

All mutations are serialized by the CLI.

---

## 9. Security

* CLI uses Cloudflare API token with:

  * KV read/write
  * Worker secrets read
* Worker only has KV read/write for stats
* No public admin endpoints

---

## 10. Terraform scope

Terraform manages only:

* Cloudflare Worker
* KV namespaces
* rmax.to domain
* Routes
* Secrets (API tokens)

Never individual redirect records.

---

## 11. Failure handling

| Case           | Behavior |
| -------------- | -------- |
| KV unavailable | 503      |
| Missing code   | 404      |
| Disabled       | 404      |
| Expired        | 410      |
| Loop detected  | 500      |

---

## Reserved Codes

Certain path segments are reserved and may not be claimed as redirect codes. This prevents accidental collisions with infrastructure and admin functionality.

Reserved codes (canonical list):

- `api` — reserved for programmatic endpoints and tooling
- `admin` — reserved for administrative interfaces
- `www` — reserved to avoid conflicts with subdomain-style redirects

Rationale:

- These values appear in the codebase and in validation rules enforced by the CLI and core library; documenting them here makes the invariant explicit for operators and auditors.
- Blocking these values prevents redirect loops, accidental exposure of control endpoints, and namespace pollution.

If additional reserved codes are required in the future, they MUST be added to this list, the schema validation rules, and corresponding unit tests.


## Three actionable next steps

1. Implement the Worker using the above flow.
2. Create the `rmax` CLI skeleton (list/get/set/disable).
3. Stand up the two KV namespaces and wire IAM.

---

See also:
- [schema](./schema.md)
- [operations](./operations.md)
- [DEVELOPMENT](./DEVELOPMENT.md)
- [security](./security.md)
