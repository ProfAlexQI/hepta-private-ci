#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-approval-non-acceptance-final-index-report.sh"
IDENTITY_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-denial-gate.sh"
IDENTITY_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_BINDING_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator approval non-acceptance final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$IDENTITY_GATE" ]] || {
  echo "missing operator identity/session binding denial gate: $IDENTITY_GATE" >&2
  exit 1
}
[[ -f "$IDENTITY_DOC" ]] || {
  echo "missing operator identity/session binding denial doc: $IDENTITY_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity non-acceptance attachment report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_approval_non_acceptance_final_index"
  and .public_ga_operator_approval_non_acceptance_final_index_ready == true
  and .public_ga_operator_approval_non_acceptance_final_index_blocked == true
  and .operator_approval_request_sent == false
  and .operator_approval_recorded == false
  and .operator_identity_accepted == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
' <<<"$source_json" >/dev/null

identity_static_mention_count="$(
  grep -Ec 'operator_identity|operator identity|operator_session|operator session|session_binding|identity/session' "$IDENTITY_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson identity_static_mention_count "$identity_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_non_acceptance_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_approval_non_acceptance_final_index_surface: $source.surface,
    source_public_ga_operator_approval_non_acceptance_final_index_ready: $source.public_ga_operator_approval_non_acceptance_final_index_ready,
    source_public_ga_operator_approval_non_acceptance_final_index_blocked: $source.public_ga_operator_approval_non_acceptance_final_index_blocked,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    public_ga_operator_approval_non_acceptance_final_index_attached: true,
    public_ga_operator_identity_non_acceptance_attachment_ready: true,
    public_ga_operator_identity_non_acceptance_attachment_blocked: true,
    operator_identity_session_binding_denial_gate_present: true,
    operator_identity_session_binding_denial_doc_present: true,
    operator_identity_session_static_mention_count: $identity_static_mention_count,
    operator_identity_session_binding_denial_gate_invoked: false,
    operator_identity_session_replay_gate_invoked: false,
    operator_intent_consent_reconfirmation_gate_invoked: false,
    long_soak_required_by_source_identity_gate: true,
    long_soak_started: false,
    public_ga_operator_packet_required_approval_static_count: $source.public_ga_operator_packet_required_approval_static_count,
    public_ga_operator_approval_packet_invoked: false,
    public_ga_operator_packet_sent: false,
    public_ga_operator_packet_recorded: false,
    public_ga_operator_packet_accepted: false,
    operator_approval_request_sent: false,
    operator_approval_recorded: false,
    operator_approval_accepted: false,
    operator_identity_binding_requested: false,
    operator_identity_binding_recorded: false,
    operator_identity_accepted: false,
    operator_identity_hash_recorded: false,
    operator_session_accepted: false,
    operator_session_recorded: false,
    operator_session_persisted: false,
    session_binding_recorded: false,
    session_binding_persisted: false,
    operator_acknowledgement_accepted: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 22,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_non_acceptance_readback_without_identity_acceptance",
    local_gate: "scripts/hepta-systems-public-ga-operator-approval-final-index-operator-identity-non-acceptance-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_APPROVAL_FINAL_INDEX_OPERATOR_IDENTITY_NON_ACCEPTANCE_2026-06-21.md",
    source_files: {
      public_ga_operator_approval_non_acceptance_final_index_report: "scripts/hepta-systems-public-ga-operator-approval-non-acceptance-final-index-report.sh",
      operator_identity_session_binding_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-binding-denial-gate.sh",
      operator_identity_session_binding_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_BINDING_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      public_ga_operator_approval_packet_invoked: false,
      public_ga_operator_packet_sent: false,
      public_ga_operator_packet_recorded: false,
      public_ga_operator_packet_accepted: false,
      operator_approval_request_sent: false,
      operator_approval_recorded: false,
      operator_approval_accepted: false,
      operator_identity_session_binding_denial_gate_invoked: false,
      operator_identity_session_replay_gate_invoked: false,
      operator_intent_consent_reconfirmation_gate_invoked: false,
      operator_identity_binding_requested: false,
      operator_identity_binding_recorded: false,
      operator_identity_accepted: false,
      operator_identity_hash_recorded: false,
      operator_session_accepted: false,
      operator_session_recorded: false,
      operator_session_persisted: false,
      session_binding_recorded: false,
      session_binding_persisted: false,
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
