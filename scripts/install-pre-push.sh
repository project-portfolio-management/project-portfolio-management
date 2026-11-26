#!/usr/bin/env bash
# Install scripts/pre-push.sh as this clone's pre-push hook (LC-4). Refuses to replace a different hook already there.
# Usage: scripts/install-pre-push.sh [--uninstall]
set -euo pipefail
root="$(git rev-parse --show-toplevel)"
hooks="$(git rev-parse --git-path hooks)"
hook="$hooks/pre-push"
marker="# installed by scripts/install-pre-push.sh"
if [ "${1:-}" = "--uninstall" ]; then
  if [ -f "$hook" ] && grep -qF "$marker" "$hook"; then rm "$hook"; echo "removed $hook"; else echo "no hook of ours at $hook"; fi
  exit 0
fi
if [ -e "$hook" ] && ! grep -qF "$marker" "$hook"; then
  echo "refusing: $hook exists and was not installed by this script; move it aside first" >&2
  exit 1
fi
mkdir -p "$hooks"
cat > "$hook" <<EOF
#!/usr/bin/env bash
$marker
exec "$root/scripts/pre-push.sh" "\$@"
EOF
chmod +x "$hook"
echo "installed $hook (skip one push with PPM_SKIP_PRE_PUSH=\"<why>\"; remove with scripts/install-pre-push.sh --uninstall)"
