#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-gap-summary-report.sh"
READBACK_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-readback-index-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_GAP_SUMMARY_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-gap-summary-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 5f report: $REPORT"
[[ -x "$READBACK_REPORT" ]] || fail "missing executable Phase 5e readback index report: $READBACK_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 5f architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 5f required evidence gap summary report"
fi

grep -q 'Controlled Live Required Evidence Gap Summary' "$DOC" \
  || fail "architecture note must document Controlled Live Required Evidence Gap Summary"
grep -q 'by owner and cutover risk without accepting evidence' "$DOC" \
  || fail "architecture note must document owner/risk summary without acceptance"
grep -q 'no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed evidence gap boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_gap_summary"
  and .status == "ready_blocked"
  and .gate == "controlled_live_required_evidence_gap_summary_gate"
  and .schema_version == "controlled_live_required_evidence_gap_summary_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_readback_index_ready == true
  and .source_index_entry_count == 7
  and .source_missing_evidence_count == 7
  and .lib_export_present == true
  and .gap_entry_count == 7
  and .missing_evidence_count == 7
  and .owner_count == 7
  and .risk_bucket_count == 3
  and .high_risk_gap_count == 6
  and .operator_visible_gap_count == 7
  and .queryable_gap_count == 7
  and .evidence_recorded_count == 0
  and .gap_summary_ready == true
  and .approval_acceptance_ready == false
  and .approval_accepted == false
  and .blocker_waived_count == 0
  and .credential_read_allowed == false
  and .evidence_recording_allowed == false
  and .evidence_persisted == false
  and .controlled_live_cutover_ready == false
  and .live_execution_allowed == false
  and (.entries | length) == 7
  and (.entries | all(.queryable == true and .operator_visible == true and .evidence_missing == true and .evidence_state == "missing" and (.gap_key | length) > 0 and (.cutover_risk | length) > 0 and .evidence_recorded == false and .evidence_recording_allowed == false and .credential_read_allowed == false and .approval_acceptance_allowed == false and .blocker_waiver_allowed == false and .persistence_allowed == false and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .owner == "hepta_systems_lane_owner" and .risk_bucket == "medium")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .owner == "operator" and .risk_bucket == "critical")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .owner == "runtime_soak_owner" and .risk_bucket == "high")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing" and .owner == "credential_boundary_owner" and .risk_bucket == "critical")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing" and .owner == "transport_boundary_owner" and .risk_bucket == "critical")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing" and .owner == "rollback_rehearsal_owner" and .risk_bucket == "high")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing" and .owner == "kill_switch_owner" and .risk_bucket == "high")
  and (.next_actions | index("phase5g_controlled_live_required_evidence_gap_diff_view_without_acceptance")) != null
  and .next_migration_step == "phase5g_controlled_live_required_evidence_gap_diff_view_without_acceptance"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_readback_index"
  and .status == "ready_blocked"
  and .readback_index_ready == true
  and .index_entry_count == 7
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
  cargo test -p hepta-runtime controlled_live_required_evidence_gap_summary --lib
)

printf 'hepta-systems-controlled-live-required-evidence-gap-summary-gate: PASS: evidence gaps are summarized by owner/risk without acceptance, recording, credentials, waivers, persistence, or live execution\n'
