#!/usr/bin/env bash
# Tests for the pre-push gate (LC-4): the installer refuses a foreign hook and installs and removes its own; a skip is
# recorded with its reason; a failing stage fails the gate; a deleted ref is not checked. Runs in a throwaway repository.
# Usage: bash scripts/test-pre-push.sh
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
tmp="$(mktemp -d "${TMPDIR:-/tmp}/ppm-pre-push-test.XXXXXX")"
trap 'rm -rf "$tmp"' EXIT
fail() { echo "FAIL: $*" >&2; exit 1; }

cd "$tmp"
git init --quiet repo && cd repo
git config user.email test@example.invalid && git config user.name test
mkdir scripts && cp "$here/pre-push.sh" "$here/install-pre-push.sh" scripts/
git add scripts && git commit --quiet -m init
hook="$(git rev-parse --git-path hooks)/pre-push"

# A hook someone else wrote is never replaced.
mkdir -p "$(dirname "$hook")" && printf '#!/bin/sh\necho theirs\n' > "$hook"
if scripts/install-pre-push.sh >/dev/null 2>&1; then fail "replaced a foreign hook"; fi
grep -q theirs "$hook" || fail "the foreign hook was changed"
rm "$hook"

# Ours installs, reinstalls over itself, and uninstalls.
scripts/install-pre-push.sh >/dev/null || fail "install"
[ -x "$hook" ] || fail "hook not executable"
grep -q 'scripts/pre-push.sh' "$hook" || fail "hook does not call the gate"
scripts/install-pre-push.sh >/dev/null || fail "reinstall over our own hook"
scripts/install-pre-push.sh --uninstall >/dev/null && [ ! -e "$hook" ] || fail "uninstall"

# A skip is recorded with the commit and the reason, and passes.
PPM_SKIP_PRE_PUSH="hotfix, checked by hand" scripts/pre-push.sh </dev/null 2>/dev/null || fail "a skip failed"
log="$(git rev-parse --git-common-dir)/pre-push-skips.log"
grep -q "$(git rev-parse HEAD).*hotfix, checked by hand" "$log" || fail "the skip was not recorded"

# A stage that fails fails the gate (this repository has none of the scripts the docs stage runs) and names it.
if out="$(PPM_PRE_PUSH_STAGES=docs scripts/pre-push.sh HEAD </dev/null 2>&1)"; then fail "a failing stage passed"; fi
echo "$out" | grep -q "FAIL  generated status block" || fail "the failing check is not named: $out"
[ "$(git worktree list | wc -l)" -eq 1 ] || fail "the worktree was left behind"

# As a hook: a deleted ref (all-zero local sha) is not checked; with nothing else, HEAD is.
zero=0000000000000000000000000000000000000000
out="$(printf 'refs/heads/gone %s refs/heads/gone %s\n' "$zero" "$(git rev-parse HEAD)" | PPM_PRE_PUSH_STAGES=none scripts/pre-push.sh 2>&1)" || fail "a deletion failed: $out"
echo "$out" | grep -q "all checks passed" || fail "unexpected: $out"
echo "pre-push gate: all tests passed"
