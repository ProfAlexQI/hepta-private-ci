# Temporal-Lite Replay Alignment Recovery Receipt Local Persistence Readback

Date: 2026-06-30

Surface:
`workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback`

Gate:
`workflow_temporal_lite_replay_alignment_recovery_receipt_local_persistence_readback_gate`

This is a local persistence replay alignment recovery receipt readback boundary.
It consumes the local recovery-window readback from reopened SQLite/WAL
`temporal_lite_events` history and projects the recovery receipt, receipt ack,
and receipt digest contracts for the 9 workflow event contracts.

The boundary remains readback-only:

- no replay execution, checkpoint write, rollback anchor write, recovery window
  persistence, recovery receipt persistence, WorkGraph projection write, runtime
  event-log write, runtime SQLite write, runtime store persistence, workflow
  execution, rollback execution, provider invocation, model invocation,
  Gateway/Auth mutation, Native POST mutation, Telegram transport mutation,
  channel send, package, release, Public GA promotion, canary activation, or
  live execution
- no runtime feature gate is enabled
- no runtime store persistence is allowed
- no source recovery-window result is persisted by this boundary

Closed boundary: no replay execution, checkpoint write, rollback anchor write, recovery window persistence, recovery receipt persistence, WorkGraph projection write, runtime event-log write, runtime SQLite write, runtime store persistence, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The local SQLite/WAL coverage is test-only. Tests create a temporary local
SQLite database, append the minimal local Temporal-lite event fixture, reopen the
database, read back the event history, and project deterministic replay,
checkpoint/rollback anchors, lease/idempotency, event-log/SQLite adapter,
WorkGraph projection, replay alignment, checkpoint consistency, rollback
consistency, recovery window, and finally recovery receipt readback.

The source gate recursion is bounded to source report invariants and one
targeted Rust test. This keeps the new gate from recursively running the full
upstream local-persistence gate chain while preserving the readback contract and
the reopened SQLite/WAL coverage.

In gate terms, source gate recursion is bounded to source report invariants and one targeted Rust test.

Expected projection:

- recovery receipt projections: 9/9
- recovery receipt keys: 9/9
- recovery receipt digests: 9/9
- recovery receipt acks: 9/9
- replay-alignment receipt matches: 9/9
- SQLite readback validated: 9/9
- mismatch count: 0
- replay/checkpoint/rollback/recovery-window/recovery-receipt/runtime writes: 0

Recommended next step:

`hepta_systems_gate_recursion_cost_boundary_readback`
