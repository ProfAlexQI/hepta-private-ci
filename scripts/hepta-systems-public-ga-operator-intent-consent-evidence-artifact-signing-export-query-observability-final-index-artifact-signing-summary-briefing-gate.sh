#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-export-query-observability-final-index-artifact-signing-summary-briefing-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-export-query-observability-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_EXPORT_QUERY_OBSERVABILITY_FINAL_INDEX_ARTIFACT_SIGNING_SUMMARY_BRIEFING_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-export-query-observability-final-index-artifact-signing-summary-briefing-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing summary/briefing attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing export/query/observability final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent evidence artifact signing summary/briefing attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent evidence artifact signing summary/briefing attachment report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Summary/Briefing Attachment' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Summary/Briefing Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke artifact signing summary/briefing gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_final_index_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_final_index"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_attachment_blocked == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_static_mention_count >= 40
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_denial_gate_invoked == false
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_denial_gate_invoked == false
  and .operator_summary_recorded == false
  and .operator_briefing_recorded == false
  and .signing_receipt_readback_recorded == false
  and .status_banner_recorded == false
  and .briefing_delivery_recorded == false
  and .external_briefing_delivered == false
  and .telegram_briefing_delivered == false
  and .operator_acceptance_from_summary_recorded == false
  and .operator_approval_from_summary_derived == false
  and .release_publication_authority_from_summary_briefing_derived == false
  and .activation_authority_from_summary_briefing_derived == false
  and .install_from_summary_briefing_executed == false
  and .active_binary_from_summary_briefing_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .attachment_blocker_count == 82
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback_without_export"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-export-query-observability-final-index-artifact-signing-summary-briefing-gate: PASS: Public GA operator identity/session intent consent evidence artifact signing summary/briefing attachment is ready but blocked without export/query/observability acceptance\n'
