#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

CONTRACT_PATH="${HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_PATH:-docs/architecture/HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_2026-06-15.md}"
READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615}"
HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"

require_file() {
  local path="$1"
  if [[ ! -s "$path" ]]; then
    printf 'Missing required file: %s\n' "$path" >&2
    exit 1
  fi
}

require_marker() {
  local path="$1"
  local marker="$2"
  if ! grep -Fq "$marker" "$path"; then
    printf 'Missing marker in %s: %s\n' "$path" "$marker" >&2
    exit 1
  fi
}

require_file "$CONTRACT_PATH"
require_file "$HANDOFF_PATH"

MARKERS=(
  'HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_READY:true'
  'HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_DATE:2026-06-15'
  'HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_GAP_ID:message_report_send'
  'HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_LIVE_WIRING:matrix_report_content_result_retry_loaded_source_fetch_modal_live;moderation_workflow_blocked'
  'HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation'
  'ReportContent Result'
  'Source Retention Result'
  'Moderation Queue Persistence Result'
  'Policy Lookup Result'
  'Reviewer Assignment Result'
  'Appeal Workflow Result'
  'Enforcement Workflow Result'
  'Workflow Result Taxonomy UI Packet'
  'queue_operation_id'
  'enforcement_result'
  'Retry, Cancel, And Audit Rules'
  'PositiveConfirmationModal'
)

for marker in "${MARKERS[@]}"; do
  require_marker "$CONTRACT_PATH" "$marker"
done

jq -e '
  (.items | length) == 12
  and (.items | map(select(
    .id == "message_report_send"
    and .status == "partial_live_backend_contract_remaining"
    and .ui_lane_state == "complete"
    and .next_owner_lane == "backend_contract"
    and .acceptance_state.current_ui_evidence == "machine_checked"
    and .acceptance_state.side_effect_boundary == "locked"
    and .acceptance_state.ui_contract == "ready"
    and .acceptance_state.backend_contract == "required"
    and .acceptance_state.live_wiring == "matrix_report_content_result_retry_loaded_source_fetch_modal_live;moderation_workflow_blocked"
    and (.required_backend_contracts | index("moderation queue persistence result") != null)
    and (.required_backend_contracts | index("policy lookup and reviewer assignment result") != null)
    and (.required_backend_contracts | index("evidence/source retention contract") != null)
    and (.required_backend_contracts | index("appeal/enforcement workflow result") != null)
    and (.required_backend_contracts | index("report retry/cancel/error taxonomy") != null)
    and (.required_backend_contracts | index("workflow result taxonomy packet") != null)
  )) | length) == 1
' "$HANDOFF_PATH" >/dev/null

jq -n \
  --arg status "ready" \
  --arg contract_path "$CONTRACT_PATH" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg backend_handoff_path "$HANDOFF_PATH" \
  '{
    product:"Hepta Native",
    gate:"message_report_backend_contract",
    status:$status,
    contract_path:$contract_path,
    readiness_dir:$readiness_dir,
    backend_handoff_path:$backend_handoff_path,
    gap_id:"message_report_send",
    verified_live_wiring:"matrix_report_content_result_retry_loaded_source_fetch_modal_live;moderation_workflow_blocked",
    required_contract_groups:[
      "ReportContent result",
      "source retention",
      "moderation queue persistence",
      "policy lookup",
      "reviewer assignment",
      "appeal workflow",
      "enforcement workflow",
      "retry cancel audit taxonomy",
      "workflow result taxonomy packet"
    ],
    side_effects:{
      matrix_login:false,
      gateway_call:false,
      provider_invoked:false,
      channel_delivery:false,
      external_mutation:false
    }
  }'
