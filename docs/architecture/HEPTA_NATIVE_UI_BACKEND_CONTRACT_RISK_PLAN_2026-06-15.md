# Hepta Native UI Backend Contract Risk Plan

Date: 2026-06-15
Status: UI lane complete / backend contract handoff required
Owner lane: hepta-ui
Source readiness artifact: `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615`

Machine markers:

- `HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_READY:true`
- `HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_DATE:2026-06-15`
- `HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_SOURCE_READINESS:/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615`
- `HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_SCOPE:apps/hepta-native,apps/hepta-control-ui,ui-fixtures,packaging,screenshot-gates`
- `HEPTA_NATIVE_UI_BACKEND_CONTRACT_RISK_PLAN_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation`

## Current State

The Native UI base-gap queue is no longer blocked on UI surface work. All 12 base gaps are `ui_lane_state=complete` and `ui_contract_ready`, with combined readiness evidence:

- Control UI browser screenshots: 3
- Native fixture screenshots: 28
- Native packaging gate: ready
- Native base gap backend handoff: 12/12
- External side effects: none, except the local loopback server spawned by the readiness gate

Remaining work is backend contract and adapter work. The UI lane should not promote these remaining gaps by widening runtime, gateway, provider auth, Telegram delivery, or unguarded mutation paths.

## Critical Risk Buckets

1. Write-side safety and authority

   Covers room settings power/member moderation, notification account-data and pusher writes, account/session writes, moderation actions, avatar transforms, and device/session mutations. These require confirmation, idempotency, stale-target, source-hash, retry, and audit contracts before additional live writes are wired.

2. Queue and progress truth

   Covers accepted attachment queues, media playback/decrypt/decode progress, voice recording/upload progress, background persistence, pause/resume/cancel, and delivery receipts. The risk is presenting controls whose SDK/runtime result semantics are not yet contractually stable.

3. Remote result shape and pagination

   Covers Matrix `/search` remote date/pins/full scope, Matrix link event-context route adapters, edit-history full remote result adapters, server-authored full-body diffs, and source reconciliation. The risk is drifting from Matrix result shape or losing cursor/stale-source guarantees.

4. Cross-device and platform permissions

   Covers notification pusher devices, mobile picker/share sheet, microphone/camera/photo-library permission, account portal/password/SSO, and cross-session revoke/trust. The risk is platform-specific permission or cross-device state that cannot be truthfully proven by fixture-only evidence.

5. Product readiness claims

   True-window smoke is optional and currently not run by default. Public/external readiness must not be claimed from fixture screenshots alone; keep fixture readiness, packaging readiness, and true-window readiness separate.

## Backend Contract Handoff Matrix

### P1 message_search

- UI evidence: loaded timeline search, Matrix `/search` first page, sender/media filters, older pagination, retry, cached current-room context pagination, parsed event_context previews, source-only `FetchEventSource`, loaded result actions, remote date/pins/scope/full-result taxonomy packet.
- live_wiring: `matrix_server_search_v3_search_next_batch_retry_context_pagination_sender_media_url_filter_loaded_scope_filters_loaded_result_actions_server_result_context_preview_source_refetch_live;remote_date_pins_scope_full_result_blocked`
- Backend contract blocker: remote date/pins/scope, richer result cursor, full remote result rendering, remote-result taxonomy slots.
- Next adapter plan: define a remote search result envelope with cursor ownership, filter echo, source availability, event-context previews, and stale-query discard.

### P2 file_upload_send

- UI evidence: live single-file `SendAttachment`, SDK timeline progress/error/sent, `AbortLocalSend`, accepted queue snapshots, queue drilldown, typed SDK queue contract, accepted queue/progress/result taxonomy packet.
- live_wiring: `matrix_sdk_send_attachment_queue_live;failed_handoff_retry_sendattachment_live;timeline_progress_sendhandle_cancel_result_live;cross_platform_queue_controls_blocked`
- Backend contract blocker: accepted-queue progress, delivery, retry, cancel, reorder/remove, cross-platform picker/share sheet.
- Next adapter plan: define stable queue item identity, local echo identity, cancel/retry eligibility, progress units, and delivery receipt mapping before adding live queue controls.

### P3 media_download_playback

