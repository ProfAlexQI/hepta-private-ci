#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

PLAN_PATH="docs/architecture/HEPTA_NATIVE_NON_BASE_EDGE_RISK_PLAN_2026-06-15.md"
READINESS_DIR="${HEPTA_NATIVE_NON_BASE_EDGE_RISK_PLAN_READINESS_DIR:-${HEPTA_UI_PRODUCT_READINESS_DIR:-}}"
REPORT_PATH="${HEPTA_NATIVE_NON_BASE_EDGE_RISK_PLAN_REPORT:-}"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "missing required file: $path" >&2
    exit 1
  fi
}

require_contains() {
  local path="$1"
  local needle="$2"
  if ! grep -Fq "$needle" "$path"; then
    echo "missing marker in $path: $needle" >&2
    exit 1
  fi
}

require_file "$PLAN_PATH"
require_file "apps/hepta-native/src/home/location_preview.rs"
require_file "apps/hepta-native/src/home/hepta_telegram_base_contract.rs"
require_file "apps/hepta-native/src/tsp/create_wallet_modal.rs"
require_file "apps/hepta-native/src/tsp/create_did_modal.rs"
require_file "apps/hepta-native/src/tsp/verify_user.rs"
require_file "apps/hepta-native/src/home/rooms_list.rs"
require_file "apps/hepta-native/src/room/room_display_filter.rs"
require_file "apps/hepta-native/src/home/add_room.rs"
require_file "apps/hepta-native/src/home/space_lobby.rs"
require_file "apps/hepta-native/src/home/editing_pane.rs"
require_file "scripts/hepta-native-fixture-visual-smoke.sh"
require_file "scripts/hepta-ui-product-readiness-gate.sh"
require_file "docs/architecture/HEPTA_NATIVE_TSP_WALLET_DESTRUCTIVE_IMPORT_BACKEND_CONTRACT_2026-06-15.md"
require_file "scripts/hepta-native-tsp-wallet-destructive-import-contract-gate.sh"
require_file "docs/architecture/HEPTA_NATIVE_SPACES_ROOM_MEMBERSHIP_EDGE_BACKEND_CONTRACT_2026-06-15.md"
require_file "scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh"
require_file "docs/architecture/HEPTA_NATIVE_EDIT_POLL_DETAIL_BACKEND_CONTRACT_2026-06-15.md"
require_file "scripts/hepta-native-edit-poll-detail-contract-gate.sh"

