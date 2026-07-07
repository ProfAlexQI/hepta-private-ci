# Controlled Live Evidence Receipt Store Persistence Open Preconditions Readback

This note records a local-only controlled-live readback surface for the Hepta
systems lane. It consumes the in-memory receipt shadow write rehearsal and
lists the conditions that must become true before any real receipt persistence
can be enabled.

## Scope

The controlled live evidence receipt store persistence open-preconditions
readback is `ready_blocked`. It is ready as a condition catalog and blocked as
a persistence or live cutover.

The readback covers the same seven unchanged-missing controlled-live evidence
gaps:

- `dirty_worktree_boundary`
- `operator_live_approval_missing`
- `fresh_soak_readback_missing`
- `credential_boundary_attestation_missing`
- `gateway_native_telegram_post_boundary_approval_missing`
- `rollback_rehearsal_missing`
- `kill_switch_rehearsal_missing`

Each entry keeps the shadow receipt rehearsal route and adds a persistence
open-precondition route:

- `readback://controlled-live/evidence-receipt-store/persistence-open-preconditions/<blocker-id>`

## Required Before Persistence

Each entry lists these required but currently missing conditions:

- explicit operator approval for the receipt store write
- accepted evidence for the blocker
- store path write grant for the projected receipt path
- atomic append plan enabled for the metadata-only receipt
- post-write readback path bound and persisted
- rollback rehearsal verified for the receipt append
- retention policy committed for the receipt metadata

All of those conditions are present as requirements and absent as approvals,
grants, persisted readbacks, committed policies, or verified rehearsals.

## Boundary

This is a controlled live evidence receipt store persistence open-preconditions
readback without opening persistence. It deliberately performs no receipt store
write, receipt persistence, approval request, approval acceptance, approval
recording, evidence recording, evidence persistence, blocker waiver, credential
read, packet send, attachment send, packet persistence, attachment persistence,
readback persistence, ledger write, event-log write, SQLite write, Native POST
mutation, Telegram transport mutation, gateway/auth mutation, channel send,
provider call, model call, replay, rollback, kill-switch rehearsal execution,
kill-switch mutation, package, release, Public GA promotion, or live execution.

Gate phrase: controlled live evidence receipt store persistence open-preconditions readback without opening persistence.

Closed boundary: no receipt store write, receipt persistence, approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, packet send, attachment send, packet persistence, attachment persistence, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- precondition entries: 7
- ready precondition catalog entries: 7
- required operator approvals: 7
- present operator approvals: 0
- required evidence acceptances: 7
- present evidence acceptances: 0
- required store path write grants: 7
- present store path write grants: 0
- required atomic append plans: 7
- enabled atomic append plans: 0
- required post-write readbacks: 7
- persisted post-write readbacks: 0
- required rollback rehearsals: 7
- verified rollback rehearsals: 0
- required retention policies: 7
- committed retention policies: 0
- confirmed persistence denials: 7
- confirmed ledger denials: 7
- confirmed event-log denials: 7
- confirmed SQLite denials: 7
- confirmed live denials: 7
- recorded evidence: 0
- waived blockers: 0
- receipt store writes: 0
- live execution: 0

## Verification

The local gate validates:

- The receipt shadow write rehearsal remains `ready_blocked`.
- Seven persistence open-precondition entries are listed.
- Every entry remains `missing -> missing` and `blocked_missing_evidence`.
- Operator approval, evidence acceptance, store path grant, atomic append,
  post-write readback, rollback rehearsal, and retention policy are required
  and not yet present.
- Receipt-store write, receipt persistence, ledger write, event-log write,
  SQLite write, credential read, transport mutation, rollback, kill-switch
  mutation, and live execution remain disabled.
- The targeted hepta-runtime Rust tests pass.

## Next Move

The next local-only gate should be
`controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance`.
It should assemble the operator-facing acceptance packet from these required
conditions while still refusing approval acceptance, evidence recording,
receipt persistence, ledger writes, SQLite writes, and live execution.
