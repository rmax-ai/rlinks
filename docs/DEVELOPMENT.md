# `DEVELOPMENT.md`

## 1. Prime directive

Every line of code in **rlinks** must make link routing **more predictable**.

Breaking a link is a production outage.
A broken redirect is equivalent to deleting a DNS record.

---

## 2. Repository structure

```
rlinks/
  crates/
    rlinks-core/
    rlinks-kv/
    rlinks-cli/
  docs/
    schema.md
    architecture.md
  .env.example
  GUIDELINES.md
  DEVELOPMENT.md
```

Each crate has a single responsibility:

* `rlinks-core` — domain model + validation
* `rlinks-kv` — Cloudflare integration
* `rlinks-cli` — user interface

No cross-layer leakage.

---

## 3. Local development

You must have:

* Rust stable
* Cloudflare account
* A KV namespace for dev

Environment:

```
CLOUDFLARE_ACCOUNT_ID
CLOUDFLARE_API_TOKEN
KV_NAMESPACE_ID
RMAX_DOMAIN=rmax.to
```

Never use production KV in development.

---

## 4. Schema-first development

Any change to redirect behavior must start with the schema.

You must:

1. Update the Rust struct
2. Update `docs/schema.md`
3. Add or update validation logic
4. Add migration logic if needed

Code that does not flow from schema is invalid.

---

## 5. Testing discipline

Every change must include at least one of:

* Unit test (validation, parsing)
* Integration test (KV round-trip)
* Property test (invariants)

Minimum required invariants:

* No redirect loops
* HTTPS enforcement
* Reserved word blocking
* Status gating

No PR may weaken invariants.

---

## 6. KV safety rules

All KV writes must be:

* Atomic
* Version-aware
* Logged

Never overwrite a record without:

* Reading it first
* Preserving immutable fields
* Updating `updated_at`

---

## 7. Migrations

Schema versions are forward-only.

If you add a field:

* It must be optional for older records
* A migration tool must exist

No in-place destructive rewrites.

---

## 8. Release flow

```
git commit
→ tests
→ cargo build
→ version bump
→ tag
→ ship CLI
```

The Worker and the CLI must agree on schema version before release.

---

## 9. Production changes

All production changes go through:

```
rlinks set / disable / delete
```

Never through:

* Wrangler
* Cloudflare dashboard
* Direct KV writes

---

## 10. Debugging

If a redirect is wrong:

1. Fetch its KV object
2. Inspect schema version
3. Check hit logs
4. Reproduce via CLI

Never guess.

---

## 11. Performance guardrails

Edge path:

* Must remain single KV lookup
* Must not parse large JSON
* Must not allocate large objects

CLI path:

* Can be slow
* Can be verbose
* Must be correct

---

## Three development rules

1. Schema before code.
2. Validation before write.
3. Observability before features.

---

See also:
- [GUIDELINES](./GUIDELINES.md)
- [schema](./schema.md)
- [SPEC](./SPEC.md)

