#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-acceptance-decision-recording-boundary-readback-without-recording-report.sh"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-controlled-live-evidence-receipt-store-operator-acceptance-packet-readback-without-acceptance-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_EVIDENCE_RECEIPT_STORE_ACCEPTANCE_DECISION_RECORDING_BOUNDARY_READBACK_WITHOUT_RECORDING_2026-07-07.md"

fail() {
  printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-decision-recording-boundary-readback-without-recording-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable receipt store acceptance decision recording boundary report: $REPORT"
[[ -x "$SOURCE_REPORT" ]] || fail "missing executable operator acceptance packet report: $SOURCE_REPORT"
[[ -f "$DOC" ]] || fail "missing architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the controlled-live evidence receipt store acceptance decision recording boundary report"
fi

rg -q 'Controlled Live Evidence Receipt Store Acceptance Decision Recording Boundary Readback Without Recording' "$DOC" \
  || fail "architecture note must document Controlled Live Evidence Receipt Store Acceptance Decision Recording Boundary Readback Without Recording"
rg -q 'controlled live evidence receipt store acceptance decision recording boundary readback without recording' "$DOC" \
  || fail "architecture note must document acceptance decision recording boundary readback without recording"
rg -q 'no acceptance decision recording, acceptance decision persistence, denial receipt persistence, evidence recording, evidence persistence, receipt store write, receipt persistence, ledger write, event-log write, SQLite write, credential read, operator packet send, operator packet persistence, approval request, approval acceptance, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, provider call, model call, replay, rollback, kill-switch rehearsal execution, kill-switch mutation, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed acceptance decision recording boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording"
  and .status == "ready_blocked"
  and .gate == "controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_gate"
  and .schema_version == "controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_operator_acceptance_packet_ready == true
  and .source_packet_entry_count == 7
  and .source_operator_acceptance_present_count == 0
  and .source_evidence_acceptance_present_count == 0
  and .source_acceptance_decision_recorded_count == 0
  and .source_operator_packet_sent == false
  and .source_operator_packet_persisted == false
  and .source_live_execution_allowed == false
  and .lib_export_present == true
  and .recording_boundary_id == "controlled-live-evidence-receipt-store-acceptance-decision-recording-boundary"
  and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary"
  and .acceptance_decision_record_schema_version == "controlled_live_evidence_receipt_store_acceptance_decision_record_v1"
  and .boundary_entry_count == 7
  and .boundary_projected_count == 7
  and .boundary_ready_count == 7
  and .decision_record_schema_projected_count == 7
  and .acceptance_decision_request_attached_count == 7
  and .operator_acceptance_required_count == 7
  and .operator_acceptance_present_count == 0
  and .evidence_acceptance_required_count == 7
  and .evidence_acceptance_present_count == 0
  and .recording_precondition_missing_count == 7
  and .decision_recording_allowed_count == 0
  and .acceptance_decision_recorded_count == 0
  and .acceptance_decision_persisted_count == 0
  and .decision_idempotency_key_projected_count == 7
  and .decision_idempotency_key_unique_count == 7
  and .post_record_readback_route_projected_count == 7
  and .rollback_anchor_projected_count == 7
  and .denial_receipt_projected_count == 7
  and .denial_receipt_persisted_count == 0
  and .receipt_store_written_count == 0
  and .receipt_persisted_count == 0
  and .ledger_written_count == 0
  and .workflow_event_log_written_count == 0
  and .sqlite_written_count == 0
  and .live_mutation_allowed_count == 0
  and .acceptance_decision_recording_boundary_readback_ready == true
  and .acceptance_decision_recording_allowed == false
  and .acceptance_decision_recorded == false
  and .acceptance_decision_persisted == false
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
  and (.blockers | index("operator_acceptance_missing")) != null
  and (.blockers | index("evidence_acceptance_missing")) != null
  and (.blockers | index("acceptance_decision_recording_disabled")) != null
  and (.blockers | index("acceptance_decision_persistence_disabled")) != null
  and (.blockers | index("denial_receipt_persistence_disabled")) != null
  and (.blockers | index("evidence_recording_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("receipt_store_write_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("workflow_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.entries | length) == 7
  and (.entries | all(
    (.id | startswith("evidence_receipt_store_acceptance_decision_recording_boundary_without_recording_"))
    and (.source_packet_entry_id | startswith("evidence_receipt_store_operator_acceptance_packet_without_acceptance_"))
    and (.source_acceptance_decision_request_id | startswith("acceptance-decision-request:controlled-live-evidence-receipt-store:"))
    and (.source_acceptance_decision_request_route | startswith("readback://controlled-live/evidence-receipt-store/operator-acceptance-packet/decision-request/"))
    and (.source_non_acceptance_receipt_id | startswith("non-acceptance-receipt:controlled-live-evidence-receipt-store:"))
    and .source_operator_acceptance_packet_id == "controlled-live-evidence-receipt-store-operator-acceptance-packet"
    and .source_operator_acceptance_packet_route == "operator-packet://controlled-live/evidence-receipt-store/acceptance"
    and (.receipt_id | startswith("controlled-live-evidence-receipt-preflight:"))
    and (.receipt_path | startswith(".hepta/controlled-live/evidence-receipts/status-canary/"))
    and .recording_boundary_id == "controlled-live-evidence-receipt-store-acceptance-decision-recording-boundary"
    and (.recording_boundary_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/"))
    and (.acceptance_decision_record_id | startswith("acceptance-decision-record:controlled-live-evidence-receipt-store:"))
    and .acceptance_decision_record_schema_version == "controlled_live_evidence_receipt_store_acceptance_decision_record_v1"
    and (.acceptance_decision_idempotency_key | startswith("controlled-live-evidence-receipt-store.acceptance-decision-recording.idempotency."))
    and (.post_record_readback_route | startswith("readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/post-record/"))
    and (.rollback_anchor | startswith("rollback-anchor://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/"))
    and (.denial_receipt_id | startswith("acceptance-decision-recording-denial-receipt:controlled-live-evidence-receipt-store:"))
    and .operator_status == "blocked_missing_evidence"
    and .observed_state == "acceptance_decision_recording_boundary_projected_without_recording"
    and .previous_state == "missing"
    and .current_state == "missing"
    and .state_delta == "unchanged_missing"
    and .boundary_projected == true
    and .boundary_ready == true
    and .source_packet_ready == true
    and .source_packet_sent == false
    and .source_packet_persisted == false
    and .decision_record_schema_projected == true
    and .acceptance_decision_request_attached == true
    and .operator_acceptance_required == true
    and .operator_acceptance_present == false
    and .evidence_acceptance_required == true
    and .evidence_acceptance_present == false
    and .recording_precondition_missing == true
    and .acceptance_decision_recording_allowed == false
    and .acceptance_decision_recorded == false
    and .acceptance_decision_persisted == false
    and .decision_idempotency_key_projected == true
    and .post_record_readback_route_projected == true
    and .rollback_anchor_projected == true
    and .denial_receipt_projected == true
    and .denial_receipt_persisted == false
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
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/operator-live-approval-missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/fresh-soak-readback-missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing" and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/credential-boundary-attestation-missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing" and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/gateway-native-telegram-post-boundary-approval-missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing" and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/rollback-rehearsal-missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing" and .recording_boundary_route == "readback://controlled-live/evidence-receipt-store/acceptance-decision-recording-boundary/kill-switch-rehearsal-missing")
  and (.next_actions | index("controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence")) != null
  and .recommended_next_gate == "controlled_live_evidence_receipt_store_recording_denial_receipt_readback_without_persistence"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_evidence_receipt_store_operator_acceptance_packet_readback_without_acceptance"
  and .status == "ready_blocked"
  and .operator_acceptance_packet_readback_ready == true
  and .packet_entry_count == 7
  and .operator_acceptance_present_count == 0
  and .evidence_acceptance_present_count == 0
  and .acceptance_decision_recorded_count == 0
  and .operator_packet_sent == false
  and .operator_packet_persisted == false
  and .receipt_persisted_count == 0
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_evidence_receipt_store_acceptance_decision_recording_boundary_readback_without_recording --lib
)

printf 'hepta-systems-controlled-live-evidence-receipt-store-acceptance-decision-recording-boundary-readback-without-recording-gate: PASS: acceptance decision recording boundary is read back without recording, persistence, or live execution\n'
