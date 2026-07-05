#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
TOOL_MINIMAL_REPORT="$ROOT/scripts/hepta-systems-tool-registry-minimal-read-only-invocation-ledger-receipt-readback-report.sh"
LIFECYCLE_REPORT="$ROOT/scripts/hepta-systems-plugin-lifecycle-state-machine-report.sh"
PARSER_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-manifest-parser-fields-report.sh"
FIXTURE_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-manifest-fixture-declarations-report.sh"
REGISTRY_DRY_RUN_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-registry-source-of-truth-dry-run-report.sh"
SCHEMA_CUTOVER_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-manifest-schema-cutover-preflight-report.sh"
PLUGIN_MANIFEST="$ROOT/plugins/hepta-system/.codex-plugin/plugin.json"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_CANONICAL_MANIFEST_PERMISSION_ACTIVATION_CONTRACT_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-canonical-manifest-permission-activation-contract-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$TOOL_MINIMAL_REPORT" ]] || fail "missing executable tool minimal readback report: $TOOL_MINIMAL_REPORT"
[[ -x "$LIFECYCLE_REPORT" ]] || fail "missing executable plugin lifecycle report: $LIFECYCLE_REPORT"
[[ -x "$PARSER_REPORT" ]] || fail "missing executable manifest parser report: $PARSER_REPORT"
[[ -x "$FIXTURE_REPORT" ]] || fail "missing executable manifest fixture report: $FIXTURE_REPORT"
[[ -x "$REGISTRY_DRY_RUN_REPORT" ]] || fail "missing executable registry dry-run report: $REGISTRY_DRY_RUN_REPORT"
[[ -x "$SCHEMA_CUTOVER_REPORT" ]] || fail "missing executable schema cutover report: $SCHEMA_CUTOVER_REPORT"
[[ -f "$PLUGIN_MANIFEST" ]] || fail "missing hepta-system plugin manifest: $PLUGIN_MANIFEST"
[[ -f "$RUST_SOURCE" ]] || fail "missing canonical plugin contract Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing canonical plugin contract architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the canonical plugin contract report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$TOOL_MINIMAL_REPORT" >"$tmpdir/tool_minimal.json" || fail "failed to render tool minimal readback report"
"$LIFECYCLE_REPORT" >"$tmpdir/lifecycle.json" || fail "failed to render plugin lifecycle report"
"$PARSER_REPORT" >"$tmpdir/parser.json" || fail "failed to render manifest parser report"
"$FIXTURE_REPORT" >"$tmpdir/fixture.json" || fail "failed to render manifest fixture report"
"$REGISTRY_DRY_RUN_REPORT" >"$tmpdir/registry.json" || fail "failed to render registry dry-run report"
"$SCHEMA_CUTOVER_REPORT" >"$tmpdir/schema_cutover.json" || fail "failed to render schema cutover report"

