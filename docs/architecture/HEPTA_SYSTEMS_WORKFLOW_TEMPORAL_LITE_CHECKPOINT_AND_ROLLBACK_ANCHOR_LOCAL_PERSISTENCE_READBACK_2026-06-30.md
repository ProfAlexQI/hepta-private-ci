# Hepta Systems Workflow Temporal-Lite Checkpoint And Rollback Anchor Local Persistence Readback - 2026-06-30

This slice implements
`workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback`.
It takes deterministic replay readback from the SQLite/WAL local event history
and projects checkpoint and rollback anchor readback rows while keeping runtime
checkpoint writes, rollback anchor writes, anchor persistence, replay
execution, rollback execution, canary, and live paths closed.

This is a local persistence checkpoint and rollback anchor readback. The Rust
tests write only to a temporary SQLite database, reopen that database, read the
append-only `temporal_lite_events` history, project deterministic replay
readback rows, and then project checkpoint/rollback anchor keys and digests
from those replay rows.

The checkpoint and rollback anchor readback is now explicitly derived from the
same single append-only event store interface provenance carried by the
deterministic replay validator. The source replay validator must report
`source_append_only_event_store_interface_ready = true` and
`replay_validator_derived_from_event_store_interface = true` before this slice
can be ready. This keeps checkpoint and rollback anchors derived from the local
event-store interface without writing checkpoints, rollback anchors, or runtime
state.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-report.sh`
- Source surface:
  `workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback`
- Source entries: 9
- Source interface:
  `WorkflowTemporalLiteAppendOnlyEventStore`
- Source interface provenance:
  `source_append_only_event_store_interface_ready = true`
- Source table:
  `temporal_lite_events`
- Source scope:
  `local_tempdb_sqlite_wal_readback_test_covered_runtime_read_write_blocked`
- Source runtime state: runtime event-log write, runtime SQLite write, runtime
  store persistence, replay projection persistence, workflow execution, replay
  execution, rollback execution, and live execution are all disabled

## Anchor Readback

For each local replay readback row, this slice projects:

- `checkpoint_anchor_key`
- `rollback_anchor_key`
- `checkpoint_source_key`
- `rollback_source_anchor`
- `checkpoint_readback_digest`
- `rollback_readback_digest`
- `durable_anchor_pair_projected`
- checkpoint and rollback write-denial flags

Expected counts:

- `replay_readback_projection_count = 9`
- `checkpoint_anchor_readback_count = 9`
- `rollback_anchor_readback_count = 9`
- `durable_anchor_pair_count = 9`
- `checkpoint_digest_count = 9`
- `rollback_digest_count = 9`
- `anchor_mismatch_count = 0`
- `checkpoint_anchors_derived_from_event_store_interface = true`

## Closed Boundary

This slice has no runtime event-log write, runtime SQLite write, runtime store persistence, checkpoint write, rollback anchor write, anchor persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The report is readback-only. The only SQLite read/write activity is covered by
Rust tests against a temporary local database, and the runtime flags continue
to project all write, replay execution, rollback execution, and live paths as
false.

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-local-persistence-readback-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-local-persistence-readback-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback.rs`

## Next Move

The next workflow slice should be:

`workflow_temporal_lite_lease_idempotency_index_local_persistence_readback`

That slice should move lease and idempotency index readback onto the SQLite/WAL
local event history while keeping runtime event-log writes, runtime SQLite
writes, lease acquisition, idempotency index persistence, workflow execution,
replay execution, rollback execution, transport mutation, release, canary
activation, and live execution closed.
