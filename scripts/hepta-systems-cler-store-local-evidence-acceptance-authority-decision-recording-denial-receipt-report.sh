#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-boundary-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_DENIAL_RECEIPT_2026-07-07.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-denial-receipt-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable local authority decision recording boundary source report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing local authority decision recording denial receipt Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

source_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_BOUNDARY_JSON:-}"
source_cache_input_present=false
source_report_render_count=0
if [[ -n "$source_json" ]]; then
  [[ -f "$source_json" ]] || fail "missing cached local authority decision recording boundary report: $source_json"
  source_cache_input_present=true
else
  source_json="$tmpdir/local-authority-decision-recording-boundary.json"
  "$SOURCE_REPORT" >"$source_json" || fail "failed to render local authority decision recording boundary report"
  source_report_render_count=1
fi
jq -e . "$source_json" >/dev/null || fail "source report did not render valid JSON"

jq -n \
  --slurpfile source "$source_json" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson source_cache_input_present "$source_cache_input_present" \
  --argjson source_report_render_count "$source_report_render_count" \
  --arg collection_id "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-decision-recording-denial-receipts" \
  --arg collection_route "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-denial-receipts" \
  --arg denial_schema_version "controlled_live_local_evidence_acceptance_authority_decision_recording_denial_receipt_v1" \
  --arg denial_reason "local_evidence_acceptance_authority_decision_recording_disabled_authority_missing_no_local_acceptance" \
  '
  def hyphen_id($id): $id | gsub("_"; "-");
  def denial_receipt_id($id):
    "local-evidence-acceptance-authority-decision-recording-denial-receipt:controlled-live-evidence-receipt-store:" + $id + ":not-persisted";
  def denial_digest($id):
    "sha256:local-evidence-acceptance-authority-decision-recording-denial-receipt:" + $id + ":not-persisted";
  def denial_idempotency($id):
    "local-evidence-acceptance-authority-decision-recording-denial-receipt-idempotency:controlled-live-evidence-receipt-store:" + $id;
  ($source[0]) as $src |
  ($src.entries | map({
    id:("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_without_persistence_" + .source_blocker_id),
    source_blocker_id,
    source_boundary_entry_id:.id,
    source_recording_boundary_id:.recording_boundary_id,
    source_recording_boundary_route:.recording_boundary_route,
    source_terminal_closeout_id,
    source_terminal_closeout_key,
    source_terminal_closeout_route,
    source_terminal_reason,
    source_terminal_state,
    source_persistence_denial_id,
    source_persistence_denial_route,
    source_persistence_denial_reason,
    source_packet_persistence_denial_receipt_id,
    source_non_send_readback_id:.source_packet_non_send_readback_id,
    source_non_send_readback_route:.source_packet_non_send_readback_route,
    source_authority_packet_id,
    source_authority_packet_route,
    source_authority_packet_key,
    source_authority_decision_request_id,
    source_authority_decision_request_route,
    source_non_authority_receipt_id,
    source_non_authority_receipt_route,
    source_authority_decision_record_id:.authority_decision_record_id,
    source_authority_decision_record_schema_version:.authority_decision_record_schema_version,
    source_authority_decision_idempotency_key:.authority_decision_idempotency_key,
    source_post_record_readback_route:.post_record_readback_route,
    source_rollback_anchor:.rollback_anchor,
    source_projected_denial_receipt_id:.denial_receipt_id,
    denial_receipt_id:denial_receipt_id(.source_blocker_id),
    denial_receipt_route:($collection_route + "/" + hyphen_id(.source_blocker_id)),
    denial_receipt_digest:denial_digest(.source_blocker_id),
    denial_receipt_schema_version:$denial_schema_version,
    denial_receipt_idempotency_key:denial_idempotency(.source_blocker_id),
    recording_denial_reason:$denial_reason,
    recording_denial_state:"authority_decision_recording_denied_without_persistence",
    observed_state:"local_evidence_acceptance_authority_decision_recording_denial_receipt_projected_without_persistence",
    source_recording_boundary_attached:.boundary_projected,
    source_terminal_closeout_attached,
    source_persistence_denial_attached,
    source_packet_persistence_denial_receipt_attached,
    source_non_send_readback_attached,
    source_authority_packet_attached,
    source_authority_decision_request_attached,
    source_non_authority_receipt_attached,
    source_authority_decision_record_id_attached:true,
    denial_receipt_projected:true,
    denial_receipt_digest_projected:true,
    denial_receipt_readback_route_projected:true,
    denial_receipt_idempotency_key_projected:true,
    recording_denial_reason_projected:true,
    recording_precondition_missing,
    authority_decision_recording_disabled:(.authority_decision_recording_allowed == false),
    authority_decision_recording_allowed:false,
    authority_decision_recorded:false,
    authority_decision_persisted:false,
    denial_receipt_persistence_allowed:false,
    denial_receipt_persisted:false,
    operator_packet_sent:false,
    operator_packet_persisted:false,
    non_authority_receipt_persisted:false,
    local_evidence_acceptance_authority_required,
    local_evidence_acceptance_authority_present:false,
    local_evidence_acceptance_allowed:false,
    local_evidence_acceptance_recording_allowed:false,
    local_evidence_acceptance_recorded:false,
    evidence_acceptance_recording_allowed:false,
    evidence_acceptance_recorded:false,
    evidence_recording_allowed:false,
    evidence_recorded:false,
    receipt_store_write_attempt_recording_allowed:false,
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
  ($entries | map(select(.denial_receipt_projected == true)) | length) as $denial_receipt_projected_count |
  ($entries | map(select(.denial_receipt_digest_projected == true)) | length) as $denial_receipt_digest_projected_count |
  ($entries | map(select(.denial_receipt_readback_route_projected == true)) | length) as $denial_receipt_readback_route_projected_count |
  ($entries | map(select(.denial_receipt_idempotency_key_projected == true)) | length) as $denial_receipt_idempotency_key_projected_count |
  ($entries | map(.denial_receipt_idempotency_key) | unique | length) as $denial_receipt_idempotency_key_unique_count |
  ($entries | map(select(.source_recording_boundary_attached == true)) | length) as $source_recording_boundary_attached_count |
  ($entries | map(select(.source_terminal_closeout_attached == true)) | length) as $source_terminal_closeout_attached_entry_count |
  ($entries | map(select(.source_persistence_denial_attached == true)) | length) as $source_persistence_denial_attached_entry_count |
  ($entries | map(select(.source_packet_persistence_denial_receipt_attached == true)) | length) as $source_packet_persistence_denial_receipt_attached_entry_count |
  ($entries | map(select(.source_non_send_readback_attached == true)) | length) as $source_non_send_readback_attached_entry_count |
  ($entries | map(select(.source_authority_packet_attached == true)) | length) as $source_authority_packet_attached_entry_count |
  ($entries | map(select(.source_authority_decision_request_attached == true)) | length) as $source_authority_decision_request_attached_entry_count |
  ($entries | map(select(.source_non_authority_receipt_attached == true)) | length) as $source_non_authority_receipt_attached_entry_count |
  ($entries | map(select(.source_authority_decision_record_id_attached == true)) | length) as $source_authority_decision_record_id_attached_count |
  ($entries | map(select(.recording_denial_reason_projected == true)) | length) as $recording_denial_reason_projected_count |
  ($entries | map(select(.recording_precondition_missing == true)) | length) as $recording_precondition_missing_count |
  ($entries | map(select(.authority_decision_recording_disabled == true)) | length) as $authority_decision_recording_disabled_count |
  ($entries | map(select(.authority_decision_recorded == true)) | length) as $authority_decision_recorded_count |
  ($entries | map(select(.authority_decision_persisted == true)) | length) as $authority_decision_persisted_count |
  ($entries | map(select(.denial_receipt_persisted == true)) | length) as $denial_receipt_persisted_count |
  ($entries | map(select(.operator_packet_sent == true)) | length) as $operator_packet_sent_count |
  ($entries | map(select(.operator_packet_persisted == true)) | length) as $operator_packet_persisted_count |
  ($entries | map(select(.non_authority_receipt_persisted == true)) | length) as $non_authority_receipt_persisted_count |
  ($entries | map(select(.local_evidence_acceptance_authority_present == true)) | length) as $local_evidence_acceptance_authority_present_count |
  ($entries | map(select(.local_evidence_acceptance_allowed == true)) | length) as $local_evidence_acceptance_allowed_count |
  ($entries | map(select(.local_evidence_acceptance_recorded == true)) | length) as $local_evidence_acceptance_recorded_count |
  ($entries | map(select(.evidence_acceptance_recorded == true)) | length) as $evidence_acceptance_recorded_count |
  ($entries | map(select(.evidence_recorded == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.receipt_store_write_attempt_recorded == true)) | length) as $receipt_store_write_attempt_recorded_count |
  ($entries | map(select(.receipt_store_written == true)) | length) as $receipt_store_written_count |
  ($entries | map(select(.receipt_persisted == true or .denial_receipt_persisted == true)) | length) as $receipt_persisted_count |
  ($entries | map(select(.ledger_written == true)) | length) as $ledger_written_count |
  ($entries | map(select(.workflow_event_log_written == true)) | length) as $workflow_event_log_written_count |
  ($entries | map(select(.sqlite_written == true)) | length) as $sqlite_written_count |
  ($entries | map(select(.live_mutation_allowed == true)) | length) as $live_mutation_allowed_count |
  ($src.local_evidence_acceptance_authority_decision_recording_boundary_readback_ready == true
    and $src.boundary_entry_count == 7
    and $src.boundary_projected_count == 7
    and $src.boundary_ready_count == 7
    and $src.source_terminal_closeout_attached_count == 7
    and $src.source_persistence_denial_attached_count == 7
    and $src.source_packet_persistence_denial_receipt_attached_count == 7
    and $src.source_non_send_readback_attached_count == 7
    and $src.source_authority_packet_attached_count == 7
    and $src.source_authority_decision_request_attached_count == 7
    and $src.source_non_authority_receipt_attached_count == 7
    and $src.decision_record_schema_projected_count == 7
    and $src.local_evidence_acceptance_authority_required_count == 7
    and $src.local_evidence_acceptance_authority_present_count == 0
    and $src.recording_precondition_missing_count == 7
    and $src.decision_recording_allowed_count == 0
    and $src.authority_decision_recorded_count == 0
    and $src.authority_decision_persisted_count == 0
    and $src.denial_receipt_persisted_count == 0
    and $src.operator_packet_sent_count == 0
    and $src.operator_packet_persisted_count == 0
    and $src.non_authority_receipt_persisted_count == 0
    and $src.local_evidence_acceptance_allowed_count == 0
    and $src.local_evidence_acceptance_recorded_count == 0
    and $src.evidence_acceptance_recorded_count == 0
    and $src.evidence_recorded_count == 0
    and $src.receipt_store_write_attempt_recorded_count == 0
    and $src.receipt_store_written_count == 0
    and $src.receipt_persisted_count == 0
    and $src.live_execution_allowed == false
    and $lib_export_present == true
    and ($entries | length) == 7
    and $denial_receipt_projected_count == 7
    and $denial_receipt_digest_projected_count == 7
    and $denial_receipt_readback_route_projected_count == 7
    and $denial_receipt_idempotency_key_projected_count == 7
    and $denial_receipt_idempotency_key_unique_count == 7
    and $source_recording_boundary_attached_count == 7
    and $source_terminal_closeout_attached_entry_count == 7
    and $source_persistence_denial_attached_entry_count == 7
    and $source_packet_persistence_denial_receipt_attached_entry_count == 7
    and $source_non_send_readback_attached_entry_count == 7
    and $source_authority_packet_attached_entry_count == 7
    and $source_authority_decision_request_attached_entry_count == 7
    and $source_non_authority_receipt_attached_entry_count == 7
    and $source_authority_decision_record_id_attached_count == 7
    and $recording_denial_reason_projected_count == 7
    and $recording_precondition_missing_count == 7
    and $authority_decision_recording_disabled_count == 7
    and $authority_decision_recorded_count == 0
    and $authority_decision_persisted_count == 0
    and $denial_receipt_persisted_count == 0
    and $operator_packet_sent_count == 0
    and $operator_packet_persisted_count == 0
    and $non_authority_receipt_persisted_count == 0
    and $local_evidence_acceptance_authority_present_count == 0
    and $local_evidence_acceptance_allowed_count == 0
    and $local_evidence_acceptance_recorded_count == 0
    and $evidence_acceptance_recorded_count == 0
    and $evidence_recorded_count == 0
    and $receipt_store_write_attempt_recorded_count == 0
    and $receipt_store_written_count == 0
    and $receipt_persisted_count == 0
    and $ledger_written_count == 0
    and $workflow_event_log_written_count == 0
    and $sqlite_written_count == 0
    and $live_mutation_allowed_count == 0
    and ($entries | all(.observed_state == "local_evidence_acceptance_authority_decision_recording_denial_receipt_projected_without_persistence"
      and .recording_denial_state == "authority_decision_recording_denied_without_persistence"
      and .source_recording_boundary_attached == true
      and .source_terminal_closeout_attached == true
      and .source_persistence_denial_attached == true
      and .source_packet_persistence_denial_receipt_attached == true
      and .source_non_send_readback_attached == true
      and .source_authority_packet_attached == true
      and .source_authority_decision_request_attached == true
      and .source_non_authority_receipt_attached == true
      and .source_authority_decision_record_id_attached == true
      and .denial_receipt_projected == true
      and .denial_receipt_digest_projected == true
      and .denial_receipt_readback_route_projected == true
      and .denial_receipt_idempotency_key_projected == true
      and .recording_denial_reason_projected == true
      and .recording_precondition_missing == true
      and .authority_decision_recording_disabled == true
      and .authority_decision_recording_allowed == false
      and .authority_decision_recorded == false
      and .authority_decision_persisted == false
      and .denial_receipt_persistence_allowed == false
      and .denial_receipt_persisted == false
      and .operator_packet_sent == false
      and .operator_packet_persisted == false
      and .non_authority_receipt_persisted == false
      and .local_evidence_acceptance_authority_required == true
      and .local_evidence_acceptance_authority_present == false
      and .local_evidence_acceptance_allowed == false
      and .local_evidence_acceptance_recording_allowed == false
      and .local_evidence_acceptance_recorded == false
      and .evidence_acceptance_recording_allowed == false
      and .evidence_acceptance_recorded == false
      and .evidence_recording_allowed == false
      and .evidence_recorded == false
      and .receipt_store_write_attempt_recording_allowed == false
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
    surface:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence_gate",
    schema_version:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence_v1",
    plugin_id:"hepta-system@hepta-local",
    source_recording_boundary_ready:$src.local_evidence_acceptance_authority_decision_recording_boundary_readback_ready,
    source_boundary_entry_count:$src.boundary_entry_count,
    source_boundary_projected_count:$src.boundary_projected_count,
    source_boundary_ready_count:$src.boundary_ready_count,
    source_terminal_closeout_attached_count:$src.source_terminal_closeout_attached_count,
    source_persistence_denial_attached_count:$src.source_persistence_denial_attached_count,
    source_packet_persistence_denial_receipt_attached_count:$src.source_packet_persistence_denial_receipt_attached_count,
    source_non_send_readback_attached_count:$src.source_non_send_readback_attached_count,
    source_authority_packet_attached_count:$src.source_authority_packet_attached_count,
    source_authority_decision_request_attached_count:$src.source_authority_decision_request_attached_count,
    source_non_authority_receipt_attached_count:$src.source_non_authority_receipt_attached_count,
    source_decision_record_schema_projected_count:$src.decision_record_schema_projected_count,
    source_local_evidence_acceptance_authority_required_count:$src.local_evidence_acceptance_authority_required_count,
    source_local_evidence_acceptance_authority_present_count:$src.local_evidence_acceptance_authority_present_count,
    source_recording_precondition_missing_count:$src.recording_precondition_missing_count,
    source_decision_recording_allowed_count:$src.decision_recording_allowed_count,
    source_authority_decision_recorded_count:$src.authority_decision_recorded_count,
    source_authority_decision_persisted_count:$src.authority_decision_persisted_count,
    source_denial_receipt_persisted_count:$src.denial_receipt_persisted_count,
    source_operator_packet_sent_count:$src.operator_packet_sent_count,
    source_operator_packet_persisted_count:$src.operator_packet_persisted_count,
    source_non_authority_receipt_persisted_count:$src.non_authority_receipt_persisted_count,
    source_local_evidence_acceptance_allowed_count:$src.local_evidence_acceptance_allowed_count,
    source_local_evidence_acceptance_recorded_count:$src.local_evidence_acceptance_recorded_count,
    source_evidence_acceptance_recorded_count:$src.evidence_acceptance_recorded_count,
    source_evidence_recorded_count:$src.evidence_recorded_count,
    source_receipt_store_write_attempt_recorded_count:$src.receipt_store_write_attempt_recorded_count,
    source_receipt_store_written_count:$src.receipt_store_written_count,
    source_receipt_persisted_count:$src.receipt_persisted_count,
    source_live_execution_allowed:$src.live_execution_allowed,
    source_cache_mode:(if $source_cache_input_present then "provided_source_json" else "rendered_once_temp_source_json" end),
    source_cache_input_present:$source_cache_input_present,
    source_report_render_count:$source_report_render_count,
    target_source_reuse_count:1,
    lib_export_present:$lib_export_present,
    denial_receipt_collection_id:$collection_id,
    denial_receipt_collection_route:$collection_route,
    denial_receipt_schema_version:$denial_schema_version,
    denial_receipt_entry_count:($entries | length),
    denial_receipt_projected_count:$denial_receipt_projected_count,
    denial_receipt_digest_projected_count:$denial_receipt_digest_projected_count,
    denial_receipt_readback_route_projected_count:$denial_receipt_readback_route_projected_count,
    denial_receipt_idempotency_key_projected_count:$denial_receipt_idempotency_key_projected_count,
    denial_receipt_idempotency_key_unique_count:$denial_receipt_idempotency_key_unique_count,
    source_recording_boundary_attached_count:$source_recording_boundary_attached_count,
    source_terminal_closeout_attached_entry_count:$source_terminal_closeout_attached_entry_count,
    source_persistence_denial_attached_entry_count:$source_persistence_denial_attached_entry_count,
    source_packet_persistence_denial_receipt_attached_entry_count:$source_packet_persistence_denial_receipt_attached_entry_count,
    source_non_send_readback_attached_entry_count:$source_non_send_readback_attached_entry_count,
    source_authority_packet_attached_entry_count:$source_authority_packet_attached_entry_count,
    source_authority_decision_request_attached_entry_count:$source_authority_decision_request_attached_entry_count,
    source_non_authority_receipt_attached_entry_count:$source_non_authority_receipt_attached_entry_count,
    source_authority_decision_record_id_attached_count:$source_authority_decision_record_id_attached_count,
    recording_denial_reason_projected_count:$recording_denial_reason_projected_count,
    recording_precondition_missing_count:$recording_precondition_missing_count,
    authority_decision_recording_disabled_count:$authority_decision_recording_disabled_count,
    authority_decision_recorded_count:$authority_decision_recorded_count,
    authority_decision_persisted_count:$authority_decision_persisted_count,
    denial_receipt_persisted_count:$denial_receipt_persisted_count,
    operator_packet_sent_count:$operator_packet_sent_count,
    operator_packet_persisted_count:$operator_packet_persisted_count,
    non_authority_receipt_persisted_count:$non_authority_receipt_persisted_count,
    local_evidence_acceptance_authority_present_count:$local_evidence_acceptance_authority_present_count,
    local_evidence_acceptance_allowed_count:$local_evidence_acceptance_allowed_count,
    local_evidence_acceptance_recorded_count:$local_evidence_acceptance_recorded_count,
    evidence_acceptance_recorded_count:$evidence_acceptance_recorded_count,
    evidence_recorded_count:$evidence_recorded_count,
    receipt_store_write_attempt_recorded_count:$receipt_store_write_attempt_recorded_count,
    receipt_store_written_count:$receipt_store_written_count,
    receipt_persisted_count:$receipt_persisted_count,
    ledger_written_count:$ledger_written_count,
    workflow_event_log_written_count:$workflow_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_mutation_allowed_count:$live_mutation_allowed_count,
    local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_ready:$ready,
    authority_decision_recording_allowed:false,
    authority_decision_recorded:false,
    authority_decision_persisted:false,
    denial_receipt_persistence_allowed:false,
    denial_receipt_persisted:false,
    local_evidence_acceptance_authority_allowed:false,
    non_authority_receipt_persistence_allowed:false,
    local_evidence_acceptance_allowed:false,
    local_evidence_acceptance_recording_allowed:false,
    evidence_acceptance_recording_allowed:false,
    evidence_recording_allowed:false,
    receipt_store_write_attempt_recording_allowed:false,
    receipt_store_write_allowed:false,
    receipt_persistence_allowed:false,
    ledger_write_allowed:false,
    workflow_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    credential_read_allowed:false,
    live_execution_allowed:false,
    blockers:[
      "local_evidence_acceptance_authority_missing",
      "authority_decision_recording_disabled",
      "authority_decision_persistence_disabled",
      "authority_decision_denial_receipt_persistence_disabled",
      "operator_packet_send_disabled",
      "operator_packet_persistence_disabled",
      "non_authority_receipt_persistence_disabled",
      "local_evidence_acceptance_disabled",
      "local_evidence_acceptance_recording_disabled",
      "evidence_acceptance_recording_disabled",
      "evidence_recording_disabled",
      "receipt_store_write_attempt_recording_disabled",
      "receipt_store_write_disabled",
      "receipt_persistence_disabled",
      "ledger_write_disabled",
      "workflow_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    entries:$entries,
    next_actions:["controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_retention_replay_readback_without_persistence"],
    recommended_next_gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_retention_replay_readback_without_persistence",
    side_effect_free:true,
    side_effects:{
      authority_decision_recorded:false,
      authority_decision_persisted:false,
      denial_receipt_persisted:false,
      operator_packet_sent:false,
      operator_packet_persisted:false,
      non_authority_receipt_persisted:false,
      local_evidence_acceptance_authority_accepted:false,
      local_evidence_acceptance_recorded:false,
      evidence_acceptance_recorded:false,
      evidence_recorded:false,
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