- UI evidence: live `FetchMedia`, confirmed `SaveMedia`, retry, system opener outcome, cached Open folder/Replay stale validation, cached file metadata snapshot, decrypt/decode/opener/queue result taxonomy packet.
- live_wiring: `matrix_sdk_fetchmedia_save_media_retry_opener_openfolder_replay_live;inline_decrypt_queue_controls_blocked`
- Backend contract blocker: inline decrypt/decode, playback progress, codec fallback, queue controls.
- Next adapter plan: define playback state transitions, decrypt result, decode result, unsupported codec result, cached file invalidation, and opener-vs-inline ownership.

### P4 notifications

- UI evidence: confirmed notification mode writes, retry, mode clipboard, live keyword-list read, confirmed keyword Add/Remove, live default room-mode read/write, pusher/device capability read, local schedule snapshot.
- live_wiring: `matrix_room_notification_mode_keyword_mutation_retry_keyword_list_default_mode_read_write_pusher_status_live;timed_pusher_sound_raw_rules_blocked`
- Backend contract blocker: full account-data rule edits, pusher writes, sound/badge, timed mute, result fanout.
- Next adapter plan: define rule identity, pusher identity, raw rule diff, pusher write result, timed expiration semantics, and cross-device refresh policy.

### P5 room_settings

- UI evidence: read-only settings, clipboard rows, server-backed refresh, confirmed name/topic/alias/avatar/history/join-rule/tombstone writes, retry, mutation packet, typed room-state contract, and power/member result taxonomy packet.
- live_wiring: `matrix_room_settings_power_member_server_refresh_name_topic_alias_avatar_history_joinrule_tombstone_retry_live;power_member_blocked`
- Backend contract blocker: power-level writes and member moderation writes.
- Next adapter plan: define preflight power diff, membership target guard, permission check result, stale room-state hash, retry semantics, and rollback display.

### P6 matrix_link_resolution

- UI evidence: loaded jump/source, current-room event pagination, compact preview, source-only preview event fetch, cached Server `PreviewMatrixLinkTarget` refresh, retry, confirmed join/knock/invite, browser opener, route packets, route/event-context result taxonomy packet.
- live_wiring: `matrix_link_compact_preview_retry_server_context_refresh_current_room_event_pagination_source_fetch_browser_opener_room_or_alias_join_knock_current_room_user_invite_live;event_context_route_adapter_blocked`
- Backend contract blocker: richer event context and route/result adapter.
- Next adapter plan: define route target identity, Matrix event-context request/result/error envelope, via server echo, source availability, non-current-room guard, browser/join/knock/invite separation, and source-hash/stale-target taxonomy before route promotion.

### P7 message_report_send

- UI evidence: confirmed `ReportContent`, result/retry, moderation packet, reviewer packet, typed moderation workflow contract, workflow result taxonomy packet, status clipboard, loaded/source-fetch source modal.
- live_wiring: `matrix_report_content_result_retry_loaded_source_fetch_modal_live;moderation_workflow_blocked`
- Backend contract blocker: moderation service actions beyond report content.
- Next adapter plan: define queue id, reviewer assignment, policy lookup, appeal, enforcement action, redaction/kick/ban/ignore result, and audit trail before exposing live workflow controls.

### P8 message_edit_history

- UI evidence: live paginated `FetchEditHistory`, retry, synthetic full snapshot, loaded side-by-side preview/full-body diff, diff clipboard, replacement source modal, original source fallback, remote full-history/source result taxonomy packet.
- live_wiring: `matrix_edit_history_paginated_relations_retry_local_full_snapshot_modal_loaded_original_replacement_source_side_by_side_preview_full_body_diff_live;remote_full_history_result_adapter_context_source_reconciliation_server_backed_full_body_diff_blocked`
- Backend contract blocker: remote full-history result adapter, event context, source reconciliation, server-backed full-body diff payload.
- Next adapter plan: define relation page envelope, replacement/original source reconciliation, source-hash acceptance, server-authored full-body diff payload, blocked result taxonomy, and stale-target guards.

### P9 mention_picker_send

