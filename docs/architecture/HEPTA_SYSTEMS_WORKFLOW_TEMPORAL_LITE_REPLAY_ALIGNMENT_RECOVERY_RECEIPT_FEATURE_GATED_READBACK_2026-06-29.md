# Temporal-Lite Replay Alignment Recovery Receipt Feature-Gated Readback

This note closes the local readback step after the Temporal-Lite
replay-alignment recovery window contract is queryable.

## Scope

- Surface:
  `workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback`.
- Source: the Temporal-Lite replay-alignment recovery window readback.
- Projection: 9 recovery-window entries become test-only recovery receipt
  projections, recovery receipt keys, recovery receipt ack keys, recovery
  receipt digests, and deterministic replay-alignment receipt match markers.

## Boundary

This is a test-only replay alignment recovery receipt readback. It makes the
recovery-receipt contract queryable without executing replay, writing
checkpoint state, writing rollback anchors, persisting recovery windows,
persisting recovery receipts, or applying receipt state to durable backends.

The closed boundary is explicit: no replay execution, checkpoint write, rollback anchor write, recovery window persistence, recovery receipt persistence, WorkGraph projection write, event-log write, SQLite write, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Source Gate Cost Boundary

The source gate recursion is bounded to source report invariants at this layer.
The gate validates the recovery-window report shape, counts, and closed
side-effect flags directly, then runs only the recovery-receipt targeted Rust
tests. It does not recursively invoke the full upstream source-gate chain.

## Current Next Gate

After the replay-alignment recovery receipt contract is represented in the
matrix, the next local gate is
`hepta_systems_gate_recursion_cost_boundary_readback`.

That gate should measure and compact the recursive matrix/report/gate cost
without changing workflow execution, replay execution, source gate semantics,
event-log persistence, SQLite persistence, release, canary activation, or live
execution.
