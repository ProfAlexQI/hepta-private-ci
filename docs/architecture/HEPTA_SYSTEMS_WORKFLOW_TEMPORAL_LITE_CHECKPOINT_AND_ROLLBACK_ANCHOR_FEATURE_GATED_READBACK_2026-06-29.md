# Temporal-Lite Checkpoint And Rollback Anchor Feature-Gated Readback

This note closes the local readback step after the Temporal-Lite deterministic replay validator.

## Scope

- Surface: `workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback`.
- Source: the deterministic replay validator report.
- Projection: 9 deterministic replay projections are mapped into checkpoint anchor and rollback anchor readback pairs.
- Validation: checkpoint anchor keys, rollback anchor keys, checkpoint digests, rollback digests, durable anchor pairing, and mismatch count are checked.
- Result: all 9 anchor pairs are projected in memory and `anchor_mismatch_count` stays at `0`.

## Boundary

This is a test-only checkpoint and rollback anchor readback. It does not write checkpoints, write rollback anchors, persist anchor state, execute replay, or execute rollback.

The closed boundary is explicit: no runtime event-log write, SQLite write, checkpoint write, rollback anchor write, anchor persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Next Gate

The next local gate is `current_reality_matrix_compact_cache_boundary_readback`.

That gate should reduce the current matrix/dashboard generation cost without weakening the live blockers or recording any evidence, approvals, or decisions.
