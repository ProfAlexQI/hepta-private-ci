#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-reconfirmation-readback-report.sh"
ATTACHMENT_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-terminal-decision-status-final-index-intent-consent-reconfirmation-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_RECONFIRMATION_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-reconfirmation-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent readback report: $REPORT"
[[ -x "$ATTACHMENT_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent attachment gate: $ATTACHMENT_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent readback report"
fi

grep -q 'Public GA Operator Identity/Session Operator Intent/Consent Reconfirmation Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Operator Intent/Consent Reconfirmation Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke intent/consent gates"
grep -q '17 release/live blockers' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed blocker count"
grep -q 'runner_selector=2' "$DOC" \
  || fail "architecture note must document runner selector backfeed category"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_reconfirmation_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_reconfirmation_attachment_surface == "public_ga_operator_identity_session_operator_intent_consent_reconfirmation_attachment"
  and .source_public_ga_operator_identity_session_operator_intent_consent_reconfirmation_attachment_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_reconfirmation_attachment_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_reconfirmation_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_reconfirmation_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_reconfirmation_attachment_attached == true
  and .public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_attached == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | map(select(.id == "runner_selector" and .blocker_count == 2 and .queryable == true)) | length) == 1
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | map(select(.id == "dirty_worktree_owner_freeze" and .blocker_count == 2 and .queryable == true)) | length) == 1
  and .readback_mode == "static_operator_identity_session_intent_consent_reconfirmation_snapshot_only"
  and .readback_check_count == 54
  and .operator_identity_session_operator_intent_consent_reconfirmation_denial_gate_present == true
  and .operator_identity_session_operator_intent_consent_reconfirmation_denial_doc_present == true
  and .operator_identity_session_operator_intent_consent_reconfirmation_denial_gate_invoked == false
  and .operator_identity_session_terminal_decision_status_promotion_denial_gate_invoked == false
  and .long_soak_started == false
  and .operator_intent_reconfirmed == false
  and .operator_consent_reconfirmed == false
  and .operator_intent_recorded == false
  and .operator_consent_recorded == false
  and .consent_reconfirmation_recorded == false
  and .identity_signature_recorded == false
  and .session_consent_token_recorded == false
  and .operator_approval_from_intent_consent_derived == false
  and .acceptance_from_intent_consent_recorded == false
  and .terminal_decision_from_intent_consent_recorded == false
  and .terminal_status_from_intent_consent_recorded == false
  and .release_publication_authority_from_intent_consent_derived == false
  and .activation_authority_from_intent_consent_derived == false
  and .install_from_intent_consent_executed == false
  and .service_restart_from_intent_consent_performed == false
  and .active_binary_from_intent_consent_mutated == false
  and .readback_blocker_count == 48
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "derive_public_ga_operator_identity_session_operator_intent_consent_reconfirmation_final_index_without_status_promotion"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$ATTACHMENT_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-reconfirmation-readback-gate: PASS: Public GA operator identity/session intent consent readback is ready but blocked\n'
