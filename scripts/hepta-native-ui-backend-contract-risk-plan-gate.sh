#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

PLAN_PATH="${HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_PATH:-docs/architecture/HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_2026-06-15.md}"
READINESS_DIR="${HEPTA_UI_PRODUCT_READINESS_DIR:-/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615}"
HANDOFF_PATH="$READINESS_DIR/native-base-gap-backend-handoff.json"
READINESS_PATH="$READINESS_DIR/readiness.json"
ALLOW_IN_PROGRESS="${HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_ALLOW_IN_PROGRESS:-0}"

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

require_file "$PLAN_PATH"
require_file "$HANDOFF_PATH"
if [[ ! -s "$READINESS_PATH" && "$ALLOW_IN_PROGRESS" != "1" ]]; then
  require_file "$READINESS_PATH"
fi

PLAN_MARKERS=(
  'HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_READY:true'
  'HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_DATE:2026-06-15'
  'HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_SCOPE:apps/hepta-native,apps/hepta-control-ui,ui-fixtures,packaging,screenshot-gates'
  'HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation'
  'P1 message_search'
  'remote date/pins/scope/full-result taxonomy packet'
  'P2 file_upload_send'
  'accepted queue/progress/result taxonomy packet'
  'P3 media_download_playback'
  'decrypt/decode/opener/queue result taxonomy packet'
  'P4 notifications'
  'P5 room_settings'
  'power/member result taxonomy packet'
  'P6 matrix_link_resolution'
  'route/event-context result taxonomy packet'
  'P7 message_report_send'
  'workflow result taxonomy packet'
  'P8 message_edit_history'
  'remote full-history/source result taxonomy packet'
  'P9 mention_picker_send'
  'remote hover/profile/disambiguation/edit-reply result taxonomy packet'
  'P10 voice_message_send'
  'permission/capture/upload result taxonomy packet'
  'P11 account_avatar_upload'
  'source/cropper/camera/editor artifact result taxonomy packet'
  'P12 account_management'
  'password/SSO/revoke/trust/delete result taxonomy packet'
  'Wave 1: high-risk write contracts'
  'Wave 2: queue/progress and platform permission contracts'
  'Wave 3: remote result adapters and rich UX contracts'
  'True-window smoke remains separate from fixture readiness'
)

for marker in "${PLAN_MARKERS[@]}"; do
  require_marker "$PLAN_PATH" "$marker"
done

if [[ -s "$READINESS_PATH" ]]; then
  jq -e '
    .status == "ready"
    and .ui_product_readiness_gate_ready == true
    and .control_ui.screenshot_count >= 4
    and .native.screenshot_count >= 28
    and .native.packaging_gate_ready == true
    and .native.base_gap_backend_handoff_count == 12
    and .side_effects.matrix_login == false
    and .side_effects.gateway_call == false
    and .side_effects.provider_invoked == false
    and .side_effects.channel_delivery == false
    and .side_effects.external_mutation == false
  ' "$READINESS_PATH" >/dev/null
  readiness_report_state="ready"
else
  readiness_report_state="in_progress_allowed"
fi

