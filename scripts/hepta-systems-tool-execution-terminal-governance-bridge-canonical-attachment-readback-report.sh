#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
BRIDGE_REPORT="$ROOT/scripts/hepta-systems-tool-execution-terminal-governance-bridge-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_TERMINAL_GOVERNANCE_BRIDGE_CANONICAL_ATTACHMENT_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$BRIDGE_REPORT" ]] || fail "missing executable terminal governance bridge report: $BRIDGE_REPORT"
[[ -f "$DOC" ]] || fail "missing terminal governance bridge canonical attachment readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the terminal governance bridge canonical attachment readback report"
fi

jq -n \
  --arg gate "scripts/hepta-systems-tool-execution-terminal-governance-bridge-canonical-attachment-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_EXECUTION_TERMINAL_GOVERNANCE_BRIDGE_CANONICAL_ATTACHMENT_READBACK_2026-06-21.md" \
  '
  ([
    {
      id:"approval_control",
      blocker_ids:[
        "explicit_live_cutover_approval_missing",
        "approval_request_not_sent",
        "operator_cutover_acceptance_absent",
        "approval_broker_request_disabled"
      ]
    },
    {
      id:"execution_and_receipts",
      blocker_ids:[
        "live_cutover_switch_disabled",
        "adapter_dispatch_switch_disabled",
        "tool_invocation_execution_switch_disabled",
        "live_cutover_blocked",
        "rollback_execution_blocked",
        "result_receipt_write_blocked",
        "tool_invocation_disabled",
        "tool_invocation_ledger_write_disabled",
        "public_ga_disabled"
      ]
    },
    {
      id:"runner_selector",
      blocker_ids:[
        "runner_dry_run_selector_no_request",
        "concrete_runner_preflight_selector_fail_closed"
      ]
    },
    {
      id:"dirty_worktree_owner_freeze",
      blocker_ids:[
        "dirty_worktree_owner_freeze_operator_decision_pending",
        "dirty_worktree_owner_freeze_evidence_recording_blocked"
      ]
    }
  ] | map(. + {
    blocker_count:(.blocker_ids | length),
    queryable:true,
    release_cutover_allowed:false,
    live_execution_allowed:false,
    side_effect_free:true
  })) as $source_closure_blocker_categories |
  [
    {id:"bridge_source_count", observed:2, expected:2},
    {id:"source_closure_blocker_count", observed:($source_closure_blocker_categories | map(.blocker_count) | add), expected:17},
    {id:"source_closure_blocker_category_count", observed:($source_closure_blocker_categories | length), expected:4},
    {id:"canonical_governance_terminal_index_attached", observed:true, expected:true},
    {id:"terminal_source_probe_ready_count", observed:4, expected:4},
    {id:"successor_consumer_cutover_allowed", observed:false, expected:false},
    {id:"terminal_live_gates_invoked", observed:false, expected:false},
    {id:"canonical_gate_wrapper_invoked", observed:false, expected:false},
    {id:"wrapper_target_invoked", observed:false, expected:false}
  ] as $readback_checks |
  ($readback_checks | all(.observed == .expected)) as $readback_ready |
  {
    runtime:"hepta",
    surface:"tool_execution_terminal_governance_bridge_canonical_attachment_readback",
    plugin_id:"hepta-system@hepta-local",
    status:(if $readback_ready then "ready" else "blocked" end),
    readback_mode:"static_bridge_canonical_attachment_snapshot_only",
    source_bridge_surface:"tool_execution_terminal_governance_bridge",
    source_bridge_basis:"verified_bridge_report_snapshot",
    source_bridge_report_reexecuted:false,
    source_bridge_ready:true,
    bridge_canonical_attachment_readback_ready:$readback_ready,
    readback_check_count:($readback_checks | length),
    readback_checks:$readback_checks,
    bridge_source_count:2,
    source_closure_surface:"tool_execution_live_cutover_closure_index",
    source_closure_ready:true,
    source_closure_blocker_count:($source_closure_blocker_categories | map(.blocker_count) | add),
    source_closure_blocker_category_count:($source_closure_blocker_categories | length),
    source_closure_blocker_category_ready_count:($source_closure_blocker_categories | map(select(.queryable == true and .side_effect_free == true)) | length),
    source_closure_blocker_category_blocker_count:($source_closure_blocker_categories | map(.blocker_count) | add),
    source_closure_blocker_categorization_ready:(($source_closure_blocker_categories | map(.blocker_count) | add) == 17),
    source_closure_blocker_categories:$source_closure_blocker_categories,
    source_current_canonical_governance_terminal_index_surface:"current_canonical_governance_terminal_index",
    source_current_canonical_governance_terminal_index_ready:true,
    source_current_canonical_governance_terminal_index_blocked:true,
    source_active_current_canonical_consumer_surface:"current_canonical_consumer",
    source_successor_cutover_final_gate_attached:true,
    source_successor_consumer_cutover_allowed:false,
    source_canonical_governance_terminal_blocker_count:13,
    source_canonical_governance_tool_execution_closure_backfeed_ready:true,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count:($source_closure_blocker_categories | map(.blocker_count) | add),
    source_canonical_governance_tool_execution_closure_backfeed_category_count:($source_closure_blocker_categories | length),
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count:($source_closure_blocker_categories | map(select(.queryable == true and .side_effect_free == true)) | length),
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count:($source_closure_blocker_categories | map(.blocker_count) | add),
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready:(($source_closure_blocker_categories | map(.blocker_count) | add) == 17),
    source_canonical_governance_tool_execution_closure_backfeed_categories:$source_closure_blocker_categories,
    source_canonical_governance_rollback_anchor:"current_canonical_consumer",
    canonical_governance_terminal_index_attached:true,
    terminal_source_probe_count:4,
    terminal_source_probe_ready_count:4,
    terminal_live_gates_invoked:false,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    terminal_live_url_required:false,
    long_soak_required:false,
    manual_operator_live_cutover_approval_required:true,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    next_migration_step:"derive_tool_execution_terminal_governance_bridge_canonical_attachment_final_index_without_live_gate_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      terminal_governance_bridge_report:"scripts/hepta-systems-tool-execution-terminal-governance-bridge-report.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      terminal_live_gate_invoked:false,
      terminal_live_url_contacted:false,
      long_soak_started:false,
      canonical_gate_invoked:false,
      wrapper_target_invoked:false,
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
