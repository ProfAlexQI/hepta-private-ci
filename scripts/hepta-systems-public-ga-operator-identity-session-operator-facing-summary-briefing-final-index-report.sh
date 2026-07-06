#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-operator-facing-summary-briefing-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session summary briefing readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session summary briefing final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_facing_summary_briefing_readback"
  and .public_ga_operator_identity_session_operator_facing_summary_briefing_readback_ready == true
  and .public_ga_operator_identity_session_operator_facing_summary_briefing_readback_blocked == true
  and .operator_summary_recorded == false
  and .operator_briefing_recorded == false
  and .briefing_delivery_performed == false
  and .summary_briefing_acceptance_recorded == false
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
    surface: "public_ga_operator_identity_session_operator_facing_summary_briefing_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_facing_summary_briefing_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_facing_summary_briefing_readback_ready: $source.public_ga_operator_identity_session_operator_facing_summary_briefing_readback_ready,
    source_public_ga_operator_identity_session_operator_facing_summary_briefing_readback_blocked: $source.public_ga_operator_identity_session_operator_facing_summary_briefing_readback_blocked,
    public_ga_operator_identity_session_operator_facing_summary_briefing_final_index_ready: true,
    public_ga_operator_identity_session_operator_facing_summary_briefing_final_index_blocked: true,
    public_ga_operator_identity_session_operator_facing_summary_briefing_readback_attached: true,
    public_ga_operator_identity_session_export_query_observability_final_index_attached: $source.public_ga_operator_identity_session_export_query_observability_final_index_attached,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    operator_identity_session_operator_facing_summary_briefing_non_persistence_gate_present: $source.operator_identity_session_operator_facing_summary_briefing_non_persistence_gate_present,
    operator_identity_session_operator_facing_summary_briefing_non_persistence_doc_present: $source.operator_identity_session_operator_facing_summary_briefing_non_persistence_doc_present,
    operator_identity_session_operator_facing_summary_briefing_non_persistence_gate_invoked: false,
    operator_identity_session_export_query_observability_denial_gate_invoked: false,
    long_soak_required_by_source_summary_briefing_gate: $source.long_soak_required_by_source_summary_briefing_gate,
    long_soak_started: false,
    public_ga_operator_packet_required_approval_static_count: $source.public_ga_operator_packet_required_approval_static_count,
    public_ga_operator_approval_packet_invoked: false,
    public_ga_operator_packet_sent: false,
    operator_approval_request_sent: false,
    operator_approval_recorded: false,
    operator_approval_accepted: false,
    query_registered: false,
    export_file_written: false,
    observability_metric_recorded: false,
    operator_summary_recorded: false,
    operator_summary_persisted: false,
    operator_briefing_recorded: false,
    operator_briefing_persisted: false,
    readback_digest_recorded: false,
    status_banner_recorded: false,
    exported_summary_text_recorded: false,
    exported_summary_text_persisted: false,
    operator_briefing_card_materialized: false,
    notification_recorded: false,
    timeline_recorded: false,
    dashboard_narrative_recorded: false,
    audit_narrative_recorded: false,
    briefing_delivery_recorded: false,
    briefing_delivery_performed: false,
    approval_summary_recorded: false,
    external_briefing_sent: false,
    telegram_briefing_sent: false,
    summary_briefing_acceptance_recorded: false,
    result_receipt_from_summary_briefing_recorded: false,
    result_receipt_from_summary_briefing_persisted: false,
    completion_ack_recorded: false,
    operator_approval_from_summary_briefing_accepted: false,
    release_publication_authority_from_summary_briefing_derived: false,
    activation_authority_from_summary_briefing_derived: false,
    download_link_from_summary_briefing_rendered: false,
    install_command_from_summary_briefing_rendered: false,
    install_from_summary_briefing_executed: false,
    service_restart_from_summary_briefing_performed: false,
    active_binary_from_summary_briefing_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    final_blocker_count: 42,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_facing_summary_briefing_final_index_to_public_ga_operator_identity_session_final_acknowledgement_non_acceptance_without_summary",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-operator-facing-summary-briefing-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_OPERATOR_FACING_SUMMARY_BRIEFING_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_operator_facing_summary_briefing_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-operator-facing-summary-briefing-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