require_contains "$PLAN_PATH" "Location Continuous Device Updates"
require_contains "$PLAN_PATH" "Status: UI live local control complete."
require_contains "$PLAN_PATH" "TSP Wallet Import Preflight Packet"
require_contains "$PLAN_PATH" "Status: UI local preflight and result taxonomy packets complete."
require_contains "$PLAN_PATH" "local import result taxonomy packet"
require_contains "$PLAN_PATH" "TSP Worker Receipt/Result Packet"
require_contains "$PLAN_PATH" "TSP Pending Cancel Operation Packet"
require_contains "$PLAN_PATH" "TSP Association Cancel/Remove Packet"
require_contains "$PLAN_PATH" "Spaces Selected-Room Removed/Rejoin Packet"
require_contains "$PLAN_PATH" "Spaces Re-Knock/Cancel-Prior Packet"
require_contains "$PLAN_PATH" "Spaces Unread/Mention Aggregate Packet"
require_contains "$PLAN_PATH" "Spaces Room-List Load More Pagination Packet"
require_contains "$PLAN_PATH" "Edit/Poll Answer Preview Result Packet"
require_contains "$PLAN_PATH" "Edit Attachment Preflight Packet"
require_contains "$PLAN_PATH" "TSP Wallet Destructive/Import Actions"
require_contains "$PLAN_PATH" "Status: backend contract spec/gate added"
require_contains "$PLAN_PATH" "HEPTA_NATIVE_TSP_WALLET_DESTRUCTIVE_IMPORT_BACKEND_CONTRACT_2026-06-15.md"
require_contains "$PLAN_PATH" "Spaces And Room Membership Edges"
require_contains "$PLAN_PATH" "HEPTA_NATIVE_SPACES_ROOM_MEMBERSHIP_EDGE_BACKEND_CONTRACT_2026-06-15.md"
require_contains "$PLAN_PATH" "Edit And Poll Detail Edges"
require_contains "$PLAN_PATH" "Edit/Poll detail packet"
require_contains "$PLAN_PATH" "retry/error drilldown"
require_contains "$PLAN_PATH" "HEPTA_NATIVE_EDIT_POLL_DETAIL_BACKEND_CONTRACT_2026-06-15.md"
require_contains "docs/architecture/HEPTA_NATIVE_TSP_WALLET_DESTRUCTIVE_IMPORT_BACKEND_CONTRACT_2026-06-15.md" "Delete Wallet Contract"
require_contains "docs/architecture/HEPTA_NATIVE_TSP_WALLET_DESTRUCTIVE_IMPORT_BACKEND_CONTRACT_2026-06-15.md" "Import Wallet Contract"
require_contains "scripts/hepta-native-tsp-wallet-destructive-import-contract-gate.sh" "tsp_wallet_destructive_import_backend_contract"
require_contains "docs/architecture/HEPTA_NATIVE_SPACES_ROOM_MEMBERSHIP_EDGE_BACKEND_CONTRACT_2026-06-15.md" "Re-Knock / Cancel Prior Knock Contract"
require_contains "docs/architecture/HEPTA_NATIVE_SPACES_ROOM_MEMBERSHIP_EDGE_BACKEND_CONTRACT_2026-06-15.md" "Room-list Load More pagination packet"
require_contains "docs/architecture/HEPTA_NATIVE_SPACES_ROOM_MEMBERSHIP_EDGE_BACKEND_CONTRACT_2026-06-15.md" "Space Aggregate Contract"
require_contains "scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh" "spaces_room_membership_edge_backend_contract"
require_contains "scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh" "selected_room_removed_rejoin_packet_contract"
require_contains "scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh" "space_lobby_reknock_cancel_prior_packet_preview"
require_contains "scripts/hepta-native-spaces-room-membership-edge-contract-gate.sh" "space_unread_filter_aggregate_packet_preview"
require_contains "docs/architecture/HEPTA_NATIVE_EDIT_POLL_DETAIL_BACKEND_CONTRACT_2026-06-15.md" "Edit Attachment Contract"
require_contains "docs/architecture/HEPTA_NATIVE_EDIT_POLL_DETAIL_BACKEND_CONTRACT_2026-06-15.md" "Attachment Preflight Packet"
require_contains "docs/architecture/HEPTA_NATIVE_EDIT_POLL_DETAIL_BACKEND_CONTRACT_2026-06-15.md" "Poll Answer Edit Contract"
require_contains "docs/architecture/HEPTA_NATIVE_EDIT_POLL_DETAIL_BACKEND_CONTRACT_2026-06-15.md" "Poll answer preview/result packet"
require_contains "scripts/hepta-native-edit-poll-detail-contract-gate.sh" "edit_poll_detail_backend_contract"
require_contains "scripts/hepta-native-edit-poll-detail-contract-gate.sh" "message_edit_detail_packet_preview"
require_contains "scripts/hepta-native-edit-poll-detail-contract-gate.sh" "message_edit_attachment_preflight_packet_preview"
require_contains "scripts/hepta-native-edit-poll-detail-contract-gate.sh" "poll_answer_preview_result_packet"
require_contains "scripts/hepta-native-edit-poll-detail-contract-gate.sh" "edit_save_result_mapping_packet"
require_contains "scripts/hepta-native-edit-poll-detail-contract-gate.sh" "edit_retry_error_drilldown_packet"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_message_edit_save_result_mapping_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_message_edit_attachment_preflight_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_message_edit_retry_error_drilldown_packet_ready"

require_contains "apps/hepta-native/src/home/location_preview.rs" "Start Device Updates submits only LocationRequest::StartUpdates"
require_contains "apps/hepta-native/src/home/location_preview.rs" "Stop Device Updates and Cancel submit only LocationRequest::StopUpdates"
require_contains "apps/hepta-native/src/home/location_preview.rs" "LIVE_LOCATION_CONTINUOUS_UPDATES_ACTIVE_LABEL"
require_contains "apps/hepta-native/src/home/location_preview.rs" "request_location_update(LocationRequest::StartUpdates)"
require_contains "apps/hepta-native/src/home/location_preview.rs" "request_location_update(LocationRequest::StopUpdates)"
require_contains "apps/hepta-native/src/home/location_preview.rs" "do not create a live-location Matrix event"

