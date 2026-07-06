#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
SOURCE_REPORT="$ROOT/scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-ordering-monotonicity-readback-report.sh"

[[ -x "$SOURCE_REPORT" ]] || {
  echo "missing executable Public GA operator identity/session intent consent evidence artifact signing ordering/monotonicity readback report: $SOURCE_REPORT" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required to build the Public GA operator identity/session intent consent evidence artifact signing ordering/monotonicity final index report" >&2
  exit 1
fi

source_json="$("$SOURCE_REPORT")"

jq -e '
  .surface == "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback"
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback_ready == true
  and .public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback_blocked == true
  and .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded == false
  and .artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded == false
  and .artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded == false
  and .operator_approval_from_signing_receipt_ordering_derived == false
' <<<"$source_json" >/dev/null

jq -n \
  --argjson source "$source_json" \
  '{
    runtime: "hepta",
    surface: "public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index",
    plugin_id: "hepta-system@hepta-local",
    status: "ready_blocked",
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback_surface: $source.surface,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback_ready: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback_ready,
    source_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback_blocked: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback_blocked,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_ready: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_blocked: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback_attached: true,
    public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_final_index_attached: $source.public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_final_index_attached,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_gate_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_gate_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_doc_present: $source.operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_doc_present,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_denial_gate_invoked: false,
    operator_identity_session_operator_intent_consent_evidence_artifact_signing_replay_idempotency_denial_gate_invoked: false,
    long_soak_started: false,
    public_status_claimed: false,
    public_release_claimed: false,
    public_ga_claimed: false,
    artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed: false,
    artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted: false,
    artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded: false,
    artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted: false,
    artifact_distribution_signing_notarization_receipt_ordering_monotonicity_materialized: false,
    artifact_distribution_signing_notarization_receipt_ordering_monotonicity_filesystem_written: false,
    artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded: false,
    artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted: false,
    artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded: false,
    artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted: false,
    artifact_distribution_signing_notarization_receipt_monotonicity_state_materialized: false,
    artifact_distribution_signing_notarization_receipt_duplicate_sequence_accepted: false,
    artifact_distribution_signing_notarization_receipt_stale_sequence_accepted: false,
    artifact_distribution_signing_notarization_receipt_late_arrival_accepted: false,
    artifact_distribution_signing_notarization_receipt_future_gap_accepted: false,
    artifact_distribution_signing_notarization_receipt_timestamp_rollback_accepted: false,
    artifact_distribution_signing_notarization_receipt_epoch_rollback_accepted: false,
    artifact_distribution_signing_notarization_receipt_same_sequence_different_hash_accepted: false,
    artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted: false,
    artifact_distribution_signing_notarization_receipt_ordered_query_export_accepted: false,
    artifact_distribution_signing_notarization_receipt_ordered_observability_accepted: false,
    artifact_distribution_signing_notarization_receipt_ordered_delivery_accepted: false,
    artifact_distribution_signing_notarization_receipt_ordered_status_accepted: false,
    artifact_distribution_signing_notarization_receipt_ordered_hash_status_accepted: false,
    artifact_signing_receipt_ordering_accepted: false,
    package_signing_receipt_ordering_accepted: false,
    signature_manifest_receipt_late_arrival_accepted: false,
    notarization_submission_receipt_future_gap_accepted: false,
    notarization_ticket_receipt_timestamp_rollback_accepted: false,
    stapling_receipt_epoch_rollback_accepted: false,
    installer_signing_same_sequence_hash_accepted: false,
    provenance_attestation_latest_wins_accepted: false,
    sbom_manifest_monotonic_cursor_recorded: false,
    release_asset_bundle_ordered_query_export_accepted: false,
    cdn_update_feed_ordered_observability_accepted: false,
    package_registry_ordered_status_accepted: false,
    dashboard_endpoint_ordered_hash_status_accepted: false,
    external_ordered_delivery_accepted: false,
    telegram_ordered_delivery_accepted: false,
    acceptance_from_signing_receipt_ordering_recorded: false,
    operator_approval_from_signing_receipt_ordering_derived: false,
    release_publication_authority_from_signing_receipt_ordering_derived: false,
    activation_authority_from_signing_receipt_ordering_derived: false,
    download_link_from_signing_receipt_ordering_rendered: false,
    install_command_from_signing_receipt_ordering_rendered: false,
    install_from_signing_receipt_ordering_executed: false,
    service_restart_from_signing_receipt_ordering_performed: false,
    active_binary_from_signing_receipt_ordering_mutated: false,
    provider_invoked: false,
    model_invoked: false,
    credential_read: false,
    secret_file_read: false,
    final_blocker_count: 72,
    manual_operator_live_cutover_approval_required: true,
    terminal_live_url_required: false,
    long_soak_required: false,
    public_ga_claim_allowed: false,
    public_release_published: false,
    rollback_execution_allowed: false,
    next_migration_step: "attach_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_ordering_monotonicity_final_index_to_public_ga_operator_identity_session_operator_intent_consent_evidence_artifact_signing_cancellation_supersession_without_ordering",
    local_gate: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-ordering-monotonicity-final-index-gate.sh",
    architecture_note: "docs/architecture/HEPTA_SYSTEMS_PUBLIC_GA_OPERATOR_IDENTITY_SESSION_INTENT_CONSENT_EVIDENCE_ARTIFACT_SIGNING_ORDERING_MONOTONICITY_FINAL_INDEX_2026-06-21.md",
    source_files: {
      public_ga_operator_identity_session_intent_consent_evidence_artifact_signing_ordering_monotonicity_readback_report: "scripts/hepta-systems-public-ga-operator-identity-session-intent-consent-evidence-artifact-signing-ordering-monotonicity-readback-report.sh"
    },
    side_effect_free: true,
    side_effects: ($source.side_effects)
  }'
