#!/bin/sh
# Fail (exit 1) when the newest backup under BACKUP_ROOT is older than MAX_HOURS,
# or when there is none. Wire this into monitoring so a silent backup failure
# cannot outlast the recovery point objective (operations/recovery-objectives.md).
#
#   scripts/check-backup-age.sh BACKUP_ROOT MAX_HOURS
set -eu
[ $# -eq 2 ] || { echo "usage: $0 BACKUP_ROOT MAX_HOURS" >&2; exit 2; }
ROOT="$1"; MAX_HOURS="$2"
newest="$(ls -1d "$ROOT"/ppm-backup-* 2>/dev/null | sort | tail -1 || true)"
[ -n "$newest" ] || { echo "check-backup-age: no backup under $ROOT" >&2; exit 1; }
[ -f "$newest/MANIFEST" ] || { echo "check-backup-age: $newest has no MANIFEST (incomplete)" >&2; exit 1; }
created="$(sed -n 's/^created_epoch=//p' "$newest/MANIFEST")"
now="$(date -u +%s)"
age_hours=$(( (now - created) / 3600 ))
echo "newest backup: $newest, ${age_hours}h old (limit ${MAX_HOURS}h)"
[ "$age_hours" -le "$MAX_HOURS" ] || { echo "check-backup-age: TOO OLD" >&2; exit 1; }
