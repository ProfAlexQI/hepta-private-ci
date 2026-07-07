# Controlled Live Evidence Receipt Store Acceptance Authority Packet Receipt Store Write Denial Retention Replay Readback Without Write

This note records a local-only controlled-live readback surface for the Hepta
systems lane. It consumes the receipt-store write-denial readback and makes the
retention/replay invariants for that denial queryable without recording a write
attempt, writing receipt data, executing replay, or opening live execution.

## Scope

The controlled live evidence receipt store acceptance authority packet
receipt-store write denial retention replay readback without write is
`ready_blocked`. It is ready as a queryable retention/replay projection and
blocked as retention persistence, replay-index write, expiry enforcement,
garbage collection, receipt-store write, receipt persistence, ledger write,
SQLite write, and live cutover.

The readback covers the same seven unchanged-missing controlled-live evidence
gaps. Each entry keeps the source write-denial route and adds local retention
and replay routes:

- `readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/retention/<blocker-id>`
- `readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/replay/<blocker-id>`

## Retention And Replay Contract

Each entry proves the write denial is still attached and projects retention and
replay invariants:

- `retention_policy_projected=true`
- `expiry_guard_projected=true`
- `replay_key_projected=true`
- `replay_idempotency_key_projected=true`
- `garbage_collection_denial_projected=true`
- `supersession_guard_projected=true`
- `zero_effect_digest_projected=true`

This readback does not persist retention policy, write replay indexes, enforce
expiry, perform garbage collection, record a receipt-store write attempt, write
the receipt store, or execute replay.

## Source Cache

The receipt-store chain is now evaluated through a source-cache path before
adding any deeper readback gate:

- Rust report constructors in the controlled-live evidence receipt-store chain
  use a process-local `OnceLock` so repeated targeted tests do not rebuild the
  same upstream source report over and over.
- The shell report accepts
  `HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECEIPT_STORE_WRITE_DENIAL_JSON`
  to consume an already-rendered source report.
- The shell gate renders the source receipt-store write-denial report once,
  passes that cached JSON into this report, and validates that the target
  report used `source_cache_mode=provided_source_json`.

This cache is read-only and process-local. It does not create or persist a
cache artifact outside the gate temp directory.

## Boundary

This is a controlled live evidence receipt store acceptance authority packet
receipt-store write denial retention replay readback without write. It
deliberately performs no retention policy persistence, replay index write,
expiry enforcement, garbage collection, receipt-store write attempt record,
receipt store write, receipt persistence, ledger write, event-log write, SQLite
write, credential read, Native POST mutation, Telegram transport mutation,
gateway/auth mutation, channel send, provider call, model call, replay
execution, rollback, kill-switch rehearsal execution, kill-switch mutation,
package, release, Public GA promotion, or live execution.

Gate phrase: controlled live evidence receipt store acceptance authority packet receipt-store write denial retention replay readback without write.

Closed boundary: no retention policy persistence, replay index write, expiry enforcement, garbage collection, receipt-store write attempt record, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution.

## Expected Counts

- retention/replay entries: 7
- retention policies projected: 7
- expiry guards projected: 7
- replay keys projected: 7
- replay idempotency keys projected: 7
- unique replay idempotency keys: 7
- retention readback routes projected: 7
- replay readback routes projected: 7
- garbage-collection denials projected: 7
- supersession guards projected: 7
- zero-effect digests projected: 7
- source write denials attached: 7
- retention policies persisted: 0
- replay indexes written: 0
- expiry enforcements: 0
- garbage collections performed: 0
- receipt-store write-attempt records: 0
- receipt store writes: 0
- persisted receipts: 0
- ledger writes: 0
- event-log writes: 0
- SQLite writes: 0
- live mutations: 0

## Verification

The local gate validates:

- The source receipt-store write denial remains `ready_blocked`.
- The source report is rendered once and reused by the target report through the
  cached source JSON path.
- Seven retention/replay entries are projected from the write-denial source.
- Every entry remains `missing -> missing` and `blocked_missing_evidence`.
- Retention policy persistence, replay-index write, expiry enforcement, garbage
  collection, write-attempt recording, receipt-store write, receipt persistence,
  ledger write, event-log write, SQLite write, credential read, transport
  mutation, replay execution, rollback, kill-switch mutation, and live execution
  remain disabled.
- The targeted hepta-runtime Rust tests pass.

## Next Move

The next local-only gate should be
`controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write`.
It should turn the retention/replay projection back into an operator-facing
positive precondition matrix for any future local receipt-store write, still
without recording a write attempt, writing receipt data, mutating transport,
starting a canary, or opening live execution.
