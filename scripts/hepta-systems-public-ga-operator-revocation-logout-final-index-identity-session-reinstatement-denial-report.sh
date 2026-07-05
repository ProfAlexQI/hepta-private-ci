#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-revocation-logout-final-index-report.sh"
REINSTATEMENT_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial-gate.sh"
REINSTATEMENT_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session revocation logout final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$REINSTATEMENT_GATE" ]] || {
  echo "missing operator identity/session replay reinstatement denial gate: $REINSTATEMENT_GATE" >&2
  exit 1
}
[[ -f "$REINSTATEMENT_DOC" ]] || {
  echo "missing operator identity/session replay reinstatement denial doc: $REINSTATEMENT_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session reinstatement denial report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_revocation_logout_final_index"
  and .public_ga_operator_identity_session_revocation_logout_final_index_ready == true
  and .public_ga_operator_identity_session_revocation_logout_final_index_blocked == true
  and .operator_identity_revoked == false
  and .operator_session_logged_out == false
  and .revocation_logout_authority_derived == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
' <<<"$source_json" >/dev/null

reinstatement_static_mention_count="$(
  grep -Ec 'reinstatement|revocation|logout|session lifecycle|operator_identity|operator_session|session_' "$REINSTATEMENT_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson reinstatement_static_mention_count "$reinstatement_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_reinstatement_denial_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_revocation_logout_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_revocation_logout_final_index_ready: $source.public_ga_operator_identity_session_revocation_logout_final_index_ready,
    source_public_ga_operator_identity_session_revocation_logout_final_index_blocked: $source.public_ga_operator_identity_session_revocation_logout_final_index_blocked,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_identity_session_revocation_logout_final_index_attached: true,
    public_ga_operator_identity_session_reinstatement_denial_attachment_ready: true,
    public_ga_operator_identity_session_reinstatement_denial_attachment_blocked: true,
    operator_identity_session_revocation_logout_replay_reinstatement_denial_gate_present: true,
    operator_identity_session_revocation_logout_replay_reinstatement_denial_doc_present: true,
    operator_identity_session_reinstatement_static_mention_count: $reinstatement_static_mention_count,
    operator_identity_session_revocation_logout_replay_reinstatement_denial_gate_invoked: false,
    operator_identity_session_revocation_logout_denial_gate_invoked: false,
    operator_identity_session_replay_cross_binding_denial_gate_invoked: false,
    operator_identity_session_binding_denial_gate_invoked: false,
    operator_intent_consent_reconfirmation_gate_invoked: false,
    long_soak_required_by_source_reinstatement_gate: true,
    long_soak_started: false,
    public_ga_operator_packet_required_approval_static_count: $source.public_ga_operator_packet_required_approval_static_count,
    public_ga_operator_approval_packet_invoked: false,
    public_ga_operator_packet_sent: false,
    operator_approval_request_sent: false,
    operator_approval_recorded: false,
    operator_approval_accepted: false,
    operator_identity_accepted: false,
    operator_identity_hash_recorded: false,
    operator_identity_hash_replayed: false,
    operator_identity_revocation_requested: false,
    operator_identity_revoked: false,
    operator_identity_invalidation_recorded: false,
    operator_identity_reinstatement_requested: false,
    operator_identity_reinstated: false,
    operator_session_accepted: false,
    operator_session_recorded: false,
    operator_session_persisted: false,
    operator_session_bound: false,
    operator_session_replay_requested: false,
    operator_session_cross_binding_requested: false,
    operator_session_revocation_requested: false,
    operator_session_logout_requested: false,
    operator_session_revoked: false,
    operator_session_logged_out: false,
    operator_session_reinstatement_requested: false,
    operator_session_reinstated: false,
    session_binding_recorded: false,
    session_binding_persisted: false,
    session_token_recorded: false,
    session_token_replayed: false,
    session_revocation_recorded: false,
    session_logout_recorded: false,
    session_reinstatement_recorded: false,
    session_reinstatement_persisted: false,
    session_lifecycle_status_promoted: false,
    revocation_token_recorded: false,
    logout_nonce_recorded: false,
    reinstatement_token_recorded: false,
    reinstatement_nonce_recorded: false,
    device_session_reinstatement_recorded: false,
    revocation_logout_replay_requested: false,
    revocation_logout_replay_accepted: false,
    revocation_logout_replay_recorded: false,
    logout_replay_requested: false,
    logout_replay_accepted: false,
    logout_replay_recorded: false,
    revocation_logout_authority_derived: false,
    reinstatement_authority_derived: false,
    replay_acceptance_recorded: false,
    cross_session_binding_accepted: false,
    operator_acknowledgement_accepted: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 30,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_reinstatement_denial_readback_without_reinstatement",
    local_gate: "scripts/hepta-systems-public-ga-operator-revocation-logout-final-index-identity-session-reinstatement-denial-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_REVOCATION_LOGOUT_FINAL_INDEX_IDENTITY_SESSION_REINSTATEMENT_DENIAL_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_revocation_logout_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-revocation-logout-final-index-report.sh",
      operator_identity_session_revocation_logout_replay_reinstatement_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-denial-gate.sh",
      operator_identity_session_revocation_logout_replay_reinstatement_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      public_ga_operator_approval_packet_invoked: false,
      public_ga_operator_packet_sent: false,
      operator_approval_request_sent: false,
      operator_approval_recorded: false,
      operator_approval_accepted: false,
      operator_identity_session_revocation_logout_replay_reinstatement_denial_gate_invoked: false,
      operator_identity_session_revocation_logout_denial_gate_invoked: false,
      operator_identity_session_replay_cross_binding_denial_gate_invoked: false,
      operator_identity_session_binding_denial_gate_invoked: false,
      operator_intent_consent_reconfirmation_gate_invoked: false,
      operator_identity_accepted: false,
      operator_identity_hash_recorded: false,
      operator_identity_hash_replayed: false,
      operator_identity_revocation_requested: false,
      operator_identity_revoked: false,
      operator_identity_invalidation_recorded: false,
      operator_identity_reinstatement_requested: false,
      operator_identity_reinstated: false,
      operator_session_accepted: false,
      operator_session_recorded: false,
      operator_session_persisted: false,
      operator_session_bound: false,
      operator_session_replay_requested: false,
      operator_session_cross_binding_requested: false,
      operator_session_revocation_requested: false,
      operator_session_logout_requested: false,
      operator_session_revoked: false,
      operator_session_logged_out: false,
      operator_session_reinstatement_requested: false,
      operator_session_reinstated: false,
      session_binding_recorded: false,
      session_binding_persisted: false,
      session_token_recorded: false,
      session_token_replayed: false,
      session_revocation_recorded: false,
      session_logout_recorded: false,
      session_reinstatement_recorded: false,
      session_reinstatement_persisted: false,
      session_lifecycle_status_promoted: false,
      revocation_token_recorded: false,
      logout_nonce_recorded: false,
      reinstatement_token_recorded: false,
      reinstatement_nonce_recorded: false,
      device_session_reinstatement_recorded: false,
      revocation_logout_replay_requested: false,
      revocation_logout_replay_accepted: false,
      revocation_logout_replay_recorded: false,
      logout_replay_requested: false,
      logout_replay_accepted: false,
      logout_replay_recorded: false,
      revocation_logout_authority_derived: false,
      reinstatement_authority_derived: false,
      replay_acceptance_recorded: false,
      cross_session_binding_accepted: false,
      operator_acknowledgement_accepted: false,
      telegram_send_performed: false,
      external_send_performed: false,
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
