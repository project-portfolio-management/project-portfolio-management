#!/bin/sh
# Fail (exit 1) when the newest backup under BACKUP_ROOT is older than MAX_HOURS,
# or when its age cannot be established. Wire this into monitoring so a silent backup failure
# cannot outlast the recovery point objective (operations/recovery-objectives.md).
#
#   scripts/check-backup-age.sh BACKUP_ROOT MAX_HOURS
set -eu
[ $# -eq 2 ] || { echo "usage: $0 BACKUP_ROOT MAX_HOURS" >&2; exit 2; }
ROOT="$1"; MAX_HOURS="$2"
case "$MAX_HOURS" in
  ''|*[!0-9]*) echo "check-backup-age: MAX_HOURS must be a non-negative whole number" >&2; exit 2 ;;
esac
# Published directory names start with a sortable UTC timestamp.
export LC_ALL=C
newest=""
for candidate in "$ROOT"/ppm-backup-*; do
  [ -d "$candidate" ] && newest="$candidate"
done
[ -n "$newest" ] || { echo "check-backup-age: no backup under $ROOT" >&2; exit 1; }
[ -f "$newest/MANIFEST" ] || { echo "check-backup-age: $newest has no MANIFEST (incomplete)" >&2; exit 1; }
now="$(date -u +%s)"
backup_dir="$newest" awk -v now="$now" -v hours="$MAX_HOURS" '
  /^created_epoch=/ {
    entries++
    created = substr($0, length("created_epoch=") + 1)
  }
  END {
    backup = ENVIRON["backup_dir"]
    if (entries != 1 || created !~ /^[0-9]+$/ || created + 0 > now + 0) {
      print "check-backup-age: invalid or missing created_epoch in " backup "/MANIFEST" > "/dev/stderr"
      exit 1
    }
    age = now - created
    printf "newest backup: %s, %dh %02dm %02ds old (limit %sh)\n", backup, int(age / 3600), int(age / 60) % 60, age % 60, hours
    if (age > hours * 3600) {
      print "check-backup-age: TOO OLD" > "/dev/stderr"
      exit 1
    }
  }
' < "$newest/MANIFEST"
