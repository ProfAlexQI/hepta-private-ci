#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-feature-gated-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_FEATURE_GATED_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-feature-gated-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Temporal-lite event-log/SQLite adapter report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Temporal-lite lease/idempotency gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite event-log/SQLite adapter architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Temporal-lite event-log/SQLite adapter report"
fi

grep -q 'Temporal-Lite Event-Log SQLite Adapter Feature-Gated Readback' "$DOC" \
  || fail "architecture note must document Temporal-Lite Event-Log SQLite Adapter Feature-Gated Readback"
grep -q 'test-only event-log and SQLite adapter readback' "$DOC" \
  || fail "architecture note must document test-only event-log and SQLite adapter readback"
grep -q 'no event-log write, SQLite write, adapter persistence, workflow execution, replay execution, rollback execution, provider invocation, model invocation, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed adapter/live boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback"
  and .status == "ready_blocked"
  and .gate == "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_gate"
  and .schema_version == "workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback_v1"
  and .source_lease_idempotency_ready == true
  and .source_lease_idempotency_entry_count == 9
  and .lib_export_present == true
  and .adapter_scope == "test_only_event_log_sqlite_adapter_readback_no_writes"
  and .event_log_adapter_readback_count == 9
  and .sqlite_adapter_readback_count == 9
  and .event_log_record_key_count == 9
  and .sqlite_row_key_count == 9
  and .serialization_contract_count == 9
  and .transaction_boundary_count == 9
  and .event_log_record_written_count == 0
  and .sqlite_row_written_count == 0
  and .adapter_persisted_count == 0
  and .feature_gate_required == true
  and .runtime_feature_gate_enabled == false
  and .adapter_contract_readback_materialized == true
  and .runtime_event_log_write_allowed == false
  and .runtime_sqlite_write_allowed == false
  and .event_log_adapter_write_allowed == false
  and .sqlite_adapter_write_allowed == false
  and .adapter_persistence_allowed == false
  and .workflow_execution_allowed == false
  and .replay_execution_allowed == false
  and .rollback_execution_allowed == false
  and .live_execution_allowed == false
  and .event_log_sqlite_adapter_readback_ready == true
  and (.entries | length) == 9
  and (.entries | all(.sequence >= 1 and .sequence <= 9 and (.event_id | startswith("temporal-lite.test-event.")) and (.lease_key | startswith("temporal-lite.lease.readback.")) and (.idempotency_index_key | startswith("temporal-lite.idempotency-index.readback.")) and (.idempotency_key | startswith("idempotency-key.v1.")) and (.event_log_adapter_key | startswith("temporal-lite.event-log.adapter.readback.")) and .event_log_stream == "temporal_lite_test_only_event_log_stream" and (.event_log_record_key | startswith("event-log-record.v1.")) and .event_log_record_schema == "temporal_lite_event_log_record_v1" and (.sqlite_adapter_key | startswith("temporal-lite.sqlite.adapter.readback.")) and .sqlite_table == "temporal_lite_test_only_events" and (.sqlite_row_key | startswith("sqlite-row.v1.")) and .sqlite_schema_version == "temporal_lite_sqlite_adapter_v1" and (.serialization_contract_key | startswith("temporal-lite.serialization-contract.readback.")) and (.transaction_boundary_key | startswith("temporal-lite.transaction-boundary.readback.")) and .adapter_state == "projected_not_persisted" and .readback_state == "adapter_contract_projected_in_memory_only" and .event_log_adapter_projected == true and .sqlite_adapter_projected == true and .serialization_contract_projected == true and .transaction_boundary_projected == true and .event_log_record_written == false and .sqlite_row_written == false and .adapter_persisted == false and .feature_gate_required == true and .runtime_feature_gate_enabled == false and .runtime_event_log_write_allowed == false and .runtime_sqlite_write_allowed == false and .workflow_execution_allowed == false and .replay_execution_allowed == false and .rollback_execution_allowed == false and .live_execution_allowed == false))
  and any(.entries[]; .event_contract_id == "plan_step_event_intake" and .sequence == 1)
  and any(.entries[]; .event_contract_id == "approval_event_intake" and (.event_log_record_key | startswith("event-log-record.v1.")))
  and any(.entries[]; .event_contract_id == "task_result_event_intake" and (.sqlite_row_key | startswith("sqlite-row.v1.")))
  and (.blockers | index("runtime_feature_gate_closed")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("runtime_sqlite_write_disabled")) != null
  and (.blockers | index("event_log_adapter_write_disabled")) != null
  and (.blockers | index("sqlite_adapter_write_disabled")) != null
  and (.blockers | index("adapter_persistence_disabled")) != null
  and (.blockers | index("workflow_execution_disabled")) != null
  and (.blockers | index("replay_execution_disabled")) != null
  and (.blockers | index("rollback_execution_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("temporal_lite_work_graph_projection_feature_gated_readback")) != null
  and .recommended_next_gate == "temporal_lite_work_graph_projection_feature_gated_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime workflow_temporal_lite_event_log_sqlite_adapter_feature_gated_readback --lib
)

printf 'hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-feature-gated-readback-gate: PASS: Temporal-lite event-log/SQLite adapter contract is projected, feature-gated, not persisted, and runtime-write/live blocked\n'
