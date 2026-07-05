# Temporal-Lite WorkGraph Projection Feature-Gated Readback

This note closes the local readback step after the Temporal-Lite event-log and
SQLite adapter contract projection.

## Scope

- Surface: `workflow_temporal_lite_work_graph_projection_feature_gated_readback`.
- Source: the Temporal-Lite event-log/SQLite adapter readback.
- Projection: 9 adapter entries become test-only WorkGraph node projections,
  event-edge projections, state-edge projections, projection keys, and
  projection checksums.

## Boundary

This is a test-only WorkGraph projection readback. It makes the projection
contract queryable without applying it to the WorkGraph store or durable
backends.

The closed boundary is explicit: no WorkGraph projection write, WorkGraph projection persistence, event-log write, SQLite write, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Current Next Gate

After the WorkGraph projection contract is represented in the matrix, the next
local gate is
`temporal_lite_work_graph_projection_replay_alignment_feature_gated_readback`.

That gate should remain readback-first: no replay execution, WorkGraph
projection persistence, event-log write, SQLite write, workflow execution,
rollback execution, release, canary activation, or live execution.
