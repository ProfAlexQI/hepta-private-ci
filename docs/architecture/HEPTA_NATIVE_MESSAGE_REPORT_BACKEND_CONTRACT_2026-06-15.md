# Hepta Native Message Report Backend Contract

Date: 2026-06-15
Status: backend contract required / UI evidence complete
Gap id: `message_report_send`
Source readiness artifact: `/Users/qianqi/.openclaw/tmp/hepta-ui-product-readiness.p6-matrix-link-server-refresh-20260615`

Machine markers:

- `HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_READY:true`
- `HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_DATE:2026-06-15`
- `HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_GAP_ID:message_report_send`
- `HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_LIVE_WIRING:matrix_report_content_result_retry_loaded_source_fetch_modal_live;moderation_workflow_blocked`
- `HEPTA_NATIVE_MESSAGE_REPORT_BACKEND_CONTRACT_BOUNDARY:no-gateway-runtime-provider-auth-telegram-delivery-mutation`

## UI Evidence Already Complete

The UI lane has already shipped and machine-checked:

- Confirmed `MatrixRequest::ReportContent` send/result wiring.
- Confirmed failed-state retry for cached report event/reason.
- Loaded or source-fetch `EventSourceModal` handoff through `MatrixRequest::FetchEventSource`.
- Local moderation packet snapshot.
- Moderation reviewer packet.
- Typed moderation workflow/result contract packet.
- Workflow result taxonomy packet for queue/policy/reviewer/evidence/appeal/enforcement outcomes.
- Status clipboard, workflow controls, and preflight controls.

The remaining work is not more report UI. It is backend contract and adapter work for moderation workflows beyond Matrix `report_content`.

## Promotion Boundary

Do not wire Policy, Assign, Appeal, Enforce, queue persistence, or enforcement actions until the backend contract provides request, result, error, retry, cancel, evidence retention, source-hash, and audit semantics.

Blocked until contract:

- Moderation queue persistence.
- Policy lookup.
- Reviewer assignment.
- Evidence/source retention beyond source-only current-room event JSON.
- Appeal workflow.
- Enforcement workflow.
- Redact/delete, kick, ban, ignore/block, or related moderation actions.

Forbidden as a side effect of this contract:

- Gateway/runtime/provider-auth calls.
- Telegram delivery mutation.
- Room-state or membership mutation outside an explicit enforcement result contract.
- Message send/edit/redact outside an explicit enforcement result contract.
- Account/profile mutation.
- Duplicate report automation.
- Unconfirmed retry or automatic retry.

## Contract Shapes

### ReportContent Result

This is already live in the UI lane and remains the reference shape.

Required fields:

- `room_id`
- `event_id`
- `reason`
- `score`
- `request_id`
- `source_hash`
- `submitted_at_ms`
- `completed_at_ms`
- `status`: `submitted`, `sent`, `failed`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- The result must echo the event id and reason used for the confirmed request.
- Retry is available only when a cached event id and non-empty reason are present and the user confirms.
- A stale target must not be resubmitted without a fresh source check.

### Source Retention Result

Required fields:

- `room_id`
- `event_id`
- `source_origin`: `loaded_timeline`, `source_fetch`, or `moderation_store`
- `source_hash`
- `retention_id`
- `retention_expires_at_ms`
- `redaction_state`
- `status`: `retained`, `unavailable`, `failed`, or `stale`
- `error_kind`
- `error_message`

Acceptance rules:

- Event source retention must identify whether it came from loaded UI JSON, source-only fetch, or backend moderation storage.
- Missing or redacted source must be a visible result, not an implicit failure.
- Retention id must not expose secret payloads.

### Moderation Queue Persistence Result

Required request fields:

- `room_id`
- `event_id`
- `reason`
- `reporter_user_id_hash`
- `target_user_id_hash`
- `source_hash`
- `request_id`

Required result fields:

- `queue_id`
- `room_id`
- `event_id`
- `status`: `queued`, `duplicate`, `failed`, or `stale`
- `dedupe_key`
- `created_at_ms`
- `error_kind`
- `error_message`
- `retry_eligible`
- `cancel_eligible`

Acceptance rules:

- Duplicate must be distinct from failed.
- Queue id is required before policy, assignment, appeal, or enforcement actions are enabled.
- Cancel must be explicit and must not cancel Matrix `report_content`.

### Policy Lookup Result

Required fields:

- `queue_id`
- `policy_id`
- `policy_version`
- `matched_rules`
- `recommended_actions`
- `confidence`
- `status`: `matched`, `not_matched`, `failed`, or `stale`
- `error_kind`
- `error_message`

Acceptance rules:

