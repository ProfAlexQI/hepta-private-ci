#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-report.sh"
AUDIT_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-audit-evidence-denial-gate.sh"
AUDIT_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_AUDIT_EVIDENCE_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$AUDIT_GATE" ]] || {
  echo "missing operator identity/session intent consent evidence artifact signing audit/evidence denial gate: $AUDIT_GATE" >&2
  exit 1
}
[[ -f "$AUDIT_DOC" ]] || {
  echo "missing operator identity/session intent consent evidence artifact signing audit/evidence denial doc: $AUDIT_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence artifact signing audit/evidence report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_blocked == true
  and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded == false
  and .artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded == false
  and .artifact_distribution_signing_notarization_receipt_tombstone_recorded == false
  and .operator_approval_from_signing_receipt_cancellation_derived == false
  and .public_ga_claimed == false
' <<<"$source_json" >/dev/null

audit_static_mention_count="$(
  grep -Eci 'audit|evidence|immutable|hash|merkle|attestation|witness|notary|ledger|index|delivery|query|export|observability|readback|status|receipt|authority|install|restart|active-binary|telegram|external|credential|secret|provider|model|live' "$AUDIT_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson audit_static_mention_count "$audit_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_attachment_blocked: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_denial_gate_present: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_denial_doc_present: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_static_mention_count: $audit_static_mention_count,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_artifact_signing_audit_evidence_gate: true,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    artifact_distribution_signing_notarization_receipt_audit_evidence_allowed: false,
    artifact_distribution_signing_notarization_receipt_audit_evidence_accepted: false,
    artifact_distribution_signing_notarization_receipt_audit_evidence_recorded: false,
    artifact_distribution_signing_notarization_receipt_audit_evidence_persisted: false,
    artifact_distribution_signing_notarization_receipt_audit_evidence_materialized: false,
    artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written: false,
    artifact_distribution_signing_notarization_receipt_audit_trail_recorded: false,
    artifact_distribution_signing_notarization_receipt_audit_trail_persisted: false,
    artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded: false,
    artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted: false,
    artifact_distribution_signing_notarization_receipt_hash_chain_recorded: false,
    artifact_distribution_signing_notarization_receipt_merkle_root_recorded: false,
    artifact_distribution_signing_notarization_receipt_attestation_recorded: false,
    artifact_distribution_signing_notarization_receipt_witness_recorded: false,
    artifact_distribution_signing_notarization_receipt_notary_recorded: false,
    artifact_distribution_signing_notarization_receipt_ledger_recorded: false,
    artifact_distribution_signing_notarization_receipt_ledger_persisted: false,
    artifact_distribution_signing_notarization_receipt_index_recorded: false,
    artifact_distribution_signing_notarization_receipt_index_persisted: false,
    artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded: false,
    artifact_distribution_signing_notarization_receipt_delivery_evidence_delivered: false,
    artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded: false,
    artifact_distribution_signing_notarization_receipt_observability_evidence_recorded: false,
    artifact_distribution_signing_notarization_receipt_readback_evidence_recorded: false,
    artifact_distribution_signing_notarization_receipt_status_evidence_recorded: false,
    artifact_distribution_signing_notarization_receipt_hash_status_evidence_recorded: false,
    artifact_signing_cancellation_audit_trail_recorded: false,
    package_signing_supersession_immutable_evidence_recorded: false,
    signature_manifest_withdrawal_hash_chain_recorded: false,
    notarization_submission_cancellation_attestation_recorded: false,
    notarization_ticket_supersession_witness_recorded: false,
    stapling_tombstone_ledger_index_recorded: false,
    installer_replacement_evidence_materialized: false,
    provenance_latest_replacement_immutable_evidence_recorded: false,
    sbom_supersession_evidence_exported: false,
    release_asset_cancelled_query_evidence_recorded: false,
    cdn_superseded_observability_evidence_recorded: false,
    package_registry_replacement_status_evidence_recorded: false,
    dashboard_endpoint_tombstone_hash_status_evidence_recorded: false,
    external_audit_evidence_delivered: false,
    telegram_audit_evidence_delivered: false,
    operator_acceptance_from_signing_receipt_audit_evidence_recorded: false,
    operator_approval_from_signing_receipt_audit_evidence_derived: false,
    release_publication_authority_from_signing_receipt_audit_evidence_derived: false,
    activation_authority_from_signing_receipt_audit_evidence_derived: false,
    artifact_signing_audit_evidence_authority_derived: false,
    download_link_from_signing_receipt_audit_evidence_rendered: false,
    install_command_from_signing_receipt_audit_evidence_rendered: false,
    install_from_signing_receipt_audit_evidence_executed: false,
    service_restart_from_signing_receipt_audit_evidence_performed: false,
    active_binary_from_signing_receipt_audit_evidence_mutated: false,
    memory_store_write_performed: false,
    live_kg_write_performed: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    telegram_send_performed: false,
    external_send_performed: false,
    public_ga_readiness_script_invoked: false,
    public_claim_non_promotion_denial_gate_invoked: false,
    terminal_live_gates_invoked: false,
    attachment_blocker_count: 76,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_readback_without_cancellation",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-artifact-signing-audit-evidence-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_CANCELLATION_SUPERSESSION_FINAL_INDEX_ARTIFACT_SIGNING_AUDIT_EVIDENCE_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_cancellation_supersession_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-cancellation-supersession-final-index-report.sh",
      operator_identity_session_intent_consent_evidence_artifact_signing_audit_evidence_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-audit-evidence-denial-gate.sh",
      operator_identity_session_intent_consent_evidence_artifact_signing_audit_evidence_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_AUDIT_EVIDENCE_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      operator_identity_session_operator_intent_consent_evidence_artifact_signing_audit_evidence_denial_gate_invoked: false,
      operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_invoked: false,
      artifact_distribution_signing_notarization_receipt_audit_evidence_recorded: false,
      artifact_distribution_signing_notarization_receipt_audit_evidence_persisted: false,
      artifact_distribution_signing_notarization_receipt_audit_evidence_materialized: false,
      artifact_distribution_signing_notarization_receipt_audit_evidence_filesystem_written: false,
      artifact_distribution_signing_notarization_receipt_audit_trail_recorded: false,
      artifact_distribution_signing_notarization_receipt_audit_trail_persisted: false,
      artifact_distribution_signing_notarization_receipt_immutable_evidence_recorded: false,
      artifact_distribution_signing_notarization_receipt_immutable_evidence_persisted: false,
      artifact_distribution_signing_notarization_receipt_hash_chain_recorded: false,
      artifact_distribution_signing_notarization_receipt_merkle_root_recorded: false,
      artifact_distribution_signing_notarization_receipt_attestation_recorded: false,
      artifact_distribution_signing_notarization_receipt_witness_recorded: false,
      artifact_distribution_signing_notarization_receipt_notary_recorded: false,
      artifact_distribution_signing_notarization_receipt_ledger_recorded: false,
      artifact_distribution_signing_notarization_receipt_index_recorded: false,
      artifact_distribution_signing_notarization_receipt_delivery_evidence_recorded: false,
      artifact_distribution_signing_notarization_receipt_query_export_evidence_recorded: false,
      artifact_distribution_signing_notarization_receipt_observability_evidence_recorded: false,
      artifact_distribution_signing_notarization_receipt_readback_evidence_recorded: false,
      artifact_distribution_signing_notarization_receipt_status_evidence_recorded: false,
      artifact_distribution_signing_notarization_receipt_hash_status_evidence_recorded: false,
      external_audit_evidence_delivered: false,
      telegram_audit_evidence_delivered: false,
      operator_approval_from_signing_receipt_audit_evidence_derived: false,
      release_publication_authority_from_signing_receipt_audit_evidence_derived: false,
      activation_authority_from_signing_receipt_audit_evidence_derived: false,
      download_link_from_signing_receipt_audit_evidence_rendered: false,
      install_command_from_signing_receipt_audit_evidence_rendered: false,
      install_from_signing_receipt_audit_evidence_executed: false,
      service_restart_from_signing_receipt_audit_evidence_performed: false,
      active_binary_from_signing_receipt_audit_evidence_mutated: false,
      memory_store_write_performed: false,
      live_kg_write_performed: false,
      provider_invoked: false,
      model_invoked: false,
      credential_read: false,
      secret_file_read: false,
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
      external_network_read: false,
      release_artifact_written: false,
      public_artifact_written: false,
      filesystem_written: false
    }
  }'
