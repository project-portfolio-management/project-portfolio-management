# Backup and restore

The scripts are in [`../scripts`](../scripts). They need `pg_dump`, `pg_restore`
and `psql` **at least as new as the server** (a PostgreSQL 18 server needs
version 18 tools), or you run them inside the database container (below).

## What is backed up

- the **database**, as a `pg_dump` custom-format file (`db.dump`);
- the **artifact store** (uploaded attachments, export files) as `artifacts.tar.gz`;
- a `MANIFEST` (when, server version, migration count and latest migration, row
  count per table) and `SHA256SUMS` over all of it.

The search index is not backed up; it is rebuilt (`task search_reindex`).

## Take a backup

```sh
DATABASE_URL=postgres://user:pass@db-host:5432/project_portfolio_management \
ARTIFACT_DIR=/var/lib/ppm/artifacts \
  scripts/backup.sh -o /backups
# -> /backups/ppm-backup-20261010T020000Z/{db.dump,artifacts.tar.gz,MANIFEST,SHA256SUMS}
```

A failed backup removes its partial directory. Copy the directory **off the
machine** (object storage, another site) and keep it encrypted. The script can
do the encrypting (needs `gpg`):

```sh
# a passphrase kept in a file (mode 0600) OUTSIDE the backup location ...
BACKUP_PASSPHRASE_FILE=/etc/ppm/backup.pass scripts/backup.sh -o /backups
# ... or a public key, so the machine that makes backups cannot read them back
BACKUP_GPG_RECIPIENT=backups@example.org scripts/backup.sh -o /backups
# -> db.dump.gpg, artifacts.tar.gz.gpg, MANIFEST (readable), SHA256SUMS (over the encrypted files)
```

`scripts/restore.sh` decrypts into a private temporary directory that is removed
on exit (set `BACKUP_PASSPHRASE_FILE`, or have the private key in the gpg
keyring); a wrong key stops before the database is touched, and a changed file
fails the checksum before any decryption. **Losing the key or passphrase loses
the backup.** The restore drill also runs encrypted in CI. The dump
contains personal data (see the [retention schedule](../governance/retention-schedule.md)).

### With the Compose deployment

The database and the artifact volume live in containers, so run the tools there:

```sh
export DATABASE_URL=postgresql://ppm:$POSTGRES_PASSWORD@localhost:5432/project_portfolio_management
export PG_DUMP="docker compose exec -T postgres pg_dump"
export PSQL="docker compose exec -T postgres psql"
export ARTIFACT_TAR_CMD="docker compose exec -T api tar -C /app/data/artifacts -cz ."
scripts/backup.sh -o /backups
```

(`podman compose` works the same. Use `api-dev-insecure` instead of `api` if that
is the service you run.)

### Schedule it, and watch it

```cron
# nightly at 02:00; the RPO in recovery-objectives.md assumes this interval
0 2 * * *  cd /srv/ppm && scripts/backup.sh -o /backups
# alert if the newest backup is older than the RPO (24 h here), checked hourly
0 * * * *  cd /srv/ppm && scripts/check-backup-age.sh /backups 24 || alert "ppm backup is stale"
```

Also prune old backups by your retention policy (keep at least the most recent
verified one).

## Restore

1. **Stop the service** that writes to the target database.
2. Create an empty database (`createdb`), or use `-f` to replace what is there.
3. Restore, then start the service; migrations are already applied (they are in
   the dump), and the one-shot `migrate` job is a no-op.

```sh
scripts/restore.sh /backups/ppm-backup-20261010T020000Z \
    postgres://user:pass@db-host:5432/project_portfolio_management \
    -a /var/lib/ppm/artifacts
```

`restore.sh` verifies `SHA256SUMS` first and **refuses** on any mismatch, and it
refuses to restore over a database that already has tables unless you pass `-f`.
The database restore runs in a single transaction, so a failure leaves the
target as it was.

With Compose (this exact sequence was exercised after deleting every volume):

```sh
docker compose up -d --wait postgres                       # an empty database
export DATABASE_URL=postgresql://ppm:$POSTGRES_PASSWORD@localhost:5432/project_portfolio_management
export PSQL="docker compose exec -T postgres psql"
export PG_RESTORE="docker compose exec -T postgres pg_restore" RESTORE_FROM_STDIN=1
scripts/restore.sh /backups/ppm-backup-20261010T020000Z "$DATABASE_URL"   # the database
docker compose up -d --wait migrate api                    # start the service (migrate is a no-op)
docker compose exec -T api tar -C /app/data/artifacts -xz \
    < /backups/ppm-backup-20261010T020000Z/artifacts.tar.gz               # the artifact store
```

(`podman compose` works the same; use `api-dev-insecure` instead of `api` if that
is the service you run. `restore.sh` can also unpack the artifacts itself when
the API container is already up: set `ARTIFACT_UNTAR_CMD` to the `tar -xz`
command above.)

## The restore drill

A backup that has never been restored is a hope. `scripts/restore-test.sh`
proves it:

```sh
DATABASE_URL=postgres://user:pass@db-host:5432/project_portfolio_management \
  scripts/restore-test.sh
```

It takes a backup of the source, restores it into a scratch database, and fails
unless **the migration history is identical**, **every table has the same rows
and the same content digest**, and the **restored artifact file count matches**.
`-t COMMAND` runs a command against the restored database before it is dropped
(`RESTORED_DATABASE_URL` is exported); CI uses it to run the whole request test
suite against the restored copy. `-k` keeps the scratch database so you can
inspect it and time the restore.

The CI job `restore` runs this on every push. Run it yourself on production
data on a schedule (weekly is reasonable) against a copy, and record the date
and the elapsed time.

### What was verified when this was written

- the drill passes on a database with 109 tables and real rows, and the 153
  request tests pass against the restored copy;
- a flipped byte in `db.dump`, a non-empty target without `-f`, a stale backup,
  a missing backup directory and a failing follow-up command are all refused or
  reported (exit status checked);
- a full Compose disaster exercise: upload a file, back up through the
  containers, **delete every volume**, restore, and the plan and the
  byte-identical attachment came back.

Not verified: very large databases (time them yourself), and Mode B point-in-time recovery (it is your database platform's feature).
