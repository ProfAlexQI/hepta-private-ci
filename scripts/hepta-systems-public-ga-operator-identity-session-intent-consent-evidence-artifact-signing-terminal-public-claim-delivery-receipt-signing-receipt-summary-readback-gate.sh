#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-summary-readback-report.sh"

[[ -x "$REPORT" ]] || {
  echo "missing executable signing receipt signing receipt summary/briefing readback report: $REPORT" >&2
  exit 1
}

json="$("$REPORT")"

jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_readback_blocked == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment_ready == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_denial_gate_invoked == false
  and .signing_receipt_operator_summary_recorded == false
  and .signing_receipt_operator_briefing_recorded == false
  and .signing_receipt_readback_digest_recorded == false
  and .signing_receipt_exported_summary_written == false
  and .telegram_signing_receipt_briefing_sent == false
  and .release_publication_authority_from_signing_receipt_summary_briefing_derived == false
  and .activation_authority_from_signing_receipt_summary_briefing_derived == false
  and .install_from_signing_receipt_summary_briefing_executed == false
  and .active_binary_from_signing_receipt_summary_briefing_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .readback_blocker_count == 162
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_final_index_without_observability"
  and (.side_effects | to_entries | all(.value == false))
' <<<"$json" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-summary-readback-gate: PASS: artifact signing receipt signing receipt summary/briefing readback is ready but blocked\n'