- Recommended actions are advisory until an enforcement request is confirmed.
- Policy version must be visible in the result.
- A stale queue id must require refresh before retry.

### Reviewer Assignment Result

Required fields:

- `queue_id`
- `reviewer_id_hash`
- `assignment_id`
- `assigned_at_ms`
- `status`: `assigned`, `unassigned`, `failed`, or `stale`
- `error_kind`
- `error_message`

Acceptance rules:

- Reviewer identity must be hashed or otherwise redacted.
- Assignment must be separate from policy lookup.
- Reassignment must produce a new assignment id or generation.

### Appeal Workflow Result

Required fields:

- `queue_id`
- `appeal_id`
- `appeal_state`
- `submitted_by_hash`
- `submitted_at_ms`
- `status`: `opened`, `updated`, `closed`, `failed`, or `stale`
- `error_kind`
- `error_message`

Acceptance rules:

- Appeal state must not imply enforcement was changed unless an enforcement result confirms it.
- Appeal submitter identity must be redacted.

### Enforcement Workflow Result

Required request fields:

- `queue_id`
- `room_id`
- `event_id`
- `action`: `redact`, `delete`, `kick`, `ban`, `ignore`, `block`, or `none`
- `reason`
- `policy_id`
- `assignment_id`
- `source_hash`
- `request_id`

Required result fields:

- `queue_id`
- `action`
- `target_event_id`
- `target_user_id_hash`
- `result_event_id`
- `membership_after`
- `status`: `applied`, `failed`, `permission_denied`, or `stale`
- `error_kind`
- `error_message`
- `retry_eligible`

Acceptance rules:

- Permission denial must be distinct from server/transport failure.
- Membership actions must echo resulting membership.
- Redaction/delete actions must echo the affected event id.
- Enforcement action must require confirmation and audit metadata.

### Workflow Result Taxonomy UI Packet

The UI lane now exposes a local-only Taxonomy packet that names existing live references and all blocked moderation result slots before promotion.

Live references:

- Confirmed `MatrixRequest::ReportContent` send/result/retry.
- Loaded or source-fetch `EventSourceModal` handoff.

Blocked operation ids:

- `queue_operation_id`: `not_assigned`
- `policy_lookup_operation_id`: `not_assigned`
- `reviewer_assignment_operation_id`: `not_assigned`
- `evidence_retention_operation_id`: `not_assigned`
- `appeal_operation_id`: `not_assigned`
- `enforcement_operation_id`: `not_assigned`

Blocked result taxonomy:

- `queue_result`: `queued`, `duplicate`, `cancelled`, `failed`, `stale`, `not_wired`
- `policy_result`: `matched`, `not_matched`, `failed`, `stale`, `not_wired`
- `reviewer_result`: `assigned`, `unassigned`, `conflict`, `failed`, `stale`, `not_wired`
- `evidence_result`: `retained`, `unavailable`, `failed`, `stale`, `not_wired`
- `appeal_result`: `opened`, `updated`, `closed`, `failed`, `stale`, `not_wired`
- `enforcement_result`: `none`, `redacted`, `deleted`, `kicked`, `banned`, `ignored`, `blocked`, `permission_denied`, `failed`, `stale`, `not_wired`

Acceptance rules:

- Retry must require `PositiveConfirmationModal`, backend request id, source hash, and queue id before any workflow retry.
- Cancel remains a local dismiss until a backend queue cancel result contract exists.
- Stale workflow results must compare event id, room id, reason hash, source hash, and queue generation.
- Audit metadata must not expose access tokens, raw event JSON, raw reporter ids, raw target user ids, policy secrets, reviewer identities, or full moderation reasons.

## Retry, Cancel, And Audit Rules

Every retryable action must preserve:

- request kind
- room id
- event id
- queue id when present
- normalized payload
- source hash or generation
- request id
- last error
- confirmation text

Every retry must require `PositiveConfirmationModal`. Automatic retry stays blocked.

Every moderation workflow result must produce audit metadata:

- actor identity hash
- action id
- target id hash or event id
- policy version when applicable
- source hash
- timestamp

## Verification Requirements

Before moderation workflow actions are promoted to live:

- Add focused tests for the new request/result/error/retry/cancel shape.
- Re-run `cargo test --lib message_report -- --nocapture`.
- Re-run `cargo test --lib hepta_telegram_base_ -- --nocapture`.
- Re-run `scripts/hepta-native-message-report-backend-contract-gate.sh`.
- Re-run Native fixture visual smoke.
- Re-run combined `scripts/hepta-ui-product-readiness-gate.sh`.
- Update the backend handoff `live_wiring` string only after the new live path is actually wired and proven.
