#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-terminal-decision-status-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-terminal-decision-status-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_TERMINAL_DECISION_STATUS_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-terminal-decision-status-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session terminal decision/status final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable Public GA operator identity/session terminal decision/status readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session terminal decision/status final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session terminal decision/status final index report"
fi

grep -q 'Public GA Operator Identity/Session Terminal Decision/Status Promotion Final Index' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Terminal Decision/Status Promotion Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke terminal decision/status gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_terminal_decision_status_promotion_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_terminal_decision_status_promotion_readback_surface == "public_ga_operator_identity_session_terminal_decision_status_promotion_readback"
  and .source_public_ga_operator_identity_session_terminal_decision_status_promotion_readback_ready == true
  and .source_public_ga_operator_identity_session_terminal_decision_status_promotion_readback_blocked == true
  and .public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_ready == true
  and .public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_blocked == true
  and .public_ga_operator_identity_session_terminal_decision_status_promotion_readback_attached == true
  and .public_ga_operator_identity_session_final_acknowledgement_non_acceptance_final_index_attached == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .operator_identity_session_terminal_decision_status_promotion_denial_gate_present == true
  and .operator_identity_session_terminal_decision_status_promotion_denial_doc_present == true
  and .operator_identity_session_terminal_decision_status_promotion_denial_gate_invoked == false
  and .operator_identity_session_final_acknowledgement_non_acceptance_gate_invoked == false
  and .long_soak_started == false
  and .terminal_decision_accepted == false
  and .terminal_decision_recorded == false
  and .terminal_decision_persisted == false
  and .terminal_decision_delivered == false
  and .terminal_status_recorded == false
  and .terminal_status_persisted == false
  and .status_promotion_recorded == false
  and .channel_decision_delivered == false
  and .external_decision_sent == false
  and .telegram_decision_sent == false
  and .acceptance_from_terminal_decision_recorded == false
  and .operator_approval_from_terminal_status_derived == false
  and .release_publication_authority_from_terminal_status_derived == false
  and .activation_authority_from_terminal_status_derived == false
  and .activation_command_from_terminal_status_derived == false
  and .activation_from_terminal_status_allowed == false
  and .live_execution_from_terminal_status_allowed == false
  and .download_link_from_terminal_status_rendered == false
  and .install_command_from_terminal_status_rendered == false
  and .install_from_terminal_status_executed == false
  and .service_restart_from_terminal_status_performed == false
  and .active_binary_from_terminal_status_mutated == false
  and .public_status_exposed == false
  and .public_ga_status_exposed == false
  and .public_release_status_exposed == false
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .final_blocker_count == 46
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .next_migration_step == "attach_public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_to_public_ga_operator_identity_session_operator_intent_consent_reconfirmation_without_status_promotion"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-terminal-decision-status-final-index-gate: PASS: Public GA operator identity/session terminal decision/status final index is ready but blocked\n'
