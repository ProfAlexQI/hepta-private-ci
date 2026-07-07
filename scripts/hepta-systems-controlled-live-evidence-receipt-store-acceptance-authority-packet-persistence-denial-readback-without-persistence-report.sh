#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-non-send-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_READBACK_WITHOUT_PERSISTENCE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-persistence-denial-readback-without-persistence-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable acceptance authority packet non-send report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing acceptance authority packet persistence denial Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the acceptance authority packet persistence denial report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

source_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_NON_SEND_JSON:-}"
if [[ -n "$source_json" ]]; then
  [[ -f "$source_json" ]] || fail "missing cached acceptance authority packet non-send report: $source_json"
else
  source_json="$tmpdir/acceptance-authority-packet-non-send.json"
  "$SOURCE_REPORT" >"$source_json" || fail "failed to render acceptance authority packet non-send report"
fi
jq -e . "$source_json" >/dev/null || fail "acceptance authority packet non-send report did not render valid JSON"

jq -n \
  --slurpfile source "$source_json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-persistence-denial-readback-without-persistence-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_PERSISTENCE_DENIAL_READBACK_WITHOUT_PERSISTENCE_2026-07-07.md" \
  --arg denial_reason "operator_packet_persistence_disabled_acceptance_authority_missing_receipt_store_write_disabled" \
  '
  def hyphen_id($id):
    $id | gsub("_"; "-");
  ($source[0]) as $src |
  ($src.entries | map({
    id:("evidence_receipt_store_acceptance_authority_packet_persistence_denial_without_persistence_" + .source_blocker_id),
    source_blocker_id,
    source_packet_non_send_readback_id:.packet_non_send_readback_id,
    source_packet_non_send_readback_route:.packet_non_send_readback_route,
    source_acceptance_authority_packet_id:.source_acceptance_authority_packet_id,
    source_acceptance_authority_packet_route:.source_acceptance_authority_packet_route,
    packet_persistence_denial_id:("packet-persistence-denial:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    packet_persistence_denial_route:("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/persistence-denial/" + hyphen_id(.source_blocker_id)),
    packet_persistence_denial_reason:$denial_reason,
    operator_display_order,
    operator_status,
    observed_state:"acceptance_authority_packet_persistence_denied_without_persistence",
    previous_state,
    current_state,
    state_delta,
    owner,
    risk_bucket,
    operator_label,
    required_evidence,
    packet_projected,
    packet_ready,
    non_send_projected,
    packet_unsent,
    send_disabled,
    send_allowed,
    send_attempt_recorded,
    persistence_denial_projected:true,
    packet_persistence_denied:true,
    packet_persistence_disabled:true,
    packet_persistence_allowed:false,
    packet_persistence_attempt_recorded:false,
    operator_packet_sent:false,
    operator_packet_persisted:false,
    acceptance_authority_required,
    acceptance_authority_present,
    authority_decision_request_projected,
    authority_decision_recorded:false,
    non_authority_receipt_projected,
    non_authority_receipt_persisted:false,
    acceptance_allowed:false,
    evidence_recorded:false,
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
  ($entries | map(select(.persistence_denial_projected == true)) | length) as $persistence_denial_projected_count |
  ($entries | map(select(.packet_persistence_denied == true)) | length) as $packet_persistence_denied_count |
  ($entries | map(select(.packet_persistence_disabled == true)) | length) as $packet_persistence_disabled_count |
  ($entries | map(select(.packet_persistence_allowed == true)) | length) as $packet_persistence_allowed_count |
  ($entries | map(select(.packet_persistence_attempt_recorded == true)) | length) as $packet_persistence_attempt_recorded_count |
  ($entries | map(select(.operator_packet_sent == true)) | length) as $operator_packet_sent_count |
  ($entries | map(select(.operator_packet_persisted == true)) | length) as $operator_packet_persisted_count |
  ($entries | map(select(.non_send_projected == true)) | length) as $non_send_projection_count |
  ($entries | map(select(.send_attempt_recorded == true)) | length) as $send_attempt_recorded_count |
  ($entries | map(select(.acceptance_authority_present == true)) | length) as $acceptance_authority_present_count |
  ($entries | map(select(.acceptance_allowed == true)) | length) as $acceptance_allowed_count |
  ($entries | map(select(.authority_decision_recorded == true)) | length) as $authority_decision_recorded_count |
  ($entries | map(select(.non_authority_receipt_projected == true)) | length) as $non_authority_receipt_projected_count |
  ($entries | map(select(.non_authority_receipt_persisted == true)) | length) as $non_authority_receipt_persisted_count |
  ($entries | map(select(.evidence_recorded == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.receipt_store_written == true)) | length) as $receipt_store_written_count |
  ($entries | map(select(.receipt_persisted == true or .non_authority_receipt_persisted == true)) | length) as $receipt_persisted_count |
  ($entries | map(select(.ledger_written == true)) | length) as $ledger_written_count |
  ($entries | map(select(.workflow_event_log_written == true)) | length) as $workflow_event_log_written_count |
  ($entries | map(select(.sqlite_written == true)) | length) as $sqlite_written_count |
  ($entries | map(select(.live_mutation_allowed == true)) | length) as $live_mutation_allowed_count |
  ($src.acceptance_authority_packet_non_send_readback_ready == true
    and $src.non_send_entry_count == 7
    and $src.unsent_packet_count == 7
    and $src.send_disabled_count == 7
    and $src.send_allowed_count == 0
    and $src.operator_packet_sent_count == 0
    and $src.operator_packet_persisted_count == 0
    and $src.acceptance_allowed_count == 0
    and $src.receipt_store_written_count == 0
    and $src.live_execution_allowed == false
    and $lib_export_present == true
    and ($entries | length) == 7
    and $persistence_denial_projected_count == 7
    and $packet_persistence_denied_count == 7
    and $packet_persistence_disabled_count == 7
    and $packet_persistence_allowed_count == 0
    and $packet_persistence_attempt_recorded_count == 0
    and $operator_packet_sent_count == 0
    and $operator_packet_persisted_count == 0
    and $non_send_projection_count == 7
    and $send_attempt_recorded_count == 0
    and $acceptance_authority_present_count == 0
    and $acceptance_allowed_count == 0
    and $authority_decision_recorded_count == 0
    and $non_authority_receipt_projected_count == 7
    and $non_authority_receipt_persisted_count == 0
    and $evidence_recorded_count == 0
    and $receipt_store_written_count == 0
    and $receipt_persisted_count == 0
    and $ledger_written_count == 0
    and $workflow_event_log_written_count == 0
    and $sqlite_written_count == 0
    and $live_mutation_allowed_count == 0
    and ($entries | all(.observed_state == "acceptance_authority_packet_persistence_denied_without_persistence"
      and .previous_state == "missing"
      and .current_state == "missing"
      and .state_delta == "unchanged_missing"
      and .packet_projected == true
      and .packet_ready == true
      and .non_send_projected == true
      and .packet_unsent == true
      and .send_disabled == true
      and .send_allowed == false
      and .send_attempt_recorded == false
      and .persistence_denial_projected == true
      and .packet_persistence_denied == true
      and .packet_persistence_disabled == true
      and .packet_persistence_allowed == false
      and .packet_persistence_attempt_recorded == false
      and .operator_packet_sent == false
      and .operator_packet_persisted == false
      and .acceptance_authority_required == true
      and .acceptance_authority_present == false
      and .authority_decision_request_projected == true
      and .authority_decision_recorded == false
      and .non_authority_receipt_projected == true
      and .non_authority_receipt_persisted == false
      and .acceptance_allowed == false
      and .evidence_recorded == false
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
    surface:"controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_gate",
    schema_version:"controlled_live_evidence_receipt_store_acceptance_authority_packet_persistence_denial_readback_without_persistence_v1",
    plugin_id:"hepta-system@hepta-local",
    source_non_send_ready:$src.acceptance_authority_packet_non_send_readback_ready,
    source_non_send_entry_count:$src.non_send_entry_count,
    source_unsent_packet_count:$src.unsent_packet_count,
    source_send_disabled_count:$src.send_disabled_count,
    source_send_allowed_count:$src.send_allowed_count,
    source_operator_packet_sent_count:$src.operator_packet_sent_count,
    source_operator_packet_persisted_count:$src.operator_packet_persisted_count,
    source_acceptance_allowed_count:$src.acceptance_allowed_count,
    source_receipt_store_written_count:$src.receipt_store_written_count,
    source_live_execution_allowed:$src.live_execution_allowed,
    lib_export_present:$lib_export_present,
    persistence_denial_entry_count:($entries | length),
    persistence_denial_projected_count:$persistence_denial_projected_count,
    packet_persistence_denied_count:$packet_persistence_denied_count,
    packet_persistence_disabled_count:$packet_persistence_disabled_count,
    packet_persistence_allowed_count:$packet_persistence_allowed_count,
    packet_persistence_attempt_recorded_count:$packet_persistence_attempt_recorded_count,
    operator_packet_sent_count:$operator_packet_sent_count,
    operator_packet_persisted_count:$operator_packet_persisted_count,
    non_send_projection_count:$non_send_projection_count,
    send_attempt_recorded_count:$send_attempt_recorded_count,
    acceptance_authority_present_count:$acceptance_authority_present_count,
    acceptance_allowed_count:$acceptance_allowed_count,
    authority_decision_recorded_count:$authority_decision_recorded_count,
    non_authority_receipt_projected_count:$non_authority_receipt_projected_count,
    non_authority_receipt_persisted_count:$non_authority_receipt_persisted_count,
    evidence_recorded_count:$evidence_recorded_count,
    receipt_store_written_count:$receipt_store_written_count,
    receipt_persisted_count:$receipt_persisted_count,
    ledger_written_count:$ledger_written_count,
    workflow_event_log_written_count:$workflow_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_mutation_allowed_count:$live_mutation_allowed_count,
    acceptance_authority_packet_persistence_denial_readback_ready:$ready,
    operator_packet_send_allowed:false,
    operator_packet_sent:false,
    operator_packet_persistence_allowed:false,
    operator_packet_persisted:false,
    acceptance_authority_allowed:false,
    acceptance_recording_allowed:false,
    evidence_recording_allowed:false,
    receipt_persistence_allowed:false,
    receipt_store_write_allowed:false,
    receipt_store_written:false,
    ledger_write_allowed:false,
    workflow_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    credential_read_allowed:false,
    live_execution_allowed:false,
    blockers:[
      "operator_packet_send_disabled",
      "operator_packet_persistence_disabled",
      "acceptance_authority_missing",
      "operator_acceptance_missing",
      "evidence_acceptance_missing",
      "receipt_persistence_grant_missing",
      "receipt_store_write_disabled",
      "ledger_write_disabled",
      "workflow_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    entries:$entries,
    next_actions:[
      "controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_readback_without_write",
      "keep_acceptance_authority_packet_unpersisted_unaccepted_unsent"
    ],
    recommended_next_gate:"controlled_live_evidence_receipt_store_acceptance_authority_packet_receipt_store_write_denial_readback_without_write",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      operator_packet_sent:false,
      operator_packet_persisted:false,
      packet_persistence_attempt_recorded:false,
      acceptance_authority_accepted:false,
      acceptance_recorded:false,
      evidence_recorded:false,
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
