#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-final-acknowledgement-final-index-report.sh"
READBACK_GATE="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-final-acknowledgement-readback-gate.sh"
DOC="$ROOT/docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_FINAL_ACKNOWLEDGEMENT_FINAL_INDEX_2026-06-21.md"

fail() {
  printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-final-acknowledgement-final-index-gate: FAIL: %s\n' "$1" >&2
  exit 1
}

[[ -x "$REPORT" ]] || fail "missing executable Public GA operator identity/session intent consent evidence final acknowledgement final index report: $REPORT"
[[ -x "$READBACK_GATE" ]] || fail "missing executable Public GA operator identity/session intent consent evidence final acknowledgement readback gate: $READBACK_GATE"
[[ -f "$DOC" ]] || fail "missing Public GA operator identity/session intent consent evidence final acknowledgement final index architecture note: $DOC"

if ! command -v jq >/dev/null 2>&1; then
  fail "jq is required to validate the Public GA operator identity/session intent consent evidence final acknowledgement final index report"
fi

grep -q 'Public GA Operator Identity/Session Intent/Consent Evidence Final Acknowledgement Non-Acceptance Final Index' "$DOC" \
  || fail "architecture note must document Public GA Operator Identity/Session Intent/Consent Evidence Final Acknowledgement Non-Acceptance Final Index"
grep -q 'ready-but-blocked' "$DOC" \
  || fail "architecture note must document ready-but-blocked status"
grep -q 'does not invoke' "$DOC" \
  || fail "architecture note must document that final index does not invoke final acknowledgement gates"

"$REPORT" | jq -e '
  .runtime == "hepta"
  and .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_final_index"
  and .plugin_id == "hepta-system@hepta-local"
  and .status == "ready_blocked"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_readback_surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_readback"
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_readback_ready == true
  and .source_public_ga_operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_readback_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_final_index_blocked == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_readback_attached == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_summary_briefing_final_index_attached == true
  and .operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_gate_present == true
  and .operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_doc_present == true
  and .operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_gate_invoked == false
  and .operator_identity_session_operator_intent_consent_evidence_summary_briefing_non_persistence_gate_invoked == false
  and .long_soak_started == false
  and .public_ga_operator_approval_packet_invoked == false
  and .public_ga_operator_packet_sent == false
  and .operator_approval_request_sent == false
  and .operator_approval_recorded == false
  and .operator_approval_accepted == false
  and .operator_summary_recorded == false
  and .operator_briefing_recorded == false
  and .summary_briefing_acceptance_recorded == false
  and .final_operator_acknowledgement_accepted == false
  and .final_operator_acknowledgement_recorded == false
  and .final_operator_acknowledgement_persisted == false
  and .final_operator_acknowledgement_delivered == false
  and .operator_received_recorded == false
  and .operator_confirmed_recorded == false
  and .operator_read_recorded == false
  and .operator_seen_recorded == false
  and .final_response_recorded == false
  and .completion_acknowledgement_recorded == false
  and .status_acknowledgement_recorded == false
  and .summary_acknowledgement_recorded == false
  and .briefing_acknowledgement_recorded == false
  and .readback_digest_acknowledgement_recorded == false
  and .dashboard_acknowledgement_recorded == false
  and .notification_acknowledgement_recorded == false
  and .channel_acknowledgement_delivered == false
  and .external_acknowledgement_sent == false
  and .telegram_acknowledgement_sent == false
  and .acknowledgement_acceptance_recorded == false
  and .operator_approval_from_acknowledgement_derived == false
  and .release_publication_authority_from_acknowledgement_derived == false
  and .activation_authority_from_acknowledgement_derived == false
  and .activation_command_from_acknowledgement_derived == false
  and .activation_from_acknowledgement_allowed == false
  and .live_execution_from_acknowledgement_allowed == false
  and .install_from_acknowledgement_executed == false
  and .service_restart_from_acknowledgement_performed == false
  and .active_binary_from_acknowledgement_mutated == false
  and .final_blocker_count == 56
  and .manual_operator_live_cutover_approval_required == true
  and .terminal_live_url_required == false
  and .long_soak_required == false
  and .public_ga_claim_allowed == false
  and .public_ga_claimed == false
  and .public_release_published == false
  and .rollback_execution_allowed == false
  and .next_migration_step == "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_final_acknowledgement_non_acceptance_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_terminal_decision_status_promotion_without_acknowledgement"
  and .side_effect_free == true
  and (.side_effects | to_entries | all(.value == false))
' >/dev/null

"$READBACK_GATE" >/dev/null

printf 'hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-final-acknowledgement-final-index-gate: PASS: Public GA operator identity/session intent consent evidence final acknowledgement final index is ready but blocked\n'
