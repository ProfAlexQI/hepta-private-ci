#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-report.sh"
GAP_SUMMARY_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-summary-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_DIFF_VIEW_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-gap-diff-view-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 5g report: $REPORT"
[[ -x "$GAP_SUMMARY_REPORT" ]] || fail "missing executable Phase 5f gap summary report: $GAP_SUMMARY_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 5g architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 5g required evidence gap diff view report"
fi

grep -q 'Controlled Live Required Evidence Gap Diff View' "$DOC" \
  || fail "architecture note must document Controlled Live Required Evidence Gap Diff View"
grep -q 'comparable across readbacks without accepting evidence' "$DOC" \
  || fail "architecture note must document comparable readbacks without acceptance"
grep -q 'no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed evidence diff-view boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_gap_diff_view"
  and .status == "ready_blocked"
  and .gate == "controlled_live_required_evidence_gap_diff_view_gate"
  and .schema_version == "controlled_live_required_evidence_gap_diff_view_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_gap_summary_ready == true
  and .source_gap_entry_count == 7
  and .source_missing_evidence_count == 7
  and .lib_export_present == true
  and .diff_entry_count == 7
  and .stable_diff_key_count == 7
  and .comparable_entry_count == 7
  and .unchanged_missing_count == 7
  and .owner_count == 7
  and .risk_bucket_count == 3
  and .evidence_recorded_count == 0
  and .diff_view_ready == true
  and .approval_acceptance_ready == false
  and .approval_accepted == false
  and .blocker_waived_count == 0
  and .credential_read_allowed == false
  and .evidence_recording_allowed == false
  and .evidence_persisted == false
  and .controlled_live_cutover_ready == false
  and .live_execution_allowed == false
  and (.entries | length) == 7
  and (.entries | all(.queryable == true and .operator_visible == true and .comparable == true and .previous_state == "missing" and .current_state == "missing" and .state_delta == "unchanged_missing" and (.diff_view_key | length) > 0 and (.comparison_anchor | length) > 0 and (.gap_key | length) > 0 and .evidence_recorded == false and .evidence_recording_allowed == false and .credential_read_allowed == false and .approval_acceptance_allowed == false and .blocker_waiver_allowed == false and .persistence_allowed == false and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .owner == "hepta_systems_lane_owner" and .risk_bucket == "medium" and .comparison_anchor == "gap-summary-owner-risk:dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .owner == "operator" and .risk_bucket == "critical" and .comparison_anchor == "gap-summary-owner-risk:operator-live-approval-missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .owner == "runtime_soak_owner" and .risk_bucket == "high" and .comparison_anchor == "gap-summary-owner-risk:fresh-soak-readback-missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing" and .owner == "credential_boundary_owner" and .risk_bucket == "critical" and .comparison_anchor == "gap-summary-owner-risk:credential-boundary-attestation-missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing" and .owner == "transport_boundary_owner" and .risk_bucket == "critical" and .comparison_anchor == "gap-summary-owner-risk:gateway-native-telegram-post-boundary-approval-missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing" and .owner == "rollback_rehearsal_owner" and .risk_bucket == "high" and .comparison_anchor == "gap-summary-owner-risk:rollback-rehearsal-missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing" and .owner == "kill_switch_owner" and .risk_bucket == "high" and .comparison_anchor == "gap-summary-owner-risk:kill-switch-rehearsal-missing")
  and (.next_actions | index("phase5h_controlled_live_required_evidence_gap_operator_readback_without_acceptance")) != null
  and .next_migration_step == "phase5h_controlled_live_required_evidence_gap_operator_readback_without_acceptance"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$GAP_SUMMARY_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_gap_summary"
  and .status == "ready_blocked"
  and .gap_summary_ready == true
  and .gap_entry_count == 7
  and .missing_evidence_count == 7
  and .evidence_recorded_count == 0
  and .approval_accepted == false
  and .blocker_waived_count == 0
  and .credential_read_allowed == false
  and .evidence_recording_allowed == false
  and .evidence_persisted == false
  and .controlled_live_cutover_ready == false
  and .live_execution_allowed == false
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_required_evidence_gap_diff_view --lib
)

printf 'hepta-systems-controlled-live-required-evidence-gap-diff-view-gate: PASS: evidence gaps are diffable across readbacks without acceptance, recording, credentials, waivers, persistence, or live execution\n'
