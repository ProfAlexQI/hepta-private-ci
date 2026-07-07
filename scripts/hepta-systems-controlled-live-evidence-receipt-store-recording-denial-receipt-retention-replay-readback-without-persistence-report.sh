#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-recording-denial-receipt-readback-without-persistence-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_READBACK_WITHOUT_PERSISTENCE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-recording-denial-receipt-retention-replay-readback-without-persistence-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable recording denial receipt report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing controlled-live evidence receipt store denial receipt retention replay Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the controlled-live evidence receipt store denial receipt retention replay report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

source_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECORDING_DENIAL_RECEIPT_JSON:-}"
if [[ -n "$source_json" ]]; then
  [[ -f "$source_json" ]] || fail "missing cached recording denial receipt report: $source_json"
else
  source_json="$tmpdir/recording-denial-receipt.json"
  "$SOURCE_REPORT" >"$source_json" || fail "failed to render recording denial receipt report"
fi
jq -e . "$source_json" >/dev/null || fail "recording denial receipt report did not render valid JSON"

jq -n \
  --slurpfile source "$source_json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-controlled-live-evidence-receipt-store-recording-denial-receipt-retention-replay-readback-without-persistence-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECORDING_DENIAL_RECEIPT_RETENTION_REPLAY_READBACK_WITHOUT_PERSISTENCE_2026-07-07.md" \
  --arg collection_id "controlled-live-evidence-receipt-store-recording-denial-retention-replay" \
  --arg collection_route "readback://controlled-live/evidence-receipt-store/recording-denial-receipts/retention-replay" \
  '
  def hyphen_id($id):
    $id | gsub("_"; "-");
  ($source[0]) as $src |
  ($src.entries | map({
    id:("evidence_receipt_store_recording_denial_receipt_retention_replay_without_persistence_" + .source_blocker_id),
    source_blocker_id,
    source_denial_receipt_id:.denial_receipt_id,
    source_denial_receipt_route:.denial_receipt_route,
    source_denial_receipt_digest:.denial_receipt_digest,
    source_denial_receipt_idempotency_key:.denial_receipt_idempotency_key,
    retention_policy_id:("retention-policy:controlled-live-evidence-receipt-store-recording-denial:" + .source_blocker_id),
    retention_policy_route:($collection_route + "/retention/" + hyphen_id(.source_blocker_id)),
    expiry_guard_id:("expiry-guard:controlled-live-evidence-receipt-store-recording-denial:" + .source_blocker_id),
    replay_key:("replay-key:controlled-live-evidence-receipt-store-recording-denial:" + .source_blocker_id),
    replay_idempotency_key:("controlled-live-evidence-receipt-store.recording-denial-retention-replay.idempotency." + .source_blocker_id),
    replay_readback_route:($collection_route + "/replay/" + hyphen_id(.source_blocker_id)),
    retention_readback_route:($collection_route + "/readback/" + hyphen_id(.source_blocker_id)),
    garbage_collection_denial_id:("garbage-collection-denial:controlled-live-evidence-receipt-store-recording-denial:" + .source_blocker_id),
    supersession_guard_id:("supersession-guard:controlled-live-evidence-receipt-store-recording-denial:" + .source_blocker_id),
    zero_effect_digest:("sha256:controlled-live-evidence-receipt-store-recording-denial-retention-replay-zero-effect:" + .source_blocker_id),
    retention_state:"projected_not_persisted",
    replay_state:"projected_not_written",
    operator_display_order,
    operator_status,
    observed_state:"recording_denial_receipt_retention_replay_projected_without_persistence",
    previous_state,
    current_state,
    state_delta,
    owner,
    risk_bucket,
    operator_label,
    required_evidence,
    retention_policy_projected:true,
    expiry_guard_projected:true,
    replay_key_projected:true,
    replay_idempotency_key_projected:true,
    retention_readback_route_projected:true,
    replay_readback_route_projected:true,
    garbage_collection_denial_projected:true,
    supersession_guard_projected:true,
    zero_effect_digest_projected:true,
    source_denial_receipt_attached:true,
    retention_policy_persistence_allowed:false,
    retention_policy_persisted:false,
    replay_index_write_allowed:false,
    replay_index_written:false,
    expiry_enforcement_allowed:false,
    expiry_enforced:false,
    garbage_collection_allowed:false,
    garbage_collection_performed:false,
    denial_receipt_persistence_allowed:false,
    denial_receipt_persisted:false,
    receipt_persistence_allowed:false,
    receipt_persisted:false,
    receipt_store_write_allowed:false,
    receipt_store_written:false,
    ledger_write_allowed:false,
    ledger_written:false,
    workflow_event_log_write_allowed:false,
    workflow_event_log_written:false,
    sqlite_write_allowed:false,
    sqlite_written:false,
    credential_read_allowed:false,
    live_mutation_allowed:false
  })) as $entries |
  ($entries | map(select(.retention_policy_projected == true)) | length) as $retention_policy_projected_count |
  ($entries | map(select(.expiry_guard_projected == true)) | length) as $expiry_guard_projected_count |
  ($entries | map(select(.replay_key_projected == true)) | length) as $replay_key_projected_count |
  ($entries | map(select(.replay_idempotency_key_projected == true)) | length) as $replay_idempotency_key_projected_count |
  ($entries | map(.replay_idempotency_key) | unique | length) as $replay_idempotency_key_unique_count |
  ($entries | map(select(.retention_readback_route_projected == true)) | length) as $retention_readback_route_projected_count |
  ($entries | map(select(.replay_readback_route_projected == true)) | length) as $replay_readback_route_projected_count |
  ($entries | map(select(.garbage_collection_denial_projected == true)) | length) as $garbage_collection_denial_projected_count |
  ($entries | map(select(.supersession_guard_projected == true)) | length) as $supersession_guard_projected_count |
  ($entries | map(select(.zero_effect_digest_projected == true)) | length) as $zero_effect_digest_projected_count |
  ($entries | map(select(.source_denial_receipt_attached == true)) | length) as $source_denial_receipt_attached_count |
  ($entries | map(select(.retention_policy_persisted == true)) | length) as $retention_policy_persisted_count |
  ($entries | map(select(.replay_index_written == true)) | length) as $replay_index_written_count |
  ($entries | map(select(.expiry_enforced == true)) | length) as $expiry_enforced_count |
  ($entries | map(select(.garbage_collection_performed == true)) | length) as $garbage_collection_performed_count |
  ($entries | map(select(.denial_receipt_persisted == true)) | length) as $denial_receipt_persisted_count |
  ($entries | map(select(.receipt_store_written == true)) | length) as $receipt_store_written_count |
  ($entries | map(select(.receipt_persisted == true or .denial_receipt_persisted == true)) | length) as $receipt_persisted_count |
  ($entries | map(select(.ledger_written == true)) | length) as $ledger_written_count |
  ($entries | map(select(.workflow_event_log_written == true)) | length) as $workflow_event_log_written_count |
  ($entries | map(select(.sqlite_written == true)) | length) as $sqlite_written_count |
  ($entries | map(select(.live_mutation_allowed == true)) | length) as $live_mutation_allowed_count |
  ($src.recording_denial_receipt_readback_ready == true
    and $src.denial_receipt_entry_count == 7
    and $src.denial_receipt_persisted_count == 0
    and $src.receipt_store_written_count == 0
    and $src.live_execution_allowed == false
    and $lib_export_present == true
    and ($entries | length) == 7
    and $retention_policy_projected_count == 7
    and $expiry_guard_projected_count == 7
    and $replay_key_projected_count == 7
    and $replay_idempotency_key_projected_count == 7
    and $replay_idempotency_key_unique_count == 7
    and $retention_readback_route_projected_count == 7
    and $replay_readback_route_projected_count == 7
    and $garbage_collection_denial_projected_count == 7
    and $supersession_guard_projected_count == 7
    and $zero_effect_digest_projected_count == 7
    and $source_denial_receipt_attached_count == 7
    and $retention_policy_persisted_count == 0
    and $replay_index_written_count == 0
    and $expiry_enforced_count == 0
    and $garbage_collection_performed_count == 0
    and $denial_receipt_persisted_count == 0
    and $receipt_store_written_count == 0
    and $receipt_persisted_count == 0
    and $ledger_written_count == 0
    and $workflow_event_log_written_count == 0
    and $sqlite_written_count == 0
    and $live_mutation_allowed_count == 0
    and ($entries | all(.observed_state == "recording_denial_receipt_retention_replay_projected_without_persistence"
      and .previous_state == "missing"
      and .current_state == "missing"
      and .state_delta == "unchanged_missing"
      and .retention_state == "projected_not_persisted"
      and .replay_state == "projected_not_written"
      and .retention_policy_projected == true
      and .expiry_guard_projected == true
      and .replay_key_projected == true
      and .replay_idempotency_key_projected == true
      and .retention_readback_route_projected == true
      and .replay_readback_route_projected == true
      and .garbage_collection_denial_projected == true
      and .supersession_guard_projected == true
      and .zero_effect_digest_projected == true
      and .source_denial_receipt_attached == true
      and .retention_policy_persistence_allowed == false
      and .retention_policy_persisted == false
      and .replay_index_write_allowed == false
      and .replay_index_written == false
      and .expiry_enforcement_allowed == false
      and .expiry_enforced == false
      and .garbage_collection_allowed == false
      and .garbage_collection_performed == false
      and .denial_receipt_persistence_allowed == false
      and .denial_receipt_persisted == false
      and .receipt_persistence_allowed == false
      and .receipt_persisted == false
      and .receipt_store_write_allowed == false
      and .receipt_store_written == false
      and .ledger_write_allowed == false
      and .ledger_written == false
      and .workflow_event_log_write_allowed == false
      and .workflow_event_log_written == false
      and .sqlite_write_allowed == false
      and .sqlite_written == false
      and .credential_read_allowed == false
      and .live_mutation_allowed == false))) as $ready |
  {
    runtime:"hepta",
    surface:"controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence_gate",
    schema_version:"controlled_live_evidence_receipt_store_recording_denial_receipt_retention_replay_readback_without_persistence_v1",
    plugin_id:"hepta-system@hepta-local",
    source_recording_denial_receipt_readback_ready:$src.recording_denial_receipt_readback_ready,
    source_denial_receipt_entry_count:$src.denial_receipt_entry_count,
    source_denial_receipt_persisted_count:$src.denial_receipt_persisted_count,
    source_receipt_store_written_count:$src.receipt_store_written_count,
    source_live_execution_allowed:$src.live_execution_allowed,
    lib_export_present:$lib_export_present,
    retention_replay_collection_id:$collection_id,
    retention_replay_collection_route:$collection_route,
    retention_replay_entry_count:($entries | length),
    retention_policy_projected_count:$retention_policy_projected_count,
    expiry_guard_projected_count:$expiry_guard_projected_count,
    replay_key_projected_count:$replay_key_projected_count,
    replay_idempotency_key_projected_count:$replay_idempotency_key_projected_count,
    replay_idempotency_key_unique_count:$replay_idempotency_key_unique_count,
    retention_readback_route_projected_count:$retention_readback_route_projected_count,
    replay_readback_route_projected_count:$replay_readback_route_projected_count,
    garbage_collection_denial_projected_count:$garbage_collection_denial_projected_count,
    supersession_guard_projected_count:$supersession_guard_projected_count,
    zero_effect_digest_projected_count:$zero_effect_digest_projected_count,
    source_denial_receipt_attached_count:$source_denial_receipt_attached_count,
    retention_policy_persisted_count:$retention_policy_persisted_count,
    replay_index_written_count:$replay_index_written_count,
    expiry_enforced_count:$expiry_enforced_count,
    garbage_collection_performed_count:$garbage_collection_performed_count,
    denial_receipt_persisted_count:$denial_receipt_persisted_count,
    receipt_store_written_count:$receipt_store_written_count,
    receipt_persisted_count:$receipt_persisted_count,
    ledger_written_count:$ledger_written_count,
    workflow_event_log_written_count:$workflow_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_mutation_allowed_count:$live_mutation_allowed_count,
    retention_replay_readback_ready:$ready,
    retention_policy_persistence_allowed:false,
    replay_index_write_allowed:false,
    expiry_enforcement_allowed:false,
    garbage_collection_allowed:false,
    denial_receipt_persistence_allowed:false,
    receipt_persistence_allowed:false,
    receipt_store_write_allowed:false,
    receipt_store_written:false,
    ledger_write_allowed:false,
    workflow_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    credential_read_allowed:false,
    live_execution_allowed:false,
    blockers:[
      "retention_policy_persistence_disabled",
      "replay_index_write_disabled",
      "expiry_enforcement_disabled",
      "garbage_collection_disabled",
      "denial_receipt_persistence_disabled",
      "receipt_persistence_disabled",
      "receipt_store_write_disabled",
      "ledger_write_disabled",
      "workflow_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    entries:$entries,
    next_actions:[
      "controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance",
      "keep_retention_replay_projected_without_writes_until_acceptance_preconditions_exist"
    ],
    recommended_next_gate:"controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      retention_policy_persisted:false,
      replay_index_written:false,
      expiry_enforced:false,
      garbage_collection_performed:false,
      denial_receipt_persisted:false,
      receipt_persisted:false,
      receipt_store_written:false,
      ledger_written:false,
      workflow_event_log_written:false,
      sqlite_written:false,
      credential_read:false,
      native_post_mutation_performed:false,
      gateway_or_auth_mutated:false,
      telegram_transport_mutated:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      replay_executed:false,
      rollback_executed:false,
      kill_switch_rehearsal_executed:false,
      kill_switch_mutated:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }'
