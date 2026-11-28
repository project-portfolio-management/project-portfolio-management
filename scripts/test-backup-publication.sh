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
case "$*" in *concurrent*) sleep 2 ;; esac
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
run_backup mock-concurrent-a > "$WORK/a.log" 2>&1 & a=$!
run_backup mock-concurrent-b > "$WORK/b.log" 2>&1 & b=$!
run_backup mock-concurrent-fail > "$WORK/fail.log" 2>&1 & bad=$!
sleep 1
[ "$(find "$WORK/backups" -mindepth 1 -maxdepth 1 -type d -name 'ppm-backup-*' | wc -l | tr -d ' ')" = 1 ] ||
  fail "an incomplete backup became visible"
for partial in "$WORK"/backups/.ppm-backup-*; do
  [ -d "$partial" ] || continue
  mode="$(stat -c '%a' "$partial" 2>/dev/null || stat -f '%Lp' "$partial")"
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

# The monitor reads this published manifest with the same fixed clock as backup.sh.
age_root="$WORK/age"
age_backup="$age_root/ppm-backup-age-fixture"
mkdir -p "$age_backup"
check_age() {
  PATH="$WORK/bin:$PATH" "$HERE/check-backup-age.sh" "$age_root" "$1" > "$WORK/age.log" 2>&1
}
limit_epoch=$((1795089600 - 24 * 3600))
echo "created_epoch=$limit_epoch" > "$age_backup/MANIFEST"
check_age 24 || fail "a backup exactly 24 hours old was rejected"
echo "created_epoch=$((limit_epoch - 1))" > "$age_backup/MANIFEST"
if check_age 24; then fail "a backup one second over the 24-hour limit was accepted"; fi
grep -F 'TOO OLD' "$WORK/age.log" >/dev/null || fail "the overdue backup was not reported as stale"

for manifest in 'no_timestamp' 'created_epoch=' 'created_epoch=12x' 'created_epoch=-1' \
  'created_epoch=1795089601' 'created_epoch=18446744073709551616' \
  "$(printf 'created_epoch=1795089600\ncreated_epoch=1795089600')"; do
  printf '%s\n' "$manifest" > "$age_backup/MANIFEST"
  if check_age 24; then fail "an invalid backup timestamp was accepted: $manifest"; fi
  grep -F 'created_epoch' "$WORK/age.log" >/dev/null || fail "the timestamp error was not reported"
done

# A zero-hour limit accepts only the current second. Decimal fields can have leading zeros.
echo 'created_epoch=01795089600' > "$age_backup/MANIFEST"
check_age 0 || fail "a current backup was rejected with a zero-hour limit"
echo 'created_epoch=1795089599' > "$age_backup/MANIFEST"
if check_age 0; then fail "a one-second-old backup passed a zero-hour limit"; fi
if check_age 1.5; then fail "a fractional hour limit was accepted"; else
  [ "$?" -eq 2 ] || fail "an invalid hour limit must report a usage error"
fi

# A producer can exit successfully after writing an unusable archive.
export STREAM_FIXTURE="$WORK/stream.tar.gz"
# The backup producer expands this variable in its own shell.
# shellcheck disable=SC2016
export ARTIFACT_TAR_CMD='cat "$STREAM_FIXTURE"'
reject_stream() {
  if run_backup mock > "$WORK/stream.log" 2>&1; then
    fail "published $1"
  fi
  [ "$(find "$WORK/backups" -mindepth 1 -maxdepth 1 | wc -l | tr -d ' ')" = 3 ] ||
    fail "failed stream changed the set of backup directories"
  [ -f "$first/SHA256SUMS" ] && [ -f "$second/SHA256SUMS" ] &&
    [ "$(cat "$WORK/backups/ppm-backup-20261118T120000Z-retained/MANIFEST")" = retained ] ||
    fail "failed stream damaged a previously published backup"
}
printf 'invalid tar contents\n' | gzip -c > "$STREAM_FIXTURE"
reject_stream "a valid gzip stream containing invalid tar data"
gzip -c < /dev/null > "$STREAM_FIXTURE"
reject_stream "a gzip stream with no tar data"

# Cut just the gzip footer so that a reader can still see the tar members.
size=$(wc -c < "$first/artifacts.tar.gz")
dd if="$first/artifacts.tar.gz" of="$STREAM_FIXTURE" bs=1 count=$((size - 8)) 2>/dev/null
reject_stream "a truncated gzip stream"

tar -czf "$STREAM_FIXTURE" -T /dev/null
run_backup mock > "$WORK/empty.log"
empty=$(tail -1 "$WORK/empty.log")
grep -Fx 'artifacts=0 files via ARTIFACT_TAR_CMD' "$empty/MANIFEST" >/dev/null ||
  fail "empty archive did not publish with a zero file count"

cp "$first/artifacts.tar.gz" "$STREAM_FIXTURE"
run_backup mock > "$WORK/normal.log"
normal=$(tail -1 "$WORK/normal.log")
grep -Fx 'artifacts=1 files via ARTIFACT_TAR_CMD' "$normal/MANIFEST" >/dev/null ||
  fail "normal archive did not publish with the expected file count"
cmp "$STREAM_FIXTURE" "$normal/artifacts.tar.gz" || fail "published archive differs from the stream"

echo "BACKUP PUBLICATION TEST PASSED"
