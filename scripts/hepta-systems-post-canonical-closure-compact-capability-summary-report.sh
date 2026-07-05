#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
INDEX_REPORT="$ROOT/scripts/hepta-systems-current-canonical-closure-alias-readback-index-report.sh"
INDEX_GATE="$ROOT/scripts/hepta-systems-current-canonical-closure-alias-readback-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_POST_CANONICAL_CLOSURE_COMPACT_CAPABILITY_SUMMARY_2026-06-21.md"

fail() {
  printf 'hepta-systems-post-canonical-closure-compact-capability-summary-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$INDEX_REPORT" ]] || fail "missing executable current canonical closure alias readback index report: $INDEX_REPORT"
[[ -x "$INDEX_GATE" ]] || fail "missing executable current canonical closure alias readback index gate: $INDEX_GATE"
[[ -f "$DOC" ]] || fail "missing post-canonical closure compact capability summary architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the post-canonical closure compact capability summary report"
fi

jq -n \
  --slurpfile index <("$INDEX_REPORT") \
  --arg gate "scripts/hepta-systems-post-canonical-closure-compact-capability-summary-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_POST_CANONICAL_CLOSURE_COMPACT_CAPABILITY_SUMMARY_2026-06-21.md" \
  '
  ($index[0]) as $index |
  [
    {
      id:"current_canonical_closure_alias_readback_index",
      layer:"canonical_closure",
      local_ready:$index.current_canonical_closure_alias_readback_index_ready,
      live_enabled:false,
      public_ga_enabled:false,
      source:"scripts/hepta-systems-current-canonical-closure-alias-readback-index-report.sh",
      blocker:"manual_operator_live_cutover_approval_required"
    },
    {
      id:"historical_canonical_gate_alias_readback",
      layer:"canonical_alias",
      local_ready:$index.historical_canonical_gate_alias_readback_attached,
      live_enabled:false,
      public_ga_enabled:false,
      source:"scripts/hepta-systems-historical-canonical-gate-alias-readback-report.sh",
      blocker:"canonical_gate_not_invoked_by_alias_readback"
    },
    {
      id:"current_canonical_closure",
      layer:"current_closure",
      local_ready:$index.source_current_canonical_closure_ready,
      live_enabled:false,
      public_ga_enabled:false,
      source:"scripts/hepta-systems-current-canonical-closure-report.sh",
      blocker:"wrapper_target_not_invoked_by_closure_report"
    },
    {
      id:"historical_canonical_gate_thin_wrapper_validation",
      layer:"canonical_validation",
      local_ready:true,
      live_enabled:false,
      public_ga_enabled:false,
      source:"scripts/hepta-systems-historical-canonical-gate-thin-wrapper-validation-report.sh",
      blocker:"canonical_gate_not_invoked_by_validation_report"
    },
    {
      id:"tool_execution_live_cutover_closure",
      layer:"tool_execution",
      local_ready:true,
      live_enabled:false,
      public_ga_enabled:false,
      source:"scripts/hepta-systems-tool-execution-live-cutover-closure-index-report.sh",
      blocker:"manual_operator_live_cutover_approval_required"
    },
    {
      id:"terminal_governance_bridge",
      layer:"terminal_governance",
      local_ready:true,
      live_enabled:false,
      public_ga_enabled:false,
      source:"scripts/hepta-systems-tool-execution-terminal-governance-bridge-report.sh",
      blocker:"terminal_live_gates_not_invoked_by_bridge"
    }
  ] as $surfaces |
  [
    "manual_operator_live_cutover_approval_required",
    "tool_execution_live_cutover_allowed_false",
    "tool_execution_public_ga_allowed_false",
    "canonical_gate_not_invoked_by_post_canonical_summary",
    "wrapper_target_not_invoked_by_post_canonical_summary",
    "terminal_live_gates_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started"
  ] as $summary_blockers |
  [
    "canonical_wrapper_not_restored_yet"
  ] as $retired_pre_creation_blockers |
  ($index.current_canonical_closure_alias_readback_index_ready == true
    and $index.historical_canonical_gate_alias_readback_attached == true
    and $index.historical_canonical_gate_alias_readback_pending == false
    and $index.historical_canonical_gate_name_claimed == true
    and $index.canonical_gate_wrapper_invoked == false
    and $index.wrapper_target_invoked == false
    and $index.execution_enabled_count == 0
    and $index.public_ga_enabled_count == 0
    and $index.tool_execution_live_cutover_allowed == false
    and $index.tool_execution_public_ga_allowed == false
    and ($surfaces | all(.local_ready == true and .live_enabled == false and .public_ga_enabled == false))
    and ($summary_blockers | index("canonical_wrapper_not_restored_yet") == null)
    and ($index.side_effects | to_entries | all(.value == false))) as $summary_ready |
  {
    runtime:"hepta",
    surface:"post_canonical_closure_compact_capability_summary",
    plugin_id:$index.plugin_id,
    status:(if $summary_ready then "ready" else "blocked" end),
    source_alias_readback_index_surface:$index.surface,
    source_alias_readback_index_ready:$index.current_canonical_closure_alias_readback_index_ready,
    source_historical_canonical_gate_alias_readback_attached:$index.historical_canonical_gate_alias_readback_attached,
    source_historical_canonical_gate_alias_readback_pending:$index.historical_canonical_gate_alias_readback_pending,
    source_historical_canonical_gate_name_claimed:$index.historical_canonical_gate_name_claimed,
    post_canonical_closure_compact_capability_summary_ready:$summary_ready,
    local_surface_count:($surfaces | length),
    local_surface_ready_count:($surfaces | map(select(.local_ready == true)) | length),
    execution_enabled_count:($surfaces | map(select(.live_enabled == true)) | length),
    public_ga_enabled_count:($surfaces | map(select(.public_ga_enabled == true)) | length),
    capability_surfaces:$surfaces,
    retired_pre_creation_blocker_count:($retired_pre_creation_blockers | length),
    retired_pre_creation_blockers:$retired_pre_creation_blockers,
    stale_pre_creation_blockers_present:false,
    summary_blocker_count:($summary_blockers | length),
    summary_blockers:$summary_blockers,
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
    next_migration_step:"promote_post_canonical_closure_summary_as_current_canonical_consumer_without_live_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      alias_readback_index_report:"scripts/hepta-systems-current-canonical-closure-alias-readback-index-report.sh",
      alias_readback_index_gate:"scripts/hepta-systems-current-canonical-closure-alias-readback-index-gate.sh"
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
