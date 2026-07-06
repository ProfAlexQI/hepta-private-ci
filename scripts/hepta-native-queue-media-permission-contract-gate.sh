#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

CONTRACT_PATH="${HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_CONTRACT_PATH:-docs/architecture/HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_BACKEND_CONTRACT_2026-06-15.md}"
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
  'HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_BACKEND_CONTRACT_READY:true'
  'HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_BACKEND_CONTRACT_DATE:2026-06-15'
  'HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_BACKEND_CONTRACT_GAP_IDS:file_upload_send,media_download_playback,voice_message_send,account_avatar_upload'
  'HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation'
  'Shared Queue Result Contract'
  'P2 File Upload Contract'
  'P2 Accepted Queue Progress Result Taxonomy UI Packet'
  'accepted_queue_operation_id'
  'timeline_cancel_result'
  'P3 Media Playback Contract'
  'P3 Decrypt Decode Result Taxonomy UI Packet'
  'decrypt_operation_id'
  'background_queue_result'
  'P10 Voice Recorder Contract'
  'P10 Permission Capture Upload Result Taxonomy UI Packet'
  'microphone_permission_operation_id'
  'captured_upload_queue_item_id'
  'P11 Avatar Source And Editor Contract'
  'P11 Source Cropper Camera Editor Artifact Result Taxonomy UI Packet'
  'source_identity_operation_id'
  'persistent_thumbnail_artifact_id'
  'Verification Requirements'
)

for marker in "${MARKERS[@]}"; do
  require_marker "$CONTRACT_PATH" "$marker"
done

jq -e '
  (.items | length) == 12
  and (.items | map(select(.id == "file_upload_send" and .status == "partial_live_backend_contract_remaining" and .ui_lane_state == "complete" and .acceptance_state.live_wiring == "matrix_sdk_send_attachment_queue_live;failed_handoff_retry_sendattachment_live;timeline_progress_sendhandle_cancel_result_live;cross_platform_queue_controls_blocked" and (.required_backend_contracts | index("idempotency and stale SendHandle guard") != null) and (.required_backend_contracts | index("accepted queue/progress/result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "media_download_playback" and .status == "partial_live_backend_contract_remaining" and .ui_lane_state == "complete" and .acceptance_state.live_wiring == "matrix_sdk_fetchmedia_save_media_retry_opener_openfolder_replay_live;inline_decrypt_queue_controls_blocked" and (.required_backend_contracts | index("decrypt/decode/opener/queue result taxonomy packet") != null) and (.required_backend_contracts | index("encrypted-media error taxonomy") != null))) | length) == 1
  and (.items | map(select(.id == "voice_message_send" and .status == "partial_live_backend_contract_remaining" and .ui_lane_state == "complete" and .acceptance_state.live_wiring == "matrix_sendattachment_voice_audio_file_retry_review_opener_dropcleanup_selected_audio_waveform_codec_live;recorder_capture_upload_blocked" and (.required_backend_contracts | index("microphone permission and privacy entitlement result") != null) and (.required_backend_contracts | index("permission/capture/upload result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "account_avatar_upload" and .status == "partial_live_backend_contract_remaining" and .ui_lane_state == "complete" and .acceptance_state.live_wiring == "matrix_uploadavatar_setavatar_some_retry_setavatar_none_direct_mxc_setavatar_thumbnail_full_pixel_decode_live;source_cropper_camera_editor_thumbnail_artifact_blocked" and (.required_backend_contracts | index("camera/photo-library permission and capture result") != null) and (.required_backend_contracts | index("source/cropper/camera/editor artifact result taxonomy packet") != null))) | length) == 1
  and (.items | map(select(.id == "file_upload_send" or .id == "media_download_playback" or .id == "voice_message_send" or .id == "account_avatar_upload") | select(.acceptance_state.current_ui_evidence == "machine_checked" and .acceptance_state.side_effect_boundary == "locked" and .acceptance_state.ui_contract == "ready" and .acceptance_state.backend_contract == "required")) | length) == 4
' "$HANDOFF_PATH" >/dev/null

jq -n \
  --arg status "ready" \
  --arg contract_path "$CONTRACT_PATH" \
  --arg readiness_dir "$READINESS_DIR" \
  --arg backend_handoff_path "$HANDOFF_PATH" \
  '{
    product:"Hepta Native",
    gate:"queue_media_permission_backend_contract",
    status:$status,
    contract_path:$contract_path,
    readiness_dir:$readiness_dir,
    backend_handoff_path:$backend_handoff_path,
    verified_gap_ids:[
      "file_upload_send",
      "media_download_playback",
      "voice_message_send",
      "account_avatar_upload"
    ],
    required_contract_groups:[
      "shared queue result",
      "file upload accepted queue",
      "file upload accepted queue/progress/result taxonomy packet",
      "media playback decrypt decode queue",
      "media playback decrypt/decode/opener/queue result taxonomy packet",
      "voice recorder permission capture upload",
      "voice recorder permission/capture/upload result taxonomy packet",
      "avatar cropper camera editor artifact",
      "avatar source/cropper/camera/editor artifact result taxonomy packet"
    ],
    side_effects:{
      matrix_login:false,
      gateway_call:false,
      provider_invoked:false,
      channel_delivery:false,
      external_mutation:false
    }
  }'
