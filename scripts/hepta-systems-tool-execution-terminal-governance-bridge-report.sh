#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
CLOSURE_REPORT="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-closure-index-report.sh"
CLOSURE_GATE="$ROOT/scripts/hepta-systems-tool-execution-live-cutover-closure-index-gate.sh"
CANONICAL_TERMINAL_INDEX_REPORT="$ROOT/scripts/hepta-systems-current-canonical-governance-terminal-index-report.sh"
CANONICAL_TERMINAL_INDEX_GATE="$ROOT/scripts/hepta-systems-current-canonical-governance-terminal-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_TERMINAL_GOVERNANCE_BRIDGE_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-terminal-governance-bridge-report: FAIL: %s\n' "$1" >&2
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

[[ -x "$CLOSURE_REPORT" ]] || fail "missing executable tool execution closure index report: $CLOSURE_REPORT"
[[ -x "$CLOSURE_GATE" ]] || fail "missing executable tool execution closure index gate: $CLOSURE_GATE"
[[ -x "$CANONICAL_TERMINAL_INDEX_REPORT" ]] || fail "missing executable current canonical governance terminal index report: $CANONICAL_TERMINAL_INDEX_REPORT"
[[ -x "$CANONICAL_TERMINAL_INDEX_GATE" ]] || fail "missing executable current canonical governance terminal index gate: $CANONICAL_TERMINAL_INDEX_GATE"
[[ -f "$DOC" ]] || fail "missing tool execution terminal governance bridge architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal governance bridge report"
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
  --slurpfile closure <("$CLOSURE_REPORT") \
  --slurpfile canonical <("$CANONICAL_TERMINAL_INDEX_REPORT") \
  --arg gate "scripts/hepta-systems-tool-execution-terminal-governance-bridge-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_TERMINAL_GOVERNANCE_BRIDGE_2026-06-21.md" \
  --argjson terminal_denial_index_present "$terminal_denial_index_present" \
  --argjson terminal_governance_summary_present "$terminal_governance_summary_present" \
  --argjson terminal_release_final_audit_present "$terminal_release_final_audit_present" \
  --argjson terminal_operator_readiness_present "$terminal_operator_readiness_present" \
  --argjson terminal_denial_doc_present "$terminal_denial_doc_present" \
  --argjson terminal_governance_doc_present "$terminal_governance_doc_present" \
  --argjson terminal_release_final_audit_doc_present "$terminal_release_final_audit_doc_present" \
  --argjson terminal_operator_readiness_doc_present "$terminal_operator_readiness_doc_present" \
  '
  ($closure[0]) as $closure |
  ($canonical[0]) as $canonical |
  [
    {
      id:"terminal_denial_index",
      script:"scripts/hepta-terminal-denial-index-gate.sh",
      doc:"docs/architecture/HEPTA_TERMINAL_DENIAL_INDEX_GATE.md",
      script_present:$terminal_denial_index_present,
      doc_present:$terminal_denial_doc_present,
      invoked:false
    },
    {
      id:"terminal_governance_closure_summary",
      script:"scripts/hepta-terminal-governance-closure-summary-gate.sh",
      doc:"docs/architecture/HEPTA_TERMINAL_GOVERNANCE_CLOSURE_SUMMARY_GATE.md",
      script_present:$terminal_governance_summary_present,
      doc_present:$terminal_governance_doc_present,
      invoked:false
    },
    {
      id:"terminal_release_governance_final_audit_index",
      script:"scripts/hepta-terminal-release-governance-final-audit-index-gate.sh",
      doc:"docs/architecture/HEPTA_TERMINAL_RELEASE_GOVERNANCE_FINAL_AUDIT_INDEX_GATE.md",
      script_present:$terminal_release_final_audit_present,
      doc_present:$terminal_release_final_audit_doc_present,
      invoked:false
    },
    {
      id:"terminal_operator_readiness_non_approval_index",
      script:"scripts/hepta-terminal-operator-readiness-non-approval-index-gate.sh",
      doc:"docs/architecture/HEPTA_TERMINAL_OPERATOR_READINESS_NON_APPROVAL_INDEX_GATE.md",
      script_present:$terminal_operator_readiness_present,
      doc_present:$terminal_operator_readiness_doc_present,
      invoked:false
    }
  ] as $terminal_sources |
  ($closure.tool_execution_live_cutover_closure_index_ready
    and $closure.manual_operator_live_cutover_approval_required == true
    and $closure.tool_execution_live_cutover_allowed == false
    and $closure.tool_execution_public_ga_allowed == false
    and $closure.closure_blocker_count == 17
    and $closure.closure_blocker_category_count == 4
    and $closure.closure_blocker_category_ready_count == 4
    and $closure.closure_blocker_category_blocker_count == 17
    and $closure.closure_blocker_categorization_ready == true
    and $canonical.current_canonical_governance_terminal_index_ready == true
    and $canonical.current_canonical_governance_terminal_index_blocked == true
    and $canonical.tool_execution_closure_backfeed_ready == true
    and $canonical.tool_execution_closure_backfeed_blocker_count == 17
    and $canonical.tool_execution_closure_backfeed_category_count == 4
    and $canonical.tool_execution_closure_backfeed_category_ready_count == 4
    and $canonical.tool_execution_closure_backfeed_category_blocker_count == 17
    and $canonical.tool_execution_closure_backfeed_categorization_ready == true
    and $canonical.successor_cutover_final_gate_attached == true
    and $canonical.successor_consumer_cutover_allowed == false
    and $canonical.rollback_anchor == "current_canonical_consumer"
    and $canonical.canonical_gate_wrapper_invoked == false
    and $canonical.wrapper_target_invoked == false
    and $canonical.tool_execution_live_cutover_allowed == false
    and $canonical.tool_execution_public_ga_allowed == false
    and ($terminal_sources | all(.script_present == true and .doc_present == true and .invoked == false))
    and ($closure.side_effects | to_entries | all(.value == false))
    and ($canonical.side_effects | to_entries | all(.value == false))) as $bridge_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_terminal_governance_bridge",
    plugin_id:$closure.plugin_id,
    status:(if $bridge_ready then "ready" else "blocked" end),
    source_closure_surface:$closure.surface,
    source_closure_ready:$closure.tool_execution_live_cutover_closure_index_ready,
    source_manual_operator_live_cutover_approval_required:$closure.manual_operator_live_cutover_approval_required,
    source_live_cutover_allowed:$closure.tool_execution_live_cutover_allowed,
    source_public_ga_allowed:$closure.tool_execution_public_ga_allowed,
    source_closure_blocker_count:$closure.closure_blocker_count,
    source_closure_blocker_category_count:$closure.closure_blocker_category_count,
    source_closure_blocker_category_ready_count:$closure.closure_blocker_category_ready_count,
    source_closure_blocker_category_blocker_count:$closure.closure_blocker_category_blocker_count,
    source_closure_blocker_categorization_ready:$closure.closure_blocker_categorization_ready,
    source_closure_blocker_categories:$closure.closure_blocker_categories,
    source_current_canonical_governance_terminal_index_surface:$canonical.surface,
    source_current_canonical_governance_terminal_index_ready:$canonical.current_canonical_governance_terminal_index_ready,
    source_current_canonical_governance_terminal_index_blocked:$canonical.current_canonical_governance_terminal_index_blocked,
    source_active_current_canonical_consumer_surface:$canonical.active_current_canonical_consumer_surface,
    source_successor_cutover_final_gate_attached:$canonical.successor_cutover_final_gate_attached,
    source_successor_consumer_cutover_allowed:$canonical.successor_consumer_cutover_allowed,
    source_canonical_governance_terminal_blocker_count:$canonical.terminal_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_ready:$canonical.tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count:$canonical.tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count:$canonical.tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count:$canonical.tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count:$canonical.tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready:$canonical.tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories:$canonical.tool_execution_closure_backfeed_categories,
    source_canonical_governance_rollback_anchor:$canonical.rollback_anchor,
    bridge_source_count:2,
    canonical_governance_terminal_index_attached:true,
    terminal_source_probe_count:($terminal_sources | length),
    terminal_source_probe_ready_count:($terminal_sources | map(select(.script_present == true and .doc_present == true and .invoked == false)) | length),
    terminal_source_probes:$terminal_sources,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    terminal_live_url_required:false,
    long_soak_required:false,
    tool_execution_terminal_governance_bridge_ready:$bridge_ready,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    next_migration_step:"derive_tool_execution_terminal_governance_bridge_canonical_attachment_readback_without_live_gate_invocation",
    bridge_blockers:[
      "manual_operator_live_cutover_approval_required",
      "canonical_successor_consumer_cutover_disallowed",
      "current_canonical_consumer_rollback_anchor_retained",
      "terminal_live_gates_not_invoked_by_bridge",
      "tool_execution_live_cutover_allowed_false",
      "tool_execution_public_ga_allowed_false",
      "approval_request_not_sent",
      "tool_invocation_disabled",
      "ledger_write_disabled",
      "approval_broker_request_disabled"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      closure_index_report:"scripts/hepta-systems-tool-execution-live-cutover-closure-index-report.sh",
      closure_index_gate:"scripts/hepta-systems-tool-execution-live-cutover-closure-index-gate.sh",
      current_canonical_governance_terminal_index_report:"scripts/hepta-systems-current-canonical-governance-terminal-index-report.sh",
      current_canonical_governance_terminal_index_gate:"scripts/hepta-systems-current-canonical-governance-terminal-index-gate.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      terminal_live_gate_invoked:false,
      terminal_live_url_contacted:false,
      long_soak_started:false,
      plugin_cache_mutated:false,
      tool_registered:false,
      execution_adapter_dispatched:false,
      tool_invoked:false,
      tool_invocation_ledger_written:false,
      approval_broker_mutated:false,
      approval_requested:false,
      operator_cutover_acceptance_recorded:false,
      live_cutover_started:false,
      result_receipt_written:false,
      rollback_executed:false,
      rollback_receipt_written:false,
      mcp_server_started:false,
      app_connector_started:false,
      workflow_event_log_mutated:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
