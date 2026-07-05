#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
OPERATOR_PACKET_REPORT="$ROOT/scripts/hepta-systems-controlled-live-operator-packet-preview-report.sh"
OPERATOR_READBACK_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/controlled_live_required_evidence_gap_operator_packet_attachment.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$OPERATOR_PACKET_REPORT" ]] || fail "missing executable Phase 5b operator packet preview report: $OPERATOR_PACKET_REPORT"
[[ -x "$OPERATOR_READBACK_REPORT" ]] || fail "missing executable Phase 5h operator readback report: $OPERATOR_READBACK_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 5i Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 5i architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 5i operator packet attachment report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

lib_export_present=false
if grep -q 'controlled_live_required_evidence_gap_operator_packet_attachment_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

packet_json="${HEPTA_CONTROLLED_LIVE_OPERATOR_PACKET_JSON:-}"
if [[ -n "$packet_json" ]]; then
  [[ -f "$packet_json" ]] || fail "missing cached Phase 5b operator packet preview report: $packet_json"
else
  packet_json="$tmpdir/packet.json"
  "$OPERATOR_PACKET_REPORT" >"$packet_json" || fail "failed to render Phase 5b operator packet preview report"
fi

readback_json="${HEPTA_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_JSON:-}"
if [[ -n "$readback_json" ]]; then
  [[ -f "$readback_json" ]] || fail "missing cached Phase 5h operator readback report: $readback_json"
else
  readback_json="$tmpdir/readback.json"
  "$OPERATOR_READBACK_REPORT" >"$readback_json" || fail "failed to render Phase 5h operator readback report"
fi

jq -n \
  --slurpfile packet "$packet_json" \
  --slurpfile readback "$readback_json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_2026-06-27.md" \
  '
  def attachment_key($id):
    "controlled_live.required_evidence.gap.operator_packet_attachment." + $id;
  def attachment_route($id):
    "attachment://controlled-live/operator-packet/required-evidence-gap/" + ($id | gsub("_"; "-"));
  ($packet[0]) as $packet |
  ($readback[0]) as $readback |
  ($readback.entries | map({
    id,
    source_blocker_id,
    packet_id:$packet.packet_id,
    packet_payload_hash:$packet.payload_hash,
    attachment_key:attachment_key(.source_blocker_id),
    attachment_route:attachment_route(.source_blocker_id),
    operator_readback_key,
    operator_readback_route,
    operator_display_order,
    operator_status,
    operator_action,
    previous_state,
    current_state,
    state_delta,
    owner,
    risk_bucket,
    operator_label,
    required_evidence,
    included_in_packet_attachment:true,
    operator_visible:true,
    queryable:true,
    comparable:true,
    evidence_recorded:false,
    approval_request_allowed:false,
    approval_acceptance_allowed:false,
    blocker_waiver_allowed:false,
    credential_read_allowed:false,
    evidence_recording_allowed:false,
    persistence_allowed:false,
    attachment_persistence_allowed:false,
    live_mutation_allowed:false
  })) as $entries |
  ($entries | map(select(.included_in_packet_attachment == true)) | length) as $operator_readback_attachment_count |
  ($entries | map(select(.previous_state == "missing" and .current_state == "missing" and .state_delta == "unchanged_missing")) | length) as $unchanged_missing_attachment_count |
  ($entries | map(select((.attachment_key | length) > 0 and (.attachment_route | length) > 0)) | length) as $attachment_route_count |
  ($entries | map(select(.evidence_recorded == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.blocker_waiver_allowed == true)) | length) as $blocker_waived_count |
  ($packet.operator_packet_preview_ready == true
    and $packet.approval_request_sent == false
    and $packet.packet_persisted == false
    and $readback.operator_readback_ready == true
    and $readback.operator_readback_entry_count == 7
    and $readback.unchanged_missing_count == 7
    and ($entries | length) == 7
    and $operator_readback_attachment_count == 7
    and $unchanged_missing_attachment_count == 7
    and $attachment_route_count == 7
    and $evidence_recorded_count == 0
    and $blocker_waived_count == 0
    and $lib_export_present == true
    and ($entries | all(.included_in_packet_attachment == true
      and .operator_visible == true
      and .queryable == true
      and .comparable == true
      and .packet_id == $packet.packet_id
      and .packet_payload_hash == $packet.payload_hash
      and .operator_status == "blocked_missing_evidence"
      and .previous_state == "missing"
      and .current_state == "missing"
      and .state_delta == "unchanged_missing"
      and .approval_request_allowed == false
      and .approval_acceptance_allowed == false
      and .evidence_recording_allowed == false
      and .credential_read_allowed == false
      and .persistence_allowed == false
      and .attachment_persistence_allowed == false
      and .live_mutation_allowed == false))) as $operator_packet_attachment_ready |
  {
    runtime:"hepta",
    surface:"controlled_live_required_evidence_gap_operator_packet_attachment",
    status:(if $operator_packet_attachment_ready then "ready_blocked" else "blocked" end),
    gate:"controlled_live_required_evidence_gap_operator_packet_attachment_gate",
    schema_version:"controlled_live_required_evidence_gap_operator_packet_attachment_v1",
    plugin_id:"hepta-system@hepta-local",
    source_operator_packet_preview_ready:$packet.operator_packet_preview_ready,
    source_packet_id:$packet.packet_id,
    source_scope_id:$packet.scope_id,
    source_payload_hash:$packet.payload_hash,
    source_rollback_owner:$packet.rollback_owner,
    source_operator_readback_ready:$readback.operator_readback_ready,
    source_operator_readback_entry_count:$readback.operator_readback_entry_count,
    source_unchanged_missing_count:$readback.unchanged_missing_count,
    lib_export_present:$lib_export_present,
    attachment_id:"controlled-live-required-evidence-gap-operator-packet-attachment",
    attachment_key:"controlled_live.required_evidence.gap.operator_packet_attachment",
    attached_packet_id:$packet.packet_id,
    attached_payload_hash:$packet.payload_hash,
    attachment_entry_count:($entries | length),
    operator_readback_attachment_count:$operator_readback_attachment_count,
    unchanged_missing_attachment_count:$unchanged_missing_attachment_count,
    attachment_route_count:$attachment_route_count,
    evidence_recorded_count:$evidence_recorded_count,
    operator_packet_attachment_ready:$operator_packet_attachment_ready,
    approval_request_ready:false,
    approval_request_sent:false,
    approval_acceptance_ready:false,
    approval_accepted:false,
    approval_recorded:false,
    packet_persisted:false,
    attachment_persisted:false,
    readback_persisted:false,
    blocker_waived_count:$blocker_waived_count,
    credential_read_allowed:false,
    evidence_recording_allowed:false,
    evidence_persisted:false,
    controlled_live_cutover_ready:false,
    live_execution_allowed:false,
    entries:$entries,
    next_actions:[
      "phase5j_controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_without_acceptance",
      "keep_operator_packet_attachment_unsent_unpersisted_without_acceptance"
    ],
    next_migration_step:"phase5j_controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_without_acceptance",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      approval_requested:false,
      approval_accepted:false,
      approval_recorded:false,
      evidence_recorded:false,
      evidence_persisted:false,
      blocker_waived:false,
      credential_read:false,
      packet_persisted:false,
      attachment_persisted:false,
      readback_persisted:false,
      ledger_written:false,
      workflow_event_log_written:false,
      sqlite_written:false,
      native_post_mutation_performed:false,
      gateway_or_auth_mutated:false,
      telegram_transport_mutated:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      replay_executed:false,
      rollback_executed:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }'
