#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-consumer-cutover-packet-report.sh"
CUTOVER_PREFLIGHT_REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-consumer-cutover-preflight-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CONSUMER_CUTOVER_PACKET_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-consumer-cutover-packet-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable promoted current canonical consumer cutover packet report: $REPORT"
[[ -x "$CUTOVER_PREFLIGHT_REPORT" ]] || fail "missing executable promoted current canonical consumer cutover preflight report: $CUTOVER_PREFLIGHT_REPORT"
[[ -f "$DOC" ]] || fail "missing promoted current canonical consumer cutover packet architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the promoted current canonical consumer cutover packet report"
fi

grep -q 'Promoted Current Canonical Consumer Cutover Packet' "$DOC" \
  || fail "architecture note must document Promoted Current Canonical Consumer Cutover Packet"
grep -q 'report-only' "$DOC" \
  || fail "architecture note must document report-only behavior"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the packet does not invoke the alias"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "promoted_current_canonical_consumer_cutover_packet"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_cutover_preflight_surface == "promoted_current_canonical_consumer_cutover_preflight"
  and .source_cutover_preflight_ready == true
  and .source_cutover_preflight_basis == "verified_preflight_report_snapshot"
  and .source_cutover_preflight_report_reexecuted == false
  and .source_promoted_current_canonical_consumer_surface == "promoted_current_canonical_consumer"
  and .source_current_canonical_consumer_surface == "current_canonical_consumer"
  and .terminal_successor_canonical_consumer_cutover_packet_ready == true
  and .terminal_successor_consumer_cutover_packet_kind == "report_only_non_authorizing_packet"
  and .terminal_successor_consumer_cutover_packet_required == true
  and .terminal_successor_consumer_cutover_packet_allowed == true
  and .packet_field_count == 10
  and .packet_required_field_count == 10
  and .packet_present_required_field_count == 7
  and .packet_missing_required_field_count == 3
  and (.packet_fields | any(.id == "manual_operator_live_cutover_approval" and .present == false))
  and (.packet_fields | any(.id == "live_invocation_plan" and .present == false))
  and (.packet_fields | any(.id == "public_ga_plan" and .present == false))
  and .packet_blocker_count == 10
  and (.packet_blockers | index("manual_operator_live_cutover_approval_missing")) != null
  and (.packet_blockers | index("packet_recording_disabled")) != null
  and (.packet_blockers | index("packet_acceptance_disabled")) != null
  and .direct_current_consumer_replacement_allowed == false
  and .direct_current_consumer_replacement_blocked == true
  and .dependency_cycle_detected == true
  and .current_canonical_consumer_replaced_in_place == false
  and .current_canonical_consumer_mutated == false
  and .promoted_current_canonical_consumer_mutated == false
  and .cutover_packet_recorded == false
  and .cutover_packet_accepted == false
  and .operator_live_cutover_approval_recorded == false
  and .successor_consumer_cutover_allowed == false
  and .rollback_anchor == "current_canonical_consumer"
  and .readback_mode == "static_report_readback_only"
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .capability_matrix_gate_invoked == false
  and .terminal_live_gate_invoked == false
  and .live_url_required == false
  and .long_soak_required == false
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .upstream_gate_reexecution_required == false
  and .next_migration_step == "derive_terminal_successor_canonical_consumer_cutover_packet_readback_without_live_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

printf 'hepta-systems-promoted-current-canonical-consumer-cutover-packet-gate: PASS: terminal successor consumer cutover packet is report-only and non-authorizing\n'
