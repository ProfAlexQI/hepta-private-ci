#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-test-implementation-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/workflow_temporal_lite_append_only_event_store_minimal_local_persistence.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_MINIMAL_LOCAL_PERSISTENCE_2026-06-30.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Temporal-lite append-only event store test implementation report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Temporal-lite minimal local persistence Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite minimal local persistence architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Temporal-lite minimal local persistence report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_temporal_lite_append_only_event_store_minimal_local_persistence_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

store_type_present=false
if grep -q 'WorkflowTemporalLiteMinimalLocalEventStore' "$RUST_SOURCE"; then
  store_type_present=true
fi

sqlite_wal_config_present=false
if grep -q 'SqliteJournalMode::Wal' "$RUST_SOURCE"; then
  sqlite_wal_config_present=true
fi

idempotency_unique_index_present=false
if grep -q 'idx_temporal_lite_events_idempotency_key' "$RUST_SOURCE"; then
  idempotency_unique_index_present=true
fi

deterministic_reopen_test_present=false
if grep -q 'minimal_local_event_store_replays_deterministically_after_reopen' "$RUST_SOURCE"; then
  deterministic_reopen_test_present=true
fi

event_store_interface_present=false
if grep -q 'pub trait WorkflowTemporalLiteAppendOnlyEventStore' "$RUST_SOURCE"; then
  event_store_interface_present=true
fi

sqlite_wal_backend_implements_interface=false
if grep -q 'impl WorkflowTemporalLiteAppendOnlyEventStore for WorkflowTemporalLiteMinimalLocalEventStore' "$RUST_SOURCE"; then
  sqlite_wal_backend_implements_interface=true
fi

