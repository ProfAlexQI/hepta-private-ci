#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-workflow-temporal-lite-checkpoint-and-rollback-anchor-feature-gated-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/workflow_temporal_lite_lease_idempotency_index_feature_gated_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_2026-06-29.md"

fail() {
  printf 'hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Temporal-lite checkpoint and rollback anchor report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Temporal-lite lease/idempotency Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Temporal-lite lease/idempotency architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Temporal-lite lease/idempotency report"
fi

lib_export_present=false
if grep -q 'hepta_workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-workflow-temporal-lite-lease-idempotency-index-feature-gated-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_WORKFLOW_TEMPORAL_LITE_LEASE_IDEMPOTENCY_INDEX_FEATURE_GATED_READBACK_2026-06-29.md" \
  '
  def pad3($n):
    (($n | tostring) | if length == 1 then "00" + . elif length == 2 then "0" + . else . end);
  def keyed($prefix; $entry):
    $prefix + "." + pad3($entry.sequence) + "." + $entry.event_contract_id;
  def lease_entry($entry): {
    event_contract_id:$entry.event_contract_id,
    sequence:$entry.sequence,
    event_id:$entry.event_id,
    checkpoint_anchor_key:$entry.checkpoint_anchor_key,
    rollback_anchor_key:$entry.rollback_anchor_key,
    checkpoint_readback_digest:$entry.checkpoint_readback_digest,
    rollback_readback_digest:$entry.rollback_readback_digest,
    lease_key:keyed("temporal-lite.lease.readback"; $entry),
    lease_token:("lease-token.v1." + pad3($entry.sequence) + "." + $entry.event_contract_id + "." + (($entry.checkpoint_anchor_key | length) | tostring)),
    lease_owner:"hepta-temporal-lite-test-worker",
    lease_ttl_ms:30000,
    lease_state:"projected_not_acquired",
    idempotency_index_key:keyed("temporal-lite.idempotency-index.readback"; $entry),
    idempotency_key:("idempotency-key.v1." + pad3($entry.sequence) + "." + $entry.event_contract_id + "." + (($entry.event_id | length) | tostring)),
    idempotency_index_state:"projected_not_persisted",
    duplicate_guard_key:keyed("temporal-lite.duplicate-guard.readback"; $entry),
    duplicate_guard_state:"projected_duplicate_denial_boundary",
    readback_state:"projected_in_memory_readback_only",
    lease_readback_projected:$entry.durable_anchor_pair_projected,
    lease_token_projected:true,
    idempotency_index_projected:$entry.durable_anchor_pair_projected,
    duplicate_guard_projected:$entry.durable_anchor_pair_projected,
    lease_acquired:false,
    lease_persisted:false,
    idempotency_index_written:false,
    idempotency_index_persisted:false,
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
  ($source_report.entries | map(lease_entry(.))) as $entries |
  ($entries | length) as $entry_count |
  ($entries | map(select(.lease_readback_projected == true)) | length) as $lease_readback_count |
  ($entries | map(select(.idempotency_index_projected == true)) | length) as $idempotency_index_readback_count |
  ($entries | map(select((.lease_token | length) > 0)) | length) as $lease_token_count |
  ($entries | map(select((.idempotency_key | length) > 0)) | length) as $idempotency_key_count |
  ($entries | map(select(.duplicate_guard_projected == true)) | length) as $duplicate_guard_count |
  ($entries | map(select(.lease_acquired == true)) | length) as $lease_acquired_count |
  ($entries | map(select(.lease_persisted == true)) | length) as $lease_persisted_count |
  ($entries | map(select(.idempotency_index_written == true)) | length) as $idempotency_index_written_count |
  ($entries | map(select(.idempotency_index_persisted == true)) | length) as $idempotency_index_persisted_count |
  ($source_report.checkpoint_and_rollback_anchor_readback_ready == true
    and $source_report.replay_projection_count == 9
    and $source_report.durable_anchor_pair_count == 9
    and $source_report.anchor_mismatch_count == 0
    and $source_report.runtime_feature_gate_enabled == false
    and $source_report.runtime_event_log_write_allowed == false
    and $source_report.runtime_sqlite_write_allowed == false
    and $source_report.checkpoint_write_allowed == false
    and $source_report.rollback_anchor_write_allowed == false
    and $source_report.anchor_persistence_allowed == false
    and $source_report.workflow_execution_allowed == false
    and $source_report.replay_execution_allowed == false
    and $source_report.rollback_execution_allowed == false
    and $source_report.live_execution_allowed == false
    and $lib_export_present == true
    and $entry_count == 9
    and $lease_readback_count == 9
    and $idempotency_index_readback_count == 9
    and $lease_token_count == 9
    and $idempotency_key_count == 9
    and $duplicate_guard_count == 9
    and $lease_acquired_count == 0
    and $lease_persisted_count == 0
    and $idempotency_index_written_count == 0
    and $idempotency_index_persisted_count == 0
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
    surface:"workflow_temporal_lite_lease_idempotency_index_feature_gated_readback",
    status:(if $readback_ready then "ready_blocked" else "blocked" end),
    gate:"workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_gate",
    schema_version:"workflow_temporal_lite_lease_idempotency_index_feature_gated_readback_v1",
    source_checkpoint_rollback_gate:$source_report.gate,
    source_checkpoint_rollback_ready:$source_report.checkpoint_and_rollback_anchor_readback_ready,
    source_anchor_pair_count:$source_report.durable_anchor_pair_count,
    lib_export_present:$lib_export_present,
    lease_scope:"test_only_lease_and_idempotency_readback_no_acquire_no_persistence",
    lease_readback_count:$lease_readback_count,
    idempotency_index_readback_count:$idempotency_index_readback_count,
    lease_token_count:$lease_token_count,
    idempotency_key_count:$idempotency_key_count,
    duplicate_guard_count:$duplicate_guard_count,
    lease_acquired_count:$lease_acquired_count,
    lease_persisted_count:$lease_persisted_count,
    idempotency_index_written_count:$idempotency_index_written_count,
    idempotency_index_persisted_count:$idempotency_index_persisted_count,
    feature_gate_required:true,
    runtime_feature_gate_enabled:false,
    lease_idempotency_readback_materialized:$readback_ready,
    runtime_event_log_write_allowed:false,
    runtime_sqlite_write_allowed:false,
    lease_acquisition_allowed:false,
    lease_persistence_allowed:false,
    idempotency_index_write_allowed:false,
    idempotency_index_persistence_allowed:false,
    workflow_execution_allowed:false,
    replay_execution_allowed:false,
    rollback_execution_allowed:false,
    live_execution_allowed:false,
    lease_idempotency_index_readback_ready:$readback_ready,
    entries:$entries,
    blockers:[
      "runtime_feature_gate_closed",
      "runtime_event_log_write_disabled",
      "runtime_sqlite_write_disabled",
      "lease_acquisition_disabled",
      "lease_persistence_disabled",
      "idempotency_index_write_disabled",
      "idempotency_index_persistence_disabled",
      "workflow_execution_disabled",
      "replay_execution_disabled",
      "rollback_execution_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "temporal_lite_event_log_sqlite_adapter_feature_gated_readback"
    ],
    recommended_next_gate:"temporal_lite_event_log_sqlite_adapter_feature_gated_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      filesystem_written:false,
      event_log_written:false,
      sqlite_written:false,
      lease_acquired:false,
      lease_persisted:false,
      idempotency_index_written:false,
      idempotency_index_persisted:false,
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
