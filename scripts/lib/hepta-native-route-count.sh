#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${HEPTA_REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)}"
ROUTE_REGISTRY_SOURCE="${HEPTA_ROUTE_REGISTRY_SOURCE:-${HEPTA_NATIVE_GATEWAY_SOURCE:-$REPO_ROOT/codex-rs/hepta-native-gateway/src/route_registry.rs}}"

route_count="$(
  awk '
    /^(pub\(crate\) )?const CONTROL_UI_ROUTE_SPECS: &\[ControlUiRouteSpec\] = &\[$/ {
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
  ' "$ROUTE_REGISTRY_SOURCE"
)"

case "$route_count" in
  ''|*[!0-9]*)
    echo "failed to derive native gateway source command count from CONTROL_UI_ROUTE_SPECS in $ROUTE_REGISTRY_SOURCE" >&2
    exit 1
    ;;
esac

if (( route_count == 0 )); then
  echo "native gateway route registry is empty in $ROUTE_REGISTRY_SOURCE" >&2
  exit 1
fi

printf '%s\n' "$route_count"
