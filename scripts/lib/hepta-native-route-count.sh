#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${HEPTA_REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)}"
NATIVE_GATEWAY_SOURCE="${HEPTA_NATIVE_GATEWAY_SOURCE:-$REPO_ROOT/codex-rs/cli/src/native_gateway.rs}"

route_count="$(
  awk '
    /^const CONTROL_UI_ROUTE_SPECS: &\[ControlUiRouteSpec\] = &\[$/ {
      in_route_registry = 1
      next
    }
    in_route_registry && /^\];$/ {
      print route_count
      exit
    }
    in_route_registry && /^[[:space:]]*ControlUiRouteSpec \{$/ {
      route_count += 1
    }
  ' "$NATIVE_GATEWAY_SOURCE"
)"

case "$route_count" in
  ''|*[!0-9]*)
    echo "failed to derive native gateway source command count from CONTROL_UI_ROUTE_SPECS in $NATIVE_GATEWAY_SOURCE" >&2
    exit 1
    ;;
esac

if (( route_count == 0 )); then
  echo "native gateway route registry is empty in $NATIVE_GATEWAY_SOURCE" >&2
  exit 1
fi

printf '%s\n' "$route_count"
