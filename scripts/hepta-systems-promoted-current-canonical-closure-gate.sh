#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-promoted-current-canonical-closure-report.sh"
PROMOTED_WRAPPER_GATE="$ROOT/scripts/hepta-systems-promoted-current-canonical-wrapper-gate.sh"
ALIAS_READBACK_GATE="$ROOT/scripts/hepta-systems-historical-canonical-gate-alias-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PROMOTED_CURRENT_CANONICAL_CLOSURE_2026-06-21.md"

fail() {
  printf 'hepta-systems-promoted-current-canonical-closure-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable promoted current canonical closure report: $REPORT"
[[ -x "$PROMOTED_WRAPPER_GATE" ]] || fail "missing executable promoted current canonical wrapper gate: $PROMOTED_WRAPPER_GATE"
[[ -x "$ALIAS_READBACK_GATE" ]] || fail "missing executable historical canonical gate alias readback gate: $ALIAS_READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing promoted current canonical closure architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the promoted current canonical closure report"
fi

grep -q 'Promoted Current Canonical Closure' "$DOC" \
  || fail "architecture note must document Promoted Current Canonical Closure"
grep -q 'successor closure' "$DOC" \
  || fail "architecture note must document successor closure behavior"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that the closure does not invoke the alias"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "promoted_current_canonical_closure"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_promoted_current_canonical_wrapper_surface == "promoted_current_canonical_wrapper"
  and .source_promoted_current_canonical_wrapper_ready == true
  and .source_alias_readback_surface == "historical_canonical_gate_alias_readback"
  and .source_alias_readback_ready == true
  and .source_alias_readback_mode == "static_shell_readback_only"
  and .promoted_current_canonical_closure_ready == true
  and .promoted_closure_kind == "non_circular_successor_closure"
  and .promoted_closure_input_count == 2
  and (.promoted_closure_inputs | all(.required == true and .source_ready == true and .invoked_by_report == false))
  and .promoted_wrapper_attached == true
  and .historical_canonical_gate_alias_readback_attached == true
  and .historical_canonical_gate_alias_readback_pending == false
  and .historical_canonical_gate_alias_path == "scripts/hepta-systems-canonical-gate.sh"
  and .historical_canonical_gate_alias_target == "scripts/hepta-systems-current-canonical-wrapper-gate.sh"
  and .historical_canonical_gate_alias_exec_count == 1
  and .historical_canonical_gate_alias_bash_syntax_valid == true
  and .legacy_current_canonical_closure_replaced_in_place == false
  and .legacy_current_canonical_wrapper_replaced_in_place == false
  and .historical_canonical_gate_mutated == false
  and .canonical_summary_mutated == false
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .closure_blocker_count == 9
  and (.closure_blockers | index("legacy_closure_not_replaced_in_place")) != null
  and (.closure_blockers | index("canonical_gate_not_invoked_by_promoted_closure")) != null
  and (.closure_blockers | index("wrapper_target_not_invoked_by_promoted_closure")) != null
  and .canonical_gate_wrapper_invoked == false
  and .wrapper_target_invoked == false
  and .capability_matrix_gate_invoked == false
  and .terminal_live_gate_invoked == false
  and .live_url_required == false
  and .long_soak_required == false
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "derive_promoted_current_canonical_closure_index_without_alias_invocation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$PROMOTED_WRAPPER_GATE" >/dev/null
"$ALIAS_READBACK_GATE" >/dev/null

printf 'hepta-systems-promoted-current-canonical-closure-gate: PASS: promoted current canonical closure successor is ready without alias or target invocation\n'
