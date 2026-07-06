#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-wrapper-report.sh"
MIGRATION_PREFLIGHT_GATE="$ROOT/scripts/hepta-systems-current-canonical-wrapper-promoted-consumer-migration-preflight-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_WRAPPER_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-wrapper-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable promoted current canonical wrapper report: $REPORT"
[[ -x "$MIGRATION_PREFLIGHT_GATE" ]] || fail "missing executable promoted consumer migration preflight gate: $MIGRATION_PREFLIGHT_GATE"
[[ -f "$DOC" ]] || fail "missing promoted current canonical wrapper architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the promoted current canonical wrapper report"
fi

grep -q 'Promoted Current Canonical Wrapper' "$DOC" \
  || fail "architecture note must document Promoted Current Canonical Wrapper"
grep -q 'non-circular successor' "$DOC" \
  || fail "architecture note must document non-circular successor behavior"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the successor does not invoke the alias"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "promoted_current_canonical_wrapper"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_current_canonical_consumer_surface == "current_canonical_consumer"
  and .source_current_canonical_consumer_ready == true
  and .source_promoted_consumer_surface == "post_canonical_closure_compact_capability_summary"
  and .source_migration_preflight_surface == "current_canonical_wrapper_promoted_consumer_migration_preflight"
  and .source_migration_preflight_ready == true
  and .promoted_current_canonical_wrapper_ready == true
  and .promoted_wrapper_kind == "non_circular_successor_report"
  and .promoted_wrapper_source_surface == "current_canonical_consumer"
  and .promoted_wrapper_consumes_post_canonical_summary == true
  and .promoted_wrapper_input_count == 2
  and (.promoted_wrapper_inputs | all(.required == true and .source_ready == true and .invoked_by_report == false))
  and .legacy_current_canonical_wrapper_replaced_in_place == false
  and .legacy_current_canonical_wrapper_mutated == false
  and .historical_canonical_gate_mutated == false
  and .canonical_summary_mutated == false
  and .direct_current_wrapper_source_replacement_allowed == false
  and .dependency_cycle_detected_in_direct_replacement == true
  and .successor_wrapper_surface_required == true
  and .local_surface_count == 6
  and .local_surface_ready_count == 6
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .wrapper_blocker_count == 9
  and (.wrapper_blockers | index("legacy_wrapper_not_replaced_in_place")) != null
  and (.wrapper_blockers | index("canonical_gate_not_invoked_by_promoted_wrapper")) != null
  and (.wrapper_blockers | index("wrapper_target_not_invoked_by_promoted_wrapper")) != null
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .capability_matrix_gate_invoked == false
  and .terminal_live_gate_invoked == false
  and .live_url_required == false
  and .long_soak_required == false
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "attach_promoted_current_canonical_wrapper_to_closure_successor_without_alias_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$MIGRATION_PREFLIGHT_GATE" >/dev/null

printf 'hepta-systems-promoted-current-canonical-wrapper-gate: PASS: promoted current canonical wrapper successor is ready without alias or target invocation\n'
