#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
ADAPTER_REPORT="$ROOT/scripts/hepta-systems-workflow-durable-store-adapter-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/workflow_durable_store_test_only_append_fixture.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_DURABLE_STORE_TEST_ONLY_APPEND_FIXTURE_2026-06-27.md"

fail() {
  printf 'hepta-systems-workflow-durable-store-test-only-append-fixture-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$ADAPTER_REPORT" ]] || fail "missing executable workflow durable store adapter report: $ADAPTER_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 7 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 7 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the workflow durable store test-only append fixture report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_durable_store_test_only_append_fixture_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile adapter <("$ADAPTER_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-workflow-durable-store-test-only-append-fixture-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_DURABLE_STORE_TEST_ONLY_APPEND_FIXTURE_2026-06-27.md" \
  '
  def fixture_entry($entry; $index): {
    event_contract_id:$entry.event_contract_id,
    record_kind:$entry.record_kind,
    fixture_sequence:($index + 1),
    fixture_append_key:("test-only.append-fixture." + $entry.event_contract_id),
    fixture_idempotency_key:("test-only.idempotency." + $entry.event_contract_id + "." + $entry.lease_scope),
    fixture_checkpoint_key:("test-only.checkpoint." + $entry.event_contract_id + "." + $entry.checkpoint_policy),
    fixture_replay_validation_key:("test-only.replay-validation." + $entry.event_contract_id + "." + $entry.replay_validation_policy),
    fixture_rollback_anchor:$entry.rollback_anchor,
    append_only_order_validated:true,
    idempotency_key_validated:true,
    duplicate_append_denied:true,
    checkpoint_metadata_validated:true,
    replay_validation_metadata_validated:true,
    rollback_metadata_validated:true,
    feature_gate_required:$entry.feature_gate_required,
    runtime_feature_gate_enabled:$entry.feature_gate_enabled,
    runtime_event_log_write_allowed:$entry.event_log_write_enabled,
    runtime_sqlite_write_allowed:$entry.sqlite_write_enabled,
    fixture_persistence_allowed:false,
    workflow_execution_allowed:$entry.workflow_execution_enabled,
    replay_execution_allowed:$entry.replay_execution_enabled,
    rollback_execution_allowed:$entry.rollback_execution_enabled,
    live_execution_allowed:$entry.live_execution_enabled
  };
  ($adapter[0]) as $adapter |
  ($adapter.entries | to_entries | map(fixture_entry(.value; .key))) as $entries |
  ($entries | map(select(.append_only_order_validated == true)) | length) as $append_only_sequence_count |
  ($entries | map(select(.idempotency_key_validated == true)) | length) as $idempotency_fixture_count |
  ($entries | map(select(.checkpoint_metadata_validated == true)) | length) as $checkpoint_fixture_count |
  ($entries | map(select(.replay_validation_metadata_validated == true)) | length) as $replay_validation_fixture_count |
  ($entries | map(select(.rollback_metadata_validated == true)) | length) as $rollback_fixture_count |
  ($entries | map(select(.duplicate_append_denied == true)) | length) as $duplicate_append_denial_count |
  ($adapter.temporal_lite_adapter_ready == true
    and $adapter.event_contract_count == 9
    and $adapter.adapter_entry_count == 9
    and $adapter.feature_gate_required == true
    and $adapter.feature_gate_enabled == false
    and $adapter.ready_for_event_log_write == false
    and $adapter.ready_for_sqlite_write == false
    and $adapter.ready_for_workflow_execution == false
    and $adapter.ready_for_replay_execution == false
    and $adapter.ready_for_rollback_execution == false
    and $adapter.ready_for_live_execution == false
    and $lib_export_present == true
    and ($entries | length) == 9
    and $append_only_sequence_count == 9
    and $idempotency_fixture_count == 9
    and $checkpoint_fixture_count == 9
    and $replay_validation_fixture_count == 9
    and $rollback_fixture_count == 9
    and $duplicate_append_denial_count == 9
    and ($entries | all(.feature_gate_required == true
      and .runtime_feature_gate_enabled == false
      and .runtime_event_log_write_allowed == false
      and .runtime_sqlite_write_allowed == false
      and .fixture_persistence_allowed == false
      and .workflow_execution_allowed == false
      and .replay_execution_allowed == false
      and .rollback_execution_allowed == false
      and .live_execution_allowed == false))) as $test_only_append_fixture_ready |
  {
    runtime:"hepta",
    surface:"workflow_durable_store_test_only_append_fixture",
    status:(if $test_only_append_fixture_ready then "ready" else "blocked" end),
    gate:"hepta_workflow_durable_store_test_only_append_fixture_gate",
    schema_version:"workflow_durable_store_test_only_append_fixture_v1",
    source_adapter_gate:$adapter.gate,
    source_adapter_ready:$adapter.temporal_lite_adapter_ready,
    source_adapter_event_contract_count:$adapter.event_contract_count,
    source_adapter_entry_count:$adapter.adapter_entry_count,
    lib_export_present:$lib_export_present,
    test_fixture_scope:"test_only_in_memory_fixture_no_runtime_store_write",
    event_contract_count:$adapter.event_contract_count,
    fixture_entry_count:($entries | length),
    append_only_sequence_count:$append_only_sequence_count,
    idempotency_fixture_count:$idempotency_fixture_count,
    checkpoint_fixture_count:$checkpoint_fixture_count,
    replay_validation_fixture_count:$replay_validation_fixture_count,
    rollback_fixture_count:$rollback_fixture_count,
    duplicate_append_denial_count:$duplicate_append_denial_count,
    feature_gate_required:true,
    runtime_feature_gate_enabled:false,
    test_fixture_materialized:$test_only_append_fixture_ready,
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    fixture_persistence_allowed:false,
    workflow_execution_allowed:false,
    replay_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false,
    test_only_append_fixture_ready:$test_only_append_fixture_ready,
    entries:$entries,
    blockers:[
      "runtime_feature_gate_closed",
      "runtime_event_log_write_disabled",
      "runtime_sqlite_write_disabled",
      "fixture_persistence_disabled",
      "workflow_execution_disabled",
      "replay_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "phase8_internal_read_only_hepta_system_status_invocation_without_external_network_or_mutation"
    ],
    next_migration_step:"phase8_internal_read_only_hepta_system_status_invocation_without_external_network_or_mutation",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      filesystem_written:false,
      fixture_file_written:false,
      workflow_event_log_mutated:false,
      event_log_written:false,
      sqlite_written:false,
      lease_acquired:false,
      idempotency_index_mutated:false,
      checkpoint_written:false,
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