jq -e . "$tmpdir/tool_minimal.json" >/dev/null || fail "tool minimal readback report did not render valid JSON"
jq -e . "$tmpdir/lifecycle.json" >/dev/null || fail "plugin lifecycle report did not render valid JSON"
jq -e . "$tmpdir/parser.json" >/dev/null || fail "manifest parser report did not render valid JSON"
jq -e . "$tmpdir/fixture.json" >/dev/null || fail "manifest fixture report did not render valid JSON"
jq -e . "$tmpdir/registry.json" >/dev/null || fail "registry dry-run report did not render valid JSON"
jq -e . "$tmpdir/schema_cutover.json" >/dev/null || fail "schema cutover report did not render valid JSON"
jq -e . "$PLUGIN_MANIFEST" >/dev/null || fail "plugin manifest did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile tool "$tmpdir/tool_minimal.json" \
  --slurpfile lifecycle "$tmpdir/lifecycle.json" \
  --slurpfile parser "$tmpdir/parser.json" \
  --slurpfile fixture "$tmpdir/fixture.json" \
  --slurpfile registry "$tmpdir/registry.json" \
  --slurpfile schema "$tmpdir/schema_cutover.json" \
  --slurpfile manifest "$PLUGIN_MANIFEST" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-canonical-manifest-permission-activation-contract-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_CANONICAL_MANIFEST_PERMISSION_ACTIVATION_CONTRACT_READBACK_2026-06-30.md" \
  '
  def selected_id: "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp";
  def non_selected_id: "preview:connector:hepta-system@hepta-local:hepta_system_local_app";
  def by_id($items; $id): ($items[] | select(.candidate_tool_id == $id));
  def entry($candidate_id; $kind; $route; $approval_kind; $fs_read_only; $connector_declared): {
    candidate_tool_id:$candidate_id,
    contribution_kind:$kind,
    canonical_contract_route:$route,
    manifest_identity_bound:true,
    manifest_version_bound:true,
    tool_schema_declared:true,
    input_schema_declared:true,
    output_schema_declared:true,
    permission_declared:true,
    network_none_permission_declared:true,
    filesystem_read_only_permission_declared:$fs_read_only,
    connector_permission_declared:$connector_declared,
    activation_event_declared:true,
    manual_activation_declared:true,
    tool_policy_declared:true,
    approval_policy_declared:true,
    approval_kind:$approval_kind,
    ledger_required:true,
    timeout_ms:30000,
    signature_boundary_checked:true,
    signature_present:false,
    signature_required_before_install:true,
    trust_boundary_checked:true,
    trust_root_present:false,
    trust_required_before_install:true,
    plugin_install_allowed:false,
    plugin_cache_mutated:false,
    dynamic_activation_allowed:false,
    permission_granted:false,
    signature_accepted:false,
    trust_root_accepted:false,
    mcp_server_started:false,
    app_connector_started:false,
    tool_registered:false,
    tool_invoked:false,
    ledger_written:false,
    approval_requested:false,
    receipt_persisted:false,
    runtime_event_log_written:false,
    sqlite_written:false,
    live_execution_started:false
  };
  ($tool[0]) as $tool_report |
  ($lifecycle[0]) as $lifecycle_report |
  ($parser[0]) as $parser_report |
  ($fixture[0]) as $fixture_report |
  ($registry[0]) as $registry_report |
  ($schema[0]) as $schema_report |
  ($manifest[0]) as $manifest_json |
  (by_id($schema_report.entries; selected_id)) as $selected_schema |
  (by_id($schema_report.entries; non_selected_id)) as $non_selected_schema |
  (by_id($registry_report.entries; selected_id)) as $selected_registry |
  (by_id($registry_report.entries; non_selected_id)) as $non_selected_registry |
  [
    entry(selected_id; "mcp_server"; "plugin-canonical://hepta-system/mcp/status-read-only"; "onUse"; true; false),
    entry(non_selected_id; "app_connector"; "plugin-canonical://hepta-system/app/status-read-only"; "install"; false; true)
  ] as $entries |
  ($entries | length) as $candidate_count |
  ($entries | map(select(.manifest_identity_bound == true and .manifest_version_bound == true)) | length) as $canonical_candidate_count |
  ($entries | map(select(.tool_schema_declared == true and .input_schema_declared == true and .output_schema_declared == true)) | length) as $schema_complete_count |
  ($entries | map(select(.permission_declared == true)) | length) as $permission_boundary_count |
  ($entries | map(select(.network_none_permission_declared == true)) | length) as $network_none_permission_count |
  ($entries | map(select(.filesystem_read_only_permission_declared == true)) | length) as $filesystem_read_only_permission_count |
  ($entries | map(select(.connector_permission_declared == true)) | length) as $connector_permission_count |
  ($entries | map(select(.manual_activation_declared == true)) | length) as $manual_activation_event_count |
  ($entries | map(select(.approval_policy_declared == true)) | length) as $approval_policy_count |
  ($entries | map(select(.ledger_required == true)) | length) as $ledger_required_count |
  ($entries | map(select(.timeout_ms == 30000)) | length) as $timeout_policy_count |
  ($entries | map(select(.manifest_version_bound == true)) | length) as $version_bound_count |
  ($entries | map(select(.signature_boundary_checked == true)) | length) as $signature_boundary_checked_count |
  ($entries | map(select(.trust_boundary_checked == true)) | length) as $trust_boundary_checked_count |
  ($entries | map(select(.plugin_install_allowed == false)) | length) as $install_blocked_count |
  ($entries | map(select(.dynamic_activation_allowed == false)) | length) as $activation_blocked_count |
  ($entries | map(select(.signature_present == true)) | length) as $signature_present_count |
  ($entries | map(select(.trust_root_present == true)) | length) as $trust_root_present_count |
  ($entries | map(select(.signature_accepted == true)) | length) as $signature_accepted_count |
  ($entries | map(select(.trust_root_accepted == true)) | length) as $trust_root_accepted_count |
  ($tool_report.minimal_read_only_invocation_ledger_receipt_readback_ready == true
    and $tool_report.candidate_count == 2
    and $tool_report.tool_invoked == false
    and $tool_report.ledger_written == false
    and $tool_report.approval_requested == false
    and $tool_report.receipt_persisted == false
    and $lifecycle_report.lifecycle_state_machine_ready == true
    and $lifecycle_report.fixture_shape_ready == true
    and $lifecycle_report.fixture_policy_metadata_ready == true
    and $lifecycle_report.plugin_manifest_summary.skill_count == 1
    and $lifecycle_report.plugin_manifest_summary.mcp_server_count == 1
    and $lifecycle_report.plugin_manifest_summary.app_count == 1
    and $lifecycle_report.plugin_manifest_summary.tool_schema_count == 2
    and $lifecycle_report.plugin_manifest_summary.permission_count == 2
    and $lifecycle_report.plugin_manifest_summary.activation_event_count == 2
    and $lifecycle_report.plugin_manifest_summary.tool_policy_count == 2
    and $parser_report.parser_fields_ready == true
    and $parser_report.current_fixture_declared_candidate_count == 2
    and $parser_report.current_fixture_schema_complete_count == 2
    and $parser_report.current_fixture_policy_complete_count == 2
    and $fixture_report.manifest_fixture_declarations_ready == true
    and $registry_report.registry_source_of_truth_dry_run_ready == true
    and $schema_report.manifest_schema_cutover_preflight_ready == true
    and $schema_report.registration_execution_enabled == false
    and $schema_report.tool_invocation_enabled == false
    and $schema_report.ledger_written == false
    and $schema_report.approval_requested == false
    and $selected_schema.manifest_schema_complete == true
    and $selected_schema.manifest_policy_complete == true
    and $non_selected_schema.manifest_schema_complete == true
    and $non_selected_schema.manifest_policy_complete == true
    and $selected_registry.ledger_required == true
    and $non_selected_registry.ledger_required == true
    and $manifest_json.name == "hepta-system"
    and $manifest_json.version == "0.0.0-fixture"
    and $manifest_json.skills == "./skills"
    and $manifest_json.mcpServers == "./.mcp.json"
    and $manifest_json.apps == "./.app.json"
    and (($manifest_json.toolSchemas | keys) | length) == 2
    and (($manifest_json.permissions | keys) | length) == 2
    and (($manifest_json.activationEvents | keys) | length) == 2
    and (($manifest_json.toolPolicies | keys) | length) == 2
    and $lib_export_present == true
    and $candidate_count == 2
    and $canonical_candidate_count == 2
    and $schema_complete_count == 2
    and $permission_boundary_count == 2
    and $network_none_permission_count == 2
    and $filesystem_read_only_permission_count == 1
    and $connector_permission_count == 1
    and $manual_activation_event_count == 2
    and $approval_policy_count == 2
    and $ledger_required_count == 2
    and $timeout_policy_count == 2
    and $version_bound_count == 2
    and $signature_boundary_checked_count == 2
    and $trust_boundary_checked_count == 2
    and $install_blocked_count == 2
    and $activation_blocked_count == 2
    and $signature_present_count == 0
    and $trust_root_present_count == 0
    and $signature_accepted_count == 0
    and $trust_root_accepted_count == 0
    and ($entries | all(.plugin_cache_mutated == false
      and .permission_granted == false
      and .signature_accepted == false
      and .trust_root_accepted == false
      and .mcp_server_started == false
      and .app_connector_started == false
      and .tool_registered == false
      and .tool_invoked == false
      and .ledger_written == false
      and .approval_requested == false
      and .receipt_persisted == false
      and .runtime_event_log_written == false
      and .sqlite_written == false
      and .live_execution_started == false))) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_gate",
    schema_version:"hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$manifest_json.name,
    manifest_version:$manifest_json.version,
    source_tool_registry_minimal_readback_ready:$tool_report.minimal_read_only_invocation_ledger_receipt_readback_ready,
    source_lifecycle_ready:$lifecycle_report.lifecycle_state_machine_ready,
    source_manifest_parser_ready:$parser_report.parser_fields_ready,
    source_fixture_declarations_ready:$fixture_report.manifest_fixture_declarations_ready,
    source_registry_dry_run_ready:$registry_report.registry_source_of_truth_dry_run_ready,
    source_schema_cutover_preflight_ready:$schema_report.manifest_schema_cutover_preflight_ready,
    lib_export_present:$lib_export_present,
    manifest_identity_ready:($manifest_json.name == "hepta-system"),
    manifest_version_declared:($manifest_json.version == "0.0.0-fixture"),
    fixture_version_channel:"fixture",
    canonical_manifest_contract_ready:$ready,
    skill_count:$lifecycle_report.plugin_manifest_summary.skill_count,
    mcp_server_count:$lifecycle_report.plugin_manifest_summary.mcp_server_count,
    app_connector_count:$lifecycle_report.plugin_manifest_summary.app_count,
    tool_schema_count:($manifest_json.toolSchemas | keys | length),
    permission_count:($manifest_json.permissions | keys | length),
    activation_event_count:($manifest_json.activationEvents | keys | length),
    tool_policy_count:($manifest_json.toolPolicies | keys | length),
    candidate_count:$candidate_count,
    canonical_candidate_count:$canonical_candidate_count,
    schema_complete_count:$schema_complete_count,
    permission_boundary_count:$permission_boundary_count,
    network_none_permission_count:$network_none_permission_count,
    filesystem_read_only_permission_count:$filesystem_read_only_permission_count,
    connector_permission_count:$connector_permission_count,
    manual_activation_event_count:$manual_activation_event_count,
    approval_policy_count:$approval_policy_count,
    ledger_required_count:$ledger_required_count,
    timeout_policy_count:$timeout_policy_count,
    version_bound_count:$version_bound_count,
    signature_boundary_checked_count:$signature_boundary_checked_count,
    trust_boundary_checked_count:$trust_boundary_checked_count,
    install_blocked_count:$install_blocked_count,
    activation_blocked_count:$activation_blocked_count,
    signature_present_count:$signature_present_count,
    trust_root_present_count:$trust_root_present_count,
    signature_accepted_count:$signature_accepted_count,
    trust_root_accepted_count:$trust_root_accepted_count,
    plugin_install_allowed:false,
    plugin_cache_mutation_allowed:false,
    dynamic_activation_allowed:false,
    permission_grant_allowed:false,
    signature_acceptance_allowed:false,
    trust_root_acceptance_allowed:false,
    mcp_server_start_allowed:false,
    app_connector_start_allowed:false,
    tool_registry_registration_allowed:false,
    tool_invocation_allowed:false,
    ledger_write_allowed:false,
    approval_request_allowed:false,
    receipt_persistence_allowed:false,
    runtime_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    live_execution_allowed:false,
    entries:$entries,
    blockers:[
      "plugin_install_disabled",
      "plugin_cache_mutation_disabled",
      "dynamic_activation_disabled",
      "permission_grant_disabled",
      "signature_trust_acceptance_disabled",
      "mcp_server_start_disabled",
      "app_connector_start_disabled",
      "tool_registry_registration_disabled",
      "tool_invocation_disabled",
      "ledger_write_disabled",
      "approval_request_disabled",
      "receipt_persistence_disabled",
      "runtime_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "hepta_systems_plugin_signature_trust_install_cache_boundary_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_signature_trust_install_cache_boundary_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      filesystem_written:false,
      manifest_rewritten:false,
      manifest_schema_written:false,
      plugin_installed:false,
      plugin_cache_mutated:false,
      package_lock_written:false,
      remote_sync_started:false,
      loader_invoked:false,
      dynamic_activation_started:false,
      permission_granted:false,
      signature_accepted:false,
      trust_root_accepted:false,
      mcp_server_started:false,
      app_connector_started:false,
      tool_registry_mutated:false,
      tool_registered:false,
      tool_invoked:false,
      ledger_written:false,
      approval_requested:false,
      receipt_persisted:false,
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
      canary_activated:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }
  '
