#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

CONTRACT_PATH="${HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_PATH:-docs/architecture/HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_2026-06-15.md}"
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
  'HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_READY:true'
  'HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_DATE:2026-06-15'
  'HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_GAP_ID:room_settings'
  'HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_LIVE_WIRING:matrix_room_settings_power_member_server_refresh_name_topic_alias_avatar_history_joinrule_tombstone_retry_live;power_member_blocked'
  'HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation'
  'Room Identity And Refresh Result'
  'Existing Room-State Mutation Result'
  'Power-Level Mutation Result'
  'Member Moderation Result'
  'Permission Denial Contract'
  'Power/Member Result Taxonomy UI Packet'
  'operation_id_slot'
  'audit_redaction'
  'Retry And Stale-Room Rules'
  'PositiveConfirmationModal'
)

for marker in "${MARKERS[@]}"; do
  require_marker "$CONTRACT_PATH" "$marker"
done

jq -e '
  (.items | length) == 12
  and (.items | map(select(
    .id == "room_settings"
    and .status == "partial_live_backend_contract_remaining"
    and .ui_lane_state == "complete"
    and .next_owner_lane == "backend_contract"
    and .acceptance_state.current_ui_evidence == "machine_checked"
    and .acceptance_state.side_effect_boundary == "locked"
    and .acceptance_state.ui_contract == "ready"
    and .acceptance_state.backend_contract == "required"
    and .acceptance_state.live_wiring == "matrix_room_settings_power_member_server_refresh_name_topic_alias_avatar_history_joinrule_tombstone_retry_live;power_member_blocked"
    and (.required_backend_contracts | index("power-level mutation result") != null)
    and (.required_backend_contracts | index("member moderation result and permission denial") != null)
    and (.required_backend_contracts | index("source-hash, retry, and stale-room guard") != null)
  )) | length) == 1
' "$HANDOFF_PATH" >/dev/null

jq -n \
  --arg status "ready" \
  --arg contract_path "$CONTRACT_PATH" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg backend_handoff_path "$HANDOFF_PATH" \
  '{
    product:"Hepta Native",
    gate:"room_settings_backend_contract",
    status:$status,
    contract_path:$contract_path,
    readiness_dir:$readiness_dir,
    backend_handoff_path:$backend_handoff_path,
    gap_id:"room_settings",
    verified_live_wiring:"matrix_room_settings_power_member_server_refresh_name_topic_alias_avatar_history_joinrule_tombstone_retry_live;power_member_blocked",
    required_contract_groups:[
      "room identity and refresh result",
      "existing room-state mutation result",
      "power-level mutation result",
      "member moderation result",
      "power/member result taxonomy ui packet",
      "permission denial",
      "stale-room retry guard"
    ],
    side_effects:{
      matrix_login:false,
      gateway_call:false,
      provider_invoked:false,
      channel_delivery:false,
      external_mutation:false
    }
  }'
