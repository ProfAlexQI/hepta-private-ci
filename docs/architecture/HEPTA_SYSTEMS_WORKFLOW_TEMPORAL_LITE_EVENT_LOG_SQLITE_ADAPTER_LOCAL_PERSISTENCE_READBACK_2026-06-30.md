# Hepta Systems Workflow Temporal-Lite Event-Log SQLite Adapter Local Persistence Readback - 2026-06-30

This slice implements
`workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback`.
It consumes the SQLite/WAL-backed lease and idempotency-index readback and
projects event-log plus SQLite adapter readback rows while keeping runtime
event-log writes, runtime SQLite writes, adapter persistence, canary, and live
paths closed.

This is a local persistence event-log and SQLite adapter readback. The Rust
tests write only to a temporary SQLite database, reopen that database, read the
append-only `temporal_lite_events` history, project deterministic replay rows,
checkpoint/rollback anchors, lease/idempotency readback rows, and then project
event-log/SQLite adapter keys from those local rows.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-report.sh`
- Source surface:
  `workflow_temporal_lite_lease_idempotency_index_local_persistence_readback`
- Source entries: 9
- Source table:
  `temporal_lite_events`
- Source scope:
  `local_persistence_lease_idempotency_readback_no_acquire_no_persistence`
- Source runtime state: runtime event-log write, runtime SQLite write, runtime
  store persistence, lease acquisition, lease persistence, idempotency index
  write, idempotency index persistence, workflow execution, replay execution,
  rollback execution, and live execution are all disabled

## Adapter Readback

For each local lease/idempotency readback pair, this slice projects:

- `event_log_adapter_key`
- `event_log_stream`
- `event_log_record_key`
- `event_log_record_schema`
- `sqlite_adapter_key`
- `sqlite_table`
- `sqlite_row_key`
- `sqlite_schema_version`
- `serialization_contract_key`
- `transaction_boundary_key`
- adapter write-denial and persistence-denial flags

Expected counts:

- `event_log_adapter_readback_count = 9`
- `sqlite_adapter_readback_count = 9`
- `event_log_record_key_count = 9`
- `sqlite_row_key_count = 9`
- `serialization_contract_count = 9`
- `transaction_boundary_count = 9`
- `sqlite_readback_validated_count = 9`
- `event_log_record_written_count = 0`
- `sqlite_row_written_count = 0`
- `adapter_persisted_count = 0`
- `adapter_mismatch_count = 0`

## Closed Boundary

This slice has no runtime event-log write, runtime SQLite write, runtime store persistence, event-log adapter write, SQLite adapter write, adapter persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The report is readback-only. The only SQLite read/write activity is covered by
Rust tests against a temporary local database, and the runtime flags continue
to project all write, replay execution, rollback execution, and live paths as
false.

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback.rs`

## Next Move

The next workflow slice should be:

`workflow_temporal_lite_work_graph_projection_local_persistence_readback`

That slice should move WorkGraph projection readback onto the same SQLite/WAL
local event history while keeping runtime event-log writes, runtime SQLite
writes, durable-store persistence, workflow execution, replay execution,
rollback execution, transport mutation, release, canary activation, and live
execution closed.
