#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-open-preconditions-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_DENIAL_RECEIPT_READBACK_2026-07-01.md"

fail() {
  printf 'hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-persistence-denial-receipt-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable operator evidence acceptance recording open preconditions report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing operator evidence acceptance recording persistence denial receipt Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing operator evidence acceptance recording persistence denial receipt architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the operator evidence acceptance recording persistence denial receipt report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render operator evidence acceptance recording open preconditions report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "operator evidence acceptance recording open preconditions report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-persistence-denial-receipt-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PLUGIN_TOOL_INVOCATION_READ_ONLY_STATUS_DRY_RUN_OPERATOR_EVIDENCE_ACCEPTANCE_RECORDING_PERSISTENCE_DENIAL_RECEIPT_READBACK_2026-07-01.md" \
  '
  def suffix($kind):
    if $kind == "mcp_server" then "local-mcp:read-only-status-dry-run"
    elif $kind == "app_connector" then "local-app:not-selected"
    else "unknown:not-selected"
    end;
  def persistence_receipt($kind):
    "operator-evidence-acceptance-recording-persistence-denial-receipt:hepta-system:" + suffix($kind) + ":acceptance-record-persistence-disabled";
  def persistence_digest($kind):
    "sha256:operator-evidence-acceptance-recording-persistence-denial:hepta-system:" + suffix($kind);
  def persistence_write_denial($kind):
    "operator-evidence-acceptance-recording-persistence-write-denial:hepta-system:" + suffix($kind);
  def anchor($name; $kind):
    "operator-evidence-acceptance-recording-persistence-anchor:" + $name + ":hepta-system:" + suffix($kind);
  def persistence_idempotency($kind):
    "operator-evidence-acceptance-recording-persistence-idempotency:hepta-system:" + suffix($kind);
  def entry($source_entry):
    ($source_entry.contribution_kind) as $kind |
    {
      candidate_tool_id:$source_entry.candidate_tool_id,
      contribution_kind:$kind,
      dry_run_path_selected:$source_entry.dry_run_path_selected,
      source_acceptance_recording_open_precondition_set_id:$source_entry.acceptance_recording_open_precondition_set_id,
      source_evidence_artifact_presence_precondition_id:$source_entry.evidence_artifact_presence_precondition_id,
      source_operator_identity_precondition_id:$source_entry.operator_identity_precondition_id,
      source_acceptance_recording_persistence_precondition_id:$source_entry.acceptance_recording_persistence_precondition_id,
      source_ledger_persistence_precondition_id:$source_entry.ledger_persistence_precondition_id,
      source_receipt_persistence_precondition_id:$source_entry.receipt_persistence_precondition_id,
      source_tool_registry_registration_precondition_id:$source_entry.tool_registry_registration_precondition_id,
      source_registry_lookup_precondition_id:$source_entry.registry_lookup_precondition_id,
      source_tool_invocation_precondition_id:$source_entry.tool_invocation_precondition_id,
      source_runtime_write_precondition_id:$source_entry.runtime_write_precondition_id,
      source_live_execution_precondition_id:$source_entry.live_execution_precondition_id,
      source_acceptance_recording_open_denial_receipt_id:$source_entry.acceptance_recording_open_denial_receipt_id,
      source_acceptance_recording_open_idempotency_key:$source_entry.acceptance_recording_open_idempotency_key,
      persistence_denial_receipt_id:persistence_receipt($kind),
      persistence_denial_receipt_digest:persistence_digest($kind),
      persistence_write_denial_id:persistence_write_denial($kind),
      non_recording_denial_receipt_anchor_id:anchor("non-recording-denial-receipt"; $kind),
      acceptance_recording_open_denial_receipt_anchor_id:anchor("open-denial-receipt"; $kind),
      ledger_persistence_denial_anchor_id:anchor("ledger-persistence-denial"; $kind),
      receipt_persistence_denial_anchor_id:anchor("receipt-persistence-denial"; $kind),
      tool_invocation_denial_anchor_id:anchor("tool-invocation-denial"; $kind),
      runtime_write_denial_anchor_id:anchor("runtime-write-denial"; $kind),
      live_execution_denial_anchor_id:anchor("live-execution-denial"; $kind),
      persistence_idempotency_key:persistence_idempotency($kind),
      first_persistence_denial_receipt_id:persistence_receipt($kind),
      second_persistence_denial_receipt_id:persistence_receipt($kind),
      first_persistence_denial_receipt_digest:persistence_digest($kind),
      second_persistence_denial_receipt_digest:persistence_digest($kind),
      first_persistence_idempotency_key:persistence_idempotency($kind),
      second_persistence_idempotency_key:persistence_idempotency($kind),
      persistence_denial_receipt_projected:true,
      persistence_denial_receipt_digest_projected:true,
      persistence_write_denial_projected:true,
      non_recording_denial_receipt_anchor_projected:true,
      acceptance_recording_open_denial_receipt_anchor_projected:true,
      ledger_persistence_denial_anchor_projected:true,
      receipt_persistence_denial_anchor_projected:true,
      tool_invocation_denial_anchor_projected:true,
      runtime_write_denial_anchor_projected:true,
      live_execution_denial_anchor_projected:true,
      persistence_idempotency_key_projected:true,
      stable_persistence_denial_receipt:true,
      unique_persistence_denial_receipt:true,
      stable_persistence_denial_receipt_digest:true,
      unique_persistence_denial_receipt_digest:true,
      stable_persistence_idempotency_key:true,
      unique_persistence_idempotency_key:true,
      feature_gate_opened:$source_entry.feature_gate_opened,
      dry_run_executed:$source_entry.dry_run_executed,
      operator_evidence_packet_sent:$source_entry.operator_evidence_packet_sent,
      operator_evidence_packet_persisted:$source_entry.operator_evidence_packet_persisted,
      operator_evidence_recorded:$source_entry.operator_evidence_recorded,
      operator_acceptance_recorded:$source_entry.operator_acceptance_recorded,
      acceptance_record_persisted:$source_entry.acceptance_record_persisted,
      persistence_denial_receipt_persisted:false,
      non_recording_denial_receipt_persisted:$source_entry.non_recording_denial_receipt_persisted,
      idempotency_index_written:false,
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
  ($entries | map(.first_persistence_denial_receipt_id) | unique | length) as $unique_receipt_count |
  ($entries | map(.first_persistence_denial_receipt_digest) | unique | length) as $unique_digest_count |
  ($entries | map(.first_persistence_idempotency_key) | unique | length) as $unique_idempotency_count |
  ($source_report.acceptance_recording_open_preconditions_readback_ready == true
    and $source_report.candidate_count == 2
    and $source_report.precondition_entry_count == 2
    and $source_report.acceptance_recording_persistence_precondition_projected_count == 2
    and $source_report.ledger_persistence_precondition_projected_count == 2
    and $source_report.receipt_persistence_precondition_projected_count == 2
    and $source_report.tool_registry_registration_precondition_projected_count == 2
    and $source_report.registry_lookup_precondition_projected_count == 2
    and $source_report.tool_invocation_precondition_projected_count == 2
    and $source_report.runtime_write_precondition_projected_count == 2
    and $source_report.live_execution_precondition_projected_count == 2
    and $source_report.acceptance_recording_open_denial_receipt_projected_count == 2
    and $source_report.acceptance_recording_open_idempotency_key_projected_count == 2
    and $source_report.operator_acceptance_recorded_count == 0
    and $source_report.acceptance_record_persisted_count == 0
    and $source_report.non_recording_denial_receipt_persisted_count == 0
    and $source_report.ledger_written_count == 0
    and $source_report.receipt_persisted_count == 0
    and $source_report.tool_registered_count == 0
    and $source_report.registry_lookup_executed_count == 0
    and $source_report.tool_invoked_count == 0
    and $entry_count == 2
    and $selected_count == 1
    and $non_selected_count == 1
    and ($entries | map(select(.persistence_denial_receipt_projected == true)) | length) == 2
    and ($entries | map(select(.persistence_denial_receipt_digest_projected == true)) | length) == 2
    and ($entries | map(select(.persistence_write_denial_projected == true)) | length) == 2
    and ($entries | map(select(.non_recording_denial_receipt_anchor_projected == true)) | length) == 2
    and ($entries | map(select(.acceptance_recording_open_denial_receipt_anchor_projected == true)) | length) == 2
    and ($entries | map(select(.ledger_persistence_denial_anchor_projected == true)) | length) == 2
    and ($entries | map(select(.receipt_persistence_denial_anchor_projected == true)) | length) == 2
    and ($entries | map(select(.tool_invocation_denial_anchor_projected == true)) | length) == 2
    and ($entries | map(select(.runtime_write_denial_anchor_projected == true)) | length) == 2
    and ($entries | map(select(.live_execution_denial_anchor_projected == true)) | length) == 2
    and ($entries | map(select(.persistence_idempotency_key_projected == true)) | length) == 2
    and ($entries | map(select(.stable_persistence_denial_receipt == true)) | length) == 2
    and $unique_receipt_count == 2
    and ($entries | map(select(.stable_persistence_denial_receipt_digest == true)) | length) == 2
    and $unique_digest_count == 2
    and ($entries | map(select(.stable_persistence_idempotency_key == true)) | length) == 2
    and $unique_idempotency_count == 2
    and ($entries | map(select(.feature_gate_opened == true or .dry_run_executed == true or .operator_evidence_packet_sent == true or .operator_evidence_packet_persisted == true or .operator_evidence_recorded == true or .operator_acceptance_recorded == true or .acceptance_record_persisted == true or .persistence_denial_receipt_persisted == true or .non_recording_denial_receipt_persisted == true or .idempotency_index_written == true or .ledger_written == true or .receipt_persisted == true or .tool_registered == true or .registry_lookup_executed == true or .tool_invoked == true or .mcp_server_started == true or .app_connector_started == true or .runtime_event_log_written == true or .sqlite_written == true or .live_execution_started == true)) | length) == 0) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_gate",
    schema_version:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_denial_receipt_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_acceptance_recording_open_preconditions_readback_ready:$source_report.acceptance_recording_open_preconditions_readback_ready,
    lib_export_present:$lib_export_present,
    candidate_count:$source_report.candidate_count,
    persistence_denial_entry_count:$entry_count,
    selected_read_only_status_tool_count:$selected_count,
    non_selected_preflight_boundary_count:$non_selected_count,
    persistence_denial_receipt_projected_count:($entries | map(select(.persistence_denial_receipt_projected == true)) | length),
    persistence_denial_receipt_digest_projected_count:($entries | map(select(.persistence_denial_receipt_digest_projected == true)) | length),
    persistence_write_denial_projected_count:($entries | map(select(.persistence_write_denial_projected == true)) | length),
    non_recording_denial_receipt_anchor_projected_count:($entries | map(select(.non_recording_denial_receipt_anchor_projected == true)) | length),
    acceptance_recording_open_denial_receipt_anchor_projected_count:($entries | map(select(.acceptance_recording_open_denial_receipt_anchor_projected == true)) | length),
    ledger_persistence_denial_anchor_projected_count:($entries | map(select(.ledger_persistence_denial_anchor_projected == true)) | length),
    receipt_persistence_denial_anchor_projected_count:($entries | map(select(.receipt_persistence_denial_anchor_projected == true)) | length),
    tool_invocation_denial_anchor_projected_count:($entries | map(select(.tool_invocation_denial_anchor_projected == true)) | length),
    runtime_write_denial_anchor_projected_count:($entries | map(select(.runtime_write_denial_anchor_projected == true)) | length),
    live_execution_denial_anchor_projected_count:($entries | map(select(.live_execution_denial_anchor_projected == true)) | length),
    persistence_idempotency_key_projected_count:($entries | map(select(.persistence_idempotency_key_projected == true)) | length),
    stable_persistence_denial_receipt_count:($entries | map(select(.stable_persistence_denial_receipt == true)) | length),
    unique_persistence_denial_receipt_count:$unique_receipt_count,
    stable_persistence_denial_receipt_digest_count:($entries | map(select(.stable_persistence_denial_receipt_digest == true)) | length),
    unique_persistence_denial_receipt_digest_count:$unique_digest_count,
    stable_persistence_idempotency_key_count:($entries | map(select(.stable_persistence_idempotency_key == true)) | length),
    unique_persistence_idempotency_key_count:$unique_idempotency_count,
    persistence_denial_receipt_mismatch_count:($entries | map(select(.stable_persistence_denial_receipt == false)) | length),
    duplicate_persistence_denial_receipt_count:($entry_count - $unique_receipt_count),
    persistence_denial_receipt_digest_mismatch_count:($entries | map(select(.stable_persistence_denial_receipt_digest == false)) | length),
    duplicate_persistence_denial_receipt_digest_count:($entry_count - $unique_digest_count),
    persistence_idempotency_mismatch_count:($entries | map(select(.stable_persistence_idempotency_key == false)) | length),
    duplicate_persistence_idempotency_key_count:($entry_count - $unique_idempotency_count),
    feature_gate_opened_count:($entries | map(select(.feature_gate_opened == true)) | length),
    dry_run_executed_count:($entries | map(select(.dry_run_executed == true)) | length),
    operator_evidence_packet_sent_count:($entries | map(select(.operator_evidence_packet_sent == true)) | length),
    operator_evidence_packet_persisted_count:($entries | map(select(.operator_evidence_packet_persisted == true)) | length),
    operator_evidence_recorded_count:($entries | map(select(.operator_evidence_recorded == true)) | length),
    operator_acceptance_recorded_count:($entries | map(select(.operator_acceptance_recorded == true)) | length),
    acceptance_record_persisted_count:($entries | map(select(.acceptance_record_persisted == true)) | length),
    persistence_denial_receipt_persisted_count:($entries | map(select(.persistence_denial_receipt_persisted == true)) | length),
    non_recording_denial_receipt_persisted_count:($entries | map(select(.non_recording_denial_receipt_persisted == true)) | length),
    idempotency_index_written_count:($entries | map(select(.idempotency_index_written == true)) | length),
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
    persistence_denial_receipt_readback_ready:$ready,
    feature_gate_open_allowed:false,
    dry_run_execution_allowed:false,
    operator_evidence_packet_send_allowed:false,
    operator_evidence_packet_persistence_allowed:false,
    operator_evidence_recording_allowed:false,
    operator_acceptance_recording_allowed:false,
    acceptance_record_persistence_allowed:false,
    persistence_denial_receipt_persistence_allowed:false,
    non_recording_denial_receipt_persistence_allowed:false,
    idempotency_index_write_allowed:false,
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
      "acceptance_recording_persistence_disabled",
      "acceptance_record_persistence_disabled",
      "persistence_denial_receipt_persistence_disabled",
      "non_recording_denial_receipt_persistence_disabled",
      "idempotency_index_write_disabled",
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
      "hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback"
    ],
    recommended_next_gate:"hepta_systems_plugin_tool_invocation_read_only_status_dry_run_operator_evidence_acceptance_recording_persistence_open_preconditions_readback",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      filesystem_written:false,
      feature_gate_opened:false,
      dry_run_executed:false,
      operator_evidence_packet_sent:false,
      operator_evidence_packet_persisted:false,
      operator_evidence_recorded:false,
      operator_acceptance_recorded:false,
      acceptance_record_persisted:false,
      persistence_denial_receipt_persisted:false,
      non_recording_denial_receipt_persisted:false,
      idempotency_index_written:false,
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
  }'
