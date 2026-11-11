#!/bin/sh
# Prove that a backup restores: back up SOURCE, restore it into a fresh scratch
# database, and compare the two. Exits non-zero on the first difference.
#
#   DB_URL=postgres://user:pass@host:5432/ppm scripts/restore-test.sh [-k] [-t "COMMAND"]
#
#   -k            keep the scratch database (its URL is printed) instead of dropping it
#   -t "COMMAND"  run COMMAND with RESTORED_DB_URL (and RESTORED_ARTIFACT_DIR)
#                 exported, before the scratch database is dropped. CI uses it to
#                 run the test suite against the restored database.
#
# Checks: archive checksums; the migration history is identical; every table has
# the same row count; every table has the same content digest (md5 over its rows
# as text, ordered); the restored artifact files match the archive's manifest.
#
# DB_URL must be a plain postgres://user:pass@host:port/dbname URL with no
# query string, and its role must be able to CREATE DATABASE.
set -eu

KEEP=0; AFTER=""
while getopts "kt:" opt; do
  case "$opt" in
    k) KEEP=1 ;;
    t) AFTER="$OPTARG" ;;
    *) echo "usage: $0 [-k] [-t COMMAND]" >&2; exit 2 ;;
  esac
done
[ -z "${DATABASE_URL:-}" ] || [ -n "${DB_URL:-}" ] || { echo "DATABASE_URL is no longer read (axa 0.11): set DB_URL" >&2; exit 2; }
: "${DB_URL:?set DB_URL to the source database}"
PSQL="${PSQL:-psql}"
HERE="$(cd "$(dirname "$0")" && pwd)"

base="${DB_URL%/*}"
scratch="ppm_restore_test_$(date -u +%Y%m%d%H%M%S)_$$"
SCRATCH_URL="$base/$scratch"
WORK="$(mktemp -d)"
cleanup() {
  if [ "$KEEP" = 1 ]; then echo "kept scratch database: $SCRATCH_URL"; else
    $PSQL "$base/postgres" -qc "drop database if exists \"$scratch\"" >/dev/null 2>&1 || true
  fi
  rm -rf "$WORK"
}
trap cleanup EXIT

fail() { echo "RESTORE TEST FAILED: $*" >&2; exit 1; }

echo "1/5 backing up the source ..."
out="$("$HERE/backup.sh" -o "$WORK/backups" | tail -1)"
[ -f "$out/db.dump" ] || [ -f "$out/db.dump.gpg" ] || fail "no db.dump produced"

echo "2/5 restoring into scratch database $scratch ..."
$PSQL "$base/postgres" -qc "create database \"$scratch\"" || fail "cannot create the scratch database"
"$HERE/restore.sh" "$out" "$SCRATCH_URL" -a "$WORK/artifacts" >/dev/null || fail "restore failed"

echo "3/5 comparing migration history ..."
# Both histories: sqlx's, and SeaORM's, which a database migrated before the move to sqlx keeps.
hist() {
  if [ "$($PSQL "$1" -Atc "select to_regclass('$2') is not null")" = t ]; then
    $PSQL "$1" -Atc "select coalesce(string_agg(version::text, ',' order by version), '') from $2"
  else
    echo "(no $2)"
  fi
}
mig() { echo "sqlx: $(hist "$1" _sqlx_migrations); seaorm: $(hist "$1" seaql_migrations)"; }
[ "$(mig "$DB_URL")" != "sqlx: (no _sqlx_migrations); seaorm: (no seaql_migrations)" ] || fail "the source has no migration history"
[ "$(mig "$DB_URL")" = "$(mig "$SCRATCH_URL")" ] || fail "migration history differs"

echo "4/5 comparing every table (rows and content digest) ..."
tables="$($PSQL "$DB_URL" -Atc "select relname from pg_class c join pg_namespace n on n.oid=c.relnamespace where n.nspname='public' and c.relkind='r' order by 1")"
[ -n "$tables" ] || fail "the source has no tables"
digest() {
  $PSQL "$1" -Atc "select count(*) || ':' || coalesce(md5(string_agg(x::text, E'\\n' order by x::text)), '') from \"$2\" x"
}
n=0
for t in $tables; do
  a="$(digest "$DB_URL" "$t")"; b="$(digest "$SCRATCH_URL" "$t")"
  [ "$a" = "$b" ] || fail "table $t differs (source $a, restored $b)"
  n=$((n+1))
done
echo "    $n tables identical"
rows="$($PSQL "$SCRATCH_URL" -Atc "select coalesce(sum(n_live_tup),0) from pg_stat_user_tables")"
echo "    (restored database reports about $rows live rows)"

echo "5/5 checking the artifact store ..."
if [ -f "$out/artifacts.tar.gz" ]; then
  want="$(sed -n 's/^artifacts=\([0-9]*\) files.*/\1/p' "$out/MANIFEST")"
  have="$(find "$WORK/artifacts" -type f | wc -l | tr -d ' ')"
  [ -z "$want" ] || [ "$want" = "$have" ] || fail "artifact files: archived $want, restored $have"
  echo "    $have artifact files restored"
else
  echo "    no artifact archive in this backup"
fi

if [ -n "$AFTER" ]; then
  echo "running: $AFTER"
  RESTORED_DB_URL="$SCRATCH_URL" RESTORED_ARTIFACT_DIR="$WORK/artifacts" sh -c "$AFTER" || fail "the command run against the restored database failed"
fi
echo "RESTORE TEST PASSED"
