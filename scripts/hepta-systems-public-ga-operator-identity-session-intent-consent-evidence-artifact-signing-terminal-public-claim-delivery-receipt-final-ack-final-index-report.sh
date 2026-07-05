#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-final-ack-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt final acknowledgement readback report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_blocked == true
  and .readback_check_count == 108
  and .terminal_public_claim_delivery_receipt_final_acknowledgement_recorded == false
  and .operator_received_recorded == false
  and .operator_read_recorded == false
  and .external_acknowledgement_sent == false
  and .telegram_acknowledgement_sent == false
  and .release_publication_authority_from_acknowledgement_derived == false
  and .activation_authority_from_acknowledgement_derived == false
  and .install_from_acknowledgement_executed == false
  and .active_binary_from_acknowledgement_mutated == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_readback_attached: true,
    artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_denial_gate_invoked: false,
    terminal_public_claim_delivery_receipt_final_acknowledgement_recorded: false,
    operator_received_recorded: false,
    operator_confirmed_recorded: false,
    operator_read_recorded: false,
    operator_seen_recorded: false,
    final_response_recorded: false,
    completion_acknowledgement_recorded: false,
    status_acknowledgement_recorded: false,
    summary_acknowledgement_recorded: false,
    briefing_acknowledgement_recorded: false,
    readback_digest_acknowledgement_recorded: false,
    dashboard_acknowledgement_recorded: false,
    notification_acknowledgement_recorded: false,
    channel_acknowledgement_delivered: false,
    external_acknowledgement_sent: false,
    telegram_acknowledgement_sent: false,
    acknowledgement_acceptance_recorded: false,
    operator_acceptance_from_acknowledgement_recorded: false,
    operator_approval_from_acknowledgement_derived: false,
    release_publication_authority_from_acknowledgement_derived: false,
    activation_authority_from_acknowledgement_derived: false,
    activation_command_from_acknowledgement_derived: false,
    live_execution_from_acknowledgement_allowed: false,
    install_from_acknowledgement_executed: false,
    service_restart_from_acknowledgement_performed: false,
    active_binary_from_acknowledgement_mutated: false,
    result_receipt_from_acknowledgement_recorded: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    telegram_send_performed: false,
    external_send_performed: false,
    final_blocker_count: 108,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_without_acknowledgement",
    source_files: {
      final_acknowledgement_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-final-ack-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      final_index_report_written: false,
      final_acknowledgement_final_index_recorded: false,
      final_acknowledgement_denial_gate_invoked: false
    })
  }'
