#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
READBACK_REPORT="$ROOT/scripts/hepta-systems-current-canonical-governance-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_GOVERNANCE_TERMINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-current-canonical-governance-terminal-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$READBACK_REPORT" ]] || fail "missing executable current canonical governance readback report: $READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing current canonical governance terminal index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the current canonical governance terminal index report"
fi

jq -n \
  --slurpfile readback <("$READBACK_REPORT") \
  --arg gate "scripts/hepta-systems-current-canonical-governance-terminal-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_GOVERNANCE_TERMINAL_INDEX_2026-06-21.md" \
  '
  ($readback[0]) as $readback |
  [
    {
      id:"current_canonical_governance_readback",
      surface:$readback.surface,
      source_ready:$readback.current_canonical_governance_readback_ready,
      terminal_role:"canonical_governance_blocker_source",
      invoked_by_report:false
    },
    {
      id:"active_current_canonical_consumer",
      surface:$readback.active_current_canonical_consumer_surface,
      source_ready:true,
      terminal_role:"rollback_anchor",
      invoked_by_report:false
    },
    {
      id:"successor_cutover_final_gate",
      surface:"promoted_current_canonical_consumer_cutover_final_gate",
      source_ready:$readback.successor_cutover_final_gate_attached,
      terminal_role:"manual_cutover_blocker",
      invoked_by_report:false
    }
  ] as $terminal_inputs |
  [
    "manual_operator_live_cutover_approval_required",
    "explicit_live_cutover_approval_missing",
    "cutover_packet_not_recorded",
    "cutover_packet_not_accepted",
    "successor_consumer_cutover_disallowed",
    "current_canonical_consumer_rollback_anchor_retained",
    "canonical_gate_not_invoked",
    "wrapper_target_not_invoked",
    "terminal_live_gate_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "execution_disabled",
    "public_ga_disabled"
  ] as $terminal_blockers |
  ($readback.current_canonical_governance_readback_ready == true
    and $readback.current_canonical_governance_readback_blocked == true
    and $readback.tool_execution_closure_backfeed_ready == true
    and $readback.tool_execution_closure_backfeed_blocker_count == 17
    and $readback.tool_execution_closure_backfeed_category_count == 4
    and $readback.tool_execution_closure_backfeed_category_ready_count == 4
    and $readback.tool_execution_closure_backfeed_category_blocker_count == 17
    and $readback.tool_execution_closure_backfeed_categorization_ready == true
    and $readback.source_current_canonical_governance_report_reexecuted == false
    and $readback.successor_cutover_final_gate_attached == true
    and $readback.successor_cutover_final_gate_status == "ready_blocked"
    and $readback.successor_consumer_cutover_allowed == false
    and $readback.rollback_anchor == "current_canonical_consumer"
    and $readback.active_current_canonical_consumer_replaced_in_place == false
    and $readback.cutover_packet_recorded == false
    and $readback.cutover_packet_accepted == false
    and $readback.operator_live_cutover_approval_recorded == false
    and $readback.execution_enabled_count == 0
    and $readback.public_ga_enabled_count == 0
    and $readback.canonical_gate_wrapper_invoked == false
    and $readback.wrapper_target_invoked == false
    and $readback.terminal_live_gate_invoked == false
    and $readback.tool_execution_live_cutover_allowed == false
    and $readback.tool_execution_public_ga_allowed == false
    and ($terminal_inputs | all(.source_ready == true and .invoked_by_report == false))
    and ($readback.side_effects | to_entries | all(.value == false))) as $terminal_index_ready |
  {
    runtime:"hepta",
    surface:"current_canonical_governance_terminal_index",
    plugin_id:$readback.plugin_id,
    status:(if $terminal_index_ready then "ready_blocked" else "blocked" end),
    source_governance_readback_surface:$readback.surface,
    source_governance_readback_ready:$readback.current_canonical_governance_readback_ready,
    source_governance_readback_blocked:$readback.current_canonical_governance_readback_blocked,
    source_tool_execution_closure_surface:$readback.source_tool_execution_closure_surface,
    source_tool_execution_closure_ready:$readback.source_tool_execution_closure_ready,
    tool_execution_closure_backfeed_ready:$readback.tool_execution_closure_backfeed_ready,
    tool_execution_closure_backfeed_blocker_count:$readback.tool_execution_closure_backfeed_blocker_count,
    tool_execution_closure_backfeed_category_count:$readback.tool_execution_closure_backfeed_category_count,
    tool_execution_closure_backfeed_category_ready_count:$readback.tool_execution_closure_backfeed_category_ready_count,
    tool_execution_closure_backfeed_category_blocker_count:$readback.tool_execution_closure_backfeed_category_blocker_count,
    tool_execution_closure_backfeed_categorization_ready:$readback.tool_execution_closure_backfeed_categorization_ready,
    tool_execution_closure_backfeed_categories:$readback.tool_execution_closure_backfeed_categories,
    current_canonical_governance_terminal_index_ready:$terminal_index_ready,
    current_canonical_governance_terminal_index_blocked:true,
    terminal_input_count:($terminal_inputs | length),
    terminal_inputs:$terminal_inputs,
    active_current_canonical_consumer_surface:$readback.active_current_canonical_consumer_surface,
    active_current_canonical_consumer_replaced_in_place:false,
    successor_canonical_consumer_surface:$readback.successor_canonical_consumer_surface,
    successor_cutover_final_gate_attached:true,
    successor_cutover_final_gate_status:$readback.successor_cutover_final_gate_status,
    successor_consumer_cutover_allowed:false,
    rollback_anchor:$readback.rollback_anchor,
    manual_operator_live_cutover_approval_required:true,
    explicit_live_cutover_approval_present:false,
    operator_live_cutover_approval_recorded:false,
    cutover_packet_recorded:false,
    cutover_packet_accepted:false,
    final_blocker_count:$readback.final_blocker_count,
    terminal_blocker_count:($terminal_blockers | length),
    terminal_blockers:$terminal_blockers,
    execution_enabled_count:0,
    public_ga_enabled_count:0,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    capability_matrix_gate_invoked:false,
    terminal_live_gate_invoked:false,
    live_url_required:false,
    long_soak_required:false,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    next_migration_step:"attach_current_canonical_governance_terminal_index_to_tool_execution_terminal_governance_bridge_without_live_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      current_canonical_governance_readback_report:"scripts/hepta-systems-current-canonical-governance-readback-report.sh"
    },
    side_effect_free:true,
    side_effects:$readback.side_effects
  }'
