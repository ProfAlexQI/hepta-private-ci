#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-decision-status-final-index-artifact-signing-terminal-public-claim-status-exposure-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-decision-status-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_DECISION_STATUS_FINAL_INDEX_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-decision-status-final-index-artifact-signing-terminal-public-claim-status-exposure-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable artifact signing terminal public claim/status exposure attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable artifact signing terminal decision/status final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing artifact signing terminal public claim/status exposure architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the artifact signing terminal public claim/status exposure attachment report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Terminal Public Claim/Status Exposure Attachment' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Terminal Public Claim/Status Exposure Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke artifact signing terminal public claim/status gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_final_index_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_final_index"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_final_index_attached == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_blocked == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_static_mention_count >= 30
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_denial_gate_invoked == false
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_denial_gate_invoked == false
  and .long_soak_started == false
  and .terminal_decision_recorded == false
  and .terminal_status_recorded == false
  and .status_promotion_recorded == false
  and .terminal_public_claim_status_exposure_accepted == false
  and .terminal_public_claim_status_exposure_recorded == false
  and .terminal_public_claim_status_exposure_persisted == false
  and .public_status_claimed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .release_status_exposed == false
  and .publication_status_exposed == false
  and .package_release_channel_status_exposed == false
  and .dashboard_status_exposed == false
  and .public_badge_exposed == false
  and .status_endpoint_exposed == false
  and .query_status_exposed == false
  and .export_status_exposed == false
  and .observability_status_exposed == false
  and .live_install_status_exposed == false
  and .channel_status_delivered == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .operator_approval_from_public_status_derived == false
  and .release_publication_authority_from_public_status_derived == false
  and .activation_authority_from_public_status_derived == false
  and .download_link_from_public_status_rendered == false
  and .install_command_from_public_status_rendered == false
  and .install_from_public_status_executed == false
  and .service_restart_from_public_status_performed == false
  and .active_binary_from_public_status_mutated == false
  and .attachment_blocker_count == 88
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_readback_without_status_promotion"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-decision-status-final-index-artifact-signing-terminal-public-claim-status-exposure-gate: PASS: artifact signing terminal public claim/status exposure attachment is ready but blocked without status promotion\n'
