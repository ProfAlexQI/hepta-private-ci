#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-package-channel-final-index-delivery-receipt-distribution-manifest-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-package-channel-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_PACKAGE_CHANNEL_FINAL_INDEX_DELIVERY_RECEIPT_DISTRIBUTION_MANIFEST_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-distribution-manifest-attachment-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal public claim delivery receipt distribution artifact/manifest status attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable terminal public claim delivery receipt package/release/channel status final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing terminal public claim delivery receipt distribution artifact/manifest status attachment architecture note: $DOC"

grep -q 'Terminal Public Claim Delivery Receipt Distribution Artifact/Manifest Status Attachment' "$DOC" \
  || fail "architecture note must document Terminal Public Claim Delivery Receipt Distribution Artifact/Manifest Status Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke distribution artifact/manifest gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_distribution_artifact_manifest_status_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_package_release_channel_status_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_package_release_channel_status_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_distribution_artifact_manifest_status_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_distribution_artifact_manifest_status_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_distribution_artifact_manifest_status_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_distribution_artifact_manifest_status_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_distribution_manifest_static_mention_count >= 40
  and .artifact_signing_terminal_public_claim_delivery_receipt_distribution_artifact_manifest_status_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_package_release_channel_status_denial_gate_invoked == false
  and .distribution_artifact_manifest_status_recorded == false
  and .distribution_artifact_manifest_status_persisted == false
  and .distribution_artifact_status_exposed == false
  and .manifest_status_exposed == false
  and .package_manifest_materialized == false
  and .release_manifest_published == false
  and .query_status_exposed == false
  and .export_status_exposed == false
  and .observability_status_exposed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .operator_approval_from_manifest_status_derived == false
  and .release_publication_authority_from_manifest_status_derived == false
  and .activation_authority_from_manifest_status_derived == false
  and .install_from_manifest_status_executed == false
  and .active_binary_from_manifest_status_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .attachment_blocker_count == 116
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_distribution_artifact_manifest_status_readback_without_package_channel"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-distribution-manifest-attachment-gate: PASS: terminal public claim delivery receipt distribution artifact/manifest status attachment is ready but blocked\n'
