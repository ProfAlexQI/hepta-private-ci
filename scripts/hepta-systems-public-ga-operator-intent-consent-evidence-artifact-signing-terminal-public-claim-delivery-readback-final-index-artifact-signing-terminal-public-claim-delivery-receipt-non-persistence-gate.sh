#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-final-index-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-final-index-gate.sh"
TARGET_AVAILABILITY_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-final-index-artifact-signing-terminal-public-claim-delivery-receipt-target-availability-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_READBACK_FINAL_INDEX_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_NON_PERSISTENCE_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-final-index-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable artifact signing terminal public claim delivery receipt non-persistence attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable artifact signing terminal public claim delivery/readback final index gate: $SOURCE_GATE"
[[ -x "$TARGET_AVAILABILITY_GATE" ]] || fail "missing executable artifact signing terminal public claim delivery receipt target availability gate: $TARGET_AVAILABILITY_GATE"
[[ -f "$DOC" ]] || fail "missing artifact signing terminal public claim delivery receipt non-persistence architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the artifact signing terminal public claim delivery receipt non-persistence attachment report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Terminal Public Claim Delivery Receipt Non-Persistence Attachment' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Terminal Public Claim Delivery Receipt Non-Persistence Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke delivery receipt non-persistence gates"
grep -q 'target gate present: true' "$DOC" \
  || fail "architecture note must document target gate presence"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_blocked == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_target_availability_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_final_index_attached == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_target_availability_attached == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_attachment_blocked == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_static_mention_count >= 40
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_denial_gate_invoked == false
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_readback_denial_gate_invoked == false
  and .target_availability_gate_invoked_target_denial_gate == false
  and .public_claim_delivery_recorded == false
  and .status_readback_recorded == false
  and .delivery_receipt_recorded == false
  and .delivery_receipt_persisted == false
  and .delivery_receipt_materialized == false
  and .delivery_receipt_filesystem_written == false
  and .delivery_receipt_ledger_written == false
  and .delivery_receipt_index_written == false
  and .delivery_receipt_query_registered == false
  and .delivery_receipt_exported == false
  and .delivery_receipt_observability_recorded == false
  and .delivery_receipt_status_exposed == false
  and .delivery_receipt_acknowledgement_accepted == false
  and .readback_receipt_backfilled == false
  and .operator_approval_from_delivery_receipt_derived == false
  and .release_publication_authority_from_delivery_receipt_derived == false
  and .activation_authority_from_delivery_receipt_derived == false
  and .install_from_delivery_receipt_executed == false
  and .service_restart_from_delivery_receipt_performed == false
  and .active_binary_from_delivery_receipt_mutated == false
  and .attachment_blocker_count == 92
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_non_persistence_readback_without_receipt"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null
"$TARGET_AVAILABILITY_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-readback-final-index-artifact-signing-terminal-public-claim-delivery-receipt-non-persistence-gate: PASS: artifact signing terminal public claim delivery receipt non-persistence attachment is ready but blocked\n'
