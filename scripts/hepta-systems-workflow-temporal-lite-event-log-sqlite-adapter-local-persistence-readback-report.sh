#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-local-persistence-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_LOCAL_PERSISTENCE_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Temporal-lite local lease/idempotency report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Temporal-lite local event-log/SQLite adapter Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite local event-log/SQLite adapter architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Temporal-lite local event-log/SQLite adapter report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

reopened_sqlite_adapter_test_present=false
if grep -q 'local_event_log_sqlite_adapter_uses_reopened_sqlite_event_history' "$RUST_SOURCE"; then
  reopened_sqlite_adapter_test_present=true
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --argjson reopened_sqlite_adapter_test_present "$reopened_sqlite_adapter_test_present" \
  --arg gate "scripts/hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_EVENT_LOG_SQLITE_ADAPTER_LOCAL_PERSISTENCE_READBACK_2026-06-30.md" \
  '
  def pad3($n):
    (($n | tostring) | if length == 1 then "00" + . elif length == 2 then "0" + . else . end);
  def keyed($prefix; $entry):
    $prefix + "." + pad3($entry.replay_order) + "." + $entry.event_contract_id;
  def adapter_entry($entry): {
    event_contract_id:$entry.event_contract_id,
    replay_order:$entry.replay_order,
    local_sequence:$entry.local_sequence,
    source_event_id:$entry.source_event_id,
    replay_projection_key:$entry.replay_projection_key,
    lease_readback_key:$entry.lease_readback_key,
    idempotency_index_readback_key:$entry.idempotency_index_readback_key,
    idempotency_key:$entry.idempotency_key,
    event_log_adapter_key:keyed("temporal-lite.local-event-log-adapter-readback"; $entry),
    event_log_stream:"temporal_lite_local_persistence_event_log_stream",
    event_log_record_key:("temporal-lite.local-event-log-record.v1." + pad3($entry.replay_order) + "." + $entry.event_contract_id + "." + (($entry.idempotency_key | length) | tostring) + "." + (($entry.local_sequence | tostring))),
    event_log_record_schema:"temporal_lite_local_event_log_record_v1",
    sqlite_adapter_key:keyed("temporal-lite.local-sqlite-adapter-readback"; $entry),
    sqlite_table:"temporal_lite_events",
    sqlite_row_key:("temporal-lite.local-sqlite-row.v1." + pad3($entry.replay_order) + "." + $entry.event_contract_id + "." + (($entry.source_event_id | length) | tostring) + "." + (($entry.local_sequence | tostring))),
    sqlite_schema_version:"temporal_lite_local_sqlite_adapter_v1",
    serialization_contract_key:keyed("temporal-lite.local-serialization-contract-readback"; $entry),
    transaction_boundary_key:keyed("temporal-lite.local-transaction-boundary-readback"; $entry),
    adapter_state:"projected_from_local_persistence_not_persisted",
    readback_state:"projected_from_sqlite_wal_local_persistence_readback_without_runtime_writes",
    event_log_adapter_projected:($entry.lease_readback_projected == true and $entry.idempotency_index_projected == true and $entry.duplicate_guard_projected == true and $entry.sqlite_readback_validated == true and $entry.lease_idempotency_mismatch_detected == false),
    sqlite_adapter_projected:($entry.lease_readback_projected == true and $entry.idempotency_index_projected == true and $entry.duplicate_guard_projected == true and $entry.sqlite_readback_validated == true and $entry.lease_idempotency_mismatch_detected == false),
    serialization_contract_projected:($entry.lease_readback_projected == true and $entry.idempotency_index_projected == true and $entry.duplicate_guard_projected == true and $entry.sqlite_readback_validated == true and $entry.lease_idempotency_mismatch_detected == false),
    transaction_boundary_projected:($entry.lease_readback_projected == true and $entry.idempotency_index_projected == true and $entry.duplicate_guard_projected == true and $entry.sqlite_readback_validated == true and $entry.lease_idempotency_mismatch_detected == false),
    sqlite_readback_validated:$entry.sqlite_readback_validated,
    adapter_mismatch_detected:$entry.lease_idempotency_mismatch_detected,
    event_log_record_written:false,
    sqlite_row_written:false,
    adapter_persisted:false,
    wal_mode_required:$entry.wal_mode_required,
    feature_gate_required:$entry.feature_gate_required,
    runtime_feature_gate_enabled:$entry.runtime_feature_gate_enabled,
    runtime_event_log_write_allowed:$entry.runtime_event_log_write_allowed,
    runtime_sqlite_write_allowed:$entry.runtime_sqlite_write_allowed,
    runtime_store_persistence_allowed:$entry.runtime_store_persistence_allowed,
    workflow_execution_allowed:$entry.workflow_execution_allowed,
    replay_execution_allowed:$entry.replay_execution_allowed,
    rollback_execution_allowed:$entry.rollback_execution_allowed,
    live_execution_allowed:$entry.live_execution_allowed
  };
  ($source[0]) as $source_report |
  ($source_report.entries | map(adapter_entry(.))) as $entries |
  ($entries | length) as $entry_count |
  ($entries | map(select(.event_log_adapter_projected == true)) | length) as $event_log_adapter_readback_count |
  ($entries | map(select(.sqlite_adapter_projected == true)) | length) as $sqlite_adapter_readback_count |
  ($entries | map(select((.event_log_record_key | length) > 0)) | length) as $event_log_record_key_count |
  ($entries | map(select((.sqlite_row_key | length) > 0)) | length) as $sqlite_row_key_count |
  ($entries | map(select(.serialization_contract_projected == true)) | length) as $serialization_contract_count |
  ($entries | map(select(.transaction_boundary_projected == true)) | length) as $transaction_boundary_count |
  ($entries | map(select(.sqlite_readback_validated == true)) | length) as $sqlite_readback_validated_count |
  ($entries | map(select(.event_log_record_written == true)) | length) as $event_log_record_written_count |
  ($entries | map(select(.sqlite_row_written == true)) | length) as $sqlite_row_written_count |
  ($entries | map(select(.adapter_persisted == true)) | length) as $adapter_persisted_count |
  ($entries | map(select(.adapter_mismatch_detected == true)) | length) as $adapter_mismatch_count |
  ($source_report.lease_idempotency_index_local_persistence_readback_ready == true
    and $source_report.source_append_only_event_store_interface_ready == true
    and $source_report.lease_idempotency_derived_from_event_store_interface == true
    and $source_report.source_anchor_pair_count == 9
    and $source_report.lease_readback_count == 9
    and $source_report.idempotency_index_readback_count == 9
    and $source_report.duplicate_guard_readback_count == 9
    and $source_report.lease_acquired_count == 0
    and $source_report.lease_persisted_count == 0
    and $source_report.idempotency_index_written_count == 0
    and $source_report.idempotency_index_persisted_count == 0
    and $source_report.lease_idempotency_mismatch_count == 0
    and $source_report.local_tempdb_sqlite_read_covered_by_tests == true
    and $source_report.runtime_feature_gate_enabled == false
    and $source_report.runtime_event_log_write_allowed == false
    and $source_report.runtime_sqlite_write_allowed == false
    and $source_report.runtime_store_persistence_allowed == false
    and $source_report.lease_acquire_allowed == false
    and $source_report.idempotency_index_write_allowed == false
    and $source_report.workflow_execution_allowed == false
    and $source_report.replay_execution_allowed == false
    and $source_report.rollback_execution_allowed == false
    and $source_report.live_execution_allowed == false
    and $lib_export_present == true
    and $reopened_sqlite_adapter_test_present == true
    and $entry_count == 9
    and $event_log_adapter_readback_count == 9
    and $sqlite_adapter_readback_count == 9
    and $event_log_record_key_count == 9
    and $sqlite_row_key_count == 9
    and $serialization_contract_count == 9
    and $transaction_boundary_count == 9
    and $sqlite_readback_validated_count == 9
    and $event_log_record_written_count == 0
    and $sqlite_row_written_count == 0
    and $adapter_persisted_count == 0
    and $adapter_mismatch_count == 0
    and ($entries | all(.wal_mode_required == true
      and .feature_gate_required == true
      and .runtime_feature_gate_enabled == false
      and .runtime_event_log_write_allowed == false
      and .runtime_sqlite_write_allowed == false
      and .runtime_store_persistence_allowed == false
      and .workflow_execution_allowed == false
      and .replay_execution_allowed == false
      and .rollback_execution_allowed == false
      and .live_execution_allowed == false))) as $readback_ready |
  {
    runtime:"hepta",
    surface:"workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback",
    status:(if $readback_ready then "ready_blocked" else "blocked" end),
    gate:"workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_gate",
    schema_version:"workflow_temporal_lite_event_log_sqlite_adapter_local_persistence_readback_v1",
    source_lease_idempotency_gate:$source_report.gate,
    source_lease_idempotency_ready:$source_report.lease_idempotency_index_local_persistence_readback_ready,
    source_anchor_pair_count:$source_report.source_anchor_pair_count,
    source_append_only_event_store_interface_ready:$source_report.source_append_only_event_store_interface_ready,
    source_lease_idempotency_derived_from_event_store_interface:$source_report.lease_idempotency_derived_from_event_store_interface,
    lib_export_present:$lib_export_present,
    reopened_sqlite_adapter_test_present:$reopened_sqlite_adapter_test_present,
    adapter_scope:"local_persistence_event_log_sqlite_adapter_readback_no_runtime_writes",
    sqlite_readback_scope:$source_report.sqlite_readback_scope,
    event_log_adapter_readback_count:$event_log_adapter_readback_count,
    sqlite_adapter_readback_count:$sqlite_adapter_readback_count,
    event_log_record_key_count:$event_log_record_key_count,
    sqlite_row_key_count:$sqlite_row_key_count,
    serialization_contract_count:$serialization_contract_count,
    transaction_boundary_count:$transaction_boundary_count,
    sqlite_readback_validated_count:$sqlite_readback_validated_count,
    event_log_record_written_count:$event_log_record_written_count,
    sqlite_row_written_count:$sqlite_row_written_count,
    adapter_persisted_count:$adapter_persisted_count,
    adapter_mismatch_count:$adapter_mismatch_count,
    wal_mode_required:true,
    local_tempdb_sqlite_read_covered_by_tests:true,
    runtime_feature_gate_enabled:false,
    adapter_contract_readback_materialized:$readback_ready,
    event_log_sqlite_adapter_derived_from_event_store_interface:($source_report.source_append_only_event_store_interface_ready == true
      and $source_report.lease_idempotency_derived_from_event_store_interface == true),
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    runtime_store_persistence_allowed:false,
    event_log_adapter_write_allowed:false,
    sqlite_adapter_write_allowed:false,
    adapter_persistence_allowed:false,
    workflow_execution_allowed:false,
    replay_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false,
    event_log_sqlite_adapter_local_persistence_readback_ready:$readback_ready,
    entries:$entries,
    blockers:[
      "runtime_feature_gate_closed",
      "runtime_event_log_write_disabled",
      "runtime_sqlite_write_disabled",
      "runtime_store_persistence_disabled",
      "event_log_adapter_write_disabled",
      "sqlite_adapter_write_disabled",
      "adapter_persistence_disabled",
      "workflow_execution_disabled",
      "replay_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "workflow_temporal_lite_work_graph_projection_local_persistence_readback"
    ],
    recommended_next_gate:"workflow_temporal_lite_work_graph_projection_local_persistence_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      runtime_filesystem_written:false,
      runtime_event_log_written:false,
      runtime_sqlite_written:false,
      runtime_store_persisted:false,
      event_log_adapter_written:false,
      sqlite_adapter_written:false,
      adapter_persisted:false,
      workflow_execution_started:false,
      replay_executed:false,
      rollback_executed:false,
      provider_invoked:false,
      model_invoked:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      channel_send_performed:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }'
