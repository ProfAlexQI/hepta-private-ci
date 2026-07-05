#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-reinstatement-cancellation-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-reinstatement-ordering-final-index-cancellation-supersession-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_REINSTATEMENT_CANCELLATION_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-reinstatement-cancellation-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session cancellation readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable Public GA operator identity/session cancellation attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session cancellation readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session cancellation readback report"
fi

grep -q 'Public GA Operator Identity/Session Cancellation Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Cancellation Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke cancellation gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed carry-through"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_reinstatement_cancellation_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_reinstatement_cancellation_attachment_surface == "public_ga_operator_identity_session_reinstatement_cancellation_supersession_attachment"
  and .source_public_ga_operator_identity_session_reinstatement_cancellation_attachment_ready == true
  and .source_public_ga_operator_identity_session_reinstatement_cancellation_attachment_blocked == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .public_ga_operator_identity_session_reinstatement_cancellation_readback_ready == true
  and .public_ga_operator_identity_session_reinstatement_cancellation_readback_blocked == true
  and .public_ga_operator_identity_session_reinstatement_ordering_final_index_attached == true
  and .readback_check_count == 40
  and .operator_identity_session_reinstatement_cancellation_supersession_denial_gate_invoked == false
  and .operator_identity_session_reinstatement_ordering_monotonicity_denial_gate_invoked == false
  and .long_soak_required_by_source_cancellation_gate == true
  and .long_soak_started == false
  and .cancellation_recorded == false
  and .supersession_recorded == false
  and .replacement_receipt_recorded == false
  and .tombstone_recorded == false
  and .delete_marker_recorded == false
  and .lifecycle_cancellation_supersession_recorded == false
  and .result_receipt_from_cancellation_supersession_recorded == false
  and .cancellation_supersession_authority_derived == false
  and .readback_blocker_count == 34
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_reinstatement_cancellation_final_index_without_cancellation"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-reinstatement-cancellation-readback-gate: PASS: Public GA operator identity/session cancellation readback is ready but blocked\n'
