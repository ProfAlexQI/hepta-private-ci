#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-export-query-observability-final-index-report.sh"
SUMMARY_BRIEFING_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-summary-briefing-non-persistence-denial-gate.sh"
SUMMARY_BRIEFING_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence export query observability final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$SUMMARY_BRIEFING_GATE" ]] || {
  echo "missing operator identity/session intent consent evidence summary briefing non-persistence gate: $SUMMARY_BRIEFING_GATE" >&2
  exit 1
}
[[ -f "$SUMMARY_BRIEFING_DOC" ]] || {
  echo "missing operator identity/session intent consent evidence summary briefing non-persistence doc: $SUMMARY_BRIEFING_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence summary briefing report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index_blocked == true
  and .evidence_exported == false
  and .evidence_query_registered == false
  and .evidence_observability_recorded == false
  and .evidence_export_query_observability_acceptance_recorded == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
' <<<"$source_json" >/dev/null

summary_briefing_static_mention_count="$(
  grep -Ec 'summary|briefing|readback|status|notification|delivery|acknowledgement|approval|authority|telegram|external|install|restart|active-binary' "$SUMMARY_BRIEFING_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson summary_briefing_static_mention_count "$summary_briefing_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index_attached: true,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_attachment_blocked: true,
    operator_identity_session_operator_intent_consent_evidence_summary_briefing_non_persistence_gate_present: true,
    operator_identity_session_operator_intent_consent_evidence_summary_briefing_non_persistence_doc_present: true,
    operator_identity_session_operator_intent_consent_evidence_summary_briefing_static_mention_count: $summary_briefing_static_mention_count,
    operator_identity_session_operator_intent_consent_evidence_summary_briefing_non_persistence_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_export_query_observability_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_summary_briefing_gate: true,
    long_soak_started: false,
    intent_consent_evidence_recorded: false,
    intent_consent_evidence_persisted: false,
    evidence_exported: false,
    evidence_query_registered: false,
    evidence_observability_recorded: false,
    evidence_operator_readback_recorded: false,
    evidence_audit_view_recorded: false,
    summary_requested: false,
    briefing_requested: false,
    operator_summary_recorded: false,
    operator_summary_persisted: false,
    operator_briefing_recorded: false,
    operator_briefing_persisted: false,
    readback_digest_recorded: false,
    status_banner_recorded: false,
    exported_summary_text_recorded: false,
    exported_summary_text_persisted: false,
    operator_briefing_card_materialized: false,
    notification_timeline_recorded: false,
    dashboard_narrative_recorded: false,
    audit_narrative_recorded: false,
    briefing_delivery_recorded: false,
    briefing_delivery_performed: false,
    final_summary_recorded: false,
    operator_memo_recorded: false,
    approval_summary_recorded: false,
    external_briefing_sent: false,
    telegram_briefing_sent: false,
    authority_briefing_recorded: false,
    live_status_briefing_recorded: false,
    summary_briefing_acceptance_recorded: false,
    result_receipt_from_summary_briefing_recorded: false,
    result_receipt_from_summary_briefing_persisted: false,
    final_operator_acknowledgement_accepted: false,
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
    attachment_blocker_count: 54,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_readback_without_evidence_export",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-export-query-observability-final-index-summary-briefing-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_EXPORT_QUERY_OBSERVABILITY_FINAL_INDEX_SUMMARY_BRIEFING_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_export_query_observability_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-export-query-observability-final-index-report.sh",
      operator_identity_session_intent_consent_evidence_summary_briefing_non_persistence_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-summary-briefing-non-persistence-denial-gate.sh",
      operator_identity_session_intent_consent_evidence_summary_briefing_non_persistence_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_SUMMARY_BRIEFING_NON_PERSISTENCE_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      public_ga_operator_approval_packet_invoked: false,
      operator_approval_request_sent: false,
      operator_approval_recorded: false,
      operator_approval_accepted: false,
      operator_identity_session_operator_intent_consent_evidence_summary_briefing_non_persistence_gate_invoked: false,
      operator_identity_session_operator_intent_consent_evidence_export_query_observability_denial_gate_invoked: false,
      intent_consent_evidence_recorded: false,
      intent_consent_evidence_persisted: false,
      evidence_exported: false,
      evidence_query_registered: false,
      evidence_observability_recorded: false,
      operator_summary_recorded: false,
      operator_summary_persisted: false,
      operator_briefing_recorded: false,
      operator_briefing_persisted: false,
      readback_digest_recorded: false,
      status_banner_recorded: false,
      exported_summary_text_recorded: false,
      operator_briefing_card_materialized: false,
      notification_timeline_recorded: false,
      dashboard_narrative_recorded: false,
      audit_narrative_recorded: false,
      briefing_delivery_recorded: false,
      briefing_delivery_performed: false,
      final_summary_recorded: false,
      operator_memo_recorded: false,
      approval_summary_recorded: false,
      external_briefing_sent: false,
      telegram_briefing_sent: false,
      authority_briefing_recorded: false,
      live_status_briefing_recorded: false,
      summary_briefing_acceptance_recorded: false,
      result_receipt_recorded: false,
      final_operator_acknowledgement_accepted: false,
      completion_ack_recorded: false,
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
