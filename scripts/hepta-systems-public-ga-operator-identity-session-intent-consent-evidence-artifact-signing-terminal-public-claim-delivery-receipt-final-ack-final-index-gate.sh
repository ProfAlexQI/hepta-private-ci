#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-final-ack-final-index-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-final-ack-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_FINAL_ACK_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-final-ack-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal public claim delivery receipt final acknowledgement final index report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable terminal public claim delivery receipt final acknowledgement readback gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing terminal public claim delivery receipt final acknowledgement final index architecture note: $DOC"

grep -q 'Terminal Public Claim Delivery Receipt Final Acknowledgement Final Index' "$DOC" \
  || fail "architecture note must document Terminal Public Claim Delivery Receipt Final Acknowledgement Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke final acknowledgement gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_denial_gate_invoked == false
  and .terminal_public_claim_delivery_receipt_final_acknowledgement_recorded == false
  and .operator_received_recorded == false
  and .operator_read_recorded == false
  and .external_acknowledgement_sent == false
  and .telegram_acknowledgement_sent == false
  and .release_publication_authority_from_acknowledgement_derived == false
  and .activation_authority_from_acknowledgement_derived == false
  and .install_from_acknowledgement_executed == false
  and .active_binary_from_acknowledgement_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .final_blocker_count == 108
  and .manual_operator_live_cutover_approval_required == true
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .next_migration_step == "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_without_acknowledgement"
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-final-ack-final-index-gate: PASS: terminal public claim delivery receipt final acknowledgement final index is ready but blocked\n'
