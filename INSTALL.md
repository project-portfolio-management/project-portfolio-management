# Install and use

Three pieces: a Rust library (the matcher), a Rust REST service that embeds it
and stores data in PostgreSQL, and a SvelteKit front end that calls the service.
Most people want the service and the front end. The fuller operator material is
in [`operations/`](operations/README.md).

## Requirements

| For | You need |
|---|---|
| The service | Docker or Podman with Compose (easiest), or Rust 1.96.1 (pinned in [`rust-toolchain.toml`](rust-toolchain.toml)) and PostgreSQL 18 |
| The front end | Node 26 and pnpm |
| Signing in | A token issuer the service can verify: PASETO keys from your authentication service, or a Keycloak realm (which can broker Microsoft Entra ID). See [`operations/sign-in-with-entra.md`](operations/sign-in-with-entra.md) |

Login is **on by default**. The service refuses to start without verification
keys, and refuses `production` with login off.

## 1. Run the service with Compose

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management

POSTGRES_PASSWORD='<a long random password>' \
PPM_PASETO_KEYS_URL='https://auth.example.com/.well-known/paseto-keys' \
  docker compose up -d --build --wait
curl -fsS http://localhost:5150/_readiness
```

To look around on your own machine **with login off** (never expose this):

```sh
POSTGRES_PASSWORD=x docker compose --profile dev-insecure up -d --build --wait \
  postgres migrate api-dev-insecure
curl -s http://localhost:5150/api/plans
```

The deployment guide covers configuration, volumes, backups and upgrades:
[`operations/deployment.md`](operations/deployment.md),
[`operations/backup-and-restore.md`](operations/backup-and-restore.md),
[`operations/upgrade-and-migration.md`](operations/upgrade-and-migration.md).

## 2. Run the front end

```sh
cd project-portfolio-management-ui-with-svelte
cp .env.example .env     # PROJECT_PORTFOLIO_MANAGEMENT_API_URL and AUTH_API_URL, default http://localhost:5150
pnpm install
pnpm dev                 # http://localhost:5173
```

The interface is available in several languages (English, Welsh and others);
the first path segment is the locale, for example `/en-001/plans` or
`/cy-001/plans`. See [`governance/language-statement.md`](governance/language-statement.md).

## 3. Use the matcher library on its own

```sh
cd project-portfolio-management-matcher-with-rust
cargo run        # a small demo
cargo test
```

## 4. Build and test everything from source

```sh
# Matcher and service
(cd project-portfolio-management-matcher-with-rust && cargo test)
(cd project-portfolio-management-service-with-rust && cargo test)               # database-free
(cd project-portfolio-management-service-with-rust && cargo test -- --ignored)  # needs PostgreSQL

# Front end
cd project-portfolio-management-ui-with-svelte
pnpm run check && npx vitest run && npx playwright test
```

Continuous integration runs all of this on every push; see
[`.github/workflows/ci.yml`](.github/workflows/ci.yml).

## Releases

A release is a `vX.Y.Z` tag; the workflow in
[`.github/workflows/release.yml`](.github/workflows/release.yml) builds and
publishes. Pin a version in production rather than tracking `main`.

## If something does not work

See the [runbook](operations/runbook.md), then open an issue. For a security
problem, use [SECURITY.md](SECURITY.md) instead.

## Redis (production cache)

`config/production.yaml` uses Redis as Loco's cache. Set `PROJECT_PORTFOLIO_MANAGEMENT_REDIS_URL`
(`redis://:password@host:6379/0`, or `rediss://` for TLS); `compose.yaml` starts one and reuses the database
password unless `PPM_REDIS_PASSWORD` is set. Without a reachable Redis the service starts but `/_readiness`
answers 503. The cache holds nothing that cannot be rebuilt, and nothing in the service reads or writes it yet.
