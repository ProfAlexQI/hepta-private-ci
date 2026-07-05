#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-terminal-status-final-index-delivery-receipt-signing-receipt-signing-receipt-public-status-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-terminal-status-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_SIGNING_RECEIPT_TERMINAL_STATUS_FINAL_INDEX_DELIVERY_RECEIPT_SIGNING_RECEIPT_SIGNING_RECEIPT_PUBLIC_STATUS_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-public-status-attachment-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable signing receipt public status attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable signing receipt signing receipt signing receipt terminal decision/status final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing signing receipt public status attachment architecture note: $DOC"

grep -q 'Artifact Signing Receipt Signing Receipt Signing Receipt Terminal Public Claim/Status Exposure Attachment' "$DOC" \
  || fail "architecture note must document Artifact Signing Receipt Signing Receipt Signing Receipt Terminal Public Claim/Status Exposure Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke public status gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_decision_status_promotion_final_index_blocked == true
  and .source_final_blocker_count == 278
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_static_mention_count >= 30
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_decision_status_promotion_denial_gate_invoked == false
  and .terminal_public_claim_status_exposure_recorded == false
  and .public_status_claimed == false
  and .public_ga_claimed == false
  and .public_release_claimed == false
  and .public_status_exposed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .release_publication_authority_from_public_status_derived == false
  and .activation_authority_from_public_status_derived == false
  and .install_from_public_status_executed == false
  and .active_binary_from_public_status_mutated == false
  and .attachment_blocker_count == 280
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_terminal_public_claim_status_exposure_readback_without_status_promotion"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-public-status-attachment-gate: PASS: artifact signing receipt signing receipt signing receipt terminal public claim/status exposure attachment is ready but blocked\n'
