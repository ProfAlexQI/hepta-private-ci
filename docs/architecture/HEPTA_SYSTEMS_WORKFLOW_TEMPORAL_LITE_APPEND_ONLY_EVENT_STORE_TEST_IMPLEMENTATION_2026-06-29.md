# Hepta Systems Workflow Temporal-Lite Append-Only Event Store Test Implementation - 2026-06-29

This slice implements
`temporal_lite_append_only_event_store_feature_gated_test_implementation`.
It moves workflow progress from a fixture-only readback toward a real
append-only store core, while keeping the implementation test-only and
in-memory.

This is a Temporal-Lite Append-Only Event Store Test Implementation. It is a
test-only in-memory append-only store that accepts the nine workflow durable
store fixture events, rejects duplicate idempotency keys, projects checkpoint
anchors, projects replay digests, and preserves rollback anchors.

## Source Boundary

- Source report:
  `scripts/hepta-systems-workflow-durable-store-test-only-append-fixture-report.sh`
- Source surface:
  `workflow_durable_store_test_only_append_fixture`
- Source fixture entries: 9
- Source scope:
  `test_only_in_memory_fixture_no_runtime_store_write`
- Source feature gate: required and disabled
- Source write state: runtime event-log write, SQLite write, fixture persistence, workflow execution, replay execution, rollback execution, and live execution are all disabled

## Test Store Behavior

The Rust read model materializes a `WorkflowTemporalLiteAppendOnlyTestStore`.
For each fixture event it performs two append attempts:

- First append: accepted into the in-memory store.
- Second append with the same idempotency key: denied as a duplicate.

Expected counts:

- `test_event_count = 9`
- `append_attempt_count = 18`
- `accepted_append_count = 9`
- `duplicate_append_denial_count = 9`
- `append_only_sequence_count = 9`
- `idempotency_index_entry_count = 9`
- `checkpoint_anchor_count = 9`
- `replay_digest_count = 9`
- `rollback_anchor_count = 9`

## Closed Boundary

This slice has no runtime event-log write, SQLite write, store persistence, lease acquisition, idempotency index persistence, checkpoint write, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution.

The implementation keeps these fields false:

- `runtime_feature_gate_enabled`
- `runtime_event_log_write_allowed`
- `runtime_sqlite_write_allowed`
- `store_persistence_allowed`
- `workflow_execution_allowed`
- `replay_execution_allowed`
- `rollback_execution_allowed`
- `live_execution_allowed`

## Local Gates

- Report:
  `scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation-report.sh`
- Gate:
  `scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation-gate.sh`
- Rust module:
  `codex-rs/hepta-runtime/src/workflow_temporal_lite_append_only_event_store_test_implementation.rs`

## Next Move

The next workflow slice should be:

`temporal_lite_deterministic_replay_validator_feature_gated_readback`

That slice should validate deterministic replay from the in-memory event history
without opening runtime writes, SQLite writes, workflow execution, replay
execution, rollback execution, transport mutation, release, canary activation,
or live execution.
