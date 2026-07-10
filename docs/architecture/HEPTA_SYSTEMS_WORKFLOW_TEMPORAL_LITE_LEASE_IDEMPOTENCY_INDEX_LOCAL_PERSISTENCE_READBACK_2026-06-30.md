# Hepta Systems Workflow Temporal-Lite Lease Idempotency Index Local Persistence Readback - 2026-06-30

This slice implements
`workflow_temporal_lite_lease_idempotency_index_local_persistence_readback`.
It consumes the SQLite/WAL-backed checkpoint and rollback anchor readback and
projects lease plus idempotency-index readback rows while keeping runtime lease
acquisition, idempotency writes, workflow execution, rollback execution,
canary, and live paths closed.

This is a local persistence lease and idempotency readback. The Rust tests
write only to a temporary SQLite database, reopen that database, read the
append-only `temporal_lite_events` history, project deterministic replay rows,
project checkpoint/rollback anchors, and then project lease and idempotency
index readback keys from those anchors.

The lease and idempotency readback now carries the same single append-only event store interface provenance from the minimal local persistence source,
through replay validation and checkpoint/rollback anchor readback. The source
must report `source_append_only_event_store_interface_ready = true` and
`checkpoint_anchors_derived_from_event_store_interface = true` before this
slice can be ready. This is still readback-only; it does not acquire leases or
write idempotency indexes.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-local-persistence-readback-report.sh`
- Source surface:
  `workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback`
- Source entries: 9
- Source interface:
  `WorkflowTemporalLiteAppendOnlyEventStore`
- Source interface provenance:
  `source_append_only_event_store_interface_ready = true`
- Source table:
  `temporal_lite_events`
- Source scope:
  `local_persistence_checkpoint_and_rollback_anchor_readback_no_writes`
- Source runtime state: runtime event-log write, runtime SQLite write, runtime
  store persistence, checkpoint write, rollback anchor write, anchor
  persistence, workflow execution, replay execution, rollback execution, and
  live execution are all disabled

## Lease And Idempotency Readback

For each local checkpoint/rollback anchor pair, this slice projects:

- `lease_readback_key`
- `lease_scope_key`
- `lease_owner`
- `lease_ttl_ms`
- `lease_digest`
- `idempotency_index_readback_key`
- `idempotency_key`
- `idempotency_digest`
- `duplicate_guard_key`
- lease and idempotency write-denial flags

Expected counts:

- `lease_readback_count = 9`
- `idempotency_index_readback_count = 9`
- `duplicate_guard_readback_count = 9`
- `lease_digest_count = 9`
- `idempotency_digest_count = 9`
- `lease_idempotency_pair_count = 9`
- `lease_acquired_count = 0`
- `lease_persisted_count = 0`
- `idempotency_index_written_count = 0`
- `idempotency_index_persisted_count = 0`
- `lease_idempotency_mismatch_count = 0`
- `lease_idempotency_derived_from_event_store_interface = true`

## Closed Boundary

This slice has no runtime event-log write, runtime SQLite write, runtime store persistence, lease acquisition, lease persistence, idempotency index write, idempotency index persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The report is readback-only. The only SQLite read/write activity is covered by
Rust tests against a temporary local database, and the runtime flags continue
to project all write, replay execution, rollback execution, and live paths as
false.

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_lease_idempotency_index_local_persistence_readback.rs`

## Next Move

The next workflow slice should be:

`workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback`

That slice should move the event-log/SQLite adapter readback onto the same
SQLite/WAL local event history while keeping runtime event-log writes, runtime
SQLite writes, durable-store persistence, workflow execution, replay execution,
rollback execution, transport mutation, release, canary activation, and live
execution closed.
