#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-operator-readback-report.sh"
GAP_DIFF_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-diff-view-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_OPERATOR_READBACK_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-gap-operator-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 5h report: $REPORT"
[[ -x "$GAP_DIFF_REPORT" ]] || fail "missing executable Phase 5g gap diff view report: $GAP_DIFF_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 5h architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 5h required evidence gap operator readback report"
fi

grep -q 'Controlled Live Required Evidence Gap Operator Readback' "$DOC" \
  || fail "architecture note must document Controlled Live Required Evidence Gap Operator Readback"
grep -q 'operator-facing readback without accepting evidence' "$DOC" \
  || fail "architecture note must document operator-facing readback without acceptance"
grep -q 'no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed operator readback boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_gap_operator_readback"
  and .status == "ready_blocked"
  and .gate == "controlled_live_required_evidence_gap_operator_readback_gate"
  and .schema_version == "controlled_live_required_evidence_gap_operator_readback_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_diff_view_ready == true
  and .source_diff_entry_count == 7
  and .source_unchanged_missing_count == 7
  and .lib_export_present == true
  and .operator_readback_entry_count == 7
  and .operator_visible_entry_count == 7
  and .stable_readback_key_count == 7
  and .unchanged_missing_count == 7
  and .owner_count == 7
  and .risk_bucket_count == 3
  and .evidence_recorded_count == 0
  and .operator_readback_ready == true
  and .approval_acceptance_ready == false
  and .approval_accepted == false
  and .blocker_waived_count == 0
  and .credential_read_allowed == false
  and .evidence_recording_allowed == false
  and .evidence_persisted == false
  and .readback_persisted == false
  and .controlled_live_cutover_ready == false
  and .live_execution_allowed == false
  and (.entries | length) == 7
  and (.entries | all(.queryable == true and .operator_visible == true and .comparable == true and .operator_status == "blocked_missing_evidence" and .operator_action == "collect_required_evidence_before_live_cutover" and .previous_state == "missing" and .current_state == "missing" and .state_delta == "unchanged_missing" and (.operator_readback_key | length) > 0 and (.operator_readback_route | length) > 0 and .operator_display_order > 0 and .evidence_recorded == false and .evidence_recording_allowed == false and .credential_read_allowed == false and .approval_acceptance_allowed == false and .blocker_waiver_allowed == false and .persistence_allowed == false and .readback_persistence_allowed == false and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .operator_display_order == 1 and .operator_readback_route == "readback://controlled-live/required-evidence/gap/operator/dirty-worktree-boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .operator_display_order == 2 and .operator_readback_route == "readback://controlled-live/required-evidence/gap/operator/operator-live-approval-missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .operator_display_order == 3 and .operator_readback_route == "readback://controlled-live/required-evidence/gap/operator/fresh-soak-readback-missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing" and .operator_display_order == 4 and .operator_readback_route == "readback://controlled-live/required-evidence/gap/operator/credential-boundary-attestation-missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing" and .operator_display_order == 5 and .operator_readback_route == "readback://controlled-live/required-evidence/gap/operator/gateway-native-telegram-post-boundary-approval-missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing" and .operator_display_order == 6 and .operator_readback_route == "readback://controlled-live/required-evidence/gap/operator/rollback-rehearsal-missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing" and .operator_display_order == 7 and .operator_readback_route == "readback://controlled-live/required-evidence/gap/operator/kill-switch-rehearsal-missing")
  and (.next_actions | index("phase5i_controlled_live_required_evidence_gap_operator_packet_attachment_without_acceptance")) != null
  and .next_migration_step == "phase5i_controlled_live_required_evidence_gap_operator_packet_attachment_without_acceptance"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$GAP_DIFF_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_gap_diff_view"
  and .status == "ready_blocked"
  and .diff_view_ready == true
  and .diff_entry_count == 7
  and .unchanged_missing_count == 7
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
  cargo test -p hepta-runtime controlled_live_required_evidence_gap_operator_readback --lib
)

printf 'hepta-systems-controlled-live-required-evidence-gap-operator-readback-gate: PASS: evidence gaps are operator-facing readbacks without acceptance, recording, credentials, waivers, persistence, or live execution\n'
