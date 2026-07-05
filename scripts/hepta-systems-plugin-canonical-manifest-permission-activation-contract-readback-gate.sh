#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-plugin-canonical-manifest-permission-activation-contract-readback-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_CANONICAL_MANIFEST_PERMISSION_ACTIVATION_CONTRACT_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-canonical-manifest-permission-activation-contract-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable canonical plugin contract report: $REPORT"
[[ -f "$DOC" ]] || fail "missing canonical plugin contract architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the canonical plugin contract report"
fi

rg -q 'Hepta Systems Plugin Canonical Manifest Permission Activation Contract Readback' "$DOC" \
  || fail "architecture note must document the canonical plugin contract readback"
rg -q 'manifest, permission, activation, tool policy, version, signature, and trust boundaries' "$DOC" \
  || fail "architecture note must document manifest, permission, activation, tool policy, version, signature, and trust boundaries"
rg -q 'no plugin install, cache mutation, manifest rewrite, manifest schema write, dynamic activation, permission grant, signature acceptance, trust-root acceptance, MCP server start, app connector start, ToolRegistry registration, tool invocation, ledger write, approval request, receipt persistence, runtime event-log write, SQLite write, credential read, external network, Gateway/Auth mutation, Native POST mutation, Telegram transport mutation, channel send, package/release, canary activation, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed side-effect boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback"
  and .status == "ready_blocked"
  and .gate == "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_gate"
  and .schema_version == "hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .manifest_name == "hepta-system"
  and .manifest_version == "0.0.0-fixture"
  and .source_tool_registry_minimal_readback_ready == true
  and .source_lifecycle_ready == true
  and .source_manifest_parser_ready == true
  and .source_fixture_declarations_ready == true
  and .source_registry_dry_run_ready == true
  and .source_schema_cutover_preflight_ready == true
  and .lib_export_present == true
  and .manifest_identity_ready == true
  and .manifest_version_declared == true
  and .fixture_version_channel == "fixture"
  and .canonical_manifest_contract_ready == true
  and .skill_count == 1
  and .mcp_server_count == 1
  and .app_connector_count == 1
  and .tool_schema_count == 2
  and .permission_count == 2
  and .activation_event_count == 2
  and .tool_policy_count == 2
  and .candidate_count == 2
  and .canonical_candidate_count == 2
  and .schema_complete_count == 2
  and .permission_boundary_count == 2
  and .network_none_permission_count == 2
  and .filesystem_read_only_permission_count == 1
  and .connector_permission_count == 1
  and .manual_activation_event_count == 2
  and .approval_policy_count == 2
  and .ledger_required_count == 2
  and .timeout_policy_count == 2
  and .version_bound_count == 2
  and .signature_boundary_checked_count == 2
  and .trust_boundary_checked_count == 2
  and .install_blocked_count == 2
  and .activation_blocked_count == 2
  and .signature_present_count == 0
  and .trust_root_present_count == 0
  and .signature_accepted_count == 0
  and .trust_root_accepted_count == 0
  and .plugin_install_allowed == false
  and .plugin_cache_mutation_allowed == false
  and .dynamic_activation_allowed == false
  and .permission_grant_allowed == false
  and .signature_acceptance_allowed == false
  and .trust_root_acceptance_allowed == false
  and .mcp_server_start_allowed == false
  and .app_connector_start_allowed == false
  and .tool_registry_registration_allowed == false
  and .tool_invocation_allowed == false
  and .ledger_write_allowed == false
  and .approval_request_allowed == false
  and .receipt_persistence_allowed == false
  and .runtime_event_log_write_allowed == false
  and .sqlite_write_allowed == false
  and .live_execution_allowed == false
  and (.entries | length) == 2
  and any(.entries[]; .candidate_tool_id == "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp" and .contribution_kind == "mcp_server" and .filesystem_read_only_permission_declared == true and .approval_kind == "onUse")
  and any(.entries[]; .candidate_tool_id == "preview:connector:hepta-system@hepta-local:hepta_system_local_app" and .contribution_kind == "app_connector" and .connector_permission_declared == true and .approval_kind == "install")
  and (.entries | all(.manifest_identity_bound == true and .manifest_version_bound == true and .tool_schema_declared == true and .input_schema_declared == true and .output_schema_declared == true and .permission_declared == true and .network_none_permission_declared == true and .activation_event_declared == true and .manual_activation_declared == true and .tool_policy_declared == true and .approval_policy_declared == true and .ledger_required == true and .timeout_ms == 30000 and .signature_boundary_checked == true and .signature_present == false and .signature_required_before_install == true and .trust_boundary_checked == true and .trust_root_present == false and .trust_required_before_install == true and .plugin_install_allowed == false and .plugin_cache_mutated == false and .dynamic_activation_allowed == false and .permission_granted == false and .signature_accepted == false and .trust_root_accepted == false and .mcp_server_started == false and .app_connector_started == false and .tool_registered == false and .tool_invoked == false and .ledger_written == false and .approval_requested == false and .receipt_persisted == false and .runtime_event_log_written == false and .sqlite_written == false and .live_execution_started == false))
  and (.blockers | index("plugin_install_disabled")) != null
  and (.blockers | index("plugin_cache_mutation_disabled")) != null
  and (.blockers | index("dynamic_activation_disabled")) != null
  and (.blockers | index("permission_grant_disabled")) != null
  and (.blockers | index("signature_trust_acceptance_disabled")) != null
  and (.blockers | index("mcp_server_start_disabled")) != null
  and (.blockers | index("app_connector_start_disabled")) != null
  and (.blockers | index("tool_registry_registration_disabled")) != null
  and (.blockers | index("tool_invocation_disabled")) != null
  and (.blockers | index("ledger_write_disabled")) != null
  and (.blockers | index("approval_request_disabled")) != null
  and (.blockers | index("receipt_persistence_disabled")) != null
  and (.blockers | index("runtime_event_log_write_disabled")) != null
  and (.blockers | index("sqlite_write_disabled")) != null
  and (.blockers | index("live_execution_disabled")) != null
  and (.next_actions | index("hepta_systems_plugin_signature_trust_install_cache_boundary_readback")) != null
  and .recommended_next_gate == "hepta_systems_plugin_signature_trust_install_cache_boundary_readback"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime hepta_systems_plugin_canonical_manifest_permission_activation_contract_readback --lib
)

printf 'hepta-systems-plugin-canonical-manifest-permission-activation-contract-readback-gate: PASS: canonical plugin manifest, permission, activation, version, signature, and trust boundaries are read back without install or live activation\n'
