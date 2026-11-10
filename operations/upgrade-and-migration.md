# Upgrade and migration guide

## How versions and migrations work

- A release is a tag `vX.Y.Z` (the API crate's version) and a container image
  with that tag in the registry; the release notes come from
  [`CHANGELOG.md`](../project-portfolio-management-service-with-rust/CHANGELOG.md).
  Deploy a **tag**, never `latest`, so a rollback is "deploy the previous tag".
- The database schema is changed only by **migrations compiled into the binary**.
  Production does not migrate on start, on purpose: the service
  never changes the schema as a side effect of starting. You run the migration
  deliberately, as the one-shot `migrate` job in `compose.yaml`
  (`<service> db migrate`).
- Migrations are **forward-only in practice**. `db down` exists, but it removes
  structure (and the data in it) and is not a rollback plan, so it refuses to run
  outside `development` and `test`, as `db reset` and `db truncate` do. Rollback is
  a restore (below).
- Most migrations only add. The few that rewrite existing rows say so in the
  changelog (for example `m20261009_000049_oxford_spelling` renames the stored
  risk status `materialised` to `materialized`).

## Before you upgrade

1. **Read the changelog entries between your version and the target.** Look for
   **Breaking** and **Changed**. Recent examples that need action:
   - login enforcement is now **on by default** (set up a token verifier, or the
     service refuses to start);
   - `/metrics.prom` now needs a token (give the scraper one);
   - CORS is an allow-list (set `CORS_ORIGINS` if a browser app calls the API
     from another origin);
   - rate limits apply (raise them for a heavy integration, or put it behind a
     trusted proxy so clients are told apart);
   - the route `/api/prioritisation` is now `/api/prioritization`;
   - migration `m20261109_000109` (axa 0.9) makes the outbox a record that keeps
     every event and refuses changes: **stop every relay** (`EVENT_RELAY`) before it
     runs, and unset `EVENT_RETENTION_DAYS`. It **cannot be rolled back** with
     `db down`; to go back, restore the backup. Clients that retry a write under the
     same `Idempotency-Key` after a crash now get `409` for a day, not a minute;
   - axa 0.11 renames settings and health routes: follow
     [Upgrading to axa 0.11](#upgrading-to-axa-011) **before** the new image goes out.
2. **Take a backup and check it restores** ([Backup and restore](backup-and-restore.md)):
   `scripts/backup.sh`, then `scripts/restore-test.sh`. Do not upgrade without a
   backup you have seen restore.
3. Check free disk space (the database grows during a migration that rewrites a table).
4. Note the current tag so you can go back.

## Upgrade (Compose)

```sh
git fetch --tags && git checkout v0.4.0          # or pull the image tag you deploy
podman compose pull                              # or: podman compose build
podman compose run --rm migrate                  # apply the new migrations, then it exits
podman compose up -d --wait api                  # start the new version
curl -fsS http://localhost:5150/_health/ready
```

There is a short outage while the old container stops and the new one starts; a
migration that holds locks lengthens it. Plan a window.

Check the result:

```sh
podman compose run --rm api /app/project-portfolio-management db status   # all migrations applied
podman compose logs --tail=50 api                                                # no errors and no `refusing to start`
```

Then smoke-test as a real user: sign in, open the portfolio, open a plan, open a
chart page, upload and download a file. Watch the logs and the `429` and `5xx`
rates for the first hour.

## Upgrading to axa 0.11

The release that moves the service to axa 0.11 reads none of the old names below, and answers `404` on the old health
routes. Do these first, in every environment, then deploy. Each item names the old and new name.

1. **Rename the settings** in every `.env` file, `compose.yaml` override, Kubernetes `ConfigMap` or `Secret`, and CI
   job. The service **refuses to start** while an old name in the first four rows is set without its new one, and says
   which; nothing falls back to a default database.

   | Old | New | Left alone |
   |---|---|---|
   | `DATABASE_URL` | `DB_URL` | refuses to start, naming `DB_URL` |
   | `LOCO_ENV` | `APP_ENV` | refuses to start, naming `APP_ENV` |
   | `BODY_LIMIT_BYTES` | `HTTP_REQUEST_BODY_LIMIT_BYTE_COUNT` | refuses to start, naming the new name |
   | `HEADER_READ_TIMEOUT_SECONDS` | `HTTP_REQUEST_HEADER_READ_TIMEOUT_SECONDS` | refuses to start, naming the new name |
   | `HTTP_CSP` | `HTTP_CONTENT_SECURITY_POLICY` | nothing: this service sends its own policy and never read either |
   | `DATABASE_URL` for `scripts/backup.sh` and `scripts/restore-test.sh` | `DB_URL` | the script stops, naming `DB_URL` |
   | `RESTORED_DATABASE_URL` (set by `scripts/restore-test.sh -t`) | `RESTORED_DB_URL` | a `-t` command that reads the old name gets nothing |

   Check each environment before you deploy: `<service> check` with that environment's settings exits non-zero, naming
   the setting, if one is still old.
2. **Repoint every probe before upgrading.** Liveness on `/_ping` or `/_health` goes to `/_health/live`; readiness on
   `/_readiness` goes to `/_health/ready`; add a startup probe on `/_health/startup` if the service is slow to boot. The
   same for load balancers, uptime checks and the image's own `HEALTHCHECK` (the new image already uses
   `/_health/live`). The old routes answer `404`, or `401` with login on, so a probe left on one restarts the
   container again and again or never marks it ready. The new routes do not exist on the old version either, so repoint
   in the same change that deploys the new image (and back again if you roll back).
3. **Set `SHUTDOWN_DRAIN_SECONDS` on Kubernetes** (0 by default, which keeps the old behaviour): a little above the
   time the cluster takes to remove a pod from the Service. Readiness answers `503` `shutting_down` from the moment
   shutdown begins, and `terminationGracePeriodSeconds` must exceed the drain plus `SHUTDOWN_TIMEOUT_SECONDS`.
4. **Count `5xx`, not `500`, in alerts.** A database that cannot serve for now (no free connection, a lost connection,
   a read-only, shutting-down or full server) answers `503` `unavailable` with `Retry-After: 5`. The shipped alert
   (`operations/prometheus/alerts.yml`) already counts `5..`.
5. **Expect the new response headers**: a response that sets a cookie is `no-store`, and credential headers are marked
   sensitive so no layer logs them. The service's own `cross-origin-opener-policy`, `cross-origin-resource-policy` and
   `permissions-policy` are unchanged, except that `permissions-policy` now also turns off `browsing-topics`.

No migration comes with the axa 0.11 change, so going back is redeploying the previous tag, with the old names and
probes restored.

## Roll back

Go back to the previous **image tag** and, if the new version applied
migrations, **restore the backup you took before the upgrade**:

```sh
podman compose stop api
# restore the pre-upgrade backup into the database (see backup-and-restore.md),
# including the artifact store if files were uploaded since
git checkout v0.3.0                              # the previous tag
podman compose up -d --wait api
```

If the upgrade applied **no** migration (the changelog says so, and
`db status` showed nothing new), you can redeploy the previous tag without a restore.

Anything written after the backup is lost on a restore; export it first if it matters.

## Running a migration by hand

```sh
podman compose run --rm api /app/project-portfolio-management db status    # what is applied
podman compose run --rm migrate                                                    # apply all pending
```

If a migration fails it runs in a transaction where PostgreSQL allows, so the
database is left at the last good version; read the error, fix the cause (for
example a duplicate that a new unique index rejects), and run it again. A
failure that leaves the schema half-changed is the case for a restore.

## Upgrading PostgreSQL itself

Major versions: dump with `scripts/backup.sh`, start the new server, restore
with `scripts/restore.sh`, then run `scripts/restore-test.sh` against it. Use
client tools of the **new** major version for the restore.
