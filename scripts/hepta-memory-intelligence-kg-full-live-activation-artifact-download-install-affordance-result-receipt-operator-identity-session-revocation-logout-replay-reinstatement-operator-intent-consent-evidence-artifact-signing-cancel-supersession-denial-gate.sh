#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${HEPTA_LIVE_URL:-http://127.0.0.1:7373}"
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd -P)"
MIN_LONG_SOAK_SAMPLES="${HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES:-24}"

cd "$REPO_ROOT"

source scripts/lib/hepta-json-report-capture.sh

if [[ "$MIN_LONG_SOAK_SAMPLES" -lt 24 ]]; then
  echo "minimum long-soak samples must be at least 24" >&2
  exit 1
fi

sha256_text() {
  printf '%s' "$1" | shasum -a 256 | awk '{print $1}'
}

ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_ORDERING_MONOTONICITY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-ordering-monotonicity-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-ordering-monotonicity-denial-gate.sh
)"

source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_report_sha256="$(
  sha256_text "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_ORDERING_MONOTONICITY_JSON"
)"
artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-cancellation-supersession-denial:$source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_distribution_signing_notarization_receipt_cancellation_supersession_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-cancellation-supersession:no-cancellation:no-supersession:no-replacement:no-tombstone:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_ORDERING_MONOTONICITY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_receipt_replay_idempotency_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed_count",
      "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted_count",
      "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded_count",
      "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted_count",
      "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_materialized_count",
      "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_filesystem_written_count",
      "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded_count",
      "artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted_count",
      "artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded_count",
      "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted_count",
      "artifact_distribution_signing_notarization_receipt_monotonicity_state_materialized_count",
      "artifact_distribution_signing_notarization_receipt_duplicate_sequence_accepted_count",
      "artifact_distribution_signing_notarization_receipt_stale_sequence_accepted_count",
      "artifact_distribution_signing_notarization_receipt_late_arrival_accepted_count",
      "artifact_distribution_signing_notarization_receipt_future_gap_accepted_count",
      "artifact_distribution_signing_notarization_receipt_timestamp_rollback_accepted_count",
      "artifact_distribution_signing_notarization_receipt_epoch_rollback_accepted_count",
      "artifact_distribution_signing_notarization_receipt_same_sequence_different_hash_accepted_count",
      "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted_count",
      "artifact_distribution_signing_notarization_receipt_ordered_query_export_accepted_count",
      "artifact_distribution_signing_notarization_receipt_ordered_observability_accepted_count",
      "artifact_distribution_signing_notarization_receipt_ordered_delivery_accepted_count",
      "artifact_distribution_signing_notarization_receipt_ordered_status_accepted_count",
      "artifact_distribution_signing_notarization_receipt_ordered_hash_status_accepted_count",
      "release_publication_authority_from_signing_receipt_ordering_derived_count",
      "activation_authority_from_signing_receipt_ordering_derived_count",
      "install_from_signing_receipt_ordering_executed_count",
      "service_restart_from_signing_receipt_ordering_performed_count",
      "active_binary_from_signing_receipt_ordering_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted",
      "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded",
      "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted",
      "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded",
      "artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted",
      "artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded",
      "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted",
      "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted",
      "public_release_claimed",
      "public_ga_claimed",
      "operator_acceptance_recorded",
      "operator_approval_recorded",
      "release_publication_authority_derived",
      "activation_authority_derived",
      "activation_allowed",
      "activation_performed",
      "memory_store_write_performed",
      "memory_store_mutated",
      "live_kg_write_performed",
      "provider_invoked",
      "model_invoked",
      "credential_read",
      "secret_file_read",
      "install_executed",
      "launchd_mutated",
      "service_restarted",
      "active_binary_mutated",
      "external_send_performed"
    ])
    and ($source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempted == true
      and .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed == false
      and .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted == false
      and .artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded == false
      and .artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted == false
      and .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_noop_confirmed == true
      and .release_publication_authority_derived == false
      and .activation_authority_derived == false
      and .install_executed == false
      and .service_restarted == false
      and .active_binary_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_gate"
      and .accepts_ordering == false
      and .records_sequence_cursor == false
      and .persists_monotonicity_state == false
      and .accepts_cancellation == false
      and .accepts_supersession == false
      and .records_operator_acceptance == false
      and .derives_release_publication_authority == false
      and .derives_activation_authority == false
      and .installs_or_restarts == false
      and .mutates_active_binary == false
      and .mutates_memory_store == false
      and .writes_kg == false
      and .invokes_provider == false
      and .reads_credentials == false
      and .sends_externally == false
    ))
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '
    def cancellation_surface($id; $status; $reason; $extra):
      {
        surface:$id,
        source_signing_receipt_ordering_monotonicity_denial_ready:true,
        canonical_noop_signing_receipt_identity_required:true,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempted:true,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed:false,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted:false,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded:false,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted:false,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized:false,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written:false,
        artifact_distribution_signing_notarization_receipt_cancellation_accepted:false,
        artifact_distribution_signing_notarization_receipt_cancellation_recorded:false,
        artifact_distribution_signing_notarization_receipt_cancellation_persisted:false,
        artifact_distribution_signing_notarization_receipt_withdrawal_accepted:false,
        artifact_distribution_signing_notarization_receipt_supersession_accepted:false,
        artifact_distribution_signing_notarization_receipt_supersession_recorded:false,
        artifact_distribution_signing_notarization_receipt_supersession_persisted:false,
        artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted:false,
        artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded:false,
        artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted:false,
        artifact_distribution_signing_notarization_receipt_tombstone_recorded:false,
        artifact_distribution_signing_notarization_receipt_tombstone_persisted:false,
        artifact_distribution_signing_notarization_receipt_delete_marker_recorded:false,
        artifact_distribution_signing_notarization_receipt_delete_marker_persisted:false,
        artifact_distribution_signing_notarization_receipt_latest_replacement_accepted:false,
        artifact_distribution_signing_notarization_receipt_ack_replacement_accepted:false,
        artifact_distribution_signing_notarization_receipt_query_replacement_accepted:false,
        artifact_distribution_signing_notarization_receipt_export_replacement_accepted:false,
        artifact_distribution_signing_notarization_receipt_observability_replacement_accepted:false,
        artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted:false,
        artifact_signing_receipt_cancellation_accepted:false,
        package_signing_receipt_cancellation_accepted:false,
        signature_manifest_receipt_withdrawal_accepted:false,
        notarization_submission_receipt_cancellation_accepted:false,
        notarization_ticket_receipt_supersession_accepted:false,
        stapling_receipt_tombstone_recorded:false,
        installer_signing_receipt_replacement_accepted:false,
        provenance_attestation_latest_replacement_accepted:false,
        sbom_manifest_supersession_accepted:false,
        release_asset_bundle_cancelled_query_export_accepted:false,
        cdn_update_feed_superseded_observability_accepted:false,
        package_registry_replacement_status_accepted:false,
        dashboard_endpoint_tombstone_hash_status_accepted:false,
        external_supersession_delivery_accepted:false,
        telegram_supersession_delivery_accepted:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        acceptance_recorded:false,
        operator_approval_derived:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        download_link_rendered:false,
        install_command_rendered:false,
        install_executed:false,
        service_restarted:false,
        launchd_mutated:false,
        active_binary_mutated:false,
        memory_store_write_performed:false,
        memory_store_mutated:false,
        live_kg_write_performed:false,
        provider_invoked:false,
        model_invoked:false,
        credential_read:false,
        secret_file_read:false,
        telegram_send_performed:false,
        channel_send_performed:false,
        external_send_performed:false,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_status:$status,
        reason:$reason
      } + $extra;
    [
      cancellation_surface("source_signing_receipt_ordering_monotonicity_report_required"; "blocked_source_signing_receipt_ordering_monotonicity_required_noop"; "source_signing_receipt_ordering_monotonicity_report_required"; {source_report_required:true}),
      cancellation_surface("duplicate_signing_receipt_cancellation"; "blocked_duplicate_signing_receipt_cancellation_noop"; "duplicate_signing_receipt_cancellation_denied"; {duplicate_signing_receipt_cancellation_requested:true}),
      cancellation_surface("stale_package_signing_receipt_cancellation"; "blocked_stale_package_signing_receipt_cancellation_noop"; "stale_package_signing_receipt_cancellation_denied"; {stale_package_signing_receipt_cancellation_requested:true}),
      cancellation_surface("signature_manifest_late_arrival_withdrawal"; "blocked_signature_manifest_late_arrival_withdrawal_noop"; "signature_manifest_late_arrival_withdrawal_denied"; {signature_manifest_late_arrival_withdrawal_requested:true}),
      cancellation_surface("notarization_submission_future_gap_cancellation"; "blocked_notarization_submission_future_gap_cancellation_noop"; "notarization_submission_future_gap_cancellation_denied"; {notarization_submission_future_gap_cancellation_requested:true}),
      cancellation_surface("notarization_ticket_rollback_supersession"; "blocked_notarization_ticket_rollback_supersession_noop"; "notarization_ticket_rollback_supersession_denied"; {notarization_ticket_rollback_supersession_requested:true}),
      cancellation_surface("stapling_epoch_rollback_tombstone"; "blocked_stapling_epoch_rollback_tombstone_noop"; "stapling_epoch_rollback_tombstone_denied"; {stapling_epoch_rollback_tombstone_requested:true}),
      cancellation_surface("installer_same_sequence_hash_replacement"; "blocked_installer_same_sequence_hash_replacement_noop"; "installer_same_sequence_hash_replacement_denied"; {installer_same_sequence_hash_replacement_requested:true}),
      cancellation_surface("provenance_latest_wins_cancellation"; "blocked_provenance_latest_wins_cancellation_noop"; "provenance_latest_wins_cancellation_denied"; {provenance_latest_wins_cancellation_requested:true}),
      cancellation_surface("sbom_monotonic_cursor_supersession"; "blocked_sbom_monotonic_cursor_supersession_noop"; "sbom_monotonic_cursor_supersession_denied"; {sbom_monotonic_cursor_supersession_requested:true}),
      cancellation_surface("release_asset_bundle_cancelled_query_export"; "blocked_release_asset_bundle_cancelled_query_export_noop"; "release_asset_bundle_cancelled_query_export_denied"; {release_asset_bundle_cancelled_query_export_requested:true}),
      cancellation_surface("cdn_update_feed_superseded_observability"; "blocked_cdn_update_feed_superseded_observability_noop"; "cdn_update_feed_superseded_observability_denied"; {cdn_update_feed_superseded_observability_requested:true}),
      cancellation_surface("package_registry_replacement_status"; "blocked_package_registry_replacement_status_noop"; "package_registry_replacement_status_denied"; {package_registry_replacement_status_requested:true}),
      cancellation_surface("dashboard_endpoint_tombstone_hash_status"; "blocked_dashboard_endpoint_tombstone_hash_status_noop"; "dashboard_endpoint_tombstone_hash_status_denied"; {dashboard_endpoint_tombstone_hash_status_requested:true}),
      cancellation_surface("external_telegram_supersession_delivery"; "blocked_external_telegram_supersession_delivery_noop"; "external_telegram_supersession_delivery_denied"; {external_supersession_delivery_requested:true, telegram_supersession_delivery_requested:true}),
      cancellation_surface("release_publication_authority_cancellation_supersession"; "blocked_release_publication_authority_cancellation_supersession_noop"; "release_publication_authority_cancellation_supersession_denied"; {release_publication_authority_cancellation_supersession_requested:true}),
      cancellation_surface("activation_live_install_supersession"; "blocked_activation_live_install_supersession_noop"; "activation_live_install_supersession_denied"; {activation_live_install_supersession_requested:true}),
      cancellation_surface("install_restart_active_binary_cancellation_path"; "blocked_install_restart_active_binary_cancellation_path_noop"; "install_restart_active_binary_cancellation_path_denied"; {install_restart_active_binary_cancellation_path_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_gate" \
    --arg source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_report_sha256 "$source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_report_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256 "$artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_cancellation_supersession_policy_hash_sha256 "$artifact_distribution_signing_notarization_receipt_cancellation_supersession_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_ORDERING_MONOTONICITY_JSON" \
    --argjson surfaces "$surfaces_json" \
    '
      def zero_object($fields): reduce $fields[] as $field ({}; .[$field]=0);
      def false_object($fields): reduce $fields[] as $field ({}; .[$field]=false);
      {
        product:$product,
        runtime:$runtime,
        status:"ready",
        base_url:$base_url,
        gate:$gate,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_v1",
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_mode:"denied_signing_receipt_ordering_monotonicity_cannot_be_cancelled_superseded_replaced_tombstoned_promoted_or_used_for_authority_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_report_sha256:$source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256:$artifact_distribution_signing_notarization_receipt_cancellation_supersession_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_policy_hash_sha256:$artifact_distribution_signing_notarization_receipt_cancellation_supersession_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count:$source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count,
        source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count:$source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count,
        source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count:$source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count,
        source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted_count:$source.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted_count,
        source_artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted_count:$source.artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted_count,
        source_release_publication_authority_from_signing_receipt_ordering_derived_count:$source.release_publication_authority_from_signing_receipt_ordering_derived_count,
        source_activation_authority_from_signing_receipt_ordering_derived_count:$source.activation_authority_from_signing_receipt_ordering_derived_count,
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_cancellation_supersession:[
          "source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_report_required",
          "signing_receipt_cancellation_denied",
          "signing_receipt_withdrawal_denied",
          "signing_receipt_supersession_denied",
          "signing_receipt_replacement_denied",
          "signing_receipt_tombstone_denied",
          "signing_receipt_delete_marker_denied",
          "signing_receipt_latest_replacement_denied",
          "signing_receipt_ack_replacement_denied",
          "signing_receipt_query_export_replacement_denied",
          "signing_receipt_observability_replacement_denied",
          "signing_receipt_lifecycle_cancellation_supersession_denied",
          "external_telegram_signing_receipt_supersession_delivery_denied",
          "release_publication_authority_from_signing_receipt_cancellation_supersession_denied",
          "activation_live_install_from_signing_receipt_supersession_denied",
          "install_restart_active_binary_from_signing_receipt_cancellation_denied",
          "memory_provider_kg_secret_external_send_from_signing_receipt_cancellation_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_gate",
            status:"allowed_report_only_next_slice",
            accepts_cancellation:false,
            accepts_supersession:false,
            records_replacement_receipt:false,
            records_tombstone:false,
            records_delete_marker:false,
            persists_lifecycle_state:false,
            records_audit_evidence:false,
            records_operator_acceptance:false,
            derives_release_publication_authority:false,
            derives_activation_authority:false,
            renders_download_link:false,
            emits_install_command:false,
            installs_or_restarts:false,
            mutates_active_binary:false,
            mutates_memory_store:false,
            writes_kg:false,
            invokes_provider:false,
            reads_credentials:false,
            sends_externally:false
          }
        ]
      }
      + zero_object([
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_accepted_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_recorded_count",
        "artifact_distribution_signing_notarization_receipt_cancellation_persisted_count",
        "artifact_distribution_signing_notarization_receipt_withdrawal_accepted_count",
        "artifact_distribution_signing_notarization_receipt_supersession_accepted_count",
        "artifact_distribution_signing_notarization_receipt_supersession_recorded_count",
        "artifact_distribution_signing_notarization_receipt_supersession_persisted_count",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted_count",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded_count",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted_count",
        "artifact_distribution_signing_notarization_receipt_tombstone_recorded_count",
        "artifact_distribution_signing_notarization_receipt_tombstone_persisted_count",
        "artifact_distribution_signing_notarization_receipt_delete_marker_recorded_count",
        "artifact_distribution_signing_notarization_receipt_delete_marker_persisted_count",
        "artifact_distribution_signing_notarization_receipt_latest_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_ack_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_query_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_export_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_observability_replacement_accepted_count",
        "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count",
        "artifact_signing_receipt_cancellation_accepted_count",
        "package_signing_receipt_cancellation_accepted_count",
        "signature_manifest_receipt_withdrawal_accepted_count",
        "notarization_submission_receipt_cancellation_accepted_count",
        "notarization_ticket_receipt_supersession_accepted_count",
        "stapling_receipt_tombstone_recorded_count",
        "installer_signing_receipt_replacement_accepted_count",
        "provenance_attestation_latest_replacement_accepted_count",
        "sbom_manifest_supersession_accepted_count",
        "release_asset_bundle_cancelled_query_export_accepted_count",
        "cdn_update_feed_superseded_observability_accepted_count",
        "package_registry_replacement_status_accepted_count",
        "dashboard_endpoint_tombstone_hash_status_accepted_count",
        "external_supersession_delivery_accepted_count",
        "telegram_supersession_delivery_accepted_count",
        "acceptance_from_signing_receipt_cancellation_recorded_count",
        "operator_approval_from_signing_receipt_cancellation_derived_count",
        "release_publication_authority_from_signing_receipt_cancellation_derived_count",
        "activation_authority_from_signing_receipt_supersession_derived_count",
        "download_link_from_signing_receipt_cancellation_rendered_count",
        "install_command_from_signing_receipt_supersession_rendered_count",
        "install_from_signing_receipt_cancellation_executed_count",
        "service_restart_from_signing_receipt_supersession_performed_count",
        "active_binary_from_signing_receipt_cancellation_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted",
        "artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized",
        "artifact_distribution_signing_notarization_receipt_cancellation_accepted",
        "artifact_distribution_signing_notarization_receipt_cancellation_recorded",
        "artifact_distribution_signing_notarization_receipt_cancellation_persisted",
        "artifact_distribution_signing_notarization_receipt_withdrawal_accepted",
        "artifact_distribution_signing_notarization_receipt_supersession_accepted",
        "artifact_distribution_signing_notarization_receipt_supersession_recorded",
        "artifact_distribution_signing_notarization_receipt_supersession_persisted",
        "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted",
        "artifact_distribution_signing_notarization_receipt_tombstone_recorded",
        "artifact_distribution_signing_notarization_receipt_delete_marker_recorded",
        "public_release_claimed",
        "public_ga_claimed",
        "operator_acceptance_recorded",
        "operator_approval_recorded",
        "release_publication_authority_derived",
        "activation_authority_derived",
        "activation_allowed",
        "activation_performed",
        "memory_store_write_performed",
        "memory_store_mutated",
        "live_kg_write_performed",
        "provider_invoked",
        "model_invoked",
        "credential_read",
        "secret_file_read",
        "install_executed",
        "launchd_mutated",
        "service_restarted",
        "active_binary_mutated",
        "external_send_performed"
      ])
      + {
        side_effects:false_object([
          "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded",
          "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted",
          "artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized",
          "artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written",
          "artifact_distribution_signing_notarization_receipt_cancellation_recorded",
          "artifact_distribution_signing_notarization_receipt_cancellation_persisted",
          "artifact_distribution_signing_notarization_receipt_supersession_recorded",
          "artifact_distribution_signing_notarization_receipt_supersession_persisted",
          "artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded",
          "artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted",
          "artifact_distribution_signing_notarization_receipt_tombstone_recorded",
          "artifact_distribution_signing_notarization_receipt_tombstone_persisted",
          "artifact_distribution_signing_notarization_receipt_delete_marker_recorded",
          "artifact_distribution_signing_notarization_receipt_delete_marker_persisted",
          "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted",
          "operator_acceptance_recorded",
          "operator_approval_recorded",
          "release_publication_authority_derived",
          "activation_authority_derived",
          "download_link_rendered",
          "install_command_rendered",
          "install_executed",
          "launchd_mutated",
          "service_restarted",
          "active_binary_mutated",
          "memory_store_write_performed",
          "memory_store_mutated",
          "live_kg_write_performed",
          "provider_invoked",
          "model_invoked",
          "credential_read",
          "secret_file_read",
          "telegram_send_performed",
          "channel_send_performed",
          "external_send_performed",
          "release_artifact_written",
          "public_artifact_written",
          "filesystem_written"
        ])
      }
    '
)"

jq -e '
  def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
  def false_fields($o; $fields): all($fields[]; $o[.] == false);

  . as $report
  | $report.runtime == "hepta"
  and $report.status == "ready"
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_cancellation_supersession_denied_count == 18
  and zero_fields($report; [
    "artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed_count",
    "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted_count",
    "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded_count",
    "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted_count",
    "artifact_distribution_signing_notarization_receipt_cancellation_supersession_materialized_count",
    "artifact_distribution_signing_notarization_receipt_cancellation_supersession_filesystem_written_count",
    "artifact_distribution_signing_notarization_receipt_cancellation_accepted_count",
    "artifact_distribution_signing_notarization_receipt_cancellation_recorded_count",
    "artifact_distribution_signing_notarization_receipt_cancellation_persisted_count",
    "artifact_distribution_signing_notarization_receipt_withdrawal_accepted_count",
    "artifact_distribution_signing_notarization_receipt_supersession_accepted_count",
    "artifact_distribution_signing_notarization_receipt_supersession_recorded_count",
    "artifact_distribution_signing_notarization_receipt_supersession_persisted_count",
    "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted_count",
    "artifact_distribution_signing_notarization_receipt_replacement_receipt_recorded_count",
    "artifact_distribution_signing_notarization_receipt_replacement_receipt_persisted_count",
    "artifact_distribution_signing_notarization_receipt_tombstone_recorded_count",
    "artifact_distribution_signing_notarization_receipt_tombstone_persisted_count",
    "artifact_distribution_signing_notarization_receipt_delete_marker_recorded_count",
    "artifact_distribution_signing_notarization_receipt_delete_marker_persisted_count",
    "artifact_distribution_signing_notarization_receipt_latest_replacement_accepted_count",
    "artifact_distribution_signing_notarization_receipt_lifecycle_cancellation_supersession_persisted_count",
    "release_publication_authority_from_signing_receipt_cancellation_derived_count",
    "activation_authority_from_signing_receipt_supersession_derived_count",
    "install_from_signing_receipt_cancellation_executed_count",
    "service_restart_from_signing_receipt_supersession_performed_count",
    "active_binary_from_signing_receipt_cancellation_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted",
    "artifact_distribution_signing_notarization_receipt_cancellation_supersession_recorded",
    "artifact_distribution_signing_notarization_receipt_cancellation_supersession_persisted",
    "artifact_distribution_signing_notarization_receipt_cancellation_accepted",
    "artifact_distribution_signing_notarization_receipt_cancellation_recorded",
    "artifact_distribution_signing_notarization_receipt_cancellation_persisted",
    "artifact_distribution_signing_notarization_receipt_withdrawal_accepted",
    "artifact_distribution_signing_notarization_receipt_supersession_accepted",
    "artifact_distribution_signing_notarization_receipt_supersession_recorded",
    "artifact_distribution_signing_notarization_receipt_supersession_persisted",
    "artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted",
    "artifact_distribution_signing_notarization_receipt_tombstone_recorded",
    "artifact_distribution_signing_notarization_receipt_delete_marker_recorded",
    "public_release_claimed",
    "public_ga_claimed",
    "operator_acceptance_recorded",
    "operator_approval_recorded",
    "release_publication_authority_derived",
    "activation_authority_derived",
    "activation_allowed",
    "activation_performed",
    "memory_store_write_performed",
    "memory_store_mutated",
    "live_kg_write_performed",
    "provider_invoked",
    "model_invoked",
    "credential_read",
    "secret_file_read",
    "install_executed",
    "launchd_mutated",
    "service_restarted",
    "active_binary_mutated",
    "external_send_performed"
  ])
  and ($report.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_cancellation_supersession_attempted == true
    and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_allowed == false
    and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_accepted == false
    and .artifact_distribution_signing_notarization_receipt_cancellation_accepted == false
    and .artifact_distribution_signing_notarization_receipt_supersession_accepted == false
    and .artifact_distribution_signing_notarization_receipt_replacement_receipt_accepted == false
    and .artifact_distribution_signing_notarization_receipt_tombstone_recorded == false
    and .artifact_distribution_signing_notarization_receipt_delete_marker_recorded == false
    and .artifact_distribution_signing_notarization_receipt_cancellation_supersession_noop_confirmed == true
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
    and .download_link_rendered == false
    and .install_command_rendered == false
    and .install_executed == false
    and .service_restarted == false
    and .active_binary_mutated == false
    and .memory_store_write_performed == false
    and .memory_store_mutated == false
    and .live_kg_write_performed == false
    and .provider_invoked == false
    and .model_invoked == false
    and .credential_read == false
    and .secret_file_read == false
    and .external_send_performed == false
  ))
  and ([.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces[] | select(.signature_manifest_late_arrival_withdrawal_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces[] | select(.stapling_epoch_rollback_tombstone_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces[] | select(.telegram_supersession_delivery_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_cancellation_supersession_surfaces[] | select(.install_restart_active_binary_cancellation_path_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_audit_evidence_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .accepts_cancellation == false
    and .accepts_supersession == false
    and .records_replacement_receipt == false
    and .records_tombstone == false
    and .records_delete_marker == false
    and .persists_lifecycle_state == false
    and .records_audit_evidence == false
    and .records_operator_acceptance == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .renders_download_link == false
    and .emits_install_command == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .invokes_provider == false
    and .reads_credentials == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt cancellation/supersession denial gate passed"