require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "Start submits only LocationRequest::StartUpdates"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "Stop and Cancel submit only LocationRequest::StopUpdates"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_live_location_continuous_updates_boundary_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_wallet_import_preflight_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_wallet_import_result_taxonomy_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_wallet_delete_preflight_result_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_worker_receipt_result_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_pending_cancel_operation_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_tsp_association_cancel_remove_packet_ready"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "TSP_WALLET_IMPORT_PREFLIGHT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "TSP_WALLET_IMPORT_RESULT_TAXONOMY_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "TSP_WORKER_RECEIPT_RESULT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "password_state not_collected"
require_contains "apps/hepta-native/src/tsp/tsp_settings_screen.rs" "vault_open_result opened|invalid_password|unsupported_vault|corrupted_database|already_imported|duplicate_path|permission_denied not_wired"
require_contains "apps/hepta-native/src/tsp/create_wallet_modal.rs" "TSP_WALLET_PENDING_CANCEL_OPERATION_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/create_did_modal.rs" "TSP_DID_PENDING_CANCEL_OPERATION_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/tsp/verify_user.rs" "TSP_ASSOCIATION_CANCEL_REMOVE_PACKET_EVIDENCE"
require_contains "scripts/hepta-native-tsp-wallet-destructive-import-contract-gate.sh" "visible_delete_preflight_result_taxonomy_import_preflight_result_taxonomy_worker_receipt_pending_cancel_and_association_operation_packets_warning_only"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "Selected-room removed/rejoin packet"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "selected_room_removed_rejoin_packet_label"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "replacement_ui_slot not_wired"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "rejoin_request_slot not_built"
require_contains "apps/hepta-native/src/home/add_room.rs" "ADD_ROOM_REKNOCK_CANCEL_PRIOR_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/add_room.rs" "add_room_reknock_cancel_prior_packet_label"
require_contains "apps/hepta-native/src/home/space_lobby.rs" "SPACE_LOBBY_REKNOCK_CANCEL_PRIOR_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/space_lobby.rs" "space_lobby_reknock_cancel_prior_packet_label"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "section_unread_aggregate_packet_label"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "header_badge_source local_zero_placeholder"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "aggregate_refresh_slot not_built"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "room_list_load_more_packet_label"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "Room-list Load More pagination packet"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "load_more_button_slot not_rendered"
require_contains "apps/hepta-native/src/home/rooms_list.rs" "explicit_cursor_slot not_exposed"
require_contains "apps/hepta-native/src/room/room_display_filter.rs" "Space unread/mention aggregate packet"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "rooms_list_section_unread_aggregate_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "rooms_list_load_more_pagination_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "space_unread_filter_aggregate_packet_preview"
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-rooms-list-selected-room-removed-rejoin-packet="focus-rejoin-slots-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-space-lobby-reknock-cancel-prior-packet="tree-action-cancel-slot-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-add-room-reknock-cancel-prior-packet="confirmed-reknock-cancel-slot-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-rooms-list-unread-aggregate-packet="loaded-row-totals-local-zero-header"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-rooms-list-load-more-pagination-packet="loaded-counts-cursor-result-slots-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-space-unread-filter-aggregate-packet="joined-space-zero-source-local"'

require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_live_location_continuous_updates_boundary_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-live-location-continuous-boundary="update-once-start-stop-no-matrix-live-event"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_tsp_wallet_import_preflight_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_tsp_wallet_import_result_taxonomy_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_tsp_wallet_delete_preflight_result_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-tsp-wallet-import-preflight-packet="picker-password-vault-persistence-not-started"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-tsp-wallet-import-result-taxonomy-packet="picker-auth-vault-persistence-result-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-tsp-wallet-delete-preflight-result-packet="preflight-result-taxonomy-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_tsp_pending_cancel_operation_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-tsp-pending-cancel-operation-packet="operation-id-stale-result-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_tsp_worker_receipt_result_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-tsp-worker-receipt-result-packet="worker-action-result-stale-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_tsp_association_cancel_remove_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-tsp-association-cancel-remove-packet="request-id-persistence-receive-loop-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_save_result_mapping_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-save-result-mapping-packet="operation-result-stale-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_attachment_preflight_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-attachment-preflight-packet="replace-remove-upload-slots-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_mention_payload_preflight_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-mention-payload-preflight-packet="tokens-existing-fresh-payload-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_mention_payload_typed_contract_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-mention-payload-typed-contract-packet="source-hash-idempotency-result-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_retry_error_drilldown_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-retry-error-drilldown-packet="failure-retry-late-result-local"'
require_contains "scripts/hepta-ui-product-readiness-gate.sh" 'data-native-telegram-live-location-continuous-boundary="update-once-start-stop-no-matrix-live-event"'
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_DETAIL_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_ATTACHMENT_PREFLIGHT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_MENTION_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_RETRY_ERROR_DRILLDOWN_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_detail_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_attachment_preflight_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_mention_payload_typed_contract_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_retry_error_drilldown_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "attachment_edit_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "add_attachment_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "caption_edit_handoff existing_confirmed_MatrixRequest_EditMessage_body_only"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "poll_answer_edit_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "save_spinner_operation_id not_assigned"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "source_hash_slot not_assigned"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "retry_idempotency_key_slot missing"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "retry_request_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "late_result_guard timeline_event_item_id_match_only_without_operation_id"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_detail_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_attachment_preflight_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_mention_payload_typed_contract_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_retry_error_drilldown_packet_preview"
require_contains "apps/hepta-native/src/home/room_screen.rs" "Poll answer preview/result packet"
require_contains "apps/hepta-native/src/home/room_screen.rs" "answer_edit_slot not_built"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "poll_answer_preview_result_packet_preview"
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-detail-packet="attachment-mention-poll-spinner-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-attachment-preflight-packet="replace-remove-upload-slots-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-poll-answer-preview-result-packet="answer-count-result-slots-read-only"'

