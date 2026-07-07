#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-denial-readback-without-write-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_READBACK_WITHOUT_WRITE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-denial-retention-replay-readback-without-write-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable receipt-store write denial report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing write-denial retention/replay Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the write-denial retention/replay report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

source_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_RECEIPT_STORE_WRITE_DENIAL_JSON:-}"
source_cache_input_present=false
if [[ -n "$source_json" ]]; then
  [[ -f "$source_json" ]] || fail "missing cached receipt-store write denial report: $source_json"
  source_cache_input_present=true
else
  source_json="$tmpdir/receipt-store-write-denial.json"
  "$SOURCE_REPORT" >"$source_json" || fail "failed to render receipt-store write denial report"
fi
jq -e . "$source_json" >/dev/null || fail "receipt-store write denial report did not render valid JSON"

jq -n \
  --slurpfile source "$source_json" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson source_cache_input_present "$source_cache_input_present" \
  --arg gate "scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-receipt-store-write-denial-retention-replay-readback-without-write-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_RECEIPT_STORE_WRITE_DENIAL_RETENTION_REPLAY_READBACK_WITHOUT_WRITE_2026-07-07.md" \
  --arg collection_id "controlled-live-evidence-receipt-store-write-denial-retention-replay" \
  --arg collection_route "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/retention-replay" \
  '
  def hyphen_id($id):
    $id | gsub("_"; "-");
  ($source[0]) as $src |
  ($src.entries | map({
    id:("evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_without_write_" + .source_blocker_id),
    source_blocker_id,
    source_receipt_store_write_denial_id:.receipt_store_write_denial_id,
    source_receipt_store_write_denial_route:.receipt_store_write_denial_route,
    source_receipt_store_write_denial_reason:.receipt_store_write_denial_reason,
    retention_policy_id:("receipt-store-write-denial-retention-policy:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    retention_policy_route:("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/retention/" + hyphen_id(.source_blocker_id)),
    expiry_guard_id:("receipt-store-write-denial-expiry-guard:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    replay_key:("receipt-store-write-denial-replay-key:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    replay_idempotency_key:("receipt-store-write-denial-replay-idempotency:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    retention_readback_route:("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/retention/" + hyphen_id(.source_blocker_id)),
    replay_readback_route:("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/receipt-store-write-denial/replay/" + hyphen_id(.source_blocker_id)),
    garbage_collection_denial_id:("receipt-store-write-denial-gc-denial:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    supersession_guard_id:("receipt-store-write-denial-supersession-guard:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    zero_effect_digest:("sha256:receipt-store-write-denial-zero-effect:" + .source_blocker_id),
    retention_state:"projected_not_persisted",
    replay_state:"projected_not_executed",
    operator_display_order,
    operator_status,
    observed_state:"receipt_store_write_denial_retention_replay_projected_without_write",
    previous_state,
    current_state,
    state_delta,
    owner,
    risk_bucket,
    operator_label,
    required_evidence,
    source_packet_unsent,
    source_write_denial_attached:.receipt_store_write_denial_projected,
    receipt_store_write_denied,
    receipt_store_write_disabled,
    retention_policy_projected:true,
    expiry_guard_projected:true,
    replay_key_projected:true,
    replay_idempotency_key_projected:true,
    retention_readback_route_projected:true,
    replay_readback_route_projected:true,
    garbage_collection_denial_projected:true,
    supersession_guard_projected:true,
    zero_effect_digest_projected:true,
    retention_policy_persistence_allowed:false,
    retention_policy_persisted:false,
    replay_index_write_allowed:false,
    replay_index_written:false,
    expiry_enforcement_allowed:false,
    expiry_enforced:false,
    garbage_collection_allowed:false,
    garbage_collection_performed:false,
    receipt_store_write_attempt_allowed:false,
    receipt_store_write_attempt_recorded:false,
    receipt_store_write_allowed:false,
    receipt_store_written:false,
    receipt_persistence_allowed:false,
    receipt_persisted:false,
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
  ($entries | map(select(.source_write_denial_attached == true)) | length) as $source_write_denial_attached_count |
  ($entries | map(select(.retention_policy_persisted == true)) | length) as $retention_policy_persisted_count |
  ($entries | map(select(.replay_index_written == true)) | length) as $replay_index_written_count |
  ($entries | map(select(.expiry_enforced == true)) | length) as $expiry_enforced_count |
  ($entries | map(select(.garbage_collection_performed == true)) | length) as $garbage_collection_performed_count |
  ($entries | map(select(.receipt_store_write_attempt_recorded == true)) | length) as $receipt_store_write_attempt_recorded_count |
  ($entries | map(select(.receipt_store_written == true)) | length) as $receipt_store_written_count |
  ($entries | map(select(.receipt_persisted == true)) | length) as $receipt_persisted_count |
  ($entries | map(select(.ledger_written == true)) | length) as $ledger_written_count |
  ($entries | map(select(.workflow_event_log_written == true)) | length) as $workflow_event_log_written_count |
  ($entries | map(select(.sqlite_written == true)) | length) as $sqlite_written_count |
  ($entries | map(select(.live_mutation_allowed == true)) | length) as $live_mutation_allowed_count |
  ($src.receipt_store_write_denial_readback_ready == true
    and $src.write_denial_entry_count == 7
    and $src.receipt_store_write_denied_count == 7
    and $src.receipt_store_write_allowed_count == 0
    and $src.receipt_store_write_attempt_recorded_count == 0
    and $src.receipt_store_written_count == 0
    and $src.receipt_persisted_count == 0
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
    and $source_write_denial_attached_count == 7
    and $retention_policy_persisted_count == 0
    and $replay_index_written_count == 0
    and $expiry_enforced_count == 0
    and $garbage_collection_performed_count == 0
    and $receipt_store_write_attempt_recorded_count == 0
    and $receipt_store_written_count == 0
    and $receipt_persisted_count == 0
    and $ledger_written_count == 0
    and $workflow_event_log_written_count == 0
    and $sqlite_written_count == 0
    and $live_mutation_allowed_count == 0
    and ($entries | all(.observed_state == "receipt_store_write_denial_retention_replay_projected_without_write"
      and .previous_state == "missing"
      and .current_state == "missing"
      and .state_delta == "unchanged_missing"
      and .source_packet_unsent == true
      and .source_write_denial_attached == true
      and .receipt_store_write_denied == true
      and .receipt_store_write_disabled == true
      and .retention_policy_projected == true
      and .expiry_guard_projected == true
      and .replay_key_projected == true
      and .replay_idempotency_key_projected == true
      and .retention_readback_route_projected == true
      and .replay_readback_route_projected == true
      and .garbage_collection_denial_projected == true
      and .supersession_guard_projected == true
      and .zero_effect_digest_projected == true
      and .retention_policy_persistence_allowed == false
      and .retention_policy_persisted == false
      and .replay_index_write_allowed == false
      and .replay_index_written == false
      and .expiry_enforcement_allowed == false
      and .expiry_enforced == false
      and .garbage_collection_allowed == false
      and .garbage_collection_performed == false
      and .receipt_store_write_attempt_allowed == false
      and .receipt_store_write_attempt_recorded == false
      and .receipt_store_write_allowed == false
      and .receipt_store_written == false
      and .receipt_persistence_allowed == false
      and .receipt_persisted == false
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
    surface:"controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_gate",
    schema_version:"controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_retention_replay_readback_without_write_v1",
    plugin_id:"hepta-system@hepta-local",
    source_write_denial_ready:$src.receipt_store_write_denial_readback_ready,
    source_write_denial_entry_count:$src.write_denial_entry_count,
    source_receipt_store_write_denied_count:$src.receipt_store_write_denied_count,
    source_receipt_store_write_allowed_count:$src.receipt_store_write_allowed_count,
    source_receipt_store_write_attempt_recorded_count:$src.receipt_store_write_attempt_recorded_count,
    source_receipt_store_written_count:$src.receipt_store_written_count,
    source_receipt_persisted_count:$src.receipt_persisted_count,
    source_live_execution_allowed:$src.live_execution_allowed,
    source_cache_mode:(if $source_cache_input_present then "provided_source_json" else "rendered_once_temp_source_json" end),
    source_cache_input_present:$source_cache_input_present,
    source_report_render_count:(if $source_cache_input_present then 0 else 1 end),
    target_source_reuse_count:1,
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
    source_write_denial_attached_count:$source_write_denial_attached_count,
    retention_policy_persisted_count:$retention_policy_persisted_count,
    replay_index_written_count:$replay_index_written_count,
    expiry_enforced_count:$expiry_enforced_count,
    garbage_collection_performed_count:$garbage_collection_performed_count,
    receipt_store_write_attempt_recorded_count:$receipt_store_write_attempt_recorded_count,
    receipt_store_written_count:$receipt_store_written_count,
    receipt_persisted_count:$receipt_persisted_count,
    ledger_written_count:$ledger_written_count,
    workflow_event_log_written_count:$workflow_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_mutation_allowed_count:$live_mutation_allowed_count,
    write_denial_retention_replay_readback_ready:$ready,
    retention_policy_persistence_allowed:false,
    replay_index_write_allowed:false,
    expiry_enforcement_allowed:false,
    garbage_collection_allowed:false,
    receipt_store_write_attempt_allowed:false,
    receipt_store_write_allowed:false,
    receipt_persistence_allowed:false,
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
      "receipt_store_write_attempt_recording_disabled",
      "receipt_store_write_disabled",
      "receipt_persistence_disabled",
      "ledger_write_disabled",
      "workflow_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    entries:$entries,
    next_actions:[
      "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write",
      "keep_write_denial_retention_replay_unpersisted_unexecuted"
    ],
    recommended_next_gate:"controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_positive_preconditions_readback_without_write",
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
      receipt_store_write_attempt_recorded:false,
      receipt_store_written:false,
      receipt_persisted:false,
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
