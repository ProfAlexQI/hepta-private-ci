#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-consumer-report.sh"
PROMOTED_SUMMARY_REPORT="$ROOT/scripts/hepta-systems-promoted-post-canonical-closure-compact-capability-summary-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CONSUMER_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-consumer-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable promoted current canonical consumer report: $REPORT"
[[ -x "$PROMOTED_SUMMARY_REPORT" ]] || fail "missing executable promoted post-canonical closure summary report: $PROMOTED_SUMMARY_REPORT"
[[ -f "$DOC" ]] || fail "missing promoted current canonical consumer architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the promoted current canonical consumer report"
fi

grep -q 'Promoted Current Canonical Consumer' "$DOC" \
  || fail "architecture note must document Promoted Current Canonical Consumer"
grep -q 'successor canonical consumer' "$DOC" \
  || fail "architecture note must document successor canonical consumer behavior"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the consumer does not invoke the alias"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "promoted_current_canonical_consumer"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_promoted_post_canonical_closure_summary_surface == "promoted_post_canonical_closure_compact_capability_summary"
  and .source_promoted_post_canonical_closure_summary_ready == true
  and .source_promoted_closure_index_attached == true
  and .source_promoted_current_canonical_closure_attached == true
  and .source_promoted_current_canonical_wrapper_attached == true
  and .source_current_canonical_consumer_attached == true
  and .promoted_current_canonical_consumer_ready == true
  and .promoted_current_canonical_consumer_surface == "promoted_post_canonical_closure_compact_capability_summary"
  and .previous_current_canonical_consumer_surface == "current_canonical_consumer"
  and .previous_current_canonical_consumer_replaced_in_place == false
  and .promoted_consumer_promotion_kind == "successor_report_only"
  and .successor_consumer_cutover_preflight_required == true
  and .canonical_consumer_input_count == 1
  and (.canonical_consumer_inputs | all(.source_ready == true and .active_successor_canonical_consumer == true and .invoked_by_report == false))
  and .local_surface_count == 7
  and .local_surface_ready_count == 7
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .stale_pre_creation_blockers_present == false
  and .promoted_current_summary_blocker_count == 10
  and (.promoted_current_summary_blockers | index("canonical_wrapper_not_restored_yet") == null)
  and (.promoted_current_summary_blockers | index("successor_consumer_cutover_preflight_pending")) != null
  and (.promoted_current_summary_blockers | index("current_canonical_consumer_not_replaced_in_place")) != null
  and .current_canonical_consumer_mutated == false
  and .promoted_current_canonical_consumer_mutated == false
  and .canonical_summary_mutated == false
  and .historical_canonical_gate_mutated == false
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
  and .next_migration_step == "evaluate_successor_canonical_consumer_cutover_preflight_without_live_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$PROMOTED_SUMMARY_REPORT" | jq -e '
  .surface == "promoted_post_canonical_closure_compact_capability_summary"
  and .promoted_post_canonical_closure_compact_capability_summary_ready == true
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
' >/dev/null

printf 'hepta-systems-promoted-current-canonical-consumer-gate: PASS: promoted current canonical consumer successor is ready without alias or target invocation\n'
