#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-closure-index-report.sh"
PROMOTED_CLOSURE_GATE="$ROOT/scripts/hepta-systems-promoted-current-canonical-closure-gate.sh"
CURRENT_CONSUMER_GATE="$ROOT/scripts/hepta-systems-current-canonical-consumer-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CLOSURE_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-closure-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable promoted current canonical closure index report: $REPORT"
[[ -x "$PROMOTED_CLOSURE_GATE" ]] || fail "missing executable promoted current canonical closure gate: $PROMOTED_CLOSURE_GATE"
[[ -x "$CURRENT_CONSUMER_GATE" ]] || fail "missing executable current canonical consumer gate: $CURRENT_CONSUMER_GATE"
[[ -f "$DOC" ]] || fail "missing promoted current canonical closure index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the promoted current canonical closure index report"
fi

grep -q 'Promoted Current Canonical Closure Index' "$DOC" \
  || fail "architecture note must document Promoted Current Canonical Closure Index"
grep -q 'successor index' "$DOC" \
  || fail "architecture note must document successor index behavior"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the index does not invoke the alias"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "promoted_current_canonical_closure_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_promoted_current_canonical_closure_surface == "promoted_current_canonical_closure"
  and .source_promoted_current_canonical_closure_ready == true
  and .source_current_canonical_consumer_surface == "current_canonical_consumer"
  and .source_current_canonical_consumer_ready == true
  and .source_current_canonical_consumer_promoted_surface == "post_canonical_closure_compact_capability_summary"
  and .promoted_current_canonical_closure_index_ready == true
  and .promoted_closure_index_kind == "non_circular_successor_index"
  and .promoted_closure_index_input_count == 2
  and (.promoted_closure_index_inputs | all(.required == true and .source_ready == true and .invoked_by_report == false))
  and .promoted_closure_attached == true
  and .promoted_wrapper_attached == true
  and .current_canonical_consumer_attached == true
  and .historical_canonical_gate_alias_readback_attached == true
  and .historical_canonical_gate_alias_readback_pending == false
  and .legacy_current_canonical_closure_index_replaced_in_place == false
  and .legacy_current_canonical_closure_replaced_in_place == false
  and .legacy_current_canonical_wrapper_replaced_in_place == false
  and .historical_canonical_gate_mutated == false
  and .canonical_summary_mutated == false
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .index_blocker_count == 9
  and (.index_blockers | index("legacy_closure_index_not_replaced_in_place")) != null
  and (.index_blockers | index("canonical_gate_not_invoked_by_promoted_closure_index")) != null
  and (.index_blockers | index("wrapper_target_not_invoked_by_promoted_closure_index")) != null
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .capability_matrix_gate_invoked == false
  and .terminal_live_gate_invoked == false
  and .live_url_required == false
  and .long_soak_required == false
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "derive_promoted_post_canonical_closure_compact_capability_summary_without_alias_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$PROMOTED_CLOSURE_GATE" >/dev/null
"$CURRENT_CONSUMER_GATE" >/dev/null

printf 'hepta-systems-promoted-current-canonical-closure-index-gate: PASS: promoted current canonical closure index is ready without alias or target invocation\n'
