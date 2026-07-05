#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-reinstatement-ordering-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-reinstatement-final-index-ordering-denial-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_REINSTATEMENT_ORDERING_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-reinstatement-ordering-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session reinstatement ordering readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable Public GA operator identity/session reinstatement ordering attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session reinstatement ordering readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session reinstatement ordering readback report"
fi

grep -q 'Public GA Operator Identity/Session Reinstatement Ordering Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Reinstatement Ordering Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke ordering gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed carry-through"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_reinstatement_ordering_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_reinstatement_ordering_denial_attachment_surface == "public_ga_operator_identity_session_reinstatement_ordering_denial_attachment"
  and .source_public_ga_operator_identity_session_reinstatement_ordering_denial_attachment_ready == true
  and .source_public_ga_operator_identity_session_reinstatement_ordering_denial_attachment_blocked == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .public_ga_operator_identity_session_reinstatement_ordering_readback_ready == true
  and .public_ga_operator_identity_session_reinstatement_ordering_readback_blocked == true
  and .public_ga_operator_identity_session_reinstatement_ordering_denial_attachment_attached == true
  and .public_ga_operator_identity_session_reinstatement_denial_final_index_attached == true
  and .readback_check_count == 38
  and .operator_identity_session_reinstatement_ordering_monotonicity_denial_gate_invoked == false
  and .operator_identity_session_revocation_logout_replay_reinstatement_denial_gate_invoked == false
  and .long_soak_required_by_source_ordering_gate == true
  and .long_soak_started == false
  and .ordering_recorded == false
  and .ordering_persisted == false
  and .ordering_materialized == false
  and .sequence_cursor_recorded == false
  and .sequence_cursor_persisted == false
  and .monotonicity_state_recorded == false
  and .monotonicity_state_persisted == false
  and .latest_wins_accepted == false
  and .monotonic_cursor_accepted == false
  and .completion_order_recorded == false
  and .ordering_authority_derived == false
  and .readback_blocker_count == 32
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_reinstatement_ordering_final_index_without_ordering"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-reinstatement-ordering-readback-gate: PASS: Public GA operator identity/session reinstatement ordering readback is ready but blocked\n'
