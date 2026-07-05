#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-summary-final-index-delivery-receipt-signing-final-ack-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt final acknowledgement attachment report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_attachment_blocked == true
  and .attachment_blocker_count == 136
  and .signing_receipt_final_acknowledgement_recorded == false
  and .signing_receipt_operator_received_recorded == false
  and .telegram_signing_receipt_acknowledgement_sent == false
  and .release_publication_authority_from_signing_receipt_acknowledgement_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_attachment_attached: true,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_summary_briefing_denial_gate_invoked: false,
    signing_receipt_final_acknowledgement_recorded: false,
    signing_receipt_operator_received_recorded: false,
    signing_receipt_operator_confirmed_recorded: false,
    signing_receipt_operator_read_recorded: false,
    signing_receipt_operator_seen_recorded: false,
    signing_receipt_final_response_recorded: false,
    signing_receipt_completion_acknowledgement_recorded: false,
    signing_receipt_status_acknowledgement_recorded: false,
    signing_receipt_summary_acknowledgement_recorded: false,
    signing_receipt_briefing_acknowledgement_recorded: false,
    signing_receipt_readback_digest_acknowledgement_recorded: false,
    signing_receipt_dashboard_acknowledgement_recorded: false,
    signing_receipt_notification_acknowledgement_recorded: false,
    external_signing_receipt_acknowledgement_sent: false,
    telegram_signing_receipt_acknowledgement_sent: false,
    signing_receipt_acknowledgement_acceptance_recorded: false,
    operator_acceptance_from_signing_receipt_acknowledgement_recorded: false,
    operator_approval_from_signing_receipt_acknowledgement_derived: false,
    release_publication_authority_from_signing_receipt_acknowledgement_derived: false,
    activation_authority_from_signing_receipt_acknowledgement_derived: false,
    install_from_signing_receipt_acknowledgement_executed: false,
    service_restart_from_signing_receipt_acknowledgement_performed: false,
    active_binary_from_signing_receipt_acknowledgement_mutated: false,
    result_receipt_from_signing_receipt_acknowledgement_recorded: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    readback_blocker_count: 136,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_final_acknowledgement_final_index_without_summary",
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
