#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

NATIVE_GATEWAY_SOURCE="codex-rs/hepta-native-gateway/src/native_gateway.rs"
ROUTE_COUNT_HELPER="scripts/lib/hepta-native-route-count.sh"

grep -Fq \
  'const NATIVE_GATEWAY_SOURCE_COMMAND_COUNT: usize = CONTROL_UI_ROUTE_SPECS.len();' \
  "$NATIVE_GATEWAY_SOURCE" \
  || {
    echo "native gateway source command count must be derived from CONTROL_UI_ROUTE_SPECS" >&2
    exit 1
  }

grep -Fq 'const CONTROL_UI_ROUTE_SPECS:' "$ROUTE_COUNT_HELPER" \
  || {
    echo "route count helper must derive its value from CONTROL_UI_ROUTE_SPECS" >&2
    exit 1
  }

derived_route_count="$(bash "$ROUTE_COUNT_HELPER")"
case "$derived_route_count" in
  ''|*[!0-9]*)
    echo "route count helper did not return a positive integer" >&2
    exit 1
    ;;
esac
(( derived_route_count > 0 )) \
  || {
    echo "route count helper returned an empty registry" >&2
    exit 1
  }

violations_file="$(mktemp "${TMPDIR:-/tmp}/hepta-route-gate-dynamic-count.XXXXXX")"
trap 'rm -f "$violations_file"' EXIT

while IFS= read -r -d '' gate; do
  perl -ne '
    if (/NATIVE_GATEWAY_SOURCE_COMMAND_COUNT:\s+usize\s+=\s+[1-9][0-9]*\s*;/) {
      print "$ARGV:$.:static_native_gateway_source_command_count:$&\n";
    }
    if (/(?<![A-Za-z0-9_])\.route_count\s*==\s*[1-9][0-9]*/) {
      print "$ARGV:$.:static_route_count:$&\n";
    }
    if (/(?<![A-Za-z0-9_])\.(?:present_)?required_marker_count\s*==\s*[1-9][0-9]*/) {
      print "$ARGV:$.:static_terminal_marker_count:$&\n";
    }
  ' "$gate" >>"$violations_file"
done < <(
  find scripts -type f \( -name '*route-gate.sh' -o -name '*lane-gate.sh' \) -print0
)

violation_count="$(wc -l <"$violations_file" | tr -d '[:space:]')"
if [[ "$violation_count" != "0" ]]; then
  echo "route/lane gates must derive route and terminal counts dynamically; found static-count regressions:" >&2
  sed -n '1,120p' "$violations_file" >&2
  exit 1
fi

jq -n \
  --arg status "ready" \
  --arg gate "hepta_route_gate_dynamic_count_regression_gate" \
  --argjson derived_route_count "$derived_route_count" \
  --argjson scanned_gate_count "$(find scripts -type f \( -name '*route-gate.sh' -o -name '*lane-gate.sh' \) | wc -l | tr -d '[:space:]')" \
  '{
    status:$status,
    gate:$gate,
    scanned_gate_count:$scanned_gate_count,
    static_native_gateway_source_command_count_regression:false,
    static_route_count_regression:false,
    static_terminal_marker_count_regression:false,
    route_count_source:"CONTROL_UI_ROUTE_SPECS",
    derived_route_count:$derived_route_count,
    dynamic_count_contract_ready:true
  }'
