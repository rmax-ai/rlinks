You are a senior Rust systems engineer bootstrapping a new production repository called "rlinks".

This project is the authoritative control plane for a global URL redirect fabric (rmax.to). It manages redirect objects stored in Cloudflare KV and is the only writer allowed.

Repository goals:
- Strongly typed redirect domain model
- Safe schema evolution
- CLI-first operation
- Cloudflare KV integration
- Zero runtime dependencies on Terraform

Create a Rust workspace with:

1. Crates
   - rlinks-core        (domain models, validation, schema, versions)
   - rlinks-kv          (Cloudflare KV client)
   - rlinks-cli         (end-user CLI)

2. Domain model (rlinks-core)
   Implement:
   - Route struct with fields:
     - v (schema version)
     - code
     - target (URL type)
     - status (active, disabled)
     - created_at
     - updated_at
     - created_by
     - meta (notes, tags)
     - rules (https_only, no_loop, expires_at)
     - stats (hits, last_hit)
   - serde Serialize / Deserialize
   - Validation:
     - valid URL
     - no redirect loops to rmax.to
     - reserved words (admin, api, www)
     - HTTPS enforcement

3. KV client (rlinks-kv)
   - Use Cloudflare REST API
   - Functions:
     - get_route(code)
     - put_route(route)
     - delete_route(code)
     - list_routes()
   - Support optimistic concurrency using metadata or updated_at

4. CLI (rlinks-cli)
   Use clap.
   Commands:
     - rlinks list
     - rlinks get <code>
     - rlinks set <code> <url>
     - rlinks disable <code>
     - rlinks delete <code>
     - rlinks stats <code>

5. Config
   Read from env:
     - CLOUDFLARE_ACCOUNT_ID
     - CLOUDFLARE_API_TOKEN
     - KV_NAMESPACE_ID
     - RMAX_DOMAIN (rmax.to)

6. Layout
   - README.md (architecture, how to use)
   - docs/schema.md
   - docs/architecture.md
   - .env.example

7. Quality
   - rustfmt
   - clippy
   - error handling via thiserror + anyhow
   - no unwraps in CLI

8. Provide:
   - Full file tree
   - Cargo.toml for workspace
   - Example implementation for `rlinks set`

Do not generate placeholder files. Generate real, compiling Rust.
```

---

## Three next steps after bootstrap

1. Add integration tests that hit a real Cloudflare KV namespace.
2. Wire `rmax.to` Worker to consume the same schema.
3. Add `hit:` log ingestion in a second CLI command.

---

## Want to go further?

* Schema migration tooling (v1 → v2)
* Signed redirect objects
* Canary deployment of link changes

