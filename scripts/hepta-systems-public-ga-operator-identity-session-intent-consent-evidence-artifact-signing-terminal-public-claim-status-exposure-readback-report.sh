#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-decision-status-final-index-artifact-signing-terminal-public-claim-status-exposure-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable artifact signing terminal public claim/status exposure attachment report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the artifact signing terminal public claim/status exposure readback report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_blocked == true
  and .terminal_public_claim_status_exposure_recorded == false
  and .public_status_claimed == false
  and .public_ga_claimed == false
  and .telegram_status_sent == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_final_index_attached,
    readback_mode: "static_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_snapshot_only",
    readback_check_count: 88,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_denial_gate_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_denial_gate_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_denial_doc_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_denial_doc_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_denial_gate_invoked: false,
    long_soak_required_by_source_artifact_signing_terminal_public_claim_status_gate: $source.long_soak_required_by_source_artifact_signing_terminal_public_claim_status_gate,
    long_soak_started: false,
    terminal_decision_recorded: false,
    terminal_status_recorded: false,
    status_promotion_recorded: false,
    terminal_public_claim_status_exposure_accepted: false,
    terminal_public_claim_status_exposure_recorded: false,
    terminal_public_claim_status_exposure_persisted: false,
    terminal_public_claim_status_exposure_materialized: false,
    terminal_public_claim_status_exposure_filesystem_written: false,
    terminal_public_claim_status_exposure_delivered: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    release_status_exposed: false,
    publication_status_exposed: false,
    package_release_channel_status_exposed: false,
    dashboard_status_exposed: false,
    public_badge_exposed: false,
    status_endpoint_exposed: false,
    query_status_exposed: false,
    export_status_exposed: false,
    observability_status_exposed: false,
    release_notes_status_exposed: false,
    changelog_status_exposed: false,
    version_tag_status_exposed: false,
    artifact_availability_status_exposed: false,
    distribution_queue_status_exposed: false,
    live_install_status_exposed: false,
    channel_status_delivered: false,
    external_status_sent: false,
    telegram_status_sent: false,
    acceptance_from_public_status_recorded: false,
    operator_approval_from_public_status_derived: false,
    release_publication_authority_from_public_status_derived: false,
    activation_authority_from_public_status_derived: false,
    download_link_from_public_status_rendered: false,
    install_command_from_public_status_rendered: false,
    install_from_public_status_executed: false,
    service_restart_from_public_status_performed: false,
    active_binary_from_public_status_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    readback_blocker_count: 88,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_final_index_without_status_promotion",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-status-exposure-readback-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_STATUS_EXPOSURE_READBACK_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_terminal_public_claim_status_exposure_attachment_report: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-decision-status-final-index-artifact-signing-terminal-public-claim-status-exposure-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
