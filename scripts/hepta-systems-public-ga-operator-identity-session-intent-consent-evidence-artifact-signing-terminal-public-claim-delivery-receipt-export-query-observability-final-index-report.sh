#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable terminal public claim delivery receipt export/query/observability readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the terminal public claim delivery receipt export/query/observability final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_blocked == true
  and .query_registered == false
  and .export_file_written == false
  and .observability_metric_recorded == false
  and .delivery_observability_recorded == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_final_index_attached,
    artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate_present,
    artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_doc_present: $source.artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_doc_present,
    artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_denial_gate_invoked: false,
    artifact_signing_terminal_public_claim_delivery_receipt_retention_expiry_gc_denial_gate_invoked: false,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    terminal_public_claim_delivery_receipt_export_query_observability_accepted: false,
    terminal_public_claim_delivery_receipt_export_query_observability_recorded: false,
    terminal_public_claim_delivery_receipt_export_query_observability_persisted: false,
    terminal_public_claim_delivery_receipt_export_query_observability_materialized: false,
    terminal_public_claim_delivery_receipt_export_query_observability_filesystem_written: false,
    query_registered: false,
    query_executed: false,
    query_result_recorded: false,
    search_index_recorded: false,
    export_accepted: false,
    export_snapshot_recorded: false,
    export_file_written: false,
    export_stream_opened: false,
    observability_metric_recorded: false,
    observability_log_recorded: false,
    observability_trace_recorded: false,
    observability_event_recorded: false,
    dashboard_panel_recorded: false,
    alert_registered: false,
    slo_recorded: false,
    operator_summary_recorded: false,
    readback_surface_recorded: false,
    audit_view_recorded: false,
    ledger_observability_recorded: false,
    index_observability_recorded: false,
    delivery_observability_recorded: false,
    result_receipt_recorded: false,
    completion_ack_recorded: false,
    operator_approval_from_export_query_observability_derived: false,
    release_publication_authority_from_export_query_observability_derived: false,
    activation_authority_from_export_query_observability_derived: false,
    install_from_export_query_observability_executed: false,
    service_restart_from_export_query_observability_performed: false,
    active_binary_from_export_query_observability_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    final_blocker_count: 104,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_export_query_observability_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_terminal_public_claim_delivery_receipt_summary_briefing_without_observability",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_TERMINAL_PUBLIC_CLAIM_DELIVERY_RECEIPT_EXPORT_QUERY_OBSERVABILITY_FINAL_INDEX_2026-06-21.md",
    source_files: {
      terminal_public_claim_delivery_receipt_export_query_observability_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-terminal-public-claim-delivery-receipt-export-query-observability-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
