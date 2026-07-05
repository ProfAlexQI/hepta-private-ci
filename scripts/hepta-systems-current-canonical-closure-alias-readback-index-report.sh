#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
CLOSURE_REPORT="$ROOT/scripts/hepta-systems-current-canonical-closure-report.sh"
CLOSURE_GATE="$ROOT/scripts/hepta-systems-current-canonical-closure-gate.sh"
ALIAS_READBACK_REPORT="$ROOT/scripts/hepta-systems-historical-canonical-gate-alias-readback-report.sh"
ALIAS_READBACK_GATE="$ROOT/scripts/hepta-systems-historical-canonical-gate-alias-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_CLOSURE_ALIAS_READBACK_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-current-canonical-closure-alias-readback-index-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$CLOSURE_REPORT" ]] || fail "missing executable current canonical closure report: $CLOSURE_REPORT"
[[ -x "$CLOSURE_GATE" ]] || fail "missing executable current canonical closure gate: $CLOSURE_GATE"
[[ -x "$ALIAS_READBACK_REPORT" ]] || fail "missing executable historical canonical gate alias readback report: $ALIAS_READBACK_REPORT"
[[ -x "$ALIAS_READBACK_GATE" ]] || fail "missing executable historical canonical gate alias readback gate: $ALIAS_READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing current canonical closure alias readback index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the current canonical closure alias readback index report"
fi

jq -n \
  --slurpfile closure <("$CLOSURE_REPORT") \
  --slurpfile alias <("$ALIAS_READBACK_REPORT") \
  --arg gate "scripts/hepta-systems-current-canonical-closure-alias-readback-index-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_CLOSURE_ALIAS_READBACK_INDEX_2026-06-21.md" \
  '
  ($closure[0]) as $closure |
  ($alias[0]) as $alias |
  [
    {
      id:"current_canonical_closure",
      surface:$closure.surface,
      source_ready:$closure.current_canonical_closure_ready,
      required:true,
      invoked_by_report:false,
      gate:"scripts/hepta-systems-current-canonical-closure-gate.sh"
    },
    {
      id:"historical_canonical_gate_alias_readback",
      surface:$alias.surface,
      source_ready:$alias.historical_canonical_gate_alias_readback_ready,
      required:true,
      invoked_by_report:false,
      gate:"scripts/hepta-systems-historical-canonical-gate-alias-readback-gate.sh"
    }
  ] as $index_inputs |
  [
    "manual_operator_live_cutover_approval_required",
    "tool_execution_live_cutover_allowed_false",
    "tool_execution_public_ga_allowed_false",
    "canonical_gate_not_invoked_by_alias_readback_index",
    "wrapper_target_not_invoked_by_alias_readback_index",
    "terminal_live_gates_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started"
  ] as $index_blockers |
  ($closure.current_canonical_closure_ready == true
    and $closure.historical_canonical_gate_thin_wrapper_validation_attached == true
    and $alias.historical_canonical_gate_alias_readback_ready == true
    and $alias.source_current_canonical_closure_ready == true
    and $alias.source_thin_wrapper_validation_attached == true
    and $closure.historical_canonical_gate_wrapper_path == $alias.historical_canonical_gate_alias_path
    and $closure.historical_canonical_gate_wrapper_target == $alias.historical_canonical_gate_alias_target
    and $closure.historical_canonical_gate_wrapper_exec_count == $alias.historical_canonical_gate_alias_exec_count
    and $closure.historical_canonical_gate_bash_syntax_valid == $alias.historical_canonical_gate_alias_bash_syntax_valid
    and $closure.canonical_gate_wrapper_invoked == false
    and $closure.wrapper_target_invoked == false
    and $alias.canonical_gate_wrapper_invoked == false
    and $alias.wrapper_target_invoked == false
    and $closure.execution_enabled_count == 0
    and $closure.public_ga_enabled_count == 0
    and $alias.execution_enabled_count == 0
    and $alias.public_ga_enabled_count == 0
    and $closure.tool_execution_live_cutover_allowed == false
    and $closure.tool_execution_public_ga_allowed == false
    and $alias.tool_execution_live_cutover_allowed == false
    and $alias.tool_execution_public_ga_allowed == false
    and ($index_inputs | all(.required == true and .source_ready == true and .invoked_by_report == false))
    and ($closure.side_effects | to_entries | all(.value == false))
    and ($alias.side_effects | to_entries | all(.value == false))) as $index_ready |
  {
    runtime:"hepta",
    surface:"current_canonical_closure_alias_readback_index",
    plugin_id:$closure.plugin_id,
    status:(if $index_ready then "ready" else "blocked" end),
    source_current_canonical_closure_surface:$closure.surface,
    source_current_canonical_closure_ready:$closure.current_canonical_closure_ready,
    source_alias_readback_surface:$alias.surface,
    source_alias_readback_ready:$alias.historical_canonical_gate_alias_readback_ready,
    source_alias_readback_mode:$alias.historical_canonical_gate_alias_readback_mode,
    current_canonical_closure_alias_readback_index_ready:$index_ready,
    index_input_count:($index_inputs | length),
    index_inputs:$index_inputs,
    historical_canonical_gate_name_claimed:true,
    historical_canonical_gate_alias_readback_attached:true,
    historical_canonical_gate_alias_readback_pending:false,
    historical_canonical_gate_alias_path:$alias.historical_canonical_gate_alias_path,
    historical_canonical_gate_alias_target:$alias.historical_canonical_gate_alias_target,
    historical_canonical_gate_alias_target_count:$alias.historical_canonical_gate_alias_target_count,
    historical_canonical_gate_alias_exec_count:$alias.historical_canonical_gate_alias_exec_count,
    historical_canonical_gate_alias_bash_syntax_valid:$alias.historical_canonical_gate_alias_bash_syntax_valid,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    capability_matrix_gate_invoked:false,
    terminal_live_gate_invoked:false,
    live_url_required:false,
    long_soak_required:false,
    execution_enabled_count:0,
    public_ga_enabled_count:0,
    manual_operator_live_cutover_approval_required:true,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    next_migration_step:"derive_post_canonical_closure_compact_capability_summary_without_live_invocation",
    index_blocker_count:($index_blockers | length),
    index_blockers:$index_blockers,
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      current_canonical_closure_report:"scripts/hepta-systems-current-canonical-closure-report.sh",
      current_canonical_closure_gate:"scripts/hepta-systems-current-canonical-closure-gate.sh",
      historical_canonical_gate_alias_readback_report:"scripts/hepta-systems-historical-canonical-gate-alias-readback-report.sh",
      historical_canonical_gate_alias_readback_gate:"scripts/hepta-systems-historical-canonical-gate-alias-readback-gate.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      historical_patch_replayed:false,
      patch_body_emitted:false,
      plugin_fixture_fabricated:false,
      canonical_summary_mutated:false,
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
