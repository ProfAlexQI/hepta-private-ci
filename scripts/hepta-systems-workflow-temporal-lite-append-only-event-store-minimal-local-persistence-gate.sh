#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_MINIMAL_LOCAL_PERSISTENCE_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite minimal local persistence report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite append-only event store test implementation gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite minimal local persistence architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite minimal local persistence report"
fi

grep -q 'Temporal-Lite Append-Only Event Store Minimal Local Persistence' "$DOC" \
  || fail "architecture note must document Temporal-Lite Append-Only Event Store Minimal Local Persistence"
grep -q 'SQLite/WAL minimal local persistence' "$DOC" \
  || fail "architecture note must document SQLite/WAL minimal local persistence"
grep -q 'temporal_lite_events' "$DOC" \
  || fail "architecture note must document the temporal_lite_events table"
grep -q 'no runtime event-log write, runtime SQLite write, runtime store persistence, runtime lease acquisition, runtime idempotency index persistence, runtime checkpoint write, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed runtime persistence/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_append_only_event_store_minimal_local_persistence"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_append_only_event_store_minimal_local_persistence_gate"
  and .schema_version == "workflow_temporal_lite_append_only_event_store_minimal_local_persistence_v1"
  and .source_test_implementation_ready == true
  and .source_test_event_count == 9
  and .source_accepted_append_count == 9
  and .source_duplicate_append_denial_count == 9
  and .lib_export_present == true
  and .store_type_present == true
  and .rust_sqlite_wal_config_present == true
  and .rust_idempotency_unique_index_present == true
  and .rust_deterministic_reopen_test_present == true
  and .sqlite_adapter_scope == "local_tempdb_sqlite_wal_append_only_store_test_covered_runtime_write_blocked"
  and .sqlite_table_count == 1
  and .sqlite_unique_index_count == 2
  and .sqlite_primary_table == "temporal_lite_events"
  and .wal_mode_required == true
  and .wal_mode_test_covered == true
  and .local_tempdb_persistence_test_covered == true
  and .local_event_contract_count == 9
  and .append_attempt_count == 18
  and .accepted_append_count == 9
  and .duplicate_append_denial_count == 9
  and .append_only_sequence_count == 9
  and .idempotency_unique_index_entry_count == 9
  and .checkpoint_anchor_count == 9
  and .replay_digest_count == 9
  and .deterministic_replay_validation_count == 9
  and .rollback_anchor_count == 9
  and .feature_gate_required == true
  and .runtime_feature_gate_enabled == false
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .runtime_store_persistence_allowed == false
  and .runtime_lease_acquire_allowed == false
  and .runtime_checkpoint_write_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .local_tempdb_sqlite_write_covered_by_tests == true
  and .minimal_local_persistence_ready == true
  and (.entries | length) == 9
  and (.entries | all(.source_sequence >= 1 and .source_sequence <= 9 and .local_sequence_projection == .source_sequence and (.event_id | startswith("temporal-lite.test-event.")) and (.aggregate_id | startswith("workflow://hepta/test-only/")) and (.idempotency_key | startswith("test-only.idempotency.")) and (.checkpoint_key | startswith("test-only.checkpoint.")) and (.replay_digest | startswith("replay-digest.v1.")) and (.rollback_anchor | length > 0) and .sqlite_table == "temporal_lite_events" and .sqlite_primary_sequence == "sequence" and .event_id_unique_constraint == "temporal_lite_events.event_id_unique" and .idempotency_unique_index == "idx_temporal_lite_events_idempotency_key" and .append_state == "covered_by_tempdb_sqlite_wal_test" and .duplicate_append_state == "duplicate_denied_by_unique_idempotency_key" and .appended_to_local_tempdb_test_store == true and .append_only_order_validated == true and .idempotency_unique_index_validated == true and .duplicate_append_denied == true and .checkpoint_anchor_projected == true and .replay_digest_projected == true and .deterministic_replay_digest_validated == true and .rollback_anchor_validated == true and .wal_mode_required == true and .local_tempdb_sqlite_write_covered_by_test == true and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .runtime_store_persistence_allowed == false and .runtime_lease_acquire_allowed == false and .runtime_checkpoint_write_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .source_sequence == 1)
  and any(.entries[]; .event_contract_id == "worker_task_event_intake" and .rollback_anchor == "rollback_to_prior_worker_task_attempt_anchor")
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("runtime_store_persistence_disabled")) != null
  and (.blockers | index("runtime_lease_acquire_disabled")) != null
  and (.blockers | index("runtime_checkpoint_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback")) != null
  and .recommended_next_gate == "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_append_only_event_store_minimal_local_persistence --lib
)

printf 'hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-gate: PASS: Temporal-lite minimal local SQLite/WAL persistence is test-covered, idempotent, replay-readable, and runtime-write/live blocked\n'
