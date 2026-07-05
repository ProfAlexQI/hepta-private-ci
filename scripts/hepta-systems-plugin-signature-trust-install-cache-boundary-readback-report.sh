#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-canonical-manifest-permission-activation-contract-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_signature_trust_install_cache_boundary_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_SIGNATURE_TRUST_INSTALL_CACHE_BOUNDARY_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-signature-trust-install-cache-boundary-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable canonical plugin contract report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing signature/trust/install-cache Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing signature/trust/install-cache architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the signature/trust/install-cache boundary report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render canonical plugin contract report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "canonical plugin contract report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_signature_trust_install_cache_boundary_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-signature-trust-install-cache-boundary-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_SIGNATURE_TRUST_INSTALL_CACHE_BOUNDARY_READBACK_2026-06-30.md" \
  '
  def boundary_route($kind):
    if $kind == "mcp_server" then
      "plugin-boundary://hepta-system/mcp/signature-trust-install-cache"
    elif $kind == "app_connector" then
      "plugin-boundary://hepta-system/app/signature-trust-install-cache"
    else
      "plugin-boundary://hepta-system/unknown/signature-trust-install-cache"
    end;
  def entry($source_entry): {
    candidate_tool_id:$source_entry.candidate_tool_id,
    contribution_kind:$source_entry.contribution_kind,
    boundary_route:boundary_route($source_entry.contribution_kind),
    signature_boundary_ready:($source_entry.signature_boundary_checked == true and $source_entry.signature_required_before_install == true),
    signature_artifact_present:$source_entry.signature_present,
    signature_digest_projected:true,
    signature_verified:false,
    signature_required_before_install:$source_entry.signature_required_before_install,
    signature_acceptance_allowed:false,
    trust_boundary_ready:($source_entry.trust_boundary_checked == true and $source_entry.trust_required_before_install == true),
    trust_root_present:$source_entry.trust_root_present,
    trust_root_digest_projected:true,
    trust_root_accepted:$source_entry.trust_root_accepted,
    trust_required_before_install:$source_entry.trust_required_before_install,
    trust_root_acceptance_allowed:false,
    install_cache_boundary_ready:($source_entry.plugin_install_allowed == false and $source_entry.plugin_cache_mutated == false and $source_entry.dynamic_activation_allowed == false),
    install_cache_route_projected:true,
    install_cache_materialized:false,
    install_cache_mutated:$source_entry.plugin_cache_mutated,
    install_cache_materialization_allowed:false,
    operator_evidence_required:true,
    operator_evidence_recorded:false,
    operator_evidence_recording_allowed:false,
    operator_acceptance_required:true,
    operator_acceptance_recorded:false,
    operator_acceptance_recording_allowed:false,
    explicit_non_acceptance_receipt_projected:true,
    plugin_install_allowed:$source_entry.plugin_install_allowed,
    dynamic_activation_allowed:$source_entry.dynamic_activation_allowed,
    permission_granted:$source_entry.permission_granted,
    mcp_server_started:$source_entry.mcp_server_started,
    app_connector_started:$source_entry.app_connector_started,
    tool_registered:$source_entry.tool_registered,
    tool_invoked:$source_entry.tool_invoked,
    ledger_written:$source_entry.ledger_written,
    approval_requested:$source_entry.approval_requested,
    receipt_persisted:$source_entry.receipt_persisted,
    runtime_event_log_written:$source_entry.runtime_event_log_written,
    sqlite_written:$source_entry.sqlite_written,
    live_execution_started:$source_entry.live_execution_started
  };
  ($source[0]) as $source_report |
  ($source_report.entries | map(entry(.))) as $entries |
  ($entries | length) as $candidate_count |
  ($entries | map(select(.signature_boundary_ready == true)) | length) as $signature_boundary_ready_count |
  ($entries | map(select(.trust_boundary_ready == true)) | length) as $trust_boundary_ready_count |
  ($entries | map(select(.install_cache_boundary_ready == true)) | length) as $install_cache_boundary_ready_count |
  ($entries | map(select(.operator_evidence_required == true)) | length) as $operator_evidence_required_count |
  ($entries | map(select(.operator_acceptance_required == true)) | length) as $operator_acceptance_required_count |
  ($entries | map(select(.explicit_non_acceptance_receipt_projected == true)) | length) as $explicit_non_acceptance_receipt_projected_count |
  ($entries | map(select(.signature_artifact_present == true)) | length) as $signature_artifact_present_count |
  ($entries | map(select(.signature_verified == true)) | length) as $signature_verified_count |
  ($entries | map(select(.trust_root_present == true)) | length) as $trust_root_present_count |
  ($entries | map(select(.trust_root_accepted == true)) | length) as $trust_root_accepted_count |
  ($entries | map(select(.install_cache_materialized == true)) | length) as $install_cache_materialized_count |
  ($entries | map(select(.install_cache_mutated == true)) | length) as $install_cache_mutated_count |
  ($entries | map(select(.operator_evidence_recorded == true)) | length) as $evidence_recorded_count |
  ($entries | map(select(.operator_acceptance_recorded == true)) | length) as $acceptance_recorded_count |
  ($entries | map(select(.plugin_install_allowed == true)) | length) as $plugin_install_allowed_count |
  ($entries | map(select(.dynamic_activation_allowed == true)) | length) as $dynamic_activation_allowed_count |
  ($source_report.canonical_manifest_contract_ready == true
    and $source_report.candidate_count == 2
    and $source_report.signature_boundary_checked_count == 2
    and $source_report.trust_boundary_checked_count == 2
    and $source_report.install_blocked_count == 2
    and $source_report.plugin_install_allowed == false
    and $source_report.plugin_cache_mutation_allowed == false
    and $source_report.dynamic_activation_allowed == false
    and $source_report.signature_acceptance_allowed == false
    and $source_report.trust_root_acceptance_allowed == false
    and $lib_export_present == true
    and $candidate_count == 2
    and $signature_boundary_ready_count == 2
    and $trust_boundary_ready_count == 2
    and $install_cache_boundary_ready_count == 2
    and $operator_evidence_required_count == 2
    and $operator_acceptance_required_count == 2
    and $explicit_non_acceptance_receipt_projected_count == 2
    and $signature_artifact_present_count == 0
    and $signature_verified_count == 0
    and $trust_root_present_count == 0
    and $trust_root_accepted_count == 0
    and $install_cache_materialized_count == 0
    and $install_cache_mutated_count == 0
    and $evidence_recorded_count == 0
    and $acceptance_recorded_count == 0
    and $plugin_install_allowed_count == 0
    and $dynamic_activation_allowed_count == 0
    and ($entries | all(.signature_acceptance_allowed == false
      and .trust_root_acceptance_allowed == false
      and .install_cache_materialization_allowed == false
      and .operator_evidence_recording_allowed == false
      and .operator_acceptance_recording_allowed == false
      and .permission_granted == false
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
    surface:"hepta_systems_plugin_signature_trust_install_cache_boundary_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_signature_trust_install_cache_boundary_readback_gate",
    schema_version:"hepta_systems_plugin_signature_trust_install_cache_boundary_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_canonical_manifest_contract_ready:$source_report.canonical_manifest_contract_ready,
    source_signature_boundary_checked_count:$source_report.signature_boundary_checked_count,
    source_trust_boundary_checked_count:$source_report.trust_boundary_checked_count,
    source_install_blocked_count:$source_report.install_blocked_count,
    lib_export_present:$lib_export_present,
    candidate_count:$candidate_count,
    signature_boundary_ready_count:$signature_boundary_ready_count,
    trust_boundary_ready_count:$trust_boundary_ready_count,
    install_cache_boundary_ready_count:$install_cache_boundary_ready_count,
    operator_evidence_required_count:$operator_evidence_required_count,
    operator_acceptance_required_count:$operator_acceptance_required_count,
    explicit_non_acceptance_receipt_projected_count:$explicit_non_acceptance_receipt_projected_count,
    signature_artifact_present_count:$signature_artifact_present_count,
    signature_verified_count:$signature_verified_count,
    trust_root_present_count:$trust_root_present_count,
    trust_root_accepted_count:$trust_root_accepted_count,
    install_cache_materialized_count:$install_cache_materialized_count,
    install_cache_mutated_count:$install_cache_mutated_count,
    evidence_recorded_count:$evidence_recorded_count,
    acceptance_recorded_count:$acceptance_recorded_count,
    plugin_install_allowed_count:$plugin_install_allowed_count,
    dynamic_activation_allowed_count:$dynamic_activation_allowed_count,
    signature_trust_install_cache_boundary_readback_ready:$ready,
    signature_acceptance_allowed:false,
    trust_root_acceptance_allowed:false,
    operator_evidence_recording_allowed:false,
    operator_acceptance_recording_allowed:false,
    plugin_install_allowed:false,
    plugin_cache_mutation_allowed:false,
    install_cache_materialization_allowed:false,
    dynamic_activation_allowed:false,
    permission_grant_allowed:false,
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
      "signature_artifact_missing",
      "trust_root_missing",
      "operator_evidence_missing",
      "operator_acceptance_missing",
      "plugin_install_disabled",
      "plugin_cache_mutation_disabled",
      "install_cache_materialization_disabled",
      "dynamic_activation_disabled",
      "permission_grant_disabled",
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
      "hepta_systems_plugin_operator_evidence_acceptance_packet_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_operator_evidence_acceptance_packet_readback",
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
      install_cache_materialized:false,
      package_lock_written:false,
      remote_sync_started:false,
      loader_invoked:false,
      dynamic_activation_started:false,
      permission_granted:false,
      signature_verified:false,
      signature_accepted:false,
      trust_root_accepted:false,
      operator_evidence_recorded:false,
      operator_acceptance_recorded:false,
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
