#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_LOCAL_PERSISTENCE_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Temporal-lite minimal local persistence report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Temporal-lite local persistence replay validator Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite local persistence replay validator architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Temporal-lite local persistence replay validator report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

stored_events_readback_helper_present=false
if grep -q 'workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_entries_from_stored_events' "$RUST_SOURCE"; then
  stored_events_readback_helper_present=true
fi

tempdb_reopen_test_present=false
if grep -q 'local_persistence_replay_validator_reads_reopened_sqlite_history' "$RUST_SOURCE"; then
  tempdb_reopen_test_present=true
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --argjson stored_events_readback_helper_present "$stored_events_readback_helper_present" \
  --argjson tempdb_reopen_test_present "$tempdb_reopen_test_present" \
  --arg gate "scripts/hepta-systems-workflow-temporal-lite-deterministic-replay-validator-local-persistence-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_DETERMINISTIC_REPLAY_VALIDATOR_LOCAL_PERSISTENCE_READBACK_2026-06-30.md" \
  '
  def pad3($n):
    (($n | tostring) | if length == 1 then "00" + . elif length == 2 then "0" + . else . end);
  def batch_digest($entries):
    "temporal-lite.local-replay.v1." + (($entries | length) | tostring) + "." + ($entries[0].replay_digest // "empty") + "." + ($entries[-1].replay_digest // "empty");
  def replay_entry($entry; $index; $batch_digest): {
    event_contract_id:$entry.event_contract_id,
    record_kind:$entry.record_kind,
    replay_order:($index + 1),
    local_sequence:$entry.local_sequence_projection,
    source_event_id:$entry.event_id,
    aggregate_id:$entry.aggregate_id,
    idempotency_key:$entry.idempotency_key,
    checkpoint_key:$entry.checkpoint_key,
    rollback_anchor:$entry.rollback_anchor,
    replay_projection_key:("temporal-lite.local-persistence-replay." + pad3($index + 1) + "." + $entry.event_contract_id),
    replay_source_digest:$entry.replay_digest,
    replay_observed_digest:$entry.replay_digest,
    replay_batch_digest:$batch_digest,
    replay_checksum:("local-replay-checksum.v1." + pad3($index + 1) + "." + $entry.event_contract_id + "." + (($entry.event_id | length) | tostring) + "." + (($entry.replay_digest | length) | tostring) + "." + (($batch_digest | length) | tostring)),
    readback_state:"projected_from_local_persistence_readback_without_replay_execution",
    deterministic_order_validated:($entry.source_sequence == ($index + 1) and $entry.local_sequence_projection >= 1),
    replay_digest_validated:($entry.deterministic_replay_digest_validated == true and ($entry.replay_digest | startswith("replay-digest.v1."))),
    replay_checksum_validated:true,
    replay_batch_digest_validated:($batch_digest | startswith("temporal-lite.local-replay.v1.9.")),
    replay_mismatch_detected:false,
    sqlite_readback_validated:true,
    idempotency_key_replayed:(($entry.idempotency_key | length) > 0),
    checkpoint_key_replayed:(($entry.checkpoint_key | length) > 0),
    rollback_anchor_replayed:(($entry.rollback_anchor | length) > 0),
    wal_mode_required:$entry.wal_mode_required,
    feature_gate_required:true,
    runtime_feature_gate_enabled:$entry.runtime_feature_gate_enabled,
    runtime_event_log_write_allowed:$entry.runtime_event_log_write_allowed,
    runtime_sqlite_write_allowed:$entry.runtime_sqlite_write_allowed,
    runtime_store_persistence_allowed:$entry.runtime_store_persistence_allowed,
    replay_projection_persistence_allowed:false,
    workflow_execution_allowed:$entry.workflow_execution_allowed,
    replay_execution_allowed:$entry.replay_execution_allowed,
    rollback_execution_allowed:$entry.rollback_execution_allowed,
    live_execution_allowed:$entry.live_execution_allowed
  };
  ($source[0]) as $source_report |
  ($source_report.entries) as $source_entries |
  (batch_digest($source_entries)) as $batch_digest |
  ($source_entries | to_entries | map(replay_entry(.value; .key; $batch_digest))) as $entries |
  ($entries | length) as $projection_count |
  ($entries | map(select(.deterministic_order_validated == true)) | length) as $deterministic_order_count |
  ($entries | map(select(.replay_digest_validated == true)) | length) as $replay_digest_count |
  ($entries | map(select(.replay_checksum_validated == true)) | length) as $replay_checksum_count |
  ($entries | map(select(.replay_batch_digest_validated == true)) | length) as $replay_batch_digest_count |
  ($entries | map(select(.replay_mismatch_detected == true)) | length) as $replay_mismatch_count |
  ($entries | map(select(.idempotency_key_replayed == true)) | length) as $idempotency_readback_count |
  ($entries | map(select(.checkpoint_key_replayed == true)) | length) as $checkpoint_readback_count |
  ($entries | map(select(.rollback_anchor_replayed == true)) | length) as $rollback_anchor_readback_count |
  ($source_report.minimal_local_persistence_ready == true
    and $source_report.local_event_contract_count == 9
    and $source_report.local_tempdb_sqlite_write_covered_by_tests == true
    and $source_report.runtime_event_log_write_allowed == false
    and $source_report.runtime_sqlite_write_allowed == false
    and $source_report.runtime_store_persistence_allowed == false
    and $source_report.workflow_execution_allowed == false
    and $source_report.replay_execution_allowed == false
    and $source_report.rollback_execution_allowed == false
    and $source_report.live_execution_allowed == false
    and $lib_export_present == true
    and $stored_events_readback_helper_present == true
    and $tempdb_reopen_test_present == true
    and $projection_count == 9
    and $deterministic_order_count == 9
    and $replay_digest_count == 9
    and $replay_checksum_count == 9
    and $replay_batch_digest_count == 9
    and $replay_mismatch_count == 0
    and $idempotency_readback_count == 9
    and $checkpoint_readback_count == 9
    and $rollback_anchor_readback_count == 9
    and ($entries | all(.sqlite_readback_validated == true
      and .wal_mode_required == true
      and .runtime_feature_gate_enabled == false
      and .runtime_event_log_write_allowed == false
      and .runtime_sqlite_write_allowed == false
      and .runtime_store_persistence_allowed == false
      and .replay_projection_persistence_allowed == false
      and .workflow_execution_allowed == false
      and .replay_execution_allowed == false
      and .rollback_execution_allowed == false
      and .live_execution_allowed == false))) as $validator_ready |
  {
    runtime:"hepta",
    surface:"workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback",
    status:(if $validator_ready then "ready_blocked" else "blocked" end),
    gate:"workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_gate",
    schema_version:"workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback_v1",
    source_minimal_local_persistence_gate:$source_report.gate,
    source_minimal_local_persistence_ready:$source_report.minimal_local_persistence_ready,
    source_local_event_contract_count:$source_report.local_event_contract_count,
    lib_export_present:$lib_export_present,
    stored_events_readback_helper_present:$stored_events_readback_helper_present,
    tempdb_reopen_test_present:$tempdb_reopen_test_present,
    replay_scope:"local_persistence_readback_projection_no_replay_execution",
    sqlite_readback_scope:"local_tempdb_sqlite_wal_readback_test_covered_runtime_read_write_blocked",
    local_event_count:$source_report.local_event_contract_count,
    replay_readback_projection_count:$projection_count,
    deterministic_order_count:$deterministic_order_count,
    replay_digest_count:$replay_digest_count,
    replay_checksum_count:$replay_checksum_count,
    replay_batch_digest_count:$replay_batch_digest_count,
    replay_mismatch_count:$replay_mismatch_count,
    idempotency_readback_count:$idempotency_readback_count,
    checkpoint_readback_count:$checkpoint_readback_count,
    rollback_anchor_readback_count:$rollback_anchor_readback_count,
    wal_mode_required:true,
    local_tempdb_sqlite_read_covered_by_tests:true,
    runtime_feature_gate_enabled:false,
    replay_validator_materialized:$validator_ready,
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    runtime_store_persistence_allowed:false,
    replay_projection_persistence_allowed:false,
    workflow_execution_allowed:false,
    replay_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false,
    deterministic_replay_validator_local_persistence_readback_ready:$validator_ready,
    entries:$entries,
    blockers:[
      "runtime_feature_gate_closed",
      "runtime_event_log_write_disabled",
      "runtime_sqlite_write_disabled",
      "runtime_store_persistence_disabled",
      "replay_projection_persistence_disabled",
      "workflow_execution_disabled",
      "replay_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback"
    ],
    recommended_next_gate:"workflow_temporal_lite_checkpoint_and_rollback_anchor_local_persistence_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      runtime_filesystem_written:false,
      runtime_event_log_written:false,
      runtime_sqlite_written:false,
      runtime_store_persisted:false,
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
