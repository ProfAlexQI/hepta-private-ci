# Hepta Native Queue Media Permission Backend Contract

Date: 2026-06-15
Status: backend contract required / UI evidence complete
Wave: 2 queue/progress and platform permission contracts
Source readiness artifact: `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615`

Machine markers:

- `HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_BACKEND_CONTRACT_READY:true`
- `HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_BACKEND_CONTRACT_DATE:2026-06-15`
- `HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_BACKEND_CONTRACT_GAP_IDS:file_upload_send,media_download_playback,voice_message_send,account_avatar_upload`
- `HEPTA_NATIVE_QUEUE_MEDIA_PERMISSION_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation`

## UI Evidence Already Complete

P2 `file_upload_send`:

- Live single-file `SendAttachment` through SDK send queue.
- Live timeline progress/error/sent local echo states.
- Live timeline `AbortLocalSend(local_echo_send_handle)` bridge.
- Confirmed failed-handoff retry.
- Local accepted queue snapshot, per-file queue drilldown, typed SDK queue contract, and queue/preflight/mobile picker controls.
- Accepted queue/progress/result taxonomy packet.

P3 `media_download_playback`:

- Live `FetchMedia` cache read.
- Confirmed `SaveMedia`.
- Guarded row-scoped `SaveMedia` retry.
- System opener outcome popup.
- Cached Open folder/Replay stale validation and eviction.
- Cached saved-file metadata snapshot, operation packet, typed playback/media queue contract, and codec/recovery controls.
- Decrypt/decode/opener/queue result taxonomy packet.

P10 `voice_message_send`:

- Live desktop audio `SendAttachment`.
- Confirmed failed-handoff retry.
- Audio file handoff.
- Selected-audio duration/codec/bounded WAV waveform analysis.
- Review Play opener.
- Drop pending-audio cleanup.
- Recorder lifecycle packet, typed recorder/upload contract, and permission/capture/upload result taxonomy packet.

P11 `account_avatar_upload`:

- Live confirmed `UploadAvatar` plus SDK `set_avatar_url(Some)`.
- Confirmed failed-state upload retry.
- Direct MXC SetAvatar(Some) and SetAvatar(None) delete.
- Bounded thumbnail/full pixel decode.
- Source clipboard, cropper/editor/preflight controls, source/editor drilldown, typed cropper-camera contract, and source/cropper/camera/editor artifact result taxonomy packet.

## Shared Promotion Boundary

Do not wire remaining queue, playback, recorder, picker, camera, or editor controls until the backend contract provides request, result, error, retry, cancel, source-hash, idempotency, and stale-handle semantics.

Forbidden as a side effect of this contract:

- Gateway/runtime/provider-auth calls.
- Telegram delivery mutation.
- Room-state or membership mutation.
- Account/profile mutation outside explicit avatar upload/set/delete result contracts.
- Unconfirmed upload, media, recorder, camera, or editor writes.
- Automatic retry without cached request identity and confirmation.
- Secret, token, or unredacted platform permission payload in logs, docs, clipboard, or gate reports.

## Shared Queue Result Contract

Required fields:

- `queue_item_id`
- `local_echo_id`
- `operation_kind`
- `source_path_hash`
- `media_mxc`
- `request_id`
- `source_hash`
- `status`: `queued`, `uploading`, `sent`, `failed`, `cancelled`, or `stale`
- `bytes_sent`
- `bytes_total`
- `progress_percent`
- `speed_bytes_per_second`
- `eta_ms`
- `error_kind`
- `error_message`
- `retry_eligible`
- `cancel_eligible`

Acceptance rules:

- Queue item identity and local echo identity must be stable across progress updates.
- Progress units must be explicit bytes and percent.
- Cancel eligibility must name whether SDK abort, local echo cancel, or backend queue cancel owns the action.
- Retry must require cached request identity and confirmation.

## P2 File Upload Contract

