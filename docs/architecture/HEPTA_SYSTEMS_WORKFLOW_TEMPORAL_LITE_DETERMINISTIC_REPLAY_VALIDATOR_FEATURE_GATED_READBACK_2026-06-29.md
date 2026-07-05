# Temporal-Lite Deterministic Replay Validator Feature-Gated Readback

This note closes the next local workflow milestone after the Temporal-Lite append-only event store test implementation.

## Scope

- Surface: `workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback`.
- Source: the test-only in-memory append-only event store report.
- Projection: 9 append-only events are mapped into deterministic replay readback rows.
- Validation: event order, replay digest echo, replay checksum construction, idempotency metadata, checkpoint keys, and rollback anchors are all checked.
- Result: mismatch count stays at `0`.

## Boundary

This is a test-only deterministic projection. It does not execute workflow replay and it does not persist the replay projection.

The closed boundary is explicit: no runtime event-log write, SQLite write, replay projection persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

## Next Gate

The next local gate is `temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback`.

That gate should stay feature-gated and readback-only until the dirty worktree and controlled-live evidence blockers are resolved.
