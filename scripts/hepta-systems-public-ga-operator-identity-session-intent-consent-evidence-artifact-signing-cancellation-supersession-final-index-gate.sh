#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_CANCELLATION_SUPERSESSION_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession final index report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Cancellation/Supersession Final Index' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Cancellation/Supersession Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke artifact signing cancellation/supersession gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_attached == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_invoked == false
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_gate_invoked == false
  and .long_soak_started == false
  and .public_ga_claimed == false
  and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted == false
  and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded == false
  and .artifact_distribution_signing_notarization_receipt_cancellation_accepted == false
  and .artifact_distribution_signing_notarization_receipt_cancellation_recorded == false
  and .artifact_distribution_signing_notarization_receipt_supersession_accepted == false
  and .artifact_distribution_signing_notarization_receipt_supersession_recorded == false
  and .artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted == false
  and .artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded == false
  and .artifact_distribution_signing_notarization_receipt_tombstone_recorded == false
  and .artifact_distribution_signing_notarization_receipt_delete_marker_recorded == false
  and .artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted == false
  and .artifact_signing_receipt_cancellation_accepted == false
  and .package_signing_receipt_cancellation_accepted == false
  and .operator_approval_from_signing_receipt_cancellation_derived == false
  and .release_publication_authority_from_signing_receipt_cancellation_derived == false
  and .activation_authority_from_signing_receipt_supersession_derived == false
  and .install_from_signing_receipt_cancellation_executed == false
  and .service_restart_from_signing_receipt_supersession_performed == false
  and .active_binary_from_signing_receipt_cancellation_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .final_blocker_count == 74
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_without_cancellation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-gate: PASS: Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession final index is ready but blocked\n'
