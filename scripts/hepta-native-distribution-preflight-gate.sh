#!/usr/bin/env bash
set -euo pipefail

# Historical entrypoint. The old static workflow-marker schema is retired and
# must not promote release readiness. Current source/package truth lives in the
# current package gate and current readiness orchestrator.
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

describe=0
[[ "${1:-}" != "--describe" ]] || describe=1
[[ $# -le "$describe" ]] || { echo "usage: $0 [--describe]" >&2; exit 64; }
binding="$(scripts/hepta-ui-source-fingerprint)"
report="$(jq -n --argjson binding "$binding" '{
  schema_version:2,
  kind:"hepta-native-distribution-preflight-historical-entrypoint",
  status:"historical_only",
  source_binding:$binding,
  current_readiness_eligible:false,
  replacement:"scripts/hepta-ui-current-readiness.sh",
  distribution_preflight_gate_ready:false,
  public_distribution_ready:false,
  public_ga_ready:false,
  signed:false,
  notarized:false,
  stapled:false,
  blockers:["legacy_distribution_marker_schema_retired","independent_release_verifier_not_implemented"]
}')"
report_path="${HEPTA_NATIVE_DISTRIBUTION_PREFLIGHT_REPORT_PATH:-}"
[[ -z "$report_path" ]] || { mkdir -p "$(dirname "$report_path")"; printf '%s\n' "$report" >"$report_path"; }
printf '%s\n' "$report"
[[ "$describe" == "1" ]] && exit 0
exit 2
