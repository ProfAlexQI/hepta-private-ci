#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
FINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-final-index-report.sh"
FINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TERMINAL_GOVERNANCE_BRIDGE_CANONICAL_ATTACHMENT_TERMINAL_DENIAL_SUMMARY_2026-06-21.md"

fail() {
  printf 'hepta-systems-terminal-governance-bridge-canonical-attachment-terminal-denial-summary-report: FAIL: %s\n' "$1" >&2
  exit 1
}

path_exists() {
  [[ -e "$1" ]]
}

bool_for() {
  if "$@"; then
    printf 'true\n'
  else
    printf 'false\n'
  fi
}

[[ -x "$FINAL_INDEX_REPORT" ]] || fail "missing executable terminal governance bridge canonical attachment final index report: $FINAL_INDEX_REPORT"
[[ -x "$FINAL_INDEX_GATE" ]] || fail "missing executable terminal governance bridge canonical attachment final index gate: $FINAL_INDEX_GATE"
[[ -f "$DOC" ]] || fail "missing terminal denial summary attachment architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal denial summary attachment report"
fi

terminal_denial_index_present="$(bool_for path_exists "$ROOT/scripts/hepta-terminal-denial-index-gate.sh")"
terminal_governance_summary_present="$(bool_for path_exists "$ROOT/scripts/hepta-terminal-governance-closure-summary-gate.sh")"
terminal_release_final_audit_present="$(bool_for path_exists "$ROOT/scripts/hepta-terminal-release-governance-final-audit-index-gate.sh")"
terminal_operator_readiness_present="$(bool_for path_exists "$ROOT/scripts/hepta-terminal-operator-readiness-non-approval-index-gate.sh")"
terminal_denial_doc_present="$(bool_for path_exists "$ROOT/docs/architecture/HEPTA_TERMINAL_DENIAL_INDEX_GATE.md")"
terminal_governance_doc_present="$(bool_for path_exists "$ROOT/docs/architecture/HEPTA_TERMINAL_GOVERNANCE_CLOSURE_SUMMARY_GATE.md")"
terminal_release_final_audit_doc_present="$(bool_for path_exists "$ROOT/docs/architecture/HEPTA_TERMINAL_RELEASE_GOVERNANCE_FINAL_AUDIT_INDEX_GATE.md")"
terminal_operator_readiness_doc_present="$(bool_for path_exists "$ROOT/docs/architecture/HEPTA_TERMINAL_OPERATOR_READINESS_NON_APPROVAL_INDEX_GATE.md")"

