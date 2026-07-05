#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-historical-canonical-missing-path-snapshot-evidence-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_HISTORICAL_CANONICAL_MISSING_PATH_SNAPSHOT_EVIDENCE_2026-06-21.md"

fail() {
  printf 'hepta-systems-historical-canonical-missing-path-snapshot-evidence-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable historical canonical missing path snapshot evidence report: $REPORT"
[[ -f "$DOC" ]] || fail "missing historical canonical missing path snapshot evidence architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the historical canonical missing path snapshot evidence report"
fi

grep -q 'Snapshot Evidence' "$DOC" \
  || fail "architecture note must document Snapshot Evidence"
grep -q 'does not use a live absence probe' "$DOC" \
  || fail "architecture note must document that snapshot evidence does not use a live absence probe"
grep -q 'historical path was absent' "$DOC" \
  || fail "architecture note must document that the historical path was absent"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "historical_canonical_missing_path_snapshot_evidence"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .snapshot_kind == "historical_missing_path_evidence"
  and .snapshot_capture_state == "pre_historical_canonical_gate_wrapper_creation"
  and (.snapshot_source_surfaces | index("tool_execution_canonical_summary_attachment_index")) != null
  and (.snapshot_source_surfaces | index("historical_canonical_gate_post_claim_impact_preflight")) != null
  and .snapshot_source_gate_status == "previously_verified_pass"
  and .snapshot_runtime_live_absence_probe_used == false
  and .snapshot_current_filesystem_probe_used == false
  and .snapshot_decouples_from_current_filesystem_state == true
  and .historical_canonical_gate_path == "scripts/hepta-systems-canonical-gate.sh"
  and .historical_canonical_gate_path_present_at_snapshot == false
  and .canonical_summary_present_count_at_snapshot == 0
  and .current_checkout_missing_canonical_summary_at_snapshot == true
  and .canonical_summary_available_at_snapshot == false
  and .canonical_summary_probe_count_at_snapshot == 8
  and .missing_canonical_source_count_at_snapshot == 8
  and (.missing_canonical_source_ids_at_snapshot | index("hepta_systems_canonical_gate")) != null
  and (.canonical_source_snapshot_probes | all(.present_at_snapshot == false))
  and .historical_compact_capability_matrix_patch_call_count_at_snapshot == 291
  and .historical_compact_capability_matrix_missing_path_count_at_snapshot == 39
  and .selected_reconstruction_candidate_snapshot.call_id == "call_rFtWhyTEAmT4jByPkr8d7L3f"
  and .selected_reconstruction_candidate_snapshot.patch_line_count == 417
  and (.selected_reconstruction_candidate_snapshot.touched_paths | index("scripts/hepta-systems-canonical-gate.sh")) != null
  and .selected_reconstruction_candidate_snapshot.current_status_counts_at_snapshot.missing == 5
  and .selected_reconstruction_candidate_snapshot.replay_risk == "requires_missing_base_path_reconstruction"
  and .selected_reconstruction_candidate_snapshot.patch_body_emitted == false
  and .selected_reconstruction_candidate_snapshot.replay_applied == false
  and .post_claim_impact_consumer_count_at_snapshot == 12
  and .post_claim_live_absence_probe_consumer_count_at_snapshot == 12
  and .post_claim_blocking_consumer_count_at_snapshot == 12
  and .snapshot_decoupling_required == true
  and .snapshot_evidence_ready == true
  and .historical_missing_path_snapshot_evidence_ready == true
  and .historical_snapshot_evidence_consumable_after_wrapper_creation == true
  and .historical_canonical_gate_name_creation_allowed_now == false
  and .historical_canonical_gate_name_claimed == false
  and .historical_canonical_gate_mutated == false
  and .wrapper_creation_performed == false
  and .execution_enabled_count == 0
  and .public_ga_enabled_count == 0
  and .manual_operator_live_cutover_approval_required == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "migrate_attachment_index_to_snapshot_evidence_source_before_wrapper_creation"
  and (.snapshot_blockers | index("affected_consumers_not_migrated_to_snapshot_evidence_yet")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

printf 'hepta-systems-historical-canonical-missing-path-snapshot-evidence-gate: PASS: historical missing-path evidence is snapshot-decoupled from current filesystem state\n'
