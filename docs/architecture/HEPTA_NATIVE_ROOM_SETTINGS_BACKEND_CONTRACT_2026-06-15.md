# Hepta Native Room Settings Backend Contract

Date: 2026-06-15
Status: backend contract required / UI evidence complete
Gap id: `room_settings`
Source readiness artifact: `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615`

Machine markers:

- `HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_READY:true`
- `HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_DATE:2026-06-15`
- `HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_GAP_ID:room_settings`
- `HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_LIVE_WIRING:matrix_room_settings_power_member_server_refresh_name_topic_alias_avatar_history_joinrule_tombstone_retry_live;power_member_blocked`
- `HEPTA_NATIVE_ROOM_SETTINGS_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation`

## UI Evidence Already Complete

The UI lane has already shipped and machine-checked:

- Read-only room settings surface and local clipboard rows for name/id, identity, permissions, and members.
- Server-backed refresh for `GetRoomPowerLevels` and `GetRoomMembers`.
- Confirmed live writes for room name, topic, canonical alias, room avatar upload/remove, history visibility, join rule, and tombstone/replacement.
- Confirmed failed-state retry for the live room-state write set.
- Room-state mutation request snapshot, field mutation drilldown packet, typed room-state mutation/result packet, power/member result taxonomy packet, and field/refresh detail controls.

The remaining work is not more room settings UI scaffolding. It is backend contract and adapter work for power-level mutation and member moderation.

## Promotion Boundary

Do not wire power-level writes or member moderation until the backend contract provides request, result, error, retry, permission-denial, source-hash, and stale-room semantics.

Blocked until contract:

- `m.room.power_levels` mutation.
- Member kick/ban/unban or other moderation mutation.
- Permission denial and insufficient-power result handling.
- Stale room-state recovery after a conflicting room-state update.
- Rollback or refresh display after partial failure.

Forbidden as a side effect of this contract:

- Gateway/runtime/provider-auth calls.
- Telegram delivery mutation.
- Account/profile mutation.
- Message send/edit/redact.
- Unconfirmed room-state or membership writes.
- Automatic retry without cached request identity and confirmation.

## Contract Shapes

### Room Identity And Refresh Result

This is already live in the UI lane and remains the read baseline.

Required fields:

- `room_id`
- `room_display_name`
- `canonical_alias`
- `join_rule`
- `history_visibility`
- `avatar_mxc`
- `power_levels_source_hash`
- `members_source_hash`
- `refreshed_at_ms`
- `status`: `loaded`, `failed`, or `stale`
- `error_kind`
- `error_message`

Acceptance rules:

- A mutation preflight must reference the source hash it was based on.
- A stale refresh must not erase the last known loaded identity.
- Member and power refresh results must stay separate, because either read can fail independently.

### Existing Room-State Mutation Result

This is already live for name, topic, canonical alias, avatar, history visibility, join rule, and tombstone.

Required fields:

- `room_id`
- `field`
- `requested_value`
- `previous_value`
- `result_value`
- `request_id`
- `source_hash`
- `submitted_at_ms`
- `completed_at_ms`
- `status`: `submitted`, `applied`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- The result must echo the field and requested value.
- Retry must be field-specific and must require `PositiveConfirmationModal`.
- Stale results must trigger a refresh prompt rather than overwriting newer local state.

### Power-Level Mutation Result

Required request fields:

- `room_id`
- `target_user_id`
- `target_role`
- `previous_power_level`
- `requested_power_level`
- `reason`
- `power_levels_source_hash`
- `request_id`

Required result fields:

- `room_id`
- `target_user_id`
- `applied_power_level`
- `previous_power_level`
- `power_levels_event_id`
- `power_levels_source_hash_before`
- `power_levels_source_hash_after`
- `status`: `applied`, `failed`, `permission_denied`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- The backend must report whether the current user had permission to change the target level.
- The result must include the before and after power-level source hashes or an equivalent generation.
- A stale source hash must block the write and require refresh before retry.
- The UI must not infer success from the requested role label alone.

### Member Moderation Result

Required request fields:

- `room_id`
- `target_user_id`
- `action`: `kick`, `ban`, `unban`, or `invite`
- `reason`
- `membership_source_hash`
- `power_levels_source_hash`
- `request_id`

Required result fields:

- `room_id`
- `target_user_id`
- `action`
- `membership_before`
- `membership_after`
- `event_id`
- `membership_source_hash_before`
- `membership_source_hash_after`
- `status`: `applied`, `failed`, `permission_denied`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- Permission denial must be distinct from transport/server failure.
- Ban and unban must not be collapsed into a generic member update.
- The result must name the resulting membership state.
- The UI must require confirmation for every moderation mutation, including retry.

### Permission Denial Contract

Required fields:

- `room_id`
- `target_user_id`
- `attempted_action`
- `required_power_level`
- `current_user_power_level`
- `target_user_power_level`
- `denial_source`
- `message`
- `retry_eligible`

### Power/Member Result Taxonomy UI Packet

Current UI evidence records the blocked promotion taxonomy before any power/member live write is added.

Required taxonomy slots before promotion:

- `operation_id_slot`: backend-assigned before any write.
- `power_levels_result`: `applied`, `permission_denied`, `forbidden`, `stale_baseline`, `invalid_delta`, or `failed`.
- `member_moderation_result`: action-specific `invite`, `kick`, `ban`, and `knock` result mapping.
- `retry_policy`: confirmation plus backend request id and source hash.
- `cancel_policy`: local dismiss until backend cancel semantics exist.
- `audit_redaction`: no access token, raw moderation reason, invite address, profile PII, or full power event JSON.

Acceptance rules:

- Permission denial is a terminal result unless a refresh shows changed power levels.
- The UI may show the denial, but must not keep a retry button enabled unless the backend marks it retryable.
- Denial copy must avoid implying that the write was attempted if preflight blocked it before submission.

## Retry And Stale-Room Rules

Every retryable write must preserve:

- request kind
- room id
- target identity
- normalized requested value or action
- source hash or generation
- request id
- last error
- confirmation text

Every retry must require `PositiveConfirmationModal`. Automatic retry stays blocked.

If source hash mismatches, the UI must ask for refresh before any retry.

## Verification Requirements

Before power-level or member moderation is promoted to live:

- Add focused tests for the new request/result/error/retry shape.
- Re-run `cargo test --lib room_settings -- --nocapture`.
- Re-run `cargo test --lib hepta_telegram_base_ -- --nocapture`.
- Re-run `scripts/hepta-native-room-settings-backend-contract-gate.sh`.
- Re-run Native fixture visual smoke.
- Re-run combined `scripts/hepta-ui-product-readiness-gate.sh`.
- Update the backend handoff `live_wiring` string only after the new live path is actually wired and proven.
