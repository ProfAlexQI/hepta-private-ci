# Hepta Systems Workflow Temporal-Lite Append-Only Event Store Minimal Local Persistence - 2026-06-30

This slice implements
`workflow_temporal_lite_append_only_event_store_minimal_local_persistence`.
It moves the Temporal-lite workflow store from a test-only in-memory append
model to SQLite/WAL minimal local persistence coverage while keeping runtime
persistence, workflow execution, replay execution, rollback execution, canary,
and live paths closed.

This is SQLite/WAL minimal local persistence. The Rust tests open a temporary
SQLite database, migrate the local append-only table, append the nine workflow
durable store fixture events, deny duplicate idempotency keys, reopen the
database, and verify deterministic replay readback from the stored event
history.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation-report.sh`
- Source surface:
  `workflow_temporal_lite_append_only_event_store_test_implementation`
- Source entries: 9
- Source scope:
  `test_only_in_memory_append_only_store_no_runtime_persistence`
- Source feature gate: required and disabled
- Source write state: runtime event-log write, runtime SQLite write, store
  persistence, workflow execution, replay execution, rollback execution, and
  live execution are all disabled

## Local SQLite Store

The local store is `WorkflowTemporalLiteMinimalLocalEventStore`. It is covered
by tests only and uses a temporary SQLite database with WAL mode enabled.

The primary table is `temporal_lite_events`.

Important columns and constraints:

- `sequence INTEGER PRIMARY KEY AUTOINCREMENT`
- `event_id TEXT NOT NULL UNIQUE`
- `aggregate_id TEXT NOT NULL`
- `idempotency_key TEXT NOT NULL UNIQUE`
- `checkpoint_key TEXT NOT NULL`
- `replay_digest TEXT NOT NULL`
- `rollback_anchor TEXT NOT NULL`
- `payload_json TEXT NOT NULL`

Important indexes:

- `idx_temporal_lite_events_idempotency_key`
- `idx_temporal_lite_events_aggregate_sequence`

The store intentionally accepts SQLite's real sequence semantics. Duplicate
`INSERT OR IGNORE` attempts may leave gaps in the autoincrement sequence, so
the invariant is accepted-row ordering and deterministic replay, not gapless
sequence numbers.

## Expected Counts

- `local_event_contract_count = 9`
- `append_attempt_count = 18`
- `accepted_append_count = 9`
- `duplicate_append_denial_count = 9`
- `append_only_sequence_count = 9`
- `idempotency_unique_index_entry_count = 9`
- `checkpoint_anchor_count = 9`
- `replay_digest_count = 9`
- `deterministic_replay_validation_count = 9`
- `rollback_anchor_count = 9`

## Closed Boundary

This slice has no runtime event-log write, runtime SQLite write, runtime store persistence, runtime lease acquisition, runtime idempotency index persistence, runtime checkpoint write, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution.

The only write performed by the implementation is inside Rust tests against a
temporary SQLite database. The runtime report continues to project all runtime
write and live flags as false.

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_append_only_event_store_minimal_local_persistence.rs`

## Next Move

The next workflow slice should be:

`workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback`

That slice should validate deterministic replay against the SQLite/WAL local
store readback while keeping runtime event-log writes, runtime SQLite writes,
workflow execution, replay execution, rollback execution, transport mutation,
release, canary activation, and live execution closed.