live_wiring: `matrix_sdk_send_attachment_queue_live;failed_handoff_retry_sendattachment_live;timeline_progress_sendhandle_cancel_result_live;cross_platform_queue_controls_blocked`

Additional required fields:

- `file_name`
- `mime_type`
- `file_size_bytes`
- `caption`
- `mention_payload_hash`
- `send_handle_state`
- `delivery_receipt_state`

Blocked controls requiring contract:

- Accepted queue pause/resume.
- Accepted queue retry.
- Reorder/remove.
- Background persistence.
- Multi-file album grouping.
- Cross-platform picker/share-sheet result.

Acceptance rules:

- Timeline local echo cancel must not be reported as accepted-queue cancel unless the queue result confirms it.
- Delivery receipt must not be inferred from SDK sent state alone.
- Stale `SendHandle` must disable cancel/retry until refreshed.

### P2 Accepted Queue Progress Result Taxonomy UI Packet

The UI lane now exposes a local-only Taxonomy packet for accepted queue/progress/result ownership before promoting true queue controls.

Live references:

- Review-row `MatrixRequest::SendAttachment`.
- `Timeline::send_attachment().use_send_queue()`.
- Timeline local echo progress/error/sent rendering.
- `MatrixRequest::AbortLocalSend` with `TimelineUpdate::LocalSendAbortResult`.
- Confirmed failed-handoff Retry for immediate worker handoff failure.

Blocked operation ids and identities:

- `accepted_queue_operation_id`: `not_assigned`
- `queue_item_id`: `timeline_owned_not_available_in_composer`
- `local_echo_id`: `timeline_owned_not_available_in_composer`

Blocked result taxonomy:

- `progress_subscription_result`: `bytes_sent`, `bytes_total`, `percent`, `speed`, `eta`, `not_subscribed_in_composer`
- `queue_result`: `queued`, `uploading`, `sent`, `failed`, `cancelled`, `stale`, `not_wired_to_composer_recovery`
- `delivery_receipt_result`: `delivered`, `failed`, `unknown`, `not_wired`
- `pause_result`: `not_wired`
- `resume_result`: `not_wired`
- `accepted_queue_retry_result`: `not_wired`
- `composer_cancel_result`: `not_wired`
- `timeline_cancel_result`: `canceled`, `already_sent_or_no_longer_cancellable`, `failed`
- `reorder_remove_result`: `not_wired`
- `background_persistence_result`: `not_wired`

Acceptance rules:

- Composer controls must not claim a queue item id, local echo id, progress stream, delivery receipt, or cancel ownership unless the backend/SDK adapter provides it.
- Timeline local echo Cancel remains the only accepted-send cancel path until a queue cancel result contract exists.
- Stale `SendHandle` promotion requires SendHandle generation, source hash, queue item id, and local echo id.
- Audit metadata must not expose raw file paths, access tokens, room secrets, caption bodies, full mention payloads, or delivery receipt secrets.

## P3 Media Playback Contract

live_wiring: `matrix_sdk_fetchmedia_save_media_retry_opener_openfolder_replay_live;inline_decrypt_queue_controls_blocked`

Required fields:

- `mxc_uri`
- `cache_key`
- `local_path_hash`
- `decrypt_status`
- `decode_status`
- `codec`
- `duration_ms`
- `playback_position_ms`
- `opener_status`
- `stale_cache_evicted`

Blocked controls requiring contract:

- Inline decrypt.
- Inline decode.
- Playback queue progress.
- Codec fallback/transcode/caption result.
- Queue retry/resume/cancel/background persistence.

Acceptance rules:

- Opener success is not inline playback success.
- Decrypt failure, decode failure, unsupported codec, and missing cached file must be distinct.
- Stale cached local paths must be evicted before Open folder/Replay is offered again.

### P3 Decrypt Decode Result Taxonomy UI Packet

Current UI state:

