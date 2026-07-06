#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-compact-capability-matrix-restore-preflight-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-tool-execution-canonical-summary-attachment-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_COMPACT_CAPABILITY_MATRIX_RESTORE_PREFLIGHT_2026-06-21.md"

fail() {
  printf 'hepta-systems-compact-capability-matrix-restore-preflight-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable compact capability matrix restore preflight report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable canonical summary attachment index gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing compact capability matrix restore preflight architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the compact capability matrix restore preflight report"
fi

grep -q 'Compact Capability Matrix Restore Preflight' "$DOC" \
  || fail "architecture note must document Compact Capability Matrix Restore Preflight"
grep -q 'manual apply check' "$DOC" \
  || fail "architecture note must document manual apply check"
grep -q 'does not replay the historical patch' "$DOC" \
  || fail "architecture note must document historical patch non-replay"
grep -q 'historical snapshot evidence' "$DOC" \
  || fail "architecture note must document historical snapshot evidence"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "compact_capability_matrix_restore_preflight"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready"
  and .source_attachment_surface == "tool_execution_canonical_summary_attachment_index"
  and .source_attachment_ready == true
  and .source_current_checkout_missing_canonical_summary == true
  and .source_historical_snapshot_missing_canonical_summary == true
  and .source_canonical_summary_probe_basis == "historical_snapshot_evidence"
  and .source_canonical_summary_current_filesystem_probe_used == false
  and .source_snapshot_surface == "historical_canonical_missing_path_snapshot_evidence"
  and .source_snapshot_ready == true
  and .source_snapshot_decouples_from_current_filesystem_state == true
  and .source_canonical_summary_present_count == 0
  and .source_historical_compact_capability_matrix_patch_call_count >= 1
  and .selected_patch_call_id == "call_rFtWhyTEAmT4jByPkr8d7L3f"
  and .selected_patch_replay_risk == "requires_missing_base_path_reconstruction"
  and .selected_patch_touched_path_count == 5
  and .selected_patch_missing_path_count == 5
  and .selected_patch_missing_path_count_basis == "historical_snapshot_evidence"
  and (.selected_patch_live_current_status_counts.missing // 0) >= 0
  and .selected_patch_body_emitted == false
  and .selected_patch_replay_applied == false
  and .manual_apply_check_entry_count == 5
  and .manual_apply_check_missing_count == 5
  and .manual_apply_check_basis == "historical_snapshot_evidence"
  and .manual_apply_check_current_filesystem_probe_used == false
  and .manual_apply_check_missing_at_snapshot_count == 5
  and (.manual_apply_check_entries | all(.historical_patch_touched == true and .missing_at_snapshot == true and .missing_evidence_basis == "historical_snapshot_evidence"))
  and any(.manual_apply_check_entries[]; .id == "historical_hepta_system_status_skill" and .restore_policy == "do_not_fabricate_plugin_fixture")
  and .current_summary_source_count == 3
  and (.current_summary_sources | all(.required == true and .available == true))
  and .historical_patch_replay_allowed == false
  and .patch_body_emission_allowed == false
  and .plugin_fixture_fabrication_allowed == false
  and .canonical_summary_mutation_allowed == false
  and .canonical_gate_invocation_allowed == false
  and .capability_matrix_gate_invocation_allowed == false
  and .restore_preflight_ready == true
  and .tool_execution_live_cutover_allowed == false
  and .tool_execution_public_ga_allowed == false
  and .next_migration_step == "migrate_attachment_phase_index_to_snapshot_evidence_source_before_wrapper_creation"
  and (.restore_blockers | index("historical_plugin_fixture_path_missing_do_not_fabricate")) != null
  and (.restore_blockers | index("historical_snapshot_canonical_summary_not_restored_yet")) != null
  and (.restore_blockers | index("manual_operator_live_cutover_approval_required")) != null
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

printf 'hepta-systems-compact-capability-matrix-restore-preflight-gate: PASS: manual apply check blocks blind replay and plugin fixture fabrication\n'
