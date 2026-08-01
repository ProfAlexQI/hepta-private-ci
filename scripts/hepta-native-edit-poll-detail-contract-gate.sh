#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

CONTRACT_PATH="docs/architecture/HEPTA_NATIVE_EDIT_POLL_DETAIL_BACKEND_CONTRACT_2026-06-15.md"
READINESS_DIR="${HEPTA_NATIVE_EDIT_POLL_DETAIL_CONTRACT_READINESS_DIR:-${HEPTA_UI_PRODUCT_READINESS_DIR:-}}"
REPORT_PATH="${HEPTA_NATIVE_EDIT_POLL_DETAIL_CONTRACT_REPORT:-}"

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

require_file "$CONTRACT_PATH"
require_file "apps/hepta-native/src/home/editing_pane.rs"
require_file "apps/hepta-native/src/home/room_screen.rs"
require_file "apps/hepta-native/src/home/hepta_telegram_base_contract.rs"
require_file "scripts/hepta-native-fixture-visual-smoke.sh"
require_file "scripts/hepta-ui-product-readiness-gate.sh"

require_contains "$CONTRACT_PATH" "Edit Attachment Contract"
require_contains "$CONTRACT_PATH" "Attachment Preflight Packet"
require_contains "$CONTRACT_PATH" "Edit Mention Payload Contract"
require_contains "$CONTRACT_PATH" "Current local packets"
require_contains "$CONTRACT_PATH" "Poll Answer Edit Contract"
require_contains "$CONTRACT_PATH" "Poll answer preview/result packet"
require_contains "$CONTRACT_PATH" "Save Spinner / Result Contract"
require_contains "$CONTRACT_PATH" "Retry/Error Drilldown Packet"
require_contains "$CONTRACT_PATH" "no unconfirmed upload"
require_contains "$CONTRACT_PATH" "Matrix Mentions"
require_contains "$CONTRACT_PATH" "source hash"
require_contains "$CONTRACT_PATH" "retry idempotency key"
require_contains "$CONTRACT_PATH" "closed poll"
require_contains "$CONTRACT_PATH" "ignored-late-result"
require_contains "$CONTRACT_PATH" "timeline_event_item_id_match_only_without_operation_id"

require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_CONFIRMATION_COMPACT_LABEL"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_LIMITS_COMPACT_LABEL"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_DETAIL_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_ATTACHMENT_PREFLIGHT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_MENTION_PAYLOAD_PREFLIGHT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_MENTION_PAYLOAD_TYPED_CONTRACT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_SAVE_RESULT_MAPPING_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "EDITING_PANE_RETRY_ERROR_DRILLDOWN_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_detail_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_attachment_preflight_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_mention_payload_preflight_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_mention_payload_typed_contract_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_save_result_mapping_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "editing_pane_retry_error_drilldown_packet_label"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "attachment_edit_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "original_attachment_scope"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "add_attachment_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "remove_attachment_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "replace_attachment_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "upload_request_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "media_delete_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "caption_edit_handoff existing_confirmed_MatrixRequest_EditMessage_body_only"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "mime_size_probe not_started"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "cancel_policy leaves_original_media_and_local_selection_untouched"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "mention_payload_scope"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "edited_at_token_count"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "literal_user_id_token_count"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "room_token_scope"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "completed_pill_reconcile_slot not_connected_to_editing_pane"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "directory_result_scope unavailable_in_editing_pane"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "fresh_mentions_payload_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "reply_sendtime_state not_reused"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "retry_source_hash_slot missing"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "stale_token_policy backend_required_before_live_mentions"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "mention_contract_version local_v0"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "token_scan_source edited_text_only"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "directory_snapshot_id_slot unavailable"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "completed_pill_snapshot_slot unavailable"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "source_hash_slot not_assigned"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "fresh_mentions_payload_result_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "retry_idempotency_key_slot missing"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "privacy_redaction token_counts_only"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "poll_answer_edit_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "save_spinner_operation_id not_assigned"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "operation_id_slot not_assigned"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "request_slot existing_confirmed_MatrixRequest_EditMessage"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "stale_result_guard timeline_event_item_id_match_only"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "repeated_save_policy not_held_until_pending_operation_id"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "failure_source existing_MatrixRequest_EditMessage_result_only"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "retry_request_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "retry_confirmation_slot not_built"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "late_result_guard timeline_event_item_id_match_only_without_operation_id"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "pending_operation_id missing_backend_contract"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "cancel_state confirmation_cancel_no_request"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "Edit extras stay local; Save Edit uses confirmation."
require_contains "apps/hepta-native/src/home/editing_pane.rs" "TODO: support editing poll answers"
require_contains "apps/hepta-native/src/home/editing_pane.rs" "Message edit confirmation opened. {EDITING_PANE_CONFIRMATION_COMPACT_LABEL}"
require_contains "apps/hepta-native/src/home/room_screen.rs" "POLL_MESSAGE_PREVIEW_READ_EVIDENCE"
require_contains "apps/hepta-native/src/home/room_screen.rs" "POLL_ANSWER_PREVIEW_RESULT_PACKET_EVIDENCE"
require_contains "apps/hepta-native/src/home/room_screen.rs" "populate_poll_message_content"
require_contains "apps/hepta-native/src/home/room_screen.rs" "Poll answer preview/result packet"
require_contains "apps/hepta-native/src/home/room_screen.rs" "edited state, answer edit slot"
require_contains "apps/hepta-native/src/home/room_screen.rs" "answer edit slot, vote response slot"
require_contains "apps/hepta-native/src/home/room_screen.rs" "result mapping, stale poll policy, and unsupported server capability boundary from already loaded PollState only"
require_contains "apps/hepta-native/src/home/room_screen.rs" "Read-only poll preview from loaded timeline state."
require_contains "apps/hepta-native/src/home/room_screen.rs" "send no poll response, edit, redact"

