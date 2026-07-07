# Controlled Live Evidence Receipt Store Recording Denial Receipt Retention Replay Readback Without Persistence

This note records a local-only controlled-live readback surface for the Hepta
systems lane. It consumes the recording denial receipt readback and projects
retention, replay, expiry, garbage-collection, and supersession invariants
without persisting any receipt or index.

## Scope

The controlled live evidence receipt store recording denial receipt retention
replay readback without persistence is `ready_blocked`. It is ready as a
retention/replay projection and blocked as persistence, replay index writes,
expiry enforcement, garbage collection, and live cutover.

The readback covers the same seven unchanged-missing controlled-live evidence
gaps:

- `dirty_worktree_boundary`
- `operator_live_approval_missing`
- `fresh_soak_readback_missing`
- `credential_boundary_attestation_missing`
- `gateway_native_telegram_post_boundary_approval_missing`
- `rollback_rehearsal_missing`
- `kill_switch_rehearsal_missing`

Each entry keeps the source denial receipt and adds retention/replay routes:

- `readback://controlled-live/evidence-receipt-store/recording-denial-receipts/retention-replay/retention/<blocker-id>`
- `readback://controlled-live/evidence-receipt-store/recording-denial-receipts/retention-replay/replay/<blocker-id>`
- `readback://controlled-live/evidence-receipt-store/recording-denial-receipts/retention-replay/readback/<blocker-id>`

## Retention And Replay Contract

Each entry projects:

- a retention policy id and route
- an expiry guard id
- a replay key and replay idempotency key
- a replay readback route and retention readback route
- a garbage-collection denial id
- a supersession guard id
- a zero-effect digest

The retention state is `projected_not_persisted` and the replay state is
`projected_not_written` for all seven entries.

## Boundary

This is a controlled live evidence receipt store recording denial receipt
retention replay readback without persistence. It deliberately performs no
retention policy persistence, replay index write, expiry enforcement, garbage
collection, denial receipt persistence, receipt store write, receipt
persistence, ledger write, event-log write, SQLite write, credential read,
Native POST mutation, Telegram transport mutation, gateway/auth mutation,
channel send, provider call, model call, replay execution, rollback,
kill-switch rehearsal execution, kill-switch mutation, package, release, Public
GA promotion, or live execution.

Gate phrase: controlled live evidence receipt store recording denial receipt retention replay readback without persistence.

Closed boundary: no retention policy persistence, replay index write, expiry enforcement, garbage collection, denial receipt persistence, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- retention/replay entries: 7
- projected retention policies: 7
- projected expiry guards: 7
- projected replay keys: 7
- projected replay idempotency keys: 7
- unique replay idempotency keys: 7
- projected retention readback routes: 7
- projected replay readback routes: 7
- projected garbage-collection denials: 7
- projected supersession guards: 7
- projected zero-effect digests: 7
- attached source denial receipts: 7
- persisted retention policies: 0
- replay index writes: 0
- expiry enforcements: 0
- garbage collections: 0
- persisted denial receipts: 0
- receipt store writes: 0
- persisted receipts: 0
- ledger writes: 0
- event-log writes: 0
- SQLite writes: 0
- live mutations: 0

## Verification

The local gate validates:

- The recording denial receipt source remains `ready_blocked`.
- Seven retention/replay entries are projected from the source denial receipts.
- Every entry remains `missing -> missing` and `blocked_missing_evidence`.
- Retention, expiry, replay, garbage-collection, supersession, and zero-effect
  digest projections exist for each denial receipt.
- Retention policy persistence, replay index writes, expiry enforcement,
  garbage collection, denial receipt persistence, receipt-store write, receipt
  persistence, ledger write, event-log write, SQLite write, credential read,
  transport mutation, replay execution, rollback, kill-switch mutation, and
  live execution remain disabled.
- The targeted hepta-runtime Rust tests pass.

## Next Move

The next local-only gate should be
`controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance`.
It should switch from negative denial projections back to the positive
conditions required before operator acceptance, evidence acceptance, receipt
persistence, ledger writes, SQLite writes, transport mutation, canary, or live
execution can open.
