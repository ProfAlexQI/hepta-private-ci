#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-signing-receipt-export-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt artifact signing receipt signing receipt export/query/observability readback report: $SOURCE_REPORT" >&2
  exit 1
}

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_readback_blocked == true
  and .readback_blocker_count == 160
  and .signing_receipt_query_registered == false
  and .signing_receipt_export_file_written == false
  and .signing_receipt_observability_metric_recorded == false
  and .signing_receipt_delivery_observability_recorded == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_retention_expiry_gc_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_retention_expiry_gc_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_retention_expiry_gc_denial_gate_invoked: false,
    signing_receipt_export_query_observability_accepted: false,
    signing_receipt_export_query_observability_recorded: false,
    signing_receipt_export_query_observability_persisted: false,
    signing_receipt_export_query_observability_materialized: false,
    signing_receipt_export_query_observability_filesystem_written: false,
    signing_receipt_query_registered: false,
    signing_receipt_query_executed: false,
    signing_receipt_query_result_recorded: false,
    signing_receipt_search_index_recorded: false,
    signing_receipt_export_accepted: false,
    signing_receipt_export_snapshot_recorded: false,
    signing_receipt_export_file_written: false,
    signing_receipt_export_stream_opened: false,
    signing_receipt_observability_metric_recorded: false,
    signing_receipt_observability_log_recorded: false,
    signing_receipt_observability_trace_recorded: false,
    signing_receipt_observability_event_recorded: false,
    signing_receipt_dashboard_panel_recorded: false,
    signing_receipt_alert_registered: false,
    signing_receipt_slo_recorded: false,
    signing_receipt_readback_surface_recorded: false,
    signing_receipt_audit_view_recorded: false,
    signing_receipt_ledger_observability_recorded: false,
    signing_receipt_index_observability_recorded: false,
    signing_receipt_delivery_observability_recorded: false,
    external_signing_receipt_observability_recorded: false,
    telegram_signing_receipt_observability_recorded: false,
    operator_approval_from_signing_receipt_export_query_observability_derived: false,
    release_publication_authority_from_signing_receipt_export_query_observability_derived: false,
    activation_authority_from_signing_receipt_export_query_observability_derived: false,
    install_from_signing_receipt_export_query_observability_executed: false,
    service_restart_from_signing_receipt_export_query_observability_performed: false,
    active_binary_from_signing_receipt_export_query_observability_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    final_blocker_count: 160,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_export_query_observability_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_artifact_signing_receipt_signing_receipt_summary_briefing_without_observability",
    side_effect_free: true,
    side_effects: ($source.side_effects + {
      final_index_report_written: false,
      signing_receipt_export_query_observability_final_index_recorded: false,
      signing_receipt_export_denial_gate_invoked: false
    })
  }'
