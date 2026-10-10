#!/bin/sh
# Restore a backup made by scripts/backup.sh into a database (and artifact directory).
#
#   scripts/restore.sh BACKUP_DIR TARGET_DATABASE_URL [-a ARTIFACT_DIR] [-f]
#
# Verifies SHA256SUMS first and refuses to continue on any mismatch. Refuses to
# restore into a database that already has tables unless -f is given (-f drops
# the existing objects it replaces: pg_restore --clean --if-exists). The target
# database must exist (createdb), and the restoring role needs to create tables.
#
# Environment: PG_RESTORE, PSQL override the commands (see scripts/backup.sh).
# ARTIFACT_UNTAR_CMD reads the gzip tar on stdin and unpacks it where the service
# expects it (compose: "docker compose exec -T api tar -C /app/data/artifacts -xz");
# it replaces -a. With the compose overrides pg_restore must read the dump from stdin
# (PG_RESTORE="docker compose exec -T postgres pg_restore" and RESTORE_FROM_STDIN=1).
# An encrypted backup (db.dump.gpg, see scripts/backup.sh) is decrypted into a private temporary
# directory that is removed on exit: set BACKUP_PASSPHRASE_FILE for a passphrase backup; a
# public-key backup needs the private key in the gpg keyring. A wrong key fails before the database
# is touched.
# Stop the service before restoring over a live database.
set -eu

[ $# -ge 2 ] || { echo "usage: $0 BACKUP_DIR TARGET_DATABASE_URL [-a ARTIFACT_DIR] [-f]" >&2; exit 2; }
DIR="$1"; TARGET="$2"; shift 2
ARTIFACTS_OUT=""; FORCE=0
while getopts "a:f" opt; do
  case "$opt" in
    a) ARTIFACTS_OUT="$OPTARG" ;;
    f) FORCE=1 ;;
    *) exit 2 ;;
  esac
done
PG_RESTORE="${PG_RESTORE:-pg_restore}"
PSQL="${PSQL:-psql}"

sha256check() { if command -v sha256sum >/dev/null 2>&1; then sha256sum -c "$@"; else shasum -a 256 -c "$@"; fi; }

[ -f "$DIR/SHA256SUMS" ] || { echo "restore: $DIR has no SHA256SUMS; not a backup" >&2; exit 1; }
echo "verifying checksums ..."
( cd "$DIR" && sha256check SHA256SUMS ) >/dev/null || { echo "restore: CHECKSUM MISMATCH in $DIR; refusing" >&2; exit 1; }

if [ -f "$DIR/db.dump.gpg" ]; then
  command -v gpg >/dev/null 2>&1 || { echo "restore: the backup is encrypted and gpg is not installed" >&2; exit 1; }
  PLAIN="$(mktemp -d)"
  trap 'rm -rf "$PLAIN"' EXIT
  decrypt() {
    if [ -n "${BACKUP_PASSPHRASE_FILE:-}" ]; then
      gpg --batch --yes --quiet --pinentry-mode loopback --passphrase-file "$BACKUP_PASSPHRASE_FILE" -o "$2" --decrypt "$1"
    else
      gpg --batch --yes --quiet -o "$2" --decrypt "$1"
    fi
  }
  echo "decrypting ..."
  decrypt "$DIR/db.dump.gpg" "$PLAIN/db.dump" 2>/dev/null \
    || { echo "restore: cannot decrypt the backup (wrong or missing key or passphrase)" >&2; exit 1; }
  if [ -f "$DIR/artifacts.tar.gz.gpg" ]; then
    decrypt "$DIR/artifacts.tar.gz.gpg" "$PLAIN/artifacts.tar.gz" 2>/dev/null \
      || { echo "restore: cannot decrypt the artifact archive" >&2; exit 1; }
  fi
  DIR_DATA="$PLAIN"
else
  DIR_DATA="$DIR"
fi

existing="$($PSQL "$TARGET" -Atc "select count(*) from pg_tables where schemaname='public'")"
if [ "$existing" != "0" ] && [ "$FORCE" != 1 ]; then
  echo "restore: target database already has $existing tables; use -f to replace them" >&2
  exit 1
fi

echo "restoring the database ..."
# Single transaction so a failure leaves the target as it was.
if [ "${RESTORE_FROM_STDIN:-0}" = 1 ]; then
  $PG_RESTORE --no-owner --no-privileges --clean --if-exists --single-transaction --dbname="$TARGET" < "$DIR_DATA/db.dump"
else
  $PG_RESTORE --no-owner --no-privileges --clean --if-exists --single-transaction --dbname="$TARGET" "$DIR_DATA/db.dump"
fi

if [ -f "$DIR_DATA/artifacts.tar.gz" ]; then
  if [ -n "${ARTIFACT_UNTAR_CMD:-}" ]; then
    echo "restoring the artifact store with ARTIFACT_UNTAR_CMD ..."
    sh -c "$ARTIFACT_UNTAR_CMD" < "$DIR_DATA/artifacts.tar.gz"
  elif [ -n "$ARTIFACTS_OUT" ]; then
    echo "restoring the artifact store into $ARTIFACTS_OUT ..."
    mkdir -p "$ARTIFACTS_OUT"
    tar -xzf "$DIR_DATA/artifacts.tar.gz" -C "$ARTIFACTS_OUT"
  else
    echo "note: the backup holds an artifact archive; pass -a ARTIFACT_DIR to restore it" >&2
  fi
fi
echo "restore complete"
