#!/bin/sh
# Fetch the Lily Design System monorepo at the pinned commit into vendor/lily-design-system/
# (spec/mobile-apps.md MA-5). The mobile apps build against its lily-design-system-swift and
# lily-design-system-kotlin directories, which are not yet published as a Swift package or Maven artifact.
#
#   scripts/fetch-lily.sh           clone or update vendor/lily-design-system to LILY_COMMIT
#   scripts/fetch-lily.sh --check   exit 1 unless vendor/lily-design-system is at LILY_COMMIT
#
# To build against another checkout instead, set LILY_DESIGN_SYSTEM_DIR to its root; this script is then not needed.
set -eu

LILY_REPO=${LILY_REPO:-https://github.com/LilyDesignSystem/lily-design-system.git}
LILY_COMMIT=7e1ceca4e1492d638b066c59a4de3d94c639ee64

ROOT=$(cd "$(dirname "$0")/.." && pwd)
DEST="$ROOT/vendor/lily-design-system"

if [ "${1:-}" = "--check" ]; then
    have=$(git -C "$DEST" rev-parse HEAD 2>/dev/null || echo none)
    if [ "$have" != "$LILY_COMMIT" ]; then
        echo "vendor/lily-design-system is at $have, expected $LILY_COMMIT: run scripts/fetch-lily.sh" >&2
        exit 1
    fi
    exit 0
fi

if [ ! -d "$DEST/.git" ]; then
    mkdir -p "$ROOT/vendor"
    git init --quiet "$DEST"
    git -C "$DEST" remote add origin "$LILY_REPO"
fi
# A shallow fetch of the one commit: the monorepo's history is large.
git -C "$DEST" fetch --quiet --depth 1 origin "$LILY_COMMIT"
git -C "$DEST" checkout --quiet --detach "$LILY_COMMIT"
echo "vendor/lily-design-system at $LILY_COMMIT"
