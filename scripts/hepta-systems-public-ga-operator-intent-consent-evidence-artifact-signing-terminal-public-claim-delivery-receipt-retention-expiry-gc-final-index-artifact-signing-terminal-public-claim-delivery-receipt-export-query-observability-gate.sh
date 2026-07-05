#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-final-index-artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_RETENTION_EXPIRY_GC_FINAL_INDEX_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_EXPORT_QUERY_OBSERVABILITY_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-final-index-artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal public claim delivery receipt export/query/observability attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable terminal public claim delivery receipt retention/expiry/GC final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing terminal public claim delivery receipt export/query/observability attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the terminal public claim delivery receipt export/query/observability attachment report"
fi

grep -q 'Terminal Public Claim Delivery Receipt Export/Query/Observability Attachment' "$DOC" \
  || fail "architecture note must document Terminal Public Claim Delivery Receipt Export/Query/Observability Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke export/query/observability gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_attachment"
  and .status == "ready_blocked"
  and .source_final_blocker_count == 102
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_static_mention_count >= 40
  and .artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_gate_invoked == false
  and .terminal_public_claim_delivery_receipt_export_query_observability_recorded == false
  and .query_registered == false
  and .query_executed == false
  and .export_file_written == false
  and .export_stream_opened == false
  and .observability_metric_recorded == false
  and .dashboard_panel_recorded == false
  and .alert_registered == false
  and .readback_surface_recorded == false
  and .audit_view_recorded == false
  and .release_publication_authority_from_export_query_observability_derived == false
  and .activation_authority_from_export_query_observability_derived == false
  and .install_from_export_query_observability_executed == false
  and .active_binary_from_export_query_observability_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .attachment_blocker_count == 104
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_without_retention"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-final-index-artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability-gate: PASS: terminal public claim delivery receipt export/query/observability attachment is ready but blocked\n'
