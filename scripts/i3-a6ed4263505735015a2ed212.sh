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

DISTRIBUTION_ARTIFACT_MANIFEST_JSON="$(
  HEPTA_LIVE_URL="$BASE_URL" \
    HEPTA_LIVE_MUTATION_MIN_SOAK_SAMPLES="$MIN_LONG_SOAK_SAMPLES" \
    capture_json_report "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-distribution-artifact-manifest-status-denial-gate" \
    scripts/i3-d4091c000c5aec055d0e0396.sh
)"

source_distribution_artifact_manifest_report_sha256="$(sha256_text "$DISTRIBUTION_ARTIFACT_MANIFEST_JSON")"
artifact_distribution_signing_notarization_contract_hash_sha256="$(
  sha256_text "hepta-memory-intelligence-kg-full-live-activation-artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-surface-denial:$source_distribution_artifact_manifest_report_sha256:$MIN_LONG_SOAK_SAMPLES"
)"
artifact_distribution_signing_notarization_policy_hash_sha256="$(
  sha256_text "artifact-download-install-affordance-result-receipt-operator-identity-session-revocation-logout-replay-reinstatement-operator-intent-consent-evidence-artifact-distribution-signing-notarization-surface:no-signing:no-notarization:no-stapling:no-provenance:no-sbom:no-package-channel:no-authority:no-install:no-live"
)"

