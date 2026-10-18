#!/bin/sh
# Back up the database and the artifact store into one verified directory.
#
#   DATABASE_URL=postgres://user:pass@host/db scripts/backup.sh [-o BACKUP_ROOT]
#
# Writes BACKUP_ROOT/ppm-backup-<UTC timestamp>/ containing
#   db.dump             pg_dump custom format (restore with scripts/restore.sh)
#   artifacts.tar.gz    the artifact store (attachments, export files), if present
#   MANIFEST            when, what, how many migrations, row counts per table
#   SHA256SUMS          checksums of every file above (verified on restore)
#
# Environment:
#   DATABASE_URL        required. The database to dump.
#   ARTIFACT_DIR        the artifact store (default: $PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR;
#                       if neither is set the artifact step is skipped and the MANIFEST says so)
#   PG_DUMP, PSQL       commands to run (default pg_dump / psql). Override to run inside
#                       the database container of a compose deployment:
#                         PG_DUMP="podman compose exec -T postgres pg_dump"
#                         PSQL="podman compose exec -T postgres psql"
#                       (DATABASE_URL is then resolved inside that container).
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

: "${DATABASE_URL:?set DATABASE_URL to the database to back up}"
PG_DUMP="${PG_DUMP:-pg_dump}"
PSQL="${PSQL:-psql}"
ARTIFACT_DIR="${ARTIFACT_DIR:-${PROJECT_PORTFOLIO_MANAGEMENT_ARTIFACT_DIR:-}}"

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
OUT="$ROOT/ppm-backup-$STAMP"
mkdir -p "$OUT"
trap 'echo "backup FAILED; removing partial $OUT" >&2; rm -rf "$OUT"' EXIT

echo "dumping the database ..."
# --no-owner/--no-privileges: restore works under a different role. -Fc: compressed, selective.
$PG_DUMP --format=custom --no-owner --no-privileges --dbname="$DATABASE_URL" > "$OUT/db.dump"

ARTIFACT_TAR_CMD="${ARTIFACT_TAR_CMD:-}"
ARTIFACT_NOTE="skipped (no ARTIFACT_DIR or ARTIFACT_TAR_CMD set)"
if [ -n "$ARTIFACT_TAR_CMD" ]; then
  echo "archiving the artifact store with ARTIFACT_TAR_CMD ..."
  sh -c "$ARTIFACT_TAR_CMD" > "$OUT/artifacts.tar.gz"
  ARTIFACT_NOTE="$(tar -tzf "$OUT/artifacts.tar.gz" | grep -vc '/$' || true) files via ARTIFACT_TAR_CMD"
elif [ -n "$ARTIFACT_DIR" ] && [ -d "$ARTIFACT_DIR" ]; then
  echo "archiving the artifact store $ARTIFACT_DIR ..."
  tar -czf "$OUT/artifacts.tar.gz" -C "$ARTIFACT_DIR" .
  ARTIFACT_NOTE="$(find "$ARTIFACT_DIR" -type f | wc -l | tr -d ' ') files from $ARTIFACT_DIR"
elif [ -n "$ARTIFACT_DIR" ]; then
  ARTIFACT_NOTE="skipped ($ARTIFACT_DIR does not exist)"
fi

encrypt "$OUT/db.dump"
[ -f "$OUT/artifacts.tar.gz" ] && encrypt "$OUT/artifacts.tar.gz"

{
  echo "encryption=$ENCRYPTION"
  echo "created_utc=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "created_epoch=$(date -u +%s)"
  echo "server_version=$($PSQL "$DATABASE_URL" -Atc 'show server_version' 2>/dev/null || echo unknown)"
  echo "migrations=$($PSQL "$DATABASE_URL" -Atc 'select count(*) from seaql_migrations' 2>/dev/null || echo unknown)"
  echo "latest_migration=$($PSQL "$DATABASE_URL" -Atc 'select version from seaql_migrations order by version desc limit 1' 2>/dev/null || echo unknown)"
  echo "artifacts=$ARTIFACT_NOTE"
  echo "search_index=not backed up (rebuild with: <service> task search_reindex)"
  echo "--- rows per table"
  $PSQL "$DATABASE_URL" -Atc "select format('%I', relname) from pg_class c join pg_namespace n on n.oid=c.relnamespace where n.nspname='public' and c.relkind='r' order by 1" 2>/dev/null \
    | while read -r t; do
        printf '%s=%s\n' "$t" "$($PSQL "$DATABASE_URL" -Atc "select count(*) from \"$t\"")"
      done
} > "$OUT/MANIFEST"

SUFFIX=""; [ "$ENCRYPTION" != none ] && SUFFIX=".gpg"
( cd "$OUT" && sha256 "db.dump$SUFFIX" MANIFEST $( [ -f "artifacts.tar.gz$SUFFIX" ] && echo "artifacts.tar.gz$SUFFIX" ) > SHA256SUMS )
trap - EXIT
echo "backup complete: $OUT"
echo "$OUT"
