#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-terminal-decision-status-final-index-report.sh"
INTENT_CONSENT_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-denial-gate.sh"
INTENT_CONSENT_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_RECONFIRMATION_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session terminal decision/status final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$INTENT_CONSENT_GATE" ]] || {
  echo "missing operator identity/session intent/consent reconfirmation denial gate: $INTENT_CONSENT_GATE" >&2
  exit 1
}
[[ -f "$INTENT_CONSENT_DOC" ]] || {
  echo "missing operator identity/session intent/consent reconfirmation denial doc: $INTENT_CONSENT_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_terminal_decision_status_promotion_final_index"
  and .public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_ready == true
  and .public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_blocked == true
  and .terminal_decision_recorded == false
  and .status_promotion_recorded == false
  and .operator_approval_from_terminal_status_derived == false
  and .activation_authority_from_terminal_status_derived == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
' <<<"$source_json" >/dev/null

intent_consent_static_mention_count="$(
  grep -Ec 'intent|consent|reconfirmation|signature|token|nonce|approval|authority|install|restart|active-binary|telegram|external|live' "$INTENT_CONSENT_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson intent_consent_static_mention_count "$intent_consent_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_reconfirmation_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_ready: $source.public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_ready,
    source_public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_blocked: $source.public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_blocked,
    public_ga_operator_identity_session_terminal_decision_status_promotion_final_index_attached: true,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_identity_session_operator_intent_consent_reconfirmation_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_reconfirmation_attachment_blocked: true,
    operator_identity_session_operator_intent_consent_reconfirmation_denial_gate_present: true,
    operator_identity_session_operator_intent_consent_reconfirmation_denial_doc_present: true,
    operator_identity_session_operator_intent_consent_static_mention_count: $intent_consent_static_mention_count,
    operator_identity_session_operator_intent_consent_reconfirmation_denial_gate_invoked: false,
    operator_identity_session_terminal_decision_status_promotion_denial_gate_invoked: false,
    operator_identity_session_final_acknowledgement_non_acceptance_gate_invoked: false,
    long_soak_required_by_source_intent_consent_gate: true,
    long_soak_started: false,
    terminal_decision_recorded: false,
    status_promotion_recorded: false,
    public_status_exposed: false,
    public_ga_status_exposed: false,
    operator_intent_reconfirmation_requested: false,
    operator_consent_reconfirmation_requested: false,
    operator_intent_reconfirmation_allowed: false,
    operator_consent_reconfirmation_allowed: false,
    operator_intent_reconfirmed: false,
    operator_consent_reconfirmed: false,
    operator_intent_recorded: false,
    operator_intent_persisted: false,
    operator_consent_recorded: false,
    operator_consent_persisted: false,
    consent_reconfirmation_recorded: false,
    consent_reconfirmation_persisted: false,
    identity_signature_recorded: false,
    session_consent_token_recorded: false,
    revocation_replay_intent_timestamp_recorded: false,
    device_session_consent_nonce_recorded: false,
    logout_replay_consent_refresh_recorded: false,
    explicit_intent_status_promoted: false,
    explicit_consent_status_promoted: false,
    consent_summary_recorded: false,
    operator_approval_from_intent_consent_derived: false,
    acceptance_from_intent_consent_recorded: false,
    terminal_decision_from_intent_consent_recorded: false,
    terminal_status_from_intent_consent_recorded: false,
    release_publication_authority_from_intent_consent_derived: false,
    activation_authority_from_intent_consent_derived: false,
    download_link_from_intent_consent_rendered: false,
    install_command_from_intent_consent_rendered: false,
    install_from_intent_consent_executed: false,
    service_restart_from_intent_consent_performed: false,
    launchd_from_intent_consent_mutated: false,
    active_binary_from_intent_consent_mutated: false,
    result_receipt_from_intent_consent_recorded: false,
    result_receipt_from_intent_consent_persisted: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 48,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_reconfirmation_readback_without_status_promotion",
    local_gate: "scripts/hepta-systems-public-ga-operator-terminal-decision-status-final-index-intent-consent-reconfirmation-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_TERMINAL_DECISION_STATUS_FINAL_INDEX_INTENT_CONSENT_RECONFIRMATION_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_terminal_decision_status_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-terminal-decision-status-final-index-report.sh",
      operator_identity_session_operator_intent_consent_reconfirmation_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-reconfirmation-denial-gate.sh",
      operator_identity_session_operator_intent_consent_reconfirmation_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_RECONFIRMATION_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      operator_identity_session_operator_intent_consent_reconfirmation_denial_gate_invoked: false,
      operator_identity_session_terminal_decision_status_promotion_denial_gate_invoked: false,
      operator_intent_recorded: false,
      operator_intent_persisted: false,
      operator_consent_recorded: false,
      operator_consent_persisted: false,
      consent_reconfirmation_recorded: false,
      identity_signature_recorded: false,
      session_consent_token_recorded: false,
      consent_nonce_recorded: false,
      consent_refresh_recorded: false,
      operator_approval_from_intent_consent_derived: false,
      acceptance_from_intent_consent_recorded: false,
      terminal_decision_from_intent_consent_recorded: false,
      terminal_status_from_intent_consent_recorded: false,
      release_publication_authority_from_intent_consent_derived: false,
      activation_authority_from_intent_consent_derived: false,
      download_link_from_intent_consent_rendered: false,
      install_command_from_intent_consent_rendered: false,
      install_from_intent_consent_executed: false,
      service_restart_from_intent_consent_performed: false,
      active_binary_from_intent_consent_mutated: false,
      memory_store_write_performed: false,
      live_kg_write_performed: false,
      external_send_performed: false,
      telegram_send_performed: false,
      long_soak_started: false,
      terminal_live_gate_invoked: false,
      terminal_live_url_contacted: false,
      public_ga_readiness_script_invoked: false,
      public_claim_non_promotion_denial_gate_invoked: false,
      public_ga_claim_recorded: false,
      public_ga_promoted: false,
      public_release_published: false,
      rollback_executed: false,
      external_network_read: false
    }
  }'