jq -n \
  --slurpfile final_index <("$FINAL_INDEX_REPORT") \
  --arg gate "scripts/hepta-systems-terminal-governance-bridge-canonical-attachment-terminal-denial-summary-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TERMINAL_GOVERNANCE_BRIDGE_CANONICAL_ATTACHMENT_TERMINAL_DENIAL_SUMMARY_2026-06-21.md" \
  --argjson terminal_denial_index_present "$terminal_denial_index_present" \
  --argjson terminal_governance_summary_present "$terminal_governance_summary_present" \
  --argjson terminal_release_final_audit_present "$terminal_release_final_audit_present" \
  --argjson terminal_operator_readiness_present "$terminal_operator_readiness_present" \
  --argjson terminal_denial_doc_present "$terminal_denial_doc_present" \
  --argjson terminal_governance_doc_present "$terminal_governance_doc_present" \
  --argjson terminal_release_final_audit_doc_present "$terminal_release_final_audit_doc_present" \
  --argjson terminal_operator_readiness_doc_present "$terminal_operator_readiness_doc_present" \
  '
  ($final_index[0]) as $final |
  [
    {
      id:"terminal_denial_index",
      script:"scripts/hepta-terminal-denial-index-gate.sh",
      doc:"docs/architecture/HEPTA_TERMINAL_DENIAL_INDEX_GATE.md",
      script_present:$terminal_denial_index_present,
      doc_present:$terminal_denial_doc_present,
      terminal_summary_role:"denial_index",
      invoked:false
    },
    {
      id:"terminal_governance_closure_summary",
      script:"scripts/hepta-terminal-governance-closure-summary-gate.sh",
      doc:"docs/architecture/HEPTA_TERMINAL_GOVERNANCE_CLOSURE_SUMMARY_GATE.md",
      script_present:$terminal_governance_summary_present,
      doc_present:$terminal_governance_doc_present,
      terminal_summary_role:"governance_summary",
      invoked:false
    },
    {
      id:"terminal_release_governance_final_audit_index",
      script:"scripts/hepta-terminal-release-governance-final-audit-index-gate.sh",
      doc:"docs/architecture/HEPTA_TERMINAL_RELEASE_GOVERNANCE_FINAL_AUDIT_INDEX_GATE.md",
      script_present:$terminal_release_final_audit_present,
      doc_present:$terminal_release_final_audit_doc_present,
      terminal_summary_role:"release_final_audit",
      invoked:false
    },
    {
      id:"terminal_operator_readiness_non_approval_index",
      script:"scripts/hepta-terminal-operator-readiness-non-approval-index-gate.sh",
      doc:"docs/architecture/HEPTA_TERMINAL_OPERATOR_READINESS_NON_APPROVAL_INDEX_GATE.md",
      script_present:$terminal_operator_readiness_present,
      doc_present:$terminal_operator_readiness_doc_present,
      terminal_summary_role:"operator_non_approval",
      invoked:false
    }
  ] as $terminal_summary_sources |
  [
    "manual_operator_live_cutover_approval_required",
    "canonical_successor_consumer_cutover_disallowed",
    "current_canonical_consumer_rollback_anchor_retained",
    "terminal_summary_gates_source_probed_not_invoked",
    "terminal_live_gates_not_invoked",
    "canonical_gate_not_invoked",
    "wrapper_target_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "public_ga_disabled"
  ] as $attachment_blockers |
  ($final.terminal_governance_bridge_canonical_attachment_final_index_ready == true
    and $final.terminal_governance_bridge_canonical_attachment_final_index_blocked == true
    and $final.current_canonical_governance_terminal_index_attached == true
    and $final.tool_execution_closure_attached == true
    and $final.source_successor_consumer_cutover_allowed == false
    and $final.source_canonical_governance_rollback_anchor == "current_canonical_consumer"
    and $final.terminal_live_gates_invoked == false
    and $final.canonical_gate_wrapper_invoked == false
    and $final.wrapper_target_invoked == false
    and $final.tool_execution_live_cutover_allowed == false
    and $final.tool_execution_public_ga_allowed == false
    and $final.source_canonical_governance_tool_execution_closure_backfeed_ready == true
    and $final.source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
    and $final.source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
    and $final.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
    and $final.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
    and $final.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
    and ($terminal_summary_sources | all(.script_present == true and .doc_present == true and .invoked == false))
    and ($final.side_effects | to_entries | all(.value == false))) as $attachment_ready |
  {
    runtime:"hepta",
    surface:"terminal_governance_bridge_canonical_attachment_terminal_denial_summary",
    plugin_id:$final.plugin_id,
    status:(if $attachment_ready then "ready_blocked" else "blocked" end),
    source_bridge_canonical_attachment_final_index_surface:$final.surface,
    source_bridge_canonical_attachment_final_index_ready:$final.terminal_governance_bridge_canonical_attachment_final_index_ready,
    source_bridge_canonical_attachment_final_index_blocked:$final.terminal_governance_bridge_canonical_attachment_final_index_blocked,
    terminal_denial_summary_attachment_ready:$attachment_ready,
    terminal_denial_summary_attachment_blocked:true,
    terminal_summary_source_probe_count:($terminal_summary_sources | length),
    terminal_summary_source_probe_ready_count:($terminal_summary_sources | map(select(.script_present == true and .doc_present == true and .invoked == false)) | length),
    terminal_summary_sources:$terminal_summary_sources,
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
    terminal_summary_gates_invoked:false,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    terminal_live_url_required:false,
    long_soak_required:false,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    next_migration_step:"derive_terminal_governance_bridge_canonical_attachment_terminal_denial_summary_readback_without_live_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_governance_bridge_canonical_attachment_final_index_report:"scripts/hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-final-index-report.sh",
      terminal_governance_bridge_canonical_attachment_final_index_gate:"scripts/hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-final-index-gate.sh"
    },
    side_effect_free:true,
    side_effects:$final.side_effects
  }'
