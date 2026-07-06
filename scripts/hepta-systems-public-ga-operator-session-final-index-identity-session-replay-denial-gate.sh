#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-session-final-index-identity-session-replay-denial-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-session-non-binding-final-index-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_SESSION_FINAL_INDEX_IDENTITY_SESSION_REPLAY_DENIAL_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-session-final-index-identity-session-replay-denial-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session replay denial attachment report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Public GA operator session non-binding final index gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session replay denial architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session replay denial attachment report"
fi

grep -q 'Public GA Operator Identity/Session Replay Denial Attachment' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Replay Denial Attachment"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that attachment does not invoke replay gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed carry-through"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_replay_denial_attachment"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_session_non_binding_final_index_surface == "public_ga_operator_session_non_binding_final_index"
  and .source_public_ga_operator_session_non_binding_final_index_ready == true
  and .source_public_ga_operator_session_non_binding_final_index_blocked == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .public_ga_operator_session_non_binding_final_index_attached == true
  and .public_ga_operator_identity_session_replay_denial_attachment_ready == true
  and .public_ga_operator_identity_session_replay_denial_attachment_blocked == true
  and .operator_identity_session_replay_cross_binding_denial_gate_present == true
  and .operator_identity_session_replay_cross_binding_denial_doc_present == true
  and .operator_identity_session_replay_denial_static_mention_count >= 10
  and .operator_identity_session_replay_cross_binding_denial_gate_invoked == false
  and .operator_identity_session_binding_denial_gate_invoked == false
  and .operator_intent_consent_reconfirmation_gate_invoked == false
  and .long_soak_required_by_source_replay_gate == true
  and .long_soak_started == false
  and .public_ga_operator_packet_required_approval_static_count == 8
  and .public_ga_operator_approval_packet_invoked == false
  and .public_ga_operator_packet_sent == false
  and .operator_approval_request_sent == false
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .operator_identity_accepted == false
  and .operator_identity_hash_recorded == false
  and .operator_identity_hash_replayed == false
  and .operator_identity_replay_requested == false
  and .operator_identity_cross_binding_requested == false
  and .operator_session_accepted == false
  and .operator_session_recorded == false
  and .operator_session_persisted == false
  and .operator_session_bound == false
  and .operator_session_replay_requested == false
  and .operator_session_cross_binding_requested == false
  and .session_binding_recorded == false
  and .session_binding_persisted == false
  and .session_token_recorded == false
  and .session_token_replayed == false
  and .session_token_cross_bound == false
  and .cross_session_binding_accepted == false
  and .replay_acceptance_recorded == false
  and .replay_authority_derived == false
  and .cross_binding_authority_derived == false
  and .attachment_blocker_count == 26
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_replay_denial_readback_without_replay_acceptance"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-session-final-index-identity-session-replay-denial-gate: PASS: Public GA operator identity/session replay denial attachment is ready but blocked without live gate invocation\n'
