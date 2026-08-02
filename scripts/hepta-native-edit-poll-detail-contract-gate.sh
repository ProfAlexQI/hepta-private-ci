#!/usr/bin/env bash
set -euo pipefail

# Historical compatibility entry point.
#
# This gate described the retired Hepta Telegram fixture/cockpit generation. It
# asserted marker strings in hepta_telegram_base_contract.rs and in the former
# monolithic product-readiness gate, then treated fixture screenshot counts as
# current product readiness. Those sources and that readiness schema are no
# longer part of the Robrix-first product shell. The gate must therefore never
# promote the current source tree.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
cd "$ROOT_DIR"

REPORT_PATH="${HEPTA_NATIVE_EDIT_POLL_DETAIL_CONTRACT_REPORT:-}"
DESCRIBE_ONLY=0

case "${1:-}" in
  "") ;;
  --describe) DESCRIBE_ONLY=1 ;;
  --help|-h)
    cat <<'EOF'
usage: scripts/hepta-native-edit-poll-detail-contract-gate.sh [--describe]

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
    kind: "hepta-native-legacy-contract-gate",
    gate: "edit_poll_detail_backend_contract",
    status: "historical_only",
    historical_gate_ready: false,
    current_readiness_eligible: false,
    historical_contract: "docs/architecture/HEPTA_NATIVE_EDIT_POLL_DETAIL_BACKEND_CONTRACT_2026-06-15.md",
    retired_dependencies: [
      "apps/hepta-native/src/home/hepta_telegram_base_contract.rs",
      "legacy Telegram fixture/cockpit markers",
      "legacy readiness.json native/control screenshot-count schema"
    ],
    replacement: "scripts/hepta-ui-current-readiness.sh",
    source_binding: $source_binding,
    blockers: [
      "retired_fixture_contract_is_not_current_product_evidence",
      "live_edit_poll_detail_behavior_requires_new_current_source_contract_and_receipt"
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
