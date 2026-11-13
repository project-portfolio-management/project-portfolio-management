#!/bin/bash
# Lint Markdown files with rumdl
#
# Usage: bash scripts/lint-markdown.sh [check|fmt]
#   check - Lint and report issues (default)
#   fmt   - Format and fix issues

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

ACTION="${1:-check}"
VERBOSE="${2:-}"

cd "$REPO_ROOT"

case "$ACTION" in
    check)
        if [ "$VERBOSE" != "--quiet" ]; then
            echo "Linting Markdown files with rumdl..."
        fi
        if command -v rumdl &> /dev/null; then
            rumdl check .
        else
            echo "ERROR: rumdl is not installed."
            echo "Install it with: cargo install rumdl"
            exit 1
        fi
        ;;
    fmt)
        if [ "$VERBOSE" != "--quiet" ]; then
            echo "Formatting Markdown files with rumdl..."
        fi
        if command -v rumdl &> /dev/null; then
            rumdl fmt .
        else
            echo "ERROR: rumdl is not installed."
            echo "Install it with: cargo install rumdl"
            exit 1
        fi
        ;;
    *)
        echo "Usage: $0 [check|fmt] [--quiet]"
        echo "  check - Lint and report issues (default)"
        echo "  fmt   - Format and fix issues"
        echo "  --quiet - Suppress output messages"
        exit 1
        ;;
esac
