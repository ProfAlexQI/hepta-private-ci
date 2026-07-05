#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-historical-canonical-gate-name-reintroduction-preflight-report.sh"
PHASE_GATE="$ROOT/scripts/hepta-systems-tool-execution-canonical-summary-attachment-phase-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_HISTORICAL_CANONICAL_GATE_NAME_REINTRODUCTION_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-historical-canonical-gate-name-reintroduction-preflight-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable historical canonical gate name preflight report: $REPORT"
[[ -x "$PHASE_GATE" ]] || fail "missing executable canonical summary attachment phase index gate: $PHASE_GATE"
[[ -f "$DOC" ]] || fail "missing historical canonical gate name preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the historical canonical gate name preflight report"
fi

grep -q 'Historical Canonical Gate Name Reintroduction Preflight' "$DOC" \
  || fail "architecture note must document Historical Canonical Gate Name Reintroduction Preflight"
grep -q 'strict-missing consumers' "$DOC" \
  || fail "architecture note must document strict-missing consumers"
grep -q 'thin local wrapper' "$DOC" \
  || fail "architecture note must document thin local wrapper"
grep -q 'snapshot evidence' "$DOC" \
  || fail "architecture note must document snapshot evidence"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "historical_canonical_gate_name_reintroduction_preflight"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_phase_index_surface == "tool_execution_canonical_summary_attachment_phase_index"
  and .source_phase_index_ready == true
  and .source_historical_missing_path_evidence_preserved == true
  and .source_historical_missing_path_evidence_basis == "historical_snapshot_evidence"
  and .source_historical_missing_path_current_filesystem_probe_used == false
  and .source_historical_snapshot_missing_canonical_summary == true
  and .source_historical_snapshot_evidence_consumable_after_wrapper_creation == true
  and .source_current_wrapper_phase_available == true
  and .source_current_wrapper_active_summary_source == true
  and .source_phase_split_present == true
  and .source_phase_split_required_before_name_claim == false
  and .source_phase_split_completed_before_name_claim == true
  and .proposed_historical_canonical_gate_path == "scripts/hepta-systems-canonical-gate.sh"
  and .proposed_alias_target == "scripts/hepta-systems-current-canonical-wrapper-gate.sh"
  and .proposed_alias_kind == "thin_local_wrapper"
  and .proposed_alias_would_invoke_live_gates == false
  and .dependent_consumer_count == 3
  and .dependent_strict_missing_consumer_count == 2
  and .dependent_blocking_consumer_count == 0
  and any(.dependent_consumers[]; .id == "canonical_summary_attachment_index_gate" and .migrated_to_phase_index == true and .would_break_if_historical_name_claimed == false)
  and any(.dependent_consumers[]; .id == "compact_capability_restore_preflight_gate" and .migrated_to_phase_index == true and .would_break_if_historical_name_claimed == false)
  and .dependent_gate_migration_required == false
  and .historical_canonical_gate_name_reintroduction_preflight_ready == true
  and .historical_canonical_gate_name_reintroduction_allowed == true
  and .historical_canonical_gate_name_claimed == true
  and .historical_canonical_gate_created == true
  and .historical_canonical_gate_executable == true
  and .historical_canonical_gate_wrapper_kind == "thin_local_exec_wrapper"
  and .historical_canonical_gate_wrapper_target == "scripts/hepta-systems-current-canonical-wrapper-gate.sh"
  and .historical_canonical_gate_wrapper_exec_count == 1
  and .historical_canonical_gate_mutated == true
  and .historical_canonical_gate_mutated_by_report == false
  and .canonical_gate_wrapper_invoked == false
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "validate_historical_canonical_gate_thin_wrapper_without_live_invocation"
  and (.preflight_blockers | index("historical_canonical_gate_thin_wrapper_validation_pending")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$PHASE_GATE" >/dev/null

printf 'hepta-systems-historical-canonical-gate-name-reintroduction-preflight-gate: PASS: canonical name reintroduction preflight is post-creation ready without live invocation\n'
