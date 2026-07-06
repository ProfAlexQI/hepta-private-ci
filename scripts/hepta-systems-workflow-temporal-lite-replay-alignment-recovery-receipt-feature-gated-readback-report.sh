#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-replay-alignment-recovery-window-feature-gated-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_RECOVERY_RECEIPT_FEATURE_GATED_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-feature-gated-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Temporal-lite replay-alignment recovery window report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Temporal-lite replay-alignment recovery receipt Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite replay-alignment recovery receipt architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Temporal-lite replay-alignment recovery receipt report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-workflow-temporal-lite-replay-alignment-recovery-receipt-feature-gated-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_REPLAY_ALIGNMENT_RECOVERY_RECEIPT_FEATURE_GATED_READBACK_2026-06-29.md" \
  '
  def pad3($n):
    (($n | tostring) | if length == 1 then "00" + . elif length == 2 then "0" + . else . end);
  def keyed($prefix; $entry):
    $prefix + "." + pad3($entry.sequence) + "." + $entry.event_contract_id;
  def recovery_receipt_entry($entry):
    ($entry.recovery_window_projected == true
      and $entry.replay_alignment_recovery_matches == true
      and $entry.recovery_window_mismatch_detected == false) as $matches |
    keyed("temporal-lite.recovery-receipt.ack.readback"; $entry) as $recovery_receipt_ack_key |
    {
      event_contract_id:$entry.event_contract_id,
      sequence:$entry.sequence,
      event_id:$entry.event_id,
      replay_alignment_key:$entry.replay_alignment_key,
      projection_replay_key:$entry.projection_replay_key,
      recovery_window_key:$entry.recovery_window_key,
      recovery_window_start_key:$entry.recovery_window_start_key,
      recovery_window_end_key:$entry.recovery_window_end_key,
      recovery_window_digest:$entry.recovery_window_digest,
      recovery_receipt_key:keyed("temporal-lite.replay-alignment.recovery-receipt.readback"; $entry),
      recovery_receipt_ack_key:$recovery_receipt_ack_key,
      recovery_receipt_digest:("replay-alignment-recovery-receipt-digest.v1." + pad3($entry.sequence) + "." + $entry.event_contract_id + "." + (($entry.recovery_window_digest | length) | tostring) + "." + (($entry.recovery_window_end_key | length) | tostring)),
      expected_recovery_receipt_key:$recovery_receipt_ack_key,
      receipt_state:"recovery_receipt_projected_not_written",
      readback_state:"replay_alignment_recovery_receipt_contract_projected_in_memory_only",
      recovery_window_projected:$entry.recovery_window_projected,
      recovery_receipt_projected:$matches,
      recovery_receipt_key_projected:true,
      recovery_receipt_digest_projected:(($entry.recovery_window_digest | length) > 0),
      recovery_receipt_ack_projected:true,
      replay_alignment_receipt_matches:$matches,
      recovery_receipt_mismatch_detected:($matches | not),
      replay_executed:false,
      checkpoint_written:false,
      rollback_anchor_written:false,
      recovery_window_persisted:false,
      recovery_receipt_persisted:false,
      work_graph_store_written:false,
      event_log_record_written:false,
      sqlite_row_written:false,
      feature_gate_required:$entry.feature_gate_required,
      runtime_feature_gate_enabled:$entry.runtime_feature_gate_enabled,
      runtime_event_log_write_allowed:$entry.runtime_event_log_write_allowed,
      runtime_sqlite_write_allowed:$entry.runtime_sqlite_write_allowed,
      workflow_execution_allowed:$entry.workflow_execution_allowed,
      replay_execution_allowed:$entry.replay_execution_allowed,
      rollback_execution_allowed:$entry.rollback_execution_allowed,
      live_execution_allowed:$entry.live_execution_allowed
    };
  ($source[0]) as $source_report |
  ($source_report.entries | map(recovery_receipt_entry(.))) as $entries |
  ($entries | length) as $entry_count |
  ($entries | map(select(.recovery_receipt_projected == true)) | length) as $recovery_receipt_projection_count |
  ($entries | map(select(.recovery_receipt_key_projected == true)) | length) as $recovery_receipt_key_count |
  ($entries | map(select(.recovery_receipt_digest_projected == true)) | length) as $recovery_receipt_digest_count |
  ($entries | map(select(.recovery_receipt_ack_projected == true)) | length) as $recovery_receipt_ack_count |
  ($entries | map(select(.replay_alignment_receipt_matches == true)) | length) as $replay_alignment_receipt_match_count |
  ($entries | map(select(.recovery_receipt_mismatch_detected == true)) | length) as $recovery_receipt_mismatch_count |
  ($entries | map(select(.replay_executed == true)) | length) as $replay_executed_count |
  ($entries | map(select(.checkpoint_written == true)) | length) as $checkpoint_written_count |
  ($entries | map(select(.rollback_anchor_written == true)) | length) as $rollback_anchor_written_count |
  ($entries | map(select(.recovery_window_persisted == true)) | length) as $recovery_window_persisted_count |
  ($entries | map(select(.recovery_receipt_persisted == true)) | length) as $recovery_receipt_persisted_count |
  ($entries | map(select(.work_graph_store_written == true)) | length) as $work_graph_store_write_count |
  ($entries | map(select(.event_log_record_written == true)) | length) as $event_log_write_count |
  ($entries | map(select(.sqlite_row_written == true)) | length) as $sqlite_write_count |
  ($source_report.recovery_window_readback_ready == true
    and $source_report.source_rollback_consistency_entry_count == 9
    and $source_report.recovery_window_projection_count == 9
    and $source_report.recovery_window_key_count == 9
    and $source_report.recovery_window_digest_count == 9
    and $source_report.replay_alignment_recovery_match_count == 9
    and $source_report.recovery_window_mismatch_count == 0
    and $source_report.replay_executed_count == 0
    and $source_report.checkpoint_written_count == 0
    and $source_report.rollback_anchor_written_count == 0
    and $source_report.recovery_window_persisted_count == 0
    and $source_report.work_graph_store_write_count == 0
    and $source_report.event_log_write_count == 0
    and $source_report.sqlite_write_count == 0
    and $source_report.runtime_feature_gate_enabled == false
    and $source_report.replay_execution_allowed == false
    and $source_report.checkpoint_write_allowed == false
    and $source_report.rollback_anchor_write_allowed == false
    and $source_report.recovery_window_persistence_allowed == false
    and $source_report.work_graph_projection_write_allowed == false
    and $source_report.runtime_event_log_write_allowed == false
    and $source_report.runtime_sqlite_write_allowed == false
    and $source_report.workflow_execution_allowed == false
    and $source_report.rollback_execution_allowed == false
    and $source_report.live_execution_allowed == false
    and $lib_export_present == true
    and $entry_count == 9
    and $recovery_receipt_projection_count == 9
    and $recovery_receipt_key_count == 9
    and $recovery_receipt_digest_count == 9
    and $recovery_receipt_ack_count == 9
    and $replay_alignment_receipt_match_count == 9
    and $recovery_receipt_mismatch_count == 0
    and $replay_executed_count == 0
    and $checkpoint_written_count == 0
    and $rollback_anchor_written_count == 0
    and $recovery_window_persisted_count == 0
    and $recovery_receipt_persisted_count == 0
    and $work_graph_store_write_count == 0
    and $event_log_write_count == 0
    and $sqlite_write_count == 0
    and ($entries | all(.feature_gate_required == true
      and .runtime_feature_gate_enabled == false
      and .runtime_event_log_write_allowed == false
      and .runtime_sqlite_write_allowed == false
      and .workflow_execution_allowed == false
      and .replay_execution_allowed == false
      and .rollback_execution_allowed == false
      and .live_execution_allowed == false))) as $readback_ready |
  {
    runtime:"hepta",
    surface:"workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback",
    status:(if $readback_ready then "ready_blocked" else "blocked" end),
    gate:"workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback_gate",
    schema_version:"workflow_temporal_lite_replay_alignment_recovery_receipt_feature_gated_readback_v1",
    source_recovery_window_gate:$source_report.gate,
    source_recovery_window_ready:$source_report.recovery_window_readback_ready,
    source_recovery_window_entry_count:($source_report.entries | length),
    lib_export_present:$lib_export_present,
    receipt_scope:"test_only_replay_alignment_recovery_receipt_readback_no_execution",
    recovery_receipt_projection_count:$recovery_receipt_projection_count,
    recovery_receipt_key_count:$recovery_receipt_key_count,
    recovery_receipt_digest_count:$recovery_receipt_digest_count,
    recovery_receipt_ack_count:$recovery_receipt_ack_count,
    replay_alignment_receipt_match_count:$replay_alignment_receipt_match_count,
    recovery_receipt_mismatch_count:$recovery_receipt_mismatch_count,
    replay_executed_count:$replay_executed_count,
    checkpoint_written_count:$checkpoint_written_count,
    rollback_anchor_written_count:$rollback_anchor_written_count,
    recovery_window_persisted_count:$recovery_window_persisted_count,
    recovery_receipt_persisted_count:$recovery_receipt_persisted_count,
    work_graph_store_write_count:$work_graph_store_write_count,
    event_log_write_count:$event_log_write_count,
    sqlite_write_count:$sqlite_write_count,
    feature_gate_required:true,
    runtime_feature_gate_enabled:false,
    recovery_receipt_contract_readback_materialized:$readback_ready,
    replay_execution_allowed:false,
    checkpoint_write_allowed:false,
    rollback_anchor_write_allowed:false,
    recovery_window_persistence_allowed:false,
    recovery_receipt_persistence_allowed:false,
    work_graph_projection_write_allowed:false,
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    workflow_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false,
    source_gate_recursion_bounded:true,
    recovery_receipt_readback_ready:$readback_ready,
    entries:$entries,
    blockers:[
      "runtime_feature_gate_closed",
      "source_gate_recursion_bounded_to_report_invariants",
      "replay_execution_disabled",
      "checkpoint_write_disabled",
      "rollback_anchor_write_disabled",
      "recovery_window_persistence_disabled",
      "recovery_receipt_persistence_disabled",
      "work_graph_projection_write_disabled",
      "runtime_event_log_write_disabled",
      "runtime_sqlite_write_disabled",
      "workflow_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "hepta_systems_gate_recursion_cost_boundary_readback"
    ],
    recommended_next_gate:"hepta_systems_gate_recursion_cost_boundary_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      filesystem_written:false,
      replay_executed:false,
      checkpoint_written:false,
      rollback_anchor_written:false,
      recovery_window_written:false,
      recovery_window_persisted:false,
      recovery_receipt_written:false,
      recovery_receipt_persisted:false,
      work_graph_projection_written:false,
      event_log_written:false,
      sqlite_written:false,
      workflow_execution_started:false,
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
