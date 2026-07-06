#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-session-non-binding-final-index-report.sh"
REPLAY_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding-denial-gate.sh"
REPLAY_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REPLAY_CROSS_BINDING_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator session non-binding final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$REPLAY_GATE" ]] || {
  echo "missing operator identity/session replay denial gate: $REPLAY_GATE" >&2
  exit 1
}
[[ -f "$REPLAY_DOC" ]] || {
  echo "missing operator identity/session replay denial doc: $REPLAY_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session replay denial report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_session_non_binding_final_index"
  and .public_ga_operator_session_non_binding_final_index_ready == true
  and .public_ga_operator_session_non_binding_final_index_blocked == true
  and .operator_identity_accepted == false
  and .operator_session_bound == false
  and .session_token_replayed == false
  and .cross_session_binding_accepted == false
  and .replay_acceptance_recorded == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
' <<<"$source_json" >/dev/null

replay_static_mention_count="$(
  grep -Ec 'replay|cross-binding|session token|identity hash|operator_identity|operator_session|session_binding' "$REPLAY_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson replay_static_mention_count "$replay_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_replay_denial_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_session_non_binding_final_index_surface: $source.surface,
    source_public_ga_operator_session_non_binding_final_index_ready: $source.public_ga_operator_session_non_binding_final_index_ready,
    source_public_ga_operator_session_non_binding_final_index_blocked: $source.public_ga_operator_session_non_binding_final_index_blocked,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_session_non_binding_final_index_attached: true,
    public_ga_operator_identity_session_replay_denial_attachment_ready: true,
    public_ga_operator_identity_session_replay_denial_attachment_blocked: true,
    operator_identity_session_replay_cross_binding_denial_gate_present: true,
    operator_identity_session_replay_cross_binding_denial_doc_present: true,
    operator_identity_session_replay_denial_static_mention_count: $replay_static_mention_count,
    operator_identity_session_replay_cross_binding_denial_gate_invoked: false,
    operator_identity_session_binding_denial_gate_invoked: false,
    operator_intent_consent_reconfirmation_gate_invoked: false,
    long_soak_required_by_source_replay_gate: true,
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
    attachment_blocker_count: 26,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_replay_denial_readback_without_replay_acceptance",
    local_gate: "scripts/hepta-systems-public-ga-operator-session-final-index-identity-session-replay-denial-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_SESSION_FINAL_INDEX_IDENTITY_SESSION_REPLAY_DENIAL_2026-06-21.md",
    source_files: {
      public_ga_operator_session_non_binding_final_index_report: "scripts/hepta-systems-public-ga-operator-session-non-binding-final-index-report.sh",
      operator_identity_session_replay_cross_binding_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-replay-cross-binding-denial-gate.sh",
      operator_identity_session_replay_cross_binding_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REPLAY_CROSS_BINDING_DENIAL_GATE.md"
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
