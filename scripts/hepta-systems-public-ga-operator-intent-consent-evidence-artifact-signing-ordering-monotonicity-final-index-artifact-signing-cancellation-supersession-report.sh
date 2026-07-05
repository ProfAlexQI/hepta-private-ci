#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-ordering-monotonicity-final-index-report.sh"
CANCELLATION_GATE="$ROOT/scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-cancel-supersession-denial-gate.sh"
CANCELLATION_DOC="$ROOT/docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_CANCEL_SUPERSESSION_DENIAL_GATE.md"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence artifact signing ordering/monotonicity final index report: $SOURCE_REPORT" >&2
  exit 1
}
[[ -f "$CANCELLATION_GATE" ]] || {
  echo "missing operator identity/session intent consent evidence artifact signing cancellation/supersession denial gate: $CANCELLATION_GATE" >&2
  exit 1
}
[[ -f "$CANCELLATION_DOC" ]] || {
  echo "missing operator identity/session intent consent evidence artifact signing cancellation/supersession denial doc: $CANCELLATION_DOC" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence artifact signing cancellation/supersession report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_blocked == true
  and .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded == false
  and .artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded == false
  and .artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded == false
  and .operator_approval_from_signing_receipt_ordering_derived == false
  and .public_ga_claimed == false
' <<<"$source_json" >/dev/null

cancellation_static_mention_count="$(
  grep -Eci 'cancellation|supersession|cancel|withdraw|replacement|tombstone|delete.marker|lifecycle|latest.replacement|ack.replacement|query|export|observability|artifact.signing|package.signing|signature.manifest|notarization|stapling|installer|provenance|sbom|release.asset|bundle|cdn|update.feed|registry|dashboard|endpoint|telegram|external|authority|download|install|restart|active-binary|live' "$CANCELLATION_GATE" || true
)"

jq -n \
  --argjson source "$source_json" \
  --argjson cancellation_static_mention_count "$cancellation_static_mention_count" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_attachment",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_attachment_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_attachment_blocked: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_present: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_doc_present: true,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_static_mention_count: $cancellation_static_mention_count,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_gate_invoked: false,
    long_soak_required_by_source_evidence_artifact_signing_cancellation_supersession_gate: true,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized: false,
    artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written: false,
    artifact_distribution_signing_notarization_receipt_cancellation_accepted: false,
    artifact_distribution_signing_notarization_receipt_cancellation_recorded: false,
    artifact_distribution_signing_notarization_receipt_cancellation_persisted: false,
    artifact_distribution_signing_notarization_receipt_withdrawal_accepted: false,
    artifact_distribution_signing_notarization_receipt_supersession_accepted: false,
    artifact_distribution_signing_notarization_receipt_supersession_recorded: false,
    artifact_distribution_signing_notarization_receipt_supersession_persisted: false,
    artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted: false,
    artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded: false,
    artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted: false,
    artifact_distribution_signing_notarization_receipt_tombstone_recorded: false,
    artifact_distribution_signing_notarization_receipt_tombstone_persisted: false,
    artifact_distribution_signing_notarization_receipt_delete_marker_recorded: false,
    artifact_distribution_signing_notarization_receipt_delete_marker_persisted: false,
    artifact_distribution_signing_notarization_receipt_latest_replacement_accepted: false,
    artifact_distribution_signing_notarization_receipt_ack_replacement_accepted: false,
    artifact_distribution_signing_notarization_receipt_query_replacement_accepted: false,
    artifact_distribution_signing_notarization_receipt_export_replacement_accepted: false,
    artifact_distribution_signing_notarization_receipt_observability_replacement_accepted: false,
    artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted: false,
    artifact_signing_receipt_cancellation_accepted: false,
    package_signing_receipt_cancellation_accepted: false,
    signature_manifest_receipt_withdrawal_accepted: false,
    notarization_submission_receipt_cancellation_accepted: false,
    notarization_ticket_receipt_supersession_accepted: false,
    stapling_receipt_tombstone_recorded: false,
    installer_signing_receipt_replacement_accepted: false,
    provenance_attestation_latest_replacement_accepted: false,
    sbom_manifest_supersession_accepted: false,
    release_asset_bundle_cancelled_query_export_accepted: false,
    cdn_update_feed_superseded_observability_accepted: false,
    package_registry_replacement_status_accepted: false,
    dashboard_endpoint_tombstone_hash_status_accepted: false,
    external_supersession_delivery_accepted: false,
    telegram_supersession_delivery_accepted: false,
    acceptance_from_signing_receipt_cancellation_recorded: false,
    operator_approval_from_signing_receipt_cancellation_derived: false,
    release_publication_authority_from_signing_receipt_cancellation_derived: false,
    activation_authority_from_signing_receipt_supersession_derived: false,
    download_link_from_signing_receipt_cancellation_rendered: false,
    install_command_from_signing_receipt_supersession_rendered: false,
    install_from_signing_receipt_cancellation_executed: false,
    service_restart_from_signing_receipt_supersession_performed: false,
    active_binary_from_signing_receipt_cancellation_mutated: false,
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
    attachment_blocker_count: 74,
    manual_operator_live_cutover_approval_required: true,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "derive_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_readback_without_ordering",
    local_gate: "scripts/hepta-systems-public-ga-operator-intent-consent-evidence-artifact-signing-ordering-monotonicity-final-index-artifact-signing-cancellation-supersession-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_ORDERING_MONOTONICITY_FINAL_INDEX_ARTIFACT_SIGNING_CANCELLATION_SUPERSESSION_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-ordering-monotonicity-final-index-report.sh",
      operator_identity_session_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate: "scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-cancel-supersession-denial-gate.sh",
      operator_identity_session_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_doc: "docs/architecture/HEPTA_MEMORY_INTELLIGENCE_KG_FULL_LIVE_ACTIVATION_ARTIFACT_DOWNLOAD_INSTALL_AFFORDANCE_RESULT_RECEIPT_OPERATOR_IDENTITY_SESSION_REVOCATION_LOGOUT_REPLAY_REINSTATEMENT_OPERATOR_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_CANCEL_SUPERSESSION_DENIAL_GATE.md"
    },
    side_effect_free: true,
    side_effects: {
      report_written: false,
      git_index_mutated: false,
      operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_denial_gate_invoked: false,
      operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_gate_invoked: false,
      artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded: false,
      artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted: false,
      artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized: false,
      artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written: false,
      artifact_distribution_signing_notarization_receipt_cancellation_recorded: false,
      artifact_distribution_signing_notarization_receipt_cancellation_persisted: false,
      artifact_distribution_signing_notarization_receipt_supersession_recorded: false,
      artifact_distribution_signing_notarization_receipt_supersession_persisted: false,
      artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded: false,
      artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted: false,
      artifact_distribution_signing_notarization_receipt_tombstone_recorded: false,
      artifact_distribution_signing_notarization_receipt_tombstone_persisted: false,
      artifact_distribution_signing_notarization_receipt_delete_marker_recorded: false,
      artifact_distribution_signing_notarization_receipt_delete_marker_persisted: false,
      artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted: false,
      operator_approval_from_signing_receipt_cancellation_derived: false,
      release_publication_authority_from_signing_receipt_cancellation_derived: false,
      activation_authority_from_signing_receipt_supersession_derived: false,
      download_link_from_signing_receipt_cancellation_rendered: false,
      install_command_from_signing_receipt_supersession_rendered: false,
      install_from_signing_receipt_cancellation_executed: false,
      service_restart_from_signing_receipt_supersession_performed: false,
      active_binary_from_signing_receipt_cancellation_mutated: false,
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
