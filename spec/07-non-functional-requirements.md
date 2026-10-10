## 7. Non-Functional Requirements

Targets are for the worldwide governmental-portfolio deployment the
Main X Index aims at. The entity is **spec-only; no code exists yet**
(§14), so every target is aspirational and tracked in §13 / §15.

- **NFR-1 — Scale.** Hundreds of thousands of plan records (every
  portfolio, project, product, and program across participating
  departments); each may own thousands of tasks and issues — hence the
  matchable/operational partition (§5.6), which keeps the high-volume
  sub-resources out of both the matcher payload and the cheap list/read
  projections. *(roadmap: `check-duplicates` must use search-blocked
  candidates rather than an in-memory scan — see OQ-2.)*
- **NFR-2 — Performance.** Read ≤ 5 ms p50; list ≤ 20 ms p50; pairwise
  match is pure CPU (no IO) and ≤ 1 ms; `check-duplicates` ≤ 500 ms p99
  at target volumes *(roadmap: requires candidate blocking via
  search)*. Sub-resource list endpoints paginate; timeline / burndown
  are computed within p99 ≤ 200 ms over a single plan's
  sub-resources.
- **NFR-3 — Availability.** Stateless app tier, horizontal scaling,
  PostgreSQL replication, health checks (`/_health`, `/_ping`),
  graceful shutdown, non-root containers. See
  `agents/share/availability.md` (shared family guidance, not in this repository).
- **NFR-4 — i18n / locales.** Plan names, goal titles, keywords,
  and free-text are Unicode-correct end to end; the matcher folds with
  NFKC and **preserves diacritics** (never strips them). `in_language`
  records the plan's language. The operator UI is multilingual in ten
  locales ([locales](locales.md)). *(roadmap: cross-language linkage of the same initiative via `same_as` /
  deterministic identifiers.)*
- **NFR-4a — Locale directory names.** Every directory (or file stem) named
  for a locale uses the form `<language>-<region>`, lowercase and hyphenated:
  `en-001`, `cy-001`, `de-de`, `zh-cn`. `-001` is the UN M.49 code for "world"
  and stands for a language with no regional variant. **A bare two-letter
  directory (`locales/en/`) never exists**: a two-letter code is **not a locale at all**: no URL prefix and no
  forwarding (`/en/plans` is an ordinary unprefixed path, not a redirect to
  `/en-001/plans`), and no place on disk, so each locale has exactly one name. The supported set is
  `LOCALES` in `project-portfolio-management-ui-with-svelte/src/lib/locales.ts`;
  see [locales](locales.md).
  *(Verified 2026-10-07: the repository has no locale directories at all —
  strings live in `src/lib/i18n.svelte.ts` keyed by these codes — and a test
  keeps it so.)*
- **NFR-5 — Security.** TLS at the edge; SSO via the central
  the authentication entity (a separate service) — the service
  verifies PASETO v4 public tokens offline against the auth-service's
  published Ed25519 key
  (`agents/share/authentication-sessions.md` (shared family guidance, not in this repository),
  supersedes the RS256-JWT model); leads / assignees are user
  identities resolved by `EntityRef`; no secrets in code or images.
- **NFR-6 — Auditability.** Soft delete with audit-friendly
  timestamps; full audit log (who/what/when, old + new JSON) and event
  streaming on every CRUD **and sub-resource write** per
  `agents/share/auditability.md` (shared family guidance, not in this repository).
  A durable event bus is roadmap (§15).
- **NFR-7 — Observability.** Structured tracing (`tracing` +
  `tracing-subscriber`, JSON logs in production); OTLP traces /
  metrics / logs *(roadmap)* per
  `agents/share/observability.md` (shared family guidance, not in this repository).
- **NFR-8 — Determinism and explainability.** Matching is a pure,
  total, deterministic function — no clocks, RNGs, IO, or panics in
  the matcher library; every score carries a breakdown an auditor can
  replay. Date-proximity scoring is computed from the records' declared
  dates, never from "now".
