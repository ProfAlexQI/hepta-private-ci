# Temporal-Lite Replay Alignment Checkpoint Consistency Feature-Gated Readback

This note closes the local readback step after the Temporal-Lite WorkGraph
projection replay-alignment contract is queryable.

## Scope

- Surface:
  `workflow_temporal_lite_replay_alignment_checkpoint_consistency_feature_gated_readback`.
- Source: the Temporal-Lite WorkGraph projection replay-alignment readback.
- Projection: 9 replay-alignment entries become test-only checkpoint
  consistency projections, checkpoint consistency keys, checkpoint readback
  keys, checkpoint consistency digests, and deterministic match markers.

## Boundary

This is a test-only replay alignment checkpoint consistency readback. It makes
the checkpoint-consistency contract queryable without executing replay,
writing checkpoint state, writing rollback anchors, or applying consistency
state to durable backends.

The closed boundary is explicit: no replay execution, checkpoint write, rollback anchor write, checkpoint consistency persistence, WorkGraph projection write, event-log write, SQLite write, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Current Next Gate

After the replay-alignment checkpoint consistency contract is represented in
the matrix, the next local gate is
`temporal_lite_replay_alignment_rollback_consistency_feature_gated_readback`.

That gate should remain readback-first: no replay execution, checkpoint write,
rollback anchor write, WorkGraph projection persistence, event-log write,
SQLite write, workflow execution, rollback execution, release, canary
activation, or live execution.
