#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-terminal-governance-bridge-canonical-attachment-terminal-denial-summary-final-index-report.sh"
FINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-terminal-governance-bridge-canonical-attachment-terminal-denial-summary-final-index-gate.sh"
TERMINAL_GOVERNANCE_CLOSURE_GATE="$ROOT/scripts/hepta-terminal-governance-closure-summary-gate.sh"
TERMINAL_GOVERNANCE_CLOSURE_DOC="$ROOT/docs/architecture/HEPTA_TERMINAL_GOVERNANCE_CLOSURE_SUMMARY_GATE.md"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_DENIAL_SUMMARY_FINAL_INDEX_TERMINAL_GOVERNANCE_CLOSURE_SUMMARY_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-denial-summary-final-index-terminal-governance-closure-summary-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$FINAL_INDEX_REPORT" ]] || fail "missing executable terminal denial summary final index report: $FINAL_INDEX_REPORT"
[[ -x "$FINAL_INDEX_GATE" ]] || fail "missing executable terminal denial summary final index gate: $FINAL_INDEX_GATE"
[[ -x "$TERMINAL_GOVERNANCE_CLOSURE_GATE" ]] || fail "missing executable terminal governance closure summary gate: $TERMINAL_GOVERNANCE_CLOSURE_GATE"
[[ -f "$TERMINAL_GOVERNANCE_CLOSURE_DOC" ]] || fail "missing terminal governance closure summary doc: $TERMINAL_GOVERNANCE_CLOSURE_DOC"
[[ -f "$DOC" ]] || fail "missing terminal governance closure summary attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal governance closure summary attachment report"
fi

jq -n \
  --slurpfile final_index <("$FINAL_INDEX_REPORT") \
  --arg gate "scripts/hepta-systems-terminal-denial-summary-final-index-terminal-governance-closure-summary-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_DENIAL_SUMMARY_FINAL_INDEX_TERMINAL_GOVERNANCE_CLOSURE_SUMMARY_2026-06-21.md" \
  '
  ($final_index[0]) as $final |
  [
    "manual_operator_live_cutover_approval_required",
    "terminal_governance_closure_summary_gate_source_probed_not_invoked",
    "terminal_summary_gates_not_invoked",
    "terminal_live_gates_not_invoked",
    "canonical_successor_consumer_cutover_disallowed",
    "current_canonical_consumer_rollback_anchor_retained",
    "canonical_gate_not_invoked",
    "wrapper_target_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "public_ga_disabled"
  ] as $attachment_blockers |
  ($final.terminal_denial_summary_attachment_final_index_ready == true
    and $final.terminal_denial_summary_attachment_final_index_blocked == true
    and $final.terminal_summary_gates_invoked == false
    and $final.terminal_live_gates_invoked == false
    and $final.canonical_gate_wrapper_invoked == false
    and $final.wrapper_target_invoked == false
    and $final.source_successor_consumer_cutover_allowed == false
    and $final.source_canonical_governance_rollback_anchor == "current_canonical_consumer"
    and $final.tool_execution_live_cutover_allowed == false
    and $final.tool_execution_public_ga_allowed == false
    and $final.source_canonical_governance_tool_execution_closure_backfeed_ready == true
    and $final.source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
    and $final.source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
    and $final.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
    and $final.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
    and $final.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
    and ($final.side_effects | to_entries | all(.value == false))) as $attachment_ready |
  {
    runtime:"hepta",
    surface:"terminal_denial_summary_final_index_terminal_governance_closure_summary",
    plugin_id:$final.plugin_id,
    status:(if $attachment_ready then "ready_blocked" else "blocked" end),
    source_terminal_denial_summary_final_index_surface:$final.surface,
    source_terminal_denial_summary_final_index_ready:$final.terminal_denial_summary_attachment_final_index_ready,
    source_terminal_denial_summary_final_index_blocked:$final.terminal_denial_summary_attachment_final_index_blocked,
    terminal_governance_closure_summary_attachment_ready:$attachment_ready,
    terminal_governance_closure_summary_attachment_blocked:true,
    terminal_denial_summary_final_index_attached:true,
    terminal_governance_closure_summary_gate_present:true,
    terminal_governance_closure_summary_doc_present:true,
    terminal_governance_closure_summary_gate_invoked:false,
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    bridge_source_count:$final.bridge_source_count,
    tool_execution_closure_attached:$final.tool_execution_closure_attached,
    current_canonical_governance_terminal_index_attached:$final.current_canonical_governance_terminal_index_attached,
    source_canonical_governance_tool_execution_closure_backfeed_ready:$final.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count:$final.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count:$final.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count:$final.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count:$final.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready:$final.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories:$final.source_canonical_governance_tool_execution_closure_backfeed_categories,
    source_active_current_canonical_consumer_surface:$final.source_active_current_canonical_consumer_surface,
    source_successor_cutover_final_gate_attached:$final.source_successor_cutover_final_gate_attached,
    source_successor_consumer_cutover_allowed:false,
    source_canonical_governance_rollback_anchor:$final.source_canonical_governance_rollback_anchor,
    attachment_blocker_count:($attachment_blockers | length),
    attachment_blockers:$attachment_blockers,
    manual_operator_live_cutover_approval_required:true,
    terminal_live_url_required:false,
    long_soak_required:false,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    next_migration_step:"derive_terminal_governance_closure_summary_attachment_readback_without_live_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_denial_summary_final_index_report:"scripts/hepta-systems-terminal-governance-bridge-canonical-attachment-terminal-denial-summary-final-index-report.sh",
      terminal_denial_summary_final_index_gate:"scripts/hepta-systems-terminal-governance-bridge-canonical-attachment-terminal-denial-summary-final-index-gate.sh",
      terminal_governance_closure_summary_gate:"scripts/hepta-terminal-governance-closure-summary-gate.sh"
    },
    side_effect_free:true,
    side_effects:$final.side_effects
  }'
