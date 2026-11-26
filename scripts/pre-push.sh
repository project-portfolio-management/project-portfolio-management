#!/usr/bin/env bash
# The pre-push gate (LC-4, spec/local-ci-cd.md): the fast checks CI's Docs, API and Matcher jobs run, before a push
# leaves this machine. Installed as .git/hooks/pre-push by scripts/install-pre-push.sh; also runnable by hand.
#
# Usage: scripts/pre-push.sh [commit]           (default: HEAD; as a hook, every commit being pushed is read from stdin)
#   PPM_PRE_PUSH_STAGES="docs rust ui"           the stages to run (default: all three)
#   PPM_SKIP_PRE_PUSH="why"                      skip the gate; the reason, commit and time are appended to
#                                                .git/pre-push-skips.log, so a skip is recorded, never silent
#
# Stages:
#   docs  in a detached worktree of the commit being pushed (so uncommitted and untracked files cannot hide or cause a
#         failure): the generated status block, OpenAPI bodies, llms files, both translation review exports, the
#         site's content copy, the offline link check, the locale README.md links and, when rumdl is installed, the
#         Markdown lint.
#   rust  in the working tree: service and matcher fmt and clippy (-D warnings), the service workspace's lib tests,
#         and, when cargo-deny is installed, its advisory check.
#   ui    in the working tree, when node_modules is installed: the UI's lint (prettier and eslint).
# What it does not run: the database suites, Playwright, coverage, licences and bans, the container builds (CI's slow
# jobs).
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"
stages="${PPM_PRE_PUSH_STAGES:-docs rust ui}"

# The commits to check: from the hook's stdin (local sha per pushed ref, deletions skipped), else the argument or HEAD.
commits=()
if [ ! -t 0 ] && [ -z "${1:-}" ]; then
  while read -r _local_ref local_sha _remote_ref _remote_sha; do
    [ -n "${local_sha:-}" ] || continue
    case "$local_sha" in *[!0]*) commits+=("$local_sha") ;; esac
  done
fi
[ "${#commits[@]}" -gt 0 ] || commits=("$(git rev-parse "${1:-HEAD}")")

if [ -n "${PPM_SKIP_PRE_PUSH:-}" ]; then
  log="$(git rev-parse --git-common-dir)/pre-push-skips.log"
  for c in "${commits[@]}"; do
    printf '%s\t%s\t%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$c" "$PPM_SKIP_PRE_PUSH" >> "$log"
  done
  echo "pre-push: skipped (${PPM_SKIP_PRE_PUSH}); recorded in $log" >&2
  exit 0
fi

failed=()
step() { # step <label> <command...>: run it, keep going, remember a failure
  local label="$1"; shift
  local out; out="$(mktemp "${TMPDIR:-/tmp}/ppm-pre-push-step.XXXXXX")"
  if "$@" > "$out" 2>&1; then
    echo "  ok    $label"
  else
    echo "  FAIL  $label"; sed 's/^/        /' "$out" | tail -20
    failed+=("$label")
  fi
  rm -f "$out"
}
has() { case " $stages " in *" $1 "*) return 0 ;; *) return 1 ;; esac; }

wt=""
cleanup() { [ -z "$wt" ] || git worktree remove --force "$wt" >/dev/null 2>&1 || true; }
trap cleanup EXIT

if has docs; then
  for c in "${commits[@]}"; do
    echo "pre-push: docs at ${c:0:8}"
    cleanup
    wt="$(mktemp -d "${TMPDIR:-/tmp}/ppm-pre-push.XXXXXX")"
    git worktree add --detach --quiet "$wt" "$c"
    step "generated status block" bash -c "cd '$wt' && bash scripts/generate-status.sh --check"
    step "OpenAPI bodies" bash -c "cd '$wt' && python3 scripts/generate-openapi-bodies.py --check"
    step "llms.txt and llms.json" bash -c "cd '$wt' && python3 scripts/generate-llms.py --check"
    step "translation review exports" bash -c "cd '$wt' && python3 scripts/export-translation-review.py --check"
    step "site translation review exports" bash -c "cd '$wt' && python3 scripts/export-site-translation-review.py --check"
    step "relative links" bash -c "cd '$wt' && python3 scripts/check-links.py ."
    # The site's rule (its Playwright test): a locale page's README.md is a link to its index.md, never a copy.
    step "locale README.md links" bash -c "cd '$wt' && ! find locales -name README.md -type f -exec sh -c 'test -e \"\$(dirname \"\$1\")/index.md\" && echo \"\$1 is a copy, not a link to index.md\"' _ {} \\; | grep ."
    if [ -d "$wt/project-portfolio-management.github.io" ]; then
      step "site content copy" bash -c "cd '$wt/project-portfolio-management.github.io' && node scripts/sync-content.mjs --check"
    fi
    if command -v rumdl >/dev/null 2>&1; then
      step "Markdown lint (rumdl)" bash -c "cd '$wt' && rumdl check ."
    fi
  done
fi

if has rust; then
  echo "pre-push: rust (working tree)"
  # The kache wrapper some machines set fails builds; these checks do not need it.
  export RUSTC_WRAPPER= CARGO_BUILD_RUSTC_WRAPPER=
  api=project-portfolio-management-service-with-rust
  matcher=project-portfolio-management-matcher-with-rust
  step "service fmt" bash -c "cd $api && cargo fmt --all --check"
  step "service clippy" bash -c "cd $api && cargo clippy --workspace --all-targets --all-features -- -D warnings"
  step "service lib tests" bash -c "cd $api && cargo test --workspace --lib"
  step "matcher fmt" bash -c "cd $matcher && cargo fmt --check"
  step "matcher clippy" bash -c "cd $matcher && cargo clippy --all-targets --all-features -- -D warnings"
  if cargo deny --version >/dev/null 2>&1; then
    step "service advisories (cargo deny)" bash -c "cd $api && cargo deny check advisories"
  fi
fi

if has ui; then
  ui=project-portfolio-management-ui-with-svelte
  if [ -d "$ui/node_modules" ]; then
    echo "pre-push: ui (working tree)"
    step "UI lint" bash -c "cd $ui && pnpm run lint"
  else
    echo "pre-push: ui skipped (no node_modules; run pnpm install in $ui)"
  fi
fi

if [ "${#failed[@]}" -gt 0 ]; then
  echo "pre-push: ${#failed[@]} check(s) failed: ${failed[*]}" >&2
  echo "pre-push: fix them, or push anyway with PPM_SKIP_PRE_PUSH=\"<why>\" (recorded in .git/pre-push-skips.log)" >&2
  exit 1
fi
echo "pre-push: all checks passed"
