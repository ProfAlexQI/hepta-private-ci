#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-distribution-artifact-manifest-status-final-index-report.sh"
ARTIFACT_SIGNING_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-notarization-surface-denial-gate.sh"
ARTIFACT_SIGNING_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_NOTARIZATION_SURFACE_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence distribution artifact/manifest status final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$ARTIFACT_SIGNING_GATE" ]] || {
  echo "missing operator identity/session intent consent evidence artifact signing/notarization surface denial gate: $ARTIFACT_SIGNING_GATE" >&2
  exit 1
}
[[ -f "$ARTIFACT_SIGNING_DOC" ]] || {
  echo "missing operator identity/session intent consent evidence artifact signing/notarization surface denial doc: $ARTIFACT_SIGNING_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence artifact signing/notarization report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_final_index_blocked == true
  and .distribution_artifact_manifest_status_recorded == false
  and .manifest_status_exposed == false
  and .operator_approval_from_manifest_status_derived == false
  and .public_ga_claimed == false
' <<<"$source_json" >/dev/null

artifact_signing_static_mention_count="$(
  grep -Eci 'signing|signature|notarization|notarize|ticket|stapling|staple|provenance|sbom|release.asset|bundle|cdn|update.feed|registry|dashboard|endpoint|query|export|observability|telegram|external|authority|install|restart|active-binary|live' "$ARTIFACT_SIGNING_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson artifact_signing_static_mention_count "$artifact_signing_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_attachment_blocked: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_denial_gate_present: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_denial_doc_present: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_static_mention_count: $artifact_signing_static_mention_count,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_artifact_signing_notarization_surface_gate: true,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    artifact_distribution_signing_notarization_surface_accepted: false,
    artifact_distribution_signing_notarization_surface_recorded: false,
    artifact_distribution_signing_notarization_surface_persisted: false,
    artifact_distribution_signing_notarization_surface_materialized: false,
    artifact_distribution_signing_notarization_surface_filesystem_written: false,
    artifact_distribution_signing_notarization_surface_delivered: false,
    artifact_distribution_signing_notarization_surface_exposed: false,
    artifact_distribution_signing_notarization_surface_executed: false,
    artifact_signed: false,
    package_signed: false,
    signature_manifest_written: false,
    checksum_binding_recorded: false,
    notarization_submitted: false,
    notarization_ticket_recorded: false,
    stapling_executed: false,
    installer_signed: false,
    provenance_attestation_published: false,
    sbom_manifest_published: false,
    release_asset_packaged: false,
    artifact_bundle_packaged: false,
    cdn_artifact_written: false,
    update_feed_artifact_written: false,
    package_registry_artifact_published: false,
    dashboard_status_exposed: false,
    public_endpoint_status_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    external_status_sent: false,
    telegram_status_sent: false,
    acceptance_from_signing_status_recorded: false,
    operator_approval_from_signing_status_derived: false,
    release_publication_authority_from_signing_status_derived: false,
    activation_authority_from_signing_status_derived: false,
    download_link_from_signing_status_rendered: false,
    install_command_from_signing_status_rendered: false,
    install_from_signing_status_executed: false,
    service_restart_from_signing_status_performed: false,
    active_binary_from_signing_status_mutated: false,
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
    attachment_blocker_count: 66,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_readback_without_manifest_status",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-distribution-artifact-manifest-status-final-index-artifact-signing-notarization-surface-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_DISTRIBUTION_ARTIFACT_MANIFEST_STATUS_FINAL_INDEX_ARTIFACT_SIGNING_NOTARIZATION_SURFACE_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_distribution_artifact_manifest_status_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-distribution-artifact-manifest-status-final-index-report.sh",
      operator_identity_session_intent_consent_evidence_artifact_signing_notarization_surface_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-notarization-surface-denial-gate.sh",
      operator_identity_session_intent_consent_evidence_artifact_signing_notarization_surface_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_NOTARIZATION_SURFACE_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      operator_identity_session_operator_intent_consent_evidence_artifact_signing_notarization_surface_denial_gate_invoked: false,
      operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_gate_invoked: false,
      artifact_distribution_signing_notarization_surface_recorded: false,
      artifact_distribution_signing_notarization_surface_persisted: false,
      artifact_distribution_signing_notarization_surface_materialized: false,
      artifact_distribution_signing_notarization_surface_filesystem_written: false,
      artifact_distribution_signing_notarization_surface_executed: false,
      artifact_signed: false,
      package_signed: false,
      signature_manifest_written: false,
      checksum_binding_recorded: false,
      notarization_submitted: false,
      notarization_ticket_recorded: false,
      stapling_executed: false,
      installer_signed: false,
      provenance_attestation_published: false,
      sbom_manifest_published: false,
      release_asset_packaged: false,
      artifact_bundle_packaged: false,
      cdn_artifact_written: false,
      update_feed_artifact_written: false,
      package_registry_artifact_published: false,
      dashboard_status_exposed: false,
      public_endpoint_status_exposed: false,
      query_status_exposed: false,
      export_status_exposed: false,
      observability_status_exposed: false,
      external_status_sent: false,
      telegram_status_sent: false,
      public_status_claimed: false,
      public_release_claimed: false,
      public_ga_claimed: false,
      operator_approval_from_signing_status_derived: false,
      release_publication_authority_from_signing_status_derived: false,
      activation_authority_from_signing_status_derived: false,
      download_link_from_signing_status_rendered: false,
      install_command_from_signing_status_rendered: false,
      install_from_signing_status_executed: false,
      service_restart_from_signing_status_performed: false,
      active_binary_from_signing_status_mutated: false,
      memory_store_write_performed: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
      telegram_send_performed: false,
      external_send_performed: false,
      long_soak_started: false,
      terminal_live_gate_invoked: false,
      terminal_live_url_contacted: false,
      public_ga_readiness_script_invoked: false,
      public_claim_non_promotion_denial_gate_invoked: false,
      public_ga_claim_recorded: false,
      public_ga_promoted: false,
      public_release_published: false,
      rollback_executed: false,
      external_network_read: false,
      release_artifact_written: false,
      public_artifact_written: false,
      filesystem_written: false
    }
  }'
