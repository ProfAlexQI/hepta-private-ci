#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-final-index-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-report.sh"
CANCELLATION_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-denial-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_ORDERING_MONOTONICITY_FINAL_INDEX_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_CANCELLATION_SUPERSESSION_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-final-index-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable attachment report: $REPORT"
[[ -f "$CANCELLATION_GATE" ]] || fail "missing artifact signing terminal public claim delivery receipt cancellation/supersession denial gate: $CANCELLATION_GATE"
[[ -f "$DOC" ]] || fail "missing cancellation/supersession attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the cancellation/supersession attachment report"
fi

grep -q 'Terminal Public Claim Delivery Receipt Ordering/Monotonicity Final Index Attachment' "$DOC" \
  || fail "architecture note must document terminal public claim delivery receipt ordering/monotonicity final index attachment"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke target gates"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_static_mention_count >= 18
  and .artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_ordering_monotonicity_denial_gate_invoked == false
  and .terminal_public_claim_delivery_receipt_cancellation_supersession_recorded == false
  and .terminal_public_claim_delivery_receipt_cancellation_accepted == false
  and .terminal_public_claim_delivery_receipt_supersession_accepted == false
  and .terminal_public_claim_delivery_receipt_replacement_receipt_recorded == false
  and .terminal_public_claim_delivery_receipt_tombstone_recorded == false
  and .terminal_public_claim_delivery_receipt_delete_marker_recorded == false
  and .terminal_public_claim_delivery_receipt_lifecycle_cancellation_supersession_recorded == false
  and .release_publication_authority_from_delivery_receipt_cancellation_supersession_derived == false
  and .activation_authority_from_delivery_receipt_cancellation_supersession_derived == false
  and .install_from_delivery_receipt_cancellation_supersession_executed == false
  and .active_binary_from_delivery_receipt_cancellation_supersession_mutated == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .attachment_blocker_count == 98
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_cancellation_supersession_readback_without_cancellation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-ordering-monotonicity-final-index-artifact-signing-terminal-public-claim-delivery-receipt-cancellation-supersession-gate: PASS: artifact signing terminal public claim delivery receipt cancellation/supersession attachment is ready but blocked\n'
