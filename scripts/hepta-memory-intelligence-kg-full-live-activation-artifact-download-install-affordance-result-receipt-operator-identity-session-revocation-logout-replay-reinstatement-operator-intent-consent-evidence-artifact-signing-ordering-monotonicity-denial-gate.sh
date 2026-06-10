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

ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_REPLAY_IDEMPOTENCY_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-replay-idempotency-denial-gate" \
    scripts/hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-signing-replay-idempotency-denial-gate.sh
)"

source_artifact_distribution_signing_notarization_receipt_replay_idempotency_report_sha256="$(
  sha256_text "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_REPLAY_IDEMPOTENCY_JSON"
)"
artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-ordering-monotonicity-denial:$source_artifact_distribution_signing_notarization_receipt_replay_idempotency_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_distribution_signing_notarization_receipt_ordering_monotonicity_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-receipt-ordering-monotonicity:no-ordering:no-monotonic-cursor:no-latest-wins:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_REPLAY_IDEMPOTENCY_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_ready == true
    and $source.source_artifact_distribution_signing_notarization_result_receipt_ready == true
    and $source.artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count == 18
    and $source.artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count == 18
    and zero_fields($source; [
      "artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed_count",
      "artifact_distribution_signing_notarization_receipt_replay_allowed_count",
      "artifact_distribution_signing_notarization_receipt_replay_accepted_count",
      "artifact_distribution_signing_notarization_receipt_replay_recorded_count",
      "artifact_distribution_signing_notarization_receipt_replay_persisted_count",
      "artifact_distribution_signing_notarization_receipt_replay_performed_count",
      "artifact_distribution_signing_notarization_receipt_duplicate_accepted_count",
      "artifact_distribution_signing_notarization_receipt_duplicate_recorded_count",
      "artifact_distribution_signing_notarization_receipt_duplicate_persisted_count",
      "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted_count",
      "artifact_distribution_signing_notarization_receipt_idempotency_key_recorded_count",
      "artifact_distribution_signing_notarization_receipt_idempotency_state_recorded_count",
      "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count",
      "artifact_distribution_signing_notarization_receipt_idempotency_state_materialized_count",
      "artifact_distribution_signing_notarization_receipt_idempotency_filesystem_written_count",
      "artifact_distribution_signing_notarization_receipt_replay_nonce_accepted_count",
      "artifact_distribution_signing_notarization_receipt_replay_nonce_recorded_count",
      "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted_count",
      "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted_count",
      "artifact_distribution_signing_notarization_receipt_completed_status_accepted_count",
      "artifact_distribution_signing_notarization_receipt_ack_replay_accepted_count",
      "artifact_distribution_signing_notarization_receipt_ledger_replay_accepted_count",
      "artifact_distribution_signing_notarization_receipt_index_replay_accepted_count",
      "artifact_distribution_signing_notarization_receipt_delivery_replay_accepted_count",
      "artifact_distribution_signing_notarization_receipt_query_replay_accepted_count",
      "artifact_distribution_signing_notarization_receipt_export_replay_accepted_count",
      "artifact_distribution_signing_notarization_receipt_observability_replay_accepted_count",
      "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted_count",
      "artifact_signing_receipt_replay_accepted_count",
      "package_signing_receipt_replay_accepted_count",
      "signature_manifest_receipt_idempotency_recorded_count",
      "notarization_submission_receipt_idempotency_persisted_count",
      "notarization_ticket_receipt_nonce_recorded_count",
      "stapling_receipt_cross_scope_reuse_accepted_count",
      "installer_signing_receipt_out_of_order_accepted_count",
      "provenance_attestation_receipt_ack_replay_accepted_count",
      "sbom_manifest_receipt_ledger_index_replay_accepted_count",
      "release_asset_bundle_receipt_export_query_replay_accepted_count",
      "cdn_update_feed_receipt_observability_replay_accepted_count",
      "package_registry_receipt_status_rebind_accepted_count",
      "dashboard_endpoint_receipt_hash_status_replay_accepted_count",
      "external_signing_receipt_delivery_replay_accepted_count",
      "telegram_signing_receipt_delivery_replay_accepted_count",
      "acceptance_from_signing_receipt_replay_recorded_count",
      "operator_approval_from_signing_receipt_replay_derived_count",
      "release_publication_authority_from_signing_receipt_replay_derived_count",
      "activation_authority_from_signing_receipt_replay_derived_count",
      "download_link_from_signing_receipt_replay_rendered_count",
      "install_command_from_signing_receipt_replay_rendered_count",
      "install_from_signing_receipt_replay_executed_count",
      "service_restart_from_signing_receipt_replay_performed_count",
      "active_binary_from_signing_receipt_replay_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "artifact_distribution_signing_notarization_receipt_replay_allowed",
      "artifact_distribution_signing_notarization_receipt_replay_accepted",
      "artifact_distribution_signing_notarization_receipt_replay_recorded",
      "artifact_distribution_signing_notarization_receipt_replay_persisted",
      "artifact_distribution_signing_notarization_receipt_replay_performed",
      "artifact_distribution_signing_notarization_receipt_duplicate_accepted",
      "artifact_distribution_signing_notarization_receipt_idempotency_key_accepted",
      "artifact_distribution_signing_notarization_receipt_idempotency_state_persisted",
      "artifact_distribution_signing_notarization_receipt_cross_scope_reuse_accepted",
      "artifact_distribution_signing_notarization_receipt_status_upgrade_accepted",
      "artifact_distribution_signing_notarization_receipt_completed_status_accepted",
      "artifact_distribution_signing_notarization_receipt_hash_status_rebind_accepted",
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
    and ($source.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces | length) == 18
    and ($source.artifact_distribution_signing_notarization_receipt_replay_idempotency_surfaces | all(
      .artifact_distribution_signing_notarization_receipt_replay_idempotency_attempted == true
      and .artifact_distribution_signing_notarization_receipt_replay_idempotency_allowed == false
      and .artifact_distribution_signing_notarization_receipt_replay_accepted == false
      and .artifact_distribution_signing_notarization_receipt_duplicate_accepted == false
      and .artifact_distribution_signing_notarization_receipt_idempotency_key_accepted == false
      and .artifact_distribution_signing_notarization_receipt_replay_idempotency_noop_confirmed == true
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
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_gate"
      and .replays_signing_receipt == false
      and .records_duplicate_receipt == false
      and .records_idempotency_key == false
      and .persists_idempotency_state == false
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
    def ordering_surface($id; $status; $reason; $extra):
      {
        surface:$id,
        source_signing_receipt_replay_idempotency_denial_ready:true,
        canonical_noop_signing_receipt_identity_required:true,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempted:true,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed:false,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted:false,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded:false,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted:false,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_materialized:false,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_filesystem_written:false,
        artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded:false,
        artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted:false,
        artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded:false,
        artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted:false,
        artifact_distribution_signing_notarization_receipt_monotonicity_state_materialized:false,
        artifact_distribution_signing_notarization_receipt_duplicate_sequence_accepted:false,
        artifact_distribution_signing_notarization_receipt_stale_sequence_accepted:false,
        artifact_distribution_signing_notarization_receipt_late_arrival_accepted:false,
        artifact_distribution_signing_notarization_receipt_future_gap_accepted:false,
        artifact_distribution_signing_notarization_receipt_timestamp_rollback_accepted:false,
        artifact_distribution_signing_notarization_receipt_epoch_rollback_accepted:false,
        artifact_distribution_signing_notarization_receipt_same_sequence_different_hash_accepted:false,
        artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted:false,
        artifact_distribution_signing_notarization_receipt_ordered_query_export_accepted:false,
        artifact_distribution_signing_notarization_receipt_ordered_observability_accepted:false,
        artifact_distribution_signing_notarization_receipt_ordered_delivery_accepted:false,
        artifact_distribution_signing_notarization_receipt_ordered_status_accepted:false,
        artifact_distribution_signing_notarization_receipt_ordered_hash_status_accepted:false,
        artifact_signing_receipt_ordering_accepted:false,
        package_signing_receipt_ordering_accepted:false,
        signature_manifest_receipt_late_arrival_accepted:false,
        notarization_submission_receipt_future_gap_accepted:false,
        notarization_ticket_receipt_timestamp_rollback_accepted:false,
        stapling_receipt_epoch_rollback_accepted:false,
        installer_signing_same_sequence_hash_accepted:false,
        provenance_attestation_latest_wins_accepted:false,
        sbom_manifest_monotonic_cursor_recorded:false,
        release_asset_bundle_ordered_query_export_accepted:false,
        cdn_update_feed_ordered_observability_accepted:false,
        package_registry_ordered_status_accepted:false,
        dashboard_endpoint_ordered_hash_status_accepted:false,
        external_ordered_delivery_accepted:false,
        telegram_ordered_delivery_accepted:false,
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
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_noop_confirmed:true,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_status:$status,
        reason:$reason
      } + $extra;
    [
      ordering_surface("source_signing_receipt_replay_idempotency_report_required"; "blocked_source_signing_receipt_replay_idempotency_required_noop"; "source_signing_receipt_replay_idempotency_report_required"; {source_report_required:true}),
      ordering_surface("duplicate_signing_receipt_sequence"; "blocked_duplicate_signing_receipt_sequence_noop"; "duplicate_signing_receipt_sequence_denied"; {duplicate_signing_receipt_sequence_requested:true}),
      ordering_surface("stale_package_signing_receipt_sequence"; "blocked_stale_package_signing_receipt_sequence_noop"; "stale_package_signing_receipt_sequence_denied"; {stale_package_signing_receipt_sequence_requested:true}),
      ordering_surface("signature_manifest_receipt_late_arrival"; "blocked_signature_manifest_receipt_late_arrival_noop"; "signature_manifest_receipt_late_arrival_denied"; {signature_manifest_receipt_late_arrival_requested:true}),
      ordering_surface("notarization_submission_receipt_future_gap"; "blocked_notarization_submission_receipt_future_gap_noop"; "notarization_submission_receipt_future_gap_denied"; {notarization_submission_receipt_future_gap_requested:true}),
      ordering_surface("notarization_ticket_timestamp_rollback"; "blocked_notarization_ticket_timestamp_rollback_noop"; "notarization_ticket_timestamp_rollback_denied"; {notarization_ticket_timestamp_rollback_requested:true}),
      ordering_surface("stapling_receipt_epoch_rollback"; "blocked_stapling_receipt_epoch_rollback_noop"; "stapling_receipt_epoch_rollback_denied"; {stapling_receipt_epoch_rollback_requested:true}),
      ordering_surface("installer_signing_same_sequence_different_hash"; "blocked_installer_signing_same_sequence_different_hash_noop"; "installer_signing_same_sequence_different_hash_denied"; {installer_signing_same_sequence_different_hash_requested:true}),
      ordering_surface("provenance_receipt_latest_wins_overwrite"; "blocked_provenance_receipt_latest_wins_overwrite_noop"; "provenance_receipt_latest_wins_overwrite_denied"; {provenance_receipt_latest_wins_overwrite_requested:true}),
      ordering_surface("sbom_receipt_monotonic_cursor"; "blocked_sbom_receipt_monotonic_cursor_noop"; "sbom_receipt_monotonic_cursor_denied"; {sbom_receipt_monotonic_cursor_requested:true}),
      ordering_surface("release_asset_bundle_receipt_ordered_query_export"; "blocked_release_asset_bundle_receipt_ordered_query_export_noop"; "release_asset_bundle_receipt_ordered_query_export_denied"; {release_asset_bundle_receipt_ordered_query_export_requested:true}),
      ordering_surface("cdn_update_feed_receipt_ordered_observability"; "blocked_cdn_update_feed_receipt_ordered_observability_noop"; "cdn_update_feed_receipt_ordered_observability_denied"; {cdn_update_feed_receipt_ordered_observability_requested:true}),
      ordering_surface("package_registry_ordered_status"; "blocked_package_registry_ordered_status_noop"; "package_registry_ordered_status_denied"; {package_registry_ordered_status_requested:true}),
      ordering_surface("dashboard_endpoint_ordered_hash_status"; "blocked_dashboard_endpoint_ordered_hash_status_noop"; "dashboard_endpoint_ordered_hash_status_denied"; {dashboard_endpoint_ordered_hash_status_requested:true}),
      ordering_surface("external_telegram_ordered_delivery"; "blocked_external_telegram_ordered_delivery_noop"; "external_telegram_ordered_delivery_denied"; {external_ordered_delivery_requested:true, telegram_ordered_delivery_requested:true}),
      ordering_surface("release_publication_authority_ordering"; "blocked_release_publication_authority_ordering_noop"; "release_publication_authority_ordering_denied"; {release_publication_authority_ordering_requested:true}),
      ordering_surface("activation_live_install_ordering"; "blocked_activation_live_install_ordering_noop"; "activation_live_install_ordering_denied"; {activation_live_install_ordering_requested:true}),
      ordering_surface("install_restart_active_binary_ordering_path"; "blocked_install_restart_active_binary_ordering_path_noop"; "install_restart_active_binary_ordering_path_denied"; {install_restart_active_binary_ordering_path_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_gate" \
    --arg source_artifact_distribution_signing_notarization_receipt_replay_idempotency_report_sha256 "$source_artifact_distribution_signing_notarization_receipt_replay_idempotency_report_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256 "$artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256" \
    --arg artifact_distribution_signing_notarization_receipt_ordering_monotonicity_policy_hash_sha256 "$artifact_distribution_signing_notarization_receipt_ordering_monotonicity_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$ARTIFACT_DISTRIBUTION_SIGNING_NOTARIZATION_RECEIPT_REPLAY_IDEMPOTENCY_JSON" \
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
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_v1",
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_mode:"denied_signing_receipt_replay_idempotency_cannot_be_ordered_cursored_latest_wins_rebound_promoted_or_used_for_authority_or_live_install",
        source_artifact_distribution_signing_notarization_receipt_replay_idempotency_gate:$source.gate,
        source_artifact_distribution_signing_notarization_receipt_replay_idempotency_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_replay_idempotency_denial_ready,
        source_artifact_distribution_signing_notarization_receipt_replay_idempotency_report_sha256:$source_artifact_distribution_signing_notarization_receipt_replay_idempotency_report_sha256,
        source_artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256:$source.artifact_distribution_signing_notarization_receipt_replay_idempotency_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256:$artifact_distribution_signing_notarization_receipt_ordering_monotonicity_contract_hash_sha256,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_policy_hash_sha256:$artifact_distribution_signing_notarization_receipt_ordering_monotonicity_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_ready:true,
        source_artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count:$source.artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count,
        source_artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count:$source.artifact_distribution_signing_notarization_receipt_replay_idempotency_attempt_count,
        source_artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count:$source.artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count,
        source_artifact_distribution_signing_notarization_receipt_replay_idempotency_accepted_count:$source.artifact_distribution_signing_notarization_receipt_replay_accepted_count,
        source_artifact_distribution_signing_notarization_receipt_replay_idempotency_persisted_count:$source.artifact_distribution_signing_notarization_receipt_idempotency_state_persisted_count,
        source_release_publication_authority_from_signing_receipt_replay_derived_count:$source.release_publication_authority_from_signing_receipt_replay_derived_count,
        source_activation_authority_from_signing_receipt_replay_derived_count:$source.activation_authority_from_signing_receipt_replay_derived_count,
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization_receipt_ordering_monotonicity:[
          "source_artifact_distribution_signing_notarization_receipt_replay_idempotency_report_required",
          "signing_receipt_duplicate_sequence_denied",
          "signing_receipt_stale_sequence_denied",
          "signing_receipt_late_arrival_denied",
          "signing_receipt_future_gap_denied",
          "signing_receipt_timestamp_rollback_denied",
          "signing_receipt_epoch_rollback_denied",
          "signing_receipt_same_sequence_different_hash_denied",
          "signing_receipt_latest_wins_overwrite_denied",
          "signing_receipt_monotonic_cursor_denied",
          "signing_receipt_ordered_query_export_denied",
          "signing_receipt_ordered_observability_denied",
          "signing_receipt_ordered_status_denied",
          "signing_receipt_ordered_hash_status_denied",
          "external_telegram_signing_receipt_ordered_delivery_denied",
          "release_publication_authority_from_signing_receipt_ordering_denied",
          "activation_live_install_from_signing_receipt_ordering_denied",
          "install_restart_active_binary_from_signing_receipt_ordering_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_gate",
            status:"allowed_report_only_next_slice",
            accepts_ordering:false,
            records_sequence_cursor:false,
            persists_monotonicity_state:false,
            accepts_cancellation:false,
            accepts_supersession:false,
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
        "artifact_signing_receipt_ordering_accepted_count",
        "package_signing_receipt_ordering_accepted_count",
        "signature_manifest_receipt_late_arrival_accepted_count",
        "notarization_submission_receipt_future_gap_accepted_count",
        "notarization_ticket_receipt_timestamp_rollback_accepted_count",
        "stapling_receipt_epoch_rollback_accepted_count",
        "installer_signing_same_sequence_hash_accepted_count",
        "provenance_attestation_latest_wins_accepted_count",
        "sbom_manifest_monotonic_cursor_recorded_count",
        "release_asset_bundle_ordered_query_export_accepted_count",
        "cdn_update_feed_ordered_observability_accepted_count",
        "package_registry_ordered_status_accepted_count",
        "dashboard_endpoint_ordered_hash_status_accepted_count",
        "external_ordered_delivery_accepted_count",
        "telegram_ordered_delivery_accepted_count",
        "acceptance_from_signing_receipt_ordering_recorded_count",
        "operator_approval_from_signing_receipt_ordering_derived_count",
        "release_publication_authority_from_signing_receipt_ordering_derived_count",
        "activation_authority_from_signing_receipt_ordering_derived_count",
        "download_link_from_signing_receipt_ordering_rendered_count",
        "install_command_from_signing_receipt_ordering_rendered_count",
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
      + false_object([
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted",
        "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_materialized",
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
      + {
        side_effects:false_object([
          "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_recorded",
          "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_persisted",
          "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_materialized",
          "artifact_distribution_signing_notarization_receipt_ordering_monotonicity_filesystem_written",
          "artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded",
          "artifact_distribution_signing_notarization_receipt_sequence_cursor_persisted",
          "artifact_distribution_signing_notarization_receipt_monotonicity_state_recorded",
          "artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted",
          "artifact_distribution_signing_notarization_receipt_monotonicity_state_materialized",
          "artifact_distribution_signing_notarization_receipt_duplicate_sequence_accepted",
          "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted",
          "artifact_distribution_signing_notarization_receipt_ordered_query_export_accepted",
          "artifact_distribution_signing_notarization_receipt_ordered_observability_accepted",
          "artifact_distribution_signing_notarization_receipt_ordered_delivery_accepted",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denial_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_replay_idempotency_ready == true
  and $report.source_artifact_distribution_signing_notarization_receipt_replay_idempotency_surface_count == 18
  and $report.source_artifact_distribution_signing_notarization_receipt_replay_idempotency_denied_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surface_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_denied_count == 18
  and zero_fields($report; [
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
    "artifact_distribution_signing_notarization_receipt_duplicate_sequence_accepted_count",
    "artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted_count",
    "artifact_signing_receipt_ordering_accepted_count",
    "package_signing_receipt_ordering_accepted_count",
    "sbom_manifest_monotonic_cursor_recorded_count",
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
  and false_fields($report; [
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
  and ($report.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces | all(
    .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_attempted == true
    and .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_allowed == false
    and .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_accepted == false
    and .artifact_distribution_signing_notarization_receipt_sequence_cursor_recorded == false
    and .artifact_distribution_signing_notarization_receipt_monotonicity_state_persisted == false
    and .artifact_distribution_signing_notarization_receipt_latest_wins_overwrite_accepted == false
    and .artifact_distribution_signing_notarization_receipt_ordering_monotonicity_noop_confirmed == true
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
  and ([.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces[] | select(.signature_manifest_receipt_late_arrival_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces[] | select(.notarization_ticket_timestamp_rollback_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces[] | select(.stapling_receipt_epoch_rollback_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces[] | select(.telegram_ordered_delivery_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_receipt_ordering_monotonicity_surfaces[] | select(.install_restart_active_binary_ordering_path_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_receipt_cancellation_supersession_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .accepts_ordering == false
    and .records_sequence_cursor == false
    and .persists_monotonicity_state == false
    and .accepts_cancellation == false
    and .accepts_supersession == false
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
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization receipt ordering/monotonicity denial gate passed"
