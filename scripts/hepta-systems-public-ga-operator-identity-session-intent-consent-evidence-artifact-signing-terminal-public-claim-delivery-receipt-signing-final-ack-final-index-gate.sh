#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-final-ack-final-index-report.sh"

[[ -x "$REPORT" ]] || {
  echo "missing executable signing receipt final acknowledgement final index report: $REPORT" >&2
  exit 1
}

json="$("$REPORT")"

jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_final_index_blocked == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_readback_ready == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_summary_briefing_denial_gate_invoked == false
  and .signing_receipt_final_acknowledgement_recorded == false
  and .signing_receipt_operator_received_recorded == false
  and .signing_receipt_operator_read_recorded == false
  and .telegram_signing_receipt_acknowledgement_sent == false
  and .release_publication_authority_from_signing_receipt_acknowledgement_derived == false
  and .activation_authority_from_signing_receipt_acknowledgement_derived == false
  and .install_from_signing_receipt_acknowledgement_executed == false
  and .active_binary_from_signing_receipt_acknowledgement_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .final_blocker_count == 136
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .next_migration_step == "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_decision_status_without_acknowledgement"
  and (.side_effects | to_entries | all(.value == false))
' <<<"$json" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-final-ack-final-index-gate: PASS: artifact signing receipt final acknowledgement final index is ready but blocked\n'
