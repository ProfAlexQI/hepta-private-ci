#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
READBACK_REPORT="$ROOT/scripts/hepta-systems-terminal-release-governance-attachment-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_RELEASE_GOVERNANCE_ATTACHMENT_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-release-governance-attachment-final-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$READBACK_REPORT" ]] || fail "missing executable terminal release governance attachment readback report: $READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing terminal release governance attachment final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal release governance attachment final index report"
fi

jq -n \
  --slurpfile readback <("$READBACK_REPORT") \
  --arg gate "scripts/hepta-systems-terminal-release-governance-attachment-final-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_RELEASE_GOVERNANCE_ATTACHMENT_FINAL_INDEX_2026-06-21.md" \
  '
  ($readback[0]) as $readback |
  [
    "manual_operator_live_cutover_approval_required",
    "terminal_release_governance_final_audit_not_invoked",
    "terminal_governance_closure_summary_gate_not_invoked",
    "terminal_summary_gates_not_invoked",
    "terminal_live_gates_not_invoked",
    "canonical_successor_consumer_cutover_disallowed",
    "current_canonical_consumer_rollback_anchor_retained",
    "canonical_gate_not_invoked",
    "wrapper_target_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "public_ga_disabled",
    "release_publication_disabled",
    "release_artifact_write_disabled",
    "public_release_claim_disabled"
  ] as $final_blockers |
  ($readback.terminal_release_governance_attachment_readback_ready == true
    and $readback.terminal_release_governance_attachment_readback_blocked == true
    and $readback.terminal_release_governance_final_audit_gate_present == true
    and $readback.terminal_release_governance_final_audit_gate_invoked == false
    and $readback.terminal_governance_closure_summary_gate_invoked == false
    and $readback.terminal_summary_gates_invoked == false
    and $readback.terminal_live_gates_invoked == false
    and $readback.canonical_gate_wrapper_invoked == false
    and $readback.wrapper_target_invoked == false
    and $readback.source_successor_consumer_cutover_allowed == false
    and $readback.source_canonical_governance_rollback_anchor == "current_canonical_consumer"
    and $readback.tool_execution_live_cutover_allowed == false
    and $readback.tool_execution_public_ga_allowed == false
    and $readback.release_publication_allowed == false
    and $readback.release_artifact_write_allowed == false
    and $readback.public_release_claim_allowed == false
    and $readback.source_canonical_governance_tool_execution_closure_backfeed_ready == true
    and $readback.source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
    and $readback.source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
    and $readback.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
    and $readback.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
    and $readback.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
    and ($readback.side_effects | to_entries | all(.value == false))) as $final_index_ready |
  {
    runtime:"hepta",
    surface:"terminal_release_governance_attachment_final_index",
    plugin_id:$readback.plugin_id,
    status:(if $final_index_ready then "ready_blocked" else "blocked" end),
    source_terminal_release_governance_attachment_readback_surface:$readback.surface,
    source_terminal_release_governance_attachment_readback_ready:$readback.terminal_release_governance_attachment_readback_ready,
    source_terminal_release_governance_attachment_readback_blocked:$readback.terminal_release_governance_attachment_readback_blocked,
    terminal_release_governance_attachment_final_index_ready:$final_index_ready,
    terminal_release_governance_attachment_final_index_blocked:true,
    terminal_governance_closure_summary_attachment_final_index_attached:true,
    terminal_release_governance_final_audit_gate_present:true,
    terminal_release_governance_final_audit_gate_invoked:false,
    terminal_governance_closure_summary_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    source_successor_consumer_cutover_allowed:false,
    source_canonical_governance_rollback_anchor:$readback.source_canonical_governance_rollback_anchor,
    source_canonical_governance_tool_execution_closure_backfeed_ready:$readback.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count:$readback.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count:$readback.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count:$readback.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count:$readback.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready:$readback.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories:$readback.source_canonical_governance_tool_execution_closure_backfeed_categories,
    final_blocker_count:($final_blockers | length),
    final_blockers:$final_blockers,
    manual_operator_live_cutover_approval_required:true,
    terminal_live_url_required:false,
    long_soak_required:false,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    release_publication_allowed:false,
    release_artifact_write_allowed:false,
    public_release_claim_allowed:false,
    next_migration_step:"attach_terminal_release_governance_attachment_final_index_to_terminal_release_artifact_non_write_lock_without_release_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_release_governance_attachment_readback_report:"scripts/hepta-systems-terminal-release-governance-attachment-readback-report.sh"
    },
    side_effect_free:true,
    side_effects:$readback.side_effects
  }'
