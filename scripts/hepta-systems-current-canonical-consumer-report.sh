#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
POST_SUMMARY_REPORT="$ROOT/scripts/hepta-systems-post-canonical-closure-compact-capability-summary-report.sh"
POST_SUMMARY_GATE="$ROOT/scripts/hepta-systems-post-canonical-closure-compact-capability-summary-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_CONSUMER_2026-06-21.md"

fail() {
  printf 'hepta-systems-current-canonical-consumer-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$POST_SUMMARY_REPORT" ]] || fail "missing executable post-canonical closure compact capability summary report: $POST_SUMMARY_REPORT"
[[ -x "$POST_SUMMARY_GATE" ]] || fail "missing executable post-canonical closure compact capability summary gate: $POST_SUMMARY_GATE"
[[ -f "$DOC" ]] || fail "missing current canonical consumer architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the current canonical consumer report"
fi

jq -n \
  --slurpfile post_summary <("$POST_SUMMARY_REPORT") \
  --arg gate "scripts/hepta-systems-current-canonical-consumer-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_CONSUMER_2026-06-21.md" \
  '
  ($post_summary[0]) as $summary |
  [
    {
      id:"post_canonical_closure_compact_capability_summary",
      surface:$summary.surface,
      source_ready:$summary.post_canonical_closure_compact_capability_summary_ready,
      active_current_canonical_consumer:true,
      supersedes:"current_compact_capability_summary",
      invoked_by_report:false,
      gate:"scripts/hepta-systems-post-canonical-closure-compact-capability-summary-gate.sh"
    }
  ] as $consumer_inputs |
  [
    "canonical_wrapper_not_restored_yet"
  ] as $retired_pre_creation_blockers |
  [
    "manual_operator_live_cutover_approval_required",
    "tool_execution_live_cutover_allowed_false",
    "tool_execution_public_ga_allowed_false",
    "canonical_gate_not_invoked_by_current_canonical_consumer",
    "wrapper_target_not_invoked_by_current_canonical_consumer",
    "terminal_live_gates_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started"
  ] as $consumer_blockers |
  ($summary.post_canonical_closure_compact_capability_summary_ready == true
    and $summary.source_alias_readback_index_ready == true
    and $summary.source_historical_canonical_gate_alias_readback_attached == true
    and $summary.source_historical_canonical_gate_alias_readback_pending == false
    and $summary.source_historical_canonical_gate_name_claimed == true
    and $summary.local_surface_count == 6
    and $summary.local_surface_ready_count == 6
    and $summary.execution_enabled_count == 0
    and $summary.public_ga_enabled_count == 0
    and $summary.retired_pre_creation_blocker_count == 1
    and ($summary.retired_pre_creation_blockers | index("canonical_wrapper_not_restored_yet")) != null
    and $summary.stale_pre_creation_blockers_present == false
    and ($summary.summary_blockers | index("canonical_wrapper_not_restored_yet") == null)
    and $summary.canonical_gate_wrapper_invoked == false
    and $summary.wrapper_target_invoked == false
    and $summary.capability_matrix_gate_invoked == false
    and $summary.terminal_live_gate_invoked == false
    and $summary.manual_operator_live_cutover_approval_required == true
    and $summary.tool_execution_live_cutover_allowed == false
    and $summary.tool_execution_public_ga_allowed == false
    and ($consumer_inputs | all(.source_ready == true and .active_current_canonical_consumer == true and .invoked_by_report == false))
    and ($consumer_blockers | index("canonical_wrapper_not_restored_yet") == null)
    and ($summary.side_effects | to_entries | all(.value == false))) as $consumer_ready |
  {
    runtime:"hepta",
    surface:"current_canonical_consumer",
    plugin_id:$summary.plugin_id,
    status:(if $consumer_ready then "ready" else "blocked" end),
    source_post_canonical_closure_summary_surface:$summary.surface,
    source_post_canonical_closure_summary_ready:$summary.post_canonical_closure_compact_capability_summary_ready,
    source_alias_readback_index_surface:$summary.source_alias_readback_index_surface,
    source_alias_readback_index_ready:$summary.source_alias_readback_index_ready,
    source_historical_canonical_gate_alias_readback_attached:$summary.source_historical_canonical_gate_alias_readback_attached,
    source_historical_canonical_gate_alias_readback_pending:$summary.source_historical_canonical_gate_alias_readback_pending,
    source_historical_canonical_gate_name_claimed:$summary.source_historical_canonical_gate_name_claimed,
    current_canonical_consumer_ready:$consumer_ready,
    current_canonical_consumer_surface:"post_canonical_closure_compact_capability_summary",
    current_canonical_consumer_report:"scripts/hepta-systems-post-canonical-closure-compact-capability-summary-report.sh",
    current_canonical_consumer_gate:"scripts/hepta-systems-post-canonical-closure-compact-capability-summary-gate.sh",
    previous_current_summary_surface:"current_compact_capability_summary",
    previous_current_summary_superseded_by_post_canonical_closure:true,
    canonical_consumer_promotion_kind:"successor_report_only",
    canonical_consumer_input_count:($consumer_inputs | length),
    canonical_consumer_inputs:$consumer_inputs,
    local_surface_count:$summary.local_surface_count,
    local_surface_ready_count:$summary.local_surface_ready_count,
    execution_enabled_count:0,
    public_ga_enabled_count:0,
    retired_pre_creation_blocker_count:($retired_pre_creation_blockers | length),
    retired_pre_creation_blockers:$retired_pre_creation_blockers,
    stale_pre_creation_blockers_present:false,
    current_summary_blocker_count:($consumer_blockers | length),
    current_summary_blockers:$consumer_blockers,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    capability_matrix_gate_invoked:false,
    terminal_live_gate_invoked:false,
    live_url_required:false,
    long_soak_required:false,
    manual_operator_live_cutover_approval_required:true,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    canonical_summary_mutation_allowed:false,
    current_canonical_wrapper_mutation_allowed:false,
    historical_canonical_gate_mutation_allowed:false,
    upstream_gate_reexecution_required:false,
    next_migration_step:"migrate_current_canonical_wrapper_to_promoted_consumer_without_alias_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      post_canonical_closure_compact_capability_summary_report:"scripts/hepta-systems-post-canonical-closure-compact-capability-summary-report.sh",
      post_canonical_closure_compact_capability_summary_gate:"scripts/hepta-systems-post-canonical-closure-compact-capability-summary-gate.sh"
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
