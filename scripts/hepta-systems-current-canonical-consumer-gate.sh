#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-current-canonical-consumer-report.sh"
POST_SUMMARY_GATE="$ROOT/scripts/hepta-systems-post-canonical-closure-compact-capability-summary-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CURRENT_CANONICAL_CONSUMER_2026-06-21.md"

fail() {
  printf 'hepta-systems-current-canonical-consumer-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable current canonical consumer report: $REPORT"
[[ -x "$POST_SUMMARY_GATE" ]] || fail "missing executable post-canonical closure compact capability summary gate: $POST_SUMMARY_GATE"
[[ -f "$DOC" ]] || fail "missing current canonical consumer architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the current canonical consumer report"
fi

grep -q 'Current Canonical Consumer' "$DOC" \
  || fail "architecture note must document Current Canonical Consumer"
grep -q 'post-canonical closure summary' "$DOC" \
  || fail "architecture note must document post-canonical closure summary promotion"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the consumer does not invoke the alias"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "current_canonical_consumer"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_post_canonical_closure_summary_surface == "post_canonical_closure_compact_capability_summary"
  and .source_post_canonical_closure_summary_ready == true
  and .source_alias_readback_index_surface == "current_canonical_closure_alias_readback_index"
  and .source_alias_readback_index_ready == true
  and .source_historical_canonical_gate_alias_readback_attached == true
  and .source_historical_canonical_gate_alias_readback_pending == false
  and .source_historical_canonical_gate_name_claimed == true
  and .current_canonical_consumer_ready == true
  and .current_canonical_consumer_surface == "post_canonical_closure_compact_capability_summary"
  and .previous_current_summary_surface == "current_compact_capability_summary"
  and .previous_current_summary_superseded_by_post_canonical_closure == true
  and .canonical_consumer_promotion_kind == "successor_report_only"
  and .canonical_consumer_input_count == 1
  and (.canonical_consumer_inputs | all(.source_ready == true and .active_current_canonical_consumer == true and .invoked_by_report == false))
  and .local_surface_count == 6
  and .local_surface_ready_count == 6
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .retired_pre_creation_blocker_count == 1
  and (.retired_pre_creation_blockers | index("canonical_wrapper_not_restored_yet")) != null
  and .stale_pre_creation_blockers_present == false
  and .current_summary_blocker_count == 8
  and (.current_summary_blockers | index("canonical_wrapper_not_restored_yet") == null)
  and (.current_summary_blockers | index("manual_operator_live_cutover_approval_required")) != null
  and (.current_summary_blockers | index("canonical_gate_not_invoked_by_current_canonical_consumer")) != null
  and (.current_summary_blockers | index("wrapper_target_not_invoked_by_current_canonical_consumer")) != null
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .capability_matrix_gate_invoked == false
  and .terminal_live_gate_invoked == false
  and .live_url_required == false
  and .long_soak_required == false
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .canonical_summary_mutation_allowed == false
  and .current_canonical_wrapper_mutation_allowed == false
  and .historical_canonical_gate_mutation_allowed == false
  and .upstream_gate_reexecution_required == false
  and .next_migration_step == "migrate_current_canonical_wrapper_to_promoted_consumer_without_alias_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$POST_SUMMARY_GATE" >/dev/null

printf 'hepta-systems-current-canonical-consumer-gate: PASS: post-canonical closure summary is promoted as current canonical consumer without live invocation\n'
