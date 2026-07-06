#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-execution-open-preconditions-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_PACKET_READBACK_2026-06-30.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-packet-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable execution open preconditions report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing operator evidence packet Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing operator evidence packet architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the operator evidence packet report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render execution open preconditions report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "execution open preconditions report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-packet-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_PACKET_READBACK_2026-06-30.md" \
  '
  def suffix($kind):
    if $kind == "mcp_server" then "local-mcp:read-only-status-dry-run"
    elif $kind == "app_connector" then "local-app:not-selected"
    else "unknown:not-selected"
    end;
  def evidence_packet($kind):
    "operator-evidence-packet:hepta-system:" + suffix($kind);
  def evidence_artifact_ref($kind):
    "operator-evidence-artifact-ref:hepta-system:" + suffix($kind) + ":missing";
  def prerequisite($name; $kind):
    "operator-evidence-prerequisite:" + $name + ":hepta-system:" + suffix($kind);
  def denial_receipt($kind):
    "operator-evidence-packet-denial:hepta-system:" + suffix($kind) + ":not-sent-not-recorded";
  def idempotency_key($kind):
    "operator-evidence-packet-idempotency:hepta-system:" + suffix($kind);
  def evidence_items:
    [
      "status_payload_snapshot_required",
      "tool_schema_digest_required",
      "policy_denial_anchor_required",
      "approval_denial_anchor_required",
      "ledger_persistence_prerequisite_required",
      "receipt_persistence_prerequisite_required",
      "tool_registry_registration_prerequisite_required",
      "registry_lookup_invocation_prerequisite_required",
      "connector_runtime_boundary_required",
      "operator_identity_acceptance_recording_required"
    ];
  def entry($source_entry):
    ($source_entry.contribution_kind) as $kind |
    {
      candidate_tool_id:$source_entry.candidate_tool_id,
      contribution_kind:$kind,
      dry_run_path_selected:$source_entry.dry_run_path_selected,
      source_execution_open_precondition_set_id:$source_entry.execution_open_precondition_set_id,
      source_operator_evidence_precondition_id:$source_entry.operator_evidence_precondition_id,
      source_operator_acceptance_recording_precondition_id:$source_entry.operator_acceptance_recording_precondition_id,
      source_ledger_persistence_precondition_id:$source_entry.ledger_persistence_precondition_id,
      source_receipt_persistence_precondition_id:$source_entry.receipt_persistence_precondition_id,
      source_tool_registry_registration_precondition_id:$source_entry.tool_registry_registration_precondition_id,
      source_registry_lookup_precondition_id:$source_entry.registry_lookup_precondition_id,
      source_tool_invocation_precondition_id:$source_entry.tool_invocation_precondition_id,
      source_connector_start_precondition_id:$source_entry.connector_start_precondition_id,
      source_runtime_write_precondition_id:$source_entry.runtime_write_precondition_id,
      source_live_execution_precondition_id:$source_entry.live_execution_precondition_id,
      source_execution_open_denial_receipt_id:$source_entry.execution_open_denial_receipt_id,
      source_execution_open_idempotency_key:$source_entry.execution_open_idempotency_key,
      operator_evidence_packet_id:evidence_packet($kind),
      operator_evidence_artifact_ref_id:evidence_artifact_ref($kind),
      operator_evidence_items:evidence_items,
      acceptance_recording_prerequisite_link_id:prerequisite("acceptance-recording"; $kind),
      ledger_persistence_prerequisite_link_id:prerequisite("ledger-persistence"; $kind),
      receipt_persistence_prerequisite_link_id:prerequisite("receipt-persistence"; $kind),
      tool_registry_registration_prerequisite_link_id:prerequisite("tool-registry-registration"; $kind),
      registry_lookup_prerequisite_link_id:prerequisite("registry-lookup"; $kind),
      tool_invocation_prerequisite_link_id:prerequisite("tool-invocation"; $kind),
      connector_start_prerequisite_link_id:prerequisite("connector-start"; $kind),
      runtime_write_prerequisite_link_id:prerequisite("runtime-write"; $kind),
      live_execution_prerequisite_link_id:prerequisite("live-execution"; $kind),
      evidence_packet_denial_receipt_id:denial_receipt($kind),
      evidence_packet_idempotency_key:idempotency_key($kind),
      first_operator_evidence_packet_id:evidence_packet($kind),
      second_operator_evidence_packet_id:evidence_packet($kind),
      first_evidence_packet_denial_receipt_id:denial_receipt($kind),
      second_evidence_packet_denial_receipt_id:denial_receipt($kind),
      first_evidence_packet_idempotency_key:idempotency_key($kind),
      second_evidence_packet_idempotency_key:idempotency_key($kind),
      operator_evidence_packet_id_projected:true,
      operator_evidence_artifact_ref_projected:true,
      acceptance_recording_prerequisite_link_projected:true,
      ledger_persistence_prerequisite_link_projected:true,
      receipt_persistence_prerequisite_link_projected:true,
      tool_registry_registration_prerequisite_link_projected:true,
      registry_lookup_prerequisite_link_projected:true,
      tool_invocation_prerequisite_link_projected:true,
      connector_start_prerequisite_link_projected:true,
      runtime_write_prerequisite_link_projected:true,
      live_execution_prerequisite_link_projected:true,
      evidence_packet_denial_receipt_projected:true,
      evidence_packet_idempotency_key_projected:true,
      stable_operator_evidence_packet:true,
      unique_operator_evidence_packet:true,
      stable_evidence_packet_denial_receipt:true,
      unique_evidence_packet_denial_receipt:true,
      stable_evidence_packet_idempotency_key:true,
      unique_evidence_packet_idempotency_key:true,
      feature_gate_opened:$source_entry.feature_gate_opened,
      dry_run_executed:$source_entry.dry_run_executed,
      operator_evidence_packet_sent:false,
      operator_evidence_packet_persisted:false,
      operator_evidence_recorded:false,
      operator_acceptance_recorded:$source_entry.operator_acceptance_recorded,
      ledger_written:$source_entry.ledger_written,
      receipt_persisted:$source_entry.receipt_persisted,
      tool_registered:$source_entry.tool_registered,
      registry_lookup_executed:$source_entry.registry_lookup_executed,
      tool_invoked:$source_entry.tool_invoked,
      mcp_server_started:$source_entry.mcp_server_started,
      app_connector_started:$source_entry.app_connector_started,
      runtime_event_log_written:$source_entry.runtime_event_log_written,
      sqlite_written:$source_entry.sqlite_written,
      live_execution_started:$source_entry.live_execution_started
    };
  ($source[0]) as $source_report |
  ($source_report.entries | map(entry(.))) as $entries |
  ($entries | length) as $entry_count |
  ($entries | map(select(.dry_run_path_selected == true)) | length) as $selected_count |
  ($entries | map(select(.dry_run_path_selected == false)) | length) as $non_selected_count |
  ($entries | map(.operator_evidence_items | length) | add) as $evidence_item_count |
  ($entries | map(.first_operator_evidence_packet_id) | unique | length) as $unique_packet_count |
  ($entries | map(.first_evidence_packet_denial_receipt_id) | unique | length) as $unique_denial_receipt_count |
  ($entries | map(.first_evidence_packet_idempotency_key) | unique | length) as $unique_idempotency_key_count |
  ($source_report.execution_open_preconditions_readback_ready == true
    and $source_report.candidate_count == 2
    and $source_report.precondition_entry_count == 2
    and $source_report.selected_read_only_status_tool_count == 1
    and $source_report.non_selected_preflight_boundary_count == 1
    and $source_report.execution_open_precondition_set_projected_count == 2
    and $source_report.operator_evidence_precondition_projected_count == 2
    and $source_report.operator_acceptance_recording_precondition_projected_count == 2
    and $source_report.ledger_persistence_precondition_projected_count == 2
    and $source_report.receipt_persistence_precondition_projected_count == 2
    and $source_report.tool_registry_registration_precondition_projected_count == 2
    and $source_report.registry_lookup_precondition_projected_count == 2
    and $source_report.tool_invocation_precondition_projected_count == 2
    and $source_report.connector_start_precondition_projected_count == 2
    and $source_report.runtime_write_precondition_projected_count == 2
    and $source_report.live_execution_precondition_projected_count == 2
    and $source_report.execution_open_denial_receipt_projected_count == 2
    and $source_report.execution_open_idempotency_key_projected_count == 2
    and $source_report.feature_gate_opened_count == 0
    and $source_report.dry_run_executed_count == 0
    and $source_report.operator_acceptance_recorded_count == 0
    and $source_report.ledger_written_count == 0
    and $source_report.receipt_persisted_count == 0
    and $source_report.tool_registered_count == 0
    and $source_report.registry_lookup_executed_count == 0
    and $source_report.tool_invoked_count == 0
    and $source_report.live_execution_started_count == 0
    and $lib_export_present == true
    and $entry_count == 2
    and $selected_count == 1
    and $non_selected_count == 1
    and $evidence_item_count == 20
    and $unique_packet_count == 2
    and $unique_denial_receipt_count == 2
    and $unique_idempotency_key_count == 2
  ) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_gate",
    schema_version:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_packet_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_execution_open_preconditions_ready:$source_report.execution_open_preconditions_readback_ready,
    lib_export_present:$lib_export_present,
    local_gate:$gate,
    architecture_note:$doc,
    candidate_count:$source_report.candidate_count,
    evidence_packet_entry_count:$entry_count,
    selected_read_only_status_tool_count:$selected_count,
    non_selected_preflight_boundary_count:$non_selected_count,
    operator_evidence_packet_id_projected_count:($entries | map(select(.operator_evidence_packet_id_projected == true)) | length),
    operator_evidence_artifact_ref_projected_count:($entries | map(select(.operator_evidence_artifact_ref_projected == true)) | length),
    operator_evidence_item_count:$evidence_item_count,
    acceptance_recording_prerequisite_link_projected_count:($entries | map(select(.acceptance_recording_prerequisite_link_projected == true)) | length),
    ledger_persistence_prerequisite_link_projected_count:($entries | map(select(.ledger_persistence_prerequisite_link_projected == true)) | length),
    receipt_persistence_prerequisite_link_projected_count:($entries | map(select(.receipt_persistence_prerequisite_link_projected == true)) | length),
    tool_registry_registration_prerequisite_link_projected_count:($entries | map(select(.tool_registry_registration_prerequisite_link_projected == true)) | length),
    registry_lookup_prerequisite_link_projected_count:($entries | map(select(.registry_lookup_prerequisite_link_projected == true)) | length),
    tool_invocation_prerequisite_link_projected_count:($entries | map(select(.tool_invocation_prerequisite_link_projected == true)) | length),
    connector_start_prerequisite_link_projected_count:($entries | map(select(.connector_start_prerequisite_link_projected == true)) | length),
    runtime_write_prerequisite_link_projected_count:($entries | map(select(.runtime_write_prerequisite_link_projected == true)) | length),
    live_execution_prerequisite_link_projected_count:($entries | map(select(.live_execution_prerequisite_link_projected == true)) | length),
    evidence_packet_denial_receipt_projected_count:($entries | map(select(.evidence_packet_denial_receipt_projected == true)) | length),
    evidence_packet_idempotency_key_projected_count:($entries | map(select(.evidence_packet_idempotency_key_projected == true)) | length),
    stable_operator_evidence_packet_count:($entries | map(select(.stable_operator_evidence_packet == true)) | length),
    unique_operator_evidence_packet_count:$unique_packet_count,
    stable_evidence_packet_denial_receipt_count:($entries | map(select(.stable_evidence_packet_denial_receipt == true)) | length),
    unique_evidence_packet_denial_receipt_count:$unique_denial_receipt_count,
    stable_evidence_packet_idempotency_key_count:($entries | map(select(.stable_evidence_packet_idempotency_key == true)) | length),
    unique_evidence_packet_idempotency_key_count:$unique_idempotency_key_count,
    operator_evidence_packet_mismatch_count:($entries | map(select(.stable_operator_evidence_packet == false)) | length),
    duplicate_operator_evidence_packet_count:($entry_count - $unique_packet_count),
    evidence_packet_denial_receipt_mismatch_count:($entries | map(select(.stable_evidence_packet_denial_receipt == false)) | length),
    duplicate_evidence_packet_denial_receipt_count:($entry_count - $unique_denial_receipt_count),
    evidence_packet_idempotency_mismatch_count:($entries | map(select(.stable_evidence_packet_idempotency_key == false)) | length),
    duplicate_evidence_packet_idempotency_key_count:($entry_count - $unique_idempotency_key_count),
    feature_gate_opened_count:($entries | map(select(.feature_gate_opened == true)) | length),
    dry_run_executed_count:($entries | map(select(.dry_run_executed == true)) | length),
    operator_evidence_packet_sent_count:($entries | map(select(.operator_evidence_packet_sent == true)) | length),
    operator_evidence_packet_persisted_count:($entries | map(select(.operator_evidence_packet_persisted == true)) | length),
    operator_evidence_recorded_count:($entries | map(select(.operator_evidence_recorded == true)) | length),
    operator_acceptance_recorded_count:($entries | map(select(.operator_acceptance_recorded == true)) | length),
    ledger_written_count:($entries | map(select(.ledger_written == true)) | length),
    receipt_persisted_count:($entries | map(select(.receipt_persisted == true)) | length),
    tool_registered_count:($entries | map(select(.tool_registered == true)) | length),
    registry_lookup_executed_count:($entries | map(select(.registry_lookup_executed == true)) | length),
    tool_invoked_count:($entries | map(select(.tool_invoked == true)) | length),
    mcp_server_started_count:($entries | map(select(.mcp_server_started == true)) | length),
    app_connector_started_count:($entries | map(select(.app_connector_started == true)) | length),
    runtime_event_log_written_count:($entries | map(select(.runtime_event_log_written == true)) | length),
    sqlite_written_count:($entries | map(select(.sqlite_written == true)) | length),
    live_execution_started_count:($entries | map(select(.live_execution_started == true)) | length),
    operator_evidence_packet_readback_ready:$ready,
    feature_gate_open_allowed:false,
    dry_run_execution_allowed:false,
    operator_evidence_packet_send_allowed:false,
    operator_evidence_packet_persistence_allowed:false,
    operator_evidence_recording_allowed:false,
    operator_acceptance_recording_allowed:false,
    ledger_persistence_allowed:false,
    receipt_persistence_allowed:false,
    tool_registry_registration_allowed:false,
    registry_lookup_execution_allowed:false,
    tool_invocation_allowed:false,
    connector_start_allowed:false,
    runtime_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    live_execution_allowed:false,
    entries:$entries,
    blockers:[
      "operator_evidence_packet_send_disabled",
      "operator_evidence_packet_persistence_disabled",
      "operator_evidence_recording_disabled",
      "operator_acceptance_recording_disabled",
      "ledger_persistence_disabled",
      "receipt_persistence_disabled",
      "tool_registry_registration_disabled",
      "registry_lookup_execution_disabled",
      "tool_invocation_disabled",
      "connector_start_disabled",
      "runtime_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_boundary_readback",
    side_effect_free:true,
    side_effects:{
      filesystem_written:false,
      feature_gate_opened:false,
      dry_run_executed:false,
      operator_evidence_packet_sent:false,
      operator_evidence_packet_persisted:false,
      operator_evidence_recorded:false,
      operator_acceptance_recorded:false,
      ledger_persisted:false,
      receipt_persisted:false,
      tool_registered:false,
      tool_registry_mutated:false,
      registry_lookup_executed:false,
      tool_invoked:false,
      connector_started:false,
      runtime_event_log_written:false,
      sqlite_written:false,
      credential_read:false,
      external_network_used:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      telegram_transport_mutated:false,
      package_or_release_written:false,
      live_execution_started:false
    }
  }
  '
