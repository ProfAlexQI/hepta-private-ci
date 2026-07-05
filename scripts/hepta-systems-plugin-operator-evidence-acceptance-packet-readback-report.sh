#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-signature-trust-install-cache-boundary-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_operator_evidence_acceptance_packet_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_OPERATOR_EVIDENCE_ACCEPTANCE_PACKET_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-operator-evidence-acceptance-packet-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable signature/trust/install-cache boundary report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing operator evidence acceptance packet Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing operator evidence acceptance packet architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the operator evidence acceptance packet report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render signature/trust/install-cache boundary report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "signature/trust/install-cache boundary report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_operator_evidence_acceptance_packet_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-operator-evidence-acceptance-packet-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_OPERATOR_EVIDENCE_ACCEPTANCE_PACKET_READBACK_2026-06-30.md" \
  '
  def packet_route($kind):
    if $kind == "mcp_server" then
      "plugin-operator-packet://hepta-system/mcp/evidence-acceptance"
    elif $kind == "app_connector" then
      "plugin-operator-packet://hepta-system/app/evidence-acceptance"
    else
      "plugin-operator-packet://hepta-system/unknown/evidence-acceptance"
    end;
  def entry($source_entry): {
    candidate_tool_id:$source_entry.candidate_tool_id,
    contribution_kind:$source_entry.contribution_kind,
    packet_route:packet_route($source_entry.contribution_kind),
    source_signature_boundary_ready:$source_entry.signature_boundary_ready,
    source_trust_boundary_ready:$source_entry.trust_boundary_ready,
    source_install_cache_boundary_ready:$source_entry.install_cache_boundary_ready,
    operator_packet_projected:true,
    operator_packet_persisted:false,
    checklist_projected:true,
    checklist_persisted:false,
    signature_artifact_evidence_required:true,
    trust_root_evidence_required:true,
    install_cache_plan_evidence_required:true,
    rollback_uninstall_plan_evidence_required:true,
    evidence_item_required_count:4,
    evidence_item_recorded_count:0,
    acceptance_check_required_count:5,
    acceptance_check_recorded_count:0,
    signature_artifact_evidence_recorded:false,
    trust_root_evidence_recorded:false,
    install_cache_plan_evidence_recorded:false,
    rollback_uninstall_plan_evidence_recorded:false,
    signature_acceptance_recorded:false,
    trust_root_acceptance_recorded:false,
    install_cache_acceptance_recorded:false,
    rollback_uninstall_acceptance_recorded:false,
    dynamic_activation_acceptance_recorded:false,
    non_acceptance_receipt_projected:true,
    non_acceptance_receipt_persisted:false,
    plugin_install_allowed:$source_entry.plugin_install_allowed,
    install_cache_materialization_allowed:$source_entry.install_cache_materialization_allowed,
    dynamic_activation_allowed:$source_entry.dynamic_activation_allowed,
    rollback_uninstall_execution_allowed:false,
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
  ($entries | length) as $packet_entry_count |
  ($entries | map(select(.operator_packet_projected == true)) | length) as $packet_projected_count |
  ($entries | map(select(.checklist_projected == true)) | length) as $checklist_projected_count |
  ($entries | map(.evidence_item_required_count) | add) as $evidence_item_required_count |
  ($entries | map(.evidence_item_recorded_count) | add) as $evidence_item_recorded_count |
  ($entries | map(.acceptance_check_required_count) | add) as $acceptance_check_required_count |
  ($entries | map(.acceptance_check_recorded_count) | add) as $acceptance_check_recorded_count |
  ($entries | map(select(.signature_artifact_evidence_required == true)) | length) as $signature_artifact_evidence_required_count |
  ($entries | map(select(.trust_root_evidence_required == true)) | length) as $trust_root_evidence_required_count |
  ($entries | map(select(.install_cache_plan_evidence_required == true)) | length) as $install_cache_plan_evidence_required_count |
  ($entries | map(select(.rollback_uninstall_plan_evidence_required == true)) | length) as $rollback_uninstall_plan_evidence_required_count |
  ($entries | map(select(.non_acceptance_receipt_projected == true)) | length) as $non_acceptance_receipt_projected_count |
  ($entries | map(select(.operator_packet_persisted == true)) | length) as $packet_persisted_count |
  ($entries | map(select(.checklist_persisted == true)) | length) as $checklist_persisted_count |
  ($entries | map(select(.receipt_persisted == true or .non_acceptance_receipt_persisted == true)) | length) as $receipt_persisted_count |
  ($entries | map(select(.plugin_install_allowed == true)) | length) as $plugin_install_allowed_count |
  ($entries | map(select(.dynamic_activation_allowed == true)) | length) as $dynamic_activation_allowed_count |
  ($source_report.signature_trust_install_cache_boundary_readback_ready == true
    and $source_report.candidate_count == 2
    and $source_report.operator_evidence_required_count == 2
    and $source_report.operator_acceptance_required_count == 2
    and $source_report.evidence_recorded_count == 0
    and $source_report.acceptance_recorded_count == 0
    and $source_report.plugin_install_allowed == false
    and $source_report.plugin_cache_mutation_allowed == false
    and $source_report.dynamic_activation_allowed == false
    and $lib_export_present == true
    and $packet_entry_count == 2
    and $packet_projected_count == 2
    and $checklist_projected_count == 2
    and $evidence_item_required_count == 8
    and $evidence_item_recorded_count == 0
    and $acceptance_check_required_count == 10
    and $acceptance_check_recorded_count == 0
    and $signature_artifact_evidence_required_count == 2
    and $trust_root_evidence_required_count == 2
    and $install_cache_plan_evidence_required_count == 2
    and $rollback_uninstall_plan_evidence_required_count == 2
    and $non_acceptance_receipt_projected_count == 2
    and $packet_persisted_count == 0
    and $checklist_persisted_count == 0
    and $receipt_persisted_count == 0
    and $plugin_install_allowed_count == 0
    and $dynamic_activation_allowed_count == 0
    and ($entries | all(.source_signature_boundary_ready == true
      and .source_trust_boundary_ready == true
      and .source_install_cache_boundary_ready == true
      and .operator_packet_persisted == false
      and .checklist_persisted == false
      and .signature_artifact_evidence_recorded == false
      and .trust_root_evidence_recorded == false
      and .install_cache_plan_evidence_recorded == false
      and .rollback_uninstall_plan_evidence_recorded == false
      and .signature_acceptance_recorded == false
      and .trust_root_acceptance_recorded == false
      and .install_cache_acceptance_recorded == false
      and .rollback_uninstall_acceptance_recorded == false
      and .dynamic_activation_acceptance_recorded == false
      and .non_acceptance_receipt_persisted == false
      and .install_cache_materialization_allowed == false
      and .rollback_uninstall_execution_allowed == false
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
    surface:"hepta_systems_plugin_operator_evidence_acceptance_packet_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_operator_evidence_acceptance_packet_readback_gate",
    schema_version:"hepta_systems_plugin_operator_evidence_acceptance_packet_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_signature_trust_install_cache_boundary_ready:$source_report.signature_trust_install_cache_boundary_readback_ready,
    lib_export_present:$lib_export_present,
    candidate_count:$source_report.candidate_count,
    packet_entry_count:$packet_entry_count,
    packet_projected_count:$packet_projected_count,
    checklist_projected_count:$checklist_projected_count,
    evidence_item_required_count:$evidence_item_required_count,
    evidence_item_recorded_count:$evidence_item_recorded_count,
    acceptance_check_required_count:$acceptance_check_required_count,
    acceptance_check_recorded_count:$acceptance_check_recorded_count,
    signature_artifact_evidence_required_count:$signature_artifact_evidence_required_count,
    trust_root_evidence_required_count:$trust_root_evidence_required_count,
    install_cache_plan_evidence_required_count:$install_cache_plan_evidence_required_count,
    rollback_uninstall_plan_evidence_required_count:$rollback_uninstall_plan_evidence_required_count,
    non_acceptance_receipt_projected_count:$non_acceptance_receipt_projected_count,
    packet_persisted_count:$packet_persisted_count,
    checklist_persisted_count:$checklist_persisted_count,
    receipt_persisted_count:$receipt_persisted_count,
    plugin_install_allowed_count:$plugin_install_allowed_count,
    dynamic_activation_allowed_count:$dynamic_activation_allowed_count,
    operator_evidence_acceptance_packet_readback_ready:$ready,
    operator_packet_send_allowed:false,
    operator_packet_persistence_allowed:false,
    evidence_recording_allowed:false,
    acceptance_recording_allowed:false,
    signature_acceptance_allowed:false,
    trust_root_acceptance_allowed:false,
    plugin_install_allowed:false,
    plugin_cache_mutation_allowed:false,
    install_cache_materialization_allowed:false,
    dynamic_activation_allowed:false,
    rollback_uninstall_execution_allowed:false,
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
      "operator_packet_send_disabled",
      "operator_packet_persistence_disabled",
      "signature_artifact_evidence_missing",
      "trust_root_evidence_missing",
      "install_cache_plan_evidence_missing",
      "rollback_uninstall_plan_evidence_missing",
      "operator_acceptance_missing",
      "evidence_recording_disabled",
      "acceptance_recording_disabled",
      "plugin_install_disabled",
      "plugin_cache_mutation_disabled",
      "install_cache_materialization_disabled",
      "dynamic_activation_disabled",
      "rollback_uninstall_execution_disabled",
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
      "hepta_systems_plugin_install_cache_noop_preflight_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_install_cache_noop_preflight_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      filesystem_written:false,
      operator_packet_sent:false,
      operator_packet_persisted:false,
      checklist_persisted:false,
      evidence_recorded:false,
      acceptance_recorded:false,
      signature_accepted:false,
      trust_root_accepted:false,
      plugin_installed:false,
      plugin_cache_mutated:false,
      install_cache_materialized:false,
      rollback_uninstall_executed:false,
      manifest_rewritten:false,
      manifest_schema_written:false,
      dynamic_activation_started:false,
      permission_granted:false,
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
