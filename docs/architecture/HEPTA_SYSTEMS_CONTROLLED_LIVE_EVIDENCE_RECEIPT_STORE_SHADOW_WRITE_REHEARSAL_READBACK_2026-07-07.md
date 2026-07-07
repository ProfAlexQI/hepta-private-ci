# Controlled Live Evidence Receipt Store Shadow Write Rehearsal Readback

This note records a local-only controlled-live readback surface for the Hepta
systems lane. It consumes the controlled-live evidence receipt store preflight
and rehearses the append-only receipt shape in memory before any receipt store
write is allowed.

## Scope

The controlled live evidence receipt store shadow write rehearsal readback is
`ready_blocked`. It is ready as an in-memory rehearsal and blocked as a live
cutover.

The rehearsal covers the same seven unchanged-missing controlled-live evidence
gaps:

- `dirty_worktree_boundary`
- `operator_live_approval_missing`
- `fresh_soak_readback_missing`
- `credential_boundary_attestation_missing`
- `gateway_native_telegram_post_boundary_approval_missing`
- `rollback_rehearsal_missing`
- `kill_switch_rehearsal_missing`

Each entry keeps the preflight receipt path, receipt id, idempotency key, and
readback route, then adds an in-memory shadow write route:

- `readback://controlled-live/evidence-receipt-store/shadow-write-rehearsal/<blocker-id>`

The shadow receipt payload is metadata-only. It is identified by a stable
fingerprint of the form
`sha256:controlled-live-evidence-receipt-shadow:<blocker-id>:metadata-only`.
The projected append-only sequence starts at `00000001` because no prior
receipt head exists.

## Boundary

This is a controlled live evidence receipt store shadow write rehearsal
readback without persistence. It deliberately performs no receipt store write,
receipt persistence, approval request, approval acceptance, approval recording,
evidence recording, evidence persistence, blocker waiver, credential read,
packet send, attachment send, packet persistence, attachment persistence,
readback persistence, ledger write, event-log write, SQLite write, Native POST
mutation, Telegram transport mutation, gateway/auth mutation, channel send,
provider call, model call, replay, rollback, kill-switch rehearsal execution,
kill-switch mutation, package, release, Public GA promotion, or live execution.

Gate phrase: controlled live evidence receipt store shadow write rehearsal readback without persistence.

Closed boundary: no receipt store write, receipt persistence, approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, packet send, attachment send, packet persistence, attachment persistence, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- shadow write rehearsal entries: 7
- ready shadow write rehearsal entries: 7
- in-memory shadow receipts rendered: 7
- projected append-only sequences: 7
- bound readback queries: 7
- projected idempotency dedup entries: 7
- projected redacted payloads: 7
- confirmed secret-payload denials: 7
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

- The receipt-store preflight remains `ready_blocked`.
- Seven in-memory shadow receipt shapes are rendered and readback-bound.
- Every entry remains `missing -> missing` and `blocked_missing_evidence`.
- Each entry has a stable shadow write route, shadow receipt id, metadata-only
  fingerprint, append-only sequence key, and projected receipt head.
- Secret payload handling is denied through the existing
  `metadata_only_no_secret_payload` policy.
- Approval, evidence recording, receipt-store write, receipt persistence,
  ledger write, event-log write, SQLite write, credential read, transport
  mutation, rollback, kill-switch mutation, and live execution remain disabled.
- The targeted hepta-runtime Rust tests pass.

## Next Move

The next local-only gate should be
`controlled_live_evidence_receipt_store_persistence_open_preconditions_readback`.
It should list the exact operator approval, evidence acceptance, store path,
atomic append, readback, rollback, and retention conditions that must become
true before any real receipt persistence can be enabled.
