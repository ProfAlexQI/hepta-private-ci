#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PROMOTED_WRAPPER_REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-wrapper-report.sh"
ALIAS_READBACK_REPORT="$ROOT/scripts/hepta-systems-historical-canonical-gate-alias-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CLOSURE_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-closure-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$PROMOTED_WRAPPER_REPORT" ]] || fail "missing executable promoted current canonical wrapper report: $PROMOTED_WRAPPER_REPORT"
[[ -x "$ALIAS_READBACK_REPORT" ]] || fail "missing executable historical canonical gate alias readback report: $ALIAS_READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing promoted current canonical closure architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the promoted current canonical closure report"
fi

jq -n \
  --slurpfile wrapper <("$PROMOTED_WRAPPER_REPORT") \
  --slurpfile alias <("$ALIAS_READBACK_REPORT") \
  --arg gate "scripts/hepta-systems-promoted-current-canonical-closure-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CLOSURE_2026-06-21.md" \
  '
  ($wrapper[0]) as $wrapper |
  ($alias[0]) as $alias |
  [
    {
      id:"promoted_current_canonical_wrapper",
      surface:$wrapper.surface,
      source_ready:$wrapper.promoted_current_canonical_wrapper_ready,
      required:true,
      invoked_by_report:false,
      gate:"scripts/hepta-systems-promoted-current-canonical-wrapper-gate.sh"
    },
    {
      id:"historical_canonical_gate_alias_readback",
      surface:$alias.surface,
      source_ready:$alias.historical_canonical_gate_alias_readback_ready,
      required:true,
      invoked_by_report:false,
      gate:"scripts/hepta-systems-historical-canonical-gate-alias-readback-gate.sh"
    }
  ] as $closure_inputs |
  [
    "manual_operator_live_cutover_approval_required",
    "tool_execution_live_cutover_allowed_false",
    "tool_execution_public_ga_allowed_false",
    "canonical_gate_not_invoked_by_promoted_closure",
    "wrapper_target_not_invoked_by_promoted_closure",
    "legacy_closure_not_replaced_in_place",
    "terminal_live_gates_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started"
  ] as $closure_blockers |
  ($wrapper.promoted_current_canonical_wrapper_ready == true
    and $wrapper.promoted_wrapper_kind == "non_circular_successor_report"
    and $wrapper.legacy_current_canonical_wrapper_replaced_in_place == false
    and $wrapper.legacy_current_canonical_wrapper_mutated == false
    and $alias.historical_canonical_gate_alias_readback_ready == true
    and $alias.historical_canonical_gate_alias_readback_mode == "static_shell_readback_only"
    and $alias.historical_canonical_gate_alias_path == "scripts/hepta-systems-canonical-gate.sh"
    and $alias.historical_canonical_gate_alias_target == "scripts/hepta-systems-current-canonical-wrapper-gate.sh"
    and $alias.historical_canonical_gate_alias_target_count == 1
    and $alias.historical_canonical_gate_alias_exec_count == 1
    and $alias.historical_canonical_gate_alias_bash_syntax_valid == true
    and $wrapper.canonical_gate_wrapper_invoked == false
    and $wrapper.wrapper_target_invoked == false
    and $alias.canonical_gate_wrapper_invoked == false
    and $alias.wrapper_target_invoked == false
    and $wrapper.execution_enabled_count == 0
    and $wrapper.public_ga_enabled_count == 0
    and $alias.execution_enabled_count == 0
    and $alias.public_ga_enabled_count == 0
    and $wrapper.tool_execution_live_cutover_allowed == false
    and $wrapper.tool_execution_public_ga_allowed == false
    and $alias.tool_execution_live_cutover_allowed == false
    and $alias.tool_execution_public_ga_allowed == false
    and ($closure_inputs | all(.required == true and .source_ready == true and .invoked_by_report == false))
    and ($wrapper.side_effects | to_entries | all(.value == false))
    and ($alias.side_effects | to_entries | all(.value == false))) as $closure_ready |
  {
    runtime:"hepta",
    surface:"promoted_current_canonical_closure",
    plugin_id:$wrapper.plugin_id,
    status:(if $closure_ready then "ready" else "blocked" end),
    source_promoted_current_canonical_wrapper_surface:$wrapper.surface,
    source_promoted_current_canonical_wrapper_ready:$wrapper.promoted_current_canonical_wrapper_ready,
    source_alias_readback_surface:$alias.surface,
    source_alias_readback_ready:$alias.historical_canonical_gate_alias_readback_ready,
    source_alias_readback_mode:$alias.historical_canonical_gate_alias_readback_mode,
    promoted_current_canonical_closure_ready:$closure_ready,
    promoted_closure_kind:"non_circular_successor_closure",
    promoted_closure_input_count:($closure_inputs | length),
    promoted_closure_inputs:$closure_inputs,
    promoted_wrapper_attached:true,
    historical_canonical_gate_alias_readback_attached:true,
    historical_canonical_gate_alias_readback_pending:false,
    historical_canonical_gate_alias_path:$alias.historical_canonical_gate_alias_path,
    historical_canonical_gate_alias_target:$alias.historical_canonical_gate_alias_target,
    historical_canonical_gate_alias_exec_count:$alias.historical_canonical_gate_alias_exec_count,
    historical_canonical_gate_alias_bash_syntax_valid:$alias.historical_canonical_gate_alias_bash_syntax_valid,
    legacy_current_canonical_closure_replaced_in_place:false,
    legacy_current_canonical_wrapper_replaced_in_place:false,
    historical_canonical_gate_mutated:false,
    canonical_summary_mutated:false,
    execution_enabled_count:0,
    public_ga_enabled_count:0,
    closure_blocker_count:($closure_blockers | length),
    closure_blockers:$closure_blockers,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    capability_matrix_gate_invoked:false,
    terminal_live_gate_invoked:false,
    live_url_required:false,
    long_soak_required:false,
    manual_operator_live_cutover_approval_required:true,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    next_migration_step:"derive_promoted_current_canonical_closure_index_without_alias_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      promoted_current_canonical_wrapper_report:"scripts/hepta-systems-promoted-current-canonical-wrapper-report.sh",
      historical_canonical_gate_alias_readback_report:"scripts/hepta-systems-historical-canonical-gate-alias-readback-report.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      historical_patch_replayed:false,
      patch_body_emitted:false,
      plugin_fixture_fabricated:false,
      canonical_summary_mutated:false,
      current_canonical_wrapper_mutated:false,
      promoted_current_canonical_wrapper_mutated:false,
      current_canonical_closure_mutated:false,
      promoted_current_canonical_closure_mutated:false,
      historical_canonical_gate_mutated:false,
      strict_missing_consumer_mutated:false,
      historical_snapshot_evidence_written:false,
      wrapper_body_emitted_by_report:false,
      canonical_gate_invoked:false,
      wrapper_target_invoked:false,
      capability_matrix_gate_invoked:false,
      terminal_live_gate_invoked:false,
      terminal_live_url_contacted:false,
      long_soak_started:false,
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
