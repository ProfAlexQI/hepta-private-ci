# Hepta Native Account Management Backend Contract

Date: 2026-06-15
Status: backend contract required / UI evidence complete
Gap id: `account_management`
Source readiness artifact: `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615`

Machine markers:

- `HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_READY:true`
- `HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_DATE:2026-06-15`
- `HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_GAP_ID:account_management`
- `HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_LIVE_WIRING:matrix_getowndevice_getdevices_retry_setdisplayname_resubmit_current_device_renamedevice_browser_portal_homeserver_opener_live;password_sso_revoke_trust_cross_session_device_mutations_blocked`
- `HEPTA_NATIVE_ACCOUNT_MANAGEMENT_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation`

## UI Evidence Already Complete

The UI lane has already shipped and machine-checked:

- Live `GetOwnDevice` current-session read/refresh.
- Live `GetDevices` read-only all-device directory.
- Confirmed failed-state `GetDevices` retry.
- Live confirmed `SetDisplayName` profile display-name mutation plus failed-state Save Name resubmit.
- Live confirmed current-device `RenameDevice` / `client.rename_device`.
- Live confirmed Browser/Portal active homeserver system-opener handoff.
- Visible session, device id, verified state, display name, session/source clipboard, current-device request snapshot, preflight controls, session/device drilldown packet, and typed account-session contract packet.
- Local password/SSO/revoke/trust/delete result taxonomy packet that records blocked operation ids, request slots, result slots, stale-session/source-hash requirements, retry/cancel policy, and audit redaction before any destructive/session action is promoted.

The remaining work is not more account UI. It is backend contract and adapter work for dedicated account portal routes, password/SSO actions, cross-session revoke/trust, device delete/trust, and account/profile mutations beyond display name/current-device rename.

## Promotion Boundary

Do not wire password/SSO, cross-session revoke/trust, device delete/trust, or additional account/profile mutations until the backend contract provides request, result, error, retry, source-hash, stale-session, and audit semantics.

Blocked until contract:

- Dedicated account-management portal route beyond confirmed homeserver opener.
- Password change.
- SSO change or SSO session management.
- Cross-session revoke.
- Cross-session trust changes.
- Device delete.
- Device trust changes.
- Account/profile mutations beyond display name and current-device rename.

Forbidden as a side effect of this contract:

- Gateway/runtime/provider-auth calls.
- Telegram delivery mutation.
- Message send/edit/redact.
- Room-state or membership mutation.
- Unconfirmed account/session/device writes.
- Secret or token payload in logs, docs, clipboard, or gate reports.
- Automatic retry without cached request identity and confirmation.

## Contract Shapes

### Current Device Result

This is already live and remains the current-session baseline.

Required fields:

- `user_id`
- `device_id`
- `display_name`
- `is_verified`
- `last_seen_ts_ms`
- `last_seen_ip_redacted`
- `source_hash`
- `refreshed_at_ms`
- `status`: `loaded`, `failed`, or `stale`
- `error_kind`
- `error_message`

Acceptance rules:

- IP address must be redacted or omitted unless an explicit privacy decision allows display.
- A stale result must not overwrite a newer loaded device state.
- Current-device identity must be separate from all-device directory entries.

### All-Device Directory Result

This is already live and remains the cross-device read baseline.

Required fields:

- `user_id`
- `devices`
- `directory_source_hash`
- `refreshed_at_ms`
- `status`: `loaded`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Each device entry must include:

- `device_id`
- `display_name`
- `is_current_device`
- `is_verified`
- `last_seen_ts_ms`
- `last_seen_ip_redacted`

Acceptance rules:

- Failed `GetDevices` retry must require `PositiveConfirmationModal`.
- Device entries must use redacted IP or omit IP.
- The directory result must include a generation/source hash before cross-session mutations are enabled.

### Display Name Result

This is already live for profile display name and remains the profile mutation baseline.

Required fields:

- `user_id`
- `previous_display_name`
- `requested_display_name`
- `result_display_name`
- `request_id`
- `source_hash`
- `status`: `applied`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- Empty and whitespace-only names must be validated before submission.
- Retry must preserve the requested display name and require confirmation.
- A stale result must prompt refresh before retry.

### Current-Device Rename Result

This is already live and remains the device mutation baseline.

Required fields:

- `user_id`
- `device_id`
- `previous_display_name`
- `requested_display_name`
- `result_display_name`
- `request_id`
- `device_source_hash`
- `status`: `applied`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- The backend must confirm the mutation applies only to the current device.
- Retry must preserve device id and requested display name and require confirmation.
- A stale or changed current device id must block retry.

### Account Portal Route Result

The UI has only a confirmed homeserver opener handoff today.

Required fields:

