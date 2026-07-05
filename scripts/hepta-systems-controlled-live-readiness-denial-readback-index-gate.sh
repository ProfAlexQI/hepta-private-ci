#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-controlled-live-readiness-denial-readback-index-report.sh"
AUDIT_GATE="$ROOT/scripts/hepta-systems-controlled-live-readiness-audit-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_CONTROLLED_LIVE_READINESS_DENIAL_READBACK_INDEX_2026-06-27.md"

fail() {
  printf 'hepta-systems-controlled-live-readiness-denial-readback-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Phase 5a report: $REPORT"
[[ -x "$AUDIT_GATE" ]] || fail "missing executable Phase 5 audit gate: $AUDIT_GATE"
[[ -f "$DOC" ]] || fail "missing Phase 5a architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Phase 5a denial readback index report"
fi

grep -q 'Controlled Live Readiness Denial Readback Index' "$DOC" \
  || fail "architecture note must document Controlled Live Readiness Denial Readback Index"
grep -q 'queryable and operator-facing' "$DOC" \
  || fail "architecture note must document queryable and operator-facing blockers"
grep -q 'no waiver, acceptance, approval request, readback persistence, live execution, Native POST mutation, Telegram transport mutation, gateway/auth mutation, replay, rollback, package, release, or Public GA promotion' "$DOC" \
  || fail "architecture note must document the closed denial readback boundary"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "controlled_live_readiness_denial_readback_index"
  and .status == "ready_blocked"
  and .gate == "controlled_live_readiness_denial_readback_index_gate"
  and .schema_version == "controlled_live_readiness_denial_readback_index_v1"
  and .plugin_id == "hepta-system@hepta-local"
  and .source_controlled_live_audit_ready == true
  and .source_controlled_live_cutover_ready == false
  and .source_cutover_blocked == true
  and .source_blocker_count == 7
  and .source_blocking_precondition_count == 7
  and .source_audit_status == "ready_blocked"
  and .lib_export_present == true
  and .index_entry_count == 7
  and .queryable_entry_count == 7
  and .operator_facing_entry_count == 7
  and .readback_route_count == 7
  and .accepted_denial_count == 0
  and .waived_blocker_count == 0
  and .readback_index_ready == true
  and .controlled_live_cutover_ready == false
  and .ready_for_approval_request == false
  and .ready_for_approval_recording == false
  and .ready_for_readback_persistence == false
  and .ready_for_live_execution == false
  and (.entries | length) == 7
  and (.entries | all(.queryable == true and .operator_facing == true and .blocks_cutover == true and .operator_recoverable == true and .waiver_allowed == false and .acceptance_allowed == false and .live_mutation_allowed == false and .current_state == "missing"))
  and any(.entries[]; .source_blocker_id == "dirty_worktree_boundary" and .query_key == "controlled_live.blockers.dirty_worktree_boundary")
  and any(.entries[]; .source_blocker_id == "operator_live_approval_missing" and .query_key == "controlled_live.blockers.operator_live_approval_missing")
  and any(.entries[]; .source_blocker_id == "fresh_soak_readback_missing" and .query_key == "controlled_live.blockers.fresh_soak_readback_missing")
  and any(.entries[]; .source_blocker_id == "credential_boundary_attestation_missing" and .query_key == "controlled_live.blockers.credential_boundary_attestation_missing")
  and any(.entries[]; .source_blocker_id == "gateway_native_telegram_post_boundary_approval_missing" and .query_key == "controlled_live.blockers.gateway_native_telegram_post_boundary_approval_missing")
  and any(.entries[]; .source_blocker_id == "rollback_rehearsal_missing" and .query_key == "controlled_live.blockers.rollback_rehearsal_missing")
  and any(.entries[]; .source_blocker_id == "kill_switch_rehearsal_missing" and .query_key == "controlled_live.blockers.kill_switch_rehearsal_missing")
  and (.next_actions | index("phase5b_controlled_live_operator_packet_preview_without_approval_request")) != null
  and .next_migration_step == "phase5b_controlled_live_operator_packet_preview_without_approval_request"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$AUDIT_GATE" >/dev/null

(
  cd "$ROOT/codex-rs"
  cargo test -p hepta-runtime controlled_live_readiness_denial_readback_index --lib
)

printf 'hepta-systems-controlled-live-readiness-denial-readback-index-gate: PASS: controlled-live blockers are queryable and operator-facing without waiver, persistence, approval request, or live execution\n'
