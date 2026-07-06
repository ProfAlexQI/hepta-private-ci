#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-status-final-index-artifact-signing-terminal-public-claim-delivery-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable artifact signing terminal public claim delivery/readback readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable artifact signing terminal public claim delivery/readback attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing artifact signing terminal public claim delivery/readback readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the artifact signing terminal public claim delivery/readback readback report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Terminal Public Claim Delivery/Readback Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Terminal Public Claim Delivery/Readback Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke artifact signing terminal public claim delivery/readback gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_attachment_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_attachment"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_attachment_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_attachment_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_attachment_attached == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_final_index_attached == true
  and .readback_mode == "static_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_snapshot_only"
  and .readback_check_count == 90
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_denial_gate_invoked == false
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_denial_gate_invoked == false
  and .long_soak_started == false
  and .public_claim_delivery_recorded == false
  and .status_readback_recorded == false
  and .channel_delivery_recorded == false
  and .external_delivery_readback_sent == false
  and .telegram_delivery_readback_sent == false
  and .delivery_receipt_recorded == false
  and .readback_receipt_recorded == false
  and .operator_approval_from_delivery_readback_derived == false
  and .release_publication_authority_from_delivery_readback_derived == false
  and .activation_authority_from_delivery_readback_derived == false
  and .install_from_delivery_readback_executed == false
  and .service_restart_from_delivery_readback_performed == false
  and .active_binary_from_delivery_readback_mutated == false
  and .readback_blocker_count == 90
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_without_public_claim"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-readback-gate: PASS: artifact signing terminal public claim delivery/readback readback is ready but blocked\n'
