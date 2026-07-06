#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-nonp-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-notarization-final-index-delivery-receipt-signing-receipt-nonp-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_NONP_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-nonp-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable artifact signing receipt signing receipt non-persistence readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable artifact signing receipt signing receipt non-persistence attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing artifact signing receipt signing receipt non-persistence readback architecture note: $DOC"

grep -q 'Artifact Signing Receipt Signing Receipt Non-Persistence Readback' "$DOC" \
  || fail "architecture note must document Artifact Signing Receipt Signing Receipt Non-Persistence Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke signing receipt gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_non_persistence_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_non_persistence_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_non_persistence_readback_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_non_persistence_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_non_persistence_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_non_persistence_denial_gate_invoked == false
  and .signing_receipt_recorded == false
  and .signing_receipt_persisted == false
  and .external_receipt_sent == false
  and .telegram_receipt_sent == false
  and .release_publication_authority_from_signing_receipt_derived == false
  and .activation_authority_from_signing_receipt_derived == false
  and .provider_invoked == false
  and .credential_read == false
  and .readback_blocker_count == 148
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-nonp-readback-gate: PASS: artifact signing receipt signing receipt non-persistence readback is ready but blocked\n'
