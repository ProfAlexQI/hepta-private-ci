#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_blocked == true
  and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded == false
  and .artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded == false
  and .artifact_distribution_signing_notarization_receipt_tombstone_recorded == false
  and .operator_approval_from_signing_receipt_cancellation_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_attached,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_doc_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_doc_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_gate_invoked: false,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written: false,
    artifact_distribution_signing_notarization_receipt_cancellation_accepted: false,
    artifact_distribution_signing_notarization_receipt_cancellation_recorded: false,
    artifact_distribution_signing_notarization_receipt_cancellation_persisted: false,
    artifact_distribution_signing_notarization_receipt_withdrawal_accepted: false,
    artifact_distribution_signing_notarization_receipt_supersession_accepted: false,
    artifact_distribution_signing_notarization_receipt_supersession_recorded: false,
    artifact_distribution_signing_notarization_receipt_supersession_persisted: false,
    artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted: false,
    artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded: false,
    artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted: false,
    artifact_distribution_signing_notarization_receipt_tombstone_recorded: false,
    artifact_distribution_signing_notarization_receipt_tombstone_persisted: false,
    artifact_distribution_signing_notarization_receipt_delete_marker_recorded: false,
    artifact_distribution_signing_notarization_receipt_delete_marker_persisted: false,
    artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted: false,
    artifact_signing_receipt_cancellation_accepted: false,
    package_signing_receipt_cancellation_accepted: false,
    signature_manifest_receipt_withdrawal_accepted: false,
    notarization_submission_receipt_cancellation_accepted: false,
    notarization_ticket_receipt_supersession_accepted: false,
    stapling_receipt_tombstone_recorded: false,
    installer_signing_receipt_replacement_accepted: false,
    provenance_attestation_latest_replacement_accepted: false,
    sbom_manifest_supersession_accepted: false,
    release_asset_bundle_cancelled_query_export_accepted: false,
    cdn_update_feed_superseded_observability_accepted: false,
    package_registry_replacement_status_accepted: false,
    dashboard_endpoint_tombstone_hash_status_accepted: false,
    external_supersession_delivery_accepted: false,
    telegram_supersession_delivery_accepted: false,
    operator_approval_from_signing_receipt_cancellation_derived: false,
    release_publication_authority_from_signing_receipt_cancellation_derived: false,
    activation_authority_from_signing_receipt_supersession_derived: false,
    download_link_from_signing_receipt_cancellation_rendered: false,
    install_command_from_signing_receipt_supersession_rendered: false,
    install_from_signing_receipt_cancellation_executed: false,
    service_restart_from_signing_receipt_supersession_performed: false,
    active_binary_from_signing_receipt_cancellation_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    final_blocker_count: 74,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_without_cancellation",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_CANCELLATION_SUPERSESSION_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
