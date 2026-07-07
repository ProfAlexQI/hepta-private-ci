#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_NON_SEND_2026-07-07.md"

fail() {
  printf 'hepta-systems-cler-store-local-evidence-acceptance-authority-packet-non-send-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable local evidence acceptance authority packet source report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing local evidence acceptance authority packet non-send Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"
command -v jq >/dev/null 2>&1 || fail "jq is required"

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

source_json="${HEPTA_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_JSON:-}"
source_cache_input_present=false
source_report_render_count=0
if [[ -n "$source_json" ]]; then
  [[ -f "$source_json" ]] || fail "missing cached local evidence acceptance authority packet report: $source_json"
  source_cache_input_present=true
else
  source_json="$tmpdir/local-evidence-acceptance-authority-packet.json"
  "$SOURCE_REPORT" >"$source_json" || fail "failed to render local evidence acceptance authority packet source report"
  source_report_render_count=1
fi
jq -e . "$source_json" >/dev/null || fail "local evidence acceptance authority packet source report did not render valid JSON"

jq -n \
  --slurpfile source "$source_json" \
  --argjson lib_export_present "$lib_export_present" \
  --argjson source_cache_input_present "$source_cache_input_present" \
  --argjson source_report_render_count "$source_report_render_count" \
  --arg gate "scripts/hepta-systems-cler-store-local-evidence-acceptance-authority-packet-non-send-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CLER_STORE_LOCAL_EVIDENCE_ACCEPTANCE_AUTHORITY_PACKET_NON_SEND_2026-07-07.md" \
  --arg denial_reason "operator_packet_send_disabled_local_evidence_acceptance_authority_missing_local_acceptance_disabled" \
  '
  def hyphen_id($id): $id | gsub("_"; "-");
  def count_true($entries; $key): $entries | map(select(.[$key] == true)) | length;
  ($source[0]) as $src |
  ($src.entries | map({
    id:("evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback_" + .source_blocker_id),
    source_blocker_id,
    source_authority_packet_id:.authority_packet_id,
    source_authority_packet_route:.authority_packet_route,
    source_authority_packet_key:.authority_packet_key,
    source_authority_decision_request_id:.authority_decision_request_id,
    source_authority_decision_request_route:.authority_decision_request_route,
    source_non_authority_receipt_id:.non_authority_receipt_id,
    source_non_authority_receipt_route:.non_authority_receipt_route,
    packet_non_send_readback_id:("local-evidence-acceptance-authority-packet-non-send:controlled-live-evidence-receipt-store:" + .source_blocker_id),
    packet_non_send_readback_route:("readback://controlled-live/evidence-receipt-store/local-evidence-acceptance/authority-packet/non-send/" + hyphen_id(.source_blocker_id)),
    packet_send_denial_reason:$denial_reason,
    observed_state:"local_evidence_acceptance_authority_packet_confirmed_unsent_without_acceptance",
    packet_projected:.packet_projected,
    packet_ready:.packet_ready,
    non_send_projected:true,
    packet_unsent:true,
    send_disabled:true,
    send_allowed:false,
    send_attempt_recorded:false,
    packet_persistence_disabled:true,
    operator_packet_sent:false,
    operator_packet_persisted:false,
    local_evidence_acceptance_authority_required:.local_acceptance_authority_required,
    local_evidence_acceptance_authority_present:.local_acceptance_authority_present,
    authority_decision_request_projected:.authority_decision_request_projected,
    authority_decision_recorded:false,
    non_authority_receipt_projected:.non_authority_receipt_projected,
    non_authority_receipt_persisted:false,
    local_evidence_acceptance_authority_allowed:false,
    local_evidence_acceptance_allowed:false,
    local_evidence_acceptance_recording_allowed:false,
    local_evidence_acceptance_recorded:false,
    evidence_acceptance_recording_allowed:false,
    evidence_acceptance_recorded:false,
    evidence_recording_allowed:false,
    evidence_recorded:false,
    receipt_store_write_attempt_recording_allowed:false,
    receipt_store_write_attempt_recorded:false,
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
  (count_true($entries; "non_send_projected")) as $non_send_projection_count |
  (count_true($entries; "packet_unsent")) as $unsent_packet_count |
  (count_true($entries; "send_disabled")) as $send_disabled_count |
  (count_true($entries; "send_allowed")) as $send_allowed_count |
  (count_true($entries; "send_attempt_recorded")) as $send_attempt_recorded_count |
  (count_true($entries; "packet_persistence_disabled")) as $packet_persistence_disabled_count |
  (count_true($entries; "operator_packet_sent")) as $operator_packet_sent_count |
  (count_true($entries; "operator_packet_persisted")) as $operator_packet_persisted_count |
  (count_true($entries; "local_evidence_acceptance_authority_present")) as $local_evidence_acceptance_authority_present_count |
  (count_true($entries; "local_evidence_acceptance_allowed")) as $local_evidence_acceptance_allowed_count |
  (count_true($entries; "local_evidence_acceptance_recorded")) as $local_evidence_acceptance_recorded_count |
  (count_true($entries; "authority_decision_recorded")) as $authority_decision_recorded_count |
  (count_true($entries; "non_authority_receipt_projected")) as $non_authority_receipt_projected_count |
  (count_true($entries; "non_authority_receipt_persisted")) as $non_authority_receipt_persisted_count |
  (count_true($entries; "evidence_acceptance_recorded")) as $evidence_acceptance_recorded_count |
  (count_true($entries; "evidence_recorded")) as $evidence_recorded_count |
  (count_true($entries; "receipt_store_write_attempt_recorded")) as $receipt_store_write_attempt_recorded_count |
  (count_true($entries; "receipt_store_written")) as $receipt_store_written_count |
  (count_true($entries; "receipt_persisted")) as $receipt_persisted_count |
  (count_true($entries; "ledger_written")) as $ledger_written_count |
  (count_true($entries; "workflow_event_log_written")) as $workflow_event_log_written_count |
  (count_true($entries; "sqlite_written")) as $sqlite_written_count |
  (count_true($entries; "live_mutation_allowed")) as $live_mutation_allowed_count |
  ($src.local_evidence_acceptance_authority_packet_readback_ready == true
    and $src.packet_entry_count == 7
    and $src.packet_ready_count == 7
    and $src.operator_packet_sent_count == 0
    and $src.operator_packet_persisted_count == 0
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
    and $non_send_projection_count == 7
    and $unsent_packet_count == 7
    and $send_disabled_count == 7
    and $send_allowed_count == 0
    and $send_attempt_recorded_count == 0
    and $packet_persistence_disabled_count == 7
    and $operator_packet_sent_count == 0
    and $operator_packet_persisted_count == 0
    and $local_evidence_acceptance_authority_present_count == 0
    and $local_evidence_acceptance_allowed_count == 0
    and $local_evidence_acceptance_recorded_count == 0
    and $authority_decision_recorded_count == 0
    and $non_authority_receipt_projected_count == 7
    and $non_authority_receipt_persisted_count == 0
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
    surface:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback_gate",
    schema_version:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_non_send_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    source_authority_packet_ready:$src.local_evidence_acceptance_authority_packet_readback_ready,
    source_packet_entry_count:$src.packet_entry_count,
    source_packet_ready_count:$src.packet_ready_count,
    source_operator_packet_sent_count:$src.operator_packet_sent_count,
    source_operator_packet_persisted_count:$src.operator_packet_persisted_count,
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
    non_send_entry_count:($entries | length),
    non_send_projection_count:$non_send_projection_count,
    unsent_packet_count:$unsent_packet_count,
    send_disabled_count:$send_disabled_count,
    send_allowed_count:$send_allowed_count,
    send_attempt_recorded_count:$send_attempt_recorded_count,
    packet_persistence_disabled_count:$packet_persistence_disabled_count,
    operator_packet_sent_count:$operator_packet_sent_count,
    operator_packet_persisted_count:$operator_packet_persisted_count,
    local_evidence_acceptance_authority_present_count:$local_evidence_acceptance_authority_present_count,
    local_evidence_acceptance_allowed_count:$local_evidence_acceptance_allowed_count,
    local_evidence_acceptance_recorded_count:$local_evidence_acceptance_recorded_count,
    authority_decision_recorded_count:$authority_decision_recorded_count,
    non_authority_receipt_projected_count:$non_authority_receipt_projected_count,
    non_authority_receipt_persisted_count:$non_authority_receipt_persisted_count,
    evidence_acceptance_recorded_count:$evidence_acceptance_recorded_count,
    evidence_recorded_count:$evidence_recorded_count,
    receipt_store_write_attempt_recorded_count:$receipt_store_write_attempt_recorded_count,
    receipt_store_written_count:$receipt_store_written_count,
    receipt_persisted_count:$receipt_persisted_count,
    ledger_written_count:$ledger_written_count,
    workflow_event_log_written_count:$workflow_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_mutation_allowed_count:$live_mutation_allowed_count,
    local_evidence_acceptance_authority_packet_non_send_readback_ready:$ready,
    operator_packet_send_allowed:false,
    operator_packet_sent:false,
    operator_packet_persistence_allowed:false,
    operator_packet_persisted:false,
    local_evidence_acceptance_authority_allowed:false,
    authority_decision_recording_allowed:false,
    non_authority_receipt_persistence_allowed:false,
    local_evidence_acceptance_allowed:false,
    local_evidence_acceptance_recording_allowed:false,
    evidence_acceptance_recording_allowed:false,
    evidence_recording_allowed:false,
    receipt_store_write_attempt_recording_allowed:false,
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
      "local_evidence_acceptance_authority_missing",
      "authority_decision_recording_disabled",
      "non_authority_receipt_persistence_disabled",
      "local_evidence_acceptance_disabled",
      "local_evidence_acceptance_recording_disabled",
      "evidence_acceptance_recording_disabled",
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
      "controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence",
      "keep_local_evidence_acceptance_authority_packet_unsent_unaccepted_unpersisted"
    ],
    recommended_next_gate:"controlled_live_evidence_receipt_store_local_evidence_acceptance_authority_packet_persistence_denial_readback_without_persistence",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      operator_packet_sent:false,
      operator_packet_persisted:false,
      local_evidence_acceptance_authority_accepted:false,
      authority_decision_recorded:false,
      non_authority_receipt_persisted:false,
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
  }
'
