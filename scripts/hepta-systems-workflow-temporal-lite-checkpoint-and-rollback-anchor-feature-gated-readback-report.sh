#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-feature-gated-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Temporal-lite deterministic replay validator report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Temporal-lite checkpoint and rollback anchor Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite checkpoint and rollback anchor architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Temporal-lite checkpoint and rollback anchor report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-feature-gated-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_CHECKPOINT_AND_ROLLBACK_ANCHOR_FEATURE_GATED_READBACK_2026-06-29.md" \
  '
  def pad3($n):
    (($n | tostring) | if length == 1 then "00" + . elif length == 2 then "0" + . else . end);
  def anchor_entry($entry): {
    event_contract_id:$entry.event_contract_id,
    sequence:$entry.sequence,
    event_id:$entry.event_id,
    replay_projection_key:$entry.replay_projection_key,
    checkpoint_anchor_key:("temporal-lite.checkpoint-anchor." + pad3($entry.sequence) + "." + $entry.event_contract_id),
    rollback_anchor_key:("temporal-lite.rollback-anchor." + pad3($entry.sequence) + "." + $entry.event_contract_id),
    checkpoint_readback_digest:("temporal-lite.checkpoint-anchor-digest.v1." + pad3($entry.sequence) + "." + $entry.event_contract_id + "." + (($entry.replay_checksum | length) | tostring)),
    rollback_readback_digest:("temporal-lite.rollback-anchor-digest.v1." + pad3($entry.sequence) + "." + $entry.event_contract_id + "." + (($entry.replay_checksum | length) | tostring)),
    anchor_pair_state:"projected_in_memory_readback_only",
    checkpoint_anchor_projected:$entry.checkpoint_key_replayed,
    rollback_anchor_projected:$entry.rollback_anchor_replayed,
    durable_anchor_pair_projected:($entry.checkpoint_key_replayed == true and $entry.rollback_anchor_replayed == true),
    checkpoint_digest_validated:$entry.replay_checksum_validated,
    rollback_digest_validated:$entry.replay_checksum_validated,
    anchor_mismatch_detected:$entry.replay_mismatch_detected,
    feature_gate_required:$entry.feature_gate_required,
    runtime_feature_gate_enabled:$entry.runtime_feature_gate_enabled,
    runtime_event_log_write_allowed:$entry.runtime_event_log_write_allowed,
    runtime_sqlite_write_allowed:$entry.runtime_sqlite_write_allowed,
    checkpoint_write_allowed:false,
    rollback_anchor_write_allowed:false,
    anchor_persistence_allowed:false,
    workflow_execution_allowed:$entry.workflow_execution_allowed,
    replay_execution_allowed:$entry.replay_execution_allowed,
    rollback_execution_allowed:$entry.rollback_execution_allowed,
    live_execution_allowed:$entry.live_execution_allowed
  };
  ($source[0]) as $source_report |
  ($source_report.entries | map(anchor_entry(.))) as $entries |
  ($entries | length) as $entry_count |
  ($entries | map(select(.checkpoint_anchor_projected == true)) | length) as $checkpoint_anchor_readback_count |
  ($entries | map(select(.rollback_anchor_projected == true)) | length) as $rollback_anchor_readback_count |
  ($entries | map(select(.durable_anchor_pair_projected == true)) | length) as $durable_anchor_pair_count |
  ($entries | map(select(.checkpoint_digest_validated == true)) | length) as $checkpoint_digest_count |
  ($entries | map(select(.rollback_digest_validated == true)) | length) as $rollback_digest_count |
  ($entries | map(select(.anchor_mismatch_detected == true)) | length) as $anchor_mismatch_count |
  ($source_report.deterministic_replay_validator_ready == true
    and $source_report.replay_projection_count == 9
    and $source_report.replay_mismatch_count == 0
    and $source_report.runtime_feature_gate_enabled == false
    and $source_report.runtime_event_log_write_allowed == false
    and $source_report.runtime_sqlite_write_allowed == false
    and $source_report.replay_projection_persistence_allowed == false
    and $source_report.workflow_execution_allowed == false
    and $source_report.replay_execution_allowed == false
    and $source_report.rollback_execution_allowed == false
    and $source_report.live_execution_allowed == false
    and $lib_export_present == true
    and $entry_count == 9
    and $checkpoint_anchor_readback_count == 9
    and $rollback_anchor_readback_count == 9
    and $durable_anchor_pair_count == 9
    and $checkpoint_digest_count == 9
    and $rollback_digest_count == 9
    and $anchor_mismatch_count == 0
    and ($entries | all(.feature_gate_required == true
      and .runtime_feature_gate_enabled == false
      and .runtime_event_log_write_allowed == false
      and .runtime_sqlite_write_allowed == false
      and .checkpoint_write_allowed == false
      and .rollback_anchor_write_allowed == false
      and .anchor_persistence_allowed == false
      and .workflow_execution_allowed == false
      and .replay_execution_allowed == false
      and .rollback_execution_allowed == false
      and .live_execution_allowed == false))) as $anchor_ready |
  {
    runtime:"hepta",
    surface:"workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback",
    status:(if $anchor_ready then "ready_blocked" else "blocked" end),
    gate:"workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_gate",
    schema_version:"workflow_temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback_v1",
    source_replay_validator_gate:$source_report.gate,
    source_replay_validator_ready:$source_report.deterministic_replay_validator_ready,
    source_replay_projection_count:$source_report.replay_projection_count,
    lib_export_present:$lib_export_present,
    anchor_scope:"test_only_checkpoint_and_rollback_anchor_readback_no_writes",
    replay_projection_count:$source_report.replay_projection_count,
    checkpoint_anchor_readback_count:$checkpoint_anchor_readback_count,
    rollback_anchor_readback_count:$rollback_anchor_readback_count,
    durable_anchor_pair_count:$durable_anchor_pair_count,
    checkpoint_digest_count:$checkpoint_digest_count,
    rollback_digest_count:$rollback_digest_count,
    anchor_mismatch_count:$anchor_mismatch_count,
    feature_gate_required:true,
    runtime_feature_gate_enabled:false,
    anchor_readback_materialized:$anchor_ready,
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    checkpoint_write_allowed:false,
    rollback_anchor_write_allowed:false,
    anchor_persistence_allowed:false,
    workflow_execution_allowed:false,
    replay_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false,
    checkpoint_and_rollback_anchor_readback_ready:$anchor_ready,
    entries:$entries,
    blockers:[
      "runtime_feature_gate_closed",
      "runtime_event_log_write_disabled",
      "runtime_sqlite_write_disabled",
      "checkpoint_write_disabled",
      "rollback_anchor_write_disabled",
      "anchor_persistence_disabled",
      "workflow_execution_disabled",
      "replay_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "current_reality_matrix_compact_cache_boundary_readback"
    ],
    recommended_next_gate:"current_reality_matrix_compact_cache_boundary_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      filesystem_written:false,
      event_log_written:false,
      sqlite_written:false,
      checkpoint_written:false,
      rollback_anchor_written:false,
      anchor_persisted:false,
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
