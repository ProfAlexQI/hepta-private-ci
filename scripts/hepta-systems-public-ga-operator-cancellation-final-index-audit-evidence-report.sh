#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-reinstatement-cancellation-final-index-report.sh"
AUDIT_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-audit-evidence-denial-gate.sh"
AUDIT_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_AUDIT_EVIDENCE_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session cancellation final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$AUDIT_GATE" ]] || {
  echo "missing operator identity/session audit evidence denial gate: $AUDIT_GATE" >&2
  exit 1
}
[[ -f "$AUDIT_DOC" ]] || {
  echo "missing operator identity/session audit evidence denial doc: $AUDIT_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session audit evidence report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_reinstatement_cancellation_final_index"
  and .public_ga_operator_identity_session_reinstatement_cancellation_final_index_ready == true
  and .public_ga_operator_identity_session_reinstatement_cancellation_final_index_blocked == true
  and .cancellation_recorded == false
  and .supersession_recorded == false
  and .cancellation_supersession_authority_derived == false
  and .source_canonical_governance_tool_execution_closure_backfeed_ready == true
  and .source_canonical_governance_tool_execution_closure_backfeed_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_category_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_ready_count == 4
  and .source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count == 17
  and .source_canonical_governance_tool_execution_closure_backfeed_categorization_ready == true
' <<<"$source_json" >/dev/null

audit_static_mention_count="$(
  grep -Ec 'audit|evidence|hash|merkle|attestation|witness|notary|ledger|readback|receipt' "$AUDIT_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson audit_static_mention_count "$audit_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_reinstatement_audit_evidence_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_reinstatement_cancellation_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_reinstatement_cancellation_final_index_ready: $source.public_ga_operator_identity_session_reinstatement_cancellation_final_index_ready,
    source_public_ga_operator_identity_session_reinstatement_cancellation_final_index_blocked: $source.public_ga_operator_identity_session_reinstatement_cancellation_final_index_blocked,
    public_ga_operator_identity_session_reinstatement_cancellation_final_index_attached: true,
    public_ga_operator_identity_session_reinstatement_audit_evidence_attachment_ready: true,
    public_ga_operator_identity_session_reinstatement_audit_evidence_attachment_blocked: true,
    source_canonical_governance_tool_execution_closure_backfeed_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_ready,
    source_canonical_governance_tool_execution_closure_backfeed_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_ready_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_ready_count,
    source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count: $source.source_canonical_governance_tool_execution_closure_backfeed_category_blocker_count,
    source_canonical_governance_tool_execution_closure_backfeed_categorization_ready: $source.source_canonical_governance_tool_execution_closure_backfeed_categorization_ready,
    source_canonical_governance_tool_execution_closure_backfeed_categories: $source.source_canonical_governance_tool_execution_closure_backfeed_categories,
    operator_identity_session_reinstatement_audit_evidence_denial_gate_present: true,
    operator_identity_session_reinstatement_audit_evidence_denial_doc_present: true,
    operator_identity_session_reinstatement_audit_evidence_static_mention_count: $audit_static_mention_count,
    operator_identity_session_reinstatement_audit_evidence_denial_gate_invoked: false,
    operator_identity_session_reinstatement_cancellation_supersession_denial_gate_invoked: false,
    operator_identity_session_reinstatement_ordering_monotonicity_denial_gate_invoked: false,
    operator_identity_session_revocation_logout_replay_reinstatement_denial_gate_invoked: false,
    long_soak_required_by_source_audit_gate: true,
    long_soak_started: false,
    public_ga_operator_packet_required_approval_static_count: $source.public_ga_operator_packet_required_approval_static_count,
    public_ga_operator_approval_packet_invoked: false,
    public_ga_operator_packet_sent: false,
    operator_approval_request_sent: false,
    operator_approval_recorded: false,
    operator_approval_accepted: false,
    cancellation_recorded: false,
    supersession_recorded: false,
    cancellation_supersession_authority_derived: false,
    audit_evidence_requested: false,
    audit_trail_append_requested: false,
    audit_trail_accepted: false,
    audit_trail_recorded: false,
    audit_trail_persisted: false,
    audit_trail_materialized: false,
    audit_trail_filesystem_written: false,
    immutable_evidence_requested: false,
    immutable_evidence_accepted: false,
    immutable_evidence_recorded: false,
    immutable_evidence_persisted: false,
    immutable_evidence_materialized: false,
    immutable_evidence_filesystem_written: false,
    hash_chain_requested: false,
    hash_chain_recorded: false,
    hash_chain_persisted: false,
    merkle_root_requested: false,
    merkle_root_recorded: false,
    merkle_root_persisted: false,
    attestation_requested: false,
    attestation_recorded: false,
    attestation_persisted: false,
    witness_requested: false,
    witness_recorded: false,
    witness_persisted: false,
    notary_requested: false,
    notary_recorded: false,
    notary_persisted: false,
    ledger_evidence_requested: false,
    ledger_evidence_recorded: false,
    ledger_evidence_persisted: false,
    index_evidence_recorded: false,
    delivery_evidence_recorded: false,
    readback_evidence_recorded: false,
    audit_evidence_acceptance_recorded: false,
    result_receipt_from_audit_evidence_recorded: false,
    result_receipt_from_audit_evidence_persisted: false,
    operator_approval_from_audit_evidence_derived: false,
    release_publication_authority_from_audit_evidence_derived: false,
    activation_authority_from_audit_evidence_derived: false,
    audit_evidence_authority_derived: false,
    download_link_from_audit_evidence_rendered: false,
    install_command_from_audit_evidence_rendered: false,
    install_from_audit_evidence_executed: false,
    active_binary_from_audit_evidence_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 36,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_ga_claimed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_reinstatement_audit_evidence_readback_without_evidence",
    local_gate: "scripts/hepta-systems-public-ga-operator-cancellation-final-index-audit-evidence-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_CANCELLATION_FINAL_INDEX_AUDIT_EVIDENCE_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_reinstatement_cancellation_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-reinstatement-cancellation-final-index-report.sh",
      operator_identity_session_reinstatement_audit_evidence_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-audit-evidence-denial-gate.sh",
      operator_identity_session_reinstatement_audit_evidence_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_AUDIT_EVIDENCE_DENIAL_GATE.md"
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
      operator_identity_session_reinstatement_audit_evidence_denial_gate_invoked: false,
      operator_identity_session_reinstatement_cancellation_supersession_denial_gate_invoked: false,
      operator_identity_session_reinstatement_ordering_monotonicity_denial_gate_invoked: false,
      operator_identity_session_revocation_logout_replay_reinstatement_denial_gate_invoked: false,
      cancellation_recorded: false,
      supersession_recorded: false,
      cancellation_supersession_authority_derived: false,
      audit_trail_recorded: false,
      audit_trail_persisted: false,
      audit_trail_materialized: false,
      audit_trail_filesystem_written: false,
      immutable_evidence_recorded: false,
      immutable_evidence_persisted: false,
      hash_chain_recorded: false,
      hash_chain_persisted: false,
      merkle_root_recorded: false,
      merkle_root_persisted: false,
      attestation_recorded: false,
      attestation_persisted: false,
      witness_recorded: false,
      witness_persisted: false,
      notary_recorded: false,
      notary_persisted: false,
      ledger_evidence_recorded: false,
      ledger_evidence_persisted: false,
      index_evidence_recorded: false,
      delivery_evidence_recorded: false,
      readback_evidence_recorded: false,
      audit_evidence_acceptance_recorded: false,
      result_receipt_from_audit_evidence_recorded: false,
      result_receipt_from_audit_evidence_persisted: false,
      operator_approval_from_audit_evidence_derived: false,
      release_publication_authority_from_audit_evidence_derived: false,
      activation_authority_from_audit_evidence_derived: false,
      audit_evidence_authority_derived: false,
      download_link_from_audit_evidence_rendered: false,
      install_command_from_audit_evidence_rendered: false,
      install_from_audit_evidence_executed: false,
      active_binary_from_audit_evidence_mutated: false,
      memory_store_write_performed: false,
      live_kg_write_performed: false,
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
