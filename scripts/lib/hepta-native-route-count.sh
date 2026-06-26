#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${HEPTA_REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)}"
NATIVE_GATEWAY_SOURCE="${HEPTA_NATIVE_GATEWAY_SOURCE:-$REPO_ROOT/codex-rs/cli/src/native_gateway.rs}"

route_count="$(
  sed -n 's/^const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = \([0-9][0-9]*\);$/\1/p' \
    "$NATIVE_GATEWAY_SOURCE" | head -n 1
)"

case "$route_count" in
  ''|*[!0-9]*)
    echo "failed to derive native gateway source command count from $NATIVE_GATEWAY_SOURCE" >&2
    exit 1
    ;;
esac

printf '%s\n' "$route_count"
