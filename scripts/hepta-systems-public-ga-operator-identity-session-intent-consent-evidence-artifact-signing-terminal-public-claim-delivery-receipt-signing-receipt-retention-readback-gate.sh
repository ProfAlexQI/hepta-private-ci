#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-retention-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-audit-final-index-delivery-receipt-signing-receipt-retention-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_RETENTION_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-retention-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt retention/expiry/GC readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt retention/expiry/GC attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing terminal public claim delivery receipt artifact signing receipt signing receipt retention/expiry/GC readback architecture note: $DOC"

grep -q 'Terminal Public Claim Delivery Receipt Artifact Signing Receipt Signing Receipt Retention/Expiry/GC Readback' "$DOC" \
  || fail "architecture note must document Terminal Public Claim Delivery Receipt Artifact Signing Receipt Signing Receipt Retention/Expiry/GC Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke signing retention gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_retention_expiry_gc_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_retention_expiry_gc_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_retention_expiry_gc_readback_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_retention_expiry_gc_denial_gate_invoked == false
  and .signing_receipt_retention_policy_recorded == false
  and .signing_receipt_expiry_timer_started == false
  and .signing_receipt_garbage_collection_decision_recorded == false
  and .signing_receipt_garbage_collection_executed == false
  and .signing_receipt_archive_recorded == false
  and .signing_receipt_compaction_recorded == false
  and .external_signing_receipt_retention_recorded == false
  and .telegram_signing_receipt_retention_recorded == false
  and .operator_approval_from_signing_receipt_retention_derived == false
  and .release_publication_authority_from_signing_receipt_retention_derived == false
  and .activation_authority_from_signing_receipt_retention_derived == false
  and .install_from_signing_receipt_retention_executed == false
  and .active_binary_from_signing_receipt_retention_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .readback_blocker_count == 158
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-retention-readback-gate: PASS: terminal public claim delivery receipt artifact signing receipt signing receipt retention/expiry/GC readback is ready but blocked\n'
