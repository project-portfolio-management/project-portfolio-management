# Upgrade and migration guide

## How versions and migrations work

- A release is a tag `vX.Y.Z` (the API crate's version) and a container image
  with that tag in the registry; the release notes come from
  [`CHANGELOG.md`](../project-portfolio-management-api-with-rust/CHANGELOG.md).
  Deploy a **tag**, never `latest`, so a rollback is "deploy the previous tag".
- The database schema is changed only by **migrations compiled into the binary**.
  `config/production.yaml` sets `auto_migrate: false` on purpose: the service
  never changes the schema as a side effect of starting. You run the migration
  deliberately, as the one-shot `migrate` job in `compose.yaml`
  (`<service> db migrate`).
- Migrations are **forward-only in practice**. `db down` exists, but it removes
  structure (and the data in it) and is not a rollback plan. Rollback is a
  restore (below).
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
   - the route `/api/prioritisation` is now `/api/prioritization`.
2. **Take a backup and check it restores** ([Backup and restore](backup-and-restore.md)):
   `scripts/backup.sh`, then `scripts/restore-test.sh`. Do not upgrade without a
   backup you have seen restore.
3. Check free disk space (the database grows during a migration that rewrites a table).
4. Note the current tag so you can go back.

## Upgrade (Compose)

```sh
git fetch --tags && git checkout v0.4.0          # or pull the image tag you deploy
docker compose pull                              # or: docker compose build
docker compose run --rm migrate                  # apply the new migrations, then it exits
docker compose up -d --wait api                  # start the new version
curl -fsS http://localhost:5150/_readiness
```

There is a short outage while the old container stops and the new one starts; a
migration that holds locks lengthens it. Plan a window.

Check the result:

```sh
docker compose run --rm api /app/project-portfolio-management-service db status   # all migrations applied
docker compose logs --tail=50 api                                                # no errors and no `refusing to start`
```

Then smoke-test as a real user: sign in, open the portfolio, open a plan, open a
chart page, upload and download a file. Watch the logs and the `429` and `5xx`
rates for the first hour.

## Roll back

Go back to the previous **image tag** and, if the new version applied
migrations, **restore the backup you took before the upgrade**:

```sh
docker compose stop api
# restore the pre-upgrade backup into the database (see backup-and-restore.md),
# including the artifact store if files were uploaded since
git checkout v0.3.0                              # the previous tag
docker compose up -d --wait api
```

If the upgrade applied **no** migration (the changelog says so, and
`db status` showed nothing new), you can redeploy the previous tag without a restore.

Anything written after the backup is lost on a restore; export it first if it matters.

## Running a migration by hand

```sh
docker compose run --rm api /app/project-portfolio-management-service db status    # what is applied
docker compose run --rm migrate                                                    # apply all pending
```

If a migration fails it runs in a transaction where PostgreSQL allows, so the
database is left at the last good version; read the error, fix the cause (for
example a duplicate that a new unique index rejects), and run it again. A
failure that leaves the schema half-changed is the case for a restore.

## Upgrading PostgreSQL itself

Major versions: dump with `scripts/backup.sh`, start the new server, restore
with `scripts/restore.sh`, then run `scripts/restore-test.sh` against it. Use
client tools of the **new** major version for the restore.