- File, Audio, and Video media rows expose a local `Taxonomy` packet for plain and encrypted media.
- Live result references are limited to existing `MatrixRequest::FetchMedia` image/cache reads, confirmed `MatrixRequest::SaveMedia` Download/Play result mapping, cached Open folder/Replay stale validation plus local OS opener handoff, and confirmed plain-MXC SaveMedia Retry.
- Inline playback, decrypt/decode, codec fallback, background queue, delivery/read receipt, and stale inline/decrypt local-file result slots stay local/not-wired.

Blocked result taxonomy:

- `playback_session_id`: `not_assigned`
- `playback_progress_result`: `playhead`, `buffered`, `completed`, `failed`, `stale`, `not_wired`
- `inline_player_result`: `opened`, `paused`, `seeked`, `failed`, `stale`, `not_wired`
- `decrypt_operation_id`: `not_assigned`
- `decrypt_result`: `decrypted`, `missing_key`, `unsupported`, `failed`, `stale`, `not_wired`
- `decode_result`: `decoded_image`, `decoded_audio`, `decoded_video`, `unsupported_codec`, `failed`, `stale`, `not_wired`
- `codec_fallback_result`: `transcoded`, `captions_loaded`, `quality_switched`, `failed`, `stale`, `not_wired`
- `background_queue_result`: `queued`, `resumed`, `cancelled`, `failed`, `stale`, `not_wired`
- `delivery_receipt_result`: `not_wired`
- `cached_file_stale_result`: cached Open folder/Replay validation only; broader inline/decrypt/queue stale policy not built.

## P10 Voice Recorder Contract

live_wiring: `matrix_sendattachment_voice_audio_file_retry_review_opener_dropcleanup_selected_audio_waveform_codec_live;recorder_capture_upload_blocked`

Required fields:

- `permission_state`
- `privacy_entitlement_state`
- `audio_session_id`
- `capture_file_hash`
- `duration_ms`
- `sample_rate`
- `channel_count`
- `codec`
- `waveform_bucket_count`
- `transcription_state`
- `review_player_state`
- `cleanup_state`
- `upload_queue_item_id`

Blocked controls requiring contract:

- Microphone permission.
- Recorder/audio session.
- Capture file lifecycle.
- Recorder waveform/timer.
- Codec conversion.
- Transcription.
- Inline review player.
- Mobile picker captured upload.

Acceptance rules:

- Permission denied, entitlement missing, recorder failure, and upload failure must be distinct.
- Captured files must have cleanup confirmation.
- Review Drop must name whether it deletes a pending file, clears UI state, cancels upload, or all three.

### P10 Permission Capture Upload Result Taxonomy UI Packet

Current local UI packet:

- `VOICE_MESSAGE_RECORDER_RESULT_TAXONOMY_PACKET_EVIDENCE`
- Visible `Taxonomy` control in the voice capture lifecycle row.

Live result references named by the UI:

- Confirmed desktop audio review `MatrixRequest::SendAttachment`.
- `Timeline::send_attachment().use_send_queue()`.
- PositiveConfirmationModal-gated failed-handoff Retry.
- Selected-audio bounded WAV duration/codec/waveform analysis.
- Review Play local system-opener handoff.
- Drop pending-audio local cleanup.

Blocked/not-wired result slots:

- `microphone_permission_operation_id`: not assigned.
- `privacy_entitlement_result`: not wired.
- `audio_session_id`: not assigned.
- `recorder_session_id`: not assigned.
- `capture_file_identity`: not assigned.
- `waveform_timer_result`: not wired.
- `codec_transcription_result`: not wired.
- `review_player_result`: not wired.
- `mobile_picker_share_result`: not wired.
- `captured_upload_queue_item_id`: not assigned.
- `delivery_result`: not wired.
- `stale_capture_result`: not wired.
- `retry_cancel_result`: not wired.
- `audit_redaction`: raw path, microphone buffer, and transcript data redacted.

Forbidden before backend adapter promotion:

- Microphone permission request.
- Privacy entitlement mutation.
- Audio session activation.
- Platform recorder start.
- Temporary recording write.
- Recorder waveform/timer capture.
- Codec conversion or transcription service call.
- Inline review player.
- Mobile picker/share-sheet handoff.
- Captured upload queue mutation.
- Extra `MatrixRequest::SendAttachment` beyond confirmed desktop review send and confirmed failed-handoff Retry.

