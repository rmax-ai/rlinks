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
6. Log hit asynchronously (append-only):

   ```
   hit:{code}:{YYYYMMDD}:{uuid}
   ```
7. Return `301 Location: obj.target`

*Note: The Worker does not update `stats` in KV. Stats are computed asynchronously from hit logs.*

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

Error response formats

When the Worker returns an error status (4xx/5xx) it MUST return a machine-readable JSON body for API clients and a small, human-friendly HTML page for browsers. The Worker's behavior is as follows:

- If the request includes an Accept header that prefers `application/json` (or the request is to an API-style path), return Content-Type: application/json and a JSON error object (see example below).
- Otherwise return Content-Type: text/html with a concise HTML page containing the status and a short human-oriented message.

Standard JSON error envelope

```json
{
  "error": {
    "code": "NOT_FOUND",
    "status": 404,
    "message": "Requested redirect code not found",
    "details": null,
    "ts": "2026-01-29T12:34:56Z"
  }
}
```

Guidelines for fields:

- code: short machine identifier (e.g. NOT_FOUND, EXPIRED, LOOP_DETECTED, KV_UNAVAILABLE).
- status: the numeric HTTP status code.
- message: human-readable description suitable for logs and diagnostics.
- details: optional object or string with implementation-level information (avoid leaking secrets).
- ts: ISO-8601 timestamp of the error event.

HTML fallback

Return a minimal HTML page with the status code and message for browser clients. HTML pages MUST not contain any sensitive details (stack traces, tokens, or internal IDs).

Examples

- 404 (Missing code) — JSON: {error.code: "NOT_FOUND", status: 404}
- 410 (Expired) — JSON: {error.code: "EXPIRED", status: 410}
- 500 (Loop detected) — JSON: {error.code: "LOOP_DETECTED", status: 500}
- 503 (KV unavailable) — JSON: {error.code: "KV_UNAVAILABLE", status: 503}

Operators: these response shapes are normative for clients and tests. Update CLI error handling and unit tests to assert on the JSON envelope when the client requests JSON responses.

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