- UI evidence: cached suggestions, completed-token pill tray, `SendMessage/add_mentions`, attachment caption mentions, live user-directory search, result promotion, local hover-card, typed mention contract, remote hover/profile/disambiguation/edit-reply result taxonomy packet.
- live_wiring: `matrix_sendmessage_attachment_caption_mentions_pill_tray_user_directory_search_result_promotion_hover_card_snapshot_live;rich_disambiguation_remote_hover_edit_payload_scopes_blocked`
- Backend contract blocker: rich picker, duplicate-name disambiguation, remote hover/profile adapter, rich attachment editor, edit/reply mention rewrite scopes.
- Next adapter plan: define mention token source, display-name collision result, remote profile hover result, edit/reply rewrite rules, attachment-caption parity, blocked result taxonomy, stale-token/source-hash guard, and audit redaction.

### P10 voice_message_send

- UI evidence: live desktop audio `SendAttachment`, retry, audio file handoff, selected-audio WAV metadata/waveform analysis, review Play opener, Drop cleanup, recorder lifecycle packets, permission/capture/upload result taxonomy packet.
- live_wiring: `matrix_sendattachment_voice_audio_file_retry_review_opener_dropcleanup_selected_audio_waveform_codec_live;recorder_capture_upload_blocked`
- Backend contract blocker: microphone permission, recorder session, capture, recorder waveform, codec/transcription, inline review player, mobile picker, captured upload.
- Next adapter plan: define permission result, recording file lifecycle, waveform sampling units, codec/transcription results, review player ownership, and cleanup guarantees.

### P11 account_avatar_upload

- UI evidence: confirmed UploadAvatar and set-avatar URL, retry, direct MXC SetAvatar, delete avatar, file upload, source clipboard, bounded thumbnail/full pixel decode, cropper/editor packets, and source/cropper/camera/editor artifact result taxonomy packet.
- live_wiring: `matrix_uploadavatar_setavatar_some_retry_setavatar_none_direct_mxc_setavatar_thumbnail_full_pixel_decode_live;source_cropper_camera_editor_thumbnail_artifact_blocked`
- Backend contract blocker: source/cropper/camera/editor/thumbnail artifacts and transformed-image handoff result mapping.
- Next adapter plan: define source identity, crop transform, permission/capture result, persistent thumbnail artifact, transformed upload identity, and stale file guard.

### P12 account_management

- UI evidence: live own-device read, all-device directory, retry, display-name mutation, current-device rename, homeserver portal opener, current-device/session clipboard, typed account-session contract, password/SSO/revoke/trust/delete result taxonomy packet.
- live_wiring: `matrix_getowndevice_getdevices_retry_setdisplayname_resubmit_current_device_renamedevice_browser_portal_homeserver_opener_live;password_sso_revoke_trust_cross_session_device_mutations_blocked`
- Backend contract blocker: dedicated account portal, password/SSO, cross-session revoke/trust, device delete/trust, account/profile mutations beyond display name/current-device rename.
- Next adapter plan: define session identity, device trust/delete result, password/SSO route result, portal opener outcome, and cross-session stale guard.

## Execution Waves

Wave 1: high-risk write contracts

- P4 notifications: account-data/pusher/timed mute writes.
- P5 room_settings: power-level and member moderation writes.
- P7 message_report_send: moderation workflow actions.
- P12 account_management: cross-session revoke/trust/delete and password/SSO.

Wave 2: queue/progress and platform permission contracts

- P2 file_upload_send: accepted queue controls and progress truth.
- P3 media_download_playback: inline decrypt/decode/playback queue.
- P10 voice_message_send: microphone/recorder/captured upload.
- P11 account_avatar_upload: cropper/camera/editor transformed upload.

Wave 3: remote result adapters and rich UX contracts

- P1 message_search: remote date/pins/scope and full result cursor plus result taxonomy packet.
- P6 matrix_link_resolution: event-context route/result adapter and route taxonomy packet.
- P8 message_edit_history: remote full-history full-body diff, source reconciliation, and result taxonomy packet.
- P9 mention_picker_send: remote hover/profile/disambiguation and edit/reply scopes plus result taxonomy packet.

## Verification Plan

Every promotion from `partial_live_backend_contract_remaining` must pass:

- focused Rust tests for the touched module
- full `cargo test --lib hepta_telegram_base_ -- --nocapture`
- `bash -n` for touched gates/scripts
- stale marker scan for old blocked/live_wiring phrases
- `git diff --check`
- Native fixture visual smoke
- combined `scripts/hepta-ui-product-readiness-gate.sh`
- a refreshed backend handoff report with side-effect fields explicit

True-window smoke remains separate from fixture readiness. Public or external readiness must require a true-window pass or a named blocked report with local permission details.
