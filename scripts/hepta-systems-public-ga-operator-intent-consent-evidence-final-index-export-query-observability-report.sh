#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-persistence-final-index-report.sh"
EXPORT_QUERY_OBSERVABILITY_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-export-query-observability-denial-gate.sh"
EXPORT_QUERY_OBSERVABILITY_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_EXPORT_QUERY_OBSERVABILITY_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$EXPORT_QUERY_OBSERVABILITY_GATE" ]] || {
  echo "missing operator identity/session intent consent evidence export query observability denial gate: $EXPORT_QUERY_OBSERVABILITY_GATE" >&2
  exit 1
}
[[ -f "$EXPORT_QUERY_OBSERVABILITY_DOC" ]] || {
  echo "missing operator identity/session intent consent evidence export query observability denial doc: $EXPORT_QUERY_OBSERVABILITY_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence export query observability report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_persistence_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_persistence_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_persistence_final_index_blocked == true
  and .intent_consent_evidence_recorded == false
  and .evidence_receipt_recorded == false
  and .evidence_filesystem_written == false
  and .evidence_ledger_written == false
  and .operator_approval_from_evidence_derived == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
' <<<"$source_json" >/dev/null

export_query_observability_static_mention_count="$(
  grep -Ec 'evidence|export|query|observability|dashboard|readback|audit|authority|summary|telegram|external|install|restart|active-binary' "$EXPORT_QUERY_OBSERVABILITY_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson export_query_observability_static_mention_count "$export_query_observability_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_persistence_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_persistence_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_persistence_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_persistence_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_persistence_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_persistence_final_index_attached: true,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_attachment_blocked: true,
    operator_identity_session_operator_intent_consent_evidence_export_query_observability_denial_gate_present: true,
    operator_identity_session_operator_intent_consent_evidence_export_query_observability_denial_doc_present: true,
    operator_identity_session_operator_intent_consent_evidence_export_query_observability_static_mention_count: $export_query_observability_static_mention_count,
    operator_identity_session_operator_intent_consent_evidence_export_query_observability_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_persistence_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_export_query_observability_gate: true,
    long_soak_started: false,
    operator_intent_evidence_recorded: false,
    operator_consent_evidence_recorded: false,
    intent_consent_evidence_recorded: false,
    intent_consent_evidence_persisted: false,
    evidence_receipt_recorded: false,
    evidence_receipt_persisted: false,
    evidence_materialized: false,
    evidence_filesystem_written: false,
    evidence_ledger_written: false,
    evidence_index_recorded: false,
    evidence_export_requested: false,
    evidence_export_accepted: false,
    evidence_exported: false,
    evidence_export_snapshot_recorded: false,
    evidence_export_snapshot_persisted: false,
    evidence_export_file_written: false,
    evidence_export_stream_opened: false,
    evidence_query_requested: false,
    evidence_query_registered: false,
    evidence_query_executed: false,
    evidence_query_result_recorded: false,
    evidence_query_result_persisted: false,
    evidence_search_index_recorded: false,
    evidence_search_index_persisted: false,
    evidence_observability_requested: false,
    evidence_observability_recorded: false,
    evidence_metric_recorded: false,
    evidence_log_recorded: false,
    evidence_trace_recorded: false,
    evidence_event_recorded: false,
    evidence_dashboard_panel_recorded: false,
    evidence_alert_registered: false,
    evidence_slo_recorded: false,
    evidence_operator_readback_recorded: false,
    evidence_readback_recorded: false,
    evidence_audit_view_recorded: false,
    evidence_external_observability_recorded: false,
    evidence_telegram_observability_recorded: false,
    evidence_authority_view_recorded: false,
    evidence_live_install_view_recorded: false,
    evidence_export_query_observability_acceptance_recorded: false,
    operator_summary_recorded: false,
    operator_briefing_recorded: false,
    identity_session_binding_from_evidence_recorded: false,
    operator_approval_from_evidence_derived: false,
    acceptance_from_evidence_recorded: false,
    terminal_decision_from_evidence_recorded: false,
    terminal_status_from_evidence_recorded: false,
    release_publication_authority_from_evidence_derived: false,
    activation_authority_from_evidence_derived: false,
    download_link_from_evidence_rendered: false,
    install_command_from_evidence_rendered: false,
    install_from_evidence_executed: false,
    service_restart_from_evidence_performed: false,
    active_binary_from_evidence_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 52,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_readback_without_evidence",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-final-index-export-query-observability-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_FINAL_INDEX_EXPORT_QUERY_OBSERVABILITY_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_persistence_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-persistence-final-index-report.sh",
      operator_identity_session_intent_consent_evidence_export_query_observability_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-export-query-observability-denial-gate.sh",
      operator_identity_session_intent_consent_evidence_export_query_observability_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_EXPORT_QUERY_OBSERVABILITY_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      public_ga_operator_approval_packet_invoked: false,
      operator_approval_request_sent: false,
      operator_approval_recorded: false,
      operator_approval_accepted: false,
      operator_identity_session_operator_intent_consent_evidence_export_query_observability_denial_gate_invoked: false,
      operator_identity_session_operator_intent_consent_evidence_persistence_denial_gate_invoked: false,
      operator_intent_evidence_recorded: false,
      operator_consent_evidence_recorded: false,
      intent_consent_evidence_recorded: false,
      intent_consent_evidence_persisted: false,
      evidence_receipt_recorded: false,
      evidence_receipt_persisted: false,
      evidence_materialized: false,
      evidence_filesystem_written: false,
      evidence_ledger_written: false,
      evidence_index_recorded: false,
      evidence_exported: false,
      evidence_export_snapshot_persisted: false,
      evidence_export_file_written: false,
      evidence_export_stream_opened: false,
      evidence_query_registered: false,
      evidence_query_executed: false,
      evidence_query_result_recorded: false,
      evidence_search_index_persisted: false,
      evidence_observability_recorded: false,
      evidence_metric_recorded: false,
      evidence_log_recorded: false,
      evidence_trace_recorded: false,
      evidence_event_recorded: false,
      evidence_dashboard_panel_recorded: false,
      evidence_alert_registered: false,
      evidence_slo_recorded: false,
      evidence_operator_readback_recorded: false,
      evidence_audit_view_recorded: false,
      evidence_external_observability_recorded: false,
      evidence_telegram_observability_recorded: false,
      operator_summary_recorded: false,
      operator_briefing_recorded: false,
      operator_approval_from_evidence_derived: false,
      release_publication_authority_derived: false,
      activation_authority_derived: false,
      download_link_rendered: false,
      install_command_rendered: false,
      install_executed: false,
      service_restarted: false,
      active_binary_mutated: false,
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
