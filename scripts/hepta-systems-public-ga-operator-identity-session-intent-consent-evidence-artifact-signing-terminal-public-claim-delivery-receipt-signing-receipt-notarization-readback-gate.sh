#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-notarization-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-distribution-manifest-final-index-delivery-receipt-signing-receipt-notarization-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_NOTARIZATION_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-notarization-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable artifact signing receipt signing/notarization readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable artifact signing receipt signing/notarization attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing artifact signing receipt signing/notarization readback architecture note: $DOC"

grep -q 'Artifact Signing Receipt Artifact Signing/Notarization Surface Readback' "$DOC" \
  || fail "architecture note must document Artifact Signing Receipt Artifact Signing/Notarization Surface Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke signing/notarization gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_artifact_signing_notarization_surface_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_artifact_signing_notarization_surface_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_artifact_signing_notarization_surface_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_artifact_signing_notarization_surface_attachment_attached == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_artifact_signing_notarization_surface_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_artifact_signing_notarization_surface_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_artifact_signing_notarization_surface_denial_gate_invoked == false
  and .artifact_signing_notarization_surface_recorded == false
  and .artifact_signing_status_exposed == false
  and .notarization_submitted == false
  and .notarization_ticket_recorded == false
  and .stapling_executed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .release_publication_authority_from_signing_status_derived == false
  and .activation_authority_from_signing_status_derived == false
  and .install_from_signing_status_executed == false
  and .active_binary_from_signing_status_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .readback_blocker_count == 146
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_artifact_signing_notarization_surface_final_index_without_manifest_status"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-notarization-readback-gate: PASS: artifact signing receipt signing/notarization readback is ready but blocked\n'