- **NFR-9 — Quality gates.** Green `cargo test`,
  `cargo clippy --all-targets -- -D warnings`, `cargo fmt --check`
  (both crates); `pnpm run check` strict 0/0 and production build
  (front-end).
- **NFR-10 — Spelling.** English text uses **Oxford English Dictionary
  spelling** (Oxford spelling): British spelling with **`-ize`** endings, so
  "organization", "prioritization", "utilization", "recognize", "materialized";
  but "analyse", "colour", "programme", "licence" (noun), "centre". This holds
  everywhere — prose, specs, comments, identifiers, routes, JSON keys and
  stored values (e.g. the risk status `materialized`, the route
  `/api/prioritization`). The only exceptions are proper names and published
  titles quoted as published, and text in a locale other than English.
- **NFR-11 — Durations in days.** Every duration phrased in days says
  immediately which kind: **"calendar days"** or **"business days"** — never
  a bare "days" ("5 business days", "within 30 calendar days"). This applies
  to prose, specs, comments, API descriptions, UI text and error messages. A
  field or parameter that holds a count of days names its unit in its
  documentation the same way.
- **NFR-12 — Secure by default.** Login enforcement on `/api/*` is **on**
  unless it is explicitly turned off
  (`PROJECT_PORTFOLIO_MANAGEMENT_REQUIRE_AUTH=false`, or
  `settings.require_auth: false` in a non-production loco config). The service
  **refuses to start** when the loco environment is `production` and
  enforcement is off, and when enforcement is on with no token verifier and no
  key URL; turning it off logs a loud warning. Only `/_health`, `/_ping`, the
  OpenAPI documents, Swagger UI and the per-user calendar feeds are public;
  `/metrics.prom` needs a token.
- **NFR-13 — Security headers and CORS.** Every API response carries
  `X-Content-Type-Options: nosniff`, a restrictive `Content-Security-Policy`
  (`default-src 'none'; frame-ancestors 'none'`), `X-Frame-Options: DENY`,
  `Referrer-Policy`, `Permissions-Policy`, `Cross-Origin-Opener-Policy` and
  `Cross-Origin-Resource-Policy`, and `Strict-Transport-Security` only over
  TLS. The front-end sets the same set and a strict CSP (no `'unsafe-inline'`
  for scripts or style elements). CORS is an **explicit allow-list** of exact
  origins; with none configured, only same-origin callers are served.
- **NFR-14 — Rate limiting.** Requests are rate limited per caller (credential,
  else client address): a general bucket, a tighter one for expensive routes,
  and one for failed authentication. A refusal is `429` with `Retry-After` and
  `RateLimit-*` headers. Health and ping are exempt. The client address honours
  `X-Forwarded-For` only from configured trusted proxies. State is per process.
- **NFR-15 — Licence.** The repository is offered under any one of MIT,
  Apache-2.0, BSD-3-Clause, GPL-2.0-only or GPL-3.0-only, or custom terms by
  arrangement; every manifest declares the one SPDX expression
  `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only`. The
  licence texts live in the root **`LICENSE/`** directory, one file per
  licence, and nowhere else. A root **`LICENSE.md`** explains the choice, states
  the expression, links each text and says how to ask for custom terms; it is
  the only licence-related file outside `LICENSE/`. A test keeps the manifests,
  `LICENSE/` and `LICENSE.md` consistent.
- **NFR-16 — Operations pack and recovery targets.** The repository ships what
  an operator needs: a deployment guide (compose today), backup and restore
  scripts for the database **and** the artifact store, an upgrade and migration
  guide, and a runbook. **Recovery targets are written down**: recovery point
  objective (RPO) and recovery time objective (RTO), stated per backup mode,
  with the scripts' own guarantee distinguished from what point-in-time
  recovery adds. A restore is **tested automatically**: a CI job restores a
  `pg_dump` into a fresh database, checks it against the source and runs the
  test suite against it.
