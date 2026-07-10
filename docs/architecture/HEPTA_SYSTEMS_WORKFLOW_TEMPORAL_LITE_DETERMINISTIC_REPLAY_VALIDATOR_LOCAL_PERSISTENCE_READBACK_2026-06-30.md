# Hepta Systems Workflow Temporal-Lite Deterministic Replay Validator Local Persistence Readback - 2026-06-30

This slice implements
`workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback`.
It takes the SQLite/WAL minimal local persistence layer and projects a
deterministic replay validator readback from the local event history while
keeping runtime persistence, workflow replay execution, rollback execution,
canary, and live paths closed.

This is a local persistence readback projection. The Rust tests write only to a
temporary SQLite database, reopen that database, read the append-only
`temporal_lite_events` history, and validate deterministic replay projection
rows from the persisted event order and replay digests.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-report.sh`
- Source surface:
  `workflow_temporal_lite_append_only_event_store_minimal_local_persistence`
- Source entries: 9
- Source table:
  `temporal_lite_events`
- Source scope:
  `local_tempdb_sqlite_wal_append_only_store_test_covered_runtime_write_blocked`
- Source runtime state: runtime event-log write, runtime SQLite write, runtime
  store persistence, workflow execution, replay execution, rollback execution,
  and live execution are all disabled

## Replay Readback

For each persisted local event, the validator projects:

- `replay_order`
- `local_sequence`
- `replay_projection_key`
- `replay_source_digest`
- `replay_observed_digest`
- `replay_batch_digest`
- `local-replay-checksum`
- idempotency, checkpoint, and rollback metadata readback flags

Expected counts:

- `local_event_count = 9`
- `replay_readback_projection_count = 9`
- `deterministic_order_count = 9`
- `replay_digest_count = 9`
- `replay_checksum_count = 9`
- `replay_batch_digest_count = 9`
- `replay_mismatch_count = 0`
- `idempotency_readback_count = 9`
- `checkpoint_readback_count = 9`
- `rollback_anchor_readback_count = 9`

## Closed Boundary

This slice has no runtime event-log write, runtime SQLite write, runtime store persistence, replay projection persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The report is readback-only. The only SQLite read/write activity is covered by
Rust tests against a temporary local database, and the runtime flags continue
to project all write, replay execution, rollback execution, and live paths as
false.

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback.rs`

## Next Move

The next workflow slice should be:

`workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback`

That slice should move checkpoint and rollback anchor readback onto the
SQLite/WAL local event history while keeping runtime event-log writes, runtime
SQLite writes, checkpoint writes, rollback anchor writes, workflow execution,
replay execution, rollback execution, transport mutation, release, canary
activation, and live execution closed.
