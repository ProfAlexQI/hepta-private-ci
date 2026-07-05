#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-approval-non-acceptance-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-packet-final-index-operator-approval-non-acceptance-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_APPROVAL_NON_ACCEPTANCE_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-approval-non-acceptance-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator approval non-acceptance readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable Public GA operator approval non-acceptance attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator approval non-acceptance readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator approval non-acceptance readback report"
fi

grep -q 'Public GA Operator Approval Non-Acceptance Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Approval Non-Acceptance Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke operator approval gates"
grep -q 'canonical terminal closure backfeed' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed carry-through"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_approval_non_acceptance_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_approval_non_acceptance_attachment_surface == "public_ga_operator_approval_non_acceptance_attachment"
  and .source_public_ga_operator_approval_non_acceptance_attachment_ready == true
  and .source_public_ga_operator_approval_non_acceptance_attachment_blocked == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | length) == 4
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "runner_selector" and .blocker_count == 2)
  and any(.source_canonical_governance_tool_execution_closure_backfeed_categories[]; .id == "dirty_worktree_owner_freeze" and .blocker_count == 2)
  and .public_ga_operator_approval_non_acceptance_readback_ready == true
  and .public_ga_operator_approval_non_acceptance_readback_blocked == true
  and .public_ga_operator_approval_non_acceptance_attachment_attached == true
  and .readback_mode == "static_operator_approval_non_acceptance_snapshot_only"
  and .readback_check_count == 26
  and .operator_approval_non_acceptance_gate_present == true
  and .operator_approval_non_acceptance_doc_present == true
  and .operator_approval_non_acceptance_gate_invoked == false
  and .operator_approval_gap_ledger_summary_gate_invoked == false
  and .long_soak_required_by_source_non_acceptance_gate == true
  and .long_soak_started == false
  and .public_ga_operator_packet_required_approval_static_count == 8
  and .public_ga_operator_approval_packet_invoked == false
  and .public_ga_operator_packet_sent == false
  and .public_ga_operator_packet_recorded == false
  and .public_ga_operator_packet_accepted == false
  and .operator_approval_request_sent == false
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .operator_identity_accepted == false
  and .operator_acknowledgement_accepted == false
  and .telegram_send_performed == false
  and .external_send_performed == false
  and .readback_blocker_count == 20
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_approval_non_acceptance_final_index_without_packet_send"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-approval-non-acceptance-readback-gate: PASS: Public GA operator approval non-acceptance readback is ready but blocked\n'
