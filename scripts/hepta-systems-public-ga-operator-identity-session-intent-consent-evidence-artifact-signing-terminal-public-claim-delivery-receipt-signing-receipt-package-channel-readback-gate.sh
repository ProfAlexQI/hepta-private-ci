#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-package-channel-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-public-status-final-index-delivery-receipt-signing-receipt-package-channel-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_PACKAGE_CHANNEL_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-package-channel-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable artifact signing receipt signing receipt package/release/channel status readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable artifact signing receipt signing receipt package/release/channel status attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing artifact signing receipt signing receipt package/release/channel status readback architecture note: $DOC"

grep -q 'Artifact Signing Receipt Package/Release/Channel Status Readback' "$DOC" \
  || fail "architecture note must document Artifact Signing Receipt Package/Release/Channel Status Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke package/release/channel gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_attachment_attached == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_denial_gate_invoked == false
  and .package_release_channel_status_recorded == false
  and .package_release_channel_status_persisted == false
  and .package_channel_status_exposed == false
  and .release_channel_status_exposed == false
  and .registry_channel_status_exposed == false
  and .update_feed_status_exposed == false
  and .dashboard_channel_status_exposed == false
  and .status_endpoint_channel_status_exposed == false
  and .query_status_exposed == false
  and .export_status_exposed == false
  and .observability_status_exposed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .operator_approval_from_package_channel_derived == false
  and .release_publication_authority_from_package_channel_derived == false
  and .activation_authority_from_package_channel_derived == false
  and .install_from_package_channel_executed == false
  and .active_binary_from_package_channel_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .readback_check_count == 170
  and .readback_blocker_count == 170
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_package_release_channel_status_final_index_without_public_status"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-package-channel-readback-gate: PASS: artifact signing receipt signing receipt package/release/channel status readback is ready but blocked\n'