- **NFR-17 — Information governance pack.** The repository ships documents a
  buyer's information governance lead can adapt rather than write from scratch:
  a data protection impact assessment (DPIA) template pre-filled with what this
  service does, a data-flow diagram, a record of processing, a retention
  schedule per kind of record, an accessibility statement (WCAG 2.2 AA), a
  language statement, and answers to the Digital Technology Assessment Criteria
  (DTAC) as a checklist. A test lists every column that can hold personal data
  and fails when a migration adds one the pack's inventory does not name.
- **NFR-18 — Sign-in with an external OIDC provider.** A deployment can offer
  sign-in through Microsoft Entra ID (or any OpenID Connect provider): the
  front-end's server side runs the authorization-code flow with PKCE against an
  OIDC provider (directly, or a Keycloak realm that brokers Entra), keeps tokens
  server-side, and calls the API with the provider's access token, which the API
  verifies. Roles or groups reach ABAC only through an explicit map. The path
  is documented and covered by an end-to-end test that signs in through an
  Entra-compatible provider.
- **NFR-19 — Every page in every supported locale.** All text a person reads on
  a page comes from the locale catalogue (`i18n.svelte.ts`), none is hard-coded
  in English, and Welsh (`cy-001`) is complete: every English key has a Welsh
  entry. A CI test fails when a template or helper holds visible English text
  outside the catalogue, or when `cy-001` lacks a key. Translations are
  reviewed by a professional translator and the review is recorded; until then
  they are marked draft. Text that the API itself generates (derived sentences
  in responses) is not part of the catalogue and is listed as a limitation.

- **NFR-20 — Public-repository files.** The repository root carries the
  standard files of a public project, listed in
  [`special-files-for-public-repos`](special-files-for-public-repos/index.md):
  `README.md`, `LICENSE.md` (with SPDX information), `CITATION.cff`, `NEWS.md`,
  `COMPARISONS.md`, `BENCHMARKS.md`, `INSTALL.md`, `CONTRIBUTING.md`, `RFC.md`,
  `CODEOWNERS`, `MAINTAINERS.md` (Joel Parker Henderson as sole maintainer),
  `CHANGELOG.md`, `AI_STATEMENT.md`, `GOVERNANCE.md` and `SECURITY.md`. They
  state facts the repository can support: a benchmark page holds only numbers
  from a recorded run; a comparison says what was and was not read; the AI
  statement describes the practice as it is (including that commits made in AI
  sessions carry a `Co-Authored-By` trailer). A test fails when one is missing or
  empty, when the maintainer or owner address is absent from the files that need
  it, or when the citation file lacks its required fields.

- **NFR-21 — Controls for regulated buyers.** A deployment can meet the common
  expectations of a health or government buyer: an **idle session timeout**
  (default 30 minutes, configurable); an **audit trail the database refuses to
  change or delete** except through the retention task; a **retention purge**
  that is a dry run unless told otherwise and audits what it removes; **backups
  that can be encrypted**, with a restore drill that proves they decrypt; an
  optional **strict mode for operator-configured outbound targets**; an
  **access review** report for recertification; **break-glass access** to private plans (time-limited, reasoned,
  witnessed and audited); **attachment content checks** with an
  optional virus-scan hook that fails closed; and an
  administrator-only **data-subject tool** that exports everything held about a
  person reference and pseudonymises it (not deletes it) across the schema, audit
  trail included. The limits
  are stated where the controls are: a database administrator can disable the
  trigger; the purge covers only the records whose deletion is unambiguous. The
  honest assessment is [`governance/production-readiness.md`](../governance/production-readiness.md).

### 7.x Time and time zones

Doctrine (LT-9, `src/tz.rs`): **store UTC** (`timestamptz`, from `Utc::now()`);
a `date` is a **calendar date with no zone**; **the server never reads its own
zone** (`chrono::Local` is banned by `clippy.toml` and by a unit test, so any
process `TZ` yields identical answers); **portfolio-wide views use the UTC
date** so two callers see one answer; **personal views take an explicit IANA
zone** per request (`?tz=`), and fixed offsets are refused because they cannot
follow daylight saving.
