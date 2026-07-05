# Hepta Systems Workflow Temporal-Lite WorkGraph Projection Replay Alignment Local Persistence Readback - 2026-06-30

This slice implements
`workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback`.
It consumes the SQLite/WAL-backed WorkGraph projection local persistence
readback and projects replay-alignment keys, projection-replay keys, replay
alignment checksums, deterministic alignment flags, and write-denial flags while
keeping replay execution, projection alignment persistence, runtime writes,
canary, and live paths closed.

This is a local persistence WorkGraph projection replay alignment readback. The
Rust tests write only to a temporary SQLite database, reopen that database, read
the append-only `temporal_lite_events` history, project deterministic replay
rows, checkpoint/rollback anchors, lease/idempotency readback rows,
event-log/SQLite adapter readback rows, WorkGraph projection rows, and then
derive replay-alignment rows from those local projection rows.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-local-persistence-readback-report.sh`
- Source surface:
  `workflow_temporal_lite_work_graph_projection_local_persistence_readback`
- Source entries: 9
- Source table:
  `temporal_lite_events`
- Source scope:
  `local_persistence_work_graph_projection_readback_no_persistence`
- Source runtime state: runtime event-log write, runtime SQLite write, runtime
  store persistence, WorkGraph projection write, WorkGraph projection
  persistence, workflow execution, replay execution, rollback execution, and
  live execution are all disabled

## Replay Alignment Readback

For each local WorkGraph projection row, this slice projects:

- `replay_alignment_key`
- `projection_replay_key`
- `replay_alignment_checksum`
- `expected_replay_projection_key`
- deterministic alignment flag
- replay execution denial flag
- projection alignment persistence denial flag

Expected counts:

- `replay_alignment_projection_count = 9`
- `projection_replay_key_count = 9`
- `replay_alignment_checksum_count = 9`
- `deterministic_alignment_count = 9`
- `sqlite_readback_validated_count = 9`
- `replay_alignment_mismatch_count = 0`
- `replay_executed_count = 0`
- `projection_alignment_persisted_count = 0`
- `work_graph_store_write_count = 0`
- `event_log_write_count = 0`
- `sqlite_write_count = 0`

## Closed Boundary

This slice has no replay execution, projection alignment persistence, WorkGraph projection write, runtime event-log write, runtime SQLite write, runtime store persistence, workflow execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The report is readback-only. The only SQLite read/write activity is covered by
Rust tests against a temporary local database, and the runtime flags continue
to project all write, replay execution, rollback execution, and live paths as
false.

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-replay-alignment-local-persistence-readback-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-replay-alignment-local-persistence-readback-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback.rs`

## Next Move

The next workflow slice should be:

`workflow_temporal_lite_replay_alignment_checkpoint_consistency_local_persistence_readback`

That slice should compare the local replay-alignment projection with checkpoint
readback anchors from the SQLite/WAL event history while keeping runtime
event-log writes, runtime SQLite writes, durable-store persistence, workflow
execution, replay execution, rollback execution, transport mutation, release,
canary activation, and live execution closed.
