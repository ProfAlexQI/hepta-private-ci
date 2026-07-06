#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-terminal-public-claim-status-exposure-final-index-report.sh"
PACKAGE_CHANNEL_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-package-release-channel-status-exposure-denial-gate.sh"
PACKAGE_CHANNEL_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_PACKAGE_RELEASE_CHANNEL_STATUS_EXPOSURE_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence terminal public claim/status final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$PACKAGE_CHANNEL_GATE" ]] || {
  echo "missing operator identity/session intent consent evidence package/release channel status exposure denial gate: $PACKAGE_CHANNEL_GATE" >&2
  exit 1
}
[[ -f "$PACKAGE_CHANNEL_DOC" ]] || {
  echo "missing operator identity/session intent consent evidence package/release channel status exposure denial doc: $PACKAGE_CHANNEL_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence package/release channel status report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_blocked == true
  and .public_status_claimed == false
  and .public_release_claimed == false
  and .public_ga_claimed == false
  and .package_release_channel_status_exposed == false
' <<<"$source_json" >/dev/null

package_release_channel_static_mention_count="$(
  grep -Ec 'package|release.channel|release|channel|registry|feed|cdn|sbom|provenance|signature|notarization|version|dashboard|endpoint|query|export|observability|telegram|external|authority|install|restart|active-binary|live' "$PACKAGE_CHANNEL_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson package_release_channel_static_mention_count "$package_release_channel_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_attachment_blocked: true,
    operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate_present: true,
    operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_doc_present: true,
    operator_identity_session_operator_intent_consent_evidence_package_release_channel_static_mention_count: $package_release_channel_static_mention_count,
    operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_package_release_channel_status_gate: true,
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
    attachment_blocker_count: 62,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_readback_without_public_claim",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-terminal-public-claim-status-final-index-package-release-channel-status-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_TERMINAL_PUBLIC_CLAIM_STATUS_FINAL_INDEX_PACKAGE_RELEASE_CHANNEL_STATUS_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_terminal_public_claim_status_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-terminal-public-claim-status-exposure-final-index-report.sh",
      operator_identity_session_intent_consent_evidence_package_release_channel_status_exposure_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-package-release-channel-status-exposure-denial-gate.sh",
      operator_identity_session_intent_consent_evidence_package_release_channel_status_exposure_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_PACKAGE_RELEASE_CHANNEL_STATUS_EXPOSURE_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      operator_identity_session_operator_intent_consent_evidence_package_release_channel_status_exposure_denial_gate_invoked: false,
      operator_identity_session_operator_intent_consent_evidence_terminal_public_claim_status_exposure_denial_gate_invoked: false,
      package_release_channel_status_exposure_recorded: false,
      package_release_channel_status_exposure_persisted: false,
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
      public_status_claimed: false,
      public_release_claimed: false,
      public_ga_claimed: false,
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
      external_send_performed: false,
      telegram_send_performed: false,
      long_soak_started: false,
      terminal_live_gate_invoked: false,
      terminal_live_url_contacted: false,
      public_ga_readiness_script_invoked: false,
      public_claim_non_promotion_denial_gate_invoked: false,
      public_ga_claim_recorded: false,
      public_ga_promoted: false,
      public_release_published: false,
      rollback_executed: false,
      external_network_read: false
    }
  }'
