# Hepta Native Productization Blocker Rollup - 2026-06-15

Status: UI productization rollup ready / backend-owned blockers remain
Owner lane: hepta-ui
Latest readiness artifact: `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615`
True-window reference artifact: `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.contract-waves-true-window-20260615`
True-window smoke screenshot reference: `/Users/qianqi/.openclaw/tmp/hepta-native-window-smoke.glass-contract-20260615-2301`

Machine markers:

- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_READY:true`
- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_DATE:2026-06-15`
- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_STATUS:ui-product-readiness-local-ready_backend-owned-blockers`
- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_SCOPE:apps/hepta-native,apps/hepta-control-ui,ui-fixtures,packaging,screenshot-gates`
- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_SOURCE_READINESS:/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615`
- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_LATEST_READINESS:/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.mention-taxonomy-20260615`
- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_TRUE_WINDOW_REFERENCE:/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.contract-waves-true-window-20260615`
- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_TRUE_WINDOW_SMOKE_REFERENCE:/Users/qianqi/.openclaw/tmp/hepta-native-window-smoke.glass-contract-20260615-2301`
- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_NATIVE_GLASS_CONTRACT:computed-style-desktop-mobile`
- `HEPTA_NATIVE_PRODUCTIZATION_BLOCKER_ROLLUP_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation`

## Evidence Snapshot

The UI lane has local, fixture-backed, and gate-backed evidence for the Native
Telegram/Matrix product surface:

- Latest combined readiness is ready with 3 Control UI screenshots, 28 Native
  fixture screenshots, 13 key screenshots, packaging ready, backend contract
  gates ready, and non-base edge gates ready.
- The 12 base gaps remain `partial_live_backend_contract_remaining`, with
  `ui_lane_state=complete`, `next_owner_lane=backend_contract`, and explicit
  side-effect boundaries.
- Wave 1 high-risk writes, Wave 2 queue/platform permissions, and Wave 3
  remote result/rich UX all have local result taxonomy packets where the UI
  can prove what is live, what is blocked, and what must stay backend-owned.
- Native tempered-glass styling is gate-backed by desktop and mobile browser
  computed-style probes: graphite dark body, translucent panels, glass
  hairlines, backdrop blur, teal active/send accent, no horizontal overflow,
  and zero light-surface regressions.
- Non-base edge coverage is ready for Location continuous update controls, TSP
  destructive/import/cancel/association packets, Spaces/room membership edge
  packets, and Edit/Poll detail packets.
- The latest default readiness does not force true-window capture, but the
  true-window reference remains ready through retained desktop/mobile Makepad
  window screenshots when the older aggregate tmp readiness folder is absent.

## Base Gap Closure Matrix

Remaining backend-owned blockers are grouped by wave below.

Wave 1 high-risk writes remain UI-closed and backend-owned:

Wave 1 high-risk write blockers remain backend-owned:

- P4 notifications: timed mute, raw/global account-data rule writes, pusher
  writes, sound/badge tuning, and cross-device result fanout.
- P5 room_settings: power-level writes and member moderation writes.
- P7 message_report_send: moderation queue, policy, reviewer, appeal,
  enforcement, redaction/kick/ban/ignore workflows.
- P12 account_management: password/SSO, cross-session revoke/trust, device
  delete/trust, and account/session actions beyond confirmed display-name and
  current-device rename paths.

Wave 2 queue/progress and platform permissions remain backend-owned:

Wave 2 queue/progress and platform blockers remain backend-owned:

- P2 file_upload_send: accepted queue retry/resume/cancel/reorder/remove,
  progress subscription, delivery receipt mapping, and cross-platform picker
  truth.
- P3 media_download_playback: inline decrypt/decode/playback, codec fallback,
  captions, playback progress, and queue controls.
- P10 voice_message_send: microphone permission, recorder/audio session,
  capture file lifecycle, waveform capture, codec/transcription, inline review
  player, mobile picker, and captured upload queue.
- P11 account_avatar_upload: source identity, cropper/editor transforms,
  camera/photo-library permissions, persistent thumbnail artifacts, mobile
  capture, and transformed-image upload/set-avatar result mapping.

Wave 3 remote result adapters and rich UX remain backend-owned:

Wave 3 remote result/rich UX blockers remain backend-owned:

- P1 message_search: remote date/pins/scope, sort, cross-room search, full
  result cursor/rendering, and stale-query behavior.
- P6 matrix_link_resolution: route target identity, richer event-context
  windows, non-current-room context, via echo, access denial, and stale target
  behavior.
- P8 message_edit_history: remote full-history result adapter, event context,
  replacement/original source reconciliation, and server-backed full-body diff.
- P9 mention_picker_send: rich picker result promotion, richer directory UI,
  duplicate-name disambiguation, remote hover/profile adapter, rich attachment
  editor, edit/reply mention payload rewrite, stale-token/source-hash guard,
  and audit redaction.

## Non-Base Edge Closure Matrix

Non-base edge closure is local evidence only:

The non-base UI lane evidence is locally closed but not promoted to backend
behavior:

- Location Start/Stop device updates are visible local controls around
  `LocationRequest::StartUpdates` and `LocationRequest::StopUpdates`; they do
  not create live-location Matrix events.
- TSP Delete/Import/pending cancel/association cancel-remove packets are
  visible local preflight/result taxonomy evidence; destructive filesystem,
  wallet database, and TSP cancel/remove behavior remains unwired.
- Spaces and room membership edge packets cover re-knock/cancel-prior,
  selected-room removed/rejoin, Load More pagination, and unread/mention
  aggregates without promoting new membership or pagination requests.
- Edit/Poll detail packets cover edit attachments, edit mention payload,
  poll answer edits, save mapping, and retry/error drilldowns without
  promoting new attachment, poll-answer, or edit/rewrite adapters.

## Readiness Claims

Default readiness claim:

- Safe to claim local UI fixture readiness, packaging readiness, backend
  contract gate readiness, non-base edge gate readiness, and Native
  desktop/mobile tempered-glass fixture styling for the checked artifacts.

Not safe to claim without a fresh or referenced true-window pass:

- Public/external visual readiness for live desktop/mobile windows.
- Release/distribution readiness that depends on a current unlocked-screen
  true-window capture.

Known gate behavior:

- The isolated Native packaging gate may cold-compile long enough to time out
  on shorter waits. The gate default startup wait is 900 seconds, and the
  report records `runner.startup_timeout_sec` so cold-compile false failures
  can be separated from product failures.

## Future Plan

1. Backend contract lane should implement typed adapters only from the listed
   blocker groups, preserving confirmation, source-hash, stale-target,
   idempotency, retry/cancel, audit redaction, and side-effect boundaries.
2. UI lane should keep fixture, screenshot, packaging, and contract gates
   current while avoiding live promotion of backend-owned adapters.
3. Before external/public readiness claims, run combined readiness plus
   true-window smoke with an unlocked desktop and record the artifact.
4. If continuing UI-local work, prefer visual QA, compact-density polish,
   screenshot regression coverage, or additional non-base local evidence
   packets over widening Matrix/gateway/runtime behavior.

## Forbidden Side Effects

Promotion Rules:

- No backend-owned blocker may be promoted from a local taxonomy packet into a
  live adapter unless its typed contract, operation id, source hash,
  stale-target guard, retry/cancel policy, and audit-redaction behavior are
  implemented and covered by the corresponding gate.
- No public, external, or distribution readiness claim may use fixture-only
  evidence without combined readiness, packaging, side-effect boundary checks,
  and either a fresh true-window pass or an explicitly referenced true-window
  artifact.
- No Native glass/visual refresh claim may pass rollup unless combined
  readiness includes the desktop/mobile computed-style glass contract and zero
  light-surface failures.

The rollup and its gate do not require or permit Matrix login, gateway calls,
provider invocation, Telegram/channel delivery, external mutation, live
runtime mutation, destructive filesystem changes, account/profile mutation,
room-state mutation, membership mutation, or backend adapter promotion.
