#!/usr/bin/env bash
set -euo pipefail

# Historical entrypoint retained so old automation fails with a machine-readable
# migration receipt instead of interpreting the new package schema as the old
# backend/fixture contract. Use hepta-native-current-package-gate.sh directly.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

describe=0
[[ "${1:-}" != "--describe" ]] || describe=1
[[ $# -le "$describe" ]] || { echo "usage: $0 [--describe]" >&2; exit 64; }
binding="$(scripts/hepta-ui-source-fingerprint)"
report="$(jq -n --argjson binding "$binding" '{
  schema_version:2,
  kind:"hepta-native-packaging-gate-historical-entrypoint",
  status:"historical_only",
  source_binding:$binding,
  current_readiness_eligible:false,
  replacement:"scripts/hepta-native-current-package-gate.sh",
  local_package_ready:false,
  public_ga_ready:false,
  blockers:["legacy_packaging_schema_retired","formal_unsigned_packaging_pipeline_not_implemented"]
}')"
report_path="${HEPTA_NATIVE_PACKAGING_GATE_REPORT_PATH:-}"
[[ -z "$report_path" ]] || { mkdir -p "$(dirname "$report_path")"; printf '%s\n' "$report" >"$report_path"; }
printf '%s\n' "$report"
[[ "$describe" == "1" ]] && exit 0
exit 2
