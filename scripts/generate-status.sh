#!/bin/sh
# Regenerate the GENERATED block in spec/14-implementation-status.md.
#
#   scripts/generate-status.sh           rewrite the block in place
#   scripts/generate-status.sh --check   regenerate to a temp file, diff against
#                                        the committed file, exit 1 (printing the
#                                        diff) if they differ
#
# Everything is derived from the source tree with grep/find/awk: no network, no
# database, no compilation. Output is deterministic (sorted, no timestamps), so
# running it twice changes nothing. Works with POSIX sh, BSD (macOS) and GNU tools.
set -eu
LC_ALL=C
export LC_ALL

ROOT=$(cd "$(dirname "$0")/.." && pwd)
STATUS="$ROOT/spec/14-implementation-status.md"
API="$ROOT/project-portfolio-management-service-with-rust"
MATCHER="$ROOT/project-portfolio-management-matcher-with-rust"
UI="$ROOT/project-portfolio-management-ui-with-svelte"
TASKS="$ROOT/tasks.md"
START='<!-- generated:start -->'
END='<!-- generated:end -->'

# count_in <ERE> <dir>... : matching lines across every *.rs under the dirs (target/ excluded)
count_rs() {
  pat=$1; shift
  find "$@" -name '*.rs' -not -path '*/target/*' -print0 | xargs -0 cat | grep -cE "$pat" || true
}
# count_ts <ERE> <dir>... : matching lines across *.ts under the dirs
count_ts() {
  pat=$1; shift
  find "$@" -name '*.ts' -not -path '*/node_modules/*' -not -path '*/.svelte-kit/*' -print0 | xargs -0 cat | grep -cE "$pat" || true
}
task_count() { # <prefix> <x| >
  grep -cE "^- \\[$2\\] \\*\\*$1-[0-9]" "$TASKS" || true
}
cargo_version() { sed -n 's/^version *= *"\([^"]*\)".*/\1/p' "$1" | sed -n 1p; }

generate() {
  migrations=$(find "$API/migration/src" -name 'm2*.rs' | wc -l | tr -d ' ')
  mounted=$(find "$API/src/controllers" -name '*.rs' -print0 | xargs -0 cat | grep -cE '\.add\(' || true)
  documented=$(cat "$API/src/openapi.rs" "$API/src/openapi_more.rs" | grep -cE '^[[:space:]]*"/api/[^"]*"[[:space:]]*:' || true)
  svc_tests=$(count_rs '^[[:space:]]*#\[(tokio::)?test' "$API/src" "$API/tests" "$API/migration/src")
  svc_ignored=$(count_rs '^[[:space:]]*#\[ignore' "$API/src" "$API/tests" "$API/migration/src")
  mat_tests=$(count_rs '^[[:space:]]*#\[(tokio::)?test' "$MATCHER/src" "$MATCHER/tests")
  mat_ignored=$(count_rs '^[[:space:]]*#\[ignore' "$MATCHER/src" "$MATCHER/tests")
  vitest=$(count_ts '^[[:space:]]*(it|test)(\.each)?[(`]' "$UI/tests/unit")
  playwright=$(count_ts '^[[:space:]]*test(\.each)?\(' "$UI/tests/e2e")
  locales=$(awk '/^export const LOCALES = \[/{f=1;next} f&&/\] as const/{exit} f&&/"/{n++} END{print n+0}' "$UI/src/lib/locales.ts")
  ui_version=$(sed -n 's/^  "version": *"\([^"]*\)".*/\1/p' "$UI/package.json" | sed -n 1p)

  echo "$START"
  echo "<!-- This block is written by scripts/generate-status.sh; do not edit it by hand. -->"
  echo
  echo "**Generated facts** (counted from the source tree by \`scripts/generate-status.sh\`;"
  echo "no network, database or compilation; counts are source lines, not test runs):"
  echo
  echo "| Fact | Value | Method |"
  echo "|---|---|---|"
  echo "| Service version | $(cargo_version "$API/Cargo.toml") | \`version\` in the service \`Cargo.toml\` |"
  echo "| Matcher version | $(cargo_version "$MATCHER/Cargo.toml") | \`version\` in the matcher \`Cargo.toml\` |"
  echo "| Front-end version | $ui_version | \`version\` in the front-end \`package.json\` |"
  echo "| Migrations | $migrations | files named \`m2*.rs\` in \`migration/src\` |"
  echo "| Mounted route registrations | $mounted | \`.add(\` calls in \`src/controllers/*.rs\` (one per registration; a path registered for several methods counts once per method, so this exceeds the documented-path count) |"
  echo "| Documented API paths | $documented | quoted \`\"/api/...\"\` keys in \`src/openapi.rs\` and \`src/openapi_more.rs\` |"
  echo "| Service tests | $svc_tests, plus $svc_ignored \`#[ignore]\`d (database or container gated) | \`#[test]\` and \`#[tokio::test]\` lines; \`#[ignore]\` lines, in \`src\`, \`tests\`, \`migration/src\` |"
  echo "| Matcher tests | $mat_tests, plus $mat_ignored \`#[ignore]\`d | the same patterns in the matcher \`src\` and \`tests\` |"
  echo "| Front-end unit tests (vitest) | $vitest | \`it(\` / \`test(\` lines in \`tests/unit\` |"
  echo "| Front-end end-to-end tests (Playwright) | $playwright | \`test(\` lines in \`tests/e2e\` |"
  echo "| Locales | $locales | entries of \`LOCALES\` in \`src/lib/locales.ts\` |"
  echo
  echo "**Task boxes in [\`tasks.md\`](../tasks.md)** (ticked / open, by prefix):"
  echo
  echo "| Prefix | Ticked | Open |"
  echo "|---|---|---|"
  tt=0; to=0
  for p in LT RM PL SC PJ KO MS CR DP EX HD CC; do
    t=$(task_count "$p" x); o=$(task_count "$p" ' ')
    tt=$((tt + t)); to=$((to + o))
    echo "| $p | $t | $o |"
  done
  echo "| **Total** | **$tt** | **$to** |"
  echo
  echo "$END"
}

splice() { # <status file> <block file> -> stdout
  awk -v s="$START" -v e="$END" -v blk="$2" '
    $0==s { while ((getline line < blk) > 0) print line; skip=1; next }
    $0==e { skip=0; next }
    !skip { print }' "$1"
}

grep -qF "$START" "$STATUS" && grep -qF "$END" "$STATUS" || { echo "markers missing in $STATUS" >&2; exit 2; }
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
generate > "$tmp/block"
splice "$STATUS" "$tmp/block" > "$tmp/new"

case "${1:-}" in
  --check)
    if diff -u "$STATUS" "$tmp/new"; then echo "status block is up to date"; else
      echo "spec/14-implementation-status.md is stale: run scripts/generate-status.sh" >&2; exit 1; fi ;;
  "") cat "$tmp/new" > "$STATUS" ;;
  *) echo "usage: $0 [--check]" >&2; exit 2 ;;
esac
