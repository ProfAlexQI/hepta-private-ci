#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-plugin-tool-invocation-read-only-status-dry-run-operator-evidence-acceptance-recording-persistence-shadow-write-rehearsal-readback-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_systems_tool_registry_shadow_registration_lookup_readback.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_SHADOW_REGISTRATION_LOOKUP_READBACK_2026-07-01.md"

fail() {
  printf 'hepta-systems-tool-registry-shadow-registration-lookup-readback-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable persistence shadow write rehearsal report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing ToolRegistry shadow registration lookup Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing ToolRegistry shadow registration lookup architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the ToolRegistry shadow registration lookup report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" || fail "failed to render persistence shadow write rehearsal report"
jq -e . "$tmpdir/source.json" >/dev/null || fail "persistence shadow write rehearsal report did not render valid JSON"

lib_export_present=false
if grep -q 'hepta_systems_tool_registry_shadow_registration_lookup_readback_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-tool-registry-shadow-registration-lookup-readback-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_TOOL_REGISTRY_SHADOW_REGISTRATION_LOOKUP_READBACK_2026-07-01.md" \
  '
  def suffix($kind):
    if $kind == "mcp_server" then "local-mcp:read-only-status-dry-run"
    elif $kind == "app_connector" then "local-app:not-selected"
    else "unknown:not-selected"
    end;
  def plan($kind): "tool-registry-shadow-registration-plan:hepta-system:" + suffix($kind);
  def registry_key($kind): "tool-registry-shadow-entry:hepta-system:" + suffix($kind);
  def payload_digest($kind): "sha256:tool-registry-shadow-registration-payload:hepta-system:" + suffix($kind);
  def lookup_query($kind): "tool-registry-shadow-lookup-query:hepta-system:" + suffix($kind);
  def lookup_result($kind): "tool-registry-shadow-lookup-result:hepta-system:" + suffix($kind) + ":not-executed";
  def duplicate_check($kind): "tool-registry-shadow-duplicate-check:hepta-system:" + suffix($kind) + ":unique";
  def replay_anchor($kind): "tool-registry-shadow-idempotency-replay-anchor:hepta-system:" + suffix($kind);
  def approval_ledger_anchor($kind): "tool-registry-shadow-approval-ledger-replay-anchor:hepta-system:" + suffix($kind) + ":not-written";
  def entry($source_entry):
    ($source_entry.contribution_kind) as $kind |
    {
      candidate_tool_id:$source_entry.candidate_tool_id,
      contribution_kind:$kind,
      dry_run_path_selected:$source_entry.dry_run_path_selected,
      source_shadow_acceptance_record_envelope_id:$source_entry.shadow_acceptance_record_envelope_id,
      source_shadow_write_intent_id:$source_entry.shadow_write_intent_id,
      source_shadow_write_payload_digest:$source_entry.shadow_write_payload_digest,
      source_shadow_idempotency_replay_key:$source_entry.shadow_idempotency_replay_key,
      source_shadow_receipt_preview_id:$source_entry.shadow_receipt_preview_id,
      source_shadow_store_target_id:$source_entry.shadow_store_target_id,
      source_shadow_replay_result_id:$source_entry.shadow_replay_result_id,
      shadow_registry_registration_plan_id:plan($kind),
      shadow_registry_entry_key:registry_key($kind),
      shadow_registration_payload_digest:payload_digest($kind),
      shadow_lookup_query_id:lookup_query($kind),
      shadow_lookup_result_id:lookup_result($kind),
      shadow_duplicate_check_id:duplicate_check($kind),
      shadow_idempotency_replay_anchor_id:replay_anchor($kind),
      shadow_approval_ledger_replay_anchor_id:approval_ledger_anchor($kind),
      first_shadow_registry_entry_key:registry_key($kind),
      second_shadow_registry_entry_key:registry_key($kind),
      first_shadow_registration_payload_digest:payload_digest($kind),
      second_shadow_registration_payload_digest:payload_digest($kind),
      first_shadow_lookup_query_id:lookup_query($kind),
      second_shadow_lookup_query_id:lookup_query($kind),
      first_shadow_idempotency_replay_anchor_id:replay_anchor($kind),
      second_shadow_idempotency_replay_anchor_id:replay_anchor($kind),
      source_shadow_acceptance_record_envelope_linked:true,
      source_shadow_write_intent_linked:true,
      source_shadow_payload_digest_linked:true,
      source_shadow_idempotency_replay_key_linked:true,
      source_shadow_receipt_preview_linked:true,
      source_shadow_store_target_linked:true,
      source_shadow_replay_result_linked:true,
      shadow_registry_registration_plan_projected:true,
      shadow_registry_entry_key_projected:true,
      shadow_registration_payload_digest_projected:true,
      shadow_lookup_query_projected:true,
      shadow_lookup_result_projected:true,
      shadow_duplicate_check_projected:true,
      shadow_idempotency_replay_anchor_projected:true,
      shadow_approval_ledger_replay_anchor_projected:true,
      stable_shadow_registry_entry_key:true,
      unique_shadow_registry_entry_key:true,
      stable_shadow_registration_payload_digest:true,
      unique_shadow_registration_payload_digest:true,
      stable_shadow_lookup_query:true,
      unique_shadow_lookup_query:true,
      stable_shadow_idempotency_replay_anchor:true,
      unique_shadow_idempotency_replay_anchor:true,
      feature_gate_opened:$source_entry.feature_gate_opened,
      shadow_write_executed:$source_entry.shadow_write_executed,
      shadow_write_materialized:$source_entry.shadow_write_materialized,
      shadow_store_written:$source_entry.shadow_store_written,
      test_tmp_written:$source_entry.test_tmp_written,
      shadow_registry_materialized:false,
      shadow_lookup_executed:false,
      tool_registered:$source_entry.tool_registered,
      tool_registry_mutated:false,
      registry_lookup_executed:$source_entry.registry_lookup_executed,
      tool_invoked:$source_entry.tool_invoked,
      approval_requested:false,
      ledger_written:$source_entry.ledger_written,
      receipt_persisted:$source_entry.receipt_persisted,
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
  ($entries | map(.first_shadow_registry_entry_key) | unique | length) as $unique_registry_key_count |
  ($entries | map(.first_shadow_registration_payload_digest) | unique | length) as $unique_payload_digest_count |
  ($entries | map(.first_shadow_lookup_query_id) | unique | length) as $unique_lookup_query_count |
  ($entries | map(.first_shadow_idempotency_replay_anchor_id) | unique | length) as $unique_replay_anchor_count |
  ($source_report.persistence_shadow_write_rehearsal_readback_ready == true
    and $source_report.candidate_count == 2
    and $source_report.shadow_write_entry_count == 2
    and $source_report.shadow_write_rehearsal_item_count == 14
    and $source_report.shadow_write_executed_count == 0
    and $source_report.shadow_store_written_count == 0
    and $source_report.registry_lookup_executed_count == 0
    and $entry_count == 2
    and $selected_count == 1
    and $non_selected_count == 1
    and ($entries | map(select(.source_shadow_acceptance_record_envelope_linked == true)) | length) == 2
    and ($entries | map(select(.source_shadow_write_intent_linked == true)) | length) == 2
    and ($entries | map(select(.source_shadow_payload_digest_linked == true)) | length) == 2
    and ($entries | map(select(.source_shadow_idempotency_replay_key_linked == true)) | length) == 2
    and ($entries | map(select(.source_shadow_receipt_preview_linked == true)) | length) == 2
    and ($entries | map(select(.source_shadow_store_target_linked == true)) | length) == 2
    and ($entries | map(select(.source_shadow_replay_result_linked == true)) | length) == 2
    and ($entries | map(select(.shadow_registry_registration_plan_projected == true)) | length) == 2
    and ($entries | map(select(.shadow_registry_entry_key_projected == true)) | length) == 2
    and ($entries | map(select(.shadow_registration_payload_digest_projected == true)) | length) == 2
    and ($entries | map(select(.shadow_lookup_query_projected == true)) | length) == 2
    and ($entries | map(select(.shadow_lookup_result_projected == true)) | length) == 2
    and ($entries | map(select(.shadow_duplicate_check_projected == true)) | length) == 2
    and ($entries | map(select(.shadow_idempotency_replay_anchor_projected == true)) | length) == 2
    and ($entries | map(select(.shadow_approval_ledger_replay_anchor_projected == true)) | length) == 2
    and ($entry_count * 8) == 16
    and ($entries | map(select(.stable_shadow_registry_entry_key == true)) | length) == 2
    and $unique_registry_key_count == 2
    and ($entries | map(select(.stable_shadow_registration_payload_digest == true)) | length) == 2
    and $unique_payload_digest_count == 2
    and ($entries | map(select(.stable_shadow_lookup_query == true)) | length) == 2
    and $unique_lookup_query_count == 2
    and ($entries | map(select(.stable_shadow_idempotency_replay_anchor == true)) | length) == 2
    and $unique_replay_anchor_count == 2
    and ($entries | map(select(.feature_gate_opened == true or .shadow_write_executed == true or .shadow_write_materialized == true or .shadow_store_written == true or .test_tmp_written == true or .shadow_registry_materialized == true or .shadow_lookup_executed == true or .tool_registered == true or .tool_registry_mutated == true or .registry_lookup_executed == true or .tool_invoked == true or .approval_requested == true or .ledger_written == true or .receipt_persisted == true or .mcp_server_started == true or .app_connector_started == true or .runtime_event_log_written == true or .sqlite_written == true or .live_execution_started == true)) | length) == 0) as $ready |
  {
    runtime:"hepta",
    surface:"hepta_systems_tool_registry_shadow_registration_lookup_readback",
    status:(if $ready then "ready_blocked" else "blocked" end),
    gate:"hepta_systems_tool_registry_shadow_registration_lookup_readback_gate",
    schema_version:"hepta_systems_tool_registry_shadow_registration_lookup_readback_v1",
    plugin_id:"hepta-system@hepta-local",
    manifest_name:$source_report.manifest_name,
    manifest_version:$source_report.manifest_version,
    source_persistence_shadow_write_rehearsal_readback_ready:$source_report.persistence_shadow_write_rehearsal_readback_ready,
    lib_export_present:$lib_export_present,
    candidate_count:$source_report.candidate_count,
    registry_shadow_entry_count:$entry_count,
    selected_read_only_status_tool_count:$selected_count,
    non_selected_preflight_boundary_count:$non_selected_count,
    source_shadow_acceptance_record_envelope_linked_count:($entries | map(select(.source_shadow_acceptance_record_envelope_linked == true)) | length),
    source_shadow_write_intent_linked_count:($entries | map(select(.source_shadow_write_intent_linked == true)) | length),
    source_shadow_payload_digest_linked_count:($entries | map(select(.source_shadow_payload_digest_linked == true)) | length),
    source_shadow_idempotency_replay_key_linked_count:($entries | map(select(.source_shadow_idempotency_replay_key_linked == true)) | length),
    source_shadow_receipt_preview_linked_count:($entries | map(select(.source_shadow_receipt_preview_linked == true)) | length),
    source_shadow_store_target_linked_count:($entries | map(select(.source_shadow_store_target_linked == true)) | length),
    source_shadow_replay_result_linked_count:($entries | map(select(.source_shadow_replay_result_linked == true)) | length),
    shadow_registry_registration_plan_projected_count:($entries | map(select(.shadow_registry_registration_plan_projected == true)) | length),
    shadow_registry_entry_key_projected_count:($entries | map(select(.shadow_registry_entry_key_projected == true)) | length),
    shadow_registration_payload_digest_projected_count:($entries | map(select(.shadow_registration_payload_digest_projected == true)) | length),
    shadow_lookup_query_projected_count:($entries | map(select(.shadow_lookup_query_projected == true)) | length),
    shadow_lookup_result_projected_count:($entries | map(select(.shadow_lookup_result_projected == true)) | length),
    shadow_duplicate_check_projected_count:($entries | map(select(.shadow_duplicate_check_projected == true)) | length),
    shadow_idempotency_replay_anchor_projected_count:($entries | map(select(.shadow_idempotency_replay_anchor_projected == true)) | length),
    shadow_approval_ledger_replay_anchor_projected_count:($entries | map(select(.shadow_approval_ledger_replay_anchor_projected == true)) | length),
    tool_registry_shadow_item_count:($entry_count * 8),
    stable_shadow_registry_entry_key_count:($entries | map(select(.stable_shadow_registry_entry_key == true)) | length),
    unique_shadow_registry_entry_key_count:$unique_registry_key_count,
    stable_shadow_registration_payload_digest_count:($entries | map(select(.stable_shadow_registration_payload_digest == true)) | length),
    unique_shadow_registration_payload_digest_count:$unique_payload_digest_count,
    stable_shadow_lookup_query_count:($entries | map(select(.stable_shadow_lookup_query == true)) | length),
    unique_shadow_lookup_query_count:$unique_lookup_query_count,
    stable_shadow_idempotency_replay_anchor_count:($entries | map(select(.stable_shadow_idempotency_replay_anchor == true)) | length),
    unique_shadow_idempotency_replay_anchor_count:$unique_replay_anchor_count,
    shadow_registry_entry_key_mismatch_count:($entries | map(select(.stable_shadow_registry_entry_key == false)) | length),
    duplicate_shadow_registry_entry_key_count:($entry_count - $unique_registry_key_count),
    shadow_registration_payload_digest_mismatch_count:($entries | map(select(.stable_shadow_registration_payload_digest == false)) | length),
    duplicate_shadow_registration_payload_digest_count:($entry_count - $unique_payload_digest_count),
    shadow_lookup_query_mismatch_count:($entries | map(select(.stable_shadow_lookup_query == false)) | length),
    duplicate_shadow_lookup_query_count:($entry_count - $unique_lookup_query_count),
    shadow_idempotency_replay_anchor_mismatch_count:($entries | map(select(.stable_shadow_idempotency_replay_anchor == false)) | length),
    duplicate_shadow_idempotency_replay_anchor_count:($entry_count - $unique_replay_anchor_count),
    feature_gate_opened_count:($entries | map(select(.feature_gate_opened == true)) | length),
    shadow_write_executed_count:($entries | map(select(.shadow_write_executed == true)) | length),
    shadow_write_materialized_count:($entries | map(select(.shadow_write_materialized == true)) | length),
    shadow_store_written_count:($entries | map(select(.shadow_store_written == true)) | length),
    test_tmp_written_count:($entries | map(select(.test_tmp_written == true)) | length),
    shadow_registry_materialized_count:($entries | map(select(.shadow_registry_materialized == true)) | length),
    shadow_lookup_executed_count:($entries | map(select(.shadow_lookup_executed == true)) | length),
    tool_registered_count:($entries | map(select(.tool_registered == true)) | length),
    tool_registry_mutated_count:($entries | map(select(.tool_registry_mutated == true)) | length),
    registry_lookup_executed_count:($entries | map(select(.registry_lookup_executed == true)) | length),
    tool_invoked_count:($entries | map(select(.tool_invoked == true)) | length),
    approval_requested_count:($entries | map(select(.approval_requested == true)) | length),
    ledger_written_count:($entries | map(select(.ledger_written == true)) | length),
    receipt_persisted_count:($entries | map(select(.receipt_persisted == true)) | length),
    mcp_server_started_count:($entries | map(select(.mcp_server_started == true)) | length),
    app_connector_started_count:($entries | map(select(.app_connector_started == true)) | length),
    runtime_event_log_written_count:($entries | map(select(.runtime_event_log_written == true)) | length),
    sqlite_written_count:($entries | map(select(.sqlite_written == true)) | length),
    live_execution_started_count:($entries | map(select(.live_execution_started == true)) | length),
    tool_registry_shadow_registration_lookup_readback_ready:$ready,
    feature_gate_open_allowed:false,
    shadow_write_execution_allowed:false,
    shadow_store_write_allowed:false,
    test_tmp_write_allowed:false,
    shadow_registry_materialization_allowed:false,
    shadow_lookup_execution_allowed:false,
    tool_registry_registration_allowed:false,
    tool_registry_mutation_allowed:false,
    registry_lookup_execution_allowed:false,
    tool_invocation_allowed:false,
    approval_request_allowed:false,
    ledger_persistence_allowed:false,
    receipt_persistence_allowed:false,
    connector_start_allowed:false,
    runtime_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    live_execution_allowed:false,
    entries:$entries,
    blockers:[
      "feature_gate_closed",
      "shadow_write_execution_disabled",
      "shadow_store_write_disabled",
      "test_tmp_write_disabled",
      "shadow_registry_materialization_disabled",
      "shadow_lookup_execution_disabled",
      "tool_registry_registration_disabled",
      "tool_registry_mutation_disabled",
      "registry_lookup_execution_disabled",
      "tool_invocation_disabled",
      "approval_request_disabled",
      "ledger_persistence_disabled",
      "receipt_persistence_disabled",
      "connector_start_disabled",
      "runtime_event_log_write_disabled",
      "sqlite_write_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "close_controlled_live_evidence_before_status_canary_start"
    ],
    recommended_next_gate:"close_controlled_live_evidence_before_status_canary_start",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      filesystem_written:false,
      feature_gate_opened:false,
      shadow_write_executed:false,
      shadow_write_materialized:false,
      shadow_store_written:false,
      test_tmp_written:false,
      shadow_registry_materialized:false,
      shadow_lookup_executed:false,
      tool_registered:false,
      tool_registry_mutated:false,
      registry_lookup_executed:false,
      tool_invoked:false,
      approval_requested:false,
      ledger_persisted:false,
      receipt_persisted:false,
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
