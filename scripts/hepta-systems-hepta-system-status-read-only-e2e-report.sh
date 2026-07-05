#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
PLUGIN_LIFECYCLE_REPORT="$ROOT/scripts/hepta-systems-plugin-lifecycle-state-machine-report.sh"
TOOL_DISPATCH_REPORT="$ROOT/scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh"
WORKFLOW_ADAPTER_REPORT="$ROOT/scripts/hepta-systems-workflow-durable-store-adapter-report.sh"
PLUGIN_STATUS_SKILL="$ROOT/plugins/hepta-system/skills/hepta-system-status/SKILL.md"
NATIVE_RUNTIME_STATUS="$ROOT/apps/hepta-native/src/hepta_runtime_status.rs"
NATIVE_HOME_RUNTIME_STATUS="$ROOT/apps/hepta-native/src/home/hepta_runtime_status.rs"
NATIVE_ACTION_BRIDGE="$ROOT/apps/hepta-native/src/hepta_action_bridge.rs"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_system_status_read_only_e2e.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_HEPTA_SYSTEM_STATUS_READ_ONLY_E2E_2026-06-27.md"

fail() {
  printf 'hepta-systems-hepta-system-status-read-only-e2e-report: FAIL: %s\n' "$1" >&2
  exit 1
}

path_exists() { [[ -e "$1" ]]; }
bool_for() {
  if "$@"; then printf 'true\n'; else printf 'false\n'; fi
}

[[ -x "$PLUGIN_LIFECYCLE_REPORT" ]] || fail "missing executable plugin lifecycle report: $PLUGIN_LIFECYCLE_REPORT"
[[ -x "$TOOL_DISPATCH_REPORT" ]] || fail "missing executable tool dispatch report: $TOOL_DISPATCH_REPORT"
[[ -x "$WORKFLOW_ADAPTER_REPORT" ]] || fail "missing executable workflow adapter report: $WORKFLOW_ADAPTER_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 4 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 4 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 4 read-only E2E report"
fi

