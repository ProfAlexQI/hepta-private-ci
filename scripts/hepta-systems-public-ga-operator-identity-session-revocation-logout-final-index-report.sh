#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-revocation-logout-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session revocation logout readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session revocation logout final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_revocation_logout_readback"
  and .public_ga_operator_identity_session_revocation_logout_readback_ready == true
  and .public_ga_operator_identity_session_revocation_logout_readback_blocked == true
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

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_revocation_logout_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_revocation_logout_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_revocation_logout_readback_ready: $source.public_ga_operator_identity_session_revocation_logout_readback_ready,
    source_public_ga_operator_identity_session_revocation_logout_readback_blocked: $source.public_ga_operator_identity_session_revocation_logout_readback_blocked,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_identity_session_revocation_logout_final_index_ready: true,
    public_ga_operator_identity_session_revocation_logout_final_index_blocked: true,
    public_ga_operator_identity_session_revocation_logout_readback_attached: true,
    public_ga_operator_identity_session_replay_denial_final_index_attached: $source.public_ga_operator_identity_session_replay_denial_final_index_attached,
    operator_identity_session_revocation_logout_denial_gate_present: $source.operator_identity_session_revocation_logout_denial_gate_present,
    operator_identity_session_revocation_logout_denial_doc_present: $source.operator_identity_session_revocation_logout_denial_doc_present,
    operator_identity_session_revocation_logout_denial_gate_invoked: false,
    operator_identity_session_replay_cross_binding_denial_gate_invoked: false,
    operator_identity_session_binding_denial_gate_invoked: false,
    operator_intent_consent_reconfirmation_gate_invoked: false,
    long_soak_required_by_source_revocation_gate: $source.long_soak_required_by_source_revocation_gate,
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
    operator_session_accepted: false,
    operator_session_recorded: false,
    operator_session_persisted: false,
    operator_session_bound: false,
    operator_session_revocation_requested: false,
    operator_session_logout_requested: false,
    operator_session_revoked: false,
    operator_session_logged_out: false,
    session_binding_recorded: false,
    session_binding_persisted: false,
    session_token_recorded: false,
    session_token_replayed: false,
    session_revocation_recorded: false,
    session_logout_recorded: false,
    session_lifecycle_status_promoted: false,
    revocation_token_recorded: false,
    logout_nonce_recorded: false,
    revocation_logout_authority_derived: false,
    replay_acceptance_recorded: false,
    cross_session_binding_accepted: false,
    operator_acknowledgement_accepted: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    final_blocker_count: 28,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_revocation_logout_final_index_to_public_ga_operator_identity_session_reinstatement_denial_without_revocation_or_logout",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-revocation-logout-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_revocation_logout_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-revocation-logout-readback-report.sh"
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
      operator_session_accepted: false,
      operator_session_recorded: false,
      operator_session_persisted: false,
      operator_session_bound: false,
      operator_session_revocation_requested: false,
      operator_session_logout_requested: false,
      operator_session_revoked: false,
      operator_session_logged_out: false,
      session_binding_recorded: false,
      session_binding_persisted: false,
      session_token_recorded: false,
      session_token_replayed: false,
      session_revocation_recorded: false,
      session_logout_recorded: false,
      session_lifecycle_status_promoted: false,
      revocation_token_recorded: false,
      logout_nonce_recorded: false,
      revocation_logout_authority_derived: false,
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
