#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-public-status-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable signing receipt public status readback report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_blocked == true
  and .readback_check_count == 140
  and .terminal_public_claim_status_exposure_recorded == false
  and .public_status_claimed == false
  and .public_ga_claimed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
  and .release_publication_authority_from_public_status_derived == false
  and .activation_authority_from_public_status_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_decision_status_promotion_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_decision_status_promotion_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_decision_status_promotion_denial_gate_invoked: false,
    terminal_public_claim_status_exposure_recorded: false,
    terminal_public_claim_status_exposure_persisted: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    public_status_exposed: false,
    public_ga_status_exposed: false,
    public_release_status_exposed: false,
    release_status_exposed: false,
    publication_status_exposed: false,
    package_release_channel_status_exposed: false,
    dashboard_status_exposed: false,
    public_badge_exposed: false,
    status_endpoint_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    artifact_availability_status_exposed: false,
    distribution_queue_status_exposed: false,
    channel_status_delivered: false,
    external_status_sent: false,
    telegram_status_sent: false,
    acceptance_from_public_status_recorded: false,
    operator_approval_from_public_status_derived: false,
    release_publication_authority_from_public_status_derived: false,
    activation_authority_from_public_status_derived: false,
    activation_command_from_public_status_derived: false,
    live_execution_from_public_status_allowed: false,
    download_link_from_public_status_rendered: false,
    install_command_from_public_status_rendered: false,
    install_from_public_status_executed: false,
    service_restart_from_public_status_performed: false,
    active_binary_from_public_status_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    final_blocker_count: 140,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_without_public_status",
    source_files: {
      signing_public_claim_status_exposure_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-public-status-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      final_index_report_written: false,
      public_claim_status_exposure_final_index_recorded: false,
      public_claim_status_exposure_denial_gate_invoked: false
    })
  }'
