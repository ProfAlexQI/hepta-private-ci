# Temporal-Lite Replay Alignment Recovery Window Feature-Gated Readback

This note closes the local readback step after the Temporal-Lite
replay-alignment rollback consistency contract is queryable.

## Scope

- Surface:
  `workflow_temporal_lite_replay_alignment_recovery_window_feature_gated_readback`.
- Source: the Temporal-Lite replay-alignment rollback consistency readback.
- Projection: 9 rollback-consistency entries become test-only recovery window
  projections, recovery window keys, recovery window start/end keys, recovery
  window digests, and deterministic replay-alignment recovery match markers.

## Boundary

This is a test-only replay alignment recovery window readback. It makes the
recovery-window contract queryable without executing replay, writing
checkpoint state, writing rollback anchors, persisting recovery windows, or
applying recovery state to durable backends.

The closed boundary is explicit: no replay execution, checkpoint write, rollback anchor write, recovery window persistence, WorkGraph projection write, event-log write, SQLite write, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Current Next Gate

After the replay-alignment recovery window contract is represented in the
matrix, the next local gate is
`temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback`.

That gate should remain readback-first: no replay execution, checkpoint write,
rollback anchor write, recovery window persistence, WorkGraph projection
persistence, event-log write, SQLite write, workflow execution, rollback
execution, release, canary activation, or live execution.