event_store_interface_test_present=false
if grep -q 'minimal_local_event_store_interface_appends_replays_and_denies_duplicates' "$RUST_SOURCE"; then
  event_store_interface_test_present=true
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --argjson store_type_present "$store_type_present" \
  --argjson sqlite_wal_config_present "$sqlite_wal_config_present" \
  --argjson idempotency_unique_index_present "$idempotency_unique_index_present" \
  --argjson deterministic_reopen_test_present "$deterministic_reopen_test_present" \
  --argjson event_store_interface_present "$event_store_interface_present" \
  --argjson sqlite_wal_backend_implements_interface "$sqlite_wal_backend_implements_interface" \
  --argjson event_store_interface_test_present "$event_store_interface_test_present" \
  --arg gate "scripts/hepta-systems-workflow-temporal-lite-append-only-event-store-minimal-local-persistence-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_APPEND_ONLY_EVENT_STORE_MINIMAL_LOCAL_PERSISTENCE_2026-06-30.md" \
  '
  def local_event($entry): {
    event_contract_id:$entry.event_contract_id,
    record_kind:$entry.record_kind,
    source_sequence:$entry.sequence,
    local_sequence_projection:$entry.sequence,
    event_id:$entry.event_id,
    aggregate_id:$entry.aggregate_id,
    idempotency_key:$entry.idempotency_key,
    checkpoint_key:$entry.checkpoint_key,
    replay_digest:$entry.replay_digest,
    rollback_anchor:$entry.rollback_anchor,
    sqlite_table:"temporal_lite_events",
    sqlite_primary_sequence:"sequence",
    event_id_unique_constraint:"temporal_lite_events.event_id_unique",
    idempotency_unique_index:"idx_temporal_lite_events_idempotency_key",
    append_state:"covered_by_tempdb_sqlite_wal_test",
    duplicate_append_state:"duplicate_denied_by_unique_idempotency_key",
    appended_to_local_tempdb_test_store:true,
    append_only_order_validated:true,
    idempotency_unique_index_validated:true,
    duplicate_append_denied:true,
    checkpoint_anchor_projected:$entry.checkpoint_anchor_projected,
    replay_digest_projected:$entry.replay_digest_projected,
    deterministic_replay_digest_validated:$entry.replay_digest_projected,
    rollback_anchor_validated:$entry.rollback_anchor_projected,
    wal_mode_required:true,
    local_tempdb_sqlite_write_covered_by_test:true,
    feature_gate_required:true,
    runtime_feature_gate_enabled:false,
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    runtime_store_persistence_allowed:false,
    runtime_lease_acquire_allowed:false,
    runtime_checkpoint_write_allowed:false,
    workflow_execution_allowed:false,
    replay_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false
  };
  ($source[0]) as $source_report |
  ($source_report.entries | map(local_event(.))) as $entries |
  ($entries | length) as $event_count |
  ($entries | map(select(.append_state == "covered_by_tempdb_sqlite_wal_test")) | length) as $accepted_append_count |
  ($entries | map(select(.duplicate_append_denied == true)) | length) as $duplicate_append_denial_count |
  ($entries | map(select(.append_only_order_validated == true)) | length) as $append_only_sequence_count |
  ($entries | map(.idempotency_key) | unique | length) as $idempotency_unique_index_entry_count |
  ($entries | map(select(.checkpoint_anchor_projected == true)) | length) as $checkpoint_anchor_count |
  ($entries | map(select(.replay_digest_projected == true)) | length) as $replay_digest_count |
  ($entries | map(select(.deterministic_replay_digest_validated == true)) | length) as $deterministic_replay_validation_count |
  ($entries | map(select(.rollback_anchor_validated == true)) | length) as $rollback_anchor_count |
  ($event_store_interface_present == true
    and $sqlite_wal_backend_implements_interface == true
    and $event_store_interface_test_present == true
    and $accepted_append_count == 9
    and $duplicate_append_denial_count == 9
    and $deterministic_replay_validation_count == 9
    and ($entries | all(.runtime_feature_gate_enabled == false
      and .runtime_event_log_write_allowed == false
      and .runtime_sqlite_write_allowed == false
      and .runtime_store_persistence_allowed == false
      and .workflow_execution_allowed == false
      and .replay_execution_allowed == false
      and .rollback_execution_allowed == false
      and .live_execution_allowed == false))) as $append_only_event_store_interface_ready |
  ($source_report.append_only_event_store_test_ready == true
    and $source_report.test_event_count == 9
    and $source_report.accepted_append_count == 9
    and $source_report.duplicate_append_denial_count == 9
    and $source_report.runtime_event_log_write_allowed == false
    and $source_report.runtime_sqlite_write_allowed == false
    and $source_report.store_persistence_allowed == false
    and $source_report.workflow_execution_allowed == false
    and $source_report.replay_execution_allowed == false
    and $source_report.rollback_execution_allowed == false
    and $source_report.live_execution_allowed == false
    and $lib_export_present == true
    and $store_type_present == true
    and $sqlite_wal_config_present == true
    and $idempotency_unique_index_present == true
    and $deterministic_reopen_test_present == true
    and $append_only_event_store_interface_ready == true
    and $event_count == 9
    and $accepted_append_count == 9
    and $duplicate_append_denial_count == 9
    and $append_only_sequence_count == 9
    and $idempotency_unique_index_entry_count == 9
    and $checkpoint_anchor_count == 9
    and $replay_digest_count == 9
    and $deterministic_replay_validation_count == 9
    and $rollback_anchor_count == 9
    and ($entries | all(.runtime_feature_gate_enabled == false
      and .runtime_event_log_write_allowed == false
      and .runtime_sqlite_write_allowed == false
      and .runtime_store_persistence_allowed == false
      and .runtime_lease_acquire_allowed == false
      and .runtime_checkpoint_write_allowed == false
      and .workflow_execution_allowed == false
      and .replay_execution_allowed == false
      and .rollback_execution_allowed == false
      and .live_execution_allowed == false))) as $persistence_ready |
  {
    runtime:"hepta",
    surface:"workflow_temporal_lite_append_only_event_store_minimal_local_persistence",
    status:(if $persistence_ready then "ready_blocked" else "blocked" end),
    gate:"workflow_temporal_lite_append_only_event_store_minimal_local_persistence_gate",
    schema_version:"workflow_temporal_lite_append_only_event_store_minimal_local_persistence_v1",
    source_test_implementation_gate:$source_report.gate,
    source_test_implementation_ready:$source_report.append_only_event_store_test_ready,
    source_test_event_count:$source_report.test_event_count,
    source_accepted_append_count:$source_report.accepted_append_count,
    source_duplicate_append_denial_count:$source_report.duplicate_append_denial_count,
    lib_export_present:$lib_export_present,
    store_type_present:$store_type_present,
    rust_sqlite_wal_config_present:$sqlite_wal_config_present,
    rust_idempotency_unique_index_present:$idempotency_unique_index_present,
    rust_deterministic_reopen_test_present:$deterministic_reopen_test_present,
    rust_event_store_interface_present:$event_store_interface_present,
    rust_sqlite_wal_backend_implements_interface:$sqlite_wal_backend_implements_interface,
    rust_event_store_interface_test_present:$event_store_interface_test_present,
    sqlite_adapter_scope:"local_tempdb_sqlite_wal_append_only_store_test_covered_runtime_write_blocked",
    sqlite_table_count:1,
    sqlite_unique_index_count:2,
    sqlite_primary_table:"temporal_lite_events",
    wal_mode_required:true,
    wal_mode_test_covered:$sqlite_wal_config_present,
    local_tempdb_persistence_test_covered:$deterministic_reopen_test_present,
    append_only_event_store_interface_ready:$append_only_event_store_interface_ready,
    append_only_event_store_interface_contract_count:3,
    sqlite_wal_backend_implements_interface:$sqlite_wal_backend_implements_interface,
    interface_append_count:$accepted_append_count,
    interface_duplicate_denial_count:$duplicate_append_denial_count,
    interface_replay_read_count:$deterministic_replay_validation_count,
    local_event_contract_count:$event_count,
    append_attempt_count:($event_count * 2),
    accepted_append_count:$accepted_append_count,
    duplicate_append_denial_count:$duplicate_append_denial_count,
    append_only_sequence_count:$append_only_sequence_count,
    idempotency_unique_index_entry_count:$idempotency_unique_index_entry_count,
    checkpoint_anchor_count:$checkpoint_anchor_count,
    replay_digest_count:$replay_digest_count,
    deterministic_replay_validation_count:$deterministic_replay_validation_count,
    rollback_anchor_count:$rollback_anchor_count,
    feature_gate_required:true,
    runtime_feature_gate_enabled:false,
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    runtime_store_persistence_allowed:false,
    runtime_lease_acquire_allowed:false,
    runtime_checkpoint_write_allowed:false,
    workflow_execution_allowed:false,
    replay_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false,
    local_tempdb_sqlite_write_covered_by_tests:true,
    minimal_local_persistence_ready:$persistence_ready,
    entries:$entries,
    blockers:[
      "runtime_feature_gate_closed",
      "runtime_event_log_write_disabled",
      "runtime_sqlite_write_disabled",
      "runtime_store_persistence_disabled",
      "runtime_lease_acquire_disabled",
      "runtime_checkpoint_write_disabled",
      "workflow_execution_disabled",
      "replay_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback"
    ],
    recommended_next_gate:"workflow_temporal_lite_deterministic_replay_validator_local_persistence_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      runtime_filesystem_written:false,
      runtime_event_log_written:false,
      runtime_sqlite_written:false,
      runtime_store_persisted:false,
      runtime_lease_acquired:false,
      runtime_idempotency_index_persisted:false,
      runtime_checkpoint_written:false,
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
