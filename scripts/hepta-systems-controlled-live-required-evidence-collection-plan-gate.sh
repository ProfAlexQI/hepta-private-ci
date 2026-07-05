#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-required-evidence-collection-plan-report.sh"
NON_SEND_READBACK_GATE="$ROOT/scripts/hepta-systems-controlled-live-operator-packet-non-send-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_REQUIRED_EVIDENCE_COLLECTION_PLAN_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-required-evidence-collection-plan-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 5d report: $REPORT"
[[ -x "$NON_SEND_READBACK_GATE" ]] || fail "missing executable Phase 5c non-send readback gate: $NON_SEND_READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing Phase 5d architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 5d required evidence collection plan report"
fi

grep -q 'Controlled Live Required Evidence Collection Plan' "$DOC" \
  || fail "architecture note must document Controlled Live Required Evidence Collection Plan"
grep -q 'without recording evidence' "$DOC" \
  || fail "architecture note must document no evidence recording"
grep -q 'no approval request, approval acceptance, approval recording, evidence recording, evidence persistence, blocker waiver, credential read, packet send, readback persistence, ledger write, event-log write, SQLite write, Native POST mutation, Telegram transport mutation, gateway/auth mutation, channel send, replay, rollback, package, release, Public GA promotion, or live execution' "$DOC" \
  || fail "architecture note must document the closed evidence collection boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_required_evidence_collection_plan"
  and .status == "ready_blocked"
  and .gate == "controlled_live_required_evidence_collection_plan_gate"
  and .schema_version == "controlled_live_required_evidence_collection_plan_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_denial_readback_index_ready == true
  and .source_non_send_readback_ready == true
  and .source_blocker_count == 7
  and .lib_export_present == true
  and .plan_entry_count == 7
  and .queryable_plan_count == 7
  and .operator_visible_plan_count == 7
  and .required_evidence_count == 7
  and .evidence_recorded_count == 0
  and .evidence_collection_plan_ready == true
  and .approval_acceptance_ready == false
  and .approval_accepted == false
  and .blocker_waived_count == 0
  and .credential_read_allowed == false
  and .evidence_recording_allowed == false
  and .evidence_persisted == false
  and .controlled_live_cutover_ready == false
  and .live_execution_allowed == false
  and (.entries | length) == 7
  and (.entries | all(.queryable == true and .operator_visible == true and .evidence_required == true and .evidence_recorded == false and .evidence_recording_allowed == false and .credential_read_allowed == false and .approval_acceptance_allowed == false and .blocker_waiver_allowed == false and .persistence_allowed == false and .live_mutation_allowed == false))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing")
  and (.next_actions | index("phase5e_controlled_live_required_evidence_readback_index_without_recording")) != null
  and .next_migration_step == "phase5e_controlled_live_required_evidence_readback_index_without_recording"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$NON_SEND_READBACK_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_required_evidence_collection_plan --lib
)

printf 'hepta-systems-controlled-live-required-evidence-collection-plan-gate: PASS: required evidence is queryable without recording evidence, credentials, approvals, waivers, or live execution\n'
