Website Creation Prompt for rlinks

# Objective
Produce a concise, high-signal, single-page marketing-style site that explains what **rlinks** is, why it exists, how it works, and how to contribute or operate it. The content should reflect the project's focus on identity-grade infrastructure, strict safety guarantees, and the CLI-first operational workflow for the rmax.to redirect fabric.

# Audience
Primarily internal engineers, infrastructure stewards, and technical stakeholders who need to understand and trust the redirect control plane. Secondarily, curious engineers evaluating the architecture and guarantees of a global redirect fabric.

# Messaging pillars
1. **Identity infrastructure, not a marketing shortener** – emphasize that rlinks is infrastructure for canonical links, not a SaaS stack or marketing tool.
2. **Single-writer/hot path safety** – highlight the separation between Terraform-managed infra, the CLI control plane, and the stateless edge worker; mention Cloudflare KV as the runtime truth.
3. **Typed, auditable redirect objects** – describe schema-first design, versioned records, loggable hits, and forward-only schema evolution.
4. **Observability and reversibility** – note append-only hit logs, auditability of every mutation, disable-over-delete policy, and the safety constraints enforced by the CLI.
5. **Developer discipline** – point visitors to key docs (`GUIDELINES.md`, `DEVELOPMENT.md`, `schema.md`, `docs/architecture.md`, etc.) and summarize development best practices.

# Suggested layout
1. **Hero section**
   - Tagline: “rlinks keeps `rmax.to` redirects eternal, safe, and observable.”
   - Subtext referencing typed redirect objects, CLI control, Cloudflare edge runtime, and <10ms global redirects.
   - CTA buttons: “Read the architecture” (link to docs/architecture.md) and “Inspect the schema” (link to docs/schema.md).
2. **What is rlinks?**
   - Draw from README: “authoritative control plane, typed redirect system, CLI-first, global edge runtime.”
   - Clarify it is not a shortener or multi-tenant SaaS; reserved for identity infrastructure.
3. **Architecture & planes**
   - Visual or simplified ASCII flow: Terraform → Cloudflare Platform → rlinks CLI → KV → Worker → Internet.
   - Describe infrastructure/control/runtime planes and their responsibilities.
4. **Redirect lifecycle & schema**
   - Outline creation via CLI, validation, KV write, worker lookup, hit logging, disable/rotate.
   - Mention schema fields (version `v`, `code`, `target`, `rules`, `stats`, etc.) and invariants (no loops, reserved words, HTTPS, immutable metadata, forward-only versioning).
5. **Safety guarantees**
   - Use list to describe invariants and observability requirements (no loops, reserved names, disable instead of delete, append-only hits, no silent failures).
   - Mention command discipline and incidents treated like production outages.
6. **CLI & operations**
   - Highlight commands: list/get/set/disable/delete/stats and the strict write pipeline (rlinks-cli → rlinks-core → rlinks-kv → Cloudflare KV).
   - Include snippet showing `rlinks set`/`disable`/`get` etc.
7. **Development & contribution**
   - Summarize developer rules (schema-first, tests for invariants, local dev env with Cloudflare KV, no direct KV writes, mind performance guardrails).
   - Link to key docs and mention required tests (cargo test, clippy, fmt) per rules.
8. **Roadmap & next steps**
   - Mention features from README roadmap (analytics, canary routing, temporal redirects, signed links, geo-aware routing).
   - Optionally call out three actionable next steps from SPEC (implement worker flow, CLI skeleton, stand up namespaces).
9. **Footer**
   - License (internal), philosophy quote (“Links are part of identity... infrastructure must be boring”), contact info placeholder (e.g., created_by field references) and note that CLI/worker must agree on schema before release.

# Tone & style
Technical, confident, and calm. Avoid marketing fluff. Use short, declarative sentences. When referencing commands or config, use monospace.

# Assets & references
- README.md for high-level vision and roadmap.
- docs/GUIDELINES.md, docs/DEVELOPMENT.md, docs/architecture.md, docs/SPEC.md, docs/schema.md.
- .env.example for configuration references.

# Deliverable format
Provide this prompt file to a website generator (or designer) so they can craft the site copy and structure described above. Ensure all linked docs or data points are referenced explicitly, with relative URLs where appropriate.
