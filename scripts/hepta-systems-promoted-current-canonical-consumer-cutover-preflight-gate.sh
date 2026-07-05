#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-consumer-cutover-preflight-report.sh"
PROMOTED_CONSUMER_REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-consumer-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CONSUMER_CUTOVER_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-consumer-cutover-preflight-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable promoted current canonical consumer cutover preflight report: $REPORT"
[[ -x "$PROMOTED_CONSUMER_REPORT" ]] || fail "missing executable promoted current canonical consumer report: $PROMOTED_CONSUMER_REPORT"
[[ -f "$DOC" ]] || fail "missing promoted current canonical consumer cutover preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the promoted current canonical consumer cutover preflight report"
fi

grep -q 'Promoted Current Canonical Consumer Cutover Preflight' "$DOC" \
  || fail "architecture note must document Promoted Current Canonical Consumer Cutover Preflight"
grep -q 'dependency cycle' "$DOC" \
  || fail "architecture note must document dependency cycle"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the preflight does not invoke the alias"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "promoted_current_canonical_consumer_cutover_preflight"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_promoted_current_canonical_consumer_surface == "promoted_current_canonical_consumer"
  and .source_promoted_current_canonical_consumer_ready == true
  and .source_current_canonical_consumer_surface == "current_canonical_consumer"
  and .source_current_canonical_consumer_ready == true
  and .source_promoted_consumer_summary_surface == "promoted_post_canonical_closure_compact_capability_summary"
  and .cutover_preflight_ready == true
  and .direct_current_consumer_replacement_allowed == false
  and .direct_current_consumer_replacement_blocked == true
  and .dependency_cycle_detected == true
  and (.dependency_cycle_path | index("current_canonical_consumer")) != null
  and (.dependency_cycle_path | index("promoted_post_canonical_closure_compact_capability_summary")) != null
  and (.dependency_cycle_path | index("promoted_current_canonical_closure_index")) != null
  and .terminal_successor_consumer_cutover_packet_required == true
  and .terminal_successor_consumer_cutover_packet_allowed == true
  and .current_canonical_consumer_replaced_in_place == false
  and .current_canonical_consumer_mutated == false
  and .promoted_current_canonical_consumer_mutated == false
  and .canonical_summary_mutated == false
  and .historical_canonical_gate_mutated == false
  and .cutover_option_count == 2
  and (.cutover_options | any(.id == "replace_current_canonical_consumer_with_promoted_successor" and .would_create_dependency_cycle == true and .allowed == false))
  and (.cutover_options | any(.id == "create_terminal_successor_consumer_cutover_packet" and .would_create_dependency_cycle == false and .allowed == true))
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .cutover_blocker_count == 10
  and (.cutover_blockers | index("direct_current_consumer_replacement_blocked_by_dependency_cycle")) != null
  and (.cutover_blockers | index("terminal_successor_consumer_cutover_packet_required")) != null
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
  and .next_migration_step == "create_terminal_successor_canonical_consumer_cutover_packet_without_live_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$PROMOTED_CONSUMER_REPORT" | jq -e '
  .surface == "promoted_current_canonical_consumer"
  and .promoted_current_canonical_consumer_ready == true
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
' >/dev/null

printf 'hepta-systems-promoted-current-canonical-consumer-cutover-preflight-gate: PASS: direct successor consumer cutover is blocked and packet path is ready without alias or target invocation\n'
