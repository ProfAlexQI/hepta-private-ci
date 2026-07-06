#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-package-release-channel-status-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-package-release-channel-status-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_PACKAGE_RELEASE_CHANNEL_STATUS_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-package-release-channel-status-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent evidence package/release channel status final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent evidence package/release channel status readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent evidence package/release channel status final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent evidence package/release channel status final index report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Package/Release Channel Status Exposure Final Index' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Package/Release Channel Status Exposure Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke package/release channel gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_attached == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_attached == true
  and .operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate_invoked == false
  and .operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_gate_invoked == false
  and .long_soak_started == false
  and .package_release_channel_status_exposure_accepted == false
  and .package_release_channel_status_exposure_recorded == false
  and .package_release_channel_status_exposure_persisted == false
  and .package_channel_status_exposed == false
  and .release_channel_status_exposed == false
  and .update_feed_status_exposed == false
  and .package_registry_status_exposed == false
  and .cdn_status_exposed == false
  and .sbom_status_exposed == false
  and .signature_status_exposed == false
  and .notarization_status_exposed == false
  and .version_tag_status_exposed == false
  and .dashboard_status_exposed == false
  and .public_endpoint_status_exposed == false
  and .query_status_exposed == false
  and .export_status_exposed == false
  and .observability_status_exposed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .operator_approval_from_package_status_derived == false
  and .release_publication_authority_from_package_status_derived == false
  and .activation_authority_from_package_status_derived == false
  and .install_from_package_status_executed == false
  and .service_restart_from_package_status_performed == false
  and .active_binary_from_package_status_mutated == false
  and .final_blocker_count == 62
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_without_package_channel"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-package-release-channel-status-final-index-gate: PASS: Public GA operator identity/session intent consent evidence package/release channel status final index is ready but blocked\n'
