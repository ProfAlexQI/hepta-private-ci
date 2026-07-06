#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-nonp-final-index-delivery-receipt-signing-receipt-replay-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-nonp-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SIGNING_RECEIPT_NONP_FINAL_INDEX_DELIVERY_RECEIPT_SIGNING_RECEIPT_REPLAY_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-replay-attachment-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable terminal public claim delivery receipt artifact signing receipt replay/idempotency attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable terminal public claim delivery receipt artifact signing receipt non-persistence final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing terminal public claim delivery receipt artifact signing receipt replay/idempotency attachment architecture note: $DOC"

grep -q 'Terminal Public Claim Delivery Receipt Artifact Signing Receipt Signing Receipt Replay/Idempotency Attachment' "$DOC" \
  || fail "architecture note must document Terminal Public Claim Delivery Receipt Artifact Signing Receipt Signing Receipt Replay/Idempotency Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke signing replay gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_replay_idempotency_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_non_persistence_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_non_persistence_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_replay_idempotency_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_replay_idempotency_attachment_blocked == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_replay_idempotency_denial_gate_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_replay_idempotency_denial_doc_present == true
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_replay_idempotency_static_mention_count >= 40
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_replay_idempotency_denial_gate_invoked == false
  and .artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_non_persistence_denial_gate_invoked == false
  and .signing_receipt_replay_accepted == false
  and .signing_receipt_replay_recorded == false
  and .signing_receipt_replay_persisted == false
  and .signing_receipt_replay_performed == false
  and .signing_receipt_duplicate_accepted == false
  and .signing_receipt_idempotency_key_accepted == false
  and .signing_receipt_idempotency_key_recorded == false
  and .signing_receipt_idempotency_state_recorded == false
  and .signing_receipt_idempotency_state_persisted == false
  and .signing_receipt_replay_nonce_accepted == false
  and .signing_receipt_cross_scope_reuse_accepted == false
  and .signing_receipt_status_upgrade_accepted == false
  and .signing_receipt_ack_replay_accepted == false
  and .signing_receipt_hash_status_rebind_accepted == false
  and .artifact_signing_receipt_replay_accepted == false
  and .notarization_ticket_receipt_replay_accepted == false
  and .external_signing_receipt_replay_accepted == false
  and .telegram_signing_receipt_replay_accepted == false
  and .operator_approval_from_signing_receipt_replay_derived == false
  and .release_publication_authority_from_signing_receipt_replay_derived == false
  and .activation_authority_from_signing_receipt_replay_derived == false
  and .install_from_signing_receipt_replay_executed == false
  and .active_binary_from_signing_receipt_replay_mutated == false
  and .provider_invoked == false
  and .credential_read == false
  and .attachment_blocker_count == 150
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-terminal-public-claim-delivery-receipt-signing-receipt-replay-attachment-gate: PASS: terminal public claim delivery receipt artifact signing receipt replay/idempotency attachment is ready but blocked\n'
