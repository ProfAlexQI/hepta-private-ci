#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-notarization-surface-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-distribution-artifact-manifest-status-final-index-artifact-signing-notarization-surface-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_NOTARIZATION_SURFACE_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-notarization-surface-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing/notarization readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing/notarization attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent evidence artifact signing/notarization readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent evidence artifact signing/notarization readback report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing/Notarization Surface Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing/Notarization Surface Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke artifact signing/notarization gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_attachment_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_attachment"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_attachment_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_attachment_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_attachment_attached == true
  and .readback_mode == "static_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_snapshot_only"
  and .readback_check_count == 66
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_denial_gate_invoked == false
  and .long_soak_started == false
  and .artifact_distribution_signing_notarization_surface_recorded == false
  and .artifact_signed == false
  and .package_signed == false
  and .signature_manifest_written == false
  and .notarization_submitted == false
  and .notarization_ticket_recorded == false
  and .stapling_executed == false
  and .installer_signed == false
  and .provenance_attestation_published == false
  and .sbom_manifest_published == false
  and .release_asset_packaged == false
  and .cdn_artifact_written == false
  and .package_registry_artifact_published == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .operator_approval_from_signing_status_derived == false
  and .release_publication_authority_from_signing_status_derived == false
  and .activation_authority_from_signing_status_derived == false
  and .install_from_signing_status_executed == false
  and .service_restart_from_signing_status_performed == false
  and .active_binary_from_signing_status_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .readback_blocker_count == 66
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_final_index_without_manifest_status"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-notarization-surface-readback-gate: PASS: Public GA operator identity/session intent consent evidence artifact signing/notarization readback is ready but blocked\n'
