#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-readback-index-report.sh"
EVIDENCE_PLAN_REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-collection-plan-report.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_READBACK_INDEX_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-readback-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 5e report: $REPORT"
[[ -x "$EVIDENCE_PLAN_REPORT" ]] || fail "missing executable Phase 5d evidence collection plan report: $EVIDENCE_PLAN_REPORT"
[[ -f "$DOC" ]] || fail "missing Phase 5e architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 5e required evidence readback index report"
fi

grep -q 'Controlled Live Required Evidence Readback Index' "$DOC" \
  || fail "architecture note must document Controlled Live Required Evidence Readback Index"
grep -q 'queryable and diffable without recording evidence' "$DOC" \
  || fail "architecture note must document queryable diffable no-recording behavior"
grep -q 'no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed evidence readback boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_readback_index"
  and .status == "ready_blocked"
  and .gate == "controlled_live_required_evidence_readback_index_gate"
  and .schema_version == "controlled_live_required_evidence_readback_index_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_evidence_collection_plan_ready == true
  and .source_plan_entry_count == 7
  and .lib_export_present == true
  and .index_entry_count == 7
  and .queryable_entry_count == 7
  and .operator_visible_entry_count == 7
  and .diffable_entry_count == 7
  and .fingerprint_count == 7
  and .evidence_recorded_count == 0
  and .readback_index_ready == true
  and .approval_acceptance_ready == false
  and .approval_accepted == false
  and .blocker_waived_count == 0
  and .credential_read_allowed == false
  and .evidence_recording_allowed == false
  and .evidence_persisted == false
  and .controlled_live_cutover_ready == false
  and .live_execution_allowed == false
  and (.entries | length) == 7
  and (.entries | all(.queryable == true and .operator_visible == true and .diffable == true and (.fingerprint | length) > 0 and .evidence_state == "missing" and .evidence_recorded == false and .evidence_recording_allowed == false and .credential_read_allowed == false and .approval_acceptance_allowed == false and .blocker_waiver_allowed == false and .persistence_allowed == false and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing")
  and (.next_actions | index("phase5f_controlled_live_required_evidence_gap_summary_without_acceptance")) != null
  and .next_migration_step == "phase5f_controlled_live_required_evidence_gap_summary_without_acceptance"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$EVIDENCE_PLAN_REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_collection_plan"
  and .status == "ready_blocked"
  and .evidence_collection_plan_ready == true
  and .plan_entry_count == 7
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
  cargo test -p hepta-runtime controlled_live_required_evidence_readback_index --lib
)

printf 'hepta-systems-controlled-live-required-evidence-readback-index-gate: PASS: required evidence readback index is queryable and diffable without evidence recording, credentials, approvals, waivers, persistence, or live execution\n'
