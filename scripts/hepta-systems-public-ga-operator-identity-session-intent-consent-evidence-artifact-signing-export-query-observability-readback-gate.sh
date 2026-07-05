#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-export-query-observability-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-retention-expiry-gc-final-index-artifact-signing-export-query-observability-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_EXPORT_QUERY_OBSERVABILITY_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-export-query-observability-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing export/query/observability readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing export/query/observability attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent evidence artifact signing export/query/observability readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent evidence artifact signing export/query/observability readback report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Export/Query/Observability Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Export/Query/Observability Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke artifact signing export/query/observability gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_attachment_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_attachment"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_attachment_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_attachment_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_readback_blocked == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_denial_gate_invoked == false
  and .query_registered == false
  and .query_executed == false
  and .export_file_written == false
  and .export_stream_opened == false
  and .observability_metric_recorded == false
  and .dashboard_panel_recorded == false
  and .operator_summary_recorded == false
  and .audit_view_recorded == false
  and .operator_approval_from_export_query_observability_derived == false
  and .release_publication_authority_from_export_query_observability_derived == false
  and .activation_authority_from_export_query_observability_derived == false
  and .install_from_export_query_observability_executed == false
  and .active_binary_from_export_query_observability_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .readback_check_count == 80
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_final_index_without_retention"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-export-query-observability-readback-gate: PASS: Public GA operator identity/session intent consent evidence artifact signing export/query/observability readback is ready but blocked\n'
