#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability-final-index-artifact-signing-terminal-public-claim-delivery-receipt-summary-briefing-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt summary/briefing attachment report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the terminal public claim delivery receipt summary/briefing readback report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_attachment_blocked == true
  and .operator_summary_recorded == false
  and .operator_briefing_recorded == false
  and .telegram_briefing_delivered == false
  and .activation_authority_from_summary_briefing_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate_invoked: false,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    terminal_public_claim_delivery_receipt_summary_briefing_accepted: false,
    terminal_public_claim_delivery_receipt_summary_briefing_recorded: false,
    terminal_public_claim_delivery_receipt_summary_briefing_persisted: false,
    terminal_public_claim_delivery_receipt_summary_briefing_materialized: false,
    terminal_public_claim_delivery_receipt_summary_briefing_filesystem_written: false,
    operator_summary_recorded: false,
    operator_summary_persisted: false,
    operator_briefing_recorded: false,
    operator_briefing_persisted: false,
    delivery_receipt_readback_recorded: false,
    delivery_receipt_readback_persisted: false,
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
    delivery_receipt_result_receipt_recorded: false,
    delivery_receipt_completion_ack_recorded: false,
    operator_approval_from_summary_derived: false,
    operator_approval_from_briefing_derived: false,
    release_publication_authority_from_summary_briefing_derived: false,
    activation_authority_from_summary_briefing_derived: false,
    install_from_summary_briefing_executed: false,
    service_restart_from_summary_briefing_performed: false,
    active_binary_from_summary_briefing_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    readback_check_count: 106,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_final_index_without_observability",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-summary-briefing-readback-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_SUMMARY_BRIEFING_READBACK_2026-06-21.md",
    source_files: {
      terminal_public_claim_delivery_receipt_summary_briefing_attachment_report: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability-final-index-artifact-signing-terminal-public-claim-delivery-receipt-summary-briefing-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
