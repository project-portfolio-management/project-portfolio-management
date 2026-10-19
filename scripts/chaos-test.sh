#!/bin/sh
# Chaos test of the service against a real Postgres: see scripts/chaos-test.mjs for what it breaks and checks.
#
#   scripts/chaos-test.sh [--seconds 60] [--workers 8] [--seed N]
#
# Needs Node 22+, Rust, and Podman (or RUNTIME=docker). It starts the test database from compose.test.yaml on
# TEST_DB_PORT (default 5439, to stay clear of a database another project holds on 5432), migrates it, runs the
# chaos test against a freshly built service, and removes the containers. Throwaway data only.
set -eu
cd "$(dirname "$0")/.."
RUNTIME="${RUNTIME:-podman}"
TEST_DB_PORT="${TEST_DB_PORT:-5439}"
TEST_REDIS_PORT="${TEST_REDIS_PORT:-6380}"
SVC=project-portfolio-management-service-with-rust
export TEST_DB_PORT TEST_REDIS_PORT
export DATABASE_URL="postgres://loco:loco@127.0.0.1:${TEST_DB_PORT}/project_portfolio_management_test"

( cd "$SVC" && cargo build --quiet )
BIN="$PWD/$SVC/target/debug/project-portfolio-management"

$RUNTIME compose -f "$SVC/compose.test.yaml" up -d --wait postgres
trap '$RUNTIME compose -f "$SVC/compose.test.yaml" down -v >/dev/null 2>&1' EXIT

( cd "$SVC" && LOCO_ENV=container-dev "$BIN" db migrate >/dev/null )
node scripts/chaos-test.mjs --service "$BIN" --service-dir "$SVC" --database-url "$DATABASE_URL" --runtime "$RUNTIME" \
  --report "${CHAOS_REPORT:-chaos-report.json}" "$@"