plugin_status_skill_present="$(bool_for path_exists "$PLUGIN_STATUS_SKILL")"
native_runtime_status_present="$(bool_for path_exists "$NATIVE_RUNTIME_STATUS")"
native_home_runtime_status_present="$(bool_for path_exists "$NATIVE_HOME_RUNTIME_STATUS")"
native_action_bridge_present="$(bool_for path_exists "$NATIVE_ACTION_BRIDGE")"
lib_export_present=false
if grep -q 'hepta_system_status_read_only_e2e_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile plugin <("$PLUGIN_LIFECYCLE_REPORT") \
  --slurpfile tool <("$TOOL_DISPATCH_REPORT") \
  --slurpfile workflow <("$WORKFLOW_ADAPTER_REPORT") \
  --argjson plugin_status_skill_present "$plugin_status_skill_present" \
  --argjson native_runtime_status_present "$native_runtime_status_present" \
  --argjson native_home_runtime_status_present "$native_home_runtime_status_present" \
  --argjson native_action_bridge_present "$native_action_bridge_present" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-hepta-system-status-read-only-e2e-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_HEPTA_SYSTEM_STATUS_READ_ONLY_E2E_2026-06-27.md" \
  '
  def link($id; $layer; $route; $source; $ready; $evidence): {
    id:$id,
    layer:$layer,
    route:$route,
    source_surface:$source,
    ready:$ready,
    mutation_enabled:false,
    evidence:$evidence
  };
  ($plugin[0]) as $plugin |
  ($tool[0]) as $tool |
  ($workflow[0]) as $workflow |
  ($plugin.lifecycle_state_machine_ready == true
    and $plugin.source_of_truth_ready == true
    and $plugin.plugin_id == "hepta-system@hepta-local"
    and $plugin.live_mutation_ready == false
    and $plugin_status_skill_present == true) as $plugin_ready |
  ($tool.read_only_dispatch_preflight_ready == true
    and $tool.plugin_id == "hepta-system@hepta-local"
    and $tool.candidate_count == 2
    and $tool.dispatch_preflight_ready_count == 2
    and $tool.tool_invocation_enabled == false
    and $tool.ledger_written == false
    and $tool.approval_requested == false
    and $tool.result_receipt_written == false
    and $tool.live_mutation_ready == false) as $tool_ready |
  ($workflow.temporal_lite_adapter_ready == true
    and $workflow.event_contract_count == 9
    and $workflow.noop_receipt_count == 9
    and $workflow.feature_gate_enabled == false
    and $workflow.ready_for_event_log_write == false
    and $workflow.ready_for_sqlite_write == false
    and $workflow.ready_for_workflow_execution == false
    and $workflow.ready_for_live_execution == false) as $workflow_ready |
  ($native_runtime_status_present == true
    and $native_home_runtime_status_present == true
    and $native_action_bridge_present == true) as $native_ready |
  [
    link("hepta_system_status_plugin_fixture"; "plugins"; "status_plugin_fixture_ready"; "plugins/hepta-system/skills/hepta-system-status/SKILL.md"; $plugin_ready; "plugin lifecycle ready and status fixture skill present"),
    link("tool_registry_read_only_dispatch_preflight"; "tools"; "tool_registry_dispatch_preflight_ready"; "scripts/hepta-systems-tool-registry-read-only-dispatch-preflight-report.sh"; $tool_ready; "2 candidates project lookup, ledger preview, approval preflight, and receipt without invocation"),
    link("workflow_durable_store_adapter_noop_receipt"; "workflow"; "workflow_adapter_noop_receipt_ready"; "scripts/hepta-systems-workflow-durable-store-adapter-report.sh"; $workflow_ready; "9 workflow adapter entries project noop receipts behind disabled feature gate"),
    link("native_read_only_console_projection"; "native"; "native_read_only_console_projection_ready"; "apps/hepta-native/src/hepta_runtime_status.rs"; $native_ready; "Native runtime status pane and action bridge are present and read-only")
  ] as $links |
  ($lib_export_present
    and ($links | length) == 4
    and ($links | all(.ready == true and .mutation_enabled == false))) as $e2e_ready |
  {
    runtime:"hepta",
    surface:"hepta_system_status_read_only_e2e",
    status:(if $e2e_ready then "ready" else "blocked" end),
    gate:"hepta_system_status_read_only_e2e_gate",
    schema_version:"hepta_system_status_read_only_e2e_v1",
    plugin_id:"hepta-system@hepta-local",
    source_plugin_lifecycle_ready:$plugin_ready,
    source_tool_dispatch_ready:$tool_ready,
    source_workflow_adapter_ready:$workflow_ready,
    native_read_only_console_ready:$native_ready,
    plugin_status_skill_present:$plugin_status_skill_present,
    native_runtime_status_present:$native_runtime_status_present,
    native_home_runtime_status_present:$native_home_runtime_status_present,
    native_action_bridge_present:$native_action_bridge_present,
    lib_export_present:$lib_export_present,
    chain_link_count:($links | length),
    chain_ready_count:($links | map(select(.ready == true)) | length),
    read_only_e2e_ready:$e2e_ready,
    ready_for_registration:false,
    ready_for_invocation:false,
    ready_for_ledger_write:false,
    ready_for_approval_request:false,
    ready_for_receipt_persistence:false,
    ready_for_event_log_write:false,
    ready_for_sqlite_write:false,
    ready_for_workflow_execution:false,
    ready_for_replay_execution:false,
    ready_for_rollback_execution:false,
    ready_for_native_post_mutation:false,
    ready_for_channel_send:false,
    ready_for_live_execution:false,
    chain_links:$links,
    blockers:[
      "plugin_install_disabled",
      "tool_invocation_disabled",
      "ledger_write_disabled",
      "approval_request_disabled",
      "receipt_persistence_disabled",
      "workflow_event_log_write_disabled",
      "native_post_mutation_disabled",
      "channel_send_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "phase5_keep_controlled_live_blocked_until_explicit_operator_live_approval",
      "keep_read_only_e2e_visible_without_registration_invocation_or_persistence"
    ],
    next_migration_step:"phase5_keep_controlled_live_blocked_until_explicit_operator_live_approval",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      plugin_installed:false,
      plugin_cache_mutated:false,
      tool_registered:false,
      tool_invoked:false,
      ledger_written:false,
      approval_requested:false,
      receipt_persisted:false,
      workflow_event_log_written:false,
      sqlite_written:false,
      workflow_execution_started:false,
      replay_executed:false,
      rollback_executed:false,
      native_post_mutation_performed:false,
      gateway_or_auth_mutated:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      live_execution_started:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
