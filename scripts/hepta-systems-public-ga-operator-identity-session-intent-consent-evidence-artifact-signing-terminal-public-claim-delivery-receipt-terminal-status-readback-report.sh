#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-final-ack-final-index-delivery-receipt-terminal-status-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt terminal decision/status attachment report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_blocked == true
  and .attachment_blocker_count == 110
  and .terminal_decision_recorded == false
  and .terminal_status_recorded == false
  and .status_promotion_recorded == false
  and .public_status_exposed == false
  and .telegram_decision_sent == false
  and .release_publication_authority_from_terminal_status_derived == false
  and .activation_authority_from_terminal_status_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_final_acknowledgement_denial_gate_invoked: false,
    readback_mode: "static_terminal_public_claim_delivery_receipt_terminal_decision_status_snapshot_only",
    readback_check_count: 110,
    terminal_decision_recorded: false,
    terminal_decision_persisted: false,
    terminal_status_recorded: false,
    terminal_status_persisted: false,
    status_promotion_recorded: false,
    public_status_exposed: false,
    public_ga_status_exposed: false,
    public_release_status_exposed: false,
    external_decision_sent: false,
    telegram_decision_sent: false,
    acceptance_from_terminal_decision_recorded: false,
    operator_approval_from_terminal_status_derived: false,
    release_publication_authority_from_terminal_status_derived: false,
    activation_authority_from_terminal_status_derived: false,
    activation_command_from_terminal_status_derived: false,
    live_execution_from_terminal_status_allowed: false,
    download_link_from_terminal_status_rendered: false,
    install_command_from_terminal_status_rendered: false,
    install_from_terminal_status_executed: false,
    service_restart_from_terminal_status_performed: false,
    active_binary_from_terminal_status_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    readback_blocker_count: 110,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_terminal_decision_status_promotion_final_index_without_acknowledgement",
    source_files: {
      terminal_decision_status_attachment_report: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-final-ack-final-index-delivery-receipt-terminal-status-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      readback_report_written: false,
      terminal_decision_status_readback_recorded: false,
      terminal_decision_status_promotion_denial_gate_invoked: false
    })
  }'
