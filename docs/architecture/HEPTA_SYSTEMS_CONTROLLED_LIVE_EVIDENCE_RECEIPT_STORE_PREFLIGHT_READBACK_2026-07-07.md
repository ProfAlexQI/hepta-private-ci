# Controlled Live Evidence Receipt Store Preflight Readback

This note records a local-only controlled-live readback surface for the Hepta
systems lane. It consumes the operator packet attachment kill-switch rehearsal
boundary readback and projects the receipt-store contract needed before any
status-canary evidence receipt can be written.

## Scope

The controlled live evidence receipt store preflight readback is
`ready_blocked`. It is ready as a local preflight and blocked as a live cutover.

The preflight covers the same seven unchanged-missing controlled-live evidence
gaps:

- `dirty_worktree_boundary`
- `operator_live_approval_missing`
- `fresh_soak_readback_missing`
- `credential_boundary_attestation_missing`
- `gateway_native_telegram_post_boundary_approval_missing`
- `rollback_rehearsal_missing`
- `kill_switch_rehearsal_missing`

Each entry keeps the kill-switch rehearsal boundary attached and projects a
metadata-only receipt route under:

- `.hepta/controlled-live/evidence-receipts/status-canary`
- `readback://controlled-live/evidence-receipt-store/preflight/<blocker-id>`

The projected receipt schema is
`controlled_live_evidence_receipt_v1`. The projected redaction policy is
`metadata_only_no_secret_payload`.

## Boundary

This is a controlled live evidence receipt store preflight readback without
persistence. It deliberately performs no receipt store write, receipt
persistence, approval request, approval acceptance, approval recording, evidence
recording, evidence persistence, blocker waiver, credential read, packet send,
attachment send, packet persistence, attachment persistence, readback
persistence, ledger write, event-log write, SQLite write, Native POST mutation,
Telegram transport mutation, gateway/auth mutation, channel send, provider
call, model call, replay, rollback, kill-switch rehearsal execution,
kill-switch mutation, package, release, Public GA promotion, or live execution.

Gate phrase: controlled live evidence receipt store preflight readback without persistence.

Closed boundary: no receipt store write, receipt persistence, approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, packet send, attachment send, packet persistence, attachment persistence, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- receipt-store preflight entries: 7
- ready receipt-store preflight entries: 7
- missing evidence entries: 7
- projected path allowlist entries: 7
- projected receipt schema entries: 7
- projected redaction policy entries: 7
- projected secret-payload denial entries: 7
- projected idempotency key entries: 7
- projected append-only contract entries: 7
- projected retention policy entries: 7
- projected readback query entries: 7
- projected replay guard entries: 7
- recorded evidence: 0
- waived blockers: 0
- receipt store writes: 0
- live execution: 0

## Verification

The local gate validates:

- The kill-switch rehearsal boundary readback is ready and closed.
- The preflight report exposes seven metadata-only receipt projections.
- Every entry remains `missing -> missing` and `blocked_missing_evidence`.
- Each entry has a stable receipt path, receipt id, idempotency key, and
  readback route.
- Secret payload handling is denied through
  `metadata_only_no_secret_payload`.
- Approval, evidence recording, receipt persistence, ledger write,
  event-log write, SQLite write, credential read, transport mutation,
  rollback, kill-switch mutation, and live execution remain disabled.
- The targeted hepta-runtime Rust tests pass.

## Next Move

The next local-only gate should be
`controlled_live_evidence_receipt_store_shadow_write_rehearsal_without_persistence`.
It should rehearse the append-only receipt shape and readback query in memory
while still refusing receipt persistence, evidence acceptance, ledger writes,
SQLite writes, and live execution.
