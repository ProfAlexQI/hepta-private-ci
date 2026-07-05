#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-revocation-logout-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-replay-final-index-identity-session-revocation-logout-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-revocation-logout-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session revocation logout readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable Public GA operator identity/session revocation logout attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session revocation logout readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session revocation logout readback report"
fi

grep -q 'Public GA Operator Identity/Session Revocation Logout Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Revocation Logout Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke revocation/logout gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed carry-through"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_revocation_logout_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_revocation_logout_attachment_surface == "public_ga_operator_identity_session_revocation_logout_attachment"
  and .source_public_ga_operator_identity_session_revocation_logout_attachment_ready == true
  and .source_public_ga_operator_identity_session_revocation_logout_attachment_blocked == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .public_ga_operator_identity_session_revocation_logout_readback_ready == true
  and .public_ga_operator_identity_session_revocation_logout_readback_blocked == true
  and .public_ga_operator_identity_session_revocation_logout_attachment_attached == true
  and .readback_mode == "static_operator_identity_session_revocation_logout_snapshot_only"
  and .readback_check_count == 34
  and .operator_identity_session_revocation_logout_denial_gate_present == true
  and .operator_identity_session_revocation_logout_denial_doc_present == true
  and .operator_identity_session_revocation_logout_denial_gate_invoked == false
  and .operator_identity_session_replay_cross_binding_denial_gate_invoked == false
  and .long_soak_required_by_source_revocation_gate == true
  and .long_soak_started == false
  and .public_ga_operator_packet_required_approval_static_count == 8
  and .public_ga_operator_approval_packet_invoked == false
  and .public_ga_operator_packet_sent == false
  and .operator_approval_request_sent == false
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .operator_identity_revocation_requested == false
  and .operator_identity_revoked == false
  and .operator_session_revocation_requested == false
  and .operator_session_logout_requested == false
  and .operator_session_revoked == false
  and .operator_session_logged_out == false
  and .session_revocation_recorded == false
  and .session_logout_recorded == false
  and .session_lifecycle_status_promoted == false
  and .revocation_token_recorded == false
  and .logout_nonce_recorded == false
  and .revocation_logout_authority_derived == false
  and .readback_blocker_count == 28
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_revocation_logout_final_index_without_revocation_or_logout"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-revocation-logout-readback-gate: PASS: Public GA operator identity/session revocation logout readback is ready but blocked\n'
