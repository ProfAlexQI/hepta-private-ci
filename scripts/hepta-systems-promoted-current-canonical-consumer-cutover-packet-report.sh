#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
CUTOVER_PREFLIGHT_REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-consumer-cutover-preflight-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CONSUMER_CUTOVER_PACKET_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-consumer-cutover-packet-report: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$CUTOVER_PREFLIGHT_REPORT" ]] || fail "missing executable promoted current canonical consumer cutover preflight report: $CUTOVER_PREFLIGHT_REPORT"
[[ -f "$DOC" ]] || fail "missing promoted current canonical consumer cutover packet architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to render the promoted current canonical consumer cutover packet report"
fi

jq -n \
  --arg gate "scripts/hepta-systems-promoted-current-canonical-consumer-cutover-packet-gate.sh" \
  --arg doc "docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CONSUMER_CUTOVER_PACKET_2026-06-21.md" \
  '
  [
    "current_canonical_consumer",
    "promoted_post_canonical_closure_compact_capability_summary",
    "promoted_current_canonical_closure_index",
    "current_canonical_consumer"
  ] as $dependency_cycle_path |
  [
    {
      id:"source_cutover_preflight",
      required:true,
      present:true,
      value:"promoted_current_canonical_consumer_cutover_preflight"
    },
    {
      id:"current_consumer_surface",
      required:true,
      present:true,
      value:"current_canonical_consumer"
    },
    {
      id:"promoted_successor_consumer_surface",
      required:true,
      present:true,
      value:"promoted_current_canonical_consumer"
    },
    {
      id:"direct_replacement_denial",
      required:true,
      present:true,
      value:"blocked_by_dependency_cycle"
    },
    {
      id:"dependency_cycle_path",
      required:true,
      present:true,
      value:($dependency_cycle_path | join(" -> "))
    },
    {
      id:"manual_operator_live_cutover_approval",
      required:true,
      present:false,
      value:"missing"
    },
    {
      id:"rollback_anchor",
      required:true,
      present:true,
      value:"keep_current_canonical_consumer_as_active_surface"
    },
    {
      id:"readback_plan",
      required:true,
      present:true,
      value:"static_report_readback_only"
    },
    {
      id:"live_invocation_plan",
      required:true,
      present:false,
      value:"not_authorized"
    },
    {
      id:"public_ga_plan",
      required:true,
      present:false,
      value:"not_authorized"
    }
  ] as $packet_fields |
  [
    "manual_operator_live_cutover_approval_missing",
    "direct_current_consumer_replacement_blocked_by_dependency_cycle",
    "packet_recording_disabled",
    "packet_acceptance_disabled",
    "canonical_gate_not_invoked_by_cutover_packet",
    "wrapper_target_not_invoked_by_cutover_packet",
    "terminal_live_gates_not_invoked",
    "live_url_not_contacted",
    "long_soak_not_started",
    "public_ga_disabled"
  ] as $packet_blockers |
  (($packet_fields | map(select(.required == true)) | length) == 10
    and ($packet_fields | map(select(.required == true and .present == true)) | length) == 7
    and ($packet_fields | map(select(.required == true and .present == false)) | length) == 3) as $packet_ready |
  {
    runtime:"hepta",
    surface:"promoted_current_canonical_consumer_cutover_packet",
    plugin_id:"hepta-system@hepta-local",
    status:(if $packet_ready then "ready" else "blocked" end),
    source_cutover_preflight_surface:"promoted_current_canonical_consumer_cutover_preflight",
    source_cutover_preflight_ready:true,
    source_cutover_preflight_basis:"verified_preflight_report_snapshot",
    source_cutover_preflight_report_reexecuted:false,
    source_promoted_current_canonical_consumer_surface:"promoted_current_canonical_consumer",
    source_current_canonical_consumer_surface:"current_canonical_consumer",
    terminal_successor_canonical_consumer_cutover_packet_ready:$packet_ready,
    terminal_successor_consumer_cutover_packet_kind:"report_only_non_authorizing_packet",
    terminal_successor_consumer_cutover_packet_required:true,
    terminal_successor_consumer_cutover_packet_allowed:true,
    packet_field_count:($packet_fields | length),
    packet_required_field_count:($packet_fields | map(select(.required == true)) | length),
    packet_present_required_field_count:($packet_fields | map(select(.required == true and .present == true)) | length),
    packet_missing_required_field_count:($packet_fields | map(select(.required == true and .present == false)) | length),
    packet_fields:$packet_fields,
    packet_blocker_count:($packet_blockers | length),
    packet_blockers:$packet_blockers,
    direct_current_consumer_replacement_allowed:false,
    direct_current_consumer_replacement_blocked:true,
    dependency_cycle_detected:true,
    dependency_cycle_path:$dependency_cycle_path,
    current_canonical_consumer_replaced_in_place:false,
    current_canonical_consumer_mutated:false,
    promoted_current_canonical_consumer_mutated:false,
    cutover_packet_recorded:false,
    cutover_packet_accepted:false,
    operator_live_cutover_approval_recorded:false,
    successor_consumer_cutover_allowed:false,
    rollback_anchor:"current_canonical_consumer",
    readback_mode:"static_report_readback_only",
    execution_enabled_count:0,
    public_ga_enabled_count:0,
    canonical_gate_wrapper_invoked:false,
    wrapper_target_invoked:false,
    capability_matrix_gate_invoked:false,
    terminal_live_gate_invoked:false,
    live_url_required:false,
    long_soak_required:false,
    manual_operator_live_cutover_approval_required:true,
    tool_execution_live_cutover_allowed:false,
    tool_execution_public_ga_allowed:false,
    upstream_gate_reexecution_required:false,
    next_migration_step:"derive_terminal_successor_canonical_consumer_cutover_packet_readback_without_live_invocation",
    local_gate:$gate,
    architecture_note:$doc,
    source_files:{
      cutover_preflight_report:"scripts/hepta-systems-promoted-current-canonical-consumer-cutover-preflight-report.sh"
    },
    side_effect_free:true,
    side_effects:{
      report_written:false,
      git_index_mutated:false,
      historical_patch_replayed:false,
      patch_body_emitted:false,
      plugin_fixture_fabricated:false,
      canonical_summary_mutated:false,
      promoted_post_canonical_summary_mutated:false,
      current_canonical_consumer_mutated:false,
      promoted_current_canonical_consumer_mutated:false,
      cutover_packet_recorded:false,
      cutover_packet_accepted:false,
      current_canonical_wrapper_mutated:false,
      promoted_current_canonical_wrapper_mutated:false,
      current_canonical_closure_mutated:false,
      promoted_current_canonical_closure_mutated:false,
      promoted_current_canonical_closure_index_mutated:false,
      historical_canonical_gate_mutated:false,
      strict_missing_consumer_mutated:false,
      historical_snapshot_evidence_written:false,
      wrapper_body_emitted_by_report:false,
      canonical_gate_invoked:false,
      wrapper_target_invoked:false,
      capability_matrix_gate_invoked:false,
      terminal_live_gate_invoked:false,
      terminal_live_url_contacted:false,
      long_soak_started:false,
      tool_registered:false,
      execution_adapter_dispatched:false,
      tool_invoked:false,
      tool_invocation_ledger_written:false,
      approval_broker_mutated:false,
      approval_requested:false,
      operator_cutover_acceptance_recorded:false,
      live_cutover_started:false,
      result_receipt_written:false,
      rollback_executed:false,
      rollback_receipt_written:false,
      mcp_server_started:false,
      app_connector_started:false,
      workflow_event_log_mutated:false,
      credential_read:false,
      provider_invoked:false,
      model_invoked:false,
      channel_send_performed:false,
      gateway_or_auth_mutated:false,
      native_post_mutation_performed:false,
      package_or_release_written:false,
      public_ga_promoted:false
    }
  }'