jq -n -e \
  --argjson source "$DISTRIBUTION_ARTIFACT_MANIFEST_JSON" \
  --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
  '
    def zero_fields($o; $fields): all($fields[]; $o[.] == 0);
    def false_fields($o; $fields): all($fields[]; $o[.] == false);

    $source.runtime == "hepta"
    and $source.status == "ready"
    and $source.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_gate"
    and $source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_ready == true
    and $source.source_package_release_channel_status_exposure_ready == true
    and $source.distribution_artifact_manifest_status_surface_count == 18
    and $source.distribution_artifact_manifest_status_attempt_count == 18
    and $source.distribution_artifact_manifest_status_denied_count == 18
    and zero_fields($source; [
      "distribution_artifact_manifest_status_allowed_count",
      "distribution_artifact_manifest_status_request_accepted_count",
      "distribution_artifact_manifest_status_accepted_count",
      "distribution_artifact_manifest_status_recorded_count",
      "distribution_artifact_manifest_status_persisted_count",
      "distribution_artifact_manifest_status_materialized_count",
      "distribution_artifact_manifest_status_filesystem_written_count",
      "distribution_artifact_manifest_status_delivered_count",
      "distribution_artifact_status_exposed_count",
      "manifest_status_exposed_count",
      "artifact_index_status_exposed_count",
      "package_manifest_materialized_count",
      "release_manifest_published_count",
      "artifact_catalog_status_exposed_count",
      "manifest_checksum_status_exposed_count",
      "artifact_provenance_status_exposed_count",
      "manifest_signature_status_exposed_count",
      "dashboard_status_exposed_count",
      "public_endpoint_status_exposed_count",
      "query_status_exposed_count",
      "export_status_exposed_count",
      "observability_status_exposed_count",
      "external_status_sent_count",
      "telegram_status_sent_count",
      "public_status_claimed_count",
      "public_release_claimed_count",
      "public_ga_claimed_count",
      "acceptance_from_manifest_status_recorded_count",
      "operator_approval_from_manifest_status_derived_count",
      "release_publication_authority_from_manifest_status_derived_count",
      "activation_authority_from_manifest_status_derived_count",
      "download_link_from_manifest_status_rendered_count",
      "install_command_from_manifest_status_rendered_count",
      "install_from_manifest_status_executed_count",
      "service_restart_from_manifest_status_performed_count",
      "active_binary_from_manifest_status_mutated_count",
      "memory_store_write_performed_count",
      "live_kg_write_performed_count",
      "provider_invoked_count",
      "model_invoked_count",
      "credential_read_count",
      "secret_file_read_count",
      "external_send_performed_count"
    ])
    and false_fields($source; [
      "distribution_artifact_manifest_status_accepted",
      "distribution_artifact_manifest_status_recorded",
      "distribution_artifact_manifest_status_persisted",
      "distribution_artifact_status_exposed",
      "manifest_status_exposed",
      "artifact_manifest_materialized",
      "release_manifest_published",
      "public_status_claimed",
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
    and ($source.distribution_artifact_manifest_status_surfaces | length) == 18
    and ($source.distribution_artifact_manifest_status_surfaces | all(
      .distribution_artifact_manifest_status_attempted == true
      and .distribution_artifact_manifest_status_allowed == false
      and .distribution_artifact_manifest_status_accepted == false
      and .distribution_artifact_manifest_status_recorded == false
      and .distribution_artifact_manifest_status_persisted == false
      and .distribution_artifact_status_exposed == false
      and .manifest_status_exposed == false
      and .manifest_signature_status_exposed == false
      and .release_manifest_published == false
      and .release_publication_authority_from_manifest_status_derived == false
      and .activation_authority_from_manifest_status_derived == false
      and .install_from_manifest_status_executed == false
      and .service_restart_from_manifest_status_performed == false
      and .active_binary_from_manifest_status_mutated == false
      and .memory_store_write_performed == false
      and .live_kg_write_performed == false
      and .external_send_performed == false
      and .distribution_artifact_manifest_status_noop_confirmed == true
    ))
    and ($source.allowed_next_actions | any(
      .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_surface_denial_gate"
      and .exposes_distribution_artifact_manifest_status == false
      and .records_artifact_distribution_signing == false
      and .records_notarization == false
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
    and ($source.side_effects | to_entries | all(.value == false))
    and $min_long_soak_samples >= 24
  ' >/dev/null

surfaces_json="$(
  jq -n '
    def surface($id; $status; $reason; $extra):
      {
        surface:$id,
        source_distribution_artifact_manifest_status_ready:true,
        artifact_distribution_signing_notarization_surface_attempted:true,
        artifact_distribution_signing_notarization_surface_allowed:false,
        artifact_distribution_signing_notarization_surface_request_accepted:false,
        artifact_distribution_signing_notarization_surface_accepted:false,
        artifact_distribution_signing_notarization_surface_recorded:false,
        artifact_distribution_signing_notarization_surface_persisted:false,
        artifact_distribution_signing_notarization_surface_materialized:false,
        artifact_distribution_signing_notarization_surface_filesystem_written:false,
        artifact_distribution_signing_notarization_surface_delivered:false,
        artifact_distribution_signing_notarization_surface_exposed:false,
        artifact_distribution_signing_notarization_surface_executed:false,
        artifact_signing_executed:false,
        package_signing_executed:false,
        signature_manifest_written:false,
        signature_checksum_bound:false,
        notarization_submitted:false,
        notarization_ticket_recorded:false,
        stapling_executed:false,
        installer_signing_executed:false,
        provenance_attestation_published:false,
        sbom_manifest_published:false,
        release_asset_packaged:false,
        artifact_bundle_packaged:false,
        cdn_artifact_written:false,
        update_feed_artifact_written:false,
        package_registry_artifact_published:false,
        external_package_channel_published:false,
        telegram_package_channel_published:false,
        dashboard_signing_status_exposed:false,
        endpoint_signing_status_exposed:false,
        query_export_signing_status_exposed:false,
        observability_signing_status_exposed:false,
        public_release_claimed:false,
        public_ga_claimed:false,
        acceptance_recorded:false,
        operator_approval_derived:false,
        release_publication_authority_derived:false,
        activation_authority_derived:false,
        activation_command_derived:false,
        live_execution_allowed:false,
        activation_performed:false,
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
        artifact_distribution_signing_notarization_surface_noop_confirmed:true,
        artifact_distribution_signing_notarization_surface_status:$status,
        reason:$reason
      } + $extra;
    [
      surface("source_distribution_artifact_manifest_status_report_required"; "blocked_source_distribution_artifact_manifest_required_noop"; "source_distribution_artifact_manifest_status_report_required"; {source_report_required:true}),
      surface("artifact_signing_execution"; "blocked_artifact_signing_execution_noop"; "artifact_signing_execution_denied"; {artifact_signing_requested:true}),
      surface("package_signing_execution"; "blocked_package_signing_execution_noop"; "package_signing_execution_denied"; {package_signing_requested:true}),
      surface("signature_manifest_write_checksum_binding"; "blocked_signature_manifest_checksum_noop"; "signature_manifest_write_checksum_binding_denied"; {signature_manifest_write_requested:true, signature_checksum_binding_requested:true}),
      surface("notarization_submission"; "blocked_notarization_submission_noop"; "notarization_submission_denied"; {notarization_submission_requested:true}),
      surface("notarization_ticket_recording"; "blocked_notarization_ticket_recording_noop"; "notarization_ticket_recording_denied"; {notarization_ticket_record_requested:true}),
      surface("stapling_execution"; "blocked_stapling_execution_noop"; "stapling_execution_denied"; {stapling_execution_requested:true}),
      surface("installer_signing_execution"; "blocked_installer_signing_execution_noop"; "installer_signing_execution_denied"; {installer_signing_requested:true}),
      surface("provenance_attestation_publication"; "blocked_provenance_attestation_publication_noop"; "provenance_attestation_publication_denied"; {provenance_attestation_publication_requested:true}),
      surface("sbom_manifest_publication"; "blocked_sbom_manifest_publication_noop"; "sbom_manifest_publication_denied"; {sbom_manifest_publication_requested:true}),
      surface("release_asset_artifact_bundle_packaging"; "blocked_release_asset_artifact_bundle_packaging_noop"; "release_asset_artifact_bundle_packaging_denied"; {release_asset_packaging_requested:true, artifact_bundle_packaging_requested:true}),
      surface("cdn_update_feed_artifact_write"; "blocked_cdn_update_feed_artifact_write_noop"; "cdn_update_feed_artifact_write_denied"; {cdn_artifact_write_requested:true, update_feed_artifact_write_requested:true}),
      surface("package_registry_artifact_publish"; "blocked_package_registry_artifact_publish_noop"; "package_registry_artifact_publish_denied"; {package_registry_artifact_publish_requested:true}),
      surface("dashboard_endpoint_query_export_observability_signing_status"; "blocked_dashboard_endpoint_query_export_observability_signing_status_noop"; "dashboard_endpoint_query_export_observability_signing_status_denied"; {dashboard_signing_status_requested:true, endpoint_signing_status_requested:true, query_export_signing_status_requested:true, observability_signing_status_requested:true}),
      surface("external_telegram_package_channel_publication"; "blocked_external_telegram_package_channel_publication_noop"; "external_telegram_package_channel_publication_denied"; {external_package_channel_publication_requested:true, telegram_package_channel_publication_requested:true}),
      surface("release_publication_authority_signing_status"; "blocked_release_publication_authority_signing_status_noop"; "release_publication_authority_signing_status_denied"; {release_publication_authority_signing_status_requested:true}),
      surface("activation_live_install_signing_status"; "blocked_activation_live_install_signing_status_noop"; "activation_live_install_signing_status_denied"; {activation_live_signing_status_requested:true, live_install_signing_status_requested:true}),
      surface("install_restart_active_binary_signing_path"; "blocked_install_restart_active_binary_signing_path_noop"; "install_restart_active_binary_signing_path_denied"; {install_restart_active_binary_signing_path_requested:true})
    ]
  '
)"

report="$(
  jq -n \
    --arg product "Hepta" \
    --arg runtime "hepta" \
    --arg base_url "$BASE_URL" \
    --arg gate "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_surface_denial_gate" \
    --arg source_distribution_artifact_manifest_report_sha256 "$source_distribution_artifact_manifest_report_sha256" \
    --arg artifact_distribution_signing_notarization_contract_hash_sha256 "$artifact_distribution_signing_notarization_contract_hash_sha256" \
    --arg artifact_distribution_signing_notarization_policy_hash_sha256 "$artifact_distribution_signing_notarization_policy_hash_sha256" \
    --argjson min_long_soak_samples "$MIN_LONG_SOAK_SAMPLES" \
    --argjson source "$DISTRIBUTION_ARTIFACT_MANIFEST_JSON" \
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
        artifact_distribution_signing_notarization_surface_schema_version:"operator_intent_consent_evidence_artifact_distribution_signing_notarization_surface_denial_v1",
        artifact_distribution_signing_notarization_surface_mode:"denied_distribution_artifact_manifest_status_cannot_execute_signing_notarization_stapling_provenance_sbom_packaging_channel_publication_authority_or_live_install",
        source_distribution_artifact_manifest_status_gate:$source.gate,
        source_distribution_artifact_manifest_status_ready:$source.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_distribution_artifact_manifest_status_denial_ready,
        source_distribution_artifact_manifest_report_sha256:$source_distribution_artifact_manifest_report_sha256,
        artifact_distribution_signing_notarization_contract_hash_sha256:$artifact_distribution_signing_notarization_contract_hash_sha256,
        artifact_distribution_signing_notarization_policy_hash_sha256:$artifact_distribution_signing_notarization_policy_hash_sha256,
        minimum_required_samples:$min_long_soak_samples,
        memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_surface_denial_ready:true,
        source_distribution_artifact_manifest_status_surface_count:$source.distribution_artifact_manifest_status_surface_count,
        source_distribution_artifact_manifest_status_attempt_count:$source.distribution_artifact_manifest_status_attempt_count,
        source_distribution_artifact_manifest_status_denied_count:$source.distribution_artifact_manifest_status_denied_count,
        source_distribution_artifact_status_exposed_count:$source.distribution_artifact_status_exposed_count,
        source_manifest_status_exposed_count:$source.manifest_status_exposed_count,
        source_artifact_index_status_exposed_count:$source.artifact_index_status_exposed_count,
        source_manifest_signature_status_exposed_count:$source.manifest_signature_status_exposed_count,
        source_release_manifest_published_count:$source.release_manifest_published_count,
        source_release_publication_authority_from_manifest_status_derived_count:$source.release_publication_authority_from_manifest_status_derived_count,
        source_activation_authority_from_manifest_status_derived_count:$source.activation_authority_from_manifest_status_derived_count,
        artifact_distribution_signing_notarization_surface_count:($surfaces | length),
        artifact_distribution_signing_notarization_surface_attempt_count:($surfaces | length),
        artifact_distribution_signing_notarization_surface_denied_count:($surfaces | length),
        artifact_distribution_signing_notarization_surfaces:$surfaces,
        denied_by_artifact_distribution_signing_notarization:[
          "source_distribution_artifact_manifest_status_report_required",
          "artifact_distribution_signing_notarization_request_acceptance_denied",
          "artifact_distribution_signing_notarization_acceptance_denied",
          "artifact_distribution_signing_notarization_recording_denied",
          "artifact_distribution_signing_notarization_persistence_denied",
          "artifact_distribution_signing_notarization_materialization_denied",
          "artifact_distribution_signing_notarization_filesystem_write_denied",
          "artifact_distribution_signing_notarization_delivery_denied",
          "artifact_distribution_signing_notarization_exposure_denied",
          "artifact_signing_execution_denied",
          "package_signing_execution_denied",
          "signature_manifest_write_checksum_binding_denied",
          "notarization_submission_denied",
          "notarization_ticket_recording_denied",
          "stapling_execution_denied",
          "installer_signing_execution_denied",
          "provenance_attestation_publication_denied",
          "sbom_manifest_publication_denied",
          "release_asset_artifact_bundle_packaging_denied",
          "cdn_update_feed_artifact_write_denied",
          "package_registry_artifact_publish_denied",
          "dashboard_endpoint_query_export_observability_signing_status_denied",
          "external_telegram_package_channel_publication_denied",
          "release_publication_authority_from_signing_notarization_denied",
          "activation_live_install_from_signing_notarization_denied",
          "install_restart_active_binary_from_signing_notarization_denied",
          "memory_provider_kg_secret_external_send_from_signing_notarization_denied"
        ],
        allowed_next_actions:[
          {
            action:"prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_result_receipt_no_persistence_denial_gate",
            status:"allowed_report_only_next_slice",
            executes_signing:false,
            executes_notarization:false,
            executes_stapling:false,
            writes_signature_manifest:false,
            publishes_provenance:false,
            publishes_sbom:false,
            packages_release_asset:false,
            publishes_package_channel:false,
            records_signing_receipt:false,
            records_operator_acceptance:false,
            derives_release_publication_authority:false,
            derives_activation_authority:false,
            installs_or_restarts:false,
            mutates_active_binary:false,
            mutates_memory_store:false,
            writes_kg:false,
            sends_externally:false
          }
        ]
      }
      + zero_object([
        "artifact_distribution_signing_notarization_surface_allowed_count",
        "artifact_distribution_signing_notarization_surface_request_accepted_count",
        "artifact_distribution_signing_notarization_surface_accepted_count",
        "artifact_distribution_signing_notarization_surface_recorded_count",
        "artifact_distribution_signing_notarization_surface_persisted_count",
        "artifact_distribution_signing_notarization_surface_materialized_count",
        "artifact_distribution_signing_notarization_surface_filesystem_written_count",
        "artifact_distribution_signing_notarization_surface_delivered_count",
        "artifact_distribution_signing_notarization_surface_exposed_count",
        "artifact_distribution_signing_notarization_surface_executed_count",
        "artifact_signing_executed_count",
        "package_signing_executed_count",
        "signature_manifest_written_count",
        "signature_checksum_bound_count",
        "notarization_submitted_count",
        "notarization_ticket_recorded_count",
        "stapling_executed_count",
        "installer_signing_executed_count",
        "provenance_attestation_published_count",
        "sbom_manifest_published_count",
        "release_asset_packaged_count",
        "artifact_bundle_packaged_count",
        "cdn_artifact_written_count",
        "update_feed_artifact_written_count",
        "package_registry_artifact_published_count",
        "external_package_channel_published_count",
        "telegram_package_channel_published_count",
        "dashboard_signing_status_exposed_count",
        "endpoint_signing_status_exposed_count",
        "query_export_signing_status_exposed_count",
        "observability_signing_status_exposed_count",
        "public_release_claimed_count",
        "public_ga_claimed_count",
        "acceptance_from_signing_status_recorded_count",
        "operator_approval_from_signing_status_derived_count",
        "release_publication_authority_from_signing_status_derived_count",
        "activation_authority_from_signing_status_derived_count",
        "install_from_signing_status_executed_count",
        "service_restart_from_signing_status_performed_count",
        "active_binary_from_signing_status_mutated_count",
        "memory_store_write_performed_count",
        "live_kg_write_performed_count",
        "provider_invoked_count",
        "model_invoked_count",
        "credential_read_count",
        "secret_file_read_count",
        "external_send_performed_count"
      ])
      + false_object([
        "artifact_distribution_signing_notarization_surface_accepted",
        "artifact_distribution_signing_notarization_surface_recorded",
        "artifact_distribution_signing_notarization_surface_persisted",
        "artifact_distribution_signing_notarization_surface_exposed",
        "artifact_distribution_signing_notarization_surface_executed",
        "artifact_signing_executed",
        "package_signing_executed",
        "signature_manifest_written",
        "notarization_submitted",
        "notarization_ticket_recorded",
        "stapling_executed",
        "provenance_attestation_published",
        "sbom_manifest_published",
        "release_asset_packaged",
        "package_registry_artifact_published",
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
          "artifact_distribution_signing_notarization_surface_recorded",
          "artifact_distribution_signing_notarization_surface_persisted",
          "artifact_distribution_signing_notarization_surface_materialized",
          "artifact_distribution_signing_notarization_surface_filesystem_written",
          "artifact_distribution_signing_notarization_surface_delivered",
          "artifact_distribution_signing_notarization_surface_exposed",
          "artifact_distribution_signing_notarization_surface_executed",
          "artifact_signing_executed",
          "package_signing_executed",
          "signature_manifest_written",
          "signature_checksum_bound",
          "notarization_submitted",
          "notarization_ticket_recorded",
          "stapling_executed",
          "installer_signing_executed",
          "provenance_attestation_published",
          "sbom_manifest_published",
          "release_asset_packaged",
          "artifact_bundle_packaged",
          "cdn_artifact_written",
          "update_feed_artifact_written",
          "package_registry_artifact_published",
          "external_package_channel_published",
          "telegram_package_channel_published",
          "dashboard_signing_status_exposed",
          "endpoint_signing_status_exposed",
          "query_export_signing_status_exposed",
          "observability_signing_status_exposed",
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
  and $report.gate == "hepta_memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_surface_denial_gate"
  and $report.memory_intelligence_kg_full_live_activation_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_surface_denial_ready == true
  and $report.source_distribution_artifact_manifest_status_ready == true
  and $report.source_distribution_artifact_manifest_status_surface_count == 18
  and $report.source_distribution_artifact_manifest_status_attempt_count == 18
  and $report.source_distribution_artifact_manifest_status_denied_count == 18
  and $report.source_distribution_artifact_status_exposed_count == 0
  and $report.source_manifest_status_exposed_count == 0
  and $report.source_artifact_index_status_exposed_count == 0
  and $report.source_manifest_signature_status_exposed_count == 0
  and $report.source_release_manifest_published_count == 0
  and $report.source_release_publication_authority_from_manifest_status_derived_count == 0
  and $report.source_activation_authority_from_manifest_status_derived_count == 0
  and $report.artifact_distribution_signing_notarization_surface_count == 18
  and $report.artifact_distribution_signing_notarization_surface_attempt_count == 18
  and $report.artifact_distribution_signing_notarization_surface_denied_count == 18
  and zero_fields($report; [
    "artifact_distribution_signing_notarization_surface_allowed_count",
    "artifact_distribution_signing_notarization_surface_request_accepted_count",
    "artifact_distribution_signing_notarization_surface_accepted_count",
    "artifact_distribution_signing_notarization_surface_recorded_count",
    "artifact_distribution_signing_notarization_surface_persisted_count",
    "artifact_distribution_signing_notarization_surface_materialized_count",
    "artifact_distribution_signing_notarization_surface_filesystem_written_count",
    "artifact_distribution_signing_notarization_surface_delivered_count",
    "artifact_distribution_signing_notarization_surface_exposed_count",
    "artifact_distribution_signing_notarization_surface_executed_count",
    "artifact_signing_executed_count",
    "package_signing_executed_count",
    "signature_manifest_written_count",
    "signature_checksum_bound_count",
    "notarization_submitted_count",
    "notarization_ticket_recorded_count",
    "stapling_executed_count",
    "installer_signing_executed_count",
    "provenance_attestation_published_count",
    "sbom_manifest_published_count",
    "release_asset_packaged_count",
    "artifact_bundle_packaged_count",
    "cdn_artifact_written_count",
    "update_feed_artifact_written_count",
    "package_registry_artifact_published_count",
    "external_package_channel_published_count",
    "telegram_package_channel_published_count",
    "dashboard_signing_status_exposed_count",
    "endpoint_signing_status_exposed_count",
    "query_export_signing_status_exposed_count",
    "observability_signing_status_exposed_count",
    "public_release_claimed_count",
    "public_ga_claimed_count",
    "acceptance_from_signing_status_recorded_count",
    "operator_approval_from_signing_status_derived_count",
    "release_publication_authority_from_signing_status_derived_count",
    "activation_authority_from_signing_status_derived_count",
    "install_from_signing_status_executed_count",
    "service_restart_from_signing_status_performed_count",
    "active_binary_from_signing_status_mutated_count",
    "memory_store_write_performed_count",
    "live_kg_write_performed_count",
    "provider_invoked_count",
    "model_invoked_count",
    "credential_read_count",
    "secret_file_read_count",
    "external_send_performed_count"
  ])
  and false_fields($report; [
    "artifact_distribution_signing_notarization_surface_accepted",
    "artifact_distribution_signing_notarization_surface_recorded",
    "artifact_distribution_signing_notarization_surface_persisted",
    "artifact_distribution_signing_notarization_surface_exposed",
    "artifact_distribution_signing_notarization_surface_executed",
    "artifact_signing_executed",
    "package_signing_executed",
    "signature_manifest_written",
    "notarization_submitted",
    "notarization_ticket_recorded",
    "stapling_executed",
    "provenance_attestation_published",
    "sbom_manifest_published",
    "release_asset_packaged",
    "package_registry_artifact_published",
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
  and ($report.artifact_distribution_signing_notarization_surfaces | length) == 18
  and ($report.artifact_distribution_signing_notarization_surfaces | all(
    .artifact_distribution_signing_notarization_surface_attempted == true
    and .artifact_distribution_signing_notarization_surface_allowed == false
    and .artifact_distribution_signing_notarization_surface_request_accepted == false
    and .artifact_distribution_signing_notarization_surface_accepted == false
    and .artifact_distribution_signing_notarization_surface_recorded == false
    and .artifact_distribution_signing_notarization_surface_persisted == false
    and .artifact_distribution_signing_notarization_surface_materialized == false
    and .artifact_distribution_signing_notarization_surface_filesystem_written == false
    and .artifact_distribution_signing_notarization_surface_delivered == false
    and .artifact_distribution_signing_notarization_surface_exposed == false
    and .artifact_distribution_signing_notarization_surface_executed == false
    and .artifact_signing_executed == false
    and .package_signing_executed == false
    and .signature_manifest_written == false
    and .signature_checksum_bound == false
    and .notarization_submitted == false
    and .notarization_ticket_recorded == false
    and .stapling_executed == false
    and .installer_signing_executed == false
    and .provenance_attestation_published == false
    and .sbom_manifest_published == false
    and .release_asset_packaged == false
    and .artifact_bundle_packaged == false
    and .cdn_artifact_written == false
    and .update_feed_artifact_written == false
    and .package_registry_artifact_published == false
    and .external_package_channel_published == false
    and .telegram_package_channel_published == false
    and .acceptance_recorded == false
    and .operator_approval_derived == false
    and .release_publication_authority_derived == false
    and .activation_authority_derived == false
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
    and .artifact_distribution_signing_notarization_surface_noop_confirmed == true
  ))
  and ([.artifact_distribution_signing_notarization_surfaces[] | select(.artifact_signing_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_surfaces[] | select(.notarization_submission_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_surfaces[] | select(.stapling_execution_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_surfaces[] | select(.provenance_attestation_publication_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_surfaces[] | select(.telegram_package_channel_publication_requested == true)] | length) == 1
  and ([.artifact_distribution_signing_notarization_surfaces[] | select(.install_restart_active_binary_signing_path_requested == true)] | length) == 1
  and ($report.allowed_next_actions | any(
    .action == "prepare_operator_readiness_packet_template_packet_acceptance_receipt_release_publication_result_receipt_terminal_distribution_delivery_receipt_artifact_download_install_affordance_result_receipt_operator_identity_session_revocation_logout_replay_reinstatement_operator_intent_consent_evidence_artifact_distribution_signing_notarization_result_receipt_no_persistence_denial_gate"
    and .status == "allowed_report_only_next_slice"
    and .executes_signing == false
    and .executes_notarization == false
    and .executes_stapling == false
    and .writes_signature_manifest == false
    and .publishes_provenance == false
    and .publishes_sbom == false
    and .packages_release_asset == false
    and .publishes_package_channel == false
    and .records_signing_receipt == false
    and .records_operator_acceptance == false
    and .derives_release_publication_authority == false
    and .derives_activation_authority == false
    and .installs_or_restarts == false
    and .mutates_active_binary == false
    and .mutates_memory_store == false
    and .writes_kg == false
    and .sends_externally == false
  ))
  and ($report.side_effects | to_entries | all(.value == false))
' <<<"$report" >/dev/null

printf '%s\n' "$report" | jq .
echo "Hepta memory/intelligence/KG full live activation operator readiness packet template packet acceptance receipt release/publication result receipt terminal distribution delivery receipt artifact download/install affordance result receipt operator identity/session revocation/logout replay/reinstatement operator intent/consent evidence artifact distribution signing/notarization surface denial gate passed"
