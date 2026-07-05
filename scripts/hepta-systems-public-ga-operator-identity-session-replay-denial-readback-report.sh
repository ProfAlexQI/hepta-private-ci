#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-session-final-index-identity-session-replay-denial-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session replay denial attachment report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session replay denial readback report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_replay_denial_attachment"
  and .public_ga_operator_identity_session_replay_denial_attachment_ready == true
  and .public_ga_operator_identity_session_replay_denial_attachment_blocked == true
  and .operator_identity_session_replay_cross_binding_denial_gate_invoked == false
  and .session_token_replayed == false
  and .cross_session_binding_accepted == false
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
    surface: "public_ga_operator_identity_session_replay_denial_readback",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_replay_denial_attachment_surface: $source.surface,
    source_public_ga_operator_identity_session_replay_denial_attachment_ready: $source.public_ga_operator_identity_session_replay_denial_attachment_ready,
    source_public_ga_operator_identity_session_replay_denial_attachment_blocked: $source.public_ga_operator_identity_session_replay_denial_attachment_blocked,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_identity_session_replay_denial_readback_ready: true,
    public_ga_operator_identity_session_replay_denial_readback_blocked: true,
    public_ga_operator_identity_session_replay_denial_attachment_attached: true,
    public_ga_operator_session_non_binding_final_index_attached: $source.public_ga_operator_session_non_binding_final_index_attached,
    readback_mode: "static_operator_identity_session_replay_denial_snapshot_only",
    readback_check_count: 32,
    operator_identity_session_replay_cross_binding_denial_gate_present: $source.operator_identity_session_replay_cross_binding_denial_gate_present,
    operator_identity_session_replay_cross_binding_denial_doc_present: $source.operator_identity_session_replay_cross_binding_denial_doc_present,
    operator_identity_session_replay_cross_binding_denial_gate_invoked: false,
    operator_identity_session_binding_denial_gate_invoked: false,
    operator_intent_consent_reconfirmation_gate_invoked: false,
    long_soak_required_by_source_replay_gate: $source.long_soak_required_by_source_replay_gate,
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
    operator_identity_replay_requested: false,
    operator_identity_cross_binding_requested: false,
    operator_session_accepted: false,
    operator_session_recorded: false,
    operator_session_persisted: false,
    operator_session_bound: false,
    operator_session_replay_requested: false,
    operator_session_cross_binding_requested: false,
    session_binding_recorded: false,
    session_binding_persisted: false,
    session_token_recorded: false,
    session_token_replayed: false,
    session_token_cross_bound: false,
    cross_session_binding_accepted: false,
    replay_acceptance_recorded: false,
    replay_authority_derived: false,
    cross_binding_authority_derived: false,
    operator_acknowledgement_accepted: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    readback_blocker_count: 26,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_replay_denial_final_index_without_replay_acceptance",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-replay-denial-readback-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_REPLAY_DENIAL_READBACK_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_replay_denial_attachment_report: "scripts/hepta-systems-public-ga-operator-session-final-index-identity-session-replay-denial-report.sh"
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
      operator_identity_session_replay_cross_binding_denial_gate_invoked: false,
      operator_identity_session_binding_denial_gate_invoked: false,
      operator_intent_consent_reconfirmation_gate_invoked: false,
      operator_identity_accepted: false,
      operator_identity_hash_recorded: false,
      operator_identity_hash_replayed: false,
      operator_identity_replay_requested: false,
      operator_identity_cross_binding_requested: false,
      operator_session_accepted: false,
      operator_session_recorded: false,
      operator_session_persisted: false,
      operator_session_bound: false,
      operator_session_replay_requested: false,
      operator_session_cross_binding_requested: false,
      session_binding_recorded: false,
      session_binding_persisted: false,
      session_token_recorded: false,
      session_token_replayed: false,
      session_token_cross_bound: false,
      cross_session_binding_accepted: false,
      replay_acceptance_recorded: false,
      replay_authority_derived: false,
      cross_binding_authority_derived: false,
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
