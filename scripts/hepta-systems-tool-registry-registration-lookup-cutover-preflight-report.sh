#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-tool-registry-invocation-source-of-truth-report.sh"
RUST_SOURCE="$ROOT/codex-rs/tools/src/tool_registry_registration_lookup_cutover_preflight.rs"
GATE="$ROOT/scripts/hepta-systems-tool-registry-registration-lookup-cutover-preflight-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_REGISTRATION_LOOKUP_CUTOVER_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-tool-registry-registration-lookup-cutover-preflight-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable invocation source-of-truth report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing registration lookup cutover preflight Rust source: $RUST_SOURCE"
[[ -f "$DOC" ]] || fail "missing registration lookup cutover preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the registration lookup cutover preflight report"
fi

jq -n \
  --slurpfile source <("$SOURCE_REPORT") \
  --arg gate "scripts/hepta-systems-tool-registry-registration-lookup-cutover-preflight-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_REGISTRATION_LOOKUP_CUTOVER_PREFLIGHT_2026-06-21.md" \
  '
  def lookup_entry($entry):
    ($entry.invocation_source_route == "approval_ledger_dry_run_source_only" and $entry.invocation_source_ready == true) as $source_ready |
    ($entry.registry_guard_route == "require_approval_ledger") as $approval_guard |
    {
      plugin_id:$entry.plugin_id,
      candidate_tool_id:$entry.candidate_tool_id,
      contribution_kind:$entry.contribution_kind,
      source_invocation_route:$entry.invocation_source_route,
      registry_guard_route:$entry.registry_guard_route,
      lookup_cutover_route:(if ($source_ready and $approval_guard) then "approval_ledger_lookup_dry_run" elif $source_ready then "blocked_by_lookup_guard" else "blocked_by_invocation_source" end),
      lookup_precondition_satisfied:($source_ready and $approval_guard),
      lookup_preflight_binding_present:true,
      router_registration_lookup_enabled:false,
      registry_lookup_executed:false,
      registry_source_of_truth_enabled:false,
      tool_registration_enabled:false,
      tool_invocation_enabled:false,
      ledger_write_enabled:false,
      approval_request_enabled:false,
      side_effect_free:true
    };

  ($source[0]) as $source |
  ($source.entries | map(lookup_entry(.))) as $entries |
  ($entries | map(select(.lookup_precondition_satisfied == true)) | length) as $satisfied_count |
  ($entries | map(select(.lookup_cutover_route == "approval_ledger_lookup_dry_run")) | length) as $dry_run_count |
  ($source.invocation_source_of_truth_plan_ready
    and ($entries | all(.lookup_preflight_binding_present == true))
    and $satisfied_count == $source.invocation_source_ready_count
    and $dry_run_count == $source.approval_ledger_dry_run_source_count
    and $satisfied_count == ($entries | length)
    and $dry_run_count == ($entries | length)
    and ($entries | all(if .lookup_cutover_route == "approval_ledger_lookup_dry_run" then (.registry_guard_route == "require_approval_ledger" and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false) else true end))) as $preflight_ready |
  {
    runtime:"hepta",
    surface:"tool_registry_registration_lookup_cutover_preflight",
    plugin_id:$source.plugin_id,
    status:(if $preflight_ready then "ready" else "blocked" end),
    source_invocation_surface:$source.surface,
    source_invocation_ready:$source.invocation_source_of_truth_plan_ready,
    source_invocation_ready_count:$source.invocation_source_ready_count,
    candidate_count:($entries | length),
    lookup_precondition_satisfied_count:$satisfied_count,
    lookup_blocked_count:(($entries | length) - $satisfied_count),
    approval_ledger_lookup_dry_run_count:$dry_run_count,
    all_invocation_sources_bound_to_lookup_preflight:($entries | all(.lookup_preflight_binding_present == true) and $satisfied_count == $source.invocation_source_ready_count and $dry_run_count == $source.approval_ledger_dry_run_source_count and $satisfied_count == ($entries | length) and $dry_run_count == ($entries | length)),
    all_lookup_entries_keep_approval_ledger_guard:($entries | all(if .lookup_cutover_route == "approval_ledger_lookup_dry_run" then (.registry_guard_route == "require_approval_ledger" and .router_registration_lookup_enabled == false and .registry_lookup_executed == false and .registry_source_of_truth_enabled == false and .tool_registration_enabled == false and .tool_invocation_enabled == false and .ledger_write_enabled == false and .approval_request_enabled == false) else true end)),
    registration_lookup_cutover_preflight_ready:$preflight_ready,
    registration_lookup_cutover_allowed:($preflight_ready and $satisfied_count == ($entries | length) and $dry_run_count == ($entries | length)),
    router_registration_lookup_enabled:false,
    registry_lookup_executed:false,
    registry_source_of_truth_enabled:false,
    tool_registration_enabled:false,
    tool_invocation_enabled:false,
    ledger_written:false,
    approval_requested:false,
    live_mutation_ready:false,
    next_migration_step:"restore_tool_registry_router_lookup_shadow_without_registration",
    entries:$entries,
    blockers:[
      "router_registration_lookup_disabled",
      "registry_lookup_execution_disabled",
      "registry_source_of_truth_enablement_disabled",
      "tool_registration_disabled",
      "tool_invocation_disabled",
      "ledger_write_disabled",
      "approval_request_disabled"
    ],
    next_actions:[
      "restore_tool_registry_router_lookup_shadow_without_registration",
      "keep_registration_lookup_cutover_preflight_read_only_until_shadow_gate_is_restored",
      "keep_registration_invocation_ledger_and_approval_disabled_until_explicit_cutover"
    ],
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      rust_contract:"codex-rs/tools/src/tool_registry_registration_lookup_cutover_preflight.rs",
      invocation_source_report:"scripts/hepta-systems-tool-registry-invocation-source-of-truth-report.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      plugin_cache_mutated:false,
      plugin_installed:false,
      manifest_rewritten:false,
      manifest_schema_written:false,
      registry_source_of_truth_enabled:false,
      router_registration_lookup_enabled:false,
      registry_lookup_executed:false,
      registration_cutover_executed:false,
      tool_registered:false,
      tool_invoked:false,
      tool_ledger_written:false,
      approval_requested:false,
      mcp_server_started:false,
      app_connector_started:false,
      workflow_event_log_mutated:false,
      local_storage_created:false,
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
