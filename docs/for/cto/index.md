# For the Chief Technology Officer

*Chief Technology Officer: architecture, quality, how to extend it, and where the engineering is thin.*

## Architecture

Three subprojects, one entity ([architecture](../../../spec/08-architecture.md)):

| Part | Technology |
|---|---|
| Matcher library | Rust; deterministic and probabilistic, explainable pairwise matching; embedded by the service |
| Service | Rust, Loco.rs on axum, SeaORM, PostgreSQL 18; REST under `/api`, OpenAPI derived from the routes |
| UI | SvelteKit 2, Svelte 5 SPA with a backend-for-frontend; i18n catalogues |

The toolchain is pinned in [`rust-toolchain.toml`](../../../rust-toolchain.toml). Persistence is in [spec 10](../../../spec/10-persistence.md); the API in [spec 09](../../../spec/09-api-surface.md).

## Engineering strengths worth knowing

- **Invariants live in the database where they matter:** the audit log refuses change and deletion; issued director packs and delivery-confidence assessments are immutable or append-only by trigger; CHECK constraints back the service rules.
- **Guard tests:** census tests fail the build when a new aggregate route skips the private-plan filter, a list route is unpaged, a personal-data column is missing from the inventory, or a route is missing from OpenAPI. They are fixed by classifying the new thing, not by weakening the test.
- **Real-token tests:** authorization is tested end to end with signed PASETO tokens (one test binary each, because the verifier is process-wide).
- **Supply chain:** `cargo deny`, a software bill of materials, build provenance on release, a security policy.
- **CI** builds and tests everything on every push; see `.github/workflows/`.

## Where it is thin

- Single maintainer, so bus factor is one ([`MAINTAINERS.md`](../../../MAINTAINERS.md)).
- Performance is known roughly; the response cache covers only the critical-path route; very large plans are slow.
- Sessions are in memory unless Redis is configured.
- Several director-pack sections and all pack screens are unbuilt ([`tasks.md`](../../../tasks.md)).
- AI tools are used to build it; the product contains no model. See the [AI statement](../../../AI_STATEMENT.md).

## Decisions that are yours

Hosting and Redis topology, whether to fork or to adopt upstream, who reviews the code independently, and how the service fits your identity provider (PASETO key set, or OIDC through Keycloak or Entra).

## Read next

[Architecture](../../../spec/08-architecture.md) · [Testing strategy](../../../spec/11-testing-strategy.md) · [Implementation status](../../../spec/14-implementation-status.md) · [Benchmarks](../../../BENCHMARKS.md)
