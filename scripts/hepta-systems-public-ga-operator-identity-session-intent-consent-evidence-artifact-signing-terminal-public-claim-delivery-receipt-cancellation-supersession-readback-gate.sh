#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-final-index-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_CANCELLATION_SUPERSESSION_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable cancellation/supersession readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable cancellation/supersession attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing cancellation/supersession readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the cancellation/supersession readback report"
fi

grep -q 'Terminal Public Claim Delivery Receipt Cancellation/Supersession Readback' "$DOC" \
  || fail "architecture note must document terminal public claim delivery receipt cancellation/supersession readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke target gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_attachment_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_attachment_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate_invoked == false
  and .terminal_public_claim_delivery_receipt_cancellation_supersession_recorded == false
  and .terminal_public_claim_delivery_receipt_replacement_receipt_recorded == false
  and .terminal_public_claim_delivery_receipt_tombstone_recorded == false
  and .terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_recorded == false
  and .operator_approval_from_delivery_receipt_cancellation_supersession_derived == false
  and .release_publication_authority_from_delivery_receipt_cancellation_supersession_derived == false
  and .activation_authority_from_delivery_receipt_cancellation_supersession_derived == false
  and .install_from_delivery_receipt_cancellation_supersession_executed == false
  and .active_binary_from_delivery_receipt_cancellation_supersession_mutated == false
  and .readback_check_count == 98
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-readback-gate: PASS: artifact signing terminal public claim delivery receipt cancellation/supersession readback is ready but blocked\n'
