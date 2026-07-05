#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
READBACK_REPORT="$ROOT/scripts/hepta-systems-terminal-public-ga-non-promotion-summary-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLIC_GA_NON_PROMOTION_SUMMARY_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-public-ga-non-promotion-summary-final-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$READBACK_REPORT" ]] || fail "missing executable terminal Public GA non-promotion summary readback report: $READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing terminal Public GA non-promotion summary final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal Public GA non-promotion summary final index report"
fi

jq -n \
  --slurpfile readback <("$READBACK_REPORT") \
  --arg gate "scripts/hepta-systems-terminal-public-ga-non-promotion-summary-final-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_PUBLIC_GA_NON_PROMOTION_SUMMARY_FINAL_INDEX_2026-06-21.md" \
  '
  ($readback[0]) as $readback |
  ($readback.terminal_public_ga_non_promotion_summary_readback_ready == true
    and $readback.terminal_public_ga_non_promotion_summary_readback_blocked == true
    and $readback.public_ga_readiness_non_live_attachment_final_index_attached == true
    and $readback.public_claim_non_promotion_denial_gate_invoked == false
    and $readback.public_ga_operator_approval_packet_invoked == false
    and $readback.public_ga_operator_packet_live_endpoint_read_performed == false
    and $readback.public_ga_readiness_script_invoked == false
    and $readback.public_ga_readiness_live_endpoint_read_performed == false
    and $readback.public_ga_readiness_endpoint_curl_performed == false
    and $readback.public_ga_readiness_attachment_allowed == false
    and $readback.public_ga_claim_allowed == false
    and $readback.public_ga_claimed == false
    and ($readback.side_effects | to_entries | all(.value == false))) as $final_index_ready |
  {
    runtime:"hepta",
    surface:"terminal_public_ga_non_promotion_summary_final_index",
    plugin_id:$readback.plugin_id,
    status:(if $final_index_ready then "ready_blocked" else "blocked" end),
    source_terminal_public_ga_non_promotion_summary_readback_surface:$readback.surface,
    source_terminal_public_ga_non_promotion_summary_readback_ready:$readback.terminal_public_ga_non_promotion_summary_readback_ready,
    source_terminal_public_ga_non_promotion_summary_readback_blocked:$readback.terminal_public_ga_non_promotion_summary_readback_blocked,
    source_terminal_denial_index_attachment_final_index_surface:$readback.source_terminal_denial_index_attachment_final_index_surface,
    source_terminal_denial_index_attachment_final_index_ready:$readback.source_terminal_denial_index_attachment_final_index_ready,
    source_terminal_denial_index_attachment_final_index_blocked:$readback.source_terminal_denial_index_attachment_final_index_blocked,
    source_canonical_governance_tool_execution_closure_backfeed_ready:$readback.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count:$readback.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count:$readback.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count:$readback.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count:$readback.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready:$readback.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories:$readback.source_canonical_governance_tool_execution_closure_backfeed_categories,
    terminal_public_ga_non_promotion_summary_final_index_ready:$final_index_ready,
    terminal_public_ga_non_promotion_summary_final_index_blocked:true,
    terminal_public_ga_non_promotion_summary_readback_attached:true,
    public_ga_readiness_non_live_attachment_final_index_attached:true,
    public_claim_non_promotion_denial_gate_present:true,
    public_claim_non_promotion_denial_doc_present:true,
    public_claim_non_promotion_denial_gate_invoked:false,
    public_ga_operator_approval_packet_present:true,
    public_ga_operator_approval_packet_doc_present:true,
    public_ga_operator_approval_packet_invoked:false,
    public_ga_operator_packet_live_endpoint_read_performed:false,
    public_ga_readiness_script_invoked:false,
    public_ga_readiness_live_endpoint_read_performed:false,
    public_ga_readiness_endpoint_curl_performed:false,
    public_ga_readiness_report_materialized:false,
    public_ga_readiness_attachment_recorded:false,
    public_ga_readiness_attachment_allowed:false,
    terminal_publication_evidence_non_persistence_summary_gate_invoked:false,
    hepta_watchdog_invoked:false,
    terminal_public_distribution_non_publication_lock_gate_invoked:false,
    terminal_denial_index_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    final_blocker_count:$readback.summary_blocker_count,
    manual_operator_live_cutover_approval_required:true,
    terminal_live_url_required:false,
    long_soak_required:false,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    public_distribution_publication_allowed:false,
    public_distribution_artifact_write_allowed:false,
    public_release_claim_allowed:false,
    public_ga_claim_allowed:false,
    public_release_published:false,
    public_ga_claimed:false,
    publication_evidence_summary_recorded:false,
    publication_evidence_summary_persisted:false,
    publication_evidence_receipt_persisted:false,
    publication_evidence_ledger_persisted:false,
    operator_approval_recorded:false,
    operator_identity_accepted:false,
    rollback_execution_allowed:false,
    next_migration_step:"attach_terminal_public_ga_non_promotion_summary_final_index_to_public_ga_operator_packet_non_send_readback_without_packet_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_public_ga_non_promotion_summary_readback_report:"scripts/hepta-systems-terminal-public-ga-non-promotion-summary-readback-report.sh"
    },
    side_effect_free:true,
    side_effects:$readback.side_effects
  }'
