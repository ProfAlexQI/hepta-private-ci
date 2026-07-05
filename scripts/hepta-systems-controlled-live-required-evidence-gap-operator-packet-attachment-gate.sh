#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-report.sh"
OPERATOR_PACKET_REPORT="$ROOT/scripts/hepta-systems-controlled-live-operator-packet-preview-report.sh"
OPERATOR_READBACK_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_PACKET_ATTACHMENT_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 5i report: $REPORT"
[[ -x "$OPERATOR_PACKET_REPORT" ]] || fail "missing executable Phase 5b operator packet preview report: $OPERATOR_PACKET_REPORT"
[[ -x "$OPERATOR_READBACK_REPORT" ]] || fail "missing executable Phase 5h operator readback report: $OPERATOR_READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 5i architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 5i operator packet attachment report"
fi

grep -q 'Controlled Live Required Evidence Gap Operator Packet Attachment' "$DOC" \
  || fail "architecture note must document Controlled Live Required Evidence Gap Operator Packet Attachment"
grep -q 'operator packet attachment without accepting evidence' "$DOC" \
  || fail "architecture note must document packet attachment without acceptance"
grep -q 'no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, packet persistence, attachment persistence, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed operator packet attachment boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_gap_operator_packet_attachment"
  and .status == "ready_blocked"
  and .gate == "controlled_live_required_evidence_gap_operator_packet_attachment_gate"
  and .schema_version == "controlled_live_required_evidence_gap_operator_packet_attachment_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_operator_packet_preview_ready == true
  and .source_packet_id == "controlled-live-operator-packet-preview"
  and .source_scope_id == "hepta-system-controlled-live-read-only-chain"
  and .source_payload_hash == "sha256:controlled-live-operator-packet-preview-no-live-payload"
  and .source_rollback_owner == "operator-explicit-before-live"
  and .source_operator_readback_ready == true
  and .source_operator_readback_entry_count == 7
  and .source_unchanged_missing_count == 7
  and .lib_export_present == true
  and .attachment_id == "controlled-live-required-evidence-gap-operator-packet-attachment"
  and .attachment_key == "controlled_live.required_evidence.gap.operator_packet_attachment"
  and .attached_packet_id == "controlled-live-operator-packet-preview"
  and .attached_payload_hash == "sha256:controlled-live-operator-packet-preview-no-live-payload"
  and .attachment_entry_count == 7
  and .operator_readback_attachment_count == 7
  and .unchanged_missing_attachment_count == 7
  and .attachment_route_count == 7
  and .evidence_recorded_count == 0
  and .operator_packet_attachment_ready == true
  and .approval_request_ready == false
  and .approval_request_sent == false
  and .approval_acceptance_ready == false
  and .approval_accepted == false
  and .approval_recorded == false
  and .packet_persisted == false
  and .attachment_persisted == false
  and .readback_persisted == false
  and .blocker_waived_count == 0
  and .credential_read_allowed == false
  and .evidence_recording_allowed == false
  and .evidence_persisted == false
  and .controlled_live_cutover_ready == false
  and .live_execution_allowed == false
  and (.entries | length) == 7
  and (.entries | all(.included_in_packet_attachment == true and .operator_visible == true and .queryable == true and .comparable == true and .packet_id == "controlled-live-operator-packet-preview" and .packet_payload_hash == "sha256:controlled-live-operator-packet-preview-no-live-payload" and .operator_status == "blocked_missing_evidence" and .previous_state == "missing" and .current_state == "missing" and .state_delta == "unchanged_missing" and (.attachment_key | length) > 0 and (.attachment_route | length) > 0 and (.operator_readback_key | length) > 0 and (.operator_readback_route | length) > 0 and .evidence_recorded == false and .approval_request_allowed == false and .approval_acceptance_allowed == false and .blocker_waiver_allowed == false and .credential_read_allowed == false and .evidence_recording_allowed == false and .persistence_allowed == false and .attachment_persistence_allowed == false and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .attachment_route == "attachment://controlled-live/operator-packet/required-evidence-gap/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .attachment_route == "attachment://controlled-live/operator-packet/required-evidence-gap/operator-live-approval-missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .attachment_route == "attachment://controlled-live/operator-packet/required-evidence-gap/fresh-soak-readback-missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing" and .attachment_route == "attachment://controlled-live/operator-packet/required-evidence-gap/credential-boundary-attestation-missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing" and .attachment_route == "attachment://controlled-live/operator-packet/required-evidence-gap/gateway-native-telegram-post-boundary-approval-missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing" and .attachment_route == "attachment://controlled-live/operator-packet/required-evidence-gap/rollback-rehearsal-missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing" and .attachment_route == "attachment://controlled-live/operator-packet/required-evidence-gap/kill-switch-rehearsal-missing")
  and (.next_actions | index("phase5j_controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_without_acceptance")) != null
  and .next_migration_step == "phase5j_controlled_live_required_evidence_gap_operator_packet_attachment_non_send_readback_without_acceptance"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$OPERATOR_PACKET_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_operator_packet_preview"
  and .status == "ready_blocked"
  and .operator_packet_preview_ready == true
  and .packet_id == "controlled-live-operator-packet-preview"
  and .approval_request_sent == false
  and .packet_persisted == false
  and .controlled_live_cutover_ready == false
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$OPERATOR_READBACK_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_gap_operator_readback"
  and .status == "ready_blocked"
  and .operator_readback_ready == true
  and .operator_readback_entry_count == 7
  and .unchanged_missing_count == 7
  and .evidence_recorded_count == 0
  and .approval_accepted == false
  and .readback_persisted == false
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_required_evidence_gap_operator_packet_attachment --lib
)

printf 'hepta-systems-controlled-live-required-evidence-gap-operator-packet-attachment-gate: PASS: operator readbacks are attached to the local packet preview without acceptance, sending, persistence, credentials, waivers, or live execution\n'
