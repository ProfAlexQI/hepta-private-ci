#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-artifact-signing-audit-evidence-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_CANCELLATION_SUPERSESSION_FINAL_INDEX_ARTIFACT_SIGNING_AUDIT_EVIDENCE_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-artifact-signing-audit-evidence-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing audit/evidence attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent evidence artifact signing audit/evidence attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent evidence artifact signing audit/evidence attachment report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Audit/Evidence Attachment' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Artifact Signing Audit/Evidence Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke artifact signing audit/evidence gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_attached == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_attachment_blocked == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_static_mention_count >= 40
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_denial_gate_invoked == false
  and .operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_invoked == false
  and .long_soak_started == false
  and .public_ga_claimed == false
  and .artifact_distribution_signing_notarization_receipt_audit_evidence_accepted == false
  and .artifact_distribution_signing_notarization_receipt_audit_evidence_recorded == false
  and .artifact_distribution_signing_notarization_receipt_audit_trail_recorded == false
  and .artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded == false
  and .artifact_distribution_signing_notarization_receipt_hash_chain_recorded == false
  and .artifact_distribution_signing_notarization_receipt_merkle_root_recorded == false
  and .artifact_distribution_signing_notarization_receipt_attestation_recorded == false
  and .artifact_distribution_signing_notarization_receipt_witness_recorded == false
  and .artifact_distribution_signing_notarization_receipt_notary_recorded == false
  and .artifact_distribution_signing_notarization_receipt_ledger_recorded == false
  and .artifact_distribution_signing_notarization_receipt_index_recorded == false
  and .artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded == false
  and .artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded == false
  and .artifact_distribution_signing_notarization_receipt_observability_evidence_recorded == false
  and .artifact_distribution_signing_notarization_receipt_readback_evidence_recorded == false
  and .artifact_distribution_signing_notarization_receipt_status_evidence_recorded == false
  and .artifact_distribution_signing_notarization_receipt_hash_status_evidence_recorded == false
  and .artifact_signing_cancellation_audit_trail_recorded == false
  and .package_signing_supersession_immutable_evidence_recorded == false
  and .signature_manifest_withdrawal_hash_chain_recorded == false
  and .notarization_submission_cancellation_attestation_recorded == false
  and .notarization_ticket_supersession_witness_recorded == false
  and .stapling_tombstone_ledger_index_recorded == false
  and .external_audit_evidence_delivered == false
  and .telegram_audit_evidence_delivered == false
  and .operator_approval_from_signing_receipt_audit_evidence_derived == false
  and .release_publication_authority_from_signing_receipt_audit_evidence_derived == false
  and .activation_authority_from_signing_receipt_audit_evidence_derived == false
  and .install_from_signing_receipt_audit_evidence_executed == false
  and .service_restart_from_signing_receipt_audit_evidence_performed == false
  and .active_binary_from_signing_receipt_audit_evidence_mutated == false
  and .provider_invoked == false
  and .model_invoked == false
  and .credential_read == false
  and .secret_file_read == false
  and .attachment_blocker_count == 76
  and .public_ga_claim_allowed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_readback_without_cancellation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-artifact-signing-audit-evidence-gate: PASS: Public GA operator identity/session intent consent evidence artifact signing audit/evidence attachment is ready but blocked without cancellation acceptance\n'
