#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-distribution-manifest-final-index-delivery-receipt-signing-receipt-signing-notarization-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-distribution-manifest-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_DISTRIBUTION_MANIFEST_FINAL_INDEX_DELIVERY_RECEIPT_SIGNING_RECEIPT_SIGNING_NOTARIZATION_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-signing-notarization-attachment-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable artifact signing receipt signing receipt signing/notarization attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable artifact signing receipt signing receipt distribution/manifest final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing artifact signing receipt signing receipt signing/notarization attachment architecture note: $DOC"

grep -q 'Artifact Signing Receipt Artifact Signing/Notarization Surface Attachment' "$DOC" \
  || fail "architecture note must document Artifact Signing Receipt Artifact Signing/Notarization Surface Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke signing/notarization gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_distribution_artifact_manifest_status_final_index_blocked == true
  and .source_final_blocker_count == 172
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_notarization_static_mention_count >= 30
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_denial_gate_invoked == false
  and .artifact_signing_notarization_surface_recorded == false
  and .artifact_signing_notarization_surface_persisted == false
  and .artifact_signing_status_exposed == false
  and .package_signing_status_exposed == false
  and .signature_manifest_written == false
  and .checksum_binding_recorded == false
  and .notarization_submitted == false
  and .notarization_ticket_recorded == false
  and .stapling_executed == false
  and .installer_signed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .operator_approval_from_signing_status_derived == false
  and .release_publication_authority_from_signing_status_derived == false
  and .activation_authority_from_signing_status_derived == false
  and .install_from_signing_status_executed == false
  and .active_binary_from_signing_status_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .attachment_blocker_count == 174
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_artifact_signing_notarization_surface_readback_without_manifest_status"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-signing-notarization-attachment-gate: PASS: artifact signing receipt signing receipt signing/notarization attachment is ready but blocked\n'
