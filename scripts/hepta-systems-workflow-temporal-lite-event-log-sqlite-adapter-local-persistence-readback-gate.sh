#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_LOCAL_PERSISTENCE_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite local event-log/SQLite adapter report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite local lease/idempotency gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite local event-log/SQLite adapter architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite local event-log/SQLite adapter report"
fi

grep -q 'Temporal-Lite Event-Log SQLite Adapter Local Persistence Readback' "$DOC" \
  || fail "architecture note must document Temporal-Lite Event-Log SQLite Adapter Local Persistence Readback"
grep -q 'local persistence event-log and SQLite adapter readback' "$DOC" \
  || fail "architecture note must document local persistence event-log and SQLite adapter readback"
grep -q 'no runtime event-log write, runtime SQLite write, runtime store persistence, event-log adapter write, SQLite adapter write, adapter persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, canary activation, or live execution' "$DOC" \
  || fail "architecture note must document the closed adapter/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_gate"
  and .schema_version == "workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_v1"
  and .source_lease_idempotency_ready == true
  and .source_anchor_pair_count == 9
  and .source_append_only_event_store_interface_ready == true
  and .source_lease_idempotency_derived_from_event_store_interface == true
  and .lib_export_present == true
  and .reopened_sqlite_adapter_test_present == true
  and .adapter_scope == "local_persistence_event_log_sqlite_adapter_readback_no_runtime_writes"
  and .sqlite_readback_scope == "local_tempdb_sqlite_wal_readback_test_covered_runtime_read_write_blocked"
  and .event_log_adapter_readback_count == 9
  and .sqlite_adapter_readback_count == 9
  and .event_log_record_key_count == 9
  and .sqlite_row_key_count == 9
  and .serialization_contract_count == 9
  and .transaction_boundary_count == 9
  and .sqlite_readback_validated_count == 9
  and .event_log_record_written_count == 0
  and .sqlite_row_written_count == 0
  and .adapter_persisted_count == 0
  and .adapter_mismatch_count == 0
  and .wal_mode_required == true
  and .local_tempdb_sqlite_read_covered_by_tests == true
  and .runtime_feature_gate_enabled == false
  and .adapter_contract_readback_materialized == true
  and .event_log_sqlite_adapter_derived_from_event_store_interface == true
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .runtime_store_persistence_allowed == false
  and .event_log_adapter_write_allowed == false
  and .sqlite_adapter_write_allowed == false
  and .adapter_persistence_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .event_log_sqlite_adapter_local_persistence_readback_ready == true
  and (.entries | length) == 9
  and (.entries | all(.replay_order >= 1 and .replay_order <= 9 and .local_sequence >= 1 and (.source_event_id | startswith("temporal-lite.test-event.")) and (.replay_projection_key | startswith("temporal-lite.local-persistence-replay.")) and (.lease_readback_key | startswith("temporal-lite.local-lease-readback.")) and (.idempotency_index_readback_key | startswith("temporal-lite.local-idempotency-index-readback.")) and (.idempotency_key | startswith("idempotency-key.local.v1.")) and (.event_log_adapter_key | startswith("temporal-lite.local-event-log-adapter-readback.")) and .event_log_stream == "temporal_lite_local_persistence_event_log_stream" and (.event_log_record_key | startswith("temporal-lite.local-event-log-record.v1.")) and .event_log_record_schema == "temporal_lite_local_event_log_record_v1" and (.sqlite_adapter_key | startswith("temporal-lite.local-sqlite-adapter-readback.")) and .sqlite_table == "temporal_lite_events" and (.sqlite_row_key | startswith("temporal-lite.local-sqlite-row.v1.")) and .sqlite_schema_version == "temporal_lite_local_sqlite_adapter_v1" and (.serialization_contract_key | startswith("temporal-lite.local-serialization-contract-readback.")) and (.transaction_boundary_key | startswith("temporal-lite.local-transaction-boundary-readback.")) and .adapter_state == "projected_from_local_persistence_not_persisted" and .readback_state == "projected_from_sqlite_wal_local_persistence_readback_without_runtime_writes" and .event_log_adapter_projected == true and .sqlite_adapter_projected == true and .serialization_contract_projected == true and .transaction_boundary_projected == true and .sqlite_readback_validated == true and .adapter_mismatch_detected == false and .event_log_record_written == false and .sqlite_row_written == false and .adapter_persisted == false and .wal_mode_required == true and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .runtime_store_persistence_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .replay_order == 1)
  and any(.entries[]; .event_contract_id == "approval_event_intake" and (.event_log_record_key | startswith("temporal-lite.local-event-log-record.v1.")))
  and any(.entries[]; .event_contract_id == "task_result_event_intake" and (.sqlite_row_key | startswith("temporal-lite.local-sqlite-row.v1.")))
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("runtime_store_persistence_disabled")) != null
  and (.blockers | index("event_log_adapter_write_disabled")) != null
  and (.blockers | index("sqlite_adapter_write_disabled")) != null
  and (.blockers | index("adapter_persistence_disabled")) != null
  and (.blockers | index("workflow_execution_disabled")) != null
  and (.blockers | index("replay_execution_disabled")) != null
  and (.blockers | index("rollback_execution_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("workflow_temporal_lite_work_graph_projection_local_persistence_readback")) != null
  and .recommended_next_gate == "workflow_temporal_lite_work_graph_projection_local_persistence_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback --lib
)

printf 'hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-gate: PASS: Temporal-lite local event-log/SQLite adapter reads back from SQLite/WAL history, stays mismatch-free, and keeps runtime writes/live closed\n'
