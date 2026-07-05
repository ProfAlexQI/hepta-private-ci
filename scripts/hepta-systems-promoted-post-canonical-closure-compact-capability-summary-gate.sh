#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-promoted-post-canonical-closure-compact-capability-summary-report.sh"
CLOSURE_INDEX_REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-closure-index-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_POST_CANONICAL_CLOSURE_COMPACT_CAPABILITY_SUMMARY_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-post-canonical-closure-compact-capability-summary-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable promoted post-canonical closure compact capability summary report: $REPORT"
[[ -x "$CLOSURE_INDEX_REPORT" ]] || fail "missing executable promoted current canonical closure index report: $CLOSURE_INDEX_REPORT"
[[ -f "$DOC" ]] || fail "missing promoted post-canonical closure compact capability summary architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the promoted post-canonical closure compact capability summary report"
fi

grep -q 'Promoted Post-Canonical Closure Compact Capability Summary' "$DOC" \
  || fail "architecture note must document Promoted Post-Canonical Closure Compact Capability Summary"
grep -q 'promoted closure index' "$DOC" \
  || fail "architecture note must document promoted closure index source"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the summary does not invoke the alias"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "promoted_post_canonical_closure_compact_capability_summary"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_promoted_current_canonical_closure_index_surface == "promoted_current_canonical_closure_index"
  and .source_promoted_current_canonical_closure_index_ready == true
  and .source_current_canonical_consumer_attached == true
  and .source_historical_canonical_gate_alias_readback_attached == true
  and .promoted_post_canonical_closure_compact_capability_summary_ready == true
  and .local_surface_count == 7
  and .local_surface_ready_count == 7
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and (.capability_surfaces | length) == 7
  and (.capability_surfaces | all(.local_ready == true and .live_enabled == false and .public_ga_enabled == false))
  and .retired_pre_creation_blocker_count == 1
  and (.retired_pre_creation_blockers | index("canonical_wrapper_not_restored_yet")) != null
  and .stale_pre_creation_blockers_present == false
  and .summary_blocker_count == 10
  and (.summary_blockers | index("canonical_wrapper_not_restored_yet") == null)
  and (.summary_blockers | index("successor_consumer_promotion_pending")) != null
  and (.summary_blockers | index("canonical_gate_not_invoked_by_promoted_post_canonical_summary")) != null
  and (.summary_blockers | index("wrapper_target_not_invoked_by_promoted_post_canonical_summary")) != null
  and .promoted_closure_index_attached == true
  and .promoted_current_canonical_closure_attached == true
  and .promoted_current_canonical_wrapper_attached == true
  and .current_canonical_consumer_attached == true
  and .legacy_current_canonical_closure_index_replaced_in_place == false
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
  and .next_migration_step == "promote_promoted_post_canonical_summary_as_successor_canonical_consumer_without_live_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$CLOSURE_INDEX_REPORT" | jq -e '
  .surface == "promoted_current_canonical_closure_index"
  and .promoted_current_canonical_closure_index_ready == true
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
' >/dev/null

printf 'hepta-systems-promoted-post-canonical-closure-compact-capability-summary-gate: PASS: promoted post-canonical closure summary is ready without alias or target invocation\n'
