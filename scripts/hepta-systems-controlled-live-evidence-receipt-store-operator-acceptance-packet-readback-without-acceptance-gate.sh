#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-operator-acceptance-packet-readback-without-acceptance-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-persistence-open-preconditions-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_OPERATOR_ACCEPTANCE_PACKET_READBACK_WITHOUT_ACCEPTANCE_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-operator-acceptance-packet-readback-without-acceptance-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable receipt store operator acceptance packet report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable receipt store persistence open-preconditions report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the controlled-live evidence receipt store operator acceptance packet report"
fi

rg -q 'Controlled Live Evidence Receipt Store Operator Acceptance Packet Readback Without Acceptance' "$DOC" \
  || fail "architecture note must document Controlled Live Evidence Receipt Store Operator Acceptance Packet Readback Without Acceptance"
rg -q 'controlled live evidence receipt store operator acceptance packet readback without acceptance' "$DOC" \
  || fail "architecture note must document the operator acceptance packet readback without acceptance"
rg -q 'no operator packet send, operator packet persistence, approval request, approval acceptance, approval recording, acceptance recording, evidence recording, evidence persistence, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed operator acceptance packet boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_persistence_open_preconditions_ready == true
  and .source_precondition_entry_count == 7
  and .source_operator_approval_present_count == 0
  and .source_evidence_acceptance_present_count == 0
  and .source_persistence_open_allowed == false
  and .lib_export_present == true
  and .operator_acceptance_packet_id == "controlled-live-evidence-receipt-store-operator-acceptance-packet"
  and .operator_acceptance_packet_route == "operator-packet://controlled-live/evidence-receipt-store/acceptance"
  and .operator_acceptance_packet_payload_fingerprint == "sha256:controlled-live-evidence-receipt-store-operator-acceptance-packet-no-acceptance"
  and .packet_entry_count == 7
  and .packet_projected_count == 7
  and .packet_ready_count == 7
  and .checklist_projected_count == 7
  and .operator_acceptance_required_count == 7
  and .operator_acceptance_present_count == 0
  and .evidence_acceptance_required_count == 7
  and .evidence_acceptance_present_count == 0
  and .persistence_precondition_catalog_present_count == 7
  and .persistence_open_allowed_count == 0
  and .acceptance_decision_request_projected_count == 7
  and .acceptance_decision_recorded_count == 0
  and .non_acceptance_receipt_projected_count == 7
  and .non_acceptance_receipt_persisted_count == 0
  and .operator_packet_sent_count == 0
  and .operator_packet_persisted_count == 0
  and .evidence_recorded_count == 0
  and .receipt_persisted_count == 0
  and .operator_acceptance_packet_readback_ready == true
  and .operator_packet_send_allowed == false
  and .operator_packet_sent == false
  and .operator_packet_persistence_allowed == false
  and .operator_packet_persisted == false
  and .approval_request_allowed == false
  and .approval_request_sent == false
  and .approval_acceptance_allowed == false
  and .approval_accepted == false
  and .acceptance_recording_allowed == false
  and .acceptance_recorded == false
  and .evidence_recording_allowed == false
  and .evidence_persisted == false
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
  and (.blockers | index("operator_acceptance_missing")) != null
  and (.blockers | index("evidence_acceptance_missing")) != null
  and (.blockers | index("persistence_open_preconditions_missing")) != null
  and (.blockers | index("acceptance_recording_disabled")) != null
  and (.blockers | index("evidence_recording_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("workflow_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_operator_acceptance_packet_without_acceptance_"))
    and (.receipt_id | startswith("controlled-live-evidence-receipt-preflight:"))
    and (.receipt_path | startswith(".hepta/controlled-live/evidence-receipts/status-canary/"))
    and (.persistence_precondition_route | startswith("readback://controlled-live/evidence-receipt-store/persistence-open-preconditions/"))
    and .operator_acceptance_packet_id == "controlled-live-evidence-receipt-store-operator-acceptance-packet"
    and .operator_acceptance_packet_route == "operator-packet://controlled-live/evidence-receipt-store/acceptance"
    and (.acceptance_decision_request_id | startswith("acceptance-decision-request:controlled-live-evidence-receipt-store:"))
    and (.acceptance_decision_request_route | startswith("readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/"))
    and (.non_acceptance_receipt_id | startswith("non-acceptance-receipt:controlled-live-evidence-receipt-store:"))
    and (.operator_approval_id | startswith("operator-approval:controlled-live-evidence-receipt-store:"))
    and (.evidence_acceptance_key | startswith("controlled_live.evidence_acceptance.required."))
    and .operator_status == "blocked_missing_evidence"
    and .observed_state == "operator_acceptance_packet_projected_without_acceptance"
    and .previous_state == "missing"
    and .current_state == "missing"
    and .state_delta == "unchanged_missing"
    and .packet_projected == true
    and .packet_ready == true
    and .checklist_projected == true
    and .operator_acceptance_required == true
    and .operator_acceptance_present == false
    and .evidence_acceptance_required == true
    and .evidence_acceptance_present == false
    and .persistence_precondition_catalog_present == true
    and .persistence_open_allowed == false
    and .acceptance_decision_request_projected == true
    and .acceptance_decision_recorded == false
    and .non_acceptance_receipt_projected == true
    and .non_acceptance_receipt_persisted == false
    and .operator_packet_send_allowed == false
    and .operator_packet_sent == false
    and .operator_packet_persistence_allowed == false
    and .operator_packet_persisted == false
    and .approval_request_allowed == false
    and .approval_request_sent == false
    and .approval_acceptance_allowed == false
    and .approval_accepted == false
    and .acceptance_recording_allowed == false
    and .acceptance_recorded == false
    and .evidence_recording_allowed == false
    and .evidence_recorded == false
    and .receipt_persistence_allowed == false
    and .receipt_persisted == false
    and .receipt_store_write_allowed == false
    and .receipt_store_written == false
    and .ledger_write_allowed == false
    and .workflow_event_log_write_allowed == false
    and .sqlite_write_allowed == false
    and .credential_read_allowed == false
    and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .acceptance_decision_request_route == "readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .acceptance_decision_request_route == "readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/operator-live-approval-missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .acceptance_decision_request_route == "readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/fresh-soak-readback-missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing" and .acceptance_decision_request_route == "readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/credential-boundary-attestation-missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing" and .acceptance_decision_request_route == "readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/gateway-native-telegram-post-boundary-approval-missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing" and .acceptance_decision_request_route == "readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/rollback-rehearsal-missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing" and .acceptance_decision_request_route == "readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/kill-switch-rehearsal-missing")
  and (.next_actions | index("controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording")) != null
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_persistence_open_preconditions_readback"
  and .status == "ready_blocked"
  and .persistence_open_preconditions_readback_ready == true
  and .precondition_entry_count == 7
  and .precondition_catalog_ready_count == 7
  and .operator_approval_required_count == 7
  and .operator_approval_present_count == 0
  and .evidence_acceptance_required_count == 7
  and .evidence_acceptance_present_count == 0
  and .persistence_open_allowed == false
  and .receipt_store_written == false
  and .receipt_persisted == false
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance --lib
)

printf 'hepta-systems-controlled-live-evidence-receipt-store-operator-acceptance-packet-readback-without-acceptance-gate: PASS: operator acceptance packet is read back without send, acceptance, recording, persistence, or live execution\n'
