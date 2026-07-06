#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-summary-briefing-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence artifact signing summary/briefing readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence artifact signing summary/briefing final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback_blocked == true
  and .operator_summary_recorded == false
  and .operator_briefing_recorded == false
  and .telegram_briefing_delivered == false
  and .activation_authority_from_summary_briefing_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_final_index_attached,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_denial_gate_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_denial_gate_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_denial_doc_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_denial_doc_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_export_query_observability_denial_gate_invoked: false,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_accepted: false,
    artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_recorded: false,
    artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_persisted: false,
    artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_materialized: false,
    artifact_distribution_signing_notarization_receipt_operator_facing_summary_briefing_filesystem_written: false,
    operator_summary_recorded: false,
    operator_summary_persisted: false,
    operator_briefing_recorded: false,
    operator_briefing_persisted: false,
    signing_receipt_readback_recorded: false,
    signing_receipt_readback_persisted: false,
    status_banner_recorded: false,
    exported_summary_recorded: false,
    briefing_card_recorded: false,
    notification_timeline_recorded: false,
    dashboard_narrative_recorded: false,
    audit_narrative_recorded: false,
    briefing_delivery_recorded: false,
    final_summary_recorded: false,
    operator_memo_recorded: false,
    approval_summary_recorded: false,
    external_briefing_delivered: false,
    telegram_briefing_delivered: false,
    authority_briefing_recorded: false,
    live_status_briefing_recorded: false,
    signing_receipt_result_receipt_recorded: false,
    signing_receipt_completion_ack_recorded: false,
    operator_acceptance_from_summary_recorded: false,
    operator_acceptance_from_briefing_recorded: false,
    operator_approval_from_summary_derived: false,
    operator_approval_from_briefing_derived: false,
    release_publication_authority_from_summary_briefing_derived: false,
    activation_authority_from_summary_briefing_derived: false,
    download_link_from_summary_briefing_rendered: false,
    install_command_from_summary_briefing_rendered: false,
    install_from_summary_briefing_executed: false,
    service_restart_from_summary_briefing_performed: false,
    active_binary_from_summary_briefing_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    final_blocker_count: 82,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_summary_briefing_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_final_acknowledgement_non_acceptance_without_summary",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-summary-briefing-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_SUMMARY_BRIEFING_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_summary_briefing_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-summary-briefing-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
