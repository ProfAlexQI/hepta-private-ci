#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-reinstatement-ordering-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session reinstatement ordering readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session reinstatement ordering final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_reinstatement_ordering_readback"
  and .public_ga_operator_identity_session_reinstatement_ordering_readback_ready == true
  and .public_ga_operator_identity_session_reinstatement_ordering_readback_blocked == true
  and .ordering_recorded == false
  and .sequence_cursor_recorded == false
  and .ordering_authority_derived == false
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
    surface: "public_ga_operator_identity_session_reinstatement_ordering_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_reinstatement_ordering_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_reinstatement_ordering_readback_ready: $source.public_ga_operator_identity_session_reinstatement_ordering_readback_ready,
    source_public_ga_operator_identity_session_reinstatement_ordering_readback_blocked: $source.public_ga_operator_identity_session_reinstatement_ordering_readback_blocked,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_identity_session_reinstatement_ordering_final_index_ready: true,
    public_ga_operator_identity_session_reinstatement_ordering_final_index_blocked: true,
    public_ga_operator_identity_session_reinstatement_ordering_readback_attached: true,
    public_ga_operator_identity_session_reinstatement_denial_final_index_attached: $source.public_ga_operator_identity_session_reinstatement_denial_final_index_attached,
    operator_identity_session_reinstatement_ordering_monotonicity_denial_gate_present: $source.operator_identity_session_reinstatement_ordering_monotonicity_denial_gate_present,
    operator_identity_session_reinstatement_ordering_monotonicity_denial_doc_present: $source.operator_identity_session_reinstatement_ordering_monotonicity_denial_doc_present,
    operator_identity_session_reinstatement_ordering_monotonicity_denial_gate_invoked: false,
    operator_identity_session_revocation_logout_replay_reinstatement_denial_gate_invoked: false,
    operator_identity_session_revocation_logout_denial_gate_invoked: false,
    operator_identity_session_replay_cross_binding_denial_gate_invoked: false,
    operator_identity_session_binding_denial_gate_invoked: false,
    operator_intent_consent_reconfirmation_gate_invoked: false,
    long_soak_required_by_source_ordering_gate: $source.long_soak_required_by_source_ordering_gate,
    long_soak_started: false,
    public_ga_operator_packet_required_approval_static_count: $source.public_ga_operator_packet_required_approval_static_count,
    public_ga_operator_approval_packet_invoked: false,
    public_ga_operator_packet_sent: false,
    operator_approval_request_sent: false,
    operator_approval_recorded: false,
    operator_approval_accepted: false,
    operator_identity_reinstatement_requested: false,
    operator_identity_reinstated: false,
    operator_session_reinstatement_requested: false,
    operator_session_reinstated: false,
    session_reinstatement_recorded: false,
    session_reinstatement_persisted: false,
    session_lifecycle_status_promoted: false,
    reinstatement_token_recorded: false,
    reinstatement_nonce_recorded: false,
    revocation_logout_replay_accepted: false,
    logout_replay_accepted: false,
    reinstatement_authority_derived: false,
    ordering_recorded: false,
    ordering_persisted: false,
    ordering_materialized: false,
    ordering_filesystem_written: false,
    sequence_cursor_recorded: false,
    sequence_cursor_persisted: false,
    monotonicity_state_recorded: false,
    monotonicity_state_persisted: false,
    monotonicity_state_materialized: false,
    timestamp_rollback_accepted: false,
    epoch_rollback_accepted: false,
    same_sequence_different_nonce_accepted: false,
    late_arrival_accepted: false,
    future_sequence_gap_accepted: false,
    latest_wins_accepted: false,
    monotonic_cursor_accepted: false,
    ordered_query_accepted: false,
    ordered_export_accepted: false,
    ordered_observability_accepted: false,
    ordered_delivery_accepted: false,
    completion_order_recorded: false,
    ordering_authority_derived: false,
    replay_acceptance_recorded: false,
    cross_session_binding_accepted: false,
    operator_acknowledgement_accepted: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    final_blocker_count: 32,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_reinstatement_ordering_final_index_to_public_ga_operator_identity_session_reinstatement_cancellation_without_ordering",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-reinstatement-ordering-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_REINSTATEMENT_ORDERING_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_reinstatement_ordering_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-reinstatement-ordering-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
