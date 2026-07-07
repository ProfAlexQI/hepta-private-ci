#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-readback-without-acceptance-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-positive-acceptance-preconditions-readback-without-acceptance-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_AUTHORITY_PACKET_READBACK_WITHOUT_ACCEPTANCE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-readback-without-acceptance-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable acceptance authority packet report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable positive acceptance preconditions report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the acceptance authority packet report"
fi

rg -q 'Controlled Live Evidence Receipt Store Acceptance Authority Packet Readback Without Acceptance' "$DOC" \
  || fail "architecture note must document Controlled Live Evidence Receipt Store Acceptance Authority Packet Readback Without Acceptance"
rg -q 'controlled live evidence receipt store acceptance authority packet readback without acceptance' "$DOC" \
  || fail "architecture note must document acceptance authority packet readback without acceptance"
rg -q 'no operator packet send, operator packet persistence, acceptance authority acceptance, acceptance recording, evidence recording, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay execution, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed acceptance authority packet boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_positive_preconditions_ready == true
  and .source_precondition_entry_count == 7
  and .source_acceptance_allowed_count == 0
  and .source_operator_acceptance_present_count == 0
  and .source_evidence_acceptance_present_count == 0
  and .source_receipt_store_written_count == 0
  and .source_live_execution_allowed == false
  and .lib_export_present == true
  and .acceptance_authority_packet_id == "controlled-live-evidence-receipt-store-acceptance-authority-packet"
  and .acceptance_authority_packet_route == "operator-packet://controlled-live/evidence-receipt-store/acceptance-authority"
  and .acceptance_authority_packet_payload_fingerprint == "sha256:controlled-live-evidence-receipt-store-acceptance-authority-packet-no-acceptance"
  and .packet_entry_count == 7
  and .packet_projected_count == 7
  and .packet_ready_count == 7
  and .authority_checklist_projected_count == 7
  and .authority_item_required_count == 56
  and .authority_item_present_count == 0
  and .acceptance_authority_required_count == 7
  and .acceptance_authority_present_count == 0
  and .authority_decision_request_projected_count == 7
  and .authority_decision_recorded_count == 0
  and .non_authority_receipt_projected_count == 7
  and .non_authority_receipt_persisted_count == 0
  and .operator_packet_sent_count == 0
  and .operator_packet_persisted_count == 0
  and .acceptance_allowed_count == 0
  and .evidence_recorded_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .acceptance_authority_packet_readback_ready == true
  and .operator_packet_send_allowed == false
  and .operator_packet_sent == false
  and .operator_packet_persistence_allowed == false
  and .operator_packet_persisted == false
  and .acceptance_authority_allowed == false
  and .acceptance_recording_allowed == false
  and .evidence_recording_allowed == false
  and .receipt_persistence_allowed == false
  and .receipt_store_write_allowed == false
  and .receipt_store_written == false
  and .ledger_write_allowed == false
  and .workflow_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .credential_read_allowed == false
  and .live_execution_allowed == false
  and (.blockers | index("operator_packet_send_disabled")) != null
  and (.blockers | index("operator_packet_persistence_disabled")) != null
  and (.blockers | index("acceptance_authority_missing")) != null
  and (.blockers | index("operator_acceptance_missing")) != null
  and (.blockers | index("evidence_acceptance_missing")) != null
  and (.blockers | index("receipt_persistence_grant_missing")) != null
  and (.blockers | index("atomic_append_not_enabled")) != null
  and (.blockers | index("post_write_readback_missing")) != null
  and (.blockers | index("rollback_rehearsal_missing")) != null
  and (.blockers | index("retention_policy_not_committed")) != null
  and (.blockers | index("live_cutover_approval_missing")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("workflow_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_acceptance_authority_packet_without_acceptance_"))
    and (.source_positive_precondition_set_id | startswith("positive-acceptance-preconditions:controlled-live-evidence-receipt-store:"))
    and (.source_positive_precondition_route | startswith("readback://controlled-live/evidence-receipt-store/positive-acceptance-preconditions/"))
    and .acceptance_authority_packet_id == "controlled-live-evidence-receipt-store-acceptance-authority-packet"
    and .acceptance_authority_packet_route == "operator-packet://controlled-live/evidence-receipt-store/acceptance-authority"
    and (.authority_decision_request_id | startswith("acceptance-authority-decision-request:controlled-live-evidence-receipt-store:"))
    and (.authority_decision_request_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/decision-request/"))
    and (.non_authority_receipt_id | startswith("non-authority-receipt:controlled-live-evidence-receipt-store:"))
    and .operator_status == "blocked_missing_evidence"
    and .observed_state == "acceptance_authority_packet_projected_without_acceptance"
    and .previous_state == "missing"
    and .current_state == "missing"
    and .state_delta == "unchanged_missing"
    and .packet_projected == true
    and .packet_ready == true
    and .authority_checklist_projected == true
    and .authority_item_required_count == 8
    and .authority_item_present_count == 0
    and .acceptance_authority_required == true
    and .acceptance_authority_present == false
    and .operator_acceptance_required == true
    and .operator_acceptance_present == false
    and .evidence_acceptance_required == true
    and .evidence_acceptance_present == false
    and .receipt_persistence_grant_required == true
    and .receipt_persistence_grant_present == false
    and .atomic_append_required == true
    and .atomic_append_enabled == false
    and .post_write_readback_required == true
    and .post_write_readback_persisted == false
    and .rollback_rehearsal_required == true
    and .rollback_rehearsal_verified == false
    and .retention_policy_commit_required == true
    and .retention_policy_committed == false
    and .live_cutover_approval_required == true
    and .live_cutover_approval_present == false
    and .authority_decision_request_projected == true
    and .authority_decision_recorded == false
    and .non_authority_receipt_projected == true
    and .non_authority_receipt_persisted == false
    and .operator_packet_send_allowed == false
    and .operator_packet_sent == false
    and .operator_packet_persistence_allowed == false
    and .operator_packet_persisted == false
    and .acceptance_allowed == false
    and .acceptance_recording_allowed == false
    and .evidence_recording_allowed == false
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
    and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .authority_decision_request_route == "readback://controlled-live/evidence-receipt-store/acceptance-authority-packet/decision-request/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .non_authority_receipt_id == "non-authority-receipt:controlled-live-evidence-receipt-store:operator_live_approval_missing")
  and (.next_actions | index("controlled_live_evidence_receipt_store_acceptance_authority_packet_non_send_readback")) != null
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_acceptance_authority_packet_non_send_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_positive_acceptance_preconditions_readback_without_acceptance"
  and .status == "ready_blocked"
  and .positive_acceptance_preconditions_readback_ready == true
  and .precondition_entry_count == 7
  and .acceptance_allowed_count == 0
  and .operator_acceptance_present_count == 0
  and .evidence_acceptance_present_count == 0
  and .receipt_store_written_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_evidence_receipt_store_acceptance_authority_packet_readback_without_acceptance --lib
)

printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-authority-packet-readback-without-acceptance-gate: PASS: acceptance authority packet is read back without send, acceptance, persistence, or live execution\n'
