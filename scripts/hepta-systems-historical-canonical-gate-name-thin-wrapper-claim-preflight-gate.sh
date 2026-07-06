#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-historical-canonical-gate-name-thin-wrapper-claim-preflight-report.sh"
MIGRATION_GATE="$ROOT/scripts/hepta-systems-strict-missing-consumer-phase-migration-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_HISTORICAL_CANONICAL_GATE_NAME_THIN_WRAPPER_CLAIM_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-historical-canonical-gate-name-thin-wrapper-claim-preflight-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable historical canonical gate name thin wrapper claim preflight report: $REPORT"
[[ -x "$MIGRATION_GATE" ]] || fail "missing executable strict-missing consumer phase migration gate: $MIGRATION_GATE"
[[ -f "$DOC" ]] || fail "missing historical canonical gate name thin wrapper claim preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the historical canonical gate name thin wrapper claim preflight report"
fi

grep -q 'Thin Wrapper Claim Preflight' "$DOC" \
  || fail "architecture note must document Thin Wrapper Claim Preflight"
grep -q 'claim is performed as a thin wrapper' "$DOC" \
  || fail "architecture note must document that the claim is performed as a thin wrapper"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that this preflight does not invoke the historical gate"
grep -q 'snapshot evidence' "$DOC" \
  || fail "architecture note must document snapshot evidence"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "historical_canonical_gate_name_thin_wrapper_claim_preflight"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_migration_surface == "strict_missing_consumer_phase_migration"
  and .source_migration_ready == true
  and .source_strict_missing_consumer_count == 2
  and .source_phase_successor_available_count == 2
  and .source_blocking_consumer_count_after_phase_migration == 0
  and .source_strict_missing_consumers_mutated == false
  and .source_historical_missing_path_evidence_preserved == true
  and .source_historical_missing_path_evidence_basis == "historical_snapshot_evidence"
  and .source_historical_missing_path_current_filesystem_probe_used == false
  and .source_historical_snapshot_missing_canonical_summary == true
  and .source_current_wrapper_active_summary_source == true
  and .source_snapshot_surface == "historical_canonical_missing_path_snapshot_evidence"
  and .source_snapshot_ready == true
  and .source_snapshot_decouples_from_current_filesystem_state == true
  and .proposed_historical_canonical_gate_path == "scripts/hepta-systems-canonical-gate.sh"
  and .proposed_alias_target == "scripts/hepta-systems-current-canonical-wrapper-gate.sh"
  and .proposed_alias_kind == "thin_local_wrapper"
  and .proposed_alias_would_invoke_live_gates == false
  and .historical_canonical_gate_path_present == false
  and .historical_canonical_gate_path_present_at_snapshot == false
  and .historical_canonical_gate_path_probe_basis == "historical_snapshot_evidence"
  and .historical_canonical_gate_path_current_filesystem_probe_used == false
  and .claim_check_count == 5
  and (.claim_checks | all(.required == true and .satisfied == true))
  and .historical_canonical_gate_name_thin_wrapper_claim_preflight_ready == true
  and .historical_canonical_gate_name_claim_allowed == true
  and .historical_canonical_gate_name_claimed == true
  and .historical_canonical_gate_created == true
  and .historical_canonical_gate_executable == true
  and .historical_canonical_gate_wrapper_kind == "thin_local_exec_wrapper"
  and .historical_canonical_gate_wrapper_target == "scripts/hepta-systems-current-canonical-wrapper-gate.sh"
  and .historical_canonical_gate_wrapper_exec_count == 1
  and .historical_canonical_gate_mutated == true
  and .historical_canonical_gate_mutated_by_report == false
  and .wrapper_creation_performed == true
  and .wrapper_creation_performed_by_report == false
  and .wrapper_body_present == true
  and .wrapper_body_emitted == false
  and .wrapper_target_invoked == false
  and .canonical_gate_invoked == false
  and .capability_matrix_gate_invoked == false
  and .terminal_live_gate_invoked == false
  and .live_url_required == false
  and .long_soak_required == false
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "validate_historical_canonical_gate_thin_wrapper_without_live_invocation"
  and (.claim_blockers | index("historical_canonical_gate_thin_wrapper_validation_pending")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$MIGRATION_GATE" >/dev/null

printf 'hepta-systems-historical-canonical-gate-name-thin-wrapper-claim-preflight-gate: PASS: historical canonical gate name claim is recorded as a thin wrapper without invocation\n'
