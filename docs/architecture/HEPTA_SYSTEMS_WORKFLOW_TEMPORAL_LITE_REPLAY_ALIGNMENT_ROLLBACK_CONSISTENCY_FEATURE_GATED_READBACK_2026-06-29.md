# Temporal-Lite Replay Alignment Rollback Consistency Feature-Gated Readback

This note closes the local readback step after the Temporal-Lite
replay-alignment checkpoint consistency contract is queryable.

## Scope

- Surface:
  `workflow_temporal_lite_replay_alignment_rollback_consistency_feature_gated_readback`.
- Source: the Temporal-Lite replay-alignment checkpoint consistency readback.
- Projection: 9 checkpoint-consistency entries become test-only rollback
  consistency projections, rollback consistency keys, rollback readback keys,
  rollback consistency digests, and deterministic rollback match markers.

## Boundary

This is a test-only replay alignment rollback consistency readback. It makes
the rollback-consistency contract queryable without executing replay, writing
checkpoint state, writing rollback anchors, or applying consistency state to
durable backends.

The closed boundary is explicit: no replay execution, checkpoint write, rollback anchor write, rollback consistency persistence, WorkGraph projection write, event-log write, SQLite write, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Current Next Gate

After the replay-alignment rollback consistency contract is represented in the
matrix, the next local gate is
`temporal_lite_replay_alignment_recovery_window_feature_gated_readback`.

That gate should remain readback-first: no replay execution, checkpoint write,
rollback anchor write, WorkGraph projection persistence, event-log write,
SQLite write, workflow execution, rollback execution, release, canary
activation, or live execution.
