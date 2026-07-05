#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT"

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
  --argjson scanned_gate_count "$(find scripts -type f \( -name '*route-gate.sh' -o -name '*lane-gate.sh' \) | wc -l | tr -d '[:space:]')" \
  '{
    status:$status,
    gate:$gate,
    scanned_gate_count:$scanned_gate_count,
    static_native_gateway_source_command_count_regression:false,
    static_route_count_regression:false,
    static_terminal_marker_count_regression:false,
    dynamic_count_contract_ready:true
  }'
