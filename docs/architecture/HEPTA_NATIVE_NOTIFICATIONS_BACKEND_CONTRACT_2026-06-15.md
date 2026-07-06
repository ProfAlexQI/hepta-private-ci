# Hepta Native Notifications Backend Contract

Date: 2026-06-15
Status: backend contract required / UI evidence complete
Gap id: `notifications`
Source readiness artifact: `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615`

Machine markers:

- `HEPTA_NATIVE_NOTIFICATIONS_BACKEND_CONTRACT_READY:true`
- `HEPTA_NATIVE_NOTIFICATIONS_BACKEND_CONTRACT_DATE:2026-06-15`
- `HEPTA_NATIVE_NOTIFICATIONS_BACKEND_CONTRACT_GAP_ID:notifications`
- `HEPTA_NATIVE_NOTIFICATIONS_BACKEND_CONTRACT_LIVE_WIRING:matrix_room_notification_mode_keyword_mutation_retry_keyword_list_default_mode_read_write_pusher_status_live;timed_pusher_sound_raw_rules_blocked`
- `HEPTA_NATIVE_NOTIFICATIONS_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation`

## UI Evidence Already Complete

The UI lane has already shipped and machine-checked:

- Confirmed room notification mode writes for All, Mentions, and Mute through `MatrixRequest::SetRoomNotificationMode`.
- Confirmed failed-state retry for room notification mode writes.
- Loaded mode and attention clipboard/metadata.
- Live enabled keyword-list read through `MatrixRequest::GetNotificationKeywordRules`.
- Confirmed keyword Add/Remove writes through `MatrixRequest::SetNotificationKeywordRule`.
- Live default room-mode read/write through `MatrixRequest::GetDefaultRoomNotificationMode` and `MatrixRequest::SetDefaultRoomNotificationMode`.
- Live pusher/device capability read through `MatrixRequest::GetNotificationPusherStatus`.
- Local schedule snapshot, notification rule packet, typed account-data/pusher
  contract packet, local timed/global/pusher result taxonomy packet, and
  advanced/detail/result/preflight controls.

The remaining work is not more UI scaffolding. It is backend contract and adapter work.

## Promotion Boundary

Do not wire the remaining controls until the backend contract provides request, result, error, retry, source-hash, and stale-target semantics for each write.

Blocked until contract:

- Timed mute account-data writes.
- Raw/global notification preference edits beyond the SDK keyword/default APIs.
- Pusher/device configuration writes.
- Sound and badge tuning.
- Cross-device refresh and fanout semantics.

Forbidden as a side effect of this contract:

- Gateway/runtime/provider-auth calls.
- Telegram delivery mutation.
- Room-state or membership mutation.
- Account/profile mutation outside the explicit notification account-data/pusher scope.
- Unconfirmed notification writes.
- Automatic retry without the cached request and confirmation guard.

## Contract Shapes

### Room Mode Result

This is already live in the UI lane and remains the reference shape.

Required fields:

- `room_id`
- `requested_mode`
- `previous_mode`
- `result_mode`
- `request_id`
- `source_hash`
- `submitted_at_ms`
- `completed_at_ms`
- `status`: `submitted`, `applied`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- The result must echo the requested room id and requested mode.
- A stale result must not overwrite a newer local requested mode.
- Retry is available only when a cached room id and mode are present and the user confirms.

### Keyword Rule Mutation Result

This is already live for Add/Remove and becomes the model for richer rule writes.

Required fields:

- `keyword`
- `operation`: `add` or `remove`
- `normalized_keyword`
- `enabled_keywords_before`
- `enabled_keywords_after`
- `request_id`
- `source_hash`
- `status`: `submitted`, `applied`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- Empty or whitespace-only keywords are rejected before request submission.
- Keyword normalization must be echoed in the result.
- The UI may refresh the keyword-list read after success, but must not infer success from a local input clear alone.

### Timed Mute Contract

Required request fields:

- `room_id`
- `duration_seconds`
- `starts_at_ms`
- `expires_at_ms`
- `mode_during_mute`
- `restore_mode`
- `request_id`
- `source_hash`

Required result fields:

- `room_id`
- `requested_expires_at_ms`
- `applied_expires_at_ms`
- `restore_mode`
- `account_data_event_type`
- `status`: `scheduled`, `applied`, `expired`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- Timed mute must name whether it is implemented by account-data rule edits, scheduled local restore, or a backend scheduler.
- Expiration must be visible as a result, not only as a local countdown.
- Restore must be idempotent and stale-safe.

### Global Preferences And Raw Rules

Required request fields:

- `rule_id`
- `rule_kind`
- `enabled`
- `actions_before`
- `actions_after`
- `scope`: `global`, `room`, or `keyword`
- `request_id`
- `source_hash`

Required result fields:

- `rule_id`
- `rule_kind`
- `applied_enabled`
- `applied_actions`
- `account_data_generation`
- `status`: `applied`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- The result must identify the account-data generation or equivalent source hash.
- The UI must not display raw-rule writes as applied until the result echoes the applied rule.
- Unsupported rule kinds must stay local blocked with explicit copy.

### Pusher Device Write Contract

Required request fields:

- `pusher_id`
- `device_id`
- `app_id`
- `pushkey_hash`
- `enabled`
- `sound`
- `badge`
- `request_id`
- `source_hash`

Required result fields:

- `pusher_id`
- `device_id`
- `enabled`
- `sound`
- `badge`
- `server_generation`
- `status`: `applied`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- The UI must display pushkey only as a redacted hash.
- Sound/badge writes must be tied to a pusher/device identity, not a room row alone.
- Cross-device refresh must say whether the current device, all devices, or a backend pusher list was refreshed.

### Current Result Taxonomy Packet

The UI now has a local-only taxonomy packet for the blocked timed/global/pusher
result surface. It records:

- `operation_id_slot`: `not_assigned` for timed mute, raw account-data,
  pusher/device, and sound/badge writes.
- Live result references: the existing confirmed room-mode, keyword, and
  default-mode SDK paths only.
- Timed mute states: scheduled, applied, expired, failed, and stale are
  `not_wired`.
- Raw account-data states: applied, failed, and stale are `not_wired`.
- Pusher/device states: enabled, disabled, failed, and stale are `not_wired`.
- Sound/badge states: applied, failed, and stale are `not_wired`.
- Retry/cancel/source-hash policy: confirmation required, backend request id
  required, cancel is local dismissal only, and backend generation is required
  before raw-rule or pusher writes.
- Audit redaction: no pushkey, token, gateway secret, or raw pusher payload is
  copied into the local packet.

The packet sends no notification account-data read/write outside SDK
keyword/default APIs, no pusher mutation, no push gateway/device configuration,
no timed mute write, no sound/badge tuning, no retry automation, and no
gateway/runtime/provider/auth/channel delivery call.

## Retry And Source Hash Rules

Every retryable write must preserve:

- request kind
- target identity
- normalized payload
- source hash or generation
- request id
- last error
- confirmation text

Every retry must require `PositiveConfirmationModal`. Automatic retry stays blocked.

## Verification Requirements

Before any blocked notification mutation is promoted to live:

- Add focused tests for the new request/result/error/retry shape.
- Re-run `cargo test --lib hepta_telegram_base_ -- --nocapture`.
- Re-run `scripts/hepta-native-notifications-backend-contract-gate.sh`.
- Re-run Native fixture visual smoke.
- Re-run combined `scripts/hepta-ui-product-readiness-gate.sh`.
- Update the backend handoff `live_wiring` string only after the new live path is actually wired and proven.
