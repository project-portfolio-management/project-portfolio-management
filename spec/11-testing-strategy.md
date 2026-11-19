## 11. Testing Strategy

[Implementation status](14-implementation-status.md) counts the tests in the source tree.
The following suites exercise distinct parts of the contract. CI records which passed for a revision.

### 11.1 Per subproject

| Subproject | Suite | What a passing run establishes |
|---|---|---|
| Matcher | Unit, property, public API and doctests | The library scores the supplied plan pairs, exposes its component scores and round-trips the plan DTO. |
| Service | Workspace library tests and the database-free part of the `main` integration binary | Domain calculations, validation, route and schema inventories, and repository invariants hold for the cases under test. |
| Service with PostgreSQL | The ignored request suites and real-identity suites in `tests/main.rs` | Actual handlers, migrations, database transactions and access decisions work together on the test database. |
| Operator UI | Type checks, Vitest and Playwright | The production UI build renders and responds through the tested workflows. The browser fixtures supply the API responses. |
| Recovery | The `restore` CI job | A scratch database agrees with its source, the request suites run on the restored copy, and the artifact checks in NFR-16 pass. |

### 11.2 Cross-subproject contracts

- **Service and matcher.** The matcher's `tests/public_api.rs` checks the JSON round trip.
  The service's `tests/requests/plans.rs` stores plans and finds deterministic duplicates across
  different kinds. The request tests in `tests/requests/goals.rs` read back goal edits through the plan.
  The contract also calls for coverage of the duplicate conflict on create and the storage boundary in
  §5.3. Operational sub-resource writes leave matching unchanged except for goal titles.
- **Operator UI and service.** TypeScript mirrors the service DTOs. The Playwright suite uses a
  mock API. A live-service smoke test remains proposed. For a pilot, exercise one complete write and
  read through the deployed UI in the pilot locale, including a validation error. Record the build,
  locale, expected values, and what the operator actually read back (RG-21).

### 11.2a Real Keycloak via Testcontainers (Podman)

`project-portfolio-management-service-with-rust/tests/keycloak_container.rs`
starts a real Keycloak (`quay.io/keycloak/keycloak:26.0`, fully qualified
because Podman refuses short names), imports
`tests/fixtures/keycloak/mxi-realm.json`, mints tokens with the password
grant, and drives them through the real auth guard. It complements the
DB-free `tests/keycloak_guard.rs` (local OIDC stand-in). `#[ignore]`d and
behind the `keycloak` feature; see
[testcontainers-keycloak/](testcontainers-keycloak/index.md). Run:

```sh
export DOCKER_HOST="unix://$(podman machine inspect --format '{{.ConnectionInfo.PodmanSocket.Path}}')"
export TESTCONTAINERS_RYUK_DISABLED=true
cargo test --features keycloak --test main keycloak_container:: -- --ignored
```

Token fetch and verifier configuration must use the same origin
(`http://localhost:<mapped port>`): Keycloak derives `iss` from the request host.

### 11.3 Gates

The [root agent guide](../AGENTS.md) lists the workspace and matcher formatting, lint and test
commands, the PostgreSQL suites, the restore drill and the UI gates. Each behavioural change
includes its specification, implementation and regression test in one commit.
