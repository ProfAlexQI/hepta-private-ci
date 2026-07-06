#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PROMOTED_CONSUMER_REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-consumer-report.sh"
CURRENT_CONSUMER_REPORT="$ROOT/scripts/hepta-systems-current-canonical-consumer-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CONSUMER_CUTOVER_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-consumer-cutover-preflight-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$PROMOTED_CONSUMER_REPORT" ]] || fail "missing executable promoted current canonical consumer report: $PROMOTED_CONSUMER_REPORT"
[[ -x "$CURRENT_CONSUMER_REPORT" ]] || fail "missing executable current canonical consumer report: $CURRENT_CONSUMER_REPORT"
[[ -f "$DOC" ]] || fail "missing promoted current canonical consumer cutover preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the promoted current canonical consumer cutover preflight report"
fi

jq -n \
  --slurpfile promoted <("$PROMOTED_CONSUMER_REPORT") \
  --slurpfile current <("$CURRENT_CONSUMER_REPORT") \
  --arg gate "scripts/hepta-systems-promoted-current-canonical-consumer-cutover-preflight-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CONSUMER_CUTOVER_PREFLIGHT_2026-06-21.md" \
  '
  ($promoted[0]) as $promoted |
  ($current[0]) as $current |
  [
    "current_canonical_consumer",
    "promoted_post_canonical_closure_compact_capability_summary",
    "promoted_current_canonical_closure_index",
    "current_canonical_consumer"
  ] as $direct_cutover_cycle_path |
  [
    {
      id:"replace_current_canonical_consumer_with_promoted_successor",
      target:"scripts/hepta-systems-current-canonical-consumer-report.sh",
      would_create_dependency_cycle:true,
      allowed:false,
      reason:"promoted_successor_still_depends_on_current_consumer_through_promoted_summary_and_closure_index"
    },
    {
      id:"create_terminal_successor_consumer_cutover_packet",
      target:"scripts/hepta-systems-promoted-current-canonical-consumer-cutover-packet-report.sh",
      would_create_dependency_cycle:false,
      allowed:true,
      reason:"packet_can_describe_future_manual_cutover_without_replacing_current_consumer"
    }
  ] as $cutover_options |
  [
    "direct_current_consumer_replacement_blocked_by_dependency_cycle",
    "terminal_successor_consumer_cutover_packet_required",
    "manual_operator_live_cutover_approval_required",
    "tool_execution_live_cutover_allowed_false",
    "tool_execution_public_ga_allowed_false",
    "canonical_gate_not_invoked_by_consumer_cutover_preflight",
    "wrapper_target_not_invoked_by_consumer_cutover_preflight",
    "terminal_live_gates_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started"
  ] as $cutover_blockers |
  ($promoted.promoted_current_canonical_consumer_ready == true
    and $promoted.promoted_current_canonical_consumer_surface == "promoted_post_canonical_closure_compact_capability_summary"
    and $promoted.previous_current_canonical_consumer_surface == "current_canonical_consumer"
    and $promoted.previous_current_canonical_consumer_replaced_in_place == false
    and $promoted.successor_consumer_cutover_preflight_required == true
    and $current.current_canonical_consumer_ready == true
    and $current.current_canonical_consumer_surface == "post_canonical_closure_compact_capability_summary"
    and $promoted.current_canonical_consumer_mutated == false
    and $promoted.promoted_current_canonical_consumer_mutated == false
    and $promoted.canonical_summary_mutated == false
    and $promoted.historical_canonical_gate_mutated == false
    and $promoted.canonical_gate_wrapper_invoked == false
    and $promoted.wrapper_target_invoked == false
    and $current.canonical_gate_wrapper_invoked == false
    and $current.wrapper_target_invoked == false
    and $promoted.execution_enabled_count == 0
    and $promoted.public_ga_enabled_count == 0
    and $promoted.tool_execution_live_cutover_allowed == false
    and $promoted.tool_execution_public_ga_allowed == false
    and ($cutover_options | any(.id == "replace_current_canonical_consumer_with_promoted_successor" and .would_create_dependency_cycle == true and .allowed == false))
    and ($cutover_options | any(.id == "create_terminal_successor_consumer_cutover_packet" and .would_create_dependency_cycle == false and .allowed == true))
    and ($promoted.side_effects | to_entries | all(.value == false))
    and ($current.side_effects | to_entries | all(.value == false))) as $preflight_ready |
  {
    runtime:"hepta",
    surface:"promoted_current_canonical_consumer_cutover_preflight",
    plugin_id:$promoted.plugin_id,
    status:(if $preflight_ready then "ready" else "blocked" end),
    source_promoted_current_canonical_consumer_surface:$promoted.surface,
    source_promoted_current_canonical_consumer_ready:$promoted.promoted_current_canonical_consumer_ready,
    source_current_canonical_consumer_surface:$current.surface,
    source_current_canonical_consumer_ready:$current.current_canonical_consumer_ready,
    source_promoted_consumer_summary_surface:$promoted.promoted_current_canonical_consumer_surface,
    cutover_preflight_ready:$preflight_ready,
    direct_current_consumer_replacement_allowed:false,
    direct_current_consumer_replacement_blocked:true,
    dependency_cycle_detected:true,
    dependency_cycle_path:$direct_cutover_cycle_path,
    terminal_successor_consumer_cutover_packet_required:true,
    terminal_successor_consumer_cutover_packet_allowed:true,
    current_canonical_consumer_replaced_in_place:false,
    current_canonical_consumer_mutated:false,
    promoted_current_canonical_consumer_mutated:false,
    canonical_summary_mutated:false,
    historical_canonical_gate_mutated:false,
    cutover_option_count:($cutover_options | length),
    cutover_options:$cutover_options,
    execution_enabled_count:0,
    public_ga_enabled_count:0,
    cutover_blocker_count:($cutover_blockers | length),
    cutover_blockers:$cutover_blockers,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    capability_matrix_gate_invoked:false,
    terminal_live_gate_invoked:false,
    live_url_required:false,
    long_soak_required:false,
    manual_operator_live_cutover_approval_required:true,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    upstream_gate_reexecution_required:false,
    next_migration_step:"create_terminal_successor_canonical_consumer_cutover_packet_without_live_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      promoted_current_canonical_consumer_report:"scripts/hepta-systems-promoted-current-canonical-consumer-report.sh",
      current_canonical_consumer_report:"scripts/hepta-systems-current-canonical-consumer-report.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      historical_patch_replayed:false,
      patch_body_emitted:false,
      plugin_fixture_fabricated:false,
      canonical_summary_mutated:false,
      promoted_post_canonical_summary_mutated:false,
      current_canonical_consumer_mutated:false,
      promoted_current_canonical_consumer_mutated:false,
      current_canonical_wrapper_mutated:false,
      promoted_current_canonical_wrapper_mutated:false,
      current_canonical_closure_mutated:false,
      promoted_current_canonical_closure_mutated:false,
      promoted_current_canonical_closure_index_mutated:false,
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
