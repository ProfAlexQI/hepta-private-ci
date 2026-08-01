#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${HEPTA_REPO_ROOT:-$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)}"
ROUTE_CATALOG="${HEPTA_NATIVE_ROUTE_CATALOG:-$REPO_ROOT/codex-rs/hepta-native-gateway/routes/control_ui_route_catalog_v1.jsonl}"

route_count="$(
  jq -s -er '
    (.[0] | select(.kind == "metadata" and .schema == "hepta_native_route_catalog_v2")) as $metadata
    | ([.[] | select(.kind == "control_ui_route")] | length) as $routes
    | select($metadata.route_count == $routes and $routes > 0)
    | $routes
  ' "$ROUTE_CATALOG"
)"

case "$route_count" in
  ''|*[!0-9]*)
    echo "failed to derive native gateway source command count from $ROUTE_CATALOG" >&2
    exit 1
    ;;
esac

if (( route_count == 0 )); then
  echo "native gateway route catalog is empty in $ROUTE_CATALOG" >&2
  exit 1
fi

printf '%s\n' "$route_count"
