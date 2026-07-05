#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-final-index-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_AUDIT_EVIDENCE_FINAL_INDEX_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_RETENTION_EXPIRY_GC_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-final-index-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable artifact signing terminal public claim delivery receipt retention/expiry/GC attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable artifact signing terminal public claim delivery receipt audit evidence final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing artifact signing terminal public claim delivery receipt retention/expiry/GC attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the artifact signing terminal public claim delivery receipt retention/expiry/GC attachment report"
fi

grep -q 'Terminal Public Claim Delivery Receipt Retention/Expiry/GC Attachment' "$DOC" \
  || fail "architecture note must document Terminal Public Claim Delivery Receipt Retention/Expiry/GC Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke retention/expiry/GC gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_attachment"
  and .status == "ready_blocked"
  and .source_final_blocker_count == 100
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_static_mention_count >= 40
  and .artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_audit_evidence_denial_gate_invoked == false
  and .terminal_public_claim_delivery_receipt_retention_expiry_garbage_collection_recorded == false
  and .retention_policy_recorded == false
  and .ttl_lease_recorded == false
  and .expiry_timer_started == false
  and .garbage_collection_queue_recorded == false
  and .garbage_collection_executed == false
  and .archive_recorded == false
  and .compaction_performed == false
  and .release_publication_authority_from_delivery_receipt_retention_derived == false
  and .activation_authority_from_delivery_receipt_retention_derived == false
  and .install_from_delivery_receipt_retention_executed == false
  and .active_binary_from_delivery_receipt_retention_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .attachment_blocker_count == 102
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_readback_without_audit_evidence"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-audit-evidence-final-index-artifact-signing-terminal-public-claim-delivery-receipt-retention-expiry-gc-gate: PASS: terminal public claim delivery receipt retention/expiry/GC attachment is ready but blocked\n'
