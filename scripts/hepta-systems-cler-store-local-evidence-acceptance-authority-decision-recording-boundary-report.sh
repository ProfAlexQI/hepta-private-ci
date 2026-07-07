#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-persistence-denial-terminal-no-persistence-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_DECISION_RECORDING_BOUNDARY_2026-07-07.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-decision-recording-boundary-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable local authority packet persistence-denial terminal source report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing local authority decision recording boundary Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

source_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_TERMINAL_NO_PERSISTENCE_JSON:-}"
source_cache_input_present=false
source_report_render_count=0
if [[ -n "$source_json" ]]; then
  [[ -f "$source_json" ]] || fail "missing cached local authority packet persistence-denial terminal source report: $source_json"
  source_cache_input_present=true
else
  source_json="$tmpdir/local-authority-packet-persistence-denial-terminal-no-persistence.json"
  "$SOURCE_REPORT" >"$source_json" || fail "failed to render local authority packet persistence-denial terminal source report"
  source_report_render_count=1
fi
jq -e . "$source_json" >/dev/null || fail "local authority packet persistence-denial terminal source report did not render valid JSON"

jq -n \
  --slurpfile source "$source_json" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson source_cache_input_present "$source_cache_input_present" \
  --argjson source_report_render_count "$source_report_render_count" \
  --arg boundary_id "controlled-live-evidence-receipt-store-local-evidence-acceptance-authority-decision-recording-boundary" \
  --arg boundary_route "readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-boundary" \
  --arg record_schema_version "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_record_v1" \
  '
  def hyphen_id($id): $id | gsub("_"; "-");
  def count_true($entries; $key): $entries | map(select(.[$key] == true)) | length;
  ($source[0]) as $src |
  ($src.entries | map({
    id:("evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_without_recording_" + .source_blocker_id),
    source_blocker_id,
    source_terminal_no_persistence_entry_id:.id,
    source_terminal_closeout_id:.terminal_closeout_id,
    source_terminal_closeout_key:.terminal_closeout_key,
    source_terminal_closeout_route:.terminal_closeout_route,
    source_terminal_reason:.terminal_reason,
    source_terminal_state:.terminal_state,
    source_persistence_denial_id:.source_persistence_denial_id,
    source_persistence_denial_route:.source_persistence_denial_route,
    source_persistence_denial_reason:.source_persistence_denial_reason,
    source_packet_persistence_denial_receipt_id:.source_packet_persistence_denial_receipt_id,
    source_authority_packet_id:.source_authority_packet_id,
    source_authority_packet_route:.source_authority_packet_route,
    source_authority_packet_key:.source_authority_packet_key,
    source_packet_non_send_readback_id:.source_packet_non_send_readback_id,
    source_packet_non_send_readback_route:.source_packet_non_send_readback_route,
    source_authority_decision_request_id:.source_authority_decision_request_id,
    source_authority_decision_request_route:.source_authority_decision_request_route,
    source_non_authority_receipt_id:.source_non_authority_receipt_id,
    source_non_authority_receipt_route:.source_non_authority_receipt_route,
    recording_boundary_id:$boundary_id,
    recording_boundary_route:($boundary_route + "/" + hyphen_id(.source_blocker_id)),
    authority_decision_record_id:("local-evidence-acceptance-authority-decision-record:controlled-live-evidence-receipt-store:" + .source_blocker_id + ":not-recorded"),
    authority_decision_record_schema_version:$record_schema_version,
    authority_decision_idempotency_key:("controlled-live-evidence-receipt-store.local-evidence-acceptance-authority-decision-recording.idempotency." + .source_blocker_id),
    post_record_readback_route:($boundary_route + "/post-record/" + hyphen_id(.source_blocker_id)),
    rollback_anchor:("rollback-anchor://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-decision-recording-boundary/" + hyphen_id(.source_blocker_id)),
    denial_receipt_id:("local-evidence-acceptance-authority-decision-recording-denial-receipt:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    observed_state:"local_evidence_acceptance_authority_decision_recording_boundary_projected_without_recording",
    source_terminal_closeout_attached:.terminal_no_persistence_confirmed,
    source_persistence_denial_attached:.source_persistence_denial_attached,
    source_packet_persistence_denial_receipt_attached:.source_packet_persistence_denial_receipt_attached,
    source_non_send_readback_attached:.source_non_send_readback_attached,
    source_authority_packet_attached:.source_authority_packet_attached,
    source_authority_decision_request_attached:true,
    source_non_authority_receipt_attached:true,
    boundary_projected:true,
    boundary_ready:true,
    decision_record_schema_projected:true,
    local_evidence_acceptance_authority_required:true,
    local_evidence_acceptance_authority_present:false,
    recording_precondition_missing:true,
    authority_decision_recording_allowed:false,
    authority_decision_recorded:false,
    authority_decision_persisted:false,
    decision_idempotency_key_projected:true,
    post_record_readback_route_projected:true,
    rollback_anchor_projected:true,
    denial_receipt_projected:true,
    denial_receipt_persisted:false,
    operator_packet_sent:false,
    operator_packet_persisted:false,
    non_authority_receipt_persisted:false,
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
  (count_true($entries; "boundary_projected")) as $boundary_projected_count |
  (count_true($entries; "boundary_ready")) as $boundary_ready_count |
  (count_true($entries; "source_terminal_closeout_attached")) as $source_terminal_closeout_attached_count |
  (count_true($entries; "source_persistence_denial_attached")) as $source_persistence_denial_attached_count |
  (count_true($entries; "source_packet_persistence_denial_receipt_attached")) as $source_packet_persistence_denial_receipt_attached_count |
  (count_true($entries; "source_non_send_readback_attached")) as $source_non_send_readback_attached_count |
  (count_true($entries; "source_authority_packet_attached")) as $source_authority_packet_attached_count |
  (count_true($entries; "source_authority_decision_request_attached")) as $source_authority_decision_request_attached_count |
  (count_true($entries; "source_non_authority_receipt_attached")) as $source_non_authority_receipt_attached_count |
  (count_true($entries; "decision_record_schema_projected")) as $decision_record_schema_projected_count |
  (count_true($entries; "local_evidence_acceptance_authority_required")) as $local_evidence_acceptance_authority_required_count |
  (count_true($entries; "local_evidence_acceptance_authority_present")) as $local_evidence_acceptance_authority_present_count |
  (count_true($entries; "recording_precondition_missing")) as $recording_precondition_missing_count |
  (count_true($entries; "authority_decision_recording_allowed")) as $decision_recording_allowed_count |
  (count_true($entries; "authority_decision_recorded")) as $authority_decision_recorded_count |
  (count_true($entries; "authority_decision_persisted")) as $authority_decision_persisted_count |
  (count_true($entries; "decision_idempotency_key_projected")) as $decision_idempotency_key_projected_count |
  ($entries | map(.authority_decision_idempotency_key) | unique | length) as $decision_idempotency_key_unique_count |
  (count_true($entries; "post_record_readback_route_projected")) as $post_record_readback_route_projected_count |
  (count_true($entries; "rollback_anchor_projected")) as $rollback_anchor_projected_count |
  (count_true($entries; "denial_receipt_projected")) as $denial_receipt_projected_count |
  (count_true($entries; "denial_receipt_persisted")) as $denial_receipt_persisted_count |
  (count_true($entries; "operator_packet_sent")) as $operator_packet_sent_count |
  (count_true($entries; "operator_packet_persisted")) as $operator_packet_persisted_count |
  (count_true($entries; "non_authority_receipt_persisted")) as $non_authority_receipt_persisted_count |
  (count_true($entries; "local_evidence_acceptance_allowed")) as $local_evidence_acceptance_allowed_count |
  (count_true($entries; "local_evidence_acceptance_recorded")) as $local_evidence_acceptance_recorded_count |
  (count_true($entries; "evidence_acceptance_recorded")) as $evidence_acceptance_recorded_count |
  (count_true($entries; "evidence_recorded")) as $evidence_recorded_count |
  (count_true($entries; "receipt_store_write_attempt_recorded")) as $receipt_store_write_attempt_recorded_count |
  (count_true($entries; "receipt_store_written")) as $receipt_store_written_count |
  ($entries | map(select(.receipt_persisted == true or .denial_receipt_persisted == true)) | length) as $receipt_persisted_count |
  (count_true($entries; "ledger_written")) as $ledger_written_count |
  (count_true($entries; "workflow_event_log_written")) as $workflow_event_log_written_count |
  (count_true($entries; "sqlite_written")) as $sqlite_written_count |
  (count_true($entries; "live_mutation_allowed")) as $live_mutation_allowed_count |
  ($src.terminal_no_persistence_readback_ready == true
    and $src.terminal_entry_count == 7
    and $src.terminal_closeout_projected_count == 7
    and $src.terminal_no_persistence_confirmed_count == 7
    and $src.source_retention_replay_attached_count == 7
    and $src.source_persistence_denial_attached_count == 7
    and $src.source_packet_persistence_denial_receipt_attached_count == 7
    and $src.source_non_send_readback_attached_count == 7
    and $src.source_authority_packet_attached_count == 7
    and $src.terminal_closeout_recorded_count == 0
    and $src.terminal_closeout_persisted_count == 0
    and $src.terminal_closeout_accepted_count == 0
    and $src.terminal_closeout_authoritative_count == 0
    and $src.packet_persistence_attempt_recorded_count == 0
    and $src.packet_persistence_denial_receipt_persisted_count == 0
    and $src.operator_packet_sent_count == 0
    and $src.operator_packet_persisted_count == 0
    and $src.local_evidence_acceptance_authority_present_count == 0
    and $src.local_evidence_acceptance_allowed_count == 0
    and $src.local_evidence_acceptance_recorded_count == 0
    and $src.authority_decision_recorded_count == 0
    and $src.non_authority_receipt_persisted_count == 0
    and $src.evidence_acceptance_recorded_count == 0
    and $src.evidence_recorded_count == 0
    and $src.receipt_store_write_attempt_recorded_count == 0
    and $src.receipt_store_written_count == 0
    and $src.receipt_persisted_count == 0
    and $src.live_execution_allowed == false
    and $lib_export_present == true
    and ($entries | length) == 7
    and $boundary_projected_count == 7
    and $boundary_ready_count == 7
    and $source_terminal_closeout_attached_count == 7
    and $source_persistence_denial_attached_count == 7
    and $source_packet_persistence_denial_receipt_attached_count == 7
    and $source_non_send_readback_attached_count == 7
    and $source_authority_packet_attached_count == 7
    and $source_authority_decision_request_attached_count == 7
    and $source_non_authority_receipt_attached_count == 7
    and $decision_record_schema_projected_count == 7
    and $local_evidence_acceptance_authority_required_count == 7
    and $local_evidence_acceptance_authority_present_count == 0
    and $recording_precondition_missing_count == 7
    and $decision_recording_allowed_count == 0
    and $authority_decision_recorded_count == 0
    and $authority_decision_persisted_count == 0
    and $decision_idempotency_key_projected_count == 7
    and $decision_idempotency_key_unique_count == 7
    and $post_record_readback_route_projected_count == 7
    and $rollback_anchor_projected_count == 7
    and $denial_receipt_projected_count == 7
    and $denial_receipt_persisted_count == 0
    and $operator_packet_sent_count == 0
    and $operator_packet_persisted_count == 0
    and $non_authority_receipt_persisted_count == 0
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
    and $live_mutation_allowed_count == 0) as $ready |
  {
    runtime:"hepta",
    surface:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_gate",
    schema_version:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_boundary_readback_without_recording_v1",
    plugin_id:"hepta-system@hepta-local",
    source_terminal_no_persistence_readback_ready:$src.terminal_no_persistence_readback_ready,
    source_terminal_entry_count:$src.terminal_entry_count,
    source_terminal_closeout_projected_count:$src.terminal_closeout_projected_count,
    source_terminal_no_persistence_confirmed_count:$src.terminal_no_persistence_confirmed_count,
    source_retention_replay_attached_count:$src.source_retention_replay_attached_count,
    source_terminal_source_persistence_denial_attached_count:$src.source_persistence_denial_attached_count,
    source_terminal_source_packet_persistence_denial_receipt_attached_count:$src.source_packet_persistence_denial_receipt_attached_count,
    source_terminal_source_non_send_readback_attached_count:$src.source_non_send_readback_attached_count,
    source_terminal_source_authority_packet_attached_count:$src.source_authority_packet_attached_count,
    source_terminal_closeout_recorded_count:$src.terminal_closeout_recorded_count,
    source_terminal_closeout_persisted_count:$src.terminal_closeout_persisted_count,
    source_terminal_closeout_accepted_count:$src.terminal_closeout_accepted_count,
    source_terminal_closeout_authoritative_count:$src.terminal_closeout_authoritative_count,
    source_packet_persistence_attempt_recorded_count:$src.packet_persistence_attempt_recorded_count,
    source_packet_persistence_denial_receipt_persisted_count:$src.packet_persistence_denial_receipt_persisted_count,
    source_operator_packet_sent_count:$src.operator_packet_sent_count,
    source_operator_packet_persisted_count:$src.operator_packet_persisted_count,
    source_local_evidence_acceptance_authority_present_count:$src.local_evidence_acceptance_authority_present_count,
    source_local_evidence_acceptance_allowed_count:$src.local_evidence_acceptance_allowed_count,
    source_local_evidence_acceptance_recorded_count:$src.local_evidence_acceptance_recorded_count,
    source_authority_decision_recorded_count:$src.authority_decision_recorded_count,
    source_non_authority_receipt_persisted_count:$src.non_authority_receipt_persisted_count,
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
    recording_boundary_id:$boundary_id,
    recording_boundary_route:$boundary_route,
    authority_decision_record_schema_version:$record_schema_version,
    boundary_entry_count:($entries | length),
    boundary_projected_count:$boundary_projected_count,
    boundary_ready_count:$boundary_ready_count,
    source_terminal_closeout_attached_count:$source_terminal_closeout_attached_count,
    source_persistence_denial_attached_count:$source_persistence_denial_attached_count,
    source_packet_persistence_denial_receipt_attached_count:$source_packet_persistence_denial_receipt_attached_count,
    source_non_send_readback_attached_count:$source_non_send_readback_attached_count,
    source_authority_packet_attached_count:$source_authority_packet_attached_count,
    source_authority_decision_request_attached_count:$source_authority_decision_request_attached_count,
    source_non_authority_receipt_attached_count:$source_non_authority_receipt_attached_count,
    decision_record_schema_projected_count:$decision_record_schema_projected_count,
    local_evidence_acceptance_authority_required_count:$local_evidence_acceptance_authority_required_count,
    local_evidence_acceptance_authority_present_count:$local_evidence_acceptance_authority_present_count,
    recording_precondition_missing_count:$recording_precondition_missing_count,
    decision_recording_allowed_count:$decision_recording_allowed_count,
    authority_decision_recorded_count:$authority_decision_recorded_count,
    authority_decision_persisted_count:$authority_decision_persisted_count,
    decision_idempotency_key_projected_count:$decision_idempotency_key_projected_count,
    decision_idempotency_key_unique_count:$decision_idempotency_key_unique_count,
    post_record_readback_route_projected_count:$post_record_readback_route_projected_count,
    rollback_anchor_projected_count:$rollback_anchor_projected_count,
    denial_receipt_projected_count:$denial_receipt_projected_count,
    denial_receipt_persisted_count:$denial_receipt_persisted_count,
    operator_packet_sent_count:$operator_packet_sent_count,
    operator_packet_persisted_count:$operator_packet_persisted_count,
    non_authority_receipt_persisted_count:$non_authority_receipt_persisted_count,
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
    local_evidence_acceptance_authority_decision_recording_boundary_readback_ready:$ready,
    authority_decision_recording_allowed:false,
    authority_decision_recorded:false,
    authority_decision_persisted:false,
    denial_receipt_persistence_allowed:false,
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
    next_actions:["controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence"],
    recommended_next_gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_decision_recording_denial_receipt_readback_without_persistence",
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
