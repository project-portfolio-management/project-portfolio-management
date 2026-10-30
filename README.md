# Project Portfolio Management

One entity, three subprojects: a registry of **plans** (one recursive
`/api/plans` collection; an optional `kind` label such as Portfolio, Project
or Program; any plan may contain any other) with explainable duplicate
matching, plus a project-management workspace (tasks, schedule, cost,
resources, governance).

| Subproject | What it is | Run | Test |
|---|---|---|---|
| [`project-portfolio-management-matcher-with-rust`](project-portfolio-management-matcher-with-rust) | Rust library: the `Plan` type and the matching engine, embedded by the API | `cargo run` (demo) | `cargo test`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo fmt --check` |
| [`project-portfolio-management-service-with-rust`](project-portfolio-management-service-with-rust) | loco.rs REST service; the API DTO is the matcher's `Plan` | `cargo run -- start` (needs Postgres) | `cargo test` (database-free); `DATABASE_URL=… cargo test --test mod -- --ignored --test-threads=1` and the other database binaries (need Postgres; see [AGENTS.md](AGENTS.md)) |
| [`project-portfolio-management-ui-with-svelte`](project-portfolio-management-ui-with-svelte) | SvelteKit 3 / Svelte 5 SPA with a BFF; calls the API | `pnpm install && pnpm dev` (http://localhost:5173) | `pnpm run check`, `npx vitest run`, `npx playwright test` |

The Rust toolchain is pinned in [`rust-toolchain.toml`](rust-toolchain.toml)
(1.99.1). The UI needs Node 26 and pnpm; its e2e tests stub the backend, so
they do not need the service. Run `pkill -f 'vite preview'` before
`npx playwright test` if an earlier preview server may still be running.

## Run the API from a fresh clone

`compose.yaml` at the repository root runs the API with its database; it
builds from `project-portfolio-management-service-with-rust/Dockerfile` with the
repository root as the build context.

```sh
# Secure by default: login enforcement is ON, so the API needs the public keys
# of whatever issues your tokens (see the header of compose.yaml).
POSTGRES_PASSWORD=change-me PPM_PASETO_KEYS_URL=https://auth.example.com/.well-known/paseto-keys \
  podman compose up -d --build --wait
curl -fsS http://localhost:5150/_health

# Throwaway, UNSAFE (login off), for poking with curl on your own machine:
POSTGRES_PASSWORD=x podman compose --profile dev-insecure up -d --build --wait postgres migrate api-dev-insecure
```

Without a key set the service refuses to start, and it refuses to start in
`production` with login off. Continuous integration (`.github/workflows/ci.yml`)
builds and tests everything on every push and pull request; releases are cut by
pushing a `vX.Y.Z` tag (`.github/workflows/release.yml`).

## Where things live

- [`spec/`](spec) is the entity-wide specification (start at
  [`spec/index.md`](spec/index.md)). Each subproject also has its own `spec/`
  for its internals; the entity spec wins on the cross-subproject contract.
- [`agents/`](agents) holds shared guidance for AI agents (start at
  [`agents/index.md`](agents/index.md)); each subproject has an `AGENTS.md`.
- [`tasks.md`](tasks.md) is the backlog of record for the idea-driven items
  (`LT-`, `RM-`, `PL-`, `SC-`, `KO-`, `MS-`, `CR-`, `PJ-`, all ticked, each
  saying what is "Not built"; and the open `DP-`, `EX-`, `HD-` and `CC-`
  items added 2026-10-09).
- [`plan.md`](plan.md) is the research and decision history behind those
  items (ten passes, §1 to §56).
- [`LICENSE/`](LICENSE) holds the licence texts (see [Licence](#licence)).
- [`project-portfolio-management-skill/`](project-portfolio-management-skill)
  is an agent skill describing how to work in this repo.

See [`index.md`](index.md) for a link map.

## About the project

[Install](INSTALL.md) · [Contributing](CONTRIBUTING.md) · [Governance](GOVERNANCE.md) ·
[Maintainers](MAINTAINERS.md) · [Security](SECURITY.md) · [Changelog](CHANGELOG.md) ·
[News](NEWS.md) · [Comparisons](COMPARISONS.md) · [Benchmarks](BENCHMARKS.md) ·
[Request for comments](RFC.md) · [AI statement](AI_STATEMENT.md) · [How to cite](CITATION.cff)

## Licence

Licence is any of these, or contact us for custom licence options. [`LICENSE.md`](LICENSE.md) explains the choice.

* [MIT](https://opensource.org/license/mit) ([SPDX: MIT](https://spdx.org/licenses/MIT.html)), text in [`LICENSE/MIT.txt`](LICENSE/MIT.txt)
* [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([SPDX: Apache-2.0](https://spdx.org/licenses/Apache-2.0.html)), text in [`LICENSE/Apache-2.0.txt`](LICENSE/Apache-2.0.txt)
* [GNU General Public License v2.0](https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html) ([SPDX: GPL-2.0-only](https://spdx.org/licenses/GPL-2.0-only.html)), text in [`LICENSE/GPL-2.0-only.txt`](LICENSE/GPL-2.0-only.txt)
* [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0-standalone.html) ([SPDX: GPL-3.0-only](https://spdx.org/licenses/GPL-3.0-only.html)), text in [`LICENSE/GPL-3.0-only.txt`](LICENSE/GPL-3.0-only.txt)
* [BSD 3-Clause License](https://opensource.org/license/bsd-3-clause) ([SPDX: BSD-3-Clause](https://spdx.org/licenses/BSD-3-Clause.html)), text in [`LICENSE/BSD-3-Clause.txt`](LICENSE/BSD-3-Clause.txt)

SPDX expression: `MIT OR Apache-2.0 OR BSD-3-Clause OR GPL-2.0-only OR GPL-3.0-only`.
