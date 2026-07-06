#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-strict-missing-consumer-phase-migration-report.sh"
PREFLIGHT_GATE="$ROOT/scripts/hepta-systems-historical-canonical-gate-name-reintroduction-preflight-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_STRICT_MISSING_CONSUMER_PHASE_MIGRATION_2026-06-21.md"

fail() {
  printf 'hepta-systems-strict-missing-consumer-phase-migration-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable strict-missing consumer phase migration report: $REPORT"
[[ -x "$PREFLIGHT_GATE" ]] || fail "missing executable historical canonical gate name preflight gate: $PREFLIGHT_GATE"
[[ -f "$DOC" ]] || fail "missing strict-missing consumer phase migration architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the strict-missing consumer phase migration report"
fi

grep -q 'Strict-Missing Consumer Phase Migration' "$DOC" \
  || fail "architecture note must document Strict-Missing Consumer Phase Migration"
grep -q 'phase successor' "$DOC" \
  || fail "architecture note must document phase successor behavior"
grep -q 'does not mutate strict-missing consumers' "$DOC" \
  || fail "architecture note must document that migration does not mutate strict-missing consumers"
grep -q 'snapshot evidence' "$DOC" \
  || fail "architecture note must document snapshot evidence"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "strict_missing_consumer_phase_migration"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_preflight_surface == "historical_canonical_gate_name_reintroduction_preflight"
  and .source_preflight_ready == true
  and .source_phase_index_ready == true
  and .source_historical_missing_path_evidence_preserved == true
  and .source_historical_missing_path_evidence_basis == "historical_snapshot_evidence"
  and .source_historical_missing_path_current_filesystem_probe_used == false
  and .source_historical_snapshot_missing_canonical_summary == true
  and .source_historical_snapshot_evidence_consumable_after_wrapper_creation == true
  and .source_current_wrapper_phase_available == true
  and .source_current_wrapper_active_summary_source == true
  and .source_dependent_consumer_count == 3
  and .source_dependent_strict_missing_consumer_count == 2
  and .source_dependent_blocking_consumer_count == 0
  and .strict_missing_consumer_count == 2
  and .phase_successor_available_count == 2
  and .phase_migration_ready_count == 2
  and .phase_successor_missing_count == 0
  and .strict_missing_evidence_preserved_count == 2
  and .blocking_consumer_count_before_phase_migration == 0
  and .blocking_consumer_count_after_phase_migration == 0
  and .strict_missing_consumer_phase_migration_ready == true
  and .strict_missing_consumer_mutation_allowed == false
  and .strict_missing_consumers_mutated == false
  and .historical_missing_path_evidence_preserved == true
  and .current_wrapper_active_summary_source == true
  and .ready_to_prepare_historical_name_claim_preflight == true
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
  and .proposed_historical_canonical_gate_path == "scripts/hepta-systems-canonical-gate.sh"
  and .proposed_alias_target == "scripts/hepta-systems-current-canonical-wrapper-gate.sh"
  and .proposed_alias_kind == "thin_local_wrapper"
  and .proposed_alias_would_invoke_live_gates == false
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "validate_historical_canonical_gate_thin_wrapper_without_live_invocation"
  and any(.migrated_strict_missing_consumers[]; .id == "canonical_summary_attachment_index_gate" and .phase_successor_id == "canonical_summary_attachment_phase_index_gate" and .phase_successor_available == true and .source_consumer_mutated == false and .would_block_next_name_claim_preflight == false)
  and any(.migrated_strict_missing_consumers[]; .id == "compact_capability_restore_preflight_gate" and .phase_successor_id == "current_canonical_wrapper_gate" and .phase_successor_available == true and .source_consumer_mutated == false and .would_block_next_name_claim_preflight == false)
  and (.migration_blockers | index("historical_canonical_gate_thin_wrapper_validation_pending")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

printf 'hepta-systems-strict-missing-consumer-phase-migration-gate: PASS: strict-missing consumers have phase successors with historical thin wrapper claim and without live invocation\n'
