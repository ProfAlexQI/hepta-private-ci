#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-hepta-system-status-internal-read-only-invocation-report.sh"
RUST_SOURCE="$ROOT/codex-rs/hepta-runtime/src/hepta_system_status_operator_approval_protocol.rs"
LIB_SOURCE="$ROOT/codex-rs/hepta-runtime/src/lib.rs"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_2026-06-27.md"

fail() {
  printf 'hepta-systems-hepta-system-status-operator-approval-protocol-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$SOURCE_REPORT" ]] || fail "missing executable Phase 8 internal invocation report: $SOURCE_REPORT"
[[ -f "$RUST_SOURCE" ]] || fail "missing Phase 9 Rust source: $RUST_SOURCE"
[[ -f "$LIB_SOURCE" ]] || fail "missing hepta-runtime lib source: $LIB_SOURCE"
[[ -f "$DOC" ]] || fail "missing Phase 9 architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the Phase 9 operator approval protocol report"
fi

tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

"$SOURCE_REPORT" >"$tmpdir/source.json" \
  || fail "failed to render Phase 8 internal invocation report"
jq -e . "$tmpdir/source.json" >/dev/null \
  || fail "invalid JSON rendered by Phase 8 internal invocation report"

lib_export_present=false
if grep -q 'hepta_system_status_operator_approval_protocol_report' "$LIB_SOURCE"; then
  lib_export_present=true
fi

jq -n \
  --slurpfile source "$tmpdir/source.json" \
  --argjson lib_export_present "$lib_export_present" \
  --arg gate "scripts/hepta-systems-hepta-system-status-operator-approval-protocol-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_HEPTA_SYSTEM_STATUS_OPERATOR_APPROVAL_PROTOCOL_2026-06-27.md" \
  '
  def selected_candidate_id: "preview:mcp:hepta-system@hepta-local:hepta_system_local_mcp";
  def approval_subject: "approval-subject:hepta-system/status/internal-read-only";
  def approval_nonce: "approval-nonce.hepta-system-status.internal-read-only.v1";
  def operator_session_binding: "operator-session-binding.hepta-local.explicit-accept-required.v1";
  def protocol_step($id; $route; $ready): {
    step_id:$id,
    route:$route,
    ready:$ready,
    required:true,
    source_payload_bound:$ready,
    nonce_bound:true,
    operator_session_bound:true,
    explicit_accept_required:true,
    approval_request_allowed:false,
    approval_acceptance_allowed:false,
    approval_recording_allowed:false,
    approval_broker_write_allowed:false,
    evidence_recording_allowed:false,
    credential_read_allowed:false,
    transport_mutation_allowed:false,
    persistence_allowed:false,
    live_execution_allowed:false
  };
  ($source[0]) as $source |
  {
    packet_id:"approval-packet.hepta-system-status.internal-read-only.v1",
    approval_subject:approval_subject,
    subject_route:"approval://hepta-system/status/internal-read-only/v1",
    source_payload_fingerprint:$source.status_payload_fingerprint,
    selected_candidate_tool_id:selected_candidate_id,
    nonce:approval_nonce,
    nonce_binding_key:"nonce-binding.hepta-system-status.internal-read-only.v1",
    operator_session_binding_key:operator_session_binding,
    approval_mode:"explicit_operator_accept_required_no_auto_accept",
    receipt_projection_route:"receipt://hepta-system/status/internal-read-only/operator-approval/non-acceptance",
    packet_preview_materialized:$source.internal_read_only_invocation_ready,
    explicit_accept_required:true,
    auto_accept_allowed:false,
    approval_request_sent:false,
    approval_accepted:false,
    approval_recorded:false,
    approval_broker_write_allowed:false,
    packet_persisted:false,
    live_execution_allowed:false
  } as $packet |
  [
    protocol_step("hepta-system.status.operator-approval.nonce-session-binding.v1"; "nonce_session_binding_preflight"; $source.internal_read_only_invocation_ready),
    protocol_step("hepta-system.status.operator-approval.packet-preview.v1"; "approval_packet_preview"; $source.status_payload_materialized),
    protocol_step("hepta-system.status.operator-approval.non-acceptance-receipt.v1"; "non_acceptance_receipt_projection"; ($source.receipt_projected_in_memory == true and $source.receipt_persisted == false))
  ] as $steps |
  ($steps | map(select(.ready == true)) | length) as $ready_step_count |
  ($source.internal_read_only_invocation_ready == true
    and $source.status_payload_materialized == true
    and $source.receipt_projected_in_memory == true
    and $source.receipt_persisted == false
    and $source.external_network_allowed == false
    and $source.credential_read_allowed == false
    and $source.external_tool_invoked == false
    and $source.tool_invocation_switch_enabled == false
    and $source.ledger_write_allowed == false
    and $source.approval_request_allowed == false
    and $source.approval_acceptance_allowed == false
    and $source.workflow_event_log_write_allowed == false
    and $source.sqlite_write_allowed == false
    and $source.native_post_mutation_allowed == false
    and $source.channel_send_allowed == false
    and $source.live_execution_allowed == false
    and $lib_export_present == true
    and $packet.packet_preview_materialized == true
    and $packet.explicit_accept_required == true
    and $packet.auto_accept_allowed == false
    and $packet.approval_request_sent == false
    and $packet.approval_accepted == false
    and $packet.approval_recorded == false
    and $packet.approval_broker_write_allowed == false
    and $packet.packet_persisted == false
    and $packet.live_execution_allowed == false
    and ($steps | length) == 3
    and $ready_step_count == 3
    and ($steps | all(.required == true
      and .source_payload_bound == true
      and .nonce_bound == true
      and .operator_session_bound == true
      and .explicit_accept_required == true
      and .approval_request_allowed == false
      and .approval_acceptance_allowed == false
      and .approval_recording_allowed == false
      and .approval_broker_write_allowed == false
      and .evidence_recording_allowed == false
      and .credential_read_allowed == false
      and .transport_mutation_allowed == false
      and .persistence_allowed == false
      and .live_execution_allowed == false))) as $approval_protocol_ready |
  {
    runtime:"hepta",
    surface:"hepta_system_status_operator_approval_protocol",
    status:(if $approval_protocol_ready then "ready_blocked" else "blocked" end),
    gate:"hepta_system_status_operator_approval_protocol_gate",
    schema_version:"hepta_system_status_operator_approval_protocol_v1",
    plugin_id:$source.plugin_id,
    source_invocation_gate:$source.gate,
    source_invocation_ready:$source.internal_read_only_invocation_ready,
    source_status_payload_materialized:$source.status_payload_materialized,
    source_receipt_projected_in_memory:$source.receipt_projected_in_memory,
    source_receipt_persisted:$source.receipt_persisted,
    selected_candidate_tool_id:selected_candidate_id,
    approval_subject:approval_subject,
    approval_packet_count:1,
    protocol_step_count:($steps | length),
    nonce_binding_present:true,
    session_binding_present:true,
    approval_packet_preview_ready:$packet.packet_preview_materialized,
    explicit_accept_required:$packet.explicit_accept_required,
    non_acceptance_receipt_projected:$approval_protocol_ready,
    approval_protocol_ready:$approval_protocol_ready,
    approval_request_sent:false,
    approval_request_allowed:false,
    approval_accepted:false,
    approval_acceptance_allowed:false,
    approval_recorded:false,
    approval_recording_allowed:false,
    auto_approval_enabled:false,
    evidence_recording_allowed:false,
    approval_broker_write_allowed:false,
    approval_broker_persisted:false,
    receipt_persisted:false,
    credential_read_allowed:false,
    external_network_allowed:false,
    external_tool_invoked:false,
    tool_invocation_switch_enabled:false,
    ledger_write_allowed:false,
    workflow_event_log_write_allowed:false,
    sqlite_write_allowed:false,
    transport_mutation_allowed:false,
    native_post_mutation_allowed:false,
    channel_send_allowed:false,
    live_execution_allowed:false,
    packet:$packet,
    steps:$steps,
    blockers:[
      "approval_request_not_sent",
      "approval_acceptance_requires_explicit_operator_action",
      "auto_approval_disabled",
      "approval_recording_disabled",
      "approval_broker_write_disabled",
      "evidence_recording_disabled",
      "credential_read_disabled",
      "external_network_disabled",
      "ledger_write_disabled",
      "transport_mutation_disabled",
      "receipt_persistence_disabled",
      "workflow_event_log_write_disabled",
      "sqlite_write_disabled",
      "native_post_mutation_disabled",
      "channel_send_disabled",
      "live_execution_disabled"
    ],
    next_actions:[
      "phase10_controlled_canary_readiness_plan_without_gateway_native_telegram_or_live_activation"
    ],
    next_migration_step:"phase10_controlled_canary_readiness_plan_without_gateway_native_telegram_or_live_activation",
    local_gate:$gate,
    architecture_note:$doc,
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      filesystem_written:false,
      approval_requested:false,
      approval_accepted:false,
      approval_recorded:false,
      approval_broker_written:false,
      evidence_recorded:false,
      evidence_persisted:false,
      credential_read:false,
      external_network_used:false,
      external_tool_invoked:false,
      tool_registry_switch_enabled:false,
      ledger_written:false,
      receipt_persisted:false,
      workflow_event_log_written:false,
      sqlite_written:false,
      transport_mutated:false,
      native_post_mutation_performed:false,
      channel_send_performed:false,
      provider_invoked:false,
      model_invoked:false,
      package_or_release_written:false,
      public_ga_promoted:false,
      live_execution_started:false
    }
  }'
