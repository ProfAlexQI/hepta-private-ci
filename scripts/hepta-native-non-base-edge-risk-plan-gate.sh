#!/usr/bin/env bash
set -euo pipefail

# Historical compatibility entry point.
#
# The 2026-06-15 non-base edge plan aggregated cockpit-era marker gates and the
# retired readiness.json screenshot-count schema. Its source markers were
# intentionally removed when Native returned to the Robrix-first product shell,
# so replaying it cannot establish readiness for the current source tree.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

REPORT_PATH="${HEPTA_NATIVE_NON_BASE_EDGE_RISK_PLAN_REPORT:-}"
DESCRIBE_ONLY=0

case "${1:-}" in
  "") ;;
  --describe) DESCRIBE_ONLY=1 ;;
  --help|-h)
    cat <<'EOF'
usage: scripts/hepta-native-non-base-edge-risk-plan-gate.sh [--describe]

This is a historical compatibility entry point. It cannot validate or promote
the current Robrix-first Native source. Use scripts/hepta-ui-current-readiness.sh
for current-source readiness.
EOF
    exit 0
    ;;
  *) echo "unknown argument: $1" >&2; exit 64 ;;
esac

source_binding='null'
if [[ -x scripts/hepta-ui-source-fingerprint ]] && command -v jq >/dev/null 2>&1; then
  source_binding="$(scripts/hepta-ui-source-fingerprint)"
fi

report="$(jq -n \
  --argjson source_binding "$source_binding" \
  '{
    schema_version: 1,
    kind: "hepta-native-legacy-risk-plan-gate",
    gate: "non_base_edge_risk_plan",
    status: "historical_only",
    historical_gate_ready: false,
    current_readiness_eligible: false,
    historical_plan: "docs/architecture/HEPTA_NATIVE_NON_BASE_EDGE_RISK_PLAN_2026-06-15.md",
    retired_dependencies: [
      "legacy Telegram fixture/cockpit marker contracts",
      "legacy product-readiness gate body literals",
      "legacy readiness.json native/control screenshot-count schema"
    ],
    replacement: "scripts/hepta-ui-current-readiness.sh",
    source_binding: $source_binding,
    blockers: [
      "historical_edge_plan_is_not_current_product_evidence",
      "remaining_edge_behaviors_require_current_source_bound_live_receipts"
    ],
    external_side_effects_performed: false
  }')"

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  printf '%s\n' "$report" >"$REPORT_PATH"
else
  printf '%s\n' "$report"
fi

if [[ "$DESCRIBE_ONLY" == "1" ]]; then exit 0; fi
echo "historical gate cannot promote current source; use scripts/hepta-ui-current-readiness.sh" >&2
exit 2
