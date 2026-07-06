#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-final-acknowledgement-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence artifact signing final acknowledgement readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence artifact signing final acknowledgement final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_readback_blocked == true
  and .final_operator_acknowledgement_accepted == false
  and .acknowledgement_acceptance_recorded == false
  and .operator_approval_from_acknowledgement_derived == false
  and .activation_authority_from_acknowledgement_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_final_index_attached,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_gate_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_gate_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_doc_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_doc_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_denial_gate_invoked: false,
    long_soak_required_by_source_artifact_signing_final_acknowledgement_gate: $source.long_soak_required_by_source_artifact_signing_final_acknowledgement_gate,
    long_soak_started: false,
    public_ga_operator_approval_packet_invoked: false,
    public_ga_operator_packet_sent: false,
    operator_approval_request_sent: false,
    operator_approval_recorded: false,
    operator_approval_accepted: false,
    operator_summary_recorded: false,
    operator_briefing_recorded: false,
    briefing_acknowledgement_recorded: false,
    final_operator_acknowledgement_accepted: false,
    final_operator_acknowledgement_recorded: false,
    final_operator_acknowledgement_persisted: false,
    final_operator_acknowledgement_materialized: false,
    final_operator_acknowledgement_filesystem_written: false,
    final_operator_acknowledgement_delivered: false,
    operator_received_recorded: false,
    operator_confirmed_recorded: false,
    operator_read_recorded: false,
    operator_seen_recorded: false,
    final_response_recorded: false,
    completion_acknowledgement_recorded: false,
    status_acknowledgement_recorded: false,
    summary_acknowledgement_recorded: false,
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
    activation_from_acknowledgement_allowed: false,
    live_execution_from_acknowledgement_allowed: false,
    download_link_from_acknowledgement_rendered: false,
    install_command_from_acknowledgement_rendered: false,
    install_from_acknowledgement_executed: false,
    service_restart_from_acknowledgement_performed: false,
    launchd_from_acknowledgement_mutated: false,
    active_binary_from_acknowledgement_mutated: false,
    result_receipt_from_acknowledgement_recorded: false,
    result_receipt_from_acknowledgement_persisted: false,
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
    final_blocker_count: 84,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_decision_status_promotion_without_acknowledgement",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-final-acknowledgement-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_FINAL_ACKNOWLEDGEMENT_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_final_acknowledgement_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-final-acknowledgement-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
