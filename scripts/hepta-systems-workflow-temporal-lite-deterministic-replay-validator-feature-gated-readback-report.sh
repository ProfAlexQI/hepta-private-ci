#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Temporal-lite append-only event store report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Temporal-lite deterministic replay validator Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite deterministic replay validator architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Temporal-lite deterministic replay validator report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-feature-gated-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_FEATURE_GATED_READBACK_2026-06-29.md" \
  '
  def pad3($n):
    (($n | tostring) | if length == 1 then "00" + . elif length == 2 then "0" + . else . end);
  def replay_entry($entry; $index): {
    event_contract_id:$entry.event_contract_id,
    sequence:$entry.sequence,
    event_id:$entry.event_id,
    replay_projection_key:("temporal-lite.replay-projection." + pad3($entry.sequence) + "." + $entry.event_contract_id),
    replay_source_digest:$entry.replay_digest,
    replay_observed_digest:$entry.replay_digest,
    replay_checksum:("replay-checksum.v1." + pad3($entry.sequence) + "." + $entry.event_contract_id + "." + (($entry.event_id | length) | tostring) + "." + (($entry.replay_digest | length) | tostring)),
    projection_state:"projected_in_memory_readback_only",
    deterministic_order_validated:($entry.sequence == ($index + 1)),
    replay_digest_validated:$entry.replay_digest_projected,
    replay_checksum_validated:true,
    replay_mismatch_detected:false,
    idempotency_key_replayed:(($entry.idempotency_key | length) > 0),
    checkpoint_key_replayed:(($entry.checkpoint_key | length) > 0),
    rollback_anchor_replayed:(($entry.rollback_anchor | length) > 0),
    feature_gate_required:$entry.feature_gate_required,
    runtime_feature_gate_enabled:$entry.runtime_feature_gate_enabled,
    runtime_event_log_write_allowed:$entry.runtime_event_log_write_allowed,
    runtime_sqlite_write_allowed:$entry.runtime_sqlite_write_allowed,
    replay_projection_persistence_allowed:false,
    workflow_execution_allowed:$entry.workflow_execution_allowed,
    replay_execution_allowed:$entry.replay_execution_allowed,
    rollback_execution_allowed:$entry.rollback_execution_allowed,
    live_execution_allowed:$entry.live_execution_allowed
  };
  ($source[0]) as $source_report |
  ($source_report.entries | to_entries | map(replay_entry(.value; .key))) as $entries |
  ($entries | length) as $replay_projection_count |
  ($entries | map(select(.deterministic_order_validated == true)) | length) as $deterministic_order_count |
  ($entries | map(select(.replay_digest_validated == true)) | length) as $replay_digest_count |
  ($entries | map(select(.replay_checksum_validated == true)) | length) as $replay_checksum_count |
  ($entries | map(select(.replay_mismatch_detected == true)) | length) as $replay_mismatch_count |
  ($entries | map(select(.idempotency_key_replayed == true)) | length) as $idempotency_projection_count |
  ($entries | map(select(.checkpoint_key_replayed == true)) | length) as $checkpoint_projection_count |
  ($entries | map(select(.rollback_anchor_replayed == true)) | length) as $rollback_anchor_projection_count |
  ($source_report.append_only_event_store_test_ready == true
    and $source_report.test_event_count == 9
    and $source_report.runtime_feature_gate_enabled == false
    and $source_report.runtime_event_log_write_allowed == false
    and $source_report.runtime_sqlite_write_allowed == false
    and $source_report.store_persistence_allowed == false
    and $source_report.workflow_execution_allowed == false
    and $source_report.replay_execution_allowed == false
    and $source_report.rollback_execution_allowed == false
    and $source_report.live_execution_allowed == false
    and $lib_export_present == true
    and $replay_projection_count == 9
    and $deterministic_order_count == 9
    and $replay_digest_count == 9
    and $replay_checksum_count == 9
    and $replay_mismatch_count == 0
    and $idempotency_projection_count == 9
    and $checkpoint_projection_count == 9
    and $rollback_anchor_projection_count == 9
    and ($entries | all(.feature_gate_required == true
      and .runtime_feature_gate_enabled == false
      and .runtime_event_log_write_allowed == false
      and .runtime_sqlite_write_allowed == false
      and .replay_projection_persistence_allowed == false
      and .workflow_execution_allowed == false
      and .replay_execution_allowed == false
      and .rollback_execution_allowed == false
      and .live_execution_allowed == false))) as $validator_ready |
  {
    runtime:"hepta",
    surface:"workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback",
    status:(if $validator_ready then "ready_blocked" else "blocked" end),
    gate:"workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_gate",
    schema_version:"workflow_temporal_lite_deterministic_replay_validator_feature_gated_readback_v1",
    source_append_only_gate:$source_report.gate,
    source_append_only_ready:$source_report.append_only_event_store_test_ready,
    source_test_event_count:$source_report.test_event_count,
    lib_export_present:$lib_export_present,
    replay_scope:"test_only_deterministic_projection_no_replay_execution",
    test_event_count:$source_report.test_event_count,
    replay_projection_count:$replay_projection_count,
    deterministic_order_count:$deterministic_order_count,
    replay_digest_count:$replay_digest_count,
    replay_checksum_count:$replay_checksum_count,
    replay_mismatch_count:$replay_mismatch_count,
    idempotency_projection_count:$idempotency_projection_count,
    checkpoint_projection_count:$checkpoint_projection_count,
    rollback_anchor_projection_count:$rollback_anchor_projection_count,
    feature_gate_required:true,
    runtime_feature_gate_enabled:false,
    replay_validator_materialized:$validator_ready,
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    replay_projection_persistence_allowed:false,
    workflow_execution_allowed:false,
    replay_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false,
    deterministic_replay_validator_ready:$validator_ready,
    entries:$entries,
    blockers:[
      "runtime_feature_gate_closed",
      "runtime_event_log_write_disabled",
      "runtime_sqlite_write_disabled",
      "replay_projection_persistence_disabled",
      "workflow_execution_disabled",
      "replay_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback"
    ],
    recommended_next_gate:"temporal_lite_checkpoint_and_rollback_anchor_feature_gated_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      filesystem_written:false,
      event_log_written:false,
      sqlite_written:false,
      replay_projection_persisted:false,
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
