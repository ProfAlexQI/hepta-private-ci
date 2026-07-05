#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-replay-idempotency-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-final-index-artifact-signing-replay-idempotency-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_REPLAY_IDEMPOTENCY_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-replay-idempotency-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing replay/idempotency readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing replay/idempotency attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent evidence artifact signing replay/idempotency readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent evidence artifact signing replay/idempotency readback report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Replay/Idempotency Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Replay/Idempotency Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke artifact signing replay/idempotency gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_attachment_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_attachment"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_attachment_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_attachment_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_readback_blocked == true
  and .readback_mode == "static_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_snapshot_only"
  and .readback_check_count == 70
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_denial_gate_invoked == false
  and .long_soak_started == false
  and .artifact_distribution_signing_notarization_receipt_replay_accepted == false
  and .artifact_distribution_signing_notarization_receipt_replay_recorded == false
  and .artifact_distribution_signing_notarization_receipt_idempotency_key_recorded == false
  and .artifact_distribution_signing_notarization_receipt_idempotency_state_persisted == false
  and .artifact_distribution_signing_notarization_receipt_status_upgrade_accepted == false
  and .artifact_distribution_signing_notarization_receipt_completed_status_accepted == false
  and .artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted == false
  and .artifact_signing_receipt_replay_accepted == false
  and .package_signing_receipt_replay_accepted == false
  and .external_signing_receipt_delivery_replay_accepted == false
  and .telegram_signing_receipt_delivery_replay_accepted == false
  and .operator_approval_from_signing_receipt_replay_derived == false
  and .release_publication_authority_from_signing_receipt_replay_derived == false
  and .activation_authority_from_signing_receipt_replay_derived == false
  and .install_from_signing_receipt_replay_executed == false
  and .service_restart_from_signing_receipt_replay_performed == false
  and .active_binary_from_signing_receipt_replay_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .readback_blocker_count == 70
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_final_index_without_receipt"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-replay-idempotency-readback-gate: PASS: Public GA operator identity/session intent consent evidence artifact signing replay/idempotency readback is ready but blocked\n'
