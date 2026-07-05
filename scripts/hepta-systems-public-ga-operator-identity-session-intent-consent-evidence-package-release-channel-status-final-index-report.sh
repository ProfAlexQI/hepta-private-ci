#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-package-release-channel-status-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence package/release channel status readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence package/release channel status final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_blocked == true
  and .package_release_channel_status_exposure_recorded == false
  and .package_channel_status_exposed == false
  and .release_channel_status_exposed == false
  and .operator_approval_from_package_status_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_attached,
    operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate_present: $source.operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate_present,
    operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_doc_present: $source.operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_doc_present,
    operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_package_release_channel_status_gate: $source.long_soak_required_by_source_evidence_package_release_channel_status_gate,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    package_release_channel_status_exposure_accepted: false,
    package_release_channel_status_exposure_recorded: false,
    package_release_channel_status_exposure_persisted: false,
    package_release_channel_status_exposure_materialized: false,
    package_release_channel_status_exposure_filesystem_written: false,
    package_release_channel_status_exposure_delivered: false,
    package_channel_status_exposed: false,
    release_channel_status_exposed: false,
    update_feed_status_exposed: false,
    package_registry_status_exposed: false,
    cdn_status_exposed: false,
    sbom_status_exposed: false,
    signature_status_exposed: false,
    notarization_status_exposed: false,
    version_tag_status_exposed: false,
    dashboard_status_exposed: false,
    public_endpoint_status_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    external_status_sent: false,
    telegram_status_sent: false,
    acceptance_from_package_status_recorded: false,
    operator_approval_from_package_status_derived: false,
    release_publication_authority_from_package_status_derived: false,
    activation_authority_from_package_status_derived: false,
    download_link_from_package_status_rendered: false,
    install_command_from_package_status_rendered: false,
    install_from_package_status_executed: false,
    service_restart_from_package_status_performed: false,
    active_binary_from_package_status_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    final_blocker_count: 62,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_distribution_artifact_manifest_status_without_package_channel",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-package-release-channel-status-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_PACKAGE_RELEASE_CHANNEL_STATUS_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_package_release_channel_status_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-package-release-channel-status-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
