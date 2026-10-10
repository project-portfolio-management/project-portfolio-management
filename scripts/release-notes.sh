#!/usr/bin/env bash
# Print the CHANGELOG section for a release tag.
#
#   scripts/release-notes.sh v0.3.0 [changelog-file ...]
#
# Looks for a heading `## [0.3.0]` (Keep a Changelog style, optionally followed
# by " - date") in each changelog file given, and prints that section's body up
# to the next `## ` heading. With no files, searches the crate changelogs:
# the API first, then the matcher, then the UI. The leading "v" is optional.
# Exit 1 if no changelog has the heading or the section is empty.
set -euo pipefail

if [ "$#" -lt 1 ]; then
  echo "usage: $0 <tag> [changelog-file ...]" >&2
  exit 2
fi

version="${1#v}"
shift

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [ "$#" -eq 0 ]; then
  set -- \
    "$root/project-portfolio-management-api-with-rust/CHANGELOG.md" \
    "$root/project-portfolio-management-matcher-with-rust/CHANGELOG.md" \
    "$root/project-portfolio-management-ui-with-svelte/CHANGELOG.md"
fi

for file in "$@"; do
  [ -f "$file" ] || continue
  notes="$(awk -v v="$version" '
    BEGIN { found = 0 }
    /^## / {
      if (found) exit
      if (index($0, "## [" v "]") == 1) { found = 1; next }
    }
    found { print }
  ' "$file")"
  # Trim leading and trailing blank lines.
  notes="$(printf '%s\n' "$notes" | sed -e '/./,$!d' | sed -e ':a' -e '/^\n*$/{$d;N;ba' -e '}')"
  if [ -n "$notes" ]; then
    printf '%s\n' "$notes"
    exit 0
  fi
done

echo "release-notes: no non-empty '## [$version]' section found in: $*" >&2
exit 1