- `homeserver_url`
- `portal_url`
- `route_kind`: `homeserver`, `account_portal`, `password`, `sso`, or `sessions`
- `opened_by`
- `request_id`
- `status`: `opened`, `blocked`, `failed`, or `stale`
- `error_kind`
- `error_message`

Acceptance rules:

- Opening the generic homeserver must stay distinct from opening a dedicated account portal route.
- Password and SSO routes must not be inferred from a generic homeserver opener.
- External opener handoff must require confirmation.

### Password And SSO Action Result

Required request fields:

- `user_id`
- `action`: `password_change`, `sso_start`, `sso_change`, or `sso_disconnect`
- `route_kind`
- `request_id`
- `source_hash`

Required result fields:

- `action`
- `route_url_redacted`
- `session_id_hash`
- `status`: `opened`, `completed`, `cancelled`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- No password, token, SSO code, refresh token, or secret may be stored in UI state, logs, docs, clipboard, or gate reports.
- SSO start/open and SSO completion must be separate result states.
- Cancelled SSO must be visible and not treated as failed mutation.

### Cross-Session Revoke/Trust Result

Required request fields:

- `user_id`
- `target_device_id`
- `action`: `revoke`, `trust`, or `untrust`
- `directory_source_hash`
- `request_id`

Required result fields:

- `target_device_id`
- `action`
- `device_generation_before`
- `device_generation_after`
- `status`: `applied`, `failed`, `permission_denied`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- Revoke and trust are separate actions and must not share a generic success label.
- Current-device revoke must require an explicit extra guard or stay blocked.
- A stale all-device directory generation must require refresh before retry.

### Device Delete/Trust Result

Required request fields:

- `user_id`
- `target_device_id`
- `action`: `delete`, `trust`, or `untrust`
- `directory_source_hash`
- `request_id`

Required result fields:

- `target_device_id`
- `action`
- `display_name_before`
- `status`: `applied`, `failed`, `permission_denied`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- Device delete must echo the deleted device id and previous display name.
- Trust/untrust must echo the resulting trust state.
- Permission denial must be distinct from transport/server failure.

### Password/SSO/Revoke/Trust/Delete Result Taxonomy UI Packet

This is a UI evidence packet, not live wiring.

Required local packet fields:

- `live_result_references`: `GetOwnDevice`, `GetDevices`, `SetDisplayName`, current-device `RenameDevice`, and confirmed Browser/Portal homeserver opener.
- `dedicated_portal_operation_id`: `not_assigned`
- `password_action_operation_id`: `not_assigned`
- `sso_action_operation_id`: `not_assigned`
- `cross_session_revoke_operation_id`: `not_assigned`
- `cross_session_trust_operation_id`: `not_assigned`
- `device_delete_operation_id`: `not_assigned`
- `device_trust_operation_id`: `not_assigned`
- `request_slots`: `not_built` for dedicated portal, password, SSO, revoke, trust, delete, and cross-session device actions.
- `result_taxonomy`: opened/completed/cancelled/applied/permission_denied/failed/stale values are named but `not_wired`.
- `retry_policy`: confirmation gated, backend request id required, directory/source hash required.
- `cancel_policy`: local dismiss only, no backend cancel request.
- `stale_policy`: current-device id plus all-device directory generation required before mutation.
- `audit_redaction`: no password, token, SSO code, refresh token, raw last-seen IP, or device secret.

Acceptance rules:

- The packet must not submit extra `GetOwnDevice`, open a dedicated portal route, start password/SSO, revoke/trust/delete sessions or devices, or mutate account/profile beyond the existing live display-name/current-device rename paths.
- Permission denial, cancellation, stale session, and failure must remain distinct before any live action is enabled.
- Current-device destructive actions require an extra guard or remain blocked.

## Retry, Stale-Session, And Secret Rules

Every retryable account/session action must preserve:

- request kind
- user id
- target device id when present
- normalized payload
- source hash or generation
- request id
- last error
- confirmation text

Every retry must require `PositiveConfirmationModal`. Automatic retry stays blocked.

Secret handling rules:

- No passwords.
- No tokens.
- No SSO codes.
- No pushkeys.
- No unredacted IP addresses.
- No session secrets.

These fields must not appear in UI labels, clipboard payloads, logs, docs, gate stdout, or readiness JSON.

## Verification Requirements

Before password/SSO, cross-session revoke/trust, device delete/trust, or extra account/profile mutations are promoted to live:

- Add focused tests for the new request/result/error/retry/stale-session shape.
- Re-run `cargo test --lib account_management -- --nocapture`.
- Re-run `cargo test --lib hepta_telegram_base_ -- --nocapture`.
- Re-run `scripts/hepta-native-account-management-backend-contract-gate.sh`.
- Re-run Native fixture visual smoke.
- Re-run combined `scripts/hepta-ui-product-readiness-gate.sh`.
- Update the backend handoff `live_wiring` string only after the new live path is actually wired and proven.