require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message edit unsupported feature local evidence"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "attachment add/remove"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "mention extraction"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "poll answer edits"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "poll_answer_preview_result_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_detail_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_attachment_preflight_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_mention_payload_preflight_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_mention_payload_typed_contract_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_save_result_mapping_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "message_edit_retry_error_drilldown_packet_preview"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_poll_message_preview_local_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_poll_answer_preview_result_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_message_edit_save_result_mapping_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_message_edit_attachment_preflight_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_message_edit_mention_payload_preflight_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_message_edit_mention_payload_typed_contract_packet_ready"
require_contains "apps/hepta-native/src/home/hepta_telegram_base_contract.rs" "hepta_telegram_message_edit_retry_error_drilldown_packet_ready"

require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_poll_message_preview_local_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_poll_answer_preview_result_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-poll-message-preview="loaded-pollstate-read-only"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-poll-answer-preview-result-packet="answer-count-result-slots-read-only"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_detail_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-detail-packet="attachment-mention-poll-spinner-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_attachment_preflight_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-attachment-preflight-packet="replace-remove-upload-slots-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_mention_payload_preflight_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-mention-payload-preflight-packet="tokens-existing-fresh-payload-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_mention_payload_typed_contract_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-mention-payload-typed-contract-packet="source-hash-idempotency-result-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_save_result_mapping_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-save-result-mapping-packet="operation-result-stale-local"'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'native_telegram_message_edit_retry_error_drilldown_packet_ready:true'
require_contains "scripts/hepta-native-fixture-visual-smoke.sh" 'data-native-telegram-message-edit-retry-error-drilldown-packet="failure-retry-late-result-local"'
require_contains "scripts/hepta-ui-product-readiness-gate.sh" "Telegram EditingPane exposes a local Edit/Poll detail packet"
require_contains "scripts/hepta-ui-product-readiness-gate.sh" "Telegram EditingPane exposes a local attachment preflight packet"
require_contains "scripts/hepta-ui-product-readiness-gate.sh" "Telegram EditingPane exposes a local mention payload preflight packet"
require_contains "scripts/hepta-ui-product-readiness-gate.sh" "typed contract packet"
require_contains "scripts/hepta-ui-product-readiness-gate.sh" "Telegram EditingPane exposes a local Save result mapping packet"
require_contains "scripts/hepta-ui-product-readiness-gate.sh" "Telegram EditingPane exposes a local retry/error drilldown packet"
require_contains "scripts/hepta-ui-product-readiness-gate.sh" "Telegram poll timeline items render as first-class read-only message previews"

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
  --arg gate "edit_poll_detail_backend_contract" \
  --arg status "ready" \
  --arg contract_path "$CONTRACT_PATH" \
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
    contract_path: $contract_path,
    readiness_dir: $readiness_dir,
    readiness_path: $readiness_path,
    readiness_status: $readiness_status,
    verified_groups: [
      "edit_attachment_contract",
      "edit_attachment_preflight_packet",
      "edit_mention_payload_contract",
      "edit_mention_payload_preflight_packet",
      "edit_mention_payload_typed_contract_packet",
      "poll_answer_edit_contract",
      "poll_answer_preview_result_packet",
      "edit_save_result_mapping_packet",
      "edit_retry_error_drilldown_packet",
      "save_spinner_result_contract"
    ],
    current_ui_state: "save_edit_confirmed_poll_preview_answer_result_attachment_and_mention_contract_save_mapping_and_retry_error_packets_local",
    native_screenshot_count: $native_screenshot_count,
    control_screenshot_count: $control_screenshot_count,
    packaging_ready: $packaging_ready,
    backend_contract_gates_ready: $backend_contract_gates_ready,
    side_effects: {
      matrix_login: false,
      matrix_request_submitted: false,
      attachment_upload: false,
      poll_answer_edit: false,
      room_state_mutation: false,
      membership_mutation: false,
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
