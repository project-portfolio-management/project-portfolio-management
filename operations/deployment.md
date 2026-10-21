# Deployment guide

The service is one container (the API) plus PostgreSQL. The browser front-end is
a separate Node application; this guide covers the API and its database, which
is what the root [`compose.yaml`](../compose.yaml) runs. Read
[Recovery objectives](recovery-objectives.md) before you rely on it.

## Prerequisites

- Podman with the Compose plugin, a registry the host can pull from
  (or build the image yourself: `podman compose build`).
- A **token issuer**: login is enforced, so the API must be able to verify the
  bearer tokens your identity provider issues (PASETO keys from your
  authentication service, or a Keycloak realm; see
  [sign in with Microsoft Entra ID](sign-in-with-entra.md)). Without a key set
  the service **refuses to start**.
- PostgreSQL 18 (the Compose file uses `postgres:18-alpine`).

## First start

```sh
git clone https://github.com/project-portfolio-management/project-portfolio-management.git
cd project-portfolio-management

cat > .env <<'ENV'
POSTGRES_PASSWORD=<a long random password>
PPM_PASETO_KEYS_URL=https://auth.example.com/.well-known/paseto-keys
ENV

podman compose up -d --build --wait
curl -fsS http://localhost:5150/_readiness      # {"ok":true}
```

What starts: `postgres`; a one-shot `migrate` job that applies the migrations
compiled into the binary and exits; then `api` on port 5150, healthy when
`/_readiness` answers (it pings the database, the queue and the cache).

**To try it without an identity provider** (your own machine only; login is OFF):

```sh
podman compose --profile dev-insecure up -d --build --wait postgres migrate api-dev-insecure
```

That profile uses `config/container-dev.yaml`, logs a loud warning at start, and
binds to `127.0.0.1`. The service refuses to run like that in `production`.

## Configuration

Everything is an environment variable (names start `PROJECT_PORTFOLIO_MANAGEMENT_`
unless shown). `compose.yaml` sets the essentials; the rest have defaults.

| Purpose | Variable | Default / notes |
|---|---|---|
| Database | `DATABASE_URL` | required; `DB_MIN_CONNECTIONS` (2), `DB_MAX_CONNECTIONS` (20) |
| Environment | `LOCO_ENV` | `production` in the image; `container-dev` is the insecure profile |
| Listen | `PORT`, `HOST` | `5150` |
| Login | `REQUIRE_AUTH` | **on**; only `false` turns it off, and production refuses that |
| Token keys | `PASETO_KEYS_URL`, `PASETO_KEYS`, `PASETO_KEYS_REFRESH_SECS`, `TOKEN_ISSUER`, `TOKEN_AUDIENCE` | keys are fetched at boot and refreshed; set one of URL or inline keys |
| Keycloak tokens | `KEYCLOAK_URL`, `…_REALM`, `…_AUDIENCES`, `…_ROLE_MAP` | needs a build with the `keycloak` feature; see the Entra guide |
| Access policy | `ABAC_POLICY` or `ABAC_POLICY_FILE` | built-in default (read for any signed-in caller, `access=write` to write); a file is re-read without a restart |
| Browser access | `CORS_ORIGINS` | none (same-origin only); exact origins, comma-separated, no wildcard |
| Behind a proxy | `TRUSTED_PROXIES`, `HSTS` | list your proxy's address(es) so client addresses and HTTPS are believed; `HSTS=proxy` (default) |
| Rate limits | `RATE_LIMIT` (on), `RATE_LIMIT_GENERAL_PER_MINUTE` (600) / `_BURST` (200), `RATE_LIMIT_EXPENSIVE_PER_MINUTE` (30) / `_BURST` (10), `RATE_LIMIT_FAILURES_PER_MINUTE` (6) / `_BURST` (20) | per process |
| Files | `ARTIFACT_DIR` | `/app/data/artifacts` (a volume in Compose); `ATTACHMENT_MAX_BYTES` (10 MiB); `BULK_MAX_BYTES`; `BULK_TTL_HOURS`; `REPORT_TTL_DAYS` (calendar days) |
| Search | `SEARCH_INDEX_PATH` | `/app/data/search-index`; `SEARCH_BOOT_REINDEX` |
| Events | `EVENT_TRANSPORT` (`memory`), `EVENT_RELAY`, `EVENT_RETENTION_DAYS` (7 calendar days) | the durable outbox relay is opt-in |
| Mail | `SMTP_HOST`, `SMTP_PORT`, `SMTP_USER`, `SMTP_PASSWORD`, `EMAIL_FROM` | for notifications; an optional bundled server is in [email with Stalwart](email-with-stalwart.md) |
| Telemetry | `OTLP_ENDPOINT`, `OTLP_SERVICE_NAME`, `RUST_LOG` | **`OTLP_ENDPOINT` unset means export ON to `http://localhost:4317`**; set it to the empty string to disable, or to your collector |
| Outbound targets | `OUTBOUND_STRICT` | off. `on` applies the same rule user-supplied notification destinations already get (`https`, a public address, no credentials, no `.internal` names) to the **operator-configured** webhook and worker URLs. Leave it off if those legitimately point at internal services |
| Data-subject requests | `PSEUDONYM_SALT` | optional secret mixed into pseudonyms; without it anyone who knows a person's identifier can recompute the pseudonym. Keep it unchanged: changing it changes later pseudonyms |
| Attachment scanning | `ATTACHMENT_SCAN_ADDR` | unset: no scan. `host:port` of a `clamd` (TCP): every upload is streamed to it before anything is stored; infected is refused, and an unreachable scanner **refuses uploads** (fail closed). Content checks (signature must match the declared type; programs refused) always run |
| Handling marks | `HANDLING_MARKS` | unset: off. A JSON array, lowest first, e.g. `["OFFICIAL","OFFICIAL-SENSITIVE"]`. With it set, a plan above a caller's `clearance` attribute is invisible to them (administrators included), unmarked plans are the lowest level, and every plan response carries `X-Handling-Mark`. A malformed value stops the service at start. Changing the scheme later: a stored mark the new scheme does not list counts as the highest level, so rename codes with care |
| Retention | `cargo loco task retention_purge` | a dry run unless `apply:true`; schedule it daily after reading the dry run. Covers read notifications, delivery logs and trend snapshots; see the [retention schedule](../governance/retention-schedule.md) |
| Tolerance escalations | `cargo loco task tolerance_escalations` | schedule daily: raises one escalation per red tolerance nobody has escalated (FR-54), as `system:tolerance-escalations`. Not run by the in-process ticker |
| Exception breaches | `cargo loco task exception_breaches` | schedule every few minutes: tells the owner of each delivery exception not completed within its window, once (DP-3). Not run by the in-process ticker |
| Director packs | `cargo loco task director_pack_schedule` | schedule daily, **or** set `SCHEDULER_MINUTES`, whose ticker drafts due packs too (DP-12). It only drafts; a person issues |
| Background loops | `SCHEDULER_MINUTES`, `SNAPSHOT_HOURS`, `FLOW_METRICS_SECS`, `SINKS_TICK_SECS` | each is off unless set (sinks tick on by default; `0` turns it off) |

