#!/bin/sh
# Exercise backup publication without a PostgreSQL server. The command stubs
# make concurrent runs share one timestamp and let one fail during pg_dump.
set -eu

HERE="$(cd "$(dirname "$0")" && pwd)"
WORK="$(mktemp -d)"
cleanup() { rm -rf "$WORK"; }
trap cleanup EXIT HUP INT TERM

fail() { echo "BACKUP PUBLICATION TEST FAILED: $*" >&2; exit 1; }

mkdir -p "$WORK/bin" "$WORK/artifacts" "$WORK/backups/ppm-backup-20261118T120000Z-retained"
printf 'retained\n' > "$WORK/backups/ppm-backup-20261118T120000Z-retained/MANIFEST"
printf 'attachment\n' > "$WORK/artifacts/example"

cat > "$WORK/bin/date" <<'SH'
#!/bin/sh
case "$*" in
  '-u +%Y%m%dT%H%M%SZ') echo 20261119T120000Z ;;
  '-u +%Y-%m-%dT%H:%M:%SZ') echo 2026-11-19T12:00:00Z ;;
  '-u +%s') echo 1795089600 ;;
  *) exec /bin/date "$@" ;;
esac
SH

cat > "$WORK/bin/pg-dump" <<'SH'
#!/bin/sh
sleep 2
case "$*" in
  *fail*) exit 7 ;;
  *) printf 'mock database dump\n' ;;
esac
SH

cat > "$WORK/bin/psql" <<'SH'
#!/bin/sh
case "$*" in
  *show\ server_version*) echo 18.7 ;;
  *to_regclass*) echo f ;;
  *count\(\*\)\ from\ seaql_migrations*) echo 0 ;;
  *version\ from\ seaql_migrations*) : ;;
  *select\ format*) : ;;
  *) echo "unexpected psql call: $*" >&2; exit 9 ;;
esac
SH
chmod +x "$WORK/bin/date" "$WORK/bin/pg-dump" "$WORK/bin/psql"

run_backup() {
  DB_URL="$1" ARTIFACT_DIR="$WORK/artifacts" PG_DUMP="$WORK/bin/pg-dump" \
    PSQL="$WORK/bin/psql" PATH="$WORK/bin:$PATH" \
    "$HERE/backup.sh" -o "$WORK/backups"
}

# A configured local store is required input. Its absence is found before a
# partial backup can become visible, and the retained backup stays untouched.
if DB_URL=mock ARTIFACT_DIR="$WORK/missing" PG_DUMP="$WORK/bin/pg-dump" \
  PSQL="$WORK/bin/psql" PATH="$WORK/bin:$PATH" \
  "$HERE/backup.sh" -o "$WORK/backups" > "$WORK/missing.log" 2>&1; then
  fail "a missing configured artifact directory was accepted"
fi
grep -F 'configured artifact directory does not exist' "$WORK/missing.log" >/dev/null ||
  fail "the missing artifact error was not reported"
if grep -F 'dumping the database' "$WORK/missing.log" >/dev/null; then
  fail "the database dump started before the artifact directory was validated"
fi
[ "$(cat "$WORK/backups/ppm-backup-20261118T120000Z-retained/MANIFEST")" = retained ] ||
  fail "the retained backup changed after a refused run"

# All three begin in the same second. Until their complete archives are ready,
# monitoring can see only the retained backup.
run_backup mock-success-a > "$WORK/a.log" 2>&1 & a=$!
run_backup mock-success-b > "$WORK/b.log" 2>&1 & b=$!
run_backup mock-fail > "$WORK/fail.log" 2>&1 & bad=$!
sleep 1
[ "$(find "$WORK/backups" -mindepth 1 -maxdepth 1 -type d -name 'ppm-backup-*' | wc -l | tr -d ' ')" = 1 ] ||
  fail "an incomplete backup became visible"
for partial in "$WORK"/backups/.ppm-backup-*; do
  [ -d "$partial" ] || continue
  mode="$(stat -f '%Lp' "$partial" 2>/dev/null || stat -c '%a' "$partial")"
  [ "$mode" = 700 ] || fail "$partial has mode $mode instead of 700"
done

wait "$a" || fail "the first concurrent backup failed"
wait "$b" || fail "the second concurrent backup failed"
if wait "$bad"; then fail "the deliberately failing backup succeeded"; fi

first="$(tail -1 "$WORK/a.log")"
second="$(tail -1 "$WORK/b.log")"
[ "$first" != "$second" ] || fail "same-second backups shared one destination"
for backup in "$first" "$second"; do
  [ -f "$backup/db.dump" ] || fail "$backup has no database dump"
  [ -f "$backup/artifacts.tar.gz" ] || fail "$backup has no artifact archive"
  [ -f "$backup/MANIFEST" ] || fail "$backup has no manifest"
  [ -f "$backup/SHA256SUMS" ] || fail "$backup has no checksums"
done
[ -f "$WORK/backups/ppm-backup-20261118T120000Z-retained/MANIFEST" ] || fail "the retained backup was lost"
[ "$(find "$WORK/backups" -mindepth 1 -maxdepth 1 -type d -name 'ppm-backup-*' | wc -l | tr -d ' ')" = 3 ] ||
  fail "the two completed backups were not both published"
[ -z "$(find "$WORK/backups" -mindepth 1 -maxdepth 1 -type d -name '.ppm-backup-*' -print -quit)" ] ||
  fail "a private partial directory was left behind"
PATH="$WORK/bin:$PATH" "$HERE/check-backup-age.sh" "$WORK/backups" 1 >/dev/null ||
  fail "backup-age monitoring did not accept the newest published backup"

echo "BACKUP PUBLICATION TEST PASSED"
