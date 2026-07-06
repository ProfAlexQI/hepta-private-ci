#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-noop-denial-receipt-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_POLICY_APPROVAL_LEDGER_BOUNDARY_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-policy-approval-ledger-boundary-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable plugin tool invocation noop denial receipt report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing plugin tool invocation policy approval ledger boundary Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing plugin tool invocation policy approval ledger boundary architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the plugin tool invocation policy approval ledger boundary report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render plugin tool invocation noop denial receipt report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "plugin tool invocation noop denial receipt report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-tool-invocation-policy-approval-ledger-boundary-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_POLICY_APPROVAL_LEDGER_BOUNDARY_READBACK_2026-06-30.md" \
  '
  def suffix($kind):
    if $kind == "mcp_server" then "local-mcp"
    elif $kind == "app_connector" then "local-app"
    else "unknown"
    end;
  def policy_decision_id($kind):
    "tool-policy-decision:hepta-system:" + suffix($kind) + ":deny-no-invocation";
  def policy_decision_digest($kind):
    "tool-policy-decision-digest:hepta-system:" + suffix($kind) + ":deny-v0";
  def approval_preflight_denial_id($kind):
    "approval-preflight-denial:hepta-system:" + suffix($kind) + ":no-request";
  def approval_denial_receipt_id($kind):
    "approval-denial-receipt:hepta-system:" + suffix($kind) + ":no-request";
  def ledger_write_denial_id($kind):
    "ledger-write-denial:hepta-system:" + suffix($kind) + ":no-write";
  def ledger_denial_receipt_id($kind):
    "ledger-denial-receipt:hepta-system:" + suffix($kind) + ":no-write";
  def receipt_anchor_id($kind):
    "receipt-anchor:hepta-system:" + suffix($kind) + ":no-persistence";
  def policy_boundary_receipt_id($kind):
    "policy-approval-ledger-boundary-receipt:hepta-system:" + suffix($kind) + ":read-only-denied";
  def policy_idempotency_key($kind):
    "policy-approval-ledger-idempotency:hepta-system:" + suffix($kind) + ":read-only-denied";
  def entry($source_entry): {
    candidate_tool_id:$source_entry.candidate_tool_id,
    contribution_kind:$source_entry.contribution_kind,
    source_invocation_denial_receipt_id:$source_entry.first_invocation_denial_receipt_id,
    source_noop_result_projection_id:$source_entry.noop_result_projection_id,
    source_noop_result_digest:$source_entry.noop_result_digest,
    source_ledger_denial_anchor_id:$source_entry.ledger_denial_anchor_id,
    source_approval_denial_anchor_id:$source_entry.approval_denial_anchor_id,
    source_receipt_denial_anchor_id:$source_entry.receipt_denial_anchor_id,
    source_invocation_idempotency_key:$source_entry.first_invocation_idempotency_key,
    policy_decision_id:policy_decision_id($source_entry.contribution_kind),
    policy_decision_digest:policy_decision_digest($source_entry.contribution_kind),
    approval_preflight_denial_id:approval_preflight_denial_id($source_entry.contribution_kind),
    approval_denial_receipt_id:approval_denial_receipt_id($source_entry.contribution_kind),
    ledger_write_denial_id:ledger_write_denial_id($source_entry.contribution_kind),
    ledger_denial_receipt_id:ledger_denial_receipt_id($source_entry.contribution_kind),
    receipt_anchor_id:receipt_anchor_id($source_entry.contribution_kind),
    first_policy_boundary_receipt_id:policy_boundary_receipt_id($source_entry.contribution_kind),
    second_policy_boundary_receipt_id:policy_boundary_receipt_id($source_entry.contribution_kind),
    stable_policy_boundary_receipt:true,
    unique_policy_boundary_receipt:true,
    first_policy_idempotency_key:policy_idempotency_key($source_entry.contribution_kind),
    second_policy_idempotency_key:policy_idempotency_key($source_entry.contribution_kind),
    stable_policy_idempotency_key:true,
    unique_policy_idempotency_key:true,
    policy_decision_id_projected:true,
    policy_decision_digest_projected:true,
    approval_preflight_denial_id_projected:true,
    approval_denial_receipt_projected:true,
    ledger_write_denial_id_projected:true,
    ledger_denial_receipt_projected:true,
    receipt_anchor_projected:true,
    policy_boundary_receipt_projected:true,
    policy_idempotency_key_projected:true,
    policy_decision_persisted:false,
    approval_preflight_executed:false,
    ledger_write_attempted:false,
    receipt_anchor_persisted:false,
    tool_registered:$source_entry.tool_registered,
    tool_registry_mutated:$source_entry.tool_registry_mutated,
    registry_lookup_executed:$source_entry.registry_lookup_executed,
    tool_invoked:$source_entry.tool_invoked,
    noop_result_persisted:$source_entry.noop_result_persisted,
    ledger_written:$source_entry.ledger_written,
    approval_requested:$source_entry.approval_requested,
    receipt_persisted:$source_entry.receipt_persisted,
    dynamic_activation_started:$source_entry.dynamic_activation_started,
    permission_granted:$source_entry.permission_granted,
    mcp_server_started:$source_entry.mcp_server_started,
    app_connector_started:$source_entry.app_connector_started,
    plugin_installed:$source_entry.plugin_installed,
    cache_materialized:$source_entry.cache_materialized,
    cache_mutated:$source_entry.cache_mutated,
    runtime_event_log_written:$source_entry.runtime_event_log_written,
    sqlite_written:$source_entry.sqlite_written,
    live_execution_started:$source_entry.live_execution_started
  };
  ($source[0]) as $source_report |
  ($source_report.entries | map(entry(.))) as $entries |
  ($entries | length) as $policy_boundary_entry_count |
  ($entries | map(select(.policy_decision_id_projected == true)) | length) as $policy_decision_id_projected_count |
  ($entries | map(select(.policy_decision_digest_projected == true)) | length) as $policy_decision_digest_projected_count |
  ($entries | map(select(.approval_preflight_denial_id_projected == true)) | length) as $approval_preflight_denial_id_projected_count |
  ($entries | map(select(.approval_denial_receipt_projected == true)) | length) as $approval_denial_receipt_projected_count |
  ($entries | map(select(.ledger_write_denial_id_projected == true)) | length) as $ledger_write_denial_id_projected_count |
  ($entries | map(select(.ledger_denial_receipt_projected == true)) | length) as $ledger_denial_receipt_projected_count |
  ($entries | map(select(.receipt_anchor_projected == true)) | length) as $receipt_anchor_projected_count |
  ($entries | map(select(.policy_boundary_receipt_projected == true)) | length) as $policy_boundary_receipt_projected_count |
  ($entries | map(select(.stable_policy_boundary_receipt == true)) | length) as $stable_policy_boundary_receipt_count |
  ($entries | map(.first_policy_boundary_receipt_id) | unique | length) as $unique_policy_boundary_receipt_count |
  ($entries | map(select(.policy_idempotency_key_projected == true)) | length) as $policy_idempotency_key_projected_count |
  ($entries | map(select(.stable_policy_idempotency_key == true)) | length) as $stable_policy_idempotency_key_count |
  ($entries | map(.first_policy_idempotency_key) | unique | length) as $unique_policy_idempotency_key_count |
  ($entries | map(select(.stable_policy_boundary_receipt == false)) | length) as $policy_boundary_receipt_mismatch_count |
  ($policy_boundary_entry_count - $unique_policy_boundary_receipt_count) as $duplicate_policy_boundary_receipt_count |
  ($entries | map(select(.stable_policy_idempotency_key == false)) | length) as $policy_idempotency_key_mismatch_count |
  ($policy_boundary_entry_count - $unique_policy_idempotency_key_count) as $duplicate_policy_idempotency_key_count |
  ($entries | map(select(.policy_decision_persisted == true)) | length) as $policy_decision_persisted_count |
  ($entries | map(select(.approval_preflight_executed == true)) | length) as $approval_preflight_executed_count |
  ($entries | map(select(.ledger_write_attempted == true)) | length) as $ledger_write_attempted_count |
  ($entries | map(select(.receipt_anchor_persisted == true)) | length) as $receipt_anchor_persisted_count |
  ($entries | map(select(.tool_registered == true)) | length) as $tool_registered_count |
  ($entries | map(select(.tool_registry_mutated == true)) | length) as $tool_registry_mutated_count |
  ($entries | map(select(.registry_lookup_executed == true)) | length) as $registry_lookup_executed_count |
  ($entries | map(select(.tool_invoked == true)) | length) as $tool_invoked_count |
  ($entries | map(select(.noop_result_persisted == true)) | length) as $noop_result_persisted_count |
  ($entries | map(select(.ledger_written == true)) | length) as $ledger_written_count |
  ($entries | map(select(.approval_requested == true)) | length) as $approval_requested_count |
  ($entries | map(select(.receipt_persisted == true)) | length) as $receipt_persisted_count |
  ($entries | map(select(.dynamic_activation_started == true)) | length) as $dynamic_activation_started_count |
  ($entries | map(select(.permission_granted == true)) | length) as $permission_granted_count |
  ($entries | map(select(.mcp_server_started == true)) | length) as $mcp_server_started_count |
  ($entries | map(select(.app_connector_started == true)) | length) as $app_connector_started_count |
  ($entries | map(select(.plugin_installed == true)) | length) as $plugin_installed_count |
  ($entries | map(select(.cache_materialized == true)) | length) as $cache_materialized_count |
  ($entries | map(select(.cache_mutated == true)) | length) as $cache_mutated_count |
  ($entries | map(select(.runtime_event_log_written == true)) | length) as $runtime_event_log_written_count |
  ($entries | map(select(.sqlite_written == true)) | length) as $sqlite_written_count |
  ($entries | map(select(.live_execution_started == true)) | length) as $live_execution_started_count |
  ($source_report.tool_invocation_noop_denial_receipt_readback_ready == true
    and $source_report.candidate_count == 2
    and $source_report.invocation_denial_id_projected_count == 2
    and $source_report.noop_result_projected_count == 2
    and $source_report.ledger_denial_anchor_projected_count == 2
    and $source_report.approval_denial_anchor_projected_count == 2
    and $source_report.receipt_denial_anchor_projected_count == 2
    and $source_report.idempotency_key_projected_count == 2
    and $lib_export_present == true
    and $policy_boundary_entry_count == 2
    and $policy_decision_id_projected_count == 2
    and $policy_decision_digest_projected_count == 2
    and $approval_preflight_denial_id_projected_count == 2
    and $approval_denial_receipt_projected_count == 2
    and $ledger_write_denial_id_projected_count == 2
    and $ledger_denial_receipt_projected_count == 2
    and $receipt_anchor_projected_count == 2
    and $policy_boundary_receipt_projected_count == 2
    and $stable_policy_boundary_receipt_count == 2
    and $unique_policy_boundary_receipt_count == 2
    and $policy_idempotency_key_projected_count == 2
    and $stable_policy_idempotency_key_count == 2
    and $unique_policy_idempotency_key_count == 2
    and $policy_boundary_receipt_mismatch_count == 0
    and $duplicate_policy_boundary_receipt_count == 0
    and $policy_idempotency_key_mismatch_count == 0
    and $duplicate_policy_idempotency_key_count == 0
    and $policy_decision_persisted_count == 0
    and $approval_preflight_executed_count == 0
    and $ledger_write_attempted_count == 0
    and $receipt_anchor_persisted_count == 0
    and $tool_registered_count == 0
    and $tool_registry_mutated_count == 0
    and $registry_lookup_executed_count == 0
    and $tool_invoked_count == 0
    and $noop_result_persisted_count == 0
    and $ledger_written_count == 0
    and $approval_requested_count == 0
    and $receipt_persisted_count == 0
    and $dynamic_activation_started_count == 0
    and $permission_granted_count == 0
    and $mcp_server_started_count == 0
    and $app_connector_started_count == 0
    and $plugin_installed_count == 0
    and $cache_materialized_count == 0
    and $cache_mutated_count == 0
    and $runtime_event_log_written_count == 0
    and $sqlite_written_count == 0
    and $live_execution_started_count == 0
    and ($entries | all(.policy_decision_id_projected == true
      and .policy_decision_digest_projected == true
      and .approval_preflight_denial_id_projected == true
      and .approval_denial_receipt_projected == true
      and .ledger_write_denial_id_projected == true
      and .ledger_denial_receipt_projected == true
      and .receipt_anchor_projected == true
      and .policy_boundary_receipt_projected == true
      and .stable_policy_boundary_receipt == true
      and .unique_policy_boundary_receipt == true
      and .policy_idempotency_key_projected == true
      and .stable_policy_idempotency_key == true
      and .unique_policy_idempotency_key == true
      and .policy_decision_persisted == false
      and .approval_preflight_executed == false
      and .ledger_write_attempted == false
      and .receipt_anchor_persisted == false
      and .tool_registered == false
      and .tool_registry_mutated == false
      and .registry_lookup_executed == false
      and .tool_invoked == false
      and .noop_result_persisted == false
      and .ledger_written == false
      and .approval_requested == false
      and .receipt_persisted == false
      and .dynamic_activation_started == false
      and .permission_granted == false
      and .mcp_server_started == false
      and .app_connector_started == false
      and .plugin_installed == false
      and .cache_materialized == false
      and .cache_mutated == false
      and .runtime_event_log_written == false
      and .sqlite_written == false
      and .live_execution_started == false))) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback_gate",
    schema_version:"hepta_systems_plugin_tool_invocation_policy_approval_ledger_boundary_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_tool_invocation_noop_denial_receipt_ready:$source_report.tool_invocation_noop_denial_receipt_readback_ready,
    lib_export_present:$lib_export_present,
    candidate_count:$source_report.candidate_count,
    policy_boundary_entry_count:$policy_boundary_entry_count,
    policy_decision_id_projected_count:$policy_decision_id_projected_count,
    policy_decision_digest_projected_count:$policy_decision_digest_projected_count,
    approval_preflight_denial_id_projected_count:$approval_preflight_denial_id_projected_count,
    approval_denial_receipt_projected_count:$approval_denial_receipt_projected_count,
    ledger_write_denial_id_projected_count:$ledger_write_denial_id_projected_count,
    ledger_denial_receipt_projected_count:$ledger_denial_receipt_projected_count,
    receipt_anchor_projected_count:$receipt_anchor_projected_count,
    policy_boundary_receipt_projected_count:$policy_boundary_receipt_projected_count,
    stable_policy_boundary_receipt_count:$stable_policy_boundary_receipt_count,
    unique_policy_boundary_receipt_count:$unique_policy_boundary_receipt_count,
    policy_idempotency_key_projected_count:$policy_idempotency_key_projected_count,
    stable_policy_idempotency_key_count:$stable_policy_idempotency_key_count,
    unique_policy_idempotency_key_count:$unique_policy_idempotency_key_count,
    policy_boundary_receipt_mismatch_count:$policy_boundary_receipt_mismatch_count,
    duplicate_policy_boundary_receipt_count:$duplicate_policy_boundary_receipt_count,
    policy_idempotency_key_mismatch_count:$policy_idempotency_key_mismatch_count,
    duplicate_policy_idempotency_key_count:$duplicate_policy_idempotency_key_count,
    policy_decision_persisted_count:$policy_decision_persisted_count,
    approval_preflight_executed_count:$approval_preflight_executed_count,
    ledger_write_attempted_count:$ledger_write_attempted_count,
    receipt_anchor_persisted_count:$receipt_anchor_persisted_count,
    tool_registered_count:$tool_registered_count,
    tool_registry_mutated_count:$tool_registry_mutated_count,
    registry_lookup_executed_count:$registry_lookup_executed_count,
    tool_invoked_count:$tool_invoked_count,
    noop_result_persisted_count:$noop_result_persisted_count,
    ledger_written_count:$ledger_written_count,
    approval_requested_count:$approval_requested_count,
    receipt_persisted_count:$receipt_persisted_count,
    dynamic_activation_started_count:$dynamic_activation_started_count,
    permission_granted_count:$permission_granted_count,
    mcp_server_started_count:$mcp_server_started_count,
    app_connector_started_count:$app_connector_started_count,
    plugin_installed_count:$plugin_installed_count,
    cache_materialized_count:$cache_materialized_count,
    cache_mutated_count:$cache_mutated_count,
    runtime_event_log_written_count:$runtime_event_log_written_count,
    sqlite_written_count:$sqlite_written_count,
    live_execution_started_count:$live_execution_started_count,
    tool_invocation_policy_approval_ledger_boundary_readback_ready:$ready,
    policy_decision_persistence_allowed:false,
    approval_preflight_execution_allowed:false,
    ledger_write_allowed:false,
    receipt_anchor_persistence_allowed:false,
    tool_registry_registration_allowed:false,
    tool_registry_mutation_allowed:false,
    registry_lookup_execution_allowed:false,
    tool_invocation_allowed:false,
    noop_result_persistence_allowed:false,
    approval_request_allowed:false,
    receipt_persistence_allowed:false,
    dynamic_activation_allowed:false,
    permission_grant_allowed:false,
    mcp_server_start_allowed:false,
    app_connector_start_allowed:false,
    plugin_install_allowed:false,
    plugin_cache_mutation_allowed:false,
    install_cache_materialization_allowed:false,
    runtime_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    live_execution_allowed:false,
    entries:$entries,
    blockers:[
      "policy_decision_persistence_disabled",
      "approval_preflight_execution_disabled",
      "ledger_write_attempt_disabled",
      "ledger_write_disabled",
      "receipt_anchor_persistence_disabled",
      "tool_registry_registration_disabled",
      "tool_registry_mutation_disabled",
      "registry_lookup_execution_disabled",
      "tool_invocation_disabled",
      "noop_result_persistence_disabled",
      "approval_request_disabled",
      "receipt_persistence_disabled",
      "dynamic_activation_disabled",
      "permission_grant_disabled",
      "mcp_server_start_disabled",
      "app_connector_start_disabled",
      "plugin_install_disabled",
      "plugin_cache_mutation_disabled",
      "install_cache_materialization_disabled",
      "runtime_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      filesystem_written:false,
      policy_decision_persisted:false,
      approval_preflight_executed:false,
      ledger_write_attempted:false,
      receipt_anchor_persisted:false,
      tool_registered:false,
      tool_registry_mutated:false,
      registry_lookup_executed:false,
      tool_invoked:false,
      noop_result_persisted:false,
      ledger_written:false,
      approval_requested:false,
      receipt_persisted:false,
      dynamic_activation_started:false,
      permission_granted:false,
      mcp_server_started:false,
      app_connector_started:false,
      plugin_installed:false,
      plugin_cache_mutated:false,
      install_cache_materialized:false,
      runtime_event_log_written:false,
      sqlite_written:false,
      credential_read:false,
      external_network_used:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      telegram_transport_mutated:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }'
