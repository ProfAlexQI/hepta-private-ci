# Hepta Systems Workflow Temporal-Lite WorkGraph Projection Local Persistence Readback - 2026-06-30

This slice implements
`workflow_temporal_lite_work_graph_projection_local_persistence_readback`.
It consumes the SQLite/WAL-backed event-log/SQLite adapter readback and projects
WorkGraph node, event-edge, state-edge, projection key, and checksum readback
rows while keeping WorkGraph projection persistence, runtime writes, canary, and
live paths closed.
It now requires the same single append-only event store interface provenance
that starts at the minimal local persistence backend and flows through replay,
checkpoint/rollback, lease/idempotency, and the event-log/SQLite adapter.

This is a local persistence WorkGraph projection readback. The Rust tests write
only to a temporary SQLite database, reopen that database, read the append-only
`temporal_lite_events` history, project deterministic replay rows,
checkpoint/rollback anchors, lease/idempotency readback rows, event-log/SQLite
adapter readback rows, and then project WorkGraph readback rows from those local
adapter rows.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-report.sh`
- Source surface:
  `workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback`
- Source entries: 9
- Source interface provenance:
  `source_append_only_event_store_interface_ready = true`
- Source adapter derivation:
  `source_event_log_sqlite_adapter_derived_from_event_store_interface = true`
- Source table:
  `temporal_lite_events`
- Source scope:
  `local_persistence_event_log_sqlite_adapter_readback_no_runtime_writes`
- Source runtime state: runtime event-log write, runtime SQLite write, runtime
  store persistence, adapter writes, workflow execution, replay execution,
  rollback execution, and live execution are all disabled

## WorkGraph Projection Readback

For each local event-log/SQLite adapter readback row, this slice projects:

- `work_graph_node_key`
- `work_graph_node_kind`
- `work_graph_event_edge_key`
- `work_graph_state_edge_key`
- `projection_key`
- `projection_checksum`
- projection write-denial and persistence-denial flags

Expected counts:

- `work_graph_node_projection_count = 9`
- `work_graph_event_edge_projection_count = 9`
- `work_graph_state_edge_projection_count = 9`
- `projection_key_count = 9`
- `projection_checksum_count = 9`
- `sqlite_readback_validated_count = 9`
- `projection_persisted_count = 0`
- `work_graph_store_write_count = 0`
- `event_log_write_count = 0`
- `sqlite_write_count = 0`
- `projection_mismatch_count = 0`
- `work_graph_projection_derived_from_event_store_interface = true`

## Closed Boundary

This slice has no WorkGraph projection write, WorkGraph projection persistence, runtime event-log write, runtime SQLite write, runtime store persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The report is readback-only. The only SQLite read/write activity is covered by
Rust tests against a temporary local database, and the runtime flags continue
to project all write, replay execution, rollback execution, and live paths as
false.

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-local-persistence-readback-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-local-persistence-readback-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_work_graph_projection_local_persistence_readback.rs`

## Next Move

The next workflow slice should be:

`workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback`

That slice should align the local WorkGraph projection with replay order from
the SQLite/WAL event history while keeping runtime event-log writes, runtime
SQLite writes, durable-store persistence, workflow execution, replay execution,
rollback execution, transport mutation, release, canary activation, and live
execution closed.
