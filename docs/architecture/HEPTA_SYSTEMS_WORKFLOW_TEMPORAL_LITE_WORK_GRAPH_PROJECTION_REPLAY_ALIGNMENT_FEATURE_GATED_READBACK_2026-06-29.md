# Temporal-Lite WorkGraph Projection Replay Alignment Feature-Gated Readback

This note closes the local readback step after the Temporal-Lite WorkGraph
projection contract is queryable.

## Scope

- Surface:
  `workflow_temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback`.
- Source: the Temporal-Lite WorkGraph projection feature-gated readback.
- Projection: 9 WorkGraph projection entries become test-only replay alignment
  projections, projection replay keys, replay alignment checksums, and
  deterministic alignment markers.

## Boundary

This is a test-only WorkGraph projection replay alignment readback. It makes the
replay-alignment contract queryable without executing replay, writing
alignment state, or applying projection state to durable backends.

The closed boundary is explicit: no replay execution, WorkGraph projection write, WorkGraph projection persistence, event-log write, SQLite write, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Current Next Gate

After the replay-alignment contract is represented in the matrix, the next
local gate is
`temporal_lite_replay_alignment_checkpoint_consistency_feature_gated_readback`.

That gate should remain readback-first: no replay execution, checkpoint write,
rollback anchor write, WorkGraph projection persistence, event-log write,
SQLite write, workflow execution, rollback execution, release, canary
activation, or live execution.
