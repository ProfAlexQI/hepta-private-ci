#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-replay-final-index-delivery-receipt-signing-receipt-signing-receipt-ordering-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-replay-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_SIGNING_RECEIPT_REPLAY_FINAL_INDEX_DELIVERY_RECEIPT_SIGNING_RECEIPT_SIGNING_RECEIPT_ORDERING_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-ordering-attachment-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt ordering/monotonicity attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt replay/idempotency final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing terminal public claim delivery receipt artifact signing receipt signing receipt ordering/monotonicity attachment architecture note: $DOC"

grep -q 'Terminal Public Claim Delivery Receipt Artifact Signing Receipt Signing Receipt Ordering/Monotonicity Attachment' "$DOC" \
  || fail "architecture note must document Terminal Public Claim Delivery Receipt Artifact Signing Receipt Signing Receipt Ordering/Monotonicity Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke signing receipt ordering gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_ordering_monotonicity_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_ordering_monotonicity_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_ordering_monotonicity_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_ordering_monotonicity_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_ordering_monotonicity_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_ordering_monotonicity_static_mention_count >= 40
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_ordering_monotonicity_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_signing_receipt_replay_idempotency_denial_gate_invoked == false
  and .signing_receipt_ordering_recorded == false
  and .signing_receipt_sequence_cursor_recorded == false
  and .signing_receipt_monotonicity_state_recorded == false
  and .signing_receipt_latest_wins_overwrite_accepted == false
  and .signing_receipt_ordered_status_accepted == false
  and .signing_receipt_ordered_ack_accepted == false
  and .signing_receipt_hash_sequence_rebind_accepted == false
  and .artifact_signing_receipt_signing_receipt_signing_receipt_ordering_accepted == false
  and .notarization_ticket_receipt_ordering_accepted == false
  and .external_signing_receipt_ordering_accepted == false
  and .telegram_signing_receipt_ordering_accepted == false
  and .operator_approval_from_signing_receipt_ordering_derived == false
  and .release_publication_authority_from_signing_receipt_ordering_derived == false
  and .activation_authority_from_signing_receipt_ordering_derived == false
  and .install_from_signing_receipt_ordering_executed == false
  and .active_binary_from_signing_receipt_ordering_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .attachment_blocker_count == 264
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-signing-receipt-ordering-attachment-gate: PASS: terminal public claim delivery receipt artifact signing receipt signing receipt ordering/monotonicity attachment is ready but blocked\n'
