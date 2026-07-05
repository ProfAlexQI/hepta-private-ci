#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-receipt-no-persistence-final-index-report.sh"
REPLAY_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-replay-idempotency-denial-gate.sh"
REPLAY_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_REPLAY_IDEMPOTENCY_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence artifact signing receipt no-persistence final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$REPLAY_GATE" ]] || {
  echo "missing operator identity/session intent consent evidence artifact signing replay/idempotency denial gate: $REPLAY_GATE" >&2
  exit 1
}
[[ -f "$REPLAY_DOC" ]] || {
  echo "missing operator identity/session intent consent evidence artifact signing replay/idempotency denial doc: $REPLAY_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence artifact signing replay/idempotency report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_blocked == true
  and .artifact_distribution_signing_notarization_result_receipt_surface_recorded == false
  and .artifact_signing_receipt_accepted == false
  and .notarization_submission_receipt_persisted == false
  and .operator_approval_from_signing_receipt_derived == false
  and .public_ga_claimed == false
' <<<"$source_json" >/dev/null

replay_static_mention_count="$(
  grep -Eci 'replay|duplicate|idempotency|nonce|cross.scope|reuse|status.upgrade|completed|ack|ledger|index|delivery|export|query|observability|hash|rebind|artifact.signing|package.signing|signature.manifest|notarization|stapling|installer|provenance|sbom|release.asset|bundle|cdn|update.feed|registry|dashboard|endpoint|telegram|external|authority|download|install|restart|active-binary|live' "$REPLAY_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson replay_static_mention_count "$replay_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_attachment_blocked: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_denial_gate_present: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_denial_doc_present: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_static_mention_count: $replay_static_mention_count,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_artifact_signing_replay_idempotency_gate: true,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    artifact_distribution_signing_notarization_receipt_replay_allowed: false,
    artifact_distribution_signing_notarization_receipt_replay_accepted: false,
    artifact_distribution_signing_notarization_receipt_replay_recorded: false,
    artifact_distribution_signing_notarization_receipt_replay_persisted: false,
    artifact_distribution_signing_notarization_receipt_replay_performed: false,
    artifact_distribution_signing_notarization_receipt_duplicate_accepted: false,
    artifact_distribution_signing_notarization_receipt_duplicate_recorded: false,
    artifact_distribution_signing_notarization_receipt_duplicate_persisted: false,
    artifact_distribution_signing_notarization_receipt_idempotency_key_accepted: false,
    artifact_distribution_signing_notarization_receipt_idempotency_key_recorded: false,
    artifact_distribution_signing_notarization_receipt_idempotency_state_recorded: false,
    artifact_distribution_signing_notarization_receipt_idempotency_state_persisted: false,
    artifact_distribution_signing_notarization_receipt_idempotency_state_materialized: false,
    artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written: false,
    artifact_distribution_signing_notarization_receipt_replay_nonce_accepted: false,
    artifact_distribution_signing_notarization_receipt_replay_nonce_recorded: false,
    artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted: false,
    artifact_distribution_signing_notarization_receipt_status_upgrade_accepted: false,
    artifact_distribution_signing_notarization_receipt_completed_status_accepted: false,
    artifact_distribution_signing_notarization_receipt_ack_replay_accepted: false,
    artifact_distribution_signing_notarization_receipt_ledger_replay_accepted: false,
    artifact_distribution_signing_notarization_receipt_index_replay_accepted: false,
    artifact_distribution_signing_notarization_receipt_delivery_replay_accepted: false,
    artifact_distribution_signing_notarization_receipt_query_replay_accepted: false,
    artifact_distribution_signing_notarization_receipt_export_replay_accepted: false,
    artifact_distribution_signing_notarization_receipt_observability_replay_accepted: false,
    artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted: false,
    artifact_signing_receipt_replay_accepted: false,
    package_signing_receipt_replay_accepted: false,
    signature_manifest_receipt_idempotency_recorded: false,
    notarization_submission_receipt_idempotency_persisted: false,
    notarization_ticket_receipt_nonce_recorded: false,
    stapling_receipt_cross_scope_reuse_accepted: false,
    installer_signing_receipt_out_of_order_accepted: false,
    provenance_attestation_receipt_ack_replay_accepted: false,
    sbom_manifest_receipt_ledger_index_replay_accepted: false,
    release_asset_bundle_receipt_export_query_replay_accepted: false,
    cdn_update_feed_receipt_observability_replay_accepted: false,
    package_registry_receipt_status_rebind_accepted: false,
    dashboard_endpoint_receipt_hash_status_replay_accepted: false,
    external_signing_receipt_delivery_replay_accepted: false,
    telegram_signing_receipt_delivery_replay_accepted: false,
    acceptance_from_signing_receipt_replay_recorded: false,
    operator_approval_from_signing_receipt_replay_derived: false,
    release_publication_authority_from_signing_receipt_replay_derived: false,
    activation_authority_from_signing_receipt_replay_derived: false,
    download_link_from_signing_receipt_replay_rendered: false,
    install_command_from_signing_receipt_replay_rendered: false,
    install_from_signing_receipt_replay_executed: false,
    service_restart_from_signing_receipt_replay_performed: false,
    active_binary_from_signing_receipt_replay_mutated: false,
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
    attachment_blocker_count: 70,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_readback_without_receipt",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-receipt-no-persistence-final-index-artifact-signing-replay-idempotency-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_RECEIPT_NO_PERSISTENCE_FINAL_INDEX_ARTIFACT_SIGNING_REPLAY_IDEMPOTENCY_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_receipt_no_persistence_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-receipt-no-persistence-final-index-report.sh",
      operator_identity_session_intent_consent_evidence_artifact_signing_replay_idempotency_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-replay-idempotency-denial-gate.sh",
      operator_identity_session_intent_consent_evidence_artifact_signing_replay_idempotency_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_REPLAY_IDEMPOTENCY_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_denial_gate_invoked: false,
      operator_identity_session_operator_intent_consent_evidence_artifact_signing_receipt_no_persistence_denial_gate_invoked: false,
      artifact_distribution_signing_notarization_receipt_replay_recorded: false,
      artifact_distribution_signing_notarization_receipt_replay_persisted: false,
      artifact_distribution_signing_notarization_receipt_replay_performed: false,
      artifact_distribution_signing_notarization_receipt_duplicate_recorded: false,
      artifact_distribution_signing_notarization_receipt_duplicate_persisted: false,
      artifact_distribution_signing_notarization_receipt_idempotency_key_recorded: false,
      artifact_distribution_signing_notarization_receipt_idempotency_state_recorded: false,
      artifact_distribution_signing_notarization_receipt_idempotency_state_persisted: false,
      artifact_distribution_signing_notarization_receipt_idempotency_state_materialized: false,
      artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written: false,
      artifact_distribution_signing_notarization_receipt_replay_nonce_recorded: false,
      artifact_distribution_signing_notarization_receipt_completed_status_accepted: false,
      artifact_distribution_signing_notarization_receipt_ack_replay_accepted: false,
      artifact_distribution_signing_notarization_receipt_ledger_replay_accepted: false,
      artifact_distribution_signing_notarization_receipt_index_replay_accepted: false,
      artifact_distribution_signing_notarization_receipt_delivery_replay_accepted: false,
      artifact_distribution_signing_notarization_receipt_query_replay_accepted: false,
      artifact_distribution_signing_notarization_receipt_export_replay_accepted: false,
      artifact_distribution_signing_notarization_receipt_observability_replay_accepted: false,
      operator_approval_from_signing_receipt_replay_derived: false,
      release_publication_authority_from_signing_receipt_replay_derived: false,
      activation_authority_from_signing_receipt_replay_derived: false,
      download_link_from_signing_receipt_replay_rendered: false,
      install_command_from_signing_receipt_replay_rendered: false,
      install_from_signing_receipt_replay_executed: false,
      service_restart_from_signing_receipt_replay_performed: false,
      active_binary_from_signing_receipt_replay_mutated: false,
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
