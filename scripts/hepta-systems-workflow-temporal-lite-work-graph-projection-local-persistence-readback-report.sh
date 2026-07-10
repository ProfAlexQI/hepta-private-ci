#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-event-log-sqlite-adapter-local-persistence-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/workflow_temporal_lite_work_graph_projection_local_persistence_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_LOCAL_PERSISTENCE_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-work-graph-projection-local-persistence-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Temporal-lite local event-log/SQLite adapter report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Temporal-lite local WorkGraph projection Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite local WorkGraph projection architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Temporal-lite local WorkGraph projection report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_temporal_lite_work_graph_projection_local_persistence_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

reopened_sqlite_projection_test_present=false
if grep -q 'local_work_graph_projection_uses_reopened_sqlite_event_history' "$RUST_SOURCE"; then
  reopened_sqlite_projection_test_present=true
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --argjson reopened_sqlite_projection_test_present "$reopened_sqlite_projection_test_present" \
  --arg gate "scripts/hepta-systems-workflow-temporal-lite-work-graph-projection-local-persistence-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_WORK_GRAPH_PROJECTION_LOCAL_PERSISTENCE_READBACK_2026-06-30.md" \
  '
  def pad3($n):
    (($n | tostring) | if length == 1 then "00" + . elif length == 2 then "0" + . else . end);
  def keyed($prefix; $entry):
    $prefix + "." + pad3($entry.replay_order) + "." + $entry.event_contract_id;
  def node_kind($id):
    if ($id | contains("approval")) then "approval_event"
    elif ($id | contains("task_result")) then "task_result_event"
    elif ($id | contains("checkpoint")) then "checkpoint_event"
    else "workflow_event"
    end;
  def projection_entry($entry): {
    event_contract_id:$entry.event_contract_id,
    replay_order:$entry.replay_order,
    local_sequence:$entry.local_sequence,
    source_event_id:$entry.source_event_id,
    event_log_record_key:$entry.event_log_record_key,
    sqlite_row_key:$entry.sqlite_row_key,
    work_graph_node_key:keyed("temporal-lite.local-work-graph.node-readback"; $entry),
    work_graph_node_kind:node_kind($entry.event_contract_id),
    work_graph_event_edge_key:keyed("temporal-lite.local-work-graph.event-edge-readback"; $entry),
    work_graph_state_edge_key:keyed("temporal-lite.local-work-graph.state-edge-readback"; $entry),
    projection_key:keyed("temporal-lite.local-work-graph.projection-readback"; $entry),
    projection_checksum:("temporal-lite.local-work-graph-projection-checksum.v1." + pad3($entry.replay_order) + "." + $entry.event_contract_id + "." + (($entry.event_log_record_key | length) | tostring) + "." + (($entry.sqlite_row_key | length) | tostring) + "." + (($entry.local_sequence | tostring))),
    projection_state:"projected_from_local_persistence_not_persisted",
    readback_state:"projected_from_sqlite_wal_local_persistence_readback_without_projection_writes",
    work_graph_node_projected:($entry.event_log_adapter_projected == true and $entry.sqlite_adapter_projected == true and $entry.serialization_contract_projected == true and $entry.sqlite_readback_validated == true and $entry.adapter_mismatch_detected == false),
    work_graph_event_edge_projected:($entry.event_log_adapter_projected == true and $entry.sqlite_adapter_projected == true and $entry.serialization_contract_projected == true and $entry.sqlite_readback_validated == true and $entry.adapter_mismatch_detected == false),
    work_graph_state_edge_projected:($entry.event_log_adapter_projected == true and $entry.sqlite_adapter_projected == true and $entry.serialization_contract_projected == true and $entry.sqlite_readback_validated == true and $entry.adapter_mismatch_detected == false),
    projection_checksum_projected:($entry.event_log_adapter_projected == true and $entry.sqlite_adapter_projected == true and $entry.serialization_contract_projected == true and $entry.sqlite_readback_validated == true and $entry.adapter_mismatch_detected == false),
    sqlite_readback_validated:$entry.sqlite_readback_validated,
    projection_mismatch_detected:$entry.adapter_mismatch_detected,
    projection_persisted:false,
    work_graph_store_written:false,
    event_log_record_written:false,
    sqlite_row_written:false,
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
  ($source_report.entries | map(projection_entry(.))) as $entries |
  ($entries | length) as $entry_count |
  ($entries | map(select(.work_graph_node_projected == true)) | length) as $work_graph_node_projection_count |
  ($entries | map(select(.work_graph_event_edge_projected == true)) | length) as $work_graph_event_edge_projection_count |
  ($entries | map(select(.work_graph_state_edge_projected == true)) | length) as $work_graph_state_edge_projection_count |
  ($entries | map(select((.projection_key | length) > 0)) | length) as $projection_key_count |
  ($entries | map(select(.projection_checksum_projected == true)) | length) as $projection_checksum_count |
  ($entries | map(select(.sqlite_readback_validated == true)) | length) as $sqlite_readback_validated_count |
  ($entries | map(select(.projection_persisted == true)) | length) as $projection_persisted_count |
  ($entries | map(select(.work_graph_store_written == true)) | length) as $work_graph_store_write_count |
  ($entries | map(select(.event_log_record_written == true)) | length) as $event_log_write_count |
  ($entries | map(select(.sqlite_row_written == true)) | length) as $sqlite_write_count |
  ($entries | map(select(.projection_mismatch_detected == true)) | length) as $projection_mismatch_count |
  ($source_report.event_log_sqlite_adapter_local_persistence_readback_ready == true
    and $source_report.source_append_only_event_store_interface_ready == true
    and $source_report.event_log_sqlite_adapter_derived_from_event_store_interface == true
    and $source_report.source_anchor_pair_count == 9
    and $source_report.event_log_adapter_readback_count == 9
    and $source_report.sqlite_adapter_readback_count == 9
    and $source_report.event_log_record_written_count == 0
    and $source_report.sqlite_row_written_count == 0
    and $source_report.adapter_persisted_count == 0
    and $source_report.adapter_mismatch_count == 0
    and $source_report.local_tempdb_sqlite_read_covered_by_tests == true
    and $source_report.runtime_feature_gate_enabled == false
    and $source_report.runtime_event_log_write_allowed == false
    and $source_report.runtime_sqlite_write_allowed == false
    and $source_report.runtime_store_persistence_allowed == false
    and $source_report.workflow_execution_allowed == false
    and $source_report.replay_execution_allowed == false
    and $source_report.rollback_execution_allowed == false
    and $source_report.live_execution_allowed == false
    and $lib_export_present == true
    and $reopened_sqlite_projection_test_present == true
    and $entry_count == 9
    and $work_graph_node_projection_count == 9
    and $work_graph_event_edge_projection_count == 9
    and $work_graph_state_edge_projection_count == 9
    and $projection_key_count == 9
    and $projection_checksum_count == 9
    and $sqlite_readback_validated_count == 9
    and $projection_persisted_count == 0
    and $work_graph_store_write_count == 0
    and $event_log_write_count == 0
    and $sqlite_write_count == 0
    and $projection_mismatch_count == 0
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
    surface:"workflow_temporal_lite_work_graph_projection_local_persistence_readback",
    status:(if $readback_ready then "ready_blocked" else "blocked" end),
    gate:"workflow_temporal_lite_work_graph_projection_local_persistence_readback_gate",
    schema_version:"workflow_temporal_lite_work_graph_projection_local_persistence_readback_v1",
    source_adapter_gate:$source_report.gate,
    source_adapter_ready:$source_report.event_log_sqlite_adapter_local_persistence_readback_ready,
    source_adapter_entry_count:$source_report.source_anchor_pair_count,
    source_append_only_event_store_interface_ready:$source_report.source_append_only_event_store_interface_ready,
    source_event_log_sqlite_adapter_derived_from_event_store_interface:$source_report.event_log_sqlite_adapter_derived_from_event_store_interface,
    lib_export_present:$lib_export_present,
    reopened_sqlite_projection_test_present:$reopened_sqlite_projection_test_present,
    projection_scope:"local_persistence_work_graph_projection_readback_no_persistence",
    sqlite_readback_scope:$source_report.sqlite_readback_scope,
    work_graph_node_projection_count:$work_graph_node_projection_count,
    work_graph_event_edge_projection_count:$work_graph_event_edge_projection_count,
    work_graph_state_edge_projection_count:$work_graph_state_edge_projection_count,
    projection_key_count:$projection_key_count,
    projection_checksum_count:$projection_checksum_count,
    sqlite_readback_validated_count:$sqlite_readback_validated_count,
    projection_persisted_count:$projection_persisted_count,
    work_graph_store_write_count:$work_graph_store_write_count,
    event_log_write_count:$event_log_write_count,
    sqlite_write_count:$sqlite_write_count,
    projection_mismatch_count:$projection_mismatch_count,
    wal_mode_required:true,
    local_tempdb_sqlite_read_covered_by_tests:true,
    runtime_feature_gate_enabled:false,
    projection_contract_readback_materialized:$readback_ready,
    work_graph_projection_derived_from_event_store_interface:($source_report.source_append_only_event_store_interface_ready == true
      and $source_report.event_log_sqlite_adapter_derived_from_event_store_interface == true),
    work_graph_projection_write_allowed:false,
    work_graph_projection_persistence_allowed:false,
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    runtime_store_persistence_allowed:false,
    workflow_execution_allowed:false,
    replay_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false,
    work_graph_projection_local_persistence_readback_ready:$readback_ready,
    entries:$entries,
    blockers:[
      "runtime_feature_gate_closed",
      "work_graph_projection_write_disabled",
      "work_graph_projection_persistence_disabled",
      "runtime_event_log_write_disabled",
      "runtime_sqlite_write_disabled",
      "runtime_store_persistence_disabled",
      "workflow_execution_disabled",
      "replay_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback"
    ],
    recommended_next_gate:"workflow_temporal_lite_work_graph_projection_replay_alignment_local_persistence_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      runtime_filesystem_written:false,
      work_graph_projection_written:false,
      work_graph_projection_persisted:false,
      runtime_event_log_written:false,
      runtime_sqlite_written:false,
      runtime_store_persisted:false,
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
