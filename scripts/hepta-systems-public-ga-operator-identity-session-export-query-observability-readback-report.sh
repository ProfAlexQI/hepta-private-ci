#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-retention-expiry-gc-final-index-export-query-observability-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session export query observability attachment report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session export query observability readback report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_export_query_observability_attachment"
  and .public_ga_operator_identity_session_export_query_observability_attachment_ready == true
  and .public_ga_operator_identity_session_export_query_observability_attachment_blocked == true
  and .query_registered == false
  and .export_file_written == false
  and .observability_metric_recorded == false
  and .export_query_observability_acceptance_recorded == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_export_query_observability_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_export_query_observability_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_export_query_observability_attachment_ready: $source.public_ga_operator_identity_session_export_query_observability_attachment_ready,
    source_public_ga_operator_identity_session_export_query_observability_attachment_blocked: $source.public_ga_operator_identity_session_export_query_observability_attachment_blocked,
    public_ga_operator_identity_session_export_query_observability_readback_ready: true,
    public_ga_operator_identity_session_export_query_observability_readback_blocked: true,
    public_ga_operator_identity_session_export_query_observability_attachment_attached: true,
    public_ga_operator_identity_session_retention_expiry_gc_final_index_attached: $source.public_ga_operator_identity_session_retention_expiry_gc_final_index_attached,
    readback_mode: "static_operator_identity_session_export_query_observability_snapshot_only",
    readback_check_count: 46,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    operator_identity_session_export_query_observability_denial_gate_present: $source.operator_identity_session_export_query_observability_denial_gate_present,
    operator_identity_session_export_query_observability_denial_doc_present: $source.operator_identity_session_export_query_observability_denial_doc_present,
    operator_identity_session_export_query_observability_denial_gate_invoked: false,
    operator_identity_session_retention_expiry_gc_denial_gate_invoked: false,
    long_soak_required_by_source_export_query_observability_gate: $source.long_soak_required_by_source_export_query_observability_gate,
    long_soak_started: false,
    public_ga_operator_packet_required_approval_static_count: $source.public_ga_operator_packet_required_approval_static_count,
    public_ga_operator_approval_packet_invoked: false,
    public_ga_operator_packet_sent: false,
    operator_approval_request_sent: false,
    operator_approval_recorded: false,
    operator_approval_accepted: false,
    retention_policy_recorded: false,
    expiry_timer_started: false,
    garbage_collection_scan_performed: false,
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
    export_query_observability_acceptance_recorded: false,
    result_receipt_from_export_query_observability_recorded: false,
    release_publication_authority_from_export_query_observability_derived: false,
    activation_authority_from_export_query_observability_derived: false,
    install_from_export_query_observability_executed: false,
    service_restart_from_export_query_observability_performed: false,
    active_binary_from_export_query_observability_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    readback_blocker_count: 40,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_export_query_observability_final_index_without_retention",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-export-query-observability-readback-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_EXPORT_QUERY_OBSERVABILITY_READBACK_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_export_query_observability_attachment_report: "scripts/hepta-systems-public-ga-operator-retention-expiry-gc-final-index-export-query-observability-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
