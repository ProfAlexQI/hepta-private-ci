#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-public-status-final-index-delivery-receipt-signing-package-channel-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing receipt package/release/channel status attachment report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing receipt package/release/channel status readback report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_attachment_blocked == true
  and .attachment_blocker_count == 142
  and .package_release_channel_status_recorded == false
  and .package_channel_status_exposed == false
  and .release_channel_status_exposed == false
  and .external_status_sent == false
  and .telegram_status_sent == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_final_index_attached,
    readback_mode: "static_artifact_signing_receipt_package_release_channel_status_snapshot_only",
    readback_check_count: 142,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_terminal_public_claim_status_exposure_denial_gate_invoked: false,
    package_release_channel_status_recorded: false,
    package_release_channel_status_persisted: false,
    package_release_channel_status_materialized: false,
    package_release_channel_status_filesystem_written: false,
    package_release_channel_status_delivered: false,
    package_channel_status_exposed: false,
    release_channel_status_exposed: false,
    registry_channel_status_exposed: false,
    artifact_channel_status_exposed: false,
    install_channel_status_exposed: false,
    update_feed_status_exposed: false,
    version_channel_status_exposed: false,
    distribution_channel_status_exposed: false,
    dashboard_channel_status_exposed: false,
    status_endpoint_channel_status_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    channel_status_delivered: false,
    external_status_sent: false,
    telegram_status_sent: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    operator_approval_from_package_channel_derived: false,
    release_publication_authority_from_package_channel_derived: false,
    activation_authority_from_package_channel_derived: false,
    install_from_package_channel_executed: false,
    service_restart_from_package_channel_performed: false,
    active_binary_from_package_channel_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    terminal_live_gates_invoked: false,
    readback_blocker_count: 142,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_package_release_channel_status_final_index_without_public_status",
    source_files: {
      signing_package_release_channel_status_attachment_report: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-public-status-final-index-delivery-receipt-signing-package-channel-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      readback_report_written: false,
      package_release_channel_status_readback_recorded: false,
      package_release_channel_status_denial_gate_invoked: false
    })
  }'
