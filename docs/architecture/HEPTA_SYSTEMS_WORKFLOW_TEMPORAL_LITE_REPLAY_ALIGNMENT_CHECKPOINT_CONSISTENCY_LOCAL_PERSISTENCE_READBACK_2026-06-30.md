# Hepta Systems Workflow Temporal-Lite Replay Alignment Checkpoint Consistency Local Persistence Readback - 2026-06-30

This slice implements
`workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback`.
It consumes the SQLite/WAL-backed WorkGraph projection replay-alignment local
persistence readback and projects checkpoint consistency keys, checkpoint
readback keys, checkpoint consistency digests, replay/checkpoint match flags,
and write-denial flags while keeping checkpoint writes, replay execution,
runtime writes, canary, and live paths closed.

This is a local persistence replay alignment checkpoint consistency readback.
The Rust tests write only to a temporary SQLite database, reopen that database,
read the append-only `temporal_lite_events` history, project deterministic
replay rows, checkpoint/rollback anchors, lease/idempotency readback rows,
event-log/SQLite adapter readback rows, WorkGraph projection rows,
replay-alignment rows, and then derive checkpoint consistency rows from those
local replay-alignment rows.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-replay-alignment-local-persistence-readback-report.sh`
- Source surface:
  `workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback`
- Source entries: 9
- Source table:
  `temporal_lite_events`
- Source scope:
  `local_persistence_work_graph_projection_replay_alignment_readback_no_execution`
- Source runtime state: runtime event-log write, runtime SQLite write, runtime
  store persistence, WorkGraph projection write, projection alignment
  persistence, workflow execution, replay execution, rollback execution, and
  live execution are all disabled

## Checkpoint Consistency Readback

For each local replay-alignment row, this slice projects:

- `checkpoint_consistency_key`
- `checkpoint_readback_key`
- `checkpoint_consistency_digest`
- `expected_checkpoint_projection_key`
- replay/checkpoint match flag
- checkpoint write-denial flag
- checkpoint consistency persistence-denial flag

Expected counts:

- `checkpoint_consistency_projection_count = 9`
- `checkpoint_consistency_key_count = 9`
- `checkpoint_digest_count = 9`
- `replay_alignment_checkpoint_match_count = 9`
- `sqlite_readback_validated_count = 9`
- `checkpoint_mismatch_count = 0`
- `replay_executed_count = 0`
- `checkpoint_written_count = 0`
- `rollback_anchor_written_count = 0`
- `consistency_persisted_count = 0`
- `work_graph_store_write_count = 0`
- `event_log_write_count = 0`
- `sqlite_write_count = 0`

## Closed Boundary

This slice has no replay execution, checkpoint write, rollback anchor write, checkpoint consistency persistence, WorkGraph projection write, runtime event-log write, runtime SQLite write, runtime store persistence, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The report is readback-only. The only SQLite read/write activity is covered by
Rust tests against a temporary local database, and the runtime flags continue
to project all checkpoint write, replay execution, rollback execution, and live
paths as false.

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-replay-alignment-checkpoint-consistency-local-persistence-readback-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-replay-alignment-checkpoint-consistency-local-persistence-readback-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback.rs`

## Next Move

The next workflow slice should be:

`workflow_temporal_lite_replay_alignment_rollback_consistency_local_persistence_readback`

That slice should compare the local checkpoint consistency projection with
rollback readback anchors from the SQLite/WAL event history while keeping
runtime event-log writes, runtime SQLite writes, durable-store persistence,
workflow execution, replay execution, rollback execution, transport mutation,
release, canary activation, and live execution closed.
