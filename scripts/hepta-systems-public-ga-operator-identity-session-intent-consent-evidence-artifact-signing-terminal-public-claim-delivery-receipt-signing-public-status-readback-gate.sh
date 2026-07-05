#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-public-status-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-terminal-status-final-index-delivery-receipt-signing-public-status-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_PUBLIC_STATUS_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-public-status-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable signing receipt public status readback report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable signing receipt public status attachment gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing signing receipt public status readback architecture note: $DOC"

grep -q 'Artifact Signing Receipt Terminal Public Claim/Status Exposure Readback' "$DOC" \
  || fail "architecture note must document Artifact Signing Receipt Terminal Public Claim/Status Exposure Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke public status gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_attachment_attached == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_gate_invoked == false
  and .terminal_public_claim_status_exposure_recorded == false
  and .public_status_claimed == false
  and .public_ga_claimed == false
  and .telegram_status_sent == false
  and .release_publication_authority_from_public_status_derived == false
  and .activation_authority_from_public_status_derived == false
  and .install_from_public_status_executed == false
  and .active_binary_from_public_status_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .readback_blocker_count == 140
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_final_index_without_status_promotion"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-public-status-readback-gate: PASS: artifact signing receipt terminal public claim/status exposure readback is ready but blocked\n'