`podman compose run --rm api /app/project-portfolio-management doctor --config`
prints the configuration the service resolved, and `… doctor -e production` checks it.

## TLS and a reverse proxy

The service speaks plain HTTP; terminate TLS in front of it (a load balancer,
nginx, Caddy, or your platform's ingress). Then:

1. Set `TRUSTED_PROXIES` to the proxy's address range, so the real client
   address (for rate limiting) and the `https` scheme (for HSTS) are believed.
   A header from any other peer is ignored.
2. Have the proxy send `X-Forwarded-For` and `X-Forwarded-Proto`.
3. Publish only the proxy's port; do not expose 5150 and the database port to
   the internet. In Compose, remove the `ports:` entry for `postgres` (it has
   none by default) and bind the API to the proxy's network.
4. Probe `/_readiness` from the load balancer. `/_health` and `/_ping` are
   liveness only (they stay `200` with the database down).

## Persistence

Three named volumes: `postgres-data`, `ppm-artifacts` (uploaded files; **back
it up**), and `ppm-search` (derived; safe to lose). Without `ARTIFACT_DIR` on a
volume, uploaded files would live in the container's temporary directory and
vanish on restart; the shipped `compose.yaml` and image set it up for you.

## More than one instance

The service is designed for a single node today. Before running replicas:

- **Artifacts** must be on shared storage (or replaced by an object store), or
  a file uploaded through one replica is a `404` through another.
- **Rate limits** are per process, so each replica enforces its own share.
- **The search index** is local to each replica and each rebuilds it.
- The scheduled loops would run on every replica; enable them on one.

## Kubernetes and Helm

No Helm chart is provided. The image runs under any OCI orchestrator: map
`/_health` to the liveness probe and `/_readiness` to the readiness probe, the
environment table above to a `ConfigMap` and `Secret`, `/app/data/artifacts`
to a `PersistentVolumeClaim`, and run the migration as a `Job` (or init
container) before rolling the Deployment. Write the chart when a cluster target
and its conventions are named.

## Verified

The image builds from a clean checkout, the secure stack starts and answers
`/_readiness`, anonymous API calls and `/metrics.prom` are `401`, and the
security headers are present; production refuses to start with login off or with
no key set. See [Backup and restore](backup-and-restore.md) for the disaster exercise.
