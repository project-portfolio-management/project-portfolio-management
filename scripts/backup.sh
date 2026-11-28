#!/bin/sh
# Back up the database and the artifact store into one verified directory.
#
#   DB_URL=postgres://user:pass@host/db scripts/backup.sh [-o BACKUP_ROOT]
#
# Writes BACKUP_ROOT/ppm-backup-<UTC timestamp>-<unique suffix>/ containing
#   db.dump             pg_dump custom format (restore with scripts/restore.sh)
#   artifacts.tar.gz    the artifact store (attachments, export files), if present
#   MANIFEST            when, what, how many migrations, row counts per table
#   SHA256SUMS          checksums of every file above (verified on restore)
#
# Environment:
#   DB_URL        required. The database to dump.
#   ARTIFACT_DIR        the artifact store (default: $PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR;
#                       if neither is set the artifact step is skipped and the MANIFEST says so)
#   PG_DUMP, PSQL       commands to run (default pg_dump / psql). Override to run inside
#                       the database container of a compose deployment:
#                         PG_DUMP="podman compose exec -T postgres pg_dump"
#                         PSQL="podman compose exec -T postgres psql"
#                       (DB_URL is then resolved inside that container).
#   ARTIFACT_TAR_CMD    a command that writes the artifact store as a gzip tar to stdout, for
#                       a store that is not a host directory (compose: a named volume):
#                         ARTIFACT_TAR_CMD="podman compose exec -T api tar -C /app/data/artifacts -cz ."
#
# Encryption (optional, RG-5): the database dump and the artifact archive hold personal data, so
# encrypt a backup that leaves the machine. Set ONE of
#   BACKUP_PASSPHRASE_FILE   a file holding a passphrase (mode 0600); symmetric AES-256
#   BACKUP_GPG_RECIPIENT     a public key id/email in the gpg keyring; only its private key restores
# and the files become db.dump.gpg / artifacts.tar.gz.gpg; SHA256SUMS covers the encrypted files, so
# tampering is caught before decryption. MANIFEST stays readable (table names and row counts only).
# The service-side search index is not backed up. Needs gpg. Keep the key or passphrase OUTSIDE the
# backup location: a backup you cannot decrypt is not a backup.
#
# A backup is only as good as its last restore: run scripts/restore-test.sh on a
# schedule (the CI job `restore` does) and scripts/check-backup-age.sh in monitoring.
set -eu

ROOT="./backups"
while getopts "o:" opt; do
  case "$opt" in
    o) ROOT="$OPTARG" ;;
    *) echo "usage: $0 [-o BACKUP_ROOT]" >&2; exit 2 ;;
  esac
done

[ -z "${DATABASE_URL:-}" ] || [ -n "${DB_URL:-}" ] || { echo "DATABASE_URL is no longer read (axa 0.11): set DB_URL" >&2; exit 2; }
: "${DB_URL:?set DB_URL to the database to back up}"
PG_DUMP="${PG_DUMP:-pg_dump}"
PSQL="${PSQL:-psql}"
ARTIFACT_DIR="${ARTIFACT_DIR:-${PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR:-}}"
ARTIFACT_TAR_CMD="${ARTIFACT_TAR_CMD:-}"

if [ -z "$ARTIFACT_TAR_CMD" ] && [ -n "$ARTIFACT_DIR" ] && [ ! -d "$ARTIFACT_DIR" ]; then
  echo "backup: configured artifact directory does not exist: $ARTIFACT_DIR" >&2
  exit 1
fi

PASSPHRASE_FILE="${BACKUP_PASSPHRASE_FILE:-}"
GPG_RECIPIENT="${BACKUP_GPG_RECIPIENT:-}"
ENCRYPTION="none"
if [ -n "$PASSPHRASE_FILE" ] && [ -n "$GPG_RECIPIENT" ]; then
  echo "backup: set BACKUP_PASSPHRASE_FILE or BACKUP_GPG_RECIPIENT, not both" >&2; exit 2
fi
if [ -n "$PASSPHRASE_FILE" ]; then
  [ -s "$PASSPHRASE_FILE" ] || { echo "backup: $PASSPHRASE_FILE is missing or empty" >&2; exit 2; }
  ENCRYPTION="gpg-symmetric-aes256"
elif [ -n "$GPG_RECIPIENT" ]; then
  ENCRYPTION="gpg-public-key"
fi
if [ "$ENCRYPTION" != none ]; then
  command -v gpg >/dev/null 2>&1 || { echo "backup: encryption needs gpg, which is not installed" >&2; exit 2; }
fi
# encrypt FILE -> FILE.gpg, removing the plain file; no-op when encryption is off.
encrypt() {
  [ "$ENCRYPTION" = none ] && return 0
  if [ -n "$PASSPHRASE_FILE" ]; then
    gpg --batch --yes --quiet --pinentry-mode loopback --passphrase-file "$PASSPHRASE_FILE" \
      --symmetric --cipher-algo AES256 -o "$1.gpg" "$1"
  else
    gpg --batch --yes --quiet --trust-model always --encrypt --recipient "$GPG_RECIPIENT" -o "$1.gpg" "$1"
  fi
  rm -f "$1"
}

sha256() { if command -v sha256sum >/dev/null 2>&1; then sha256sum "$@"; else shasum -a 256 "$@"; fi; }

STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$ROOT"
umask 077
WORK="$(mktemp -d "$ROOT/.ppm-backup-$STAMP.tmp.XXXXXX")"
OUT="$ROOT/ppm-backup-$STAMP-${WORK##*.}"
cleanup() {
  status=$?
  trap - EXIT HUP INT TERM
  if [ -n "$WORK" ] && [ -d "$WORK" ]; then
    echo "backup FAILED; removing private partial directory" >&2
    rm -rf "$WORK"
  fi
  exit "$status"
}
trap cleanup EXIT
trap 'exit 1' HUP INT TERM

echo "dumping the database ..."
# --no-owner/--no-privileges: restore works under a different role. -Fc: compressed, selective.
$PG_DUMP --format=custom --no-owner --no-privileges --dbname="$DB_URL" > "$WORK/db.dump"

ARTIFACT_NOTE="skipped (no ARTIFACT_DIR or ARTIFACT_TAR_CMD set)"
if [ -n "$ARTIFACT_TAR_CMD" ]; then
  echo "archiving the artifact store with ARTIFACT_TAR_CMD ..."
  sh -c "$ARTIFACT_TAR_CMD" > "$WORK/artifacts.tar.gz"
elif [ -n "$ARTIFACT_DIR" ]; then
  echo "archiving the artifact store $ARTIFACT_DIR ..."
  tar -czf "$WORK/artifacts.tar.gz" -C "$ARTIFACT_DIR" .
  ARTIFACT_NOTE="$(find "$ARTIFACT_DIR" -type f -print0 | LC_ALL=C tr -cd '\000' | wc -c | tr -d ' ') files from $ARTIFACT_DIR"
fi

if [ -f "$WORK/artifacts.tar.gz" ]; then
  # A tar reader can stop at the end-of-archive marker before checking the gzip footer.
  # Keep both exit statuses independent of the manifest's file count.
  if ! gzip -t "$WORK/artifacts.tar.gz" || ! tar -tzf "$WORK/artifacts.tar.gz" > "$WORK/artifact-members"; then
    echo "backup: cannot read the artifact archive as a complete gzip tar" >&2
    exit 1
  fi
  # GNU tar can report an empty listing for arbitrary input shorter than one header.
  # Even an empty tar has two 512-byte end-of-archive blocks.
  if [ ! -s "$WORK/artifact-members" ] && [ "$(gzip -dc "$WORK/artifacts.tar.gz" | wc -c)" -lt 1024 ]; then
    echo "backup: empty artifact archive is missing its end-of-archive blocks" >&2
    exit 1
  fi
  if [ -n "$ARTIFACT_TAR_CMD" ]; then
    count=$(awk '!/\/$/ { n++ } END { print n+0 }' "$WORK/artifact-members")
    ARTIFACT_NOTE="$count files via ARTIFACT_TAR_CMD"
  fi
  rm "$WORK/artifact-members"
fi

encrypt "$WORK/db.dump"
[ -f "$WORK/artifacts.tar.gz" ] && encrypt "$WORK/artifacts.tar.gz"

{
  echo "encryption=$ENCRYPTION"
  echo "created_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "created_epoch=$(date -u +%s)"
  echo "server_version=$($PSQL "$DB_URL" -Atc 'show server_version' 2>/dev/null || echo unknown)"
  # sqlx's history once the service has migrated with it; SeaORM's on a database it has not reached yet.
  if [ "$($PSQL "$DB_URL" -Atc "select to_regclass('_sqlx_migrations') is not null" 2>/dev/null)" = t ]; then
    echo "migrations=$($PSQL "$DB_URL" -Atc 'select count(*) from _sqlx_migrations where success' 2>/dev/null || echo unknown)"
    echo "latest_migration=$($PSQL "$DB_URL" -Atc "select version || '_' || replace(description, ' ', '_') from _sqlx_migrations where success order by version desc limit 1" 2>/dev/null || echo unknown)"
  else
    echo "migrations=$($PSQL "$DB_URL" -Atc 'select count(*) from seaql_migrations' 2>/dev/null || echo unknown)"
    echo "latest_migration=$($PSQL "$DB_URL" -Atc 'select version from seaql_migrations order by version desc limit 1' 2>/dev/null || echo unknown)"
  fi
  echo "artifacts=$ARTIFACT_NOTE"
  echo "search_index=not backed up (rebuild with: <service> task search_reindex)"
  echo "--- rows per table"
  $PSQL "$DB_URL" -Atc "select format('%I', relname) from pg_class c join pg_namespace n on n.oid=c.relnamespace where n.nspname='public' and c.relkind='r' order by 1" 2>/dev/null \
    | while read -r t; do
        printf '%s=%s\n' "$t" "$($PSQL "$DB_URL" -Atc "select count(*) from \"$t\"")"
      done
} > "$WORK/MANIFEST"

SUFFIX=""; [ "$ENCRYPTION" != none ] && SUFFIX=".gpg"
(
  cd "$WORK"
  set -- "db.dump$SUFFIX" MANIFEST
  [ ! -f "artifacts.tar.gz$SUFFIX" ] || set -- "$@" "artifacts.tar.gz$SUFFIX"
  sha256 "$@" > SHA256SUMS
)
mv "$WORK" "$OUT"
WORK=""
trap - EXIT
trap - HUP INT TERM
echo "backup complete: $OUT"
echo "$OUT"
