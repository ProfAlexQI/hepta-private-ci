#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-receipt-no-persistence-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence artifact signing receipt no-persistence readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence artifact signing receipt no-persistence final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback_blocked == true
  and .artifact_distribution_signing_notarization_result_receipt_surface_recorded == false
  and .artifact_signing_receipt_accepted == false
  and .notarization_submission_receipt_persisted == false
  and .operator_approval_from_signing_receipt_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_final_index_attached,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_gate_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_gate_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_doc_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_doc_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_artifact_signing_receipt_no_persistence_gate: $source.long_soak_required_by_source_evidence_artifact_signing_receipt_no_persistence_gate,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    artifact_distribution_signing_notarization_result_receipt_surface_accepted: false,
    artifact_distribution_signing_notarization_result_receipt_surface_recorded: false,
    artifact_distribution_signing_notarization_result_receipt_surface_persisted: false,
    artifact_distribution_signing_notarization_result_receipt_surface_materialized: false,
    artifact_distribution_signing_notarization_result_receipt_surface_filesystem_written: false,
    artifact_distribution_signing_notarization_result_receipt_surface_delivered: false,
    artifact_distribution_signing_notarization_result_receipt_surface_indexed: false,
    artifact_distribution_signing_notarization_result_receipt_surface_exported: false,
    artifact_distribution_signing_notarization_result_receipt_surface_query_registered: false,
    artifact_distribution_signing_notarization_result_receipt_surface_observability_recorded: false,
    artifact_distribution_signing_notarization_result_receipt_surface_status_exposed: false,
    artifact_signing_receipt_accepted: false,
    package_signing_receipt_accepted: false,
    signature_manifest_receipt_recorded: false,
    notarization_submission_receipt_persisted: false,
    notarization_ticket_receipt_materialized: false,
    stapling_receipt_filesystem_written: false,
    installer_signing_receipt_delivered: false,
    provenance_attestation_receipt_indexed: false,
    sbom_manifest_receipt_exported: false,
    release_asset_bundle_receipt_query_registered: false,
    cdn_update_feed_receipt_observability_recorded: false,
    package_registry_receipt_status_exposed: false,
    dashboard_endpoint_receipt_status_exposed: false,
    external_signing_receipt_delivered: false,
    telegram_signing_receipt_delivered: false,
    acceptance_from_signing_receipt_recorded: false,
    operator_approval_from_signing_receipt_derived: false,
    release_publication_authority_from_signing_receipt_derived: false,
    activation_authority_from_signing_receipt_derived: false,
    download_link_from_signing_receipt_rendered: false,
    install_command_from_signing_receipt_rendered: false,
    install_from_signing_receipt_executed: false,
    service_restart_from_signing_receipt_performed: false,
    active_binary_from_signing_receipt_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    final_blocker_count: 68,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_without_receipt",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-receipt-no-persistence-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_NO_PERSISTENCE_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_receipt_no_persistence_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-receipt-no-persistence-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
