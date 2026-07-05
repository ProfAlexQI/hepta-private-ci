# Temporal-Lite Event-Log SQLite Adapter Feature-Gated Readback

This note closes the local readback step after the Temporal-Lite lease and
idempotency index projection.

## Scope

- Surface: `workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback`.
- Source: the Temporal-Lite lease/idempotency index readback.
- Projection: 9 lease/idempotency entries become test-only event-log adapter
  keys, SQLite adapter keys, record keys, row keys, serialization contracts, and
  transaction boundary keys.

## Boundary

This is a test-only event-log and SQLite adapter readback. It makes the adapter
contract queryable without writing either durable backend.

The closed boundary is explicit: no event-log write, SQLite write, adapter persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Current Next Gate

After the adapter contract is represented in the matrix, the next local gate is
`temporal_lite_work_graph_projection_feature_gated_readback`.

That gate should remain readback-first: no WorkGraph projection persistence,
event-log write, SQLite write, workflow execution, replay execution, rollback
execution, release, canary activation, or live execution.
