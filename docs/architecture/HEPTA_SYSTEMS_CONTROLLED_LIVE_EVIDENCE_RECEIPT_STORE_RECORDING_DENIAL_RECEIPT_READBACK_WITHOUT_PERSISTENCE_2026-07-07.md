# Controlled Live Evidence Receipt Store Recording Denial Receipt Readback Without Persistence

This note records a local-only controlled-live readback surface for the Hepta
systems lane. It consumes the acceptance decision recording boundary readback
and projects queryable denial receipts for the still-denied recording attempt,
without persisting any receipt.

## Scope

The controlled live evidence receipt store recording denial receipt readback
without persistence is `ready_blocked`. It is ready as a denial receipt
projection and blocked as receipt persistence, evidence recording, approval
acceptance, and live cutover.

The readback covers the same seven unchanged-missing controlled-live evidence
gaps:

- `dirty_worktree_boundary`
- `operator_live_approval_missing`
- `fresh_soak_readback_missing`
- `credential_boundary_attestation_missing`
- `gateway_native_telegram_post_boundary_approval_missing`
- `rollback_rehearsal_missing`
- `kill_switch_rehearsal_missing`

Each entry keeps the source acceptance decision recording boundary route and
adds a denial receipt route:

- `readback://controlled-live/evidence-receipt-store/recording-denial-receipts/<blocker-id>`

The shared denial receipt identity is:

- collection id: `controlled-live-evidence-receipt-store-recording-denial-receipts`
- collection route: `readback://controlled-live/evidence-receipt-store/recording-denial-receipts`
- receipt schema: `controlled_live_evidence_receipt_store_recording_denial_receipt_v1`

## Receipt Contract

Each entry projects:

- the source recording boundary entry id and route
- the source acceptance decision record id and idempotency key
- the source denial receipt id from the recording boundary
- a recording denial receipt id
- a stable denial receipt digest
- a denial receipt idempotency key
- the denial reason `operator_acceptance_missing_evidence_acceptance_missing_recording_disabled`

The projection is queryable and diffable in stdout only. The denial receipt is
not persisted, and no idempotency index is written.

## Boundary

This is a controlled live evidence receipt store recording denial receipt
readback without persistence. It deliberately performs no denial receipt
persistence, acceptance decision recording, acceptance decision persistence,
evidence recording, evidence persistence, receipt store write, receipt
persistence, ledger write, event-log write, SQLite write, credential read,
operator packet send, operator packet persistence, approval request, approval
acceptance, Native POST mutation, Telegram transport mutation, gateway/auth
mutation, channel send, provider call, model call, replay, rollback,
kill-switch rehearsal execution, kill-switch mutation, package, release, Public
GA promotion, or live execution.

Gate phrase: controlled live evidence receipt store recording denial receipt readback without persistence.

Closed boundary: no denial receipt persistence, acceptance decision recording, acceptance decision persistence, evidence recording, evidence persistence, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, operator packet send, operator packet persistence, approval request, approval acceptance, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- denial receipt entries: 7
- projected denial receipts: 7
- projected denial receipt digests: 7
- projected denial receipt readback routes: 7
- projected denial receipt idempotency keys: 7
- unique denial receipt idempotency keys: 7
- attached source recording boundaries: 7
- attached source decision record ids: 7
- attached source denial receipt ids: 7
- projected recording denial reasons: 7
- missing operator acceptances: 7
- missing evidence acceptances: 7
- disabled decision recordings: 7
- recorded acceptance decisions: 0
- persisted acceptance decisions: 0
- persisted denial receipts: 0
- receipt store writes: 0
- persisted receipts: 0
- ledger writes: 0
- event-log writes: 0
- SQLite writes: 0
- live mutations: 0

## Verification

The local gate validates:

- The acceptance decision recording boundary source remains `ready_blocked`.
- Seven denial receipt entries are projected from the source boundary.
- Every entry remains `missing -> missing` and `blocked_missing_evidence`.
- Operator acceptance and evidence acceptance are required and absent.
- The denial receipt id, digest, readback route, idempotency key, source
  boundary, source decision record id, and denial reason are projected for each
  entry.
- Denial receipt persistence, acceptance decision recording, evidence
  recording, receipt-store write, receipt persistence, ledger write, event-log
  write, SQLite write, credential read, transport mutation, replay, rollback,
  kill-switch mutation, and live execution remain disabled.
- The targeted hepta-runtime Rust tests pass.

## Next Move

The next local-only gate should be
`controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence`.
It should project retention, replay, and expiry invariants for these denial
receipts while still refusing receipt persistence, ledger writes, SQLite writes,
transport mutation, canary, and live execution.
