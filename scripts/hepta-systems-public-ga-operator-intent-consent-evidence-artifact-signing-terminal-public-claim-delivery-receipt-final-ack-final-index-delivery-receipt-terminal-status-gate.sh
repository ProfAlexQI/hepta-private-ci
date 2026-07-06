#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-final-ack-final-index-delivery-receipt-terminal-status-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-final-ack-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_FINAL_ACK_FINAL_INDEX_DELIVERY_RECEIPT_TERMINAL_STATUS_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-terminal-status-attachment-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal public claim delivery receipt terminal decision/status attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable terminal public claim delivery receipt final acknowledgement final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing terminal public claim delivery receipt terminal decision/status attachment architecture note: $DOC"

grep -q 'Terminal Public Claim Delivery Receipt Terminal Decision/Status Attachment' "$DOC" \
  || fail "architecture note must document Terminal Public Claim Delivery Receipt Terminal Decision/Status Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke terminal decision/status gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_blocked == true
  and .source_final_blocker_count == 108
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_static_mention_count >= 30
  and .artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_denial_gate_invoked == false
  and .terminal_decision_recorded == false
  and .terminal_status_recorded == false
  and .status_promotion_recorded == false
  and .public_status_exposed == false
  and .external_decision_sent == false
  and .telegram_decision_sent == false
  and .operator_approval_from_terminal_status_derived == false
  and .release_publication_authority_from_terminal_status_derived == false
  and .activation_authority_from_terminal_status_derived == false
  and .install_from_terminal_status_executed == false
  and .active_binary_from_terminal_status_mutated == false
  and .attachment_blocker_count == 110
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_readback_without_acknowledgement"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-terminal-status-attachment-gate: PASS: terminal public claim delivery receipt terminal decision/status attachment is ready but blocked\n'
