# Hepta Systems Workflow Temporal-Lite Replay Alignment Rollback Consistency Local Persistence Readback - 2026-06-30

This slice implements
`workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback`.
It consumes the SQLite/WAL-backed replay-alignment checkpoint consistency local
persistence readback and projects rollback consistency keys, rollback readback
keys, rollback consistency digests, replay/rollback match flags, and
write-denial flags while keeping rollback writes, replay execution, runtime
writes, canary, and live paths closed.

This is a local persistence replay alignment rollback consistency readback.
The Rust tests write only to a temporary SQLite database, reopen that database,
read the append-only `temporal_lite_events` history, project deterministic
replay rows, checkpoint/rollback anchors, lease/idempotency readback rows,
event-log/SQLite adapter readback rows, WorkGraph projection rows,
replay-alignment rows, checkpoint consistency rows, and then derive rollback
consistency rows from those local checkpoint consistency rows.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-replay-alignment-checkpoint-consistency-local-persistence-readback-report.sh`
- Source surface:
  `workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback`
- Source entries: 9
- Source table:
  `temporal_lite_events`
- Source scope:
  `local_persistence_replay_alignment_checkpoint_consistency_readback_no_execution`
- Source runtime state: runtime event-log write, runtime SQLite write, runtime
  store persistence, WorkGraph projection write, checkpoint consistency
  persistence, checkpoint write, rollback anchor write, workflow execution,
  replay execution, rollback execution, and live execution are all disabled

## Rollback Consistency Readback

For each local checkpoint consistency row, this slice projects:

- `rollback_consistency_key`
- `rollback_readback_key`
- `rollback_consistency_digest`
- `expected_rollback_projection_key`
- replay/rollback match flag
- rollback write-denial flag
- rollback consistency persistence-denial flag

Expected counts:

- `rollback_consistency_projection_count = 9`
- `rollback_consistency_key_count = 9`
- `rollback_digest_count = 9`
- `replay_alignment_rollback_match_count = 9`
- `sqlite_readback_validated_count = 9`
- `rollback_mismatch_count = 0`
- `replay_executed_count = 0`
- `checkpoint_written_count = 0`
- `rollback_anchor_written_count = 0`
- `consistency_persisted_count = 0`
- `work_graph_store_write_count = 0`
- `event_log_write_count = 0`
- `sqlite_write_count = 0`

## Closed Boundary

This slice has no replay execution, checkpoint write, rollback anchor write, rollback consistency persistence, WorkGraph projection write, runtime event-log write, runtime SQLite write, runtime store persistence, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The report is readback-only. The only SQLite read/write activity is covered by
Rust tests against a temporary local database, and the runtime flags continue
to project all checkpoint write, rollback anchor write, replay execution,
rollback execution, and live paths as false.

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-replay-alignment-rollback-consistency-local-persistence-readback-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-replay-alignment-rollback-consistency-local-persistence-readback-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback.rs`

## Next Move

The next workflow slice should be:

`workflow_temporal_lite_replay_alignment_recovery_window_local_persistence_readback`

That slice should derive the local recovery window from the rollback consistency
projection and SQLite/WAL event history while keeping runtime event-log writes,
runtime SQLite writes, durable-store persistence, workflow execution, replay
execution, rollback execution, transport mutation, release, canary activation,
and live execution closed.
