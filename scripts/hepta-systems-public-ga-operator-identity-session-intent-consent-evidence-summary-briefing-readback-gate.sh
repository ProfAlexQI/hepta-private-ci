#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-summary-briefing-readback-report.sh"
SOURCE_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-intent-consent-evidence-export-query-observability-final-index-summary-briefing-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_SUMMARY_BRIEFING_READBACK_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-summary-briefing-readback-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent evidence summary briefing readback report: $REPORT"
[[ -x "$SOURCE_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent evidence summary briefing attachment gate: $SOURCE_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent evidence summary briefing readback architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent evidence summary briefing readback report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Summary Briefing Readback' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Summary Briefing Readback"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that readback does not invoke summary/briefing gates"
grep -q '17 release/live blockers' "$DOC" \
  || fail "architecture note must document canonical terminal closure backfeed blocker count"
grep -q 'runner_selector=2' "$DOC" \
  || fail "architecture note must document runner selector backfeed category"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_readback"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_attachment_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_attachment"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_attachment_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_attachment_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_attachment_attached == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_export_query_observability_final_index_attached == true
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | map(select(.id == "runner_selector" and .blocker_count == 2 and .queryable == true)) | length) == 1
  and (.source_canonical_governance_tool_execution_closure_backfeed_categories | map(select(.id == "dirty_worktree_owner_freeze" and .blocker_count == 2 and .queryable == true)) | length) == 1
  and .operator_identity_session_operator_intent_consent_evidence_summary_briefing_non_persistence_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_summary_briefing_non_persistence_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_summary_briefing_non_persistence_gate_invoked == false
  and .operator_identity_session_operator_intent_consent_evidence_export_query_observability_denial_gate_invoked == false
  and .long_soak_started == false
  and .operator_summary_recorded == false
  and .operator_summary_persisted == false
  and .operator_briefing_recorded == false
  and .operator_briefing_persisted == false
  and .readback_digest_recorded == false
  and .status_banner_recorded == false
  and .exported_summary_text_recorded == false
  and .operator_briefing_card_materialized == false
  and .briefing_delivery_recorded == false
  and .external_briefing_sent == false
  and .telegram_briefing_sent == false
  and .summary_briefing_acceptance_recorded == false
  and .final_operator_acknowledgement_accepted == false
  and .release_publication_authority_from_summary_briefing_derived == false
  and .activation_authority_from_summary_briefing_derived == false
  and .install_from_summary_briefing_executed == false
  and .service_restart_from_summary_briefing_performed == false
  and .active_binary_from_summary_briefing_mutated == false
  and .readback_check_count == 60
  and .readback_blocker_count == 54
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$SOURCE_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-summary-briefing-readback-gate: PASS: Public GA operator identity/session intent consent evidence summary briefing readback is ready but blocked\n'
