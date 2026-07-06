#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-boundary-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_OPEN_PRECONDITIONS_READBACK_2026-07-01.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-open-preconditions-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable operator evidence acceptance recording boundary report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing operator evidence acceptance recording open preconditions Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing operator evidence acceptance recording open preconditions architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the operator evidence acceptance recording open preconditions report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render operator evidence acceptance recording boundary report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "operator evidence acceptance recording boundary report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-open-preconditions-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_OPEN_PRECONDITIONS_READBACK_2026-07-01.md" \
  '
  def suffix($kind):
    if $kind == "mcp_server" then "local-mcp:read-only-status-dry-run"
    elif $kind == "app_connector" then "local-app:not-selected"
    else "unknown:not-selected"
    end;
  def precondition_set($kind):
    "operator-evidence-acceptance-recording-open-preconditions:hepta-system:" + suffix($kind);
  def precondition($name; $kind):
    "operator-evidence-acceptance-recording-open-precondition:" + $name + ":hepta-system:" + suffix($kind);
  def denial_receipt($kind):
    "operator-evidence-acceptance-recording-open-denial:hepta-system:" + suffix($kind) + ":missing-evidence-identity-acceptance-ledger-receipt-registration";
  def idempotency_key($kind):
    "operator-evidence-acceptance-recording-open-idempotency:hepta-system:" + suffix($kind);
  def precondition_items:
    [
      "evidence_artifact_presence_required",
      "operator_identity_required",
      "acceptance_recording_persistence_required",
      "ledger_persistence_required",
      "receipt_persistence_required",
      "tool_registry_registration_required",
      "registry_lookup_execution_required",
      "read_only_tool_invocation_required",
      "connector_start_boundary_required",
      "runtime_write_boundary_required",
      "live_execution_boundary_required"
    ];
  def entry($source_entry):
    ($source_entry.contribution_kind) as $kind |
    {
      candidate_tool_id:$source_entry.candidate_tool_id,
      contribution_kind:$kind,
      dry_run_path_selected:$source_entry.dry_run_path_selected,
      source_acceptance_recording_boundary_id:$source_entry.acceptance_recording_boundary_id,
      source_evidence_artifact_ref_link_id:$source_entry.evidence_artifact_ref_link_id,
      source_operator_identity_link_id:$source_entry.operator_identity_link_id,
      source_acceptance_record_prerequisite_id:$source_entry.acceptance_record_prerequisite_id,
      source_non_recording_denial_receipt_id:$source_entry.non_recording_denial_receipt_id,
      source_ledger_persistence_closure_anchor_id:$source_entry.ledger_persistence_closure_anchor_id,
      source_receipt_persistence_closure_anchor_id:$source_entry.receipt_persistence_closure_anchor_id,
      source_tool_invocation_closure_anchor_id:$source_entry.tool_invocation_closure_anchor_id,
      source_runtime_write_closure_anchor_id:$source_entry.runtime_write_closure_anchor_id,
      source_live_execution_closure_anchor_id:$source_entry.live_execution_closure_anchor_id,
      source_acceptance_recording_boundary_idempotency_key:$source_entry.acceptance_recording_boundary_idempotency_key,
      acceptance_recording_open_precondition_set_id:precondition_set($kind),
      evidence_artifact_presence_precondition_id:precondition("evidence-artifact-presence"; $kind),
      operator_identity_precondition_id:precondition("operator-identity"; $kind),
      acceptance_recording_persistence_precondition_id:precondition("acceptance-recording-persistence"; $kind),
      ledger_persistence_precondition_id:precondition("ledger-persistence"; $kind),
      receipt_persistence_precondition_id:precondition("receipt-persistence"; $kind),
      tool_registry_registration_precondition_id:precondition("tool-registry-registration"; $kind),
      registry_lookup_precondition_id:precondition("registry-lookup"; $kind),
      tool_invocation_precondition_id:precondition("tool-invocation"; $kind),
      connector_start_precondition_id:precondition("connector-start"; $kind),
      runtime_write_precondition_id:precondition("runtime-write-boundary"; $kind),
      live_execution_precondition_id:precondition("live-execution-boundary"; $kind),
      acceptance_recording_open_precondition_items:precondition_items,
      acceptance_recording_open_denial_receipt_id:denial_receipt($kind),
      acceptance_recording_open_idempotency_key:idempotency_key($kind),
      first_acceptance_recording_open_precondition_set_id:precondition_set($kind),
      second_acceptance_recording_open_precondition_set_id:precondition_set($kind),
      first_acceptance_recording_open_denial_receipt_id:denial_receipt($kind),
      second_acceptance_recording_open_denial_receipt_id:denial_receipt($kind),
      first_acceptance_recording_open_idempotency_key:idempotency_key($kind),
      second_acceptance_recording_open_idempotency_key:idempotency_key($kind),
      acceptance_recording_open_precondition_set_projected:true,
      evidence_artifact_presence_precondition_projected:true,
      operator_identity_precondition_projected:true,
      acceptance_recording_persistence_precondition_projected:true,
      ledger_persistence_precondition_projected:true,
      receipt_persistence_precondition_projected:true,
      tool_registry_registration_precondition_projected:true,
      registry_lookup_precondition_projected:true,
      tool_invocation_precondition_projected:true,
      connector_start_precondition_projected:true,
      runtime_write_precondition_projected:true,
      live_execution_precondition_projected:true,
      acceptance_recording_open_denial_receipt_projected:true,
      acceptance_recording_open_idempotency_key_projected:true,
      stable_acceptance_recording_open_precondition_set:true,
      unique_acceptance_recording_open_precondition_set:true,
      stable_acceptance_recording_open_denial_receipt:true,
      unique_acceptance_recording_open_denial_receipt:true,
      stable_acceptance_recording_open_idempotency_key:true,
      unique_acceptance_recording_open_idempotency_key:true,
      feature_gate_opened:$source_entry.feature_gate_opened,
      dry_run_executed:$source_entry.dry_run_executed,
      operator_evidence_packet_sent:$source_entry.operator_evidence_packet_sent,
      operator_evidence_packet_persisted:$source_entry.operator_evidence_packet_persisted,
      operator_evidence_recorded:$source_entry.operator_evidence_recorded,
      operator_acceptance_recorded:$source_entry.operator_acceptance_recorded,
      acceptance_record_persisted:$source_entry.acceptance_record_persisted,
      non_recording_denial_receipt_persisted:$source_entry.non_recording_denial_receipt_persisted,
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
  ($entries | length) as $precondition_entry_count |
  ($entries | map(select(.dry_run_path_selected == true)) | length) as $selected_read_only_status_tool_count |
  ($entries | map(select(.dry_run_path_selected == false)) | length) as $non_selected_preflight_boundary_count |
  ($entries | map(.acceptance_recording_open_precondition_items | length) | add) as $precondition_item_count |
  ($entries | map(.first_acceptance_recording_open_precondition_set_id) | unique | length) as $unique_precondition_set_count |
  ($entries | map(.first_acceptance_recording_open_denial_receipt_id) | unique | length) as $unique_denial_receipt_count |
  ($entries | map(.first_acceptance_recording_open_idempotency_key) | unique | length) as $unique_idempotency_key_count |
  ($source_report.acceptance_recording_boundary_readback_ready == true
    and $source_report.candidate_count == 2
    and $source_report.boundary_entry_count == 2
    and $source_report.selected_read_only_status_tool_count == 1
    and $source_report.non_selected_preflight_boundary_count == 1
    and $source_report.acceptance_recording_boundary_id_projected_count == 2
    and $source_report.evidence_artifact_ref_link_projected_count == 2
    and $source_report.operator_identity_link_projected_count == 2
    and $source_report.acceptance_record_prerequisite_projected_count == 2
    and $source_report.non_recording_denial_receipt_projected_count == 2
    and $source_report.ledger_persistence_closure_anchor_projected_count == 2
    and $source_report.receipt_persistence_closure_anchor_projected_count == 2
    and $source_report.tool_invocation_closure_anchor_projected_count == 2
    and $source_report.runtime_write_closure_anchor_projected_count == 2
    and $source_report.live_execution_closure_anchor_projected_count == 2
    and $source_report.acceptance_recording_boundary_idempotency_key_projected_count == 2
    and $source_report.feature_gate_opened_count == 0
    and $source_report.dry_run_executed_count == 0
    and $source_report.operator_evidence_packet_sent_count == 0
    and $source_report.operator_evidence_packet_persisted_count == 0
    and $source_report.operator_evidence_recorded_count == 0
    and $source_report.operator_acceptance_recorded_count == 0
    and $source_report.acceptance_record_persisted_count == 0
    and $source_report.non_recording_denial_receipt_persisted_count == 0
    and $source_report.ledger_written_count == 0
    and $source_report.receipt_persisted_count == 0
    and $source_report.tool_registered_count == 0
    and $source_report.registry_lookup_executed_count == 0
    and $source_report.tool_invoked_count == 0
    and $source_report.live_execution_started_count == 0
    and $lib_export_present == true
    and $precondition_entry_count == 2
    and $selected_read_only_status_tool_count == 1
    and $non_selected_preflight_boundary_count == 1
    and $precondition_item_count == 22
    and $unique_precondition_set_count == 2
    and $unique_denial_receipt_count == 2
    and $unique_idempotency_key_count == 2
  ) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_gate",
    schema_version:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_open_preconditions_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_acceptance_recording_boundary_readback_ready:$source_report.acceptance_recording_boundary_readback_ready,
    lib_export_present:$lib_export_present,
    local_gate:$gate,
    architecture_note:$doc,
    candidate_count:$source_report.candidate_count,
    precondition_entry_count:$precondition_entry_count,
    selected_read_only_status_tool_count:$selected_read_only_status_tool_count,
    non_selected_preflight_boundary_count:$non_selected_preflight_boundary_count,
    acceptance_recording_open_precondition_set_projected_count:($entries | map(select(.acceptance_recording_open_precondition_set_projected == true)) | length),
    evidence_artifact_presence_precondition_projected_count:($entries | map(select(.evidence_artifact_presence_precondition_projected == true)) | length),
    operator_identity_precondition_projected_count:($entries | map(select(.operator_identity_precondition_projected == true)) | length),
    acceptance_recording_persistence_precondition_projected_count:($entries | map(select(.acceptance_recording_persistence_precondition_projected == true)) | length),
    ledger_persistence_precondition_projected_count:($entries | map(select(.ledger_persistence_precondition_projected == true)) | length),
    receipt_persistence_precondition_projected_count:($entries | map(select(.receipt_persistence_precondition_projected == true)) | length),
    tool_registry_registration_precondition_projected_count:($entries | map(select(.tool_registry_registration_precondition_projected == true)) | length),
    registry_lookup_precondition_projected_count:($entries | map(select(.registry_lookup_precondition_projected == true)) | length),
    tool_invocation_precondition_projected_count:($entries | map(select(.tool_invocation_precondition_projected == true)) | length),
    connector_start_precondition_projected_count:($entries | map(select(.connector_start_precondition_projected == true)) | length),
    runtime_write_precondition_projected_count:($entries | map(select(.runtime_write_precondition_projected == true)) | length),
    live_execution_precondition_projected_count:($entries | map(select(.live_execution_precondition_projected == true)) | length),
    acceptance_recording_open_precondition_item_count:$precondition_item_count,
    acceptance_recording_open_denial_receipt_projected_count:($entries | map(select(.acceptance_recording_open_denial_receipt_projected == true)) | length),
    acceptance_recording_open_idempotency_key_projected_count:($entries | map(select(.acceptance_recording_open_idempotency_key_projected == true)) | length),
    stable_acceptance_recording_open_precondition_set_count:($entries | map(select(.stable_acceptance_recording_open_precondition_set == true)) | length),
    unique_acceptance_recording_open_precondition_set_count:$unique_precondition_set_count,
    stable_acceptance_recording_open_denial_receipt_count:($entries | map(select(.stable_acceptance_recording_open_denial_receipt == true)) | length),
    unique_acceptance_recording_open_denial_receipt_count:$unique_denial_receipt_count,
    stable_acceptance_recording_open_idempotency_key_count:($entries | map(select(.stable_acceptance_recording_open_idempotency_key == true)) | length),
    unique_acceptance_recording_open_idempotency_key_count:$unique_idempotency_key_count,
    acceptance_recording_open_precondition_mismatch_count:($entries | map(select(.stable_acceptance_recording_open_precondition_set == false)) | length),
    duplicate_acceptance_recording_open_precondition_count:($precondition_entry_count - $unique_precondition_set_count),
    acceptance_recording_open_denial_receipt_mismatch_count:($entries | map(select(.stable_acceptance_recording_open_denial_receipt == false)) | length),
    duplicate_acceptance_recording_open_denial_receipt_count:($precondition_entry_count - $unique_denial_receipt_count),
    acceptance_recording_open_idempotency_mismatch_count:($entries | map(select(.stable_acceptance_recording_open_idempotency_key == false)) | length),
    duplicate_acceptance_recording_open_idempotency_key_count:($precondition_entry_count - $unique_idempotency_key_count),
    feature_gate_opened_count:($entries | map(select(.feature_gate_opened == true)) | length),
    dry_run_executed_count:($entries | map(select(.dry_run_executed == true)) | length),
    operator_evidence_packet_sent_count:($entries | map(select(.operator_evidence_packet_sent == true)) | length),
    operator_evidence_packet_persisted_count:($entries | map(select(.operator_evidence_packet_persisted == true)) | length),
    operator_evidence_recorded_count:($entries | map(select(.operator_evidence_recorded == true)) | length),
    operator_acceptance_recorded_count:($entries | map(select(.operator_acceptance_recorded == true)) | length),
    acceptance_record_persisted_count:($entries | map(select(.acceptance_record_persisted == true)) | length),
    non_recording_denial_receipt_persisted_count:($entries | map(select(.non_recording_denial_receipt_persisted == true)) | length),
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
    acceptance_recording_open_preconditions_readback_ready:$ready,
    feature_gate_open_allowed:false,
    dry_run_execution_allowed:false,
    operator_evidence_packet_send_allowed:false,
    operator_evidence_packet_persistence_allowed:false,
    operator_evidence_recording_allowed:false,
    operator_acceptance_recording_allowed:false,
    acceptance_record_persistence_allowed:false,
    non_recording_denial_receipt_persistence_allowed:false,
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
      "evidence_artifact_missing",
      "operator_identity_missing",
      "operator_acceptance_recording_disabled",
      "acceptance_record_persistence_disabled",
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
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback",
    side_effect_free:true,
    side_effects:{
      report_written:false,
      feature_gate_opened:false,
      dry_run_executed:false,
      operator_evidence_packet_sent:false,
      operator_evidence_packet_persisted:false,
      operator_evidence_recorded:false,
      operator_acceptance_recorded:false,
      acceptance_record_persisted:false,
      non_recording_denial_receipt_persisted:false,
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
