#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

CONTRACT_PATH="${HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_CONTRACT_PATH:-docs/architecture/HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_BACKEND_CONTRACT_2026-06-15.md}"
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
  'HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_BACKEND_CONTRACT_READY:true'
  'HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_BACKEND_CONTRACT_DATE:2026-06-15'
  'HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_BACKEND_CONTRACT_GAP_IDS:message_search,matrix_link_resolution,message_edit_history,mention_picker_send'
  'HEPTA_NATIVE_REMOTE_RESULT_RICH_UX_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation'
  'Remote Result Cursor Contract'
  'P1 Message Search Contract'
  'P1 Remote Date Pins Scope Full-Result Taxonomy UI Packet'
  'remote_date_index_operation_id'
  'remote_pinned_fetch_operation_id'
  'cross_room_scope_request_id'
  'full_result_render_result'
  'P6 Matrix Link Route Contract'
  'route_adapter_request_id'
  'non_current_room_event_context_operation_id'
  'route/event-context result taxonomy packet'
  'P8 Edit History Full Result Contract'
  'P8 Edit History Remote Full-History/Source Taxonomy UI Packet'
  'remote_full_history_request_id'
  'replacement_source_reconciliation_operation_id'
  'P9 Mention Rich UX Contract'
  'P9 Mention Remote Result Taxonomy UI Packet'
  'remote_hover_profile_operation_id'
  'edit_payload_operation_id'
  'Verification Requirements'
)

for marker in "${MARKERS[@]}"; do
  require_marker "$CONTRACT_PATH" "$marker"
done

jq -e '
  (.items | length) == 12
  and (.items | map(select(.id == "message_search"
    and .status == "partial_live_backend_contract_remaining"
    and .ui_lane_state == "complete"
    and .next_owner_lane == "backend_contract"
    and .acceptance_state.current_ui_evidence == "machine_checked"
    and .acceptance_state.side_effect_boundary == "locked"
    and .acceptance_state.ui_contract == "ready"
    and .acceptance_state.backend_contract == "required"
    and .acceptance_state.live_wiring == "matrix_server_search_v3_search_next_batch_retry_context_pagination_sender_media_url_filter_loaded_scope_filters_loaded_result_actions_server_result_context_preview_source_refetch_live;remote_date_pins_scope_full_result_blocked"
    and (.required_backend_contracts | index("remote result page and cursor identity (next_batch live in hepta-ui native)") != null)
    and (.required_backend_contracts | index("remote date index, pinned fetch, cross-room scope, and sort contract") != null)
    and (.required_backend_contracts | index("remote date/pins/scope/full-result result taxonomy packet") != null)
    and (.required_backend_contracts | index("stale-query and idempotency guard") != null)
  )) | length) == 1
  and (.items | map(select(.id == "matrix_link_resolution"
    and .status == "partial_live_backend_contract_remaining"
    and .ui_lane_state == "complete"
    and .next_owner_lane == "backend_contract"
    and .acceptance_state.current_ui_evidence == "machine_checked"
    and .acceptance_state.side_effect_boundary == "locked"
    and .acceptance_state.ui_contract == "ready"
    and .acceptance_state.backend_contract == "required"
    and .acceptance_state.live_wiring == "matrix_link_compact_preview_retry_server_context_refresh_current_room_event_pagination_source_fetch_browser_opener_room_or_alias_join_knock_current_room_user_invite_live;event_context_route_adapter_blocked"
    and (.required_backend_contracts | index("room/alias/event/via route result") != null)
    and (.required_backend_contracts | index("server preview and non-current-room event context result") != null)
    and (.required_backend_contracts | index("route/event-context result taxonomy packet") != null)
    and (.required_backend_contracts | index("source-hash and stale-target guard") != null)
  )) | length) == 1
  and (.items | map(select(.id == "message_edit_history"
    and .status == "partial_live_backend_contract_remaining"
    and .ui_lane_state == "complete"
    and .next_owner_lane == "backend_contract"
    and .acceptance_state.current_ui_evidence == "machine_checked"
    and .acceptance_state.side_effect_boundary == "locked"
    and .acceptance_state.ui_contract == "ready"
    and .acceptance_state.backend_contract == "required"
    and .acceptance_state.live_wiring == "matrix_edit_history_paginated_relations_retry_local_full_snapshot_modal_loaded_original_replacement_source_side_by_side_preview_full_body_diff_live;remote_full_history_result_adapter_context_source_reconciliation_server_backed_full_body_diff_blocked"
    and (.required_backend_contracts | index("full edit-history modal request/result") != null)
    and (.required_backend_contracts | index("remote full-history/source reconciliation result taxonomy packet") != null)
    and (.required_backend_contracts | index("replacement event context/source result") != null)
    and (.required_backend_contracts | index("stale-target, retry, and source-hash guard") != null)
  )) | length) == 1
  and (.items | map(select(.id == "mention_picker_send"
    and .status == "partial_live_backend_contract_remaining"
    and .ui_lane_state == "complete"
    and .next_owner_lane == "backend_contract"
    and .acceptance_state.current_ui_evidence == "machine_checked"
    and .acceptance_state.side_effect_boundary == "locked"
    and .acceptance_state.ui_contract == "ready"
    and .acceptance_state.backend_contract == "required"
    and .acceptance_state.live_wiring == "matrix_sendmessage_attachment_caption_mentions_pill_tray_user_directory_search_result_promotion_hover_card_snapshot_live;rich_disambiguation_remote_hover_edit_payload_scopes_blocked"
    and (.required_backend_contracts | index("duplicate-name disambiguation result") != null)
    and (.required_backend_contracts | index("rich attachment editor plus edit/reply mention payload scopes") != null)
    and (.required_backend_contracts | index("remote hover/profile/disambiguation/edit-reply result taxonomy packet") != null)
    and (.required_backend_contracts | index("stale-token, source-hash, and idempotency guard") != null)
  )) | length) == 1
' "$HANDOFF_PATH" >/dev/null

jq -n \
  --arg status "ready" \
  --arg contract_path "$CONTRACT_PATH" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg backend_handoff_path "$HANDOFF_PATH" \
  '{
    product:"Hepta Native",
    gate:"remote_result_rich_ux_backend_contract",
    status:$status,
    contract_path:$contract_path,
    readiness_dir:$readiness_dir,
    backend_handoff_path:$backend_handoff_path,
    verified_gap_ids:[
      "message_search",
      "matrix_link_resolution",
      "message_edit_history",
      "mention_picker_send"
    ],
    required_contract_groups:[
      "remote result cursor",
      "message search date pins scope sort",
      "message search remote date pins scope full result taxonomy",
      "matrix link route and event context",
      "matrix link route event-context result taxonomy",
      "edit history server backed full result",
      "edit history remote full-history source result taxonomy",
      "mention rich disambiguation hover payload scopes",
      "mention remote hover profile result taxonomy"
    ],
    side_effects:{
      matrix_login:false,
      gateway_call:false,
      provider_invoked:false,
      channel_delivery:false,
      external_mutation:false
    }
  }'
