#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-operator-facing-summary-briefing-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-operator-facing-summary-briefing-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_OPERATOR_FACING_SUMMARY_BRIEFING_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-operator-facing-summary-briefing-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session summary briefing final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable Public GA operator identity/session summary briefing readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session summary briefing final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session summary briefing final index report"
fi

grep -q 'Public GA Operator Identity/Session Operator-Facing Summary Briefing Final Index' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Operator-Facing Summary Briefing Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke summary/briefing gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_facing_summary_briefing_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_facing_summary_briefing_readback_surface == "public_ga_operator_identity_session_operator_facing_summary_briefing_readback"
  and .source_public_ga_operator_identity_session_operator_facing_summary_briefing_readback_ready == true
  and .source_public_ga_operator_identity_session_operator_facing_summary_briefing_readback_blocked == true
  and .public_ga_operator_identity_session_operator_facing_summary_briefing_final_index_ready == true
  and .public_ga_operator_identity_session_operator_facing_summary_briefing_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_facing_summary_briefing_readback_attached == true
  and .public_ga_operator_identity_session_export_query_observability_final_index_attached == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .operator_identity_session_operator_facing_summary_briefing_non_persistence_gate_present == true
  and .operator_identity_session_operator_facing_summary_briefing_non_persistence_doc_present == true
  and .operator_identity_session_operator_facing_summary_briefing_non_persistence_gate_invoked == false
  and .operator_identity_session_export_query_observability_denial_gate_invoked == false
  and .long_soak_required_by_source_summary_briefing_gate == true
  and .long_soak_started == false
  and .operator_summary_recorded == false
  and .operator_summary_persisted == false
  and .operator_briefing_recorded == false
  and .operator_briefing_persisted == false
  and .readback_digest_recorded == false
  and .status_banner_recorded == false
  and .exported_summary_text_recorded == false
  and .operator_briefing_card_materialized == false
  and .notification_recorded == false
  and .timeline_recorded == false
  and .briefing_delivery_recorded == false
  and .briefing_delivery_performed == false
  and .approval_summary_recorded == false
  and .external_briefing_sent == false
  and .telegram_briefing_sent == false
  and .summary_briefing_acceptance_recorded == false
  and .result_receipt_from_summary_briefing_recorded == false
  and .completion_ack_recorded == false
  and .operator_approval_from_summary_briefing_accepted == false
  and .release_publication_authority_from_summary_briefing_derived == false
  and .activation_authority_from_summary_briefing_derived == false
  and .download_link_from_summary_briefing_rendered == false
  and .install_command_from_summary_briefing_rendered == false
  and .install_from_summary_briefing_executed == false
  and .service_restart_from_summary_briefing_performed == false
  and .active_binary_from_summary_briefing_mutated == false
  and .final_blocker_count == 42
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "attach_public_ga_operator_identity_session_operator_facing_summary_briefing_final_index_to_public_ga_operator_identity_session_final_acknowledgement_non_acceptance_without_summary"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-operator-facing-summary-briefing-final-index-gate: PASS: Public GA operator identity/session summary briefing final index is ready but blocked\n'
