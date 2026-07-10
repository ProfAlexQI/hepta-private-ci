#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-policy-approval-ledger-boundary-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_FEATURE_GATED_READ_ONLY_STATUS_DRY_RUN_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-feature-gated-read-only-status-dry-run-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable plugin tool invocation policy approval ledger boundary report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing plugin tool invocation feature-gated dry-run Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing plugin tool invocation feature-gated dry-run architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the plugin tool invocation feature-gated dry-run report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render plugin tool invocation policy approval ledger boundary report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "plugin tool invocation policy approval ledger boundary report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-tool-invocation-feature-gated-read-only-status-dry-run-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_FEATURE_GATED_READ_ONLY_STATUS_DRY_RUN_READBACK_2026-06-30.md" \
  '
  def suffix($kind):
    if $kind == "mcp_server" then "local-mcp"
    elif $kind == "app_connector" then "local-app"
    else "unknown"
    end;
  def selected($kind): $kind == "mcp_server";
  def feature_gate_id($kind): "feature-gate:hepta-system:" + suffix($kind) + ":status-dry-run";
  def dry_run_request_id($kind):
    if selected($kind) then "dry-run-request:hepta-system:local-mcp:status-read-only"
    else "dry-run-request:hepta-system:" + suffix($kind) + ":not-selected"
    end;
  def dry_run_payload_id($kind):
    if selected($kind) then "dry-run-payload:hepta-system:local-mcp:status-read-only-v0"
    else "dry-run-payload:hepta-system:" + suffix($kind) + ":not-selected"
    end;
  def dry_run_payload_digest($kind):
    if selected($kind) then "dry-run-payload-digest:hepta-system:local-mcp:status-read-only-v0"
    else "dry-run-payload-digest:hepta-system:" + suffix($kind) + ":not-selected"
    end;
  def dry_run_result_projection_id($kind):
    if selected($kind) then "dry-run-result-projection:hepta-system:local-mcp:status-read-only-v0"
    else "dry-run-result-projection:hepta-system:" + suffix($kind) + ":not-selected"
    end;
  def receipt_projection_id($kind):
    if selected($kind) then "dry-run-receipt-projection:hepta-system:local-mcp:read-only-denied"
    else "dry-run-receipt-projection:hepta-system:" + suffix($kind) + ":not-selected"
    end;
  def dry_run_receipt_id($kind):
    if selected($kind) then "dry-run-receipt:hepta-system:local-mcp:read-only-denied"
    else "dry-run-receipt:hepta-system:" + suffix($kind) + ":not-selected"
    end;
  def dry_run_idempotency_key($kind):
    if selected($kind) then "dry-run-idempotency:hepta-system:local-mcp:read-only-denied"
    else "dry-run-idempotency:hepta-system:" + suffix($kind) + ":not-selected"
    end;
  def entry($source_entry):
    ($source_entry.contribution_kind) as $kind |
    (selected($kind)) as $selected |
    {
      candidate_tool_id:$source_entry.candidate_tool_id,
      contribution_kind:$kind,
      dry_run_path_selected:$selected,
      dry_run_selection_reason:(if $selected then "selected_mcp_status_read_only_path" else "non_selected_app_connector_preflight_boundary" end),
      source_policy_decision_id:$source_entry.policy_decision_id,
      source_policy_boundary_receipt_id:$source_entry.first_policy_boundary_receipt_id,
      source_policy_idempotency_key:$source_entry.first_policy_idempotency_key,
      feature_gate_id:feature_gate_id($kind),
      feature_gate_state:"closed",
      dry_run_request_id:dry_run_request_id($kind),
      dry_run_payload_id:dry_run_payload_id($kind),
      dry_run_payload_digest:dry_run_payload_digest($kind),
      dry_run_result_projection_id:dry_run_result_projection_id($kind),
      policy_denial_id:$source_entry.policy_decision_id,
      receipt_projection_id:receipt_projection_id($kind),
      first_dry_run_receipt_id:dry_run_receipt_id($kind),
      second_dry_run_receipt_id:dry_run_receipt_id($kind),
      stable_dry_run_receipt:true,
      unique_dry_run_receipt:true,
      first_dry_run_idempotency_key:dry_run_idempotency_key($kind),
      second_dry_run_idempotency_key:dry_run_idempotency_key($kind),
      stable_idempotency_key:true,
      unique_idempotency_key:true,
      feature_gate_id_projected:true,
      feature_gate_closed:true,
      dry_run_payload_projected:$selected,
      dry_run_payload_digest_projected:$selected,
      dry_run_result_projected:$selected,
      policy_denial_projected:true,
      receipt_projection_projected:true,
      dry_run_receipt_projected:true,
      idempotency_key_projected:true,
      feature_gate_opened:false,
      dry_run_executed:false,
      dry_run_payload_persisted:false,
      dry_run_result_persisted:false,
      policy_decision_persisted:$source_entry.policy_decision_persisted,
      approval_preflight_executed:$source_entry.approval_preflight_executed,
      ledger_write_attempted:$source_entry.ledger_write_attempted,
      receipt_projection_persisted:false,
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
  ($entries | length) as $dry_run_entry_count |
  ($entries | map(select(.dry_run_path_selected == true)) | length) as $selected_read_only_status_tool_count |
  ($entries | map(select(.dry_run_path_selected == false)) | length) as $non_selected_preflight_boundary_count |
  ($entries | map(select(.feature_gate_id_projected == true)) | length) as $feature_gate_id_projected_count |
  ($entries | map(select(.feature_gate_closed == true)) | length) as $feature_gate_closed_count |
  ($entries | map(select(.dry_run_payload_projected == true)) | length) as $dry_run_payload_projected_count |
  ($entries | map(select(.dry_run_payload_digest_projected == true)) | length) as $dry_run_payload_digest_projected_count |
  ($entries | map(select(.dry_run_result_projected == true)) | length) as $dry_run_result_projection_count |
  ($entries | map(select(.policy_denial_projected == true)) | length) as $policy_denial_projected_count |
  ($entries | map(select(.receipt_projection_projected == true)) | length) as $receipt_projection_count |
  ($entries | map(select(.stable_dry_run_receipt == true)) | length) as $stable_dry_run_receipt_count |
  ($entries | map(.first_dry_run_receipt_id) | unique | length) as $unique_dry_run_receipt_count |
  ($entries | map(select(.idempotency_key_projected == true)) | length) as $idempotency_key_projected_count |
  ($entries | map(select(.stable_idempotency_key == true)) | length) as $stable_idempotency_key_count |
  ($entries | map(.first_dry_run_idempotency_key) | unique | length) as $unique_idempotency_key_count |
  ($entries | map(select(.stable_dry_run_receipt == false)) | length) as $dry_run_receipt_mismatch_count |
  ($dry_run_entry_count - $unique_dry_run_receipt_count) as $duplicate_dry_run_receipt_count |
  ($entries | map(select(.stable_idempotency_key == false)) | length) as $idempotency_key_mismatch_count |
  ($dry_run_entry_count - $unique_idempotency_key_count) as $duplicate_idempotency_key_count |
  ($entries | map(select(.feature_gate_opened == true)) | length) as $feature_gate_opened_count |
  ($entries | map(select(.dry_run_executed == true)) | length) as $dry_run_executed_count |
  ($entries | map(select(.dry_run_payload_persisted == true)) | length) as $dry_run_payload_persisted_count |
  ($entries | map(select(.dry_run_result_persisted == true)) | length) as $dry_run_result_persisted_count |
  ($entries | map(select(.policy_decision_persisted == true)) | length) as $policy_decision_persisted_count |
  ($entries | map(select(.approval_preflight_executed == true)) | length) as $approval_preflight_executed_count |
  ($entries | map(select(.ledger_write_attempted == true)) | length) as $ledger_write_attempted_count |
  ($entries | map(select(.receipt_projection_persisted == true)) | length) as $receipt_projection_persisted_count |
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
  ($source_report.tool_invocation_policy_approval_ledger_boundary_readback_ready == true
    and $source_report.candidate_count == 2
    and $source_report.policy_decision_id_projected_count == 2
    and $source_report.approval_preflight_denial_id_projected_count == 2
    and $source_report.ledger_write_denial_id_projected_count == 2
    and $source_report.receipt_anchor_projected_count == 2
    and $source_report.policy_boundary_receipt_projected_count == 2
    and $lib_export_present == true
    and $dry_run_entry_count == 2
    and $selected_read_only_status_tool_count == 1
    and $non_selected_preflight_boundary_count == 1
    and $feature_gate_id_projected_count == 2
    and $feature_gate_closed_count == 2
    and $dry_run_payload_projected_count == 1
    and $dry_run_payload_digest_projected_count == 1
    and $dry_run_result_projection_count == 1
    and $policy_denial_projected_count == 2
    and $receipt_projection_count == 2
    and $stable_dry_run_receipt_count == 2
    and $unique_dry_run_receipt_count == 2
    and $idempotency_key_projected_count == 2
    and $stable_idempotency_key_count == 2
    and $unique_idempotency_key_count == 2
    and $dry_run_receipt_mismatch_count == 0
    and $duplicate_dry_run_receipt_count == 0
    and $idempotency_key_mismatch_count == 0
    and $duplicate_idempotency_key_count == 0
    and $feature_gate_opened_count == 0
    and $dry_run_executed_count == 0
    and $dry_run_payload_persisted_count == 0
    and $dry_run_result_persisted_count == 0
    and $policy_decision_persisted_count == 0
    and $approval_preflight_executed_count == 0
    and $ledger_write_attempted_count == 0
    and $receipt_projection_persisted_count == 0
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
    and $live_execution_started_count == 0) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_gate",
    schema_version:"hepta_systems_plugin_tool_invocation_feature_gated_read_only_status_dry_run_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_policy_approval_ledger_boundary_ready:$source_report.tool_invocation_policy_approval_ledger_boundary_readback_ready,
    lib_export_present:$lib_export_present,
    candidate_count:$source_report.candidate_count,
    dry_run_entry_count:$dry_run_entry_count,
    selected_read_only_status_tool_count:$selected_read_only_status_tool_count,
    non_selected_preflight_boundary_count:$non_selected_preflight_boundary_count,
    feature_gate_id_projected_count:$feature_gate_id_projected_count,
    feature_gate_closed_count:$feature_gate_closed_count,
    dry_run_payload_projected_count:$dry_run_payload_projected_count,
    dry_run_payload_digest_projected_count:$dry_run_payload_digest_projected_count,
    dry_run_result_projection_count:$dry_run_result_projection_count,
    policy_denial_projected_count:$policy_denial_projected_count,
    receipt_projection_count:$receipt_projection_count,
    stable_dry_run_receipt_count:$stable_dry_run_receipt_count,
    unique_dry_run_receipt_count:$unique_dry_run_receipt_count,
    idempotency_key_projected_count:$idempotency_key_projected_count,
    stable_idempotency_key_count:$stable_idempotency_key_count,
    unique_idempotency_key_count:$unique_idempotency_key_count,
    dry_run_receipt_mismatch_count:$dry_run_receipt_mismatch_count,
    duplicate_dry_run_receipt_count:$duplicate_dry_run_receipt_count,
    idempotency_key_mismatch_count:$idempotency_key_mismatch_count,
    duplicate_idempotency_key_count:$duplicate_idempotency_key_count,
    feature_gate_opened_count:$feature_gate_opened_count,
    dry_run_executed_count:$dry_run_executed_count,
    dry_run_payload_persisted_count:$dry_run_payload_persisted_count,
    dry_run_result_persisted_count:$dry_run_result_persisted_count,
    policy_decision_persisted_count:$policy_decision_persisted_count,
    approval_preflight_executed_count:$approval_preflight_executed_count,
    ledger_write_attempted_count:$ledger_write_attempted_count,
    receipt_projection_persisted_count:$receipt_projection_persisted_count,
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
    feature_gated_read_only_status_dry_run_readback_ready:$ready,
    feature_gate_open_allowed:false,
    dry_run_execution_allowed:false,
    dry_run_payload_persistence_allowed:false,
    dry_run_result_persistence_allowed:false,
    policy_decision_persistence_allowed:false,
    approval_preflight_execution_allowed:false,
    ledger_write_allowed:false,
    receipt_projection_persistence_allowed:false,
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
      "feature_gate_open_disabled",
      "dry_run_execution_disabled",
      "dry_run_payload_persistence_disabled",
      "dry_run_result_persistence_disabled",
      "policy_decision_persistence_disabled",
      "approval_preflight_execution_disabled",
      "ledger_write_attempt_disabled",
      "ledger_write_disabled",
      "receipt_projection_persistence_disabled",
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
      "hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_tool_invocation_dry_run_receipt_ledger_preview_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      filesystem_written:false,
      feature_gate_opened:false,
      dry_run_executed:false,
      dry_run_payload_persisted:false,
      dry_run_result_persisted:false,
      policy_decision_persisted:false,
      approval_preflight_executed:false,
      ledger_write_attempted:false,
      receipt_projection_persisted:false,
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
