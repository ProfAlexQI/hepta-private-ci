#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-summary-final-index-report.sh"

[[ -x "$REPORT" ]] || {
  echo "missing executable signing receipt summary/briefing final index report: $REPORT" >&2
  exit 1
}

json="$("$REPORT")"

jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_summary_briefing_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_summary_briefing_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_summary_briefing_final_index_blocked == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_summary_briefing_readback_ready == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_summary_briefing_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_export_query_observability_denial_gate_invoked == false
  and .signing_receipt_operator_summary_recorded == false
  and .signing_receipt_operator_briefing_recorded == false
  and .signing_receipt_readback_digest_recorded == false
  and .signing_receipt_exported_summary_written == false
  and .signing_receipt_dashboard_narrative_recorded == false
  and .signing_receipt_audit_narrative_recorded == false
  and .telegram_signing_receipt_briefing_sent == false
  and .release_publication_authority_from_signing_receipt_summary_briefing_derived == false
  and .activation_authority_from_signing_receipt_summary_briefing_derived == false
  and .install_from_signing_receipt_summary_briefing_executed == false
  and .active_binary_from_signing_receipt_summary_briefing_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .final_blocker_count == 134
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .next_migration_step == "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_summary_briefing_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_non_acceptance_without_summary"
  and (.side_effects | to_entries | all(.value == false))
' <<<"$json" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-summary-final-index-gate: PASS: artifact signing receipt summary/briefing final index is ready but blocked\n'