jq -e '
  (.items | length) == 12
  and (.items | all(.status == "partial_live_backend_contract_remaining"))
  and (.items | all(.ui_lane_state == "complete"))
  and (.items | all(.acceptance_state.current_ui_evidence == "machine_checked"))
  and (.items | all(.acceptance_state.side_effect_boundary == "locked"))
  and (.items | all(.acceptance_state.ui_contract == "ready"))
  and (.items | all(.acceptance_state.backend_contract == "required"))
  and (.items | map(select(.id == "message_search" and .acceptance_state.live_wiring == "matrix_server_search_v3_search_next_batch_retry_context_pagination_sender_media_url_filter_loaded_scope_filters_loaded_result_actions_server_result_context_preview_source_refetch_live;remote_date_pins_scope_full_result_blocked")) | length) == 1
  and (.items | map(select(.id == "file_upload_send" and .acceptance_state.live_wiring == "matrix_sdk_send_attachment_queue_live;failed_handoff_retry_sendattachment_live;timeline_progress_sendhandle_cancel_result_live;cross_platform_queue_controls_blocked")) | length) == 1
  and (.items | map(select(.id == "media_download_playback" and .acceptance_state.live_wiring == "matrix_sdk_fetchmedia_save_media_retry_opener_openfolder_replay_live;inline_decrypt_queue_controls_blocked")) | length) == 1
  and (.items | map(select(.id == "notifications" and .acceptance_state.live_wiring == "matrix_room_notification_mode_keyword_mutation_retry_keyword_list_default_mode_read_write_pusher_status_live;timed_pusher_sound_raw_rules_blocked")) | length) == 1
  and (.items | map(select(.id == "room_settings" and .acceptance_state.live_wiring == "matrix_room_settings_power_member_server_refresh_name_topic_alias_avatar_history_joinrule_tombstone_retry_live;power_member_blocked")) | length) == 1
  and (.items | map(select(.id == "matrix_link_resolution" and .acceptance_state.live_wiring == "matrix_link_compact_preview_retry_server_context_refresh_current_room_event_pagination_source_fetch_browser_opener_room_or_alias_join_knock_current_room_user_invite_live;event_context_route_adapter_blocked")) | length) == 1
  and (.items | map(select(.id == "message_report_send" and .acceptance_state.live_wiring == "matrix_report_content_result_retry_loaded_source_fetch_modal_live;moderation_workflow_blocked")) | length) == 1
  and (.items | map(select(.id == "message_edit_history" and .acceptance_state.live_wiring == "matrix_edit_history_paginated_relations_retry_local_full_snapshot_modal_loaded_original_replacement_source_side_by_side_preview_full_body_diff_live;remote_full_history_result_adapter_context_source_reconciliation_server_backed_full_body_diff_blocked")) | length) == 1
  and (.items | map(select(.id == "mention_picker_send" and .acceptance_state.live_wiring == "matrix_sendmessage_attachment_caption_mentions_pill_tray_user_directory_search_result_promotion_hover_card_snapshot_live;rich_disambiguation_remote_hover_edit_payload_scopes_blocked")) | length) == 1
  and (.items | map(select(.id == "voice_message_send" and .acceptance_state.live_wiring == "matrix_sendattachment_voice_audio_file_retry_review_opener_dropcleanup_selected_audio_waveform_codec_live;recorder_capture_upload_blocked")) | length) == 1
  and (.items | map(select(.id == "account_avatar_upload" and .acceptance_state.live_wiring == "matrix_uploadavatar_setavatar_some_retry_setavatar_none_direct_mxc_setavatar_thumbnail_full_pixel_decode_live;source_cropper_camera_editor_thumbnail_artifact_blocked")) | length) == 1
  and (.items | map(select(.id == "account_management" and .acceptance_state.live_wiring == "matrix_getowndevice_getdevices_retry_setdisplayname_resubmit_current_device_renamedevice_browser_portal_homeserver_opener_live;password_sso_revoke_trust_cross_session_device_mutations_blocked")) | length) == 1
' "$HANDOFF_PATH" >/dev/null

jq -n \
  --arg status "ready" \
  --arg plan_path "$PLAN_PATH" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg readiness_report_path "$READINESS_PATH" \
  --arg backend_handoff_path "$HANDOFF_PATH" \
  --arg readiness_report_state "$readiness_report_state" \
  '{
    product:"Hepta Native",
    gate:"ui_backend_contract_risk_plan",
    status:$status,
    plan_path:$plan_path,
    readiness_dir:$readiness_dir,
    readiness_report_path:$readiness_report_path,
    readiness_report_state:$readiness_report_state,
    backend_handoff_path:$backend_handoff_path,
    verified_items:12,
    waves:[
      "high-risk write contracts",
      "queue/progress and platform permission contracts",
      "remote result adapters and rich UX contracts"
    ],
    side_effects:{
      matrix_login:false,
      gateway_call:false,
      provider_invoked:false,
      channel_delivery:false,
      external_mutation:false
    }
  }'
