# Runbook

For the person on call. Start with the quick reference, then find the symptom.

## Quick reference

| Thing | Where |
|---|---|
| Liveness | `GET /_health` and `GET /_ping` (stay `200` with the database down) |
| Readiness | `GET /_readiness` (`200` `{"ok":true}`; `503` when the database, queue or cache fails) |
| Metrics | `GET /metrics.prom` (needs a bearer token); flow gauges are `ppm_flow_*` |
| Logs | standard output, JSON in production (`docker compose logs api`) |
| Traces and metrics export | OTLP to `OTLP_ENDPOINT` |
| Version | `<service> version`; the image tag |
| Configuration as resolved | `<service> doctor --config` |
| Migration state | `<service> db status` |

`<service>` is `docker compose run --rm api /app/project-portfolio-management-service`.

## Symptoms

### The service will not start

The start-up gate prints a message beginning `refusing to start:`. Do what it says.

| Message | Cause | Fix |
|---|---|---|
| `authentication enforcement is off in the production environment` | `REQUIRE_AUTH=false` (or `settings.require_auth: false`) under `LOCO_ENV=production` | remove the opt-out; production never runs anonymously |
| `authentication enforcement is on but no token verifier is configured` | no `PASETO_KEYS`, `PASETO_KEYS_URL` or Keycloak settings | set one (see [Deployment](deployment.md)) |
| `PoolTimedOut` / `Conn` | the database is unreachable or the URL is wrong | check `DATABASE_URL`, the database, the network |
| YAML or `Tera` errors naming a config file | a malformed config or a missing required variable (`DATABASE_URL`) | read the line; `doctor --config` |

### Everyone gets `401`

Login is enforced. Most likely: the key set did not load or rotated, or the
issuer or audience differs. Check the logs at start (`auth:` lines); with
`PASETO_KEYS_URL`, check the URL answers from the container and that the keys
refresh (`PASETO_KEYS_REFRESH_SECS`). Check `TOKEN_ISSUER` and `TOKEN_AUDIENCE`
match what your issuer puts in tokens. A clock skew of more than a few minutes
between issuer and service also rejects tokens.

### One user gets `403`

Authenticated but not allowed: the access policy denied the action. The default
policy lets any signed-in caller read and requires `access=write` to write
(`access=admin` to delete or merge). Check the user's attributes at the issuer
(or the Keycloak role map) and `ABAC_POLICY_FILE`. The audit log records who did what.

### `429 Too Many Requests`

A rate limit tripped; the response says which and `Retry-After` how long. If a
single integration or office is hitting it:

1. Is the real client address visible? Behind a proxy, set `TRUSTED_PROXIES`;
   otherwise every user shares the proxy's address.
2. Raise the matching limit (`RATE_LIMIT_GENERAL_*`, `RATE_LIMIT_EXPENSIVE_*`).
3. Many `401`s from one address fill the failures bucket: find what is sending
   bad tokens (an expired token in a script).

### `500` on data routes while `/_health` is `200`

The database is unavailable or erroring; `/_health` is liveness only. Check
`/_readiness` (`503`), the database, connections (`DB_MAX_CONNECTIONS` vs the
server limit), and disk. The service recovers by itself when the database
returns (verified: after a database restart the next request succeeded).

### Slow

Check the database first (long queries, missing connections). The simulation
and scheduling endpoints (CPM, levelling, schedule/cost/joint risk) and the
imports and exports are the expensive ones; they are rate limited, and a very
large plan makes them slower. Look at the trace for the slow route.

### Search finds nothing or is stale

The index is derived. Rebuild it: `<service> task search_reindex`. It also
rebuilds at start when empty and records exist. If the index directory is on a
volume that was lost, this is all that is needed.

### Uploads or downloads fail

`404` on a download: the file is not in `ARTIFACT_DIR` (a lost volume, or a
second replica without shared storage). `422` on upload: the type or size is not
allowed (`ATTACHMENT_MAX_BYTES`, the allow-list). Out of disk: grow the volume,
then check the database too.

### A browser app reports a CORS error

Add the app's exact origin to `CORS_ORIGINS` and restart. There is no wildcard
by design.

### Notifications or webhooks do not arrive

Check the sink's delivery log in the app, the SMTP settings (`SMTP_*`), and that
`SINKS_TICK_SECS` is not `0`. Webhook targets on loopback addresses are refused
unless `SINKS_ALLOW_LOOPBACK` is set (for testing only).

## Routine tasks

**Rotate the token keys.** Publish the new key set at the issuer (keep the old
one until tokens signed with it expire). With `PASETO_KEYS_URL` the service
re-fetches on its refresh interval; with inline `PASETO_KEYS`, update the
secret and restart.

**Change the access policy.** Edit `ABAC_POLICY_FILE`; it is re-read without a
restart. A malformed policy falls back to the built-in default (so a bad edit
cannot open the service), and logs why.

**Take a backup now / restore.** [Backup and restore](backup-and-restore.md).

**Upgrade or roll back.** [Upgrade and migration](upgrade-and-migration.md).

**Investigate an incident.** Preserve first: take a backup, copy the logs. Then
use the audit log (per plan and estate-wide for auditors) to see who changed
what; the compliance extract and evidence pack are in the app. To cut access
quickly: revoke the user at the issuer (their token expires), rotate the keys
for everyone, or block the address or path at the proxy.

## Escalation

Record: the time, the symptom, the request ids from the logs, what you changed.
If data may have been lost or exposed, follow your organization's incident and
personal-data-breach process (see the [information governance pack](../governance/README.md)).