readiness_status="not_provided"
readiness_path=""
native_screenshot_count=0
control_screenshot_count=0
packaging_ready=false
backend_contract_gates_ready=false

if [[ -n "$READINESS_DIR" ]]; then
  readiness_path="$READINESS_DIR/readiness.json"
  require_file "$readiness_path"
  readiness_status="$(jq -r '.status // "missing"' "$readiness_path")"
  if [[ "$readiness_status" != "ready" ]]; then
    echo "readiness status is not ready: $readiness_status" >&2
    exit 1
  fi
  native_screenshot_count="$(jq -r '.native.screenshot_count // 0' "$readiness_path")"
  control_screenshot_count="$(jq -r '.control_ui.screenshot_count // 0' "$readiness_path")"
  packaging_ready="$(jq -r '.native.packaging_gate_ready // false' "$readiness_path")"
  backend_contract_gates_ready="$(jq -r '.native_backend_contract_gates_ready // false' "$readiness_path")"
  if (( native_screenshot_count < 28 )); then
    echo "expected at least 28 native fixture screenshots, got $native_screenshot_count" >&2
    exit 1
  fi
  if (( control_screenshot_count < 3 )); then
    echo "expected at least 3 control screenshots, got $control_screenshot_count" >&2
    exit 1
  fi
  if [[ "$packaging_ready" != "true" ]]; then
    echo "native packaging gate not ready" >&2
    exit 1
  fi
  if [[ "$backend_contract_gates_ready" != "true" ]]; then
    echo "backend contract wave gates not ready" >&2
    exit 1
  fi
fi

json_report="$(jq -n \
  --arg product "Hepta Native" \
  --arg gate "non_base_edge_risk_plan" \
  --arg status "ready" \
  --arg plan_path "$PLAN_PATH" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg readiness_path "$readiness_path" \
  --arg readiness_status "$readiness_status" \
  --argjson native_screenshot_count "$native_screenshot_count" \
  --argjson control_screenshot_count "$control_screenshot_count" \
  --argjson packaging_ready "$packaging_ready" \
  --argjson backend_contract_gates_ready "$backend_contract_gates_ready" \
  '{
    product: $product,
    gate: $gate,
    status: $status,
    plan_path: $plan_path,
    readiness_dir: $readiness_dir,
    readiness_path: $readiness_path,
    readiness_status: $readiness_status,
    verified_current_closure: [
      "location_continuous_device_updates_local_controls",
      "tsp_wallet_import_preflight_packet_local_controls",
      "tsp_wallet_import_result_taxonomy_packet_local_controls",
      "tsp_wallet_delete_preflight_result_taxonomy_packet",
      "tsp_worker_receipt_result_packet_local_controls",
      "tsp_pending_cancel_operation_packet_local_controls",
      "tsp_association_cancel_remove_packet_local_controls",
      "spaces_selected_room_removed_rejoin_packet_local_controls",
      "spaces_reknock_cancel_prior_packet_local_controls",
      "spaces_unread_mention_aggregate_packet_local_controls",
      "spaces_room_list_load_more_pagination_packet_local_controls",
      "edit_poll_answer_preview_result_packet_local_controls",
      "edit_attachment_preflight_packet_local_controls",
      "edit_mention_payload_typed_contract_packet_local_controls",
      "edit_save_result_mapping_packet_local_controls",
      "edit_retry_error_drilldown_packet_local_controls",
      "edit_poll_detail_packet_local_controls"
    ],
    remaining_edge_risks: [
      "tsp_wallet_destructive_import_actions",
      "spaces_room_membership_edges",
      "edit_poll_detail_edges"
    ],
    native_screenshot_count: $native_screenshot_count,
    control_screenshot_count: $control_screenshot_count,
    packaging_ready: $packaging_ready,
    backend_contract_gates_ready: $backend_contract_gates_ready,
    side_effects: {
      matrix_login: false,
      gateway_call: false,
      provider_invoked: false,
      channel_delivery: false,
      external_mutation: false
    }
  }')"

if [[ -n "$REPORT_PATH" ]]; then
  mkdir -p "$(dirname "$REPORT_PATH")"
  printf '%s\n' "$json_report" > "$REPORT_PATH"
else
  printf '%s\n' "$json_report"
fi
