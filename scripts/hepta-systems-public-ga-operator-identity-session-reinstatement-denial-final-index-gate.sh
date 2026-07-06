#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-reinstatement-denial-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-reinstatement-denial-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_REINSTATEMENT_DENIAL_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-reinstatement-denial-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session reinstatement denial final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable Public GA operator identity/session reinstatement denial readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session reinstatement denial final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session reinstatement denial final index report"
fi

grep -q 'Public GA Operator Identity/Session Reinstatement Denial Final Index' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Reinstatement Denial Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke reinstatement gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed carry-through"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_reinstatement_denial_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_reinstatement_denial_readback_surface == "public_ga_operator_identity_session_reinstatement_denial_readback"
  and .source_public_ga_operator_identity_session_reinstatement_denial_readback_ready == true
  and .source_public_ga_operator_identity_session_reinstatement_denial_readback_blocked == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .public_ga_operator_identity_session_reinstatement_denial_final_index_ready == true
  and .public_ga_operator_identity_session_reinstatement_denial_final_index_blocked == true
  and .public_ga_operator_identity_session_reinstatement_denial_readback_attached == true
  and .public_ga_operator_identity_session_revocation_logout_final_index_attached == true
  and .operator_identity_session_revocation_logout_replay_reinstatement_denial_gate_present == true
  and .operator_identity_session_revocation_logout_replay_reinstatement_denial_doc_present == true
  and .operator_identity_session_revocation_logout_replay_reinstatement_denial_gate_invoked == false
  and .operator_identity_session_revocation_logout_denial_gate_invoked == false
  and .operator_identity_session_replay_cross_binding_denial_gate_invoked == false
  and .operator_identity_session_binding_denial_gate_invoked == false
  and .operator_intent_consent_reconfirmation_gate_invoked == false
  and .long_soak_required_by_source_reinstatement_gate == true
  and .long_soak_started == false
  and .public_ga_operator_packet_required_approval_static_count == 8
  and .operator_identity_revocation_requested == false
  and .operator_identity_revoked == false
  and .operator_identity_reinstatement_requested == false
  and .operator_identity_reinstated == false
  and .operator_session_revocation_requested == false
  and .operator_session_logout_requested == false
  and .operator_session_revoked == false
  and .operator_session_logged_out == false
  and .operator_session_reinstatement_requested == false
  and .operator_session_reinstated == false
  and .session_reinstatement_recorded == false
  and .session_reinstatement_persisted == false
  and .session_lifecycle_status_promoted == false
  and .reinstatement_token_recorded == false
  and .reinstatement_nonce_recorded == false
  and .device_session_reinstatement_recorded == false
  and .revocation_logout_replay_accepted == false
  and .logout_replay_accepted == false
  and .reinstatement_authority_derived == false
  and .final_blocker_count == 30
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "attach_public_ga_operator_identity_session_reinstatement_denial_final_index_to_public_ga_operator_identity_session_reinstatement_ordering_without_reinstatement"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-reinstatement-denial-final-index-gate: PASS: Public GA operator identity/session reinstatement denial final index is ready but blocked\n'