## P11 Avatar Source And Editor Contract

live_wiring: `matrix_uploadavatar_setavatar_some_retry_setavatar_none_direct_mxc_setavatar_thumbnail_full_pixel_decode_live;source_cropper_camera_editor_thumbnail_artifact_blocked`

Required fields:

- `source_kind`: `file`, `camera`, `photo_library`, or `direct_mxc`
- `source_path_hash`
- `source_mxc`
- `crop_box`
- `aspect_ratio`
- `rotation_degrees`
- `zoom`
- `thumbnail_artifact_hash`
- `transformed_image_hash`
- `upload_mxc`
- `set_avatar_status`

Blocked controls requiring contract:

- Cropper transform.
- Camera permission and capture.
- Photo-library permission and selection.
- Persistent thumbnail artifact.
- Editor transform.
- Transformed-image upload and set-avatar mapping.

Acceptance rules:

- Header-only decode, bounded pixel decode, persistent thumbnail artifact, and uploaded avatar MXC must remain distinct.
- Camera/photo-library permission denial must not be treated as file picker cancel.
- SetAvatar(Some), direct MXC SetAvatar(Some), and SetAvatar(None) delete must remain separate result paths.

### P11 Source Cropper Camera Editor Artifact Result Taxonomy UI Packet

Current local UI packet:

- `ACCOUNT_AVATAR_UPLOAD_SOURCE_EDITOR_RESULT_TAXONOMY_PACKET_EVIDENCE`
- Visible `Taxonomy` control in the avatar source/preview controls row.

Live result references named by the UI:

- Confirmed desktop `UploadAvatar` plus SDK `Account::set_avatar_url(Some)`.
- Confirmed failed-state `UploadAvatar` Retry.
- Direct MXC `SetAvatar(Some)` plus confirmed failed-state Retry.
- `SetAvatar(None)` delete.
- Selected-file metadata and source-path clipboard.
- Bounded in-memory Thumbnail/Full-size pixel decode.

Blocked/not-wired result slots:

- `source_identity_operation_id`: not assigned.
- `camera_permission_result`: not wired.
- `photo_library_permission_result`: not wired.
- `camera_capture_result`: not wired.
- `photo_library_selection_result`: not wired.
- `crop_box_result`: not wired.
- `aspect_rotate_zoom_result`: not wired.
- `editor_transform_result`: not wired.
- `persistent_thumbnail_artifact_id`: not assigned.
- `transformed_image_hash`: not assigned.
- `transformed_upload_result`: not wired.
- `transformed_set_avatar_result`: not wired.
- `mobile_capture_result`: not wired.
- `stale_source_result`: not wired.
- `retry_cancel_result`: not wired.
- `audit_redaction`: raw path, camera buffer, thumbnail artifact, and transform data redacted.

Forbidden before backend adapter promotion:

- Camera/photo-library permission request.
- Camera capture or photo-library picker handoff.
- Cropper/editor transform.
- Persistent thumbnail artifact generation.
- Transformed image write.
- Transformed `UploadAvatar`.
- Transformed `SetAvatar(Some)`.
- Account/profile mutation beyond existing confirmed `UploadAvatar`, direct `SetAvatar(Some)`, and `SetAvatar(None)` paths.

## Verification Requirements

Before any Wave 2 blocked control is promoted to live:

- Add focused tests for the new request/result/error/retry/cancel shape.
- Re-run the touched focused module tests.
- Re-run `cargo test --lib hepta_telegram_base_ -- --nocapture`.
- Re-run `scripts/hepta-native-queue-media-permission-contract-gate.sh`.
- Re-run Native fixture visual smoke.
- Re-run combined `scripts/hepta-ui-product-readiness-gate.sh`.
- Update backend handoff `live_wiring` strings only after the new live path is actually wired and proven.
