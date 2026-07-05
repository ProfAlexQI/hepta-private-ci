#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-export-final-index-delivery-receipt-signing-receipt-summary-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt summary/briefing attachment report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment_blocked == true
  and .attachment_blocker_count == 162
  and .signing_receipt_operator_summary_recorded == false
  and .signing_receipt_operator_briefing_recorded == false
  and .telegram_signing_receipt_briefing_sent == false
  and .release_publication_authority_from_signing_receipt_summary_briefing_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_readback_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_readback_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_attachment_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_denial_gate_invoked: false,
    signing_receipt_summary_briefing_accepted: false,
    signing_receipt_summary_briefing_recorded: false,
    signing_receipt_summary_briefing_persisted: false,
    signing_receipt_summary_briefing_materialized: false,
    signing_receipt_summary_briefing_filesystem_written: false,
    signing_receipt_operator_summary_recorded: false,
    signing_receipt_operator_summary_persisted: false,
    signing_receipt_operator_briefing_recorded: false,
    signing_receipt_operator_briefing_persisted: false,
    signing_receipt_readback_digest_recorded: false,
    signing_receipt_readback_digest_persisted: false,
    signing_receipt_status_banner_recorded: false,
    signing_receipt_exported_summary_written: false,
    signing_receipt_briefing_card_recorded: false,
    signing_receipt_notification_timeline_recorded: false,
    signing_receipt_dashboard_narrative_recorded: false,
    signing_receipt_audit_narrative_recorded: false,
    signing_receipt_approval_summary_recorded: false,
    signing_receipt_final_summary_recorded: false,
    signing_receipt_operator_memo_recorded: false,
    signing_receipt_completion_summary_recorded: false,
    signing_receipt_channel_briefing_recorded: false,
    external_signing_receipt_briefing_sent: false,
    telegram_signing_receipt_briefing_sent: false,
    release_publication_authority_from_signing_receipt_summary_briefing_derived: false,
    activation_authority_from_signing_receipt_summary_briefing_derived: false,
    install_from_signing_receipt_summary_briefing_executed: false,
    service_restart_from_signing_receipt_summary_briefing_performed: false,
    active_binary_from_signing_receipt_summary_briefing_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    readback_blocker_count: 162,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_final_index_without_observability",
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
